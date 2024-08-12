Rust std::time::Instant in C++
==============================

`include/rust_time.h` contains `rust::time::Instant`, which is a port of Rust's `std::time::Instant` with memory-layout compatibility.

## No guarantee
No guarantee for compatibility.

Since `std::time::Instant` and `std::time::Duration` is not `#[repr(C)]`, use it on your own risk.

## See also
- https://github.com/youknowone/rust-ffi_types
