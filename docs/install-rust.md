# 安装 Rust

书籍资料：https://course.rs/about-book.html

## 资源下载

Microsoft C++ Build Tools [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)

rustup-init：https://www.rust-lang.org/learn/get-started

> 安装完成后，Rust 所需的 msvc 命令行程序需要手动添加到环境变量中，否则安装 Rust 时 `rustup-init` 会提示未安装 Microsoft C++ Build Tools，其位于：`%Visual Studio 安装位置%\VC\Tools\MSVC\%version%\bin\Hostx64\x64`（请自行替换其中的 %Visual Studio 安装位置%、%version% 字段）下。
>
> 注意调整安装目录位置！

在双击 rustup-init 之后，一路回车，当出现如下内容时，即代表安装成功！

```shell
stable-x86_64-pc-windows-msvc installed - rustc 1.81.0 (eeb90cda1 2024-09-04)                                                                                                                                                                                                                                                  Rust is installed now. Great!  
```

## 检测

> $ rustc -V 
>
> rustc 1.56.1 (59eed8a2a 2021-11-01) 
>
> $ cargo -V 
>
> cargo 1.57.0 (b2e52d7ca 2021-10-21)

## 更新

> rustup update

## 卸载

> rustup self uninstall
