# On-Stack Dynamic Dispatch 

## 描述

我们可以在多个值上动态调度，然而，要做到这一点，我们需要声明多个变量来绑定不同类型的对象。为了根据需要延长声明周期，我们可以使用延迟条件初始化，如下图所示：

## 例子

```rust
use std::io;
use std::fs;

// These must live longer than 'readable', and thus are declared first:
let (mut stdin_read, mut file_read);

let readable: &mut dyn io::Read = if arg == "-" {
  stdin_read = io::stdin();
  &mut stdin_read
}else {
  file_read = fs::File::open(args)?;
  &mut file_read
};

// Read from "readable" here.
```

## 动机

Rust 默认情况下会将代码单形化，这意味着每一种类型的代码都会生成一份副本，并进行独立的优化。虽然这允许在热路径上生成非常快的代码，但它也会在一些性能不重要的地方膨胀代码，从而浪费了编译时间和缓存使用。

幸运的是，Rust允许我们使用动态调度，但我们必须明确要求它。

## 优点

我们不需要在堆上分配任何东西。我们也不需要初始化一些我们以后不会用到的东西，也不需要将下面的整个代码单形化，以便同时使用File或Stdin。

## 缺点

与机遇Box的版本相比，代码需要更多的活动部件：

```rust
// We still need to ascribe the type for dynamic dispatch.
let readable: Box<dyn io::Read> = if arg == "-" {
  Box::new(io::stdin())
} else {
  Box::new(fs::File::open(arg)?)
}
// Read from 'readable' here.
```



## 讨论

Rust新手通常会了解到Rust要求所有的变量在使用前都要初始化，所以很容易忽略未使用的变量很可能是未初始化的。Rust相当努力地确保这一点，只有初始化的值才会在其作用域结束时被丢弃。

这个例子满足了Rust对我们的所有约束：

- 所有的变量在使用之前都被初始化了（在这个例子中是借用）
  - 每一个变量值保存一个类型的值。在我们的例子中，`stdin`的类型是`Stdin.` `file`的类型是`File`.`readable`的类型是`&mut dyn Read`
- 被借用值的生命周期要长于借用方值的生命周期。

## 参见

- [Finalisation in destructors](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html) and [RAII guards](https://rust-unofficial.github.io/patterns/patterns/RAII.html) can benefit from tight control over lifetimes.

  - Finalisation in destructors and RAII gurads 可以从对生命周期的严格控制中受益

- For conditionally filled `Option<&T>`s of (mutable) references, one can initialize an `Option<T>`directly and use its [`.as_ref()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) method to get an optional reference.

  对于有条件的填充`Option <&T>` 的引用（或者可变引用），可以直接初始化一个`Option<T>`，并使用它的`.as_ref()`方法来获得一个`Option reference.`







