# Rust 实战课 

Rust涉及的范围：

操作系统， 数据库， 游戏， 网络服务， web应用， 区块链， 物联网， 
嵌入式， 机器学习

Rust 安全和性能并重 

学习Rust的关键点

改变之前的学习习惯

所有权
借用和生命周期
类型系统和trait
突破抽象范式
Unsafe Rust 

## 语言核心概念讲解 
第一阶段： 帮助你进一步梳理一遍语法
第二阶段： 对Rust语言核心概念进行讲解
第三阶段： 侧重Rust异步编程

## 实战， 编写一个轻量级的异步web框架 
第一阶段, 构建异步框架基本骨架
第二阶段，为异步框架扩展功能
第三阶段， 使用异步框架和WebAssembly 技术开发一个小应用


# 1.1 Rust 语言学习观 

# 学习 Rust的十条建议

建议1，从整体出发， 不要让自己陷入到细节中
建议2，抛弃一次性学会的念头， 分层次递进式学习
建议3，和已知的知识建立联系
建议4，学会阅读源码， 从源码中学习
建议5，通过主题式阅读填补知识空白， async/await, python, c++, js, rust
建议6, 时刻把握Rust设计哲学
建议7，有意识地构建Rust语言的心智模型
建议8, 多分享读提问
建议9， 为开源做贡献， 锻炼自己
建议10， 阅读Rust编程之道 

# 1.2 Rust 语言概览

搞清楚三个问题： 

- Rust从哪里来？
- Rust 是什么?
- Rust要到哪里去？

Rust 从哪里来?

一门语言的诞生， 是为了解决一个问题。
2006年， 职业编程语言工程师， Graydon Hoare 

Graydon 对Rust语言的期望：
- 必须安全，不易崩溃。
- 不需要引入GC, 注重性能。
- 应该拥有广泛的特性， 让程序员写出易于维护、调试、
且更安全更高效的代码。

2020年5月15日， Rust稳定版发布 5周年

- 内存安全为第一准则
- 注重并发安全， 避免数据竞争 
- 持续提升性能
- 保持语言的高度一致性
- 语言必须有可见的实用性
- 注重开发体验和学习体验
- 现代化语言特性


Rust 是什么？

Rust 是新时代的C语言。

理由

- Rust语言是一门通用型语言。 
- Rust语言的内存安全方案针对的是C语言的不足。
- 安全且无缝沟通C语言
- Rust 是具有混合范式“面向过程”式的编程语言。
- 和C语言类型，担负了时代的使命。

1. Rust 语言是一门通用型语言 
- Rust 语言适合所有领域的绝大部分场景。 裸机、操作系统、网络服务、上层应用等。 
- 与其他语言横向比较，Rust拥有现代化语言特性， 应用范围覆盖到C/CPP/Java/Go/JavaScrip等领域。 

2. Rust 语言的内存安全方案针对的是C语言的不足
- 禁止堆空指针和悬垂指针进行解引用
- 读取未初始化的内存
- 缓冲区溢出
- 非法释放已释放或未分配的指针

3. 安全且无缝沟通C语言
- 通过C-ABI 零成本和C语言打交道
- 划分了Safe Rust 和 Unsafe Rust 


4. Rust 是具有混合范式的“面向过程”式的编程语言
- Rust包含了面向对象OOP,函数式FP 和泛型三种编程范式
- OOP和FP范式在Rust 语言中是作为语言特性而存在， 并非是抽象方式 
- Rust 让你专注于解决问题本身，而不受编程范式思想框架的干扰 

5 和C语言类型， 担负了时代的使命
- 操作系统遭遇发展瓶颈， Rust来拯救。 
- Rust是WASI(WebAssembly System Interface) 推广和普及的背后推手 
- 基于Rust实现的新语言如雨后春笋般出现 

Rust 到哪里去？
 
Rust将为各个领域的基础设施建设作出贡献， 未来也许在各个领域出现杀手级应用 

#1.3 语法面面观(1) ：词法结构

两大知识点：
- Rust语言版本说明
- Rust 词法结构

## Rust语言版本说明

- Rust语言的版本包括以下三个相互正交的概念: 
  - 语义化版本(Sem Ver, Semantic Versioning) 
  - 发型版本
  - Edition 版次 

语义化版本(Sem Ver, Semantic Versioning) 
  - 其格式为: 主版本号.次版本号.修订号, 依次用句号隔开。
  - 简单说一下语义版本号递增规则： 
    - 主版本号: 当做了兼容的API修改，
    - 次版本号: 当做了向下兼容的功能性新增
    - 修订号: 当做了向下兼容的问题修正 

发行版本

1. master -> Nightly 
2. beta -> Beta 
3. stable -> Stable 

Edition 版本
1. 2015 Edition 
2. 2018 Edition 
3. 2021 Edition 

## Rust 词法结构

内容包括: 
- 了解Rust编译过程 
- 六大词法结构


Rust 编译过程

Rust Code -分词-> Tokens -解析-> AST -降级-> HIR -降级-> MIR -优化-> LLVM IR -优化>  二进制 

Rust 词法结构
- 包含六大部分
  - 关键字(Keywords) 
  - 标识符(Identifier)
  - 注释(Comment)
  - 空白(Whitespace) 
  - 词条(Tokens) 
  - 路径(Path) 

关键字： 
  - 严格关键字(Strict)
  - 保留字(Reserved)
  - 弱关键字(Weak)

严格关键字: 

- as/break/const/continue/crate/if/else/struct/enum/true/false/fn/for/in/let/loop/
impl/mod/match/move/mut/pub/ref/return/self/Self/static/super/trait/type/unsafe/use/
where/while/async/await/dyn/main 

保留字：  
- abstract/become/box/do/final/macro/override/priv/typeof/unsized/virtual/yield/try 
- 被保留的关键字不代表将来一定会使用


弱关键字
- 2018 Edition: Union, 'static 
- 2015 Edition: dyn 
- 被保留的关键字不代表将来一定会使用

标识符 


