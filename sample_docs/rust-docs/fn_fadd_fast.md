fadd_fast in std::intrinsics - Rust
std
::
intrinsics
Function
fadd_fast
Copy item path
Source
pub unsafe fn fadd_fast<T>(a: T, b: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Float addition that allows optimizations based on algebraic rules.
May assume inputs are finite.
This intrinsic does not have a stable counterpart.