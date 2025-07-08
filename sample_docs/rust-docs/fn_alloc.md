alloc in std::alloc - Rust
std
::
alloc
Function
alloc
Copy item path
1.28.0
·
Source
pub unsafe fn alloc(layout:
Layout
) ->
*mut
u8
Expand description
Allocates memory with the global allocator.
This function forwards calls to the
GlobalAlloc::alloc
method
of the allocator registered with the
#[global_allocator]
attribute
if there is one, or the
std
crate’s default.
This function is expected to be deprecated in favor of the
allocate
method
of the
Global
type when it and the
Allocator
trait become stable.
§
Safety
See
GlobalAlloc::alloc
.
§
Examples
use
std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
unsafe
{
let
layout = Layout::new::<u16>();
let
ptr = alloc(layout);
if
ptr.is_null() {
        handle_alloc_error(layout);
    }
*
(ptr
as
*mut
u16) =
42
;
assert_eq!
(
*
(ptr
as
*mut
u16),
42
);

    dealloc(ptr, layout);
}