# Finalisation in destructors

## 描述

Rust并没有提供与`finally` 块相同的代码，不管函数如何退出，finally块代码都会被执行，相反，可以使用对象的析构函数来运行必须在退出之前运行的代码。

## 例子

```rust
fn bar() -> ResultM<(), ()> {
  // These don't need to be defined inside the function.
 	struct Foo;
  
  // Implement a destructor for Foo. 
  impl Drop for Foo {
    fn drop(&mut self) {
      println!("Exit");
    }
  }
  
  // The dtor of _exit will run however the function 'bar' is exites.
  let _exit = Foo;
  // Implicit return with '?' operator.
  baz()?;
  // Normal return.
  Ok(())
}
```

## 动机

如果一个函数有多个返回点，那么在退出时执行代码将变得困难和重复（因此容易出现bug）。在由于宏而隐含返回的情况下尤其如此，一个常见的例子是`？`运算符，如果结果是`Err`，则返回，如果是`Ok`，则继续。`？`是作为一种异常处理机制使用的。但是与Java（有`finally`）不同的是，没有办法调度代码在正常和异常情况下运行，panic也会提前退出功能。

## 优点

析构函数中的代码将（几乎）总是运行-- 处理恐慌 提前返回等

## 缺点

不能保证析构函数会运行，例如，如果函数中存在无限循环，或者在退出前运行函数崩溃，在已经在恐慌的线程中出现恐慌时，也不会析构函数。因此，在必须进行结束处理的情况下，不能依赖析构函数作为finalisation.

这种模式引入了一些难以注意的隐式代码，读取函数并不会给出在退出时运行析构函数的明确指示，这会使调试变得棘手。

需要一个对象和`Drop` imp 只是为了finalisation这是称沉重的boilerplate.

## 讨论

关于如何准确的存储用作finaliser的对象。有一些微妙之处。它必须保持存货，直到功能结束，然后必须被销毁。对象必须始终是一个值或者唯一拥有的指针（例如`Box< foo >` )如果使用了共享指针（例如`Rc`)，那么在函数生命周期之后，finaliser仍然可以保持活动。基于类似的元婴，finaliser不应该被移动或退出。

必须将finaliser赋给一个变量，否则它将立即被销毁。而不是当它超出作用域时，如果变量只用作finaliser，则变量名必须以` _ `开头，否则编译器将警告finaliser从未使用，但是不要调用没有后缀的 `_ `变量 在这种情况下，它将立即再次被销毁。

在Rust中，当一个对象超出作用域时运行析构函数，无论我们是达到块的末尾，还是提前返回，或者程序恐慌，都会发生这种情况。当panic时，Rust解除堆栈中每个堆栈中运行的每个对象析构函数，因此，即使在调用的函数中发生了异常，也会调用析构函数。

如果一个析构函数在展开时panic，那么就没有好的动作可以执行，所以Rust会立即中止线程，而不会运行进一步的析构函数。这意味着大部分设备并不是绝对能够运行，这还意味着，在析构函数中必须格外小心，不要惊慌，因为它可能使资源处于意外状态。

## 参见

RAII

