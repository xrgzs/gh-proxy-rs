use regex::Regex;
use once_cell::sync::Lazy;
use crate::config::CONFIG;

// GitHub URL正则匹配模式
pub static REGEX_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^(?:https?://)?github\\.com/(?P<author>.+?)/(?P<repo>.+?)/(?:releases|archive)/.*$").unwrap(),
        Regex::new(r"^(?:https?://)?github\\.com/(?P<author>.+?)/(?P<repo>.+?)/(?:blob|raw)/.*$").unwrap(),
        Regex::new(r"^(?:https?://)?github\\.com/(?P<author>.+?)/(?P<repo>.+?)/(?:info|git-).*$").unwrap(),
        Regex::new(r"^(?:https?://)?raw\\.(?:githubusercontent|github)\\.com/(?P<author>.+?)/(?P<repo>.+?)/.+?/.+$").unwrap(),
        Regex::new(r"^(?:https?://)?gist\\.(?:githubusercontent|github)\\.com/(?P<author>.+?)/.+?/.+$").unwrap(),
    ]
});

pub static BLOB_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:https?://)?github\\.com/(?P<author>.+?)/(?P<repo>.+?)/(?:blob|raw)/.*$").unwrap()
});

// 检查URL是否匹配GitHub模式
pub fn check_url(url: &str) -> Option<(String, String)> {
    for regex in REGEX_PATTERNS.iter() {
        if let Some(captures) = regex.captures(url) {
            let author = captures.name("author")?.as_str().to_string();
            let repo = captures.name("repo")?.as_str().to_string();
            return Some((author, repo));
        }
    }
    None
}

// 检查访问控制
pub fn check_access_control(author: &str, repo: &str) -> Result<bool, &'static str> {
    if !CONFIG.white_list.is_empty() {
        let allowed = CONFIG.white_list.iter().any(|(a, r)| {
            (a == "*" && r == repo) || (a == author && r == repo) || (a == author && r == "*")
        });
        if !allowed {
            return Err("Forbidden by white list.");
        }
    }
    let blocked = CONFIG.black_list.iter().any(|(a, r)| {
        (a == "*" && r == repo) || (a == author && r == repo) || (a == author && r == "*")
    });
    if blocked {
        return Err("Forbidden by black list.");
    }
    let should_redirect = CONFIG.pass_list.iter().any(|(a, r)| {
        (a == "*" && r == repo) || (a == author && r == repo) || (a == author && r == "*")
    });
    Ok(should_redirect)
}
