atomic_load_unordered in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_load_unordered
Copy item path
Source
pub unsafe fn atomic_load_unordered<T>(src:
*const T
) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Do NOT use this intrinsic; “unordered” operations do not exist in our memory model!
In terms of the Rust Abstract Machine, this operation is equivalent to
src.read()
,
i.e., it performs a non-atomic read.