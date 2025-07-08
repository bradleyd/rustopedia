atomic_xchg_seqcst in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_xchg_seqcst
Copy item path
Source
pub unsafe fn atomic_xchg_seqcst<T>(dst:
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
Stores the value at the specified memory location, returning the old value.
T
must be an integer or pointer type.
The stabilized version of this intrinsic is available on the
atomic
types via the
swap
method by passing
Ordering::SeqCst
as the
order
. For example,
AtomicBool::swap
.