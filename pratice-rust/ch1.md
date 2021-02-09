
## 第一章 Rust语言基础

### 01 课程介绍

为什么要学习Rust语言？
- 对自我的一次全面提升
    rust吸收了很对语言的优秀特性
    面向对象 函数式 借鉴了很多语言
    所有权 类型系统 简洁之美
    对其他语言的一次完美的总结
    语言背后的共同性 
    不在纠结用哪个语言了
    不考虑用哪个范式了
    学习其他语言也会有加速度

- 应用领域  
    - 操作系统
    - 数据库 
    - 游戏 
    - 网络服务 
    - web应用  
    - 区块链 
    - 物联网
    - 嵌入式 
    - 机器学习

- 职业生涯
- 抓住了时代的脉搏
    - Fortan Lisp Cobol
    - C Ada C++ Simual Problog 
    - Python Ruby Haskell Elang
    - Rust Swift Dart Typescript
    - WASI -- POSXI WebAssembly
    - Fuchsia, Redox Tock
    - 区块链领域
    - 安全和性能并重 

学习Rust的关键点和难点
- 改变之前的学习习惯 Rust 编译器检查非常安全 学习编译器的检查规则提示和编译器斗争。建立和编译器一直的心智模型。改变自己的学习习惯。建立基本的学习认知。

五道门槛
- 所有权机制
- 借用和生命周期
- 类型系统与triat
- 突破抽象范式
- unsafe rust

学完这门课我能学到什么

语言核心概念讲解
- 第一阶段 帮助你进一步梳理一遍语法
- 第二阶段 对Rust语言核心概念进行讲解
- 第三阶段 侧重于Rust异步编程

实战 编写一个轻量级的异步web框架
第一阶段 构建异步框架基本骨架
第二阶段 为异步框架扩展功能
第三阶段 使用异步框架和webAssembly技术开发一个小应用

对rust语言和rust异步实战有了一个基本的认知

### 02 内容综述

- 第一章：Rust 语言基础
    - Rust语言及其生态介绍
    - Rust语言学习观
    - 梳理Rust语法，区别Rust语言和其他语言的异同

- 第二章：Rust语言核心概念 攻克难关
    - 所有全机制
    - 类型系统
    - 借用和生命周期
    - 抽象方式
    - Rust元编程 
    - Unsafe Rust

- 第三章： Rust异步编程基础
    - 系统性掌握Rust异步编程概念 
        - 异步编程模型
        - 异步IO
    - 常用的异步运行时
        - async-std
        - tokio
        - bastion

- 第四章: 构建自己的异步web框架 
    - 从0开始构建一个异步web框架
    - 实战重点

- 第五章： 为异步框架扩展功能 进一步丰富框架的功能
    - 内置http client 支持
    - 支持TLS
    - 内置ORM支持
    - 内置Cache和Redis支持

- 第六章： 使用异步框架开发简单的todolist应用构建自己的异步web框架 
    - 异步web框架的应用

- 第七章：尾声
    - 帮助大家回顾整个课程中的重点和心得 


### 学习方法推荐与课程组织逻辑

- 内容包括
    - 我的学习方法介绍
        - 层次学习法 间断学习法
        - 你不能直接吃第五个馒头
        - 完事皆需要过程
        - 过程也分阶段
        - 阶段即结构和层次
        - 刷墙式学习法
            - 墙面基层处理（你要自己看过一遍Rust语法）
            - 刷界面漆 (第一章： 帮你梳理语法)
            - 石膏找平 （第二章： 帮你理解核心概念）
            - 刮腻子 （层层递进）
            - 打磨 
            - 刷底漆
            - 刷面漆

        
    - 本章内容的组织逻辑


### 03 Rust语言学习观

- 学习Rust的十条建议
    - 1.从整体出发，不要让自己陷入到细节中去
        - Rust语言有哪些特性
        - Rust设计哲学是什么？
        - Rust社区和生态如何？
    - 2 抛弃一次性学会的念头，分层次递进式学习
        - 内存管理
        - 类型系统
        - 所有权系统 编程范式
    - 3 和你已知的知识建立联系
    - 4 学会阅读源码，从源码中学习
    - 5 通过主题式阅读填补知识空白
    - 6 时刻把握Rust设计哲学
    - 7 有意识的构建Rust语言的心智模型
    - 8 多分享多提问多交流
    - 9 为开源项目做贡献，锻炼自己
    - 10 阅读Rust编程之道

### 04 Rust语言概览
    搞清楚三个问题：
        - Rust从哪里来？
            - 一门语言的诞生，都是为了解决一个问题
            - 职业编程语言工程师 Graydon hoare 
            - Rraydon 对Rust语言的期望
                - 必须安全，不易崩溃
                - 不需要引入GC， 注重性能
                - 应该拥有广泛的特性，让程序员写出易维护、调试、且安全更搞笑的代码
            - Rust log 承载了创作者对Rust的期望
            - 2020年5月15日 Rust稳定版发布5周年
                - 内存安全为第一准则
                - 注重并发安全，避免数据竞争
                - 持续提升性能
                - 保持语言的高度一致性
                - 语言必须有可见的实用性
                - 注重开发体验和学习体验
                - 现代化语言特性
                - 拥抱开源社区

        - Rust是什么？
            - Rust是新时代的C语言 历史地位是一样的
            - 理由
                - Rust是一门通用性语言
                    - Rust语言适合所有领域的绝大部分场景，裸机、操作系统、网络服务、上层应用等
                    - 与其他语言横向比较，Rust拥有现代语言特性、应用范围覆盖到C/CPP/Java/Go/JavaScript等语言领域

                - Rust语言的内存安全方案针对的是C语言的不足
                    - 禁止对空指针和悬垂指针进行解引用
                    - 读取未初始化的内存
                    - 缓冲区溢出
                    - 非法释放已经释放或未分配的指针
            
                - 安全且无缝沟通C语言
                    - 通过C-ABI零成本和C语言打交道
                    - 划分了Safe Rust和Unsafe Rust(依然进行安全检查)
                
                - Rust是具有混合范式的“面向过程”式的编程语言
                    - Rust包含了面向对象（OOP）、函数式（FP）和泛型三种编程范式
                    - OOP和FP范式在Rust语言中作为语言特性而存在，并非抽象方式
                    - Rust让你专注于解决问题本省，而不受编程范式思想框架的干扰

                - 和C语言类型，担负了时代的使命
                    - 操作系统遭遇发展瓶颈，Rust来拯救
                    - Rust是WASI（WebAssembly System Interface） 推广和普及的背后推手
                    - 基于Rust实现的新语言如雨后春笋般冒出
            
        - Rust到那里去?
           - Rust 将为各个领域的基础设施做出贡献，未来也许会在多个领域出现杀手级应用。

        - 小结
            - 安全 并发 性能 三连接的语言
            - safe rust && unsafe rust
            - Rust是新时代的C语言
        
    
### 05 语法面面观： 词法结构

- 两大知识点
    - Rust语言版本说明
    - Rust词法结构

- Rust 语言的版本说明
    - Rust语言的版本包括以下三个相互正交的概念
        - 语义化版本(Sem Ver, Semantic Versioning)
        - 发行版本
        - Edition 版次

    - 语义化版本(Sem Ver, Semantic Versioning)
        - 其格式为：主版本号.次版本号.修订号，依次用句点隔开。
        - 简单说一下语义版本号递增规则：
            - 主版本号：当做了不兼容的API修改
            - 次版本号：当做了向下兼容的功能性新增
            - 修订号：当做了向下兼容的问题修正

    - 发行版本
        - master -> Nightly
        - beta -> Beta
        - stable -> Stable
    
    - Edition 版次
        - 2015 Edition
        - 2018 Edition 
        - 2021 Edition
    
- 词法结构
    - 了解Rust编译过程 （词法分析 语法解析）
        - 分词 Rust Code
        - 解析 Tokens 
        - 降级 AST （Rust这里的一个特色不会在这里直接生成字节码或者机器码）
        - HIR 降级 （这里版次的概念已经没有了）对代码进行类型检查，方法查找
        - MIR 优化 借用检查 代码生成  优化 单态化
        - LLVN IR 优化
        - binary

    - 六大词法结构
        - 关键字 (keywords)
            - 严格关键字 strict
                - as break const continue crate if else struct enum true false fn for in let loop impl mod match move mut pub ref return  self Self static super trait type unsafe use where while async await dyn main
            - 保留字 reserved
                - abstract become box do final macro override priv typeof unsized virtual yield try
                - 被保留的关键字不代表将来一定会被使用

            - 弱关键字 weak
                - 2018 Edition: union, 'static
                - 2015 Edition: dyn
                - 被保留的关键字不达标一定会被使用
        
        - 标识符 (identifier)
            ```Rust 
            let thinking = "thinking"
            let thinking123_ = "thinking 123";

            // let 321_thinking = "thinking"; // error
            let _321_thinking = "thinking";
            
            // let 😁 = "thinking"; // error
            ```
        - 注释 (Comment)
        - 空白 (whitespace)
        - 词条 (tokens)
        - 路径 (path)



    



### 06 语法面面观： 面向表达式（上）

### 07 语法面面观： 面向表达式 （中）

### 08 语法面面观： 面向表达式 （下）

### 09 语法面面观： 数据类型 （上）

### 10 语法面面观 数据类型 （下）

### 11 语法面面观： 函数与闭包 （上）

### 12 语法面面观： 函数与闭包 （中）

### 13 语法面面观： 函数与闭包 （下）

### 14 语法面面观： 模式匹配

### 15 语法面面观： 智能指针 （上）

### 16 语法面面观： 智能之后怎 （下）

### 17 语法面面观： 字符与字符串 （上）

### 18 语法面面观： 字符与字符串 （下）

### 19 语法面面观：集合容器（上）

### 20 语法面面观： 集合容器 (下)

### 21 语法面面观： 迭代器 (上)

### 22 语法面面观： 迭代器 （下）

### 23 语法面面观： 模块

### 24 语法面面观： Cargo包管理器 （上）

### 25 语法面面观: Cargo包管理器 （下）

### 26 语法面面观： 实际项目的组织结构 （上）

### 27 语法面面观： 实际项目的组织结构 （下）

### 28 语法面面观： 定义自己的crate （上）

### 29 语法面面观： 定义自己的crate （中）

### 30 语法面面观： 定义自己的crate （下）

### 31 作业 第二章预告


