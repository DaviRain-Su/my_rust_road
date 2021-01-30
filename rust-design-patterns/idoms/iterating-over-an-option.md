# 在Option上使用Iterate

## 描述

`Option `可以被看作是一个包含0或1个元素的容器，特别的是，它实现了`IntoIterator` Trait,因此可以用于需要这种类型的通用代码。

## 代码示例

由于`Option`实现了`IntoIterator`，它可以作为[.extension()](https://doc.rust-lang.org/std/iter/trait.Extend.html#tymethod.extend)的参数

```rust
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

logicians.extend(turing);

// equivalent to 
if let Some(turing_inner) = turing {
  logicians.push(turing_inner);
}
```

如果你需要在现有迭代器的末尾加上一个`Option`, 你可以把它传递给[.chain()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain).

```rust
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

for logician in logicians.iter().chain(turing.iter()) {
  println!("{} is a logician", logician);
}
```

请注意，如果`Option`总是`Some`, 那么在元素上使用[std::iter::conce](https://doc.rust-lang.org/std/iter/fn.once.html) 会更习惯。

另外，由于`Option`实现了`IntoIterator`, 所以可以使用`for`循环对其进行迭代。这相当于用`if let Some(..)`, 来匹配它，在大多数情况下，你应该更喜欢后者。

## 参见

- [std::iter::once](https://doc.rust-lang.org/std/iter/fn.once.html)是一个能产生一个元素的迭代器，它比`Some(foo).into_iter()` 更易读。
- [Iterator::filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) 是[Iterator::flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)的一个版本，专门用于返回`Option`的映射函数。
- [Ref_slice crate](https://crates.io/crates/ref_slice) 提供了将`Option`转换为零元素或单元素slice的函数
- [Documentation for `Option`](https://doc.rust-lang.org/std/option/enum.Option.html)

