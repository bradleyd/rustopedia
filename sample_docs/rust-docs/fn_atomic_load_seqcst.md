atomic_load_seqcst in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_load_seqcst
Copy item path
Source
pub unsafe fn atomic_load_seqcst<T>(src:
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
Loads the current value of the pointer.
T
must be an integer or pointer type.
The stabilized version of this intrinsic is available on the
atomic
types via the
load
method by passing
Ordering::SeqCst
as the
order
. For example,
AtomicBool::load
.