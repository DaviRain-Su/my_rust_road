# Tokio Tourtials

## Introduction

本教程将带你一步步了解构建Redis客户端和服务器的过程。我们将容Rust异步编程的基础知识开始，并从哪里开始建立。我们将实现Redis命令的一个子集，但将全面了解Tokio. 

### Mini-Redis

你将在本教程中构建的项目在Github 上以Mini-Redis的形式提供。Mini-Redis是以学习Tokio为主要目标而设计的，因此评论非常好。但这也意味着Mini-Redis缺少了一些你在真正的Redis库中想要的功能。你可以在crate.io上找到生产就绪的Redis库。

我们将在教程中直接使用Mini-Redis,这使得我们可以在教程中使用Mini-Redis的部分功能，然后再在后面的教程中实现它们。

### 获得帮助

在然和时候，如果你被卡住了，你可以随时在Discord或Github讨论中获得帮助，不要担心问“初学者”的问题。我们都是从某个地方开始的，并且很乐意提供帮助。

### 先决条件

读者应该已经熟悉了Rust, Rust这本书是一个很好的入门资源。

虽然不是必须的，但使用Rust标准库或者其他语言编写网络代码的一些经验可能会有所帮助。

不需要预先具备Redis的知识。

### Rust

在开始之前，你应该确保你已经安装了Rust工具链并准备好了，如果你没有，最简单的安装方法是使用rustup.

本教程至少需要rust 1.45.0版本，但建议使用最新的Rust稳定版本。

要检查你的计算机上是否安装了Rust, 请运行一下程序。

```shell
$ rustc --version
```

你应该看到像rustc 1.46.0 (04448afe3 2020-0802)这样的输出。

### mini-redis server 

接下来，安装Mini-Redis服务器，这将用于测试我们的客户端。

```shell
$ cargo install mini-redis 
```

通过启动服务器确保它已经成功安装。

```rust
$ mini-redis-server
```

然后尝试使用mini-redis-cli获取key foo 

```shell
$ mini-redis-cli get foo
```

你应该看到 nil

准备好了

就这样，一切都准备好了，进入下一页，编写你的第一个异步Rust应用程序。

