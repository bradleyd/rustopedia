dealloc in std::alloc - Rust
std
::
alloc
Function
dealloc
Copy item path
1.28.0
·
Source
pub unsafe fn dealloc(ptr:
*mut
u8
, layout:
Layout
)
Expand description
Deallocates memory with the global allocator.
This function forwards calls to the
GlobalAlloc::dealloc
method
of the allocator registered with the
#[global_allocator]
attribute
if there is one, or the
std
crate’s default.
This function is expected to be deprecated in favor of the
deallocate
method
of the
Global
type when it and the
Allocator
trait become stable.
§
Safety
See
GlobalAlloc::dealloc
.