atomic_fence_release in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_fence_release
Copy item path
Source
pub unsafe fn atomic_fence_release()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
An atomic fence.
The stabilized version of this intrinsic is available in
atomic::fence
by passing
Ordering::Release
as the
order
.