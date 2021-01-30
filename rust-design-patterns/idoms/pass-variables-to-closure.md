# 给闭包传递变量

## 描述

默认情况下，闭包是通过借用来捕获环境的。或者你可以使用move closure来移动整个环境。然而，通常你只想移动一些变量到闭包，给它一些数据的副本，通过引用传递，或者执行一些其他的转换。



在单独的作用域中使用变量绑定来实现。

## 代码示例

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);
let closure = {
  // num1 is moved
  let num2 = num2.clone(); // num2 is cloned
  let num3 = num3.as_ref(); // num3 is borrowed
  move || {
    *num1 + *num2 + *num3;
  }
};
println!("clsure = {:?}", closure()); // 6
```

等同于

```rust
use std::rc::Rc;

let num1 = Rc::new(1);
let num2 = Rc::new(2);
let num3 = Rc::new(3);

let num2_cloned = num2.clone();
let num2_borrowed = num3.as_ref();
let closure = move || {
  *num1 + *num2_cloned + *num3_borrowed
};
println!("closure = {:?}", closure());
```

## 优点

Copy的数据与closure定义在一起，所以他们的目的更明确，即使不被closure消耗，也会被立即放弃。

无论数据是复制还是移动，Closure都使用与周围代码相同的变量名。

## 缺点

闭包体额外的缩进。