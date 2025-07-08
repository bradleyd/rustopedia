realloc in std::alloc - Rust
std
::
alloc
Function
realloc
Copy item path
1.28.0
·
Source
pub unsafe fn realloc(ptr:
*mut
u8
, layout:
Layout
, new_size:
usize
) ->
*mut
u8
Expand description
Reallocates memory with the global allocator.
This function forwards calls to the
GlobalAlloc::realloc
method
of the allocator registered with the
#[global_allocator]
attribute
if there is one, or the
std
crate’s default.
This function is expected to be deprecated in favor of the
grow
and
shrink
methods
of the
Global
type when it and the
Allocator
trait become stable.
§
Safety
See
GlobalAlloc::realloc
.