//! # isnt
//!
//! This crate contains extension methods for most boolean-valued functions the standard
//! library. For example
//!
//! ```
//! # use isnt::std_1::primitive::{IsntConstPtrExt, IsntU8Ext, IsntSliceExt};
//! fn f(x: *const u8, y: u8, z: &[u8]) -> bool {
//!     x.is_not_null() && y.is_not_ascii() && z.is_not_empty()
//! }
//! ```
//!
//! Most of the code in this crate is generated and undocumented. The organization
//! follows the module structure of the standard library.

include!("lib_generated.rs");
