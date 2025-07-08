atomic_cxchg_seqcst_seqcst in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_cxchg_seqcst_seqcst
Copy item path
Source
pub unsafe fn atomic_cxchg_seqcst_seqcst<T>(
    dst:
*mut T
,
    old: T,
    src: T,
) -> (T,
bool
)
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Stores a value if the current value is the same as the
old
value.
T
must be an integer or pointer type.
The stabilized version of this intrinsic is available on the
atomic
types via the
compare_exchange
method by passing
Ordering::SeqCst
as both the success and failure parameters.
For example,
AtomicBool::compare_exchange
.