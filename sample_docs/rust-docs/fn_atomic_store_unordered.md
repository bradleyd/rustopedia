atomic_store_unordered in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_store_unordered
Copy item path
Source
pub unsafe fn atomic_store_unordered<T>(dst:
*mut T
, val: T)
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
dst.write(val)
,
i.e., it performs a non-atomic write.