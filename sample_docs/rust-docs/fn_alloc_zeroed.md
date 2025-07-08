alloc_zeroed in std::alloc - Rust
std
::
alloc
Function
alloc_zeroed
Copy item path
1.28.0
·
Source
pub unsafe fn alloc_zeroed(layout:
Layout
) ->
*mut
u8
Expand description
Allocates zero-initialized memory with the global allocator.
This function forwards calls to the
GlobalAlloc::alloc_zeroed
method
of the allocator registered with the
#[global_allocator]
attribute
if there is one, or the
std
crate’s default.
This function is expected to be deprecated in favor of the
allocate_zeroed
method
of the
Global
type when it and the
Allocator
trait become stable.
§
Safety
See
GlobalAlloc::alloc_zeroed
.
§
Examples
use
std::alloc::{alloc_zeroed, dealloc, handle_alloc_error, Layout};
unsafe
{
let
layout = Layout::new::<u16>();
let
ptr = alloc_zeroed(layout);
if
ptr.is_null() {
        handle_alloc_error(layout);
    }
assert_eq!
(
*
(ptr
as
*mut
u16),
0
);

    dealloc(ptr, layout);
}