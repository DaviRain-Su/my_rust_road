# Chapter2 Rust初体验
- 下载和安装Rust
- 一个简单的函数
- 编写和运行单元测试
- 处理命令行参数
- 一个简单的Web服务器
- 并发
    - 到底什么是曼德罗布特集合
    - 解析成对的命令行参数
    - 像素到复数的映射
    - 绘制集合
    - 写出图像文件
    - 并发的曼德罗布特程序
    - 运行曼德罗布特绘图器
    - 安全无形

## 下载和安装

使用rustup， rust安装包的工具。
rustup update 升级rust版本

- cargo 是Rust的编译管理器、包管理器以及通用工具，可以使用Cargo来创建新项目、构建和运行程序，以及管理代码所依赖的外部库。
- rustc是Rust编译器。通常是通过Cargo来调用编译器，但有时候也需要直接调用它。
- rustdoc是rust的文档工具。如果在代码注释中以适当格式写了文档，那么rustdoc可以基于他们生成格式化的html。与rustc类似，通常也让Cargo来帮助运行rustdoc。

```
cargo new --bin proj-name # 用来创建一个二进制可执行文件的项目
cargo run # 编译并运行项目
cargo clean # 清理生成的项目
cargo build # 编译项目
```



### Cargo binary

二进制目标文件是可以在编译后运行的可执行程序。默认二进制文件名是src/main.rs. 默认为包的名称。其他二进制文件存储在src/bin目录中，可以在Cargo.toml中的`[[bin]]`表中定制每个二进制文件的位置。

二进制文件可以使用包的库的公共API，也可以与Cargo.toml中定义的`[dependencies]`链接。

你可以使用cargo run 命令和--bin `<bin-name>` 选项运行单个二进制文件。Cargo install可用于将可执行文件复制到一个公共位置。

```
# Example of customizing binaries in Cargo.toml
[[bin]]
name = "cool-tool"
test = false
bech = false


[[bin]]
name = "frobnicator"
required-features = ["frobincat"]
```