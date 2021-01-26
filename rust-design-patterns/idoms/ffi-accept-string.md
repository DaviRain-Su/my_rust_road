# 接受字符串

## 描述

在通过指针通过FFI接受字符串时，应该遵循两个原则：

- 保持外来字符串的"借用"，而不是直接复制。
- 尽量减少转换过程中的不安全代码。

## 动机

Rust的CString和CStr类型内置了对C风格字符串的支持。然而，对于从Rust函数的外来调用者那里接受的字符串，我们可以采取不同的方法。

最好的做饭很简单：以尽量减少不安全代码的方式使用CStr,并创建一个借用的切片。如果需要一个所有权的String，在字符串切片上调用to_string().

## 代码示例

```rust
pub mod unsafe_module {
  // other moudle content
  
  #[no_mangle]
  pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    let level: crate::LogLevel = match level { /* ... */};
    
    let msg_str: &str = unsafe {
      // 安全：访问预期为调用而存在的原始指针，并创建一个不超过当前堆栈框架的共享引用。
      match std::ffi::CStr::from_ptr(msg).to_str() {
        Ok(s) => s, 
        Err(e) => {
          crate::log_error("FFI string conversion failed");
          return;
        }
      }
    };
    crate::log(msg_str, level);
  }
}
```

## 优点

这个例子是为了确保：

- 不安全区块越小越好
- 具有“未跟踪”生命周期的指针成为“跟踪”的共享引用

考虑一个替代方案，实际上就是复制字符串：

```rust
pub mod unsafe_module {
  // other module content 
  
  pub extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
    //DOT NOT USE THIS CODE.
    // IT IS UGLY, VERBOSE, AND CONTAINS A SUBTLE BUG. 
    
    let level: crate::LogLevel = match level { /* .... */};
    
    let msg_len = unsafe { /* SAFETY: strlen is what it is, I guess? */
    	libc::strlen(msg)
    };
    
    let mut msg_data = Vec::with_capacity(msg_len + 1);
    
    let msg_cstr: std::ffi::CString = unsafe {
      //SAFETY: copying from a foregin pointer expected to live 
      // for the entire stack frame into owned memory
      std::ptr::copy_nonoverlapping(msg, msg_data.as_mut(), msg_len);
      
      msg_data.set_len(msg_len + 1);
      
      std::ffi::CStrig::from_vec_with_null(msg_data).unwrap()
    };
    
    let msg_str: String = unsafe {
      match msg_cstr.into_string() {
        Ok(s) => s,
        Err(e) => {
          crate::log_error("FFI string conversion failed");
          return;
        }
      }
    };
    crate::log(&msg_str, level);
  }
}
```

这个代码在两个方面比原来的代码差。

- 有很多不安全的代码，更重要的是，它必须维护更多的不变量
- 由于需要大量的运算，这个版本有一个错误，会导致Rust未定义行为。

这里的bug是指针运算中的一个简单错误：字符串被复制了，其中所有的msg_len字节。然而，最后的NULL终结符却没有被复制。

然而，Vector的大小被设置为零填充字符串的长度，而不是调整其大小，这可能会在最后增加一个零。结果，Vector中的最后一个字节是未初始化的内存，当CString在块的底部创建时，它对Vector的读取将导致未定义的行为！

像很多这样的问题一样，这将是很难追踪的问题。有时会因为字符串不是UTF-8而发生恐慌，有时会在字符串的末尾放一个奇怪的字符，有时会完全崩溃。

## 缺点

无