use axum::{extract::{Path, Request}, http::{HeaderMap, HeaderName, StatusCode}, response::{IntoResponse, Redirect, Response}};
use bytes::Bytes;
use futures::StreamExt;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{future::Future, pin::Pin, str::FromStr};
use tokio_stream::wrappers::ReceiverStream;

use crate::access::{check_url, check_access_control, BLOB_REGEX};
use crate::config::{CONFIG, HEADERS_TO_REMOVE};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .unwrap()
});

// 主要的代理处理函数
pub async fn handler(
    Path(path): Path<String>,
    headers: HeaderMap,
    request: Request,
) -> Response {
    let mut url = if path.starts_with("http") {
        path
    } else {
        format!("https://{}", path)
    };
    if url.rfind("://").map(|pos| pos > 8).unwrap_or(false) {
    } else if !url.contains("://") && url.starts_with("s:/") {
        url = url.replacen("s:/", "s://", 1);
    }
    let (author, repo) = match check_url(&url) {
        Some(tuple) => tuple,
        None => {
            return (StatusCode::FORBIDDEN, "Invalid input.").into_response();
        }
    };
    match check_access_control(&author, &repo) {
        Ok(should_redirect) => {
            if should_redirect {
                let redirect_url = format!("{}{}", CONFIG.big_server, url);
                return Redirect::permanent(&redirect_url).into_response();
            }
        }
        Err(msg) => {
            return (StatusCode::FORBIDDEN, msg).into_response();
        }
    }
    if BLOB_REGEX.is_match(&url) {
        url = url.replace("/blob/", "/raw/");
    }
    proxy_request(url, headers, request).await
}

// 代理请求函数
pub fn proxy_request(
    target_url: String,
    original_headers: HeaderMap,
    original_request: Request,
) -> Pin<Box<dyn Future<Output = Response> + Send>> {
    Box::pin(async move {
        let url = match reqwest::Url::parse(&target_url) {
            Ok(u) => u,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
            }
        };
        let mut headers = reqwest::header::HeaderMap::new();
        for (name, value) in original_headers.iter() {
            if name.as_str().to_lowercase() != "host" {
                if let (Ok(req_name), Ok(req_value)) = (
                    reqwest::header::HeaderName::from_str(name.as_str()),
                    reqwest::header::HeaderValue::from_str(
                        value.to_str().unwrap_or(""),
                    ),
                ) {
                    headers.insert(req_name, req_value);
                }
            }
        }
        let response = match CLIENT
            .get(url)
            .headers(headers)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let error_msg = format!("server error {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [("content-type", "text/html; charset=UTF-8")],
                    error_msg,
                )
                    .into_response();
            }
        };
        if let Some(content_length) = response.content_length() {
            if content_length > CONFIG.size_limit as u64 {
                let redirect_url = format!("{}{}", CONFIG.big_server, target_url);
                return Redirect::permanent(&redirect_url).into_response();
            }
        }
        if let Some(location) = response.headers().get("location") {
            if let Ok(location_str) = location.to_str() {
                if crate::access::check_url(location_str).is_none() {
                    return proxy_request(location_str.to_string(), original_headers, original_request).await;
                } else {
                    let redirect_url = format!("/{}", location_str);
                    return Redirect::permanent(&redirect_url).into_response();
                }
            }
        }
        let mut response_headers = HeaderMap::new();
        for (name, value) in response.headers().iter() {
            let name_lower = name.as_str().to_lowercase();
            if !HEADERS_TO_REMOVE.contains(&name_lower.as_str()) {
                if let Ok(header_name) = HeaderName::from_str(name.as_str()) {
                    response_headers.insert(header_name, value.clone());
                }
            }
        }
        let status = response.status();
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Bytes, axum::Error>>(1);
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        if tx.send(Ok(chunk)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(axum::Error::new(e))).await;
                        break;
                    }
                }
            }
        });
        let stream = ReceiverStream::new(rx);
        let body = axum::body::Body::from_stream(stream);
        (status, response_headers, body).into_response()
    })
}
