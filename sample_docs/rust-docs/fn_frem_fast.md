frem_fast in std::intrinsics - Rust
std
::
intrinsics
Function
frem_fast
Copy item path
Source
pub unsafe fn frem_fast<T>(a: T, b: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Float remainder that allows optimizations based on algebraic rules.
May assume inputs are finite.
This intrinsic does not have a stable counterpart.