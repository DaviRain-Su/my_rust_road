#![cfg_attr(not(feature = "std"), no_std)]



#[cfg(feature = "std")]
pub fn sum(a: i32, b: i32) -> i32 {
    println!("a + b = {}", a + b);
    a + b
}

#[cfg(not(feature = "std"))]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b + 1
}
