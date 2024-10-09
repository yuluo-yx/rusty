# 编译和运行 rust 

## cargo run

cargo 在创建项目时，默认创建的是 bin 项目，非 lib （依赖库） 项目。

```shell
cargo hello_world
```

运行上述命令时，默认就会生成一个 hello_world 的项目。之后进入项目，运行

```rust
cargo run
```

就会在当前目录下编译生成 target 文件，后执行在控制台看到 hello world 的输出！

## 手动编译

上述是 cargo 自动编译并运行，下面手动编译

```rust

# 编译
cargo build

# 运行
./target/debug/hello_world.exe
```

在执行二进制文件时，会出现一个 debug 字段，这是因为当前在以 debug 模式执行，代码编译速度非常快。但是运行速度慢。在此模式下，rust 编译器不会做任何优化。

编译执行速度更快的代码：

```rust
# 运行
cargo run --release

# 编译运行
cargo build --release
./target/release/hello_world.exe
```

## 检验代码

check 是开发中最常用的命令，执行速度非常快，主要目的是快速的检查一下代码能否编译通过。在项目变大之后，节省 build 的时间。

```rust
cargo check
```
