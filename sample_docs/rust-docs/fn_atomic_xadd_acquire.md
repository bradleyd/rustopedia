atomic_xadd_acquire in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_xadd_acquire
Copy item path
Source
pub unsafe fn atomic_xadd_acquire<T>(dst:
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
Adds to the current value, returning the previous value.
T
must be an integer or pointer type.
If
T
is a pointer type, the provenance of
src
is ignored: both the return value and the new
value stored at
*dst
will have the provenance of the old value stored there.
The stabilized version of this intrinsic is available on the
atomic
types via the
fetch_add
method by passing
Ordering::Acquire
as the
order
. For example,
AtomicIsize::fetch_add
.