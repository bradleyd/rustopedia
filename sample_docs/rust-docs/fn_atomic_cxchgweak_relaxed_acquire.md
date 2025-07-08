atomic_cxchgweak_relaxed_acquire in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_cxchgweak_relaxed_acquire
Copy item path
Source
pub unsafe fn atomic_cxchgweak_relaxed_acquire<T>(
    _dst:
*mut T
,
    _old: T,
    _src: T,
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
compare_exchange_weak
method by passing
Ordering::Relaxed
and
Ordering::Acquire
as the success and failure parameters.
For example,
AtomicBool::compare_exchange_weak
.