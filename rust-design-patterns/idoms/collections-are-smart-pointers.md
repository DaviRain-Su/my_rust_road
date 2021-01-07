

# Collections are smart pointers

## 描述

使用`Deref` triat 把集合当作智能指针，提供对数据的拥有和借用视图。

## 例子

```rust
use std::ops::Deref;

struct Vec<T> {
  data: T,
  // ...
}
impl <T> Deref for Vec<T> {
  type Target = [T];
  
  fn deref(&self) -> &[T] {
    // ...
  }
}
```

`Vev < T >` 是拥有`T`s的集合，切片`&[T] `是借用`T`s的集合。实现`Deref` 为 `Vec` 允许隐式借用从`&Vec< T >`到`&[T]`， 并且包括自动解引用，并且包括自动解引用搜索中的关系。你可能期望为`Vec`s实现大多数方法都是为片实现的。

参见`String `和`&str`.

## 动机

所有权和借用是Rust语言的关键方面，为了提供良好的用户体验，数据结构必须正确地解释这些语义。当实现一个拥有其数据的数据结构时，提供该数据的借用视图可以提供更多灵活的API

## 优点

大多数方法只能为借用视图实现，因此他们对于所有视图是隐式的。

让客户在借用或拥有数据之间做出选择。

## 缺点

边界检查时，只能通过解引用获得的方法和trait没有被考虑在内，所有使用这种模式的数据结构的泛型会变得复杂（参见`Borrow`和`AsRef` triat等）

## 讨论

智能指针和集合是类似的：智能指针指向单个对象，而集合指向多个对象。从类型系统的角度来看，两者没有什么区别。如果访问每个数据的唯一方式是通过collection，而且collection负责删除数据（即使在共享所有权的情况下，某种借用的视图可能是适当的）。 如果一个集合拥有它的数据，通常提供一个借用的数据视图是有用的，这样他就可以被多重引用。

大多数智能指针（例如`Foo < T >`) 实现`Deref < Target = T>`。 但是，集合通常会借用对自定义类型的引用。`[T]`和`str`有一些语言支持，但是在一般情况下，这是不必要的。`Foo < T >` 可以实现`Deref < Target = Bar < T >>` 其中`Bar` 是一个动态大小的类型，而`&Bar < T >` 是`Foo < T >` 中数据的一个借用视图。

通常，有序的集合将实现`Index`为`Ranges`以提供切片语法。这个目标借用将提供借用视图。

## 参见 

[Deref polymorphism anti-pattern](https://rust-unofficial.github.io/patterns/anti_patterns/deref.html) 

[Documentation for `Deref` triat.](https://doc.rust-lang.org/std/ops/trait.Deref.html) 



