# isnt

[![crates.io](https://img.shields.io/crates/v/isnt.svg)](http://crates.io/crates/isnt)
[![docs.rs](https://docs.rs/isnt/badge.svg)](http://docs.rs/isnt)

This crate contains extension methods for most boolean-valued functions the standard
library. For example

```rust
fn f(x: *const u8, y: u8, z: &[u8]) -> bool {
    x.is_not_null() && y.is_not_ascii() && z.is_not_empty()
}
```

Most of the code in this crate is generated and undocumented. The organization
follows the module structure of the standard library.
