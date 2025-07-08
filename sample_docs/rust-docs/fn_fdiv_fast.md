fdiv_fast in std::intrinsics - Rust
std
::
intrinsics
Function
fdiv_fast
Copy item path
Source
pub unsafe fn fdiv_fast<T>(a: T, b: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Float division that allows optimizations based on algebraic rules.
May assume inputs are finite.
This intrinsic does not have a stable counterpart.