# The Default Trait

## 描述

Rust中的许多类型都有一个[构造函数](https://rust-unofficial.github.io/patterns/idioms/ctor.html)，但是，这是特定于类型的；Rust不能抽象为“任何有`new()`方法的东西”，为了实现这一点，构思了[Default trait](https://doc.rust-lang.org/stable/std/default/trait.Default.html) ，他可以于容器和其他通用类型一起使用（例如，参见[Option::unwrap_or_default()](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_default)) 值得注意的是，一些容器已经在适当的地方实现了它。

不仅仅想`Cow`，`Box`，或`Arc`这样的单元素容器实现了所包含的`Default`类型的`default`，而且还可以自动为所有字段实现它的结构实现`#[derivre(Default)]`, 所以实现`default`的类型越多，他就变得越有用。

另一方面，构造函数可以接受多个参数，而`default()`方法不能，甚至可以有多个具有不同名称的构造函数，但是每个类型只能有一个`Default`实现。

## 例子

```rust
use std::{path::PathBuf, time::Duration};

// note that we can simply auto-derive Default here.
#[derive(Default, Debug)]
struct MyConfiguration {
  // Option defaults to None 
  output: Option<PathBuf>,
  // Vecs default to empty vector
  search_path: Vec<PathBuf>,
  // Duration defaults to zero time
  timeout: Duration,
  // bool defaults to false
  check: bool,
}

impl MyConfiguration {
  // add setters here
}
fn main() {
  // construct a new instance with default values
  let mut conf = MyConfiguration::default();
  con.check = true;
  println!("conf = {:#?}", conf);
}
```

## 参见

- The [constructor](https://rust-unofficial.github.io/patterns/idioms/ctor.html) idiom is another way to generate instances may or may not be "default"
- The `[Default]`(https://doc.rust-lang.org/stable/std/default/trait.Default.html) documentation (scroll down for the list of implementors)
- [`Option::unwrap_or_default()`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_default)
- [`Derive(new)`](https://crates.io/crates/derive-new/)

