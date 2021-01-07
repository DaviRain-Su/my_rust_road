# mem::replace to keep owned values in changed enums

## 描述

假设我们有一个`&mut MyEnum`, 它至少有两个变体， `A { name : String, x: u8}`和`B {name: String}`。现在我们要将`MyEnum::A` 更改为`B`,如果`x`为`0`， 同事保持`MyEnum::B`不变。

我们不需要克隆`name`就可以做到这一点。

## 例子

```rust
use std::mem;

enum MyEnum {
  A { name: String, x: u8},
  B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
  
  // we mutably borrow 'e' here, This precludes use from changing it directly 
  // as in '*e = ...', because the borrow checker won't allow it. Therefore 
  // the assignment to 'e' must be outside the 'if let' clause.
  *e = if let MyEnum::A { ref mut name, x: 0} = *e {
    
    // this takes out our 'name' and put in an empty String instead 
    // (not that empty strings don't allocate).
    // Then, construct the new enum variant (which will 
  	// be assigned to '*e', because it is the result of the 
  	// 'if let' expression).
    MyEnum::B { name: mem::replace(name, String::new())}
    
    // In all other cases, we return immediately, thus skipping the assignment
  } else {
    return 
  }
}
```

这也适用于更多的变体

```rust
use std::mem;

enum MultiVariateEnum {
  A { name: String},
  B { name: String},
  C, 
  D,
}
fn swizzle(e: &mut MultiVariateEnum) {
  use self::MultiVariateEnum::*;
  *e = match *e {
    // Ownership rules do not allow take 'name' by value, but 		// we cannot take the value out of a mutable reference, 
    // unless we replace it:
    A {ref mut name } => B { name: mem::replace(name, String::new())},
    B {ref mut name} => A { name: mem::replace(name, String::new())},
    C => D,
    D => C
  }
}
```

## 动机

在使用枚举时，我们可能希望将枚举值就地更改，也许是更改为另一个变体，这通常分两个阶段进行，以使借用检查器满意。在第一阶段，我们观察现有的值并查看它的各个部分以决定下一步要做什么。在第二阶段，我们可以有条件地更改值（如上面的例子所示）。

借用检查器不允许我们取出枚举中的`name`（因为必须有什么东西在哪里，我们当然可以`.clone()` name并将克隆后的值放入我们的`MyEnum::B`，但那将是[Clone to satisfy the borrow checker]反模式的实例。无论如何，我们可以通过只更改可变借用`e`来避免额外的分配。

`Mem::replace`让我们换掉这个值，用其他东西替换它，在本例中，我们放入一个空`String`,它不需要分配，因为，我们将原始`name`作为一个所有值，然后我们可以包装在另一个枚举中。

但是，请注意，如果我们正在使用一个`Option`并希望用`None`替换它的值，则`Option`的`take()`方法提供了一个更短 更符合习惯的代替方法。

## 优点

看 没有分配! 而且你在分配的时候可能会觉得自己像印第安纳琼斯

## 缺点

这有点啰嗦了，反复出错会让你讨厌借用检查器，编译器可能无法优化双重存储，导致性能下降，而不是在不安全的语言中所做的。

## 讨论

这种模式只有rust中感兴趣，在GC语言中，你将默认获取对值的引用(GC 将跟踪refs)而其他低级语言如C语言中，你只需简单的别名指针并稍后进行修复、

然而，在Rust，我们还需要做更多的工作才能做到这一点，一个拥有类型的可能只有一个拥有这，所以要他它拿出来，我们需要把一些东西放回去，比如印第安纳琼斯，用一袋沙子代替工艺品。

## 参见

This gets rid of the [Clone to satisfy the borrow checker] antipattern in a specific case.

[Clone to satisfy the borrow checker](TODO: Hinges on PR #23)

