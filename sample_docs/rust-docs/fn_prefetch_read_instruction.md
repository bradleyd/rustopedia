prefetch_read_instruction in std::intrinsics - Rust
std
::
intrinsics
Function
prefetch_read_instruction
Copy item path
Source
pub unsafe fn prefetch_read_instruction<T>(data:
*const T
, locality:
i32
)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
The
prefetch
intrinsic is a hint to the code generator to insert a prefetch instruction
if supported; otherwise, it is a no-op.
Prefetches have no effect on the behavior of the program but can change its performance
characteristics.
The
locality
argument must be a constant integer and is a temporal locality specifier
ranging from (0) - no locality, to (3) - extremely local keep in cache.
This intrinsic does not have a stable counterpart.