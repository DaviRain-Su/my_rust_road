# 传递字符串

## 描述

在向FFI函数传递字符串时，应该遵守四个原则：

- 尽可能延长拥有的字符串的生命周期
- 尽量减少转换过程中的不安全代码
- 如果C代码可以修改字符串数据，请使用vec而不是CString
- 除非外部函数API要求，否则字符串的所有权不应该转移到被调用者身上。

## 动机

Rust的CString和CStr类型内置了对C风格字符串的支持。然而，对于从Rust函数中发送到外部函数调用的字符串，我们可以采取不同的方法。

最好的做法是简单的：使用CString的方式来减少不安全的代码。然而，一个次要的注意事项是，对象必须有足够长的生命周期，也就是说应该最大限度的延长生命周期。此外，文档中解释说，修改CString后"round-tripping"是UB, 所以在这种情况下需要额外的工作。

## 代码示例

```rust
pub mod unsafe_module {
  // other module content 
  
  extern "C" {
    fn seterr(message: *const libc::c_char);
    fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
  }
  
  fn report_error_to_ffi<S: Into<String>>(err: S) -> Result<(), std::ffi::NullError> {
    let c_err = std::ffi::CString::new(err.into())?;
    
    unsafe {
      //SAFETY: calling an FFI whose documentation says the pointer is 
      //const, so no modificationshould occur
      // 调用一个FII, 其文档中说指针是const, 所以不应该发生修改
      seterr(c_err.as_ptr());
    }
    
    Ok(())
    // The lifetime of c_err continues until here
  }
  
  fn get_error_from_ffi() -> Result<String, std::ffi::IntoStringError> {
    let mut buffer = vec![0u8; 1024];
    unsafe {
      // SAFETY: calling an FFI whose documentaion implies 
      // that the input need only live as long as the call
      // 调用一个FFI, 其文档中暗示，输入进需要存活和调用一样长
      let written: usize = geterr(buffer.as_mut_ptr(), 1023).into();
      
      buffer.truncate(written + 1);
    }
    
    std::ffi::CString::new(buffer).unwrap().into_string()
  }
}
```

## 优点

这个例子的编写方式是为了保证：

- Unsafe 区块越小越好

- CString的存活足够的长
- 在可能的情况下，总是会传播类型转换的错误

一个常见的错误（在文档中是常见的）是在第一个代码块中不适用变量：

```rust
pub mod unsafe_module {
  // other module content 
  
  fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NullError> {
    unsafe {
      // SAFETY: whoops, this contains a dangling pointer! 
      seterr(std::ffi::CString::new(err.into())?.as_ptr());
    }
    Ok(())
  }
}
```

这段代码会导致一个悬空的指针，因为CString的生命周期并没有指针的创建而延长，不像创建了一个引用。

另一个经常被提问的问题是，一个1K的零向量的初始化是"慢"的，然而，最近的Rust版本实际上将该特定的宏优化为对zmalloc的调用，这意味着他与操作系统返回归零内存的能力一样快（这是相当快的）。

## 缺点

无