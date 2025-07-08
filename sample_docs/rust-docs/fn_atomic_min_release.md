atomic_min_release in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_min_release
Copy item path
Source
pub unsafe fn atomic_min_release<T>(dst:
*mut T
, src: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Minimum with the current value using a signed comparison.
T
must be a signed integer type.
The stabilized version of this intrinsic is available on the
atomic
signed integer types via the
fetch_min
method by passing
Ordering::Release
as the
order
. For example,
AtomicI32::fetch_min
.