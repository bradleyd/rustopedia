atomic_singlethreadfence_acqrel in std::intrinsics - Rust
std
::
intrinsics
Function
atomic_singlethreadfence_acqrel
Copy item path
Source
pub unsafe fn atomic_singlethreadfence_acqrel()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
A compiler-only memory barrier.
Memory accesses will never be reordered across this barrier by the
compiler, but no instructions will be emitted for it. This is
appropriate for operations on the same thread that may be preempted,
such as when interacting with signal handlers.
The stabilized version of this intrinsic is available in
atomic::compiler_fence
by passing
Ordering::AcqRel
as the
order
.