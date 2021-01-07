# 连接字符串format!

## 描述

可以使用可变字符串上`push`和`push_str`方法构建字符串，或者使用`+`运算符。但是，使用`format!`，往往更方便，特别是当存在文本字符串和非文本字符串混合时。

## 例子

```rust
fn say_hello(name: &str) -> String {
  // We could construct the result string manually.
  // let mut result = "Hello ".to_owned();
  // result.push_str(name);
  // result.push('!');
  // result
  
  // But using format! is better.
  format!("Hello {}!", name)
}
```

## 优点

使用`format!`通常是组合字符串最简洁和可读的方法

## 缺点

组合字符串通常不是最有效的方法 --- 对可变字符串的一系列`push` 通常刚是最有效的（特别是如果字符串已经预分配到预期大小）。

