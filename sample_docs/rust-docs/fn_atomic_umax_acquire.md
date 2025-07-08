atomic_umax_acquire in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_umax_acquire
Copy item path
Source
pub unsafe fn atomic_umax_acquire<T>(dst:
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
Maximum with the current value using an unsigned comparison.
T
must be an unsigned integer type.
The stabilized version of this intrinsic is available on the
atomic
unsigned integer types via the
fetch_max
method by passing
Ordering::Acquire
as the
order
. For example,
AtomicU32::fetch_max
.