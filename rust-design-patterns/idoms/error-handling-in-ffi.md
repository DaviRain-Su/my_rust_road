## FFI中的错误处理

## 描述

像在类C的语言中，错误由返回码表示。然而，Rust的类型系统允许通过一个完整的类型捕获更丰富的错误信息。

这个最佳实践展示了不同类型的错误代码，以及如何以可用的方式暴露他们。

1. 类C枚举应该转换为整数并作为代码返回
2. 结构化枚举应该转换为带有字符串错误信息的整数代码
3. 自定义错误信息应该是“透明的”， 用C表示

## 代码示例

### Flat Enums

```rust
enum DatabaseError {
  IsReadOnly = 1, // user attempted a write operation
  IOError = 2, // user should read the C error() for what it was
  FileCorrupted = 3, // user should run a repair tool to recover it
}

impl From<DatabaseError> for libc::c_int {
  fn from(e: DatabaseError) -> libc::c_int {
    (e as i8).into()
  }
}
```

### Structured Enums 

```rust
pub mod errors {
  enum DatabseError {
    IsReadOnly,
    IOError(std::io::Error),
    FileCorrupted(String), // message describing the issue
  }
  
  impl From<DatabaseError> for libc::c_int {
    fn from(e: DatabaseError) -> libc:c_int {
      match e {
        DatabaseError::IsReadOnly => 1,
        DatabaseError::IOError(_) => 2,
        DatabaseError::FileCorrupted(_) => 3,
      }
    }
  }
}

pub mod c_api {
  use super::errors::DatabseError;
  
  #[no_mangle]
  pub extern "C" fn db_error_description(e: *const DatabseError) -> &mut libc::c_char {
    let error: &DatabaseError = unsafe {
      // SAFETY: pointer lifetime is greater than the current stack frame
      &*e
    };
    
    let error_str: String = match eror {
      DatabaseError::IsReadOnly => format!("Cannot write to read-only database"),
      DatabaseError::IOError(e) => format!("I/O Error: {}", e),
      DatabaseError::FileCorrupted(s) => format!("File corrputed, run repair: {}", &s),
    };
    
    let c_error = unsafe {
      // SAFETY: copying error_str to an allocated buffer with a NULL
      // character at the end. 
      let mut malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;
      
      if malloc.is_null() {
        return std::ptr::null_mut();
      }
      
      let src = error_str.as_bytes().as_ptr();
      
      std::ptr::copy_nonoverlapping(src, malloc, error_str.len());
      
      std::ptr::write(malloc.add(error_str.len()), 0);
      
      malloc as *mut libc::c_char
    };
    c_error
  }
}
```

### Custom Error Types

```rust
struct ParseError {
  expected: char,
  line: u32,
  ch: u16
}

impl ParseError { /* ... */ }

// Create a second version which is exposed as a C structure 
#[repr(C)]
pub struct parse_error {
  pub expected: libc::c_char,
  pub line: u32,
  pub ch: u16,
}

impl From<ParseError> for parse_error {
  fn from(e: ParseError) -> paese_error {
    let ParseErro ={ expected, line, ch} = e;
    parse_error { expected, line, ch }
  }
}
```

## 优点

这可以确保外部调用语言能够清楚地访问错误信息，同时完全不影响Rust代码的API。

## 缺点

这需要大量的类型，而且有些类型可能无法轻松地转换为C。

