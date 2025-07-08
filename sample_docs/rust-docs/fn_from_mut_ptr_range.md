from_mut_ptr_range in std::slice - Rust
std
::
slice
Function
from_mut_ptr_range
Copy item path
Source
pub const unsafe fn from_mut_ptr_range<'a, T>(
    range:
Range
<
*mut T
>,
) -> &'a mut
[T]
🔬
This is a nightly-only experimental API. (
slice_from_ptr_range
#89792
)
Expand description
Forms a mutable slice from a pointer range.
This is the same functionality as
from_ptr_range
, except that a
mutable slice is returned.
This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.
§
Safety
Behavior is undefined if any of the following conditions are violated:
The
start
pointer of the range must be a non-null,
valid
and properly aligned pointer
to the first element of a slice.
The
end
pointer must be a
valid
and properly aligned pointer to
one past
the last element, such that the offset from the end to the start pointer is
the length of the slice.
The entire memory range of this slice must be contained within a single allocated object!
Slices can never span across multiple allocated objects.
The range must contain
N
consecutive properly initialized values of type
T
.
The memory referenced by the returned slice must not be accessed through any other pointer
(not derived from the return value) for the duration of lifetime
'a
.
Both read and write accesses are forbidden.
The total length of the range must be no larger than
isize::MAX
,
and adding that size to
start
must not “wrap around” the address space.
See the safety documentation of
pointer::offset
.
Note that a range created from
slice::as_mut_ptr_range
fulfills these requirements.
§
Panics
This function panics if
T
is a Zero-Sized Type (“ZST”).
§
Caveat
The lifetime for the returned slice is inferred from its usage. To
prevent accidental misuse, it’s suggested to tie the lifetime to whichever
source lifetime is safe in the context, such as by providing a helper
function taking the lifetime of a host value for the slice, or by explicit
annotation.
§
Examples
#![feature(slice_from_ptr_range)]
use
core::slice;
let
mut
x = [
1
,
2
,
3
];
let
range = x.as_mut_ptr_range();
unsafe
{
assert_eq!
(slice::from_mut_ptr_range(range),
&mut
[
1
,
2
,
3
]);
}