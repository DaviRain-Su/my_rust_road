# 构造者模式

## 描述

Rust 没有构造函数作为一种语言构造，相反，约定是使用一种静态的新方法来创建对象。

## 例子

```rust
// A Rust vector, see liballoc/vec.rs
pub struct Vec<T> {
  buf: RawVec<T>,
  len: usize,
}
impl <T> Vec<T> {
  // COnstructs a new, empty 'Vec<T>'
  // Note this is a static method - no self
  // This constructor doesn't take any aurgument, but some mighet in order to 
  // properly initialise an object
  pub fn new() -> Vec<T> {
    // Create a new Vec with fields properly intialised.
    Vec {
      // Note that here we are calling RawVec's constructor 
      buf: RawVec::new(),
      len: 0,
    }
  }
}
```

## 参见

用于构造存在多种配置的对象的构建器模式

