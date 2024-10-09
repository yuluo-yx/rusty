# Cargo config

Cargo.toml 是 cargo 特有的项目数据描述文件，等同于 pom.xml 和 go.mod 

Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，等同于 go.sum

> 当编写的项目是一个 bin 类型时，上传 lock 文件，如果是 lib 不需要上传！

## 依赖配置

使用 cargo 工具的最大优势就在于，能够对该项目的各种依赖项进行方便、统一和灵活的管理。

在 Cargo.toml 中，主要通过各种依赖段落来描述该项目的各种依赖项：

基于 Rust 官方仓库 crates.io，通过版本说明来描述
基于项目源代码的 git 仓库地址，通过 URL 来描述
基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述
这三种形式具体写法如下：

```toml
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```
