write_bytes in std::intrinsics - Rust
std
::
intrinsics
Function
write_bytes
Copy item path
1.0.0 (const: 1.83.0)
·
Source
pub const unsafe fn write_bytes<T>(dst:
*mut T
, val:
u8
, count:
usize
)
👎
Deprecated: import this function via
std::mem
instead
Expand description
Sets
count * size_of::<T>()
bytes of memory starting at
dst
to
val
.
write_bytes
is similar to C’s
memset
, but sets
count * size_of::<T>()
bytes to
val
.
§
Safety
Behavior is undefined if any of the following conditions are violated:
dst
must be
valid
for writes of
count * size_of::<T>()
bytes.
dst
must be properly aligned.
Note that even if the effectively copied size (
count * size_of::<T>()
) is
0
, the pointer must be properly aligned.
Additionally, note that changing
*dst
in this way can easily lead to undefined behavior (UB)
later if the written bytes are not a valid representation of some
T
. For instance, the
following is an
incorrect
use of this function:
unsafe
{
let
mut
value: u8 =
0
;
let
ptr:
*mut
bool =
&mut
value
as
*mut
u8
as
*mut
bool;
let
_bool = ptr.read();
// This is fine, `ptr` points to a valid `bool`.
ptr.write_bytes(
42u8
,
1
);
// This function itself does not cause UB...
let
_bool = ptr.read();
// ...but it makes this operation UB! ⚠️
}
§
Examples
Basic usage:
use
std::ptr;
let
mut
vec =
vec!
[
0u32
;
4
];
unsafe
{
let
vec_ptr = vec.as_mut_ptr();
    ptr::write_bytes(vec_ptr,
0xfe
,
2
);
}
assert_eq!
(vec, [
0xfefefefe
,
0xfefefefe
,
0
,
0
]);