[![crates.io](https://img.shields.io/crates/v/fn_has_this.svg)](https://crates.io/crates/fn_has_this)

# ✨ `fn_has_this`

A proc attribute macro that adds a `this` argument to a function.

This macro is well suited for conditional compilation. For example, some compilers will optimize away a `this` argument in a non-static function if it is unused, sometimes depending on calling convention. This macro makes it easy to conditionally compile your Rust programs to interact over FFI with functions compiled in this way.

## Example

```rust
#[macro_use]
extern crate fn_has_this;

use core::ffi::c_void;

#[has_this("*mut c_void")] // Will create an argument called `this` in the function
pub unsafe extern "fastcall" fn print_pointer() {
    println!("{:x?}", this);
}

#[has_this("me: *mut c_void")] // You can also specify a custom name for the `this` argument
pub unsafe extern "thiscall" fn print_pointer() {
    println!("{:x?}", me);
}

#[cfg_attr(target_os = "windows", has_this("me: *mut c_void"))] // Using `cfg_attr`, you can conditionally compile this attribute macro
pub unsafe extern "thiscall" fn print_pointer() {
    println!("{:x?}", me);
}
```
