# Rust Project1: The Rust toolsbox

Task: 创建内存的键值存储区，该存储区通过简单的测试并相应命令行参数

Goals:
- 安装Rust compiler , tools
- 学习贯穿本课程的项目结构
- 使用 cargo init / run / test / clippy / fmt
- 学习如何从crates.io中找和导入crates
- 为键值存储定义approprate数据类型

Topics: testing, the clap crate, CARGO_VERSION etc, the clippy and rustfmt tools

Extensions: the structopts crate

## 引言
在这个项目中，将创建一个简单的内存中键值存储，他将字符串映射到字符串，并通过一些测试和
响应命令行参数。这个项目的重点是一个典型的Rust项目中的工具和设置。
如果这对你来说听起来很基本，那么请不管怎么样做这个项目，因为他讨论了一些将在整个课程中使用的一般模式。

## 项目规格

cargo 项目, kvs, 构建一个命令行键值存储客户端，被称为kvs, 该客户端调用一个称为kvs的
库。
kvs可执行文件支持以下命令行参数：
- kvs set <KEY> <VALUE>
Set the value of a string key to a string
- kvs get <KEY>
Get the string value of a given string key
- kvs rm <KEY>
remove a given key
- kvs -v 
Print the version

kvs 库包含一个类型为KvStore, 它支持以下方法：
- KvStore::set(&mut self, key : String, value: String)  
Set the value of a string key to a string
- KvStore::get(&self, key: String) -> Option<String> 
Get the string value of the a string key, if the key does not exist, return None.
- KvStore::remove(&mut self, key: String) 
Remove a given key

KvStore类型在内存中存储值，因此命令行客户端只能打印version, 当从命令行运行， get/set
/rm 命令将返回一个未实现的错误。未来的项目，将在磁盘上存储，并有一个可工作的命令行接口。

github Link : https://github.com/pingcap/talent-plan/tree/master/courses/rust/projects/project-1

你讲在你自己的git仓库中使用欧冠自己的的Cargo 项目完成这个项目的工作，你将从本课程的源代码库中导入项目的测试用例

注意，在该存储库中，所有与本课程相关的内容都位于rust子目录中，你可以忽略其他目录。

本课程中的项目包含库和可执行文件，他们是可执行的，因为我们正在开发一个可以运行的应用程序。他们是库，因为提供的测试用例必须连接到他们。


```
目录结构

├── Cargo.toml
├── src
│   ├── bin
│   │   └── kvs.rs
│   └── lib.rs
└── tests
    └── tests.rs
```

你可以使用cargo new --lib , cargo init --lib（在一个clean directory)，你可能在需要在同一个目录中初始化git存储库。
