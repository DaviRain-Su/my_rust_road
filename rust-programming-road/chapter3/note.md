# 类型系统

Q1: 类型是什么？

A1: 所谓**类型**，就是对表示信息的值进行的细粒度的区分。比如整数，小数、文本等，粒度再细一点，就是布尔值、符号整型值、无符号整型值、单精度浮点数、双精度浮点数、字符和字符串，甚至还有各种自定义的类型。

在类型系统中，一切皆类型。基于类型定义的一些列组合、运算和转换方法，可以看做类型的行为。

Q2: 类型系统的作用是什么？

A2: 
**排查错误**。很多语言都会在编译期货运行期进行类型检查，以排查违规行为，确保程序正确行为。如果程序中有类型不一致的情况，或有未定义的行为发生，则可能导致错误的产生。尤其是对静态语言来说，能在编译期排查出错误是一个很大的优势，这样可以及早地处理问题，而不必等到运行后系统崩溃了再解决。

**抽象**。类型允许开发者在更高层面进行思考，这种抽象能力有助于强化编程规范和工程化系统。

**文档**。在阅读代码的时候。明确的类型声明考科一表明程序的行为

**优化效率**。这一点针对静态编译语言来说，在编译期可以通过类型检查来优化一些操作，节省运行时的时间。

**类型安全**。
    **类型安全的语言可以避免类型间无效的计算**
    **类型安全的语言可以保证内存安全**
    **类型安全的语言可以避免语义上的逻辑错误**

Q3: 类型系统的分类

A3: 
按照运行时期来分类，在编译期进行类型检查的语言属于**静态类型**，在运行期进行类型检查的语言属于**动态类型**。 

按照是否允许类型可以进行自动的隐式类型转换，如果一门语言不允许类型的自动隐式转换，在强制转换前不同的类型无法进行计算，则该语言属于**强类型**，反之属于**弱类型**。

**静态类型的语言能在编译期对代码进行静态分析**，依靠的就是类型系统。C，C++，在编译期不会检查数组是否越界访问，这属于类型系统中未定义的行为，所以它们不是类型安全的语言。

**Rust语言在编译期就能检查出数组是否越界访问**，并给出警告，让开发者及时修改，如果开发者没有修改，那么在运行时也会抛出错误并退出线程，而不会因此去访问非法的内存，从而保证了运行时的内存安全，所以**Rust是类型安全的语言**。

强大的类型系统也可以对类型进行自动推导，因此一些静态语言在编写代码时候不用显式指定具体的类型，比如**Haskell就被称为隐式静态类型**。Rust中大部分的地方还是需要显式的指定类型的，类型是Rust的一部分，因此**Rust属于显式静态类型**。

在其他语言中作为基本类型的整数，字符串、布尔值等，在Ruby和Pyhton语言中都是对象，实际上，也可以将对象看做类型，Ruby和Python语言在运行时通过一种名为Duck Typing的手段来进行运行时类型检查，以保证类型安全。在Ruby和Python语言中，对象之间通过消息进行通信，如果对象可以相应消息，则说明该对象就是正确的类型。

对象时什么样的类型，决定了他有什么样的行为，反过来，对象在不同上下文中的行为，也决定了它的类型，这其实就是一种多态性。


Q4: 什么是多态类型系统？

A4: 如果一个类型系统允许一段代码在不同的上下文中具有不同的类型，这样的类型系统就叫做多态类型系统。对于静态类型的语言来说，多态性的好处是可以在不影响类型丰富的前提下，为不同的类型编写通用的代码。

Q5: 现代编程语言包含的三种多态形式
A5: 参数或多态(Parametric polymorphism), Ad-hoc多态(Ad-hoc polymorphism), 子类型多态(Subtype polymorphism).

按照多态发生的时间来分，可以分为静多态(Static Polymorphism)和动多态(Dynamic Polymorphism). 静多态发生在编译器，动多态发生在运行时。
参数化多态和Ad-hoc多态一般是静多态，子类型多态一般是动多态。
静多态牺牲灵活性获取性能，动多态牺牲性能获取灵活性。
动多态在运行时需要查表，占用较多空间，所以一般情况下都使用静多态。
Rust语言同时支持静多态和动态，静多态就是一种零成本抽象。

参数化多态实际就是值指泛型。泛型使得语言极具表达力，同时也保证静态类型安全。

Ad-hoc多态也叫做特定多态，Ad-hoc多态是指同一种行为，在不同的上下文中会响应不同的行为实现。Haskell语言中使用Typeclass来支持Ad-hoc多态，Rust受Haskell启发，使用triat来支持Ad-hoc多态，所以Rust的triat系统的概念类似于Haskell中的Typeclass

子类型多态的概念一般用在面向对象语言中，尤其是java语，java语言中的多态就是子类型多天，它代表一种包含关系，父类型的值包含了子类型的值，所以子类型的值有时也可以看做父类型的值，反之则不然。Rust语言中并没有类似Java中的继承，所以也不存在子类型多态。所以，Rust中的类型系统目前支持参数化多态和Ad-hoc多态，也就是，泛型和triat。

Q6: Rust类型系统中包含的类型有什么？

A6: Rust是一门强类型且类型安全的静态语言，Rust中一切皆表达式，表达式皆有值。值皆有类型。Rust中一切皆类型。

Rust的类型系统包含的类型有，基本的原生类型和复合类型，作用域(即生命周期标记)，Option< T >, Result< T , E>可选类型， never类型

Q7:可确定大小类型和动态大小类型

A7: Rust中绝大部分类型都是在编译期可以确定大小的类型(Sized Type)，原生类型,u32, u64
动态大小的类型(Dynamic Sized Type, DST),str类型的字符串字面量，编译期不可能事前知道程序中会出现什么样的字符串，所以对于编译器来说，str类型的大小是无法确定的。

str, [ T ],都是动态大小的类型。 

因此rust提供了引用类型，因为引用总会有固定的且在编译期已知的大小。字符串切片&str就是一种引用类型，它由指针和长度信息组成。

as_ptr() 可以获取字符串字面量存储的地址, len() 可以获取长度信息

Q8: &str的组成

A8: &str存储在栈上，str字符串序列存储于堆上，&str由两部分组成：指针和长度信息。其中指针是固定大小的，存储的是str字符串序列的起始地址，长度信息也是固定大小的整数。
这样，&str变为了可确定大小的类型，编译器就可以正确的为其分配栈内存空间。str也会在运行时在堆上开辟内存空间。

对于字符串字面量，可以通过as_ptr和len方法，可以分别获取该字符串字面量存储的地址的长度信息。这种包含了动态大小类型地址信息和携带了长度信息的指针，叫做胖指针(Fat Pointer)。