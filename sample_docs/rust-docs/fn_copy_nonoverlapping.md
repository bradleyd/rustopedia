copy_nonoverlapping in std::intrinsics - Rust
std
::
intrinsics
Function
copy_nonoverlapping
Copy item path
1.0.0 (const: 1.83.0)
·
Source
pub const unsafe fn copy_nonoverlapping<T>(
    src:
*const T
,
    dst:
*mut T
,
    count:
usize
,
)
👎
Deprecated: import this function via
std::mem
instead
Expand description
Copies
count * size_of::<T>()
bytes from
src
to
dst
. The source
and destination must
not
overlap.
For regions of memory which might overlap, use
copy
instead.
copy_nonoverlapping
is semantically equivalent to C’s
memcpy
, but
with the source and destination arguments swapped,
and
count
counting the number of
T
s instead of bytes.
The copy is “untyped” in the sense that data may be uninitialized or otherwise violate the
requirements of
T
. The initialization state is preserved exactly.
§
Safety
Behavior is undefined if any of the following conditions are violated:
src
must be
valid
for reads of
count * size_of::<T>()
bytes.
dst
must be
valid
for writes of
count * size_of::<T>()
bytes.
Both
src
and
dst
must be properly aligned.
The region of memory beginning at
src
with a size of
count * size_of::<T>()
bytes must
not
overlap with the region of memory
beginning at
dst
with the same size.
Like
read
,
copy_nonoverlapping
creates a bitwise copy of
T
, regardless of
whether
T
is
Copy
. If
T
is not
Copy
, using
both
the values
in the region beginning at
*src
and the region beginning at
*dst
can
violate memory safety
.
Note that even if the effectively copied size (
count * size_of::<T>()
) is
0
, the pointers must be properly aligned.
§
Examples
Manually implement
Vec::append
:
use
std::ptr;
/// Moves all the elements of `src` into `dst`, leaving `src` empty.
fn
append<T>(dst:
&mut
Vec<T>, src:
&mut
Vec<T>) {
let
src_len = src.len();
let
dst_len = dst.len();
// Ensure that `dst` has enough capacity to hold all of `src`.
dst.reserve(src_len);
unsafe
{
// The call to add is always safe because `Vec` will never
        // allocate more than `isize::MAX` bytes.
let
dst_ptr = dst.as_mut_ptr().add(dst_len);
let
src_ptr = src.as_ptr();
// Truncate `src` without dropping its contents. We do this first,
        // to avoid problems in case something further down panics.
src.set_len(
0
);
// The two regions cannot overlap because mutable references do
        // not alias, and two different vectors cannot own the same
        // memory.
ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
// Notify `dst` that it now holds the contents of `src`.
dst.set_len(dst_len + src_len);
    }
}
let
mut
a =
vec!
[
'r'
];
let
mut
b =
vec!
[
'u'
,
's'
,
't'
];

append(
&mut
a,
&mut
b);
assert_eq!
(a,
&
[
'r'
,
'u'
,
's'
,
't'
]);
assert!
(b.is_empty());