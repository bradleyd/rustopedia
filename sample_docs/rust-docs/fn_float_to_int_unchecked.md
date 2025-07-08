float_to_int_unchecked in std::intrinsics - Rust
std
::
intrinsics
Function
float_to_int_unchecked
Copy item path
Source
pub unsafe fn float_to_int_unchecked<Float, Int>(value: Float) -> Int
where
    Float:
Copy
,
    Int:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Converts with LLVM’s fptoui/fptosi, which may return undef for values out of range
(
https://github.com/rust-lang/rust/issues/10184
)
Stabilized as
f32::to_int_unchecked
and
f64::to_int_unchecked
.