compare_bytes in std::intrinsics - Rust
std
::
intrinsics
Function
compare_bytes
Copy item path
Source
pub const unsafe fn compare_bytes(
    left:
*const
u8
,
    right:
*const
u8
,
    bytes:
usize
,
) ->
i32
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Lexicographically compare
[left, left + bytes)
and
[right, right + bytes)
as unsigned bytes, returning negative if
left
is less, zero if all the
bytes match, or positive if
left
is greater.
This underlies things like
<[u8]>::cmp
, and will usually lower to
memcmp
.
§
Safety
left
and
right
must each be
valid
for reads of
bytes
bytes.
Note that this applies to the whole range, not just until the first byte
that differs.  That allows optimizations that can read in large chunks.