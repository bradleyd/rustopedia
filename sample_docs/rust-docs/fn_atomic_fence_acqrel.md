atomic_fence_acqrel in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_fence_acqrel
Copy item path
Source
pub unsafe fn atomic_fence_acqrel()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
An atomic fence.
The stabilized version of this intrinsic is available in
atomic::fence
by passing
Ordering::AcqRel
as the
order
.