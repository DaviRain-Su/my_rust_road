//! substrate 中的一些类型设置
//! 
//! Shareable substrate types.

#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate core;

#[macro_use]
extern crate crunchy;

#[macro_use]
extern crate fixed_hash;

#[macro_use]
extern crate uint as unint_crate;

pub mod uint;
pub mod hash;