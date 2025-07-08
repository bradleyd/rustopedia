atomic_store_release in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_store_release
Copy item path
Source
pub unsafe fn atomic_store_release<T>(dst:
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
Stores the value at the specified memory location.
T
must be an integer or pointer type.
The stabilized version of this intrinsic is available on the
atomic
types via the
store
method by passing
Ordering::Release
as the
order
. For example,
AtomicBool::store
.