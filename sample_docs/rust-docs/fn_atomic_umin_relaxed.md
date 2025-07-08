atomic_umin_relaxed in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_umin_relaxed
Copy item path
Source
pub unsafe fn atomic_umin_relaxed<T>(dst:
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
Minimum with the current value using an unsigned comparison.
T
must be an unsigned integer type.
The stabilized version of this intrinsic is available on the
atomic
unsigned integer types via the
fetch_min
method by passing
Ordering::Relaxed
as the
order
. For example,
AtomicU32::fetch_min
.