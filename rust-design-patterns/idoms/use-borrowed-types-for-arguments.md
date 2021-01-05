# Use borrowed types for arguments 

> 对参数使用borrowed types

## 描述

当你决定使用哪种参数作为函数参数时，使用强制解引用(deref coerrcion) 的目标可以增加代码的灵活性。这样，函数将接受更多的输入类型。

这不经限于可切片类型或者胖指针类型。事实上，你总是更喜欢borrowed type 而不是 borrowing the owned type, 例如， &str Over &String, &[T] Over &Vec< T >, or &T Over &Box< T >

对于owned type 已经提供了间接层的实例，使用borrowed type可以避免使用间接层。例如，String有一个间接层，因此&String将有两个间接层。我们可以通过使用&str来避免这种情况，并且在调用函数时让&String强制转换到&str.

## 例子

在本例中，我们将说明使用&String作为函数参数与使用&str的区别。但是这些思想也适用于使用&Vec < T > 与使用&[T] 或者使用&T 与&Box< T > .

考虑一个例子，我们希望确定一个单词是否包含三个连续的元音，我们不需要拥有字符串来确定这一点。因为我们将采用一个引用。

代码可能是这样的

```rust
fn three_vowels(word: &String) -> bool {
  let mut vowel_ocunt = 0;
  for c in word.chars() {
    match c {
      'a' | 'e' | 'i' | 'o' | 'u' => {
        vowel_count += 1;
        if vowel_count >= 3 {
          return true;
        }
      }
      _ => vowel_count = 0;
    }
  }
  false
}
fn main() {
  let ferris = "Ferris".to_string();
  let curious = "Curious".to_string();
  println!("{}: {}", ferris, three_vowels(&ferris));
  println!("{}: {}", curious, three_vowels(&curious));
  
  // This work fine,but the following two lines would fail;
  // println!("Ferris: {}", three_vowles("Ferris"));
  // println!("Currious: {}", three_vowles("Curious"));
}
```

这样做很好，因为我们将&String类型作为参数传递。我们在最后两行中进行注释，这个示例就会失败，因为&str类型不会强制转换为&String。我们可以通过简单地修改参数的类型来解决这个问题。

例如，我们将函数声明更改为

```rust
fn three_vowels(word: &str) -> bool {
```

 然后两个版本将编译并打印相同的输出

```
Ferris: false
Curious: true
```

但是，等等，这还不是全部。这个故事还有更多内容。很可能你会对自己说：这不重要，我们永远不会使用&' static str作为输入方式(就像我们使用"Ferris"时所作的那样)。即使忽略这个特殊的示例，您可能仍然会发现使用&str比使用&String更具灵活性.

现在让我们举一个例子，有人给我们一个句子，我们想确定句子中的任何一个单词是否有一个包含三个连续元音的单词，我们可能应该利用我们已经定义的函数，简单的输入句子中的每个单词。

下面试一个例子：

```rust
fn three_vowels(word: &String) -> bool {
  let mut vowel_ocunt = 0;
  for c in word.chars() {
    match c {
      'a' | 'e' | 'i' | 'o' | 'u' => {
        vowel_count += 1;
        if vowel_count >= 3 {
          return true;
        }
      }
      _ => vowel_count = 0;
    }
  }
  false
}
fn main() {
  let sentence_string = "Once upon a time, there was a friendly curious crab named Ferris".to_string();
  for word in sentence_string.split(' ') {
    if three_vowels(word) {
      println!("{} has three consecutive vowels!", word);
    }
  }
}
```

带有&str类型参数声明的函数运行这个例子将产生

```
curious has three consecutive vowels!
```

但是，我们的函数使用&String类型的参数声明时，这个示例将不会运行。这是因为字符串切片是&str而不是&String,需要分配内存将&str转换为&String，但是这不是隐式的，而从String转换为&str是廉价且隐式的。

## 参见

- [Rust Language Reference on Type Coercions ](https://doc.rust-lang.org/reference/type-coercions.html)
- For more discussion on how to handle `String` and `&str` see [this blog series (2015)](https://web.archive.org/web/20201112023149/https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html) by Herman J. Radtke III.

