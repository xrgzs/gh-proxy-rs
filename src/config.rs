use once_cell::sync::Lazy;
use std::{env, fs::File, io::{BufRead, BufReader}};

// 配置常量
pub const DEFAULT_SIZE_LIMIT: usize = 1024 * 1024 * 1024 * 999; // 999GB
pub const DEFAULT_BIG_SERVER: &str = "https://ghfast.top/";
pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 8000;

// 需要删除的响应头
pub const HEADERS_TO_REMOVE: &[&str] = &[
    "transfer-encoding",
    "strict-transport-security",
    "access-control-allow-origin",
    "clear-site-data",
    "content-security-policy",
    "content-security-policy-report-only",
    "cross-origin-resource-policy",
    "x-github-request-id",
    "x-fastly-request-id",
    "via",
    "x-served-by",
    "x-cache",
    "x-cache-hits",
    "x-timer",
    "expires",
    "source-age",
];

// 全局配置结构体
pub struct Config {
    pub size_limit: usize,
    pub big_server: String,
    pub white_list: Vec<(String, String)>,
    pub black_list: Vec<(String, String)>,
    pub pass_list: Vec<(String, String)>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    size_limit: env::var("SIZE_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SIZE_LIMIT),
    big_server: env::var("BIG_SERVER").unwrap_or_else(|_| DEFAULT_BIG_SERVER.to_string()),
    white_list: read_and_process_rules("whitelist.txt"),
    black_list: read_and_process_rules("blacklist.txt"),
    pass_list: read_and_process_rules("passlist.txt"),
});

// 读取并处理规则文件
pub fn read_and_process_rules(file_path: &str) -> Vec<(String, String)> {
    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader
                .lines()
                .filter_map(|line| line.ok())
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('/').collect();
                    if parts.len() >= 2 {
                        Some((
                            parts[0].replace(' ', ""),
                            parts[1].replace(' ', ""),
                        ))
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(_) => Vec::new(),
    }
}
