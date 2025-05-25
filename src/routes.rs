use axum::{extract::Query, response::{IntoResponse, Redirect, Response}, http::StatusCode};
use std::collections::HashMap;

// 主页处理
pub async fn index(Query(params): Query<HashMap<String, String>>) -> Response {
    if let Some(q) = params.get("q") {
        let redirect_url = format!("/{}", q);
        return Redirect::permanent(&redirect_url).into_response();
    }
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found on this server.",
    )
        .into_response()
}

// robots.txt
pub async fn robots() -> &'static str {
    "User-agent: *\r\nDisallow: /"
}
