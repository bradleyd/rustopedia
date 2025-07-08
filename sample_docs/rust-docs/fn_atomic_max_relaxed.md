atomic_max_relaxed in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_max_relaxed
Copy item path
Source
pub unsafe fn atomic_max_relaxed<T>(dst:
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
Maximum with the current value using a signed comparison.
T
must be a signed integer type.
The stabilized version of this intrinsic is available on the
atomic
signed integer types via the
fetch_max
method by passing
Ordering::Relaxed
as the
order
. For example,
AtomicI32::fetch_max
.