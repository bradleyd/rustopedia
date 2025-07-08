atomic_fence_acquire in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_fence_acquire
Copy item path
Source
pub unsafe fn atomic_fence_acquire()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
An atomic fence.
The stabilized version of this intrinsic is available in
atomic::fence
by passing
Ordering::Acquire
as the
order
.