# my_rust_road

my study Rust example code

# 第二章 语言精要 

## 2.2 语句与表达式

Rust中的语法可以分为两大类：语句(Statement)和表达式(Expression). 
语句是指要执行的一些操作和产生副作用的表达式。
表达式主要用于计算求值。

语句又分两种：声明语句(Declaration statement)和表达式语句(Expression statement). 

- 声明语句， 用于声明各种语言项(Item)，包括声明变量、静态变量、常量、结构体、函数等，以及通过extern和use关键字引入包和模块等
- 表达式语句，特值以分号结尾的表达式，此表达式求值结果将会被丢弃，并总是返回单元类型(). 

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0534a17bafb192952128c38b65fddc54

## 2.3 变量与绑定

通过let关键字来创建变量，这是Rust语言从函数式语言中借鉴的语法形式。let创建的变量一般称为绑定(Binding), 它表明了标识符(Identifier）和值(Value)之间建立的一种关联关系。

# 第9章 构建健壮的程序

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f0b4b4c8a9fe2be77739237378ea626f
