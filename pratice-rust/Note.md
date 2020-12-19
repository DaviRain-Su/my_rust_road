# 实战Rust

## Rust 语言学习观

### 学习Rust的十条最佳建议

- 从整体出发，不要让自己陷入到细节中去
  - Rust语言有哪些特性？
  - Rust设计哲学是什么？
  - Rust社区和生态如何？
- 抛弃一次性学会的念头，分层次递进式学习
  - 内存管理
  - 类型系统
  - 所有权系统
  - 编程范式
- 和你已知的知识建立联系
- 学会阅读源码，从源码中学习
- 通过主题式阅读填补知识空白
- 时刻把握Rust设计哲学
- 有意识地构建Rust语言的心智模型
- 多分享多提问多交流
- 为开源项目做贡献，锻炼自己
- 阅读 《Rust编程之道》

## Rust语言概览

搞清楚三个问题：

- Rust从哪里来？
- Rust是什么？
- Rust要到哪里去？



### Rust 从哪里来？

一门语言的诞生，是为了解决一个问题。

2006年，职业编程语言工程师，Graydon Hoare 

Graydone对Rust语言的期望是：

- 必须安全，不易崩溃
- 不需要引入GC，注重性能
- 应该拥有广泛的特性，让程序员写出易于维护，调试，且更安全更高效的代码。

Rust Logo 承载了创造者对Rust语言的预期

2020年5月15日 Rust稳定版发布 5周年

- 内存安全为第一准则
- 注重并发安全，避免数据竞争
- 持续提升性能
- 保持语言的高度一致性
- 语言必须由可见的实用性
- 注重开发体验和学习体验
- 现代化语言特性
- 拥抱开源

### Rust是什么

Rust是新时代的C语言 

理由

- Rust语言是一门通用型语言
- Rust语言的内存安全方案针对的是C语言的不足
- 安全且无缝沟通C语言
- Rust是具有混合范式的面向过程式的编程语言
- 和C语言类似，担负了时代的使命

Rust语言是一门通用型语言

- Rust语言适合所有领域绝大部分场景，裸机，操作系统，网络服务，上层应用等
- 与其他语言横向比较，Rust拥有现代化语言特性，应用范围覆盖到C/Cpp/Java/Go/JavaScript等领域

Rust语言的内存安全方案针对的是C语言的不足

- 禁止对空指针和悬垂指针进行解引用
- 读取未初始化的内存
- 缓冲区溢出
- 非法释放已经释放或未分配的指针

安全且无缝沟通C语言

- 通过C-ABI 零成本和C语言打交道
- 划分了Safe Rust和Unsafe Rust 

Rust是具有混合范式的面向过程式的编程语言

- Rust包含了面向对象、函数式和泛型三种编程范式
- OOP和FP范式在Rust语言中作为语言特性而存在，并非是抽象方式
- Rust让你专注于解决问题本身，而不受编程范式思想框架的干扰

和C语言类似，担负了时代的使命

- 操作系统遭遇发展瓶颈，Rust来拯救
- Rust是WASI推广和普及的背后推手
- 基于Rust实现的语言如雨后春笋发生

### Rust到哪里去？

Rust将为各个领域的基础设施建设做出贡献，未来也许在多个领域出现杀手级应用



## 语法面面观

### 词法结构

两大知识点

- Rust语言版本说明
- Rust词法结构

#### Rust语言版本说明

- Rust语言的版本包括以下三个相互正交的概念：
  - 语义化版本(Sem Ver, Semantic Versioning)
  - 发行版本
  - Edition版次



##### 语义化版本(Sem Ver, Semantic Versioning)

- 其格式为：主版本号.次版本号.修订号， 依次用句号隔开
- 简单说一下语义化版本号递增规则：
  - 主版本号：当做了不兼容的API修改
  - 次版本号：当做了向下兼容的功能性新增
  - 修订号：当做了向下兼容的问题修正



##### 发行版本

- Master -> Nightly
- beta -> Beta
- Stable -> Stable 



##### Edition 版次

- 2015 Edition
- 2018 Edition (1.31.*)
- 2021 Edition

#### 词法结构

##### 内容：

- 了解Rust编译过程
- 六大词法结构

##### Rust编译过程

- Rust Code(UTF-8) 分词
- Tokens 解析
- AST 降级
- HIR 降级 版次的概念在这一层就没有了
- MIR 优化
- LLVM IR  优化
- 0/1

##### Rust 词法结构

- 包含六大部分
  - 关键字 keywords
  - 标识符 Identifier
  - 注释 Comment
  - 空白 Whitespace
  - 词条 Tokens
  - 路径 Path

###### 关键字

- 严格关键字 Strict
  - As, break, const, contunue, crate, if, else, struct, enum, true, false, fn, for, in, let, loop, impl, mod, match, move, mut, pub, ref, return, self, Self, static, super, trait, type, unsafe, use, where, while, async, await, dyn, main
- 保留字 Reserved 
  - Abstratct, become, box, do, final, macro, override, priv, typeof, unsized, virtual, yield, try
  - 被保留的关键字不代表将来一定会使用
- 弱关键字 Weak
- 2018 Edition: union, 'static
  - 2015 Edition: dyn
  - 被保留的关键字不代表将来一定会使用

###### 标识符

```rust
let thinking = "thinking";
let thinging123_ = "thinking 123";

// error : invalid suffix 'thinking' for integer literal
// let 321_thinking = "thinking";

// ok 
let _321_thinking = "thinking";

// non-ascii ident
// RFC: https://github.com/rust-lang/rfcs/blob/master/text/2457-non-ascii-idents.md
// error: unknown start of token: \u{1f914}
let 🐯 = "thinking ";
```



##### 注释

```rust
pub mod outer_module {
  //! - 模块级文档注释，置于模块头部
  //!! - 模块级文档注释, 
}
```



###### 空白

- Rust 中空白符包括: \n, \t, tab 等
- 任何形式的空白符在Rust中只用于分隔标记，没有语义意义。



###### 词条

- 语言项 item 基于的语言项

- 块 Block

- 语句 Stmt

- 表达式 Expr

- 模式 Pattern

- 关键字 Keyword

- 标识符 ident

- 字面量 Literal

- 生命周期 Lifetime

- 可见性 Vis

- 标点符号 Punctuation

- 分隔符 delimiter

- 词条树 Token Tree

- 属性 Attribute

  

###### 路径

- 

- 
- 

