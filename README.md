# GitHub Proxy Rust

一个基于 Rust + Axum 的 GitHub 文件代理服务器，支持流式转发、白名单/黑名单/直连列表、自动大文件跳转等功能。

## 简介

GitHub Release、Archive 以及项目文件的加速代理，支持 Clone，适合加速 GitHub Releases、Raw、Archive、Gist 等多种资源。

## 使用

- 代理访问：`https://gh.example.com/<your-url>`
- 或：`https://gh.example.com/?q=<your-url>`
- 支持私有仓库访问：
  `git clone https://<your-user>:<your-token>@gh.example.com/<your-private-repo-url>`

以下均为合法输入示例（仅示例，文件可能不存在）：

- 分支源码：https://github.com/user/project/archive/master.zip
- Release 源码：https://github.com/user/project/archive/v0.1.0.tar.gz
- Release 文件：https://github.com/user/project/releases/download/v0.1.0/example.zip
- 分支文件：https://github.com/user/project/blob/master/filename
- Commit 文件：https://github.com/user/project/blob/32323232323232323232323232323232/filename
- Gist：https://gist.githubusercontent.com/user/32323232323232323232323232323232/raw/cmd.py

## 功能特性

- 支持 GitHub Releases、Raw、Archive、Gist 等多种资源代理
- 支持自定义白名单、黑名单、直连（跳转）列表
- 自动检测大文件并跳转到大文件加速服务器
- 支持 CORS，适合前端跨域请求
- 日志输出，便于调试
- 默认不输出前端，作为接口站使用，减小特征
- 支持通过 ENV 灵活设置参数（如 `SIZE_LIMIT`、`BIG_SERVER`）
- 支持外置访问规则（`whitelist.txt`、`blacklist.txt`、`passlist.txt`）

## 快速开始

### 1. 环境准备

- Rust 建议最新版
- 依赖见 `Cargo.toml`

### 2. 编译运行

```sh
cargo run --release
```

默认监听地址：`127.0.0.1:8000`

### 3. 配置说明

- `whitelist.txt`：白名单，格式为 `author/repo`，支持 `*` 通配
- `blacklist.txt`：黑名单，格式同上
- `passlist.txt`：直连跳转列表，格式同上
- 环境变量：
  - `SIZE_LIMIT`：单文件大小限制（字节，默认约 999GB）
  - `BIG_SERVER`：大文件跳转服务器（默认 https://ghfast.top/）

#### 访问控制规则

白名单生效后再匹配黑名单，passlist 匹配到的会直接跳转到大文件服务器。

规则格式（每行一个）：

```text
user1        # 封禁/允许 user1 的所有仓库
user1/repo1  # 封禁/允许 user1 的 repo1
*/repo1      # 封禁/允许所有名为 repo1 的仓库
```

### 4. 路由说明

- `/`：主页，支持 `?q=xxx` 跳转
- `/robots.txt`：禁止爬虫
- `/{*path}`：代理所有 GitHub 相关资源

## 依赖

- [axum](https://crates.io/crates/axum)
- [reqwest](https://crates.io/crates/reqwest)
- [tokio](https://crates.io/crates/tokio)
- [tower-http](https://crates.io/crates/tower-http)
- 详见 `Cargo.toml`

## 目录结构

```
├── src/
│   ├── main.rs         # 主入口
│   ├── config.rs       # 配置与常量
│   ├── access.rs       # 访问控制
│   ├── proxy.rs        # 代理与转发
│   └── routes.rs       # 路由与页面
├── whitelist.txt       # 白名单
├── blacklist.txt       # 黑名单
├── passlist.txt        # 跳转列表
├── Cargo.toml
└── README.md
```

## 贡献

欢迎 issue 和 PR！

## License

MIT
