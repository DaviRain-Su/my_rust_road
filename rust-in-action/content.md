Dear Reader,

Thanks for taking a chance and buying this early release book on the Rust programming language and the internals of computer systems. I hope that you'll be rewarded with a fun, informative read!

亲爱的读者，
感谢你冒险购买这本提前发行的Rust编程语言和计算机系统内部的书籍。希望你能在阅读中获得乐趣和信息的回报!

Part 1: Rust Language Distinctives will provide a quick-fire introduction to the  Rust language by working through projects that begin to introduce concepts that are expanded upon later in the book, such as implementing a File API.

第1部分。Rust Language Distinctives 将通过一些项目来快速介绍Rust语言，这些项目将开始介绍本书后面扩展的概念，例如实现文件API。

Part 2: Systems Programming from the Ground Up (Almost) will shift your focus  towards the computer. You will learn how data is represented at various levels  between the application, operating system and hardware. Rust language features will be introduced as required.

第2部分：从零开始的系统编程（几乎）将把你的重点转向计算机。您将学习如何在应用程序、操作系统和硬件之间的不同层次上表示数据。Rust语言特性将根据需要进行介绍。

A note about Chapter 6: This chapter should be considered a work in progress. It  covers lots of ground. Readers progress from learning what a pointer is to  benchmarking memory allocations with system utilities for Linux. Some of the  explanations are terse... perhaps too terse.

关于第6章的说明：本章应视为正在进行的工作。它涵盖了很多内容。读者从学习什么是指针到使用Linux的系统实用程序对内存分配进行基准测试。有些解释很简洁......也许太简洁了。

We believe that the content is interesting and relevant to professional programmers,  but are considering how it should be expanded and refactored to be most relevant to the book's readers. The author and the editorial team are very interested in your views  about what works and what needs more work.

我们相信这些内容是有趣的，并且与专业程序员相关，但正在考虑如何扩展和重构，以使其与本书的读者最为相关。作者和编辑部非常希望得到您的意见，哪些是可行的，哪些是需要进一步改进的。

Part 3: Concurrent and Parallel Programming will walk through the tools that  computers offer to get work done in parallel. Its primary goal will be to explain how  multiple threads can cooperate and coordinate. A large example building a graph that  can be distributed in multiple ways will be extended throughout several chapters.

第3部分：并发和并行编程将介绍计算机提供的并行完成工作的工具。其主要目标将是解释多线程如何合作和协调。一个建立一个可以以多种方式分布的图的大例子将在几个章节中扩展。

Finally, Part 4: Extending Applications Up and Out will show Rust being used to  build up software components to extend large software projects. These projects demonstrate systems programming “with the system” rather than systems  programming “of the system”. Part 4 will deal with the complexities of handling  multiple CPU/OS pairs from the same code base.

最后，第4部分：向上和向外扩展应用程序将展示Rust被用来构建软件组件以扩展大型软件项目。这些项目展示了 "与系统一起 "的系统编程，而不是 "系统的 "系统编程。第4部分将处理从同一代码库处理多个CPU/OS对的复杂性。

# Biref contents

## 1 Introducing Rust

# Part1: Rust Language Distinctives

## 2 Language Foundations

## 3 Compound Data Types

## 4  Lifetimes, Ownership and Borrowing

# Part 2: Demystifying Systems Programming

## 5 Data in Depth

## 6 Memory

## 7 Files and Storage

## 8 Networking

## 9 Time and Time Keeping

## 10 Process, thread and Containers

## 11 Kernel

## 12 Signals, Interrupts and Exceptions



# 1 Introducing Rust

In this chapter:

• Highlighting some great features of the language and its community

• Exposing you to Rust’s syntax

• Introducing the goals of the project

• Discussing where Rust might be useful and when to avoid it

• Building your first Rust program

• Explaining how Rust compares to object-oriented and wider languages



在本章中．

- 突出语言和社区的一些伟大的特点。
- 让你了解Rust的语法
- 介绍项目的目标
- 讨论Rust在哪些方面可能有用，何时应避免使用它
- 建立您的第一个Rust程序
- 解释Rust如何与面向对象和更广泛的语言进行比较。

Welcome to Rust, the programming language that rewards your curiosity. Once you  scratch the surface, you will find a programming language with unparalleled speed and  safety that is still comfortable to use. Learning the language can be challenging, but the  rewards can be immense.

欢迎来到Rust，这门编程语言会奖励你的好奇心。一旦你从表面上看，你会发现这是一种具有无与伦比的速度和安全性的编程语言，而且使用起来还很舒服。学习这门语言可能是具有挑战性的，但回报也是巨大的。

Rust has established a strong following. In Stack Overflow’s annual developer survey,  Rust has won “most loved programming language” in 2016, 2017, 2018 and 2019. The distinction is awarded to the language that has the highest proportion of current  developers in the language who want to continue using it.

Rust已经建立了强大的追随者。在Stack Overflow的年度开发者调查中，Rust在2016年、2017年、2018年和2019年获得了 "最受喜爱的编程语言"。该殊荣授予当前该语言的开发者中希望继续使用该语言的比例最高的语言。

## 1.1 How is Rust used?

The language has proven its ability to build powerful, reliable software.

Large technology leaders have adopted Rust:

- Amazon Web Services runs its serverless computing offerings, AWS Lambda and AWS Fargate with Rust 

- Dropbox rebuilt its storage backend in Rust during 2015-2016, which manages  exabytes of storage 
- Cloudflare develops many of its services with Rust, including its public DNS,  serverless computing and packet inspection offerings 
- Google develops portions of the Chrome OS4 and Fuchsia5 operating systems in  Rust.
- Microsoft writes components of its Azure platform in Rust, including a security 
  daemon for its Internet of Things (IoT) service
- Mozilla uses Rust enhance the Firefox web browser. The browser project contains  15 million lines of code. Mozilla’s first two Rust-in-Firefox projects, its MP4  metadata parser and text encoder/decoder have led to performance and stability  improvements.
- Samsung, via its subsidiary SmartThings, uses Rust in its “Hub”. The Hub is the a  smart devices firmware backend for its Internet of Things (IoT) service.
- Oracle has developed a container runtime in Rust, to overcome problems that they  perceived with the Go reference implementation

Rust is also productive enough for fast-moving startups to deploy it:

- Sourcegraph uses Rust to serve syntax highlighting across all of its languages 
- Figma employs Rust in the performance-critical components of its mutliplayer  server
- Parity develops its client to the Ethereum blockchain in Rust

该语言已经证明了其构建强大、可靠软件的能力。

大型技术领导者已经采用了Rust。

- 亚马逊网络服务使用Rust运行其无服务器计算产品，AWS Lambda和AWS Fargate。

 - Dropbox在2015-2016年期间用Rust重建了存储后端，它管理了  内存 

 - Cloudflare利用Rust开发了许多服务，包括其公共DNS、无服务器计算和数据包检测产品。

 - Google用Rust开发了Chrome OS4和Fuchsia5操作系统的部分内容。

- 微软用Rust编写了其Azure平台的组件，包括一个安全系统守护进程，为其物联网（IoT）服务提供服务。

- Mozilla使用Rust增强Firefox网络浏览器。该浏览器项目包含1500万行代码。Mozilla的前两个Rust-in-Firefox项目，其MP4元数据解析器和文本编码器/解码器已经使性能和稳定性得到了改善。

- 三星通过其子公司SmartThings，在其 "Hub "中使用了Rust。Hub是其物联网（IoT）服务的智能设备固件后端。

- 甲骨文在Rust中开发了一个容器运行时，以克服他们认为的Go参考实现的问题。

Rust的生产力也足以让快速发展的初创公司部署它。

- Sourcegraph使用Rust为其所有语言提供语法高亮服务

- Figma在其多玩家服务器的性能关键组件中采用了Rust。

 - Parity在Rust中开发了Ethereum区块链的客户端





