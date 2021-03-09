# Rust FFI Recap 
> Tips and tricks about implementing an FFI call

## FFI

## FFI :  Foreign function interface

- various programming languages
    - php is the best
- Mature libs from a foregin language
    - normally C/C++
- High-performance implementation from a foreign language 
    - normally script/vm languages, not rust
- Reinverting wheels ???

## FFI - Use cases 

- pyqt  
    - a full binding to qt of C++
- numpy
    - FFI, LAPACK, random generator via Cython ..
    - use C/Fortran to improve performance
- rust-sdl2 // 游戏开发
    - sdl2 libs, gamming
- librustzcash // 零知识证明算法
    - C library to access crypto function implemented in Rust


## Rust FFI Call C

## Rust FFI - Calling C

```rust
extern crate libc;
use libc::size_t;

#[link(name = "snappy")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let x = unsafe {
        snappy_max_compressed_length(100)
    };

    println!("max compressed length of a 100 byte buffer: {}", x);
}
```

## Rust FFI - Building blocks 

- tyes: 
    - libc crate
    - std::os::raw::{c_void, c_char, ...}
- extern "ABI" {} blocks
    - "C" / "stdcall" / "system" / ....
    - #[link(name = "extlib")]
    - foreign function signature 
    - foreigm static global
- unsafe {} call 

## Rust FFI - hellper

- std::ffi 
    - CStr/ CString
        - owned/borrowed pattern of c string
    - OsStr / OsString
        - owned / borrowed pattern of os string 
        - platform-dependent, wstring
    - VaList (experimental)

- std::ptr
    - handle raw memory pointers
    - null() / null_mut()
- *const T / * mut T (pointer)
    - .is_null()
    - .offset(isize)
- std::mem
    - drop() / forget() 
    - transmute::<T1, T2>() 
    - MaybeUnit<T> / uninitialized()
    - zeroed()
    - ... 
- get raw pointer
    - str::as_ptr() / str::as_mut_ptr()
    - slice::as_ptr() / slice::as_mut_ptr()
    - ..::as_ptr() / ..::as_mut_ptr()
- from raw pointer
    - slice::from_raw_parts() 
    - String / Vec also have from_raw_parts(), 
        - Vec::into_raw_parts()
- Box - help alloc 
    - Box::into_raw(_: Box<T>) -> *mut T, leaks memory
    - Box::from_raw(_: *mut T) -> Box<T>, capture leaked

- language features 
    - #[repr(C)] for struct 
    - #[repr(i*)] #[repr(u*)] for enums 
    - build script: build.rs
        - cc, make, pkg-config, vcpkg
- bindgen - the code generatror

## Rust FFI - key Points

- All about memory safety!!! 
    - old-school c memory rules still work
- who owns memory ? C or Rust
    - who creates, who destroys 
    - C: malloc/ free 
    - C++: new / delete
    - Rust: Box
- where is the memory ? heap or function call stack
    - pointer to the stack: dangling pointer
- Can the value type copied? (has external reference?)
- mangle? #ifdef __cplusplus



