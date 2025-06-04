# [rust](https://www.rust-lang.org/zh-CN/)-yew

Base on rust & yew

# ⏬ Dependencies

- [Yew](https://yew.rs/zh-Hans/docs/getting-started/build-a-sample-app)
- [rust-by-example](https://github.com/rust-lang/rust-by-example?tab=readme-ov-file)
- [Rust 在前端领域的应用](https://juejin.cn/post/7076354498691596325)

# 📖 Introduction

- Create project

```bash
cargo new --bin yew-app
```

- Run project

```bash
cd yew-app

cargo web start
```

# ✨ Others

Rustup 既是 Rust 安装器又是版本管理工具

- 下载 Rustup 并安装 Rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 获取最新版本 Rust `rustup update`

Cargo：Rust 的构建工具和包管理器

- 查看版本 `cargo --version`
- 构建项目 `cargo build`
- 运行项目 `cargo run`
- 测试项目 `cargo test`
- 将库发布到 crates.io `cargo publish` _类似 npm 库_

