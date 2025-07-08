write_unaligned in std::ptr - Rust
std
::
ptr
Function
write_unaligned
Copy item path
1.17.0 (const: 1.83.0)
·
Source
pub const unsafe fn write_unaligned<T>(dst:
*mut T
, src: T)
Expand description
Overwrites a memory location with the given value without reading or
dropping the old value.
Unlike
write()
, the pointer may be unaligned.
write_unaligned
does not drop the contents of
dst
. This is safe, but it
could leak allocations or resources, so care should be taken not to overwrite
an object that should be dropped.
Additionally, it does not drop
src
. Semantically,
src
is moved into the
location pointed to by
dst
.
This is appropriate for initializing uninitialized memory, or overwriting
memory that has previously been read with
read_unaligned
.
§
Safety
Behavior is undefined if any of the following conditions are violated:
dst
must be
valid
for writes.
§
On
packed
structs
Attempting to create a raw pointer to an
unaligned
struct field with
an expression such as
&packed.unaligned as *const FieldType
creates an
intermediate unaligned reference before converting that to a raw pointer.
That this reference is temporary and immediately cast is inconsequential
as the compiler always expects references to be properly aligned.
As a result, using
&packed.unaligned as *const FieldType
causes immediate
undefined behavior
in your program.
Instead, you must use the
&raw mut
syntax to create the pointer.
You may use that constructed pointer together with this function.
An example of how to do it and how this relates to
write_unaligned
is:
#[repr(packed, C)]
struct
Packed {
    _padding: u8,
    unaligned: u32,
}
let
mut
packed: Packed =
unsafe
{ std::mem::zeroed() };
// Take the address of a 32-bit integer which is not aligned.
// In contrast to `&packed.unaligned as *mut _`, this has no undefined behavior.
let
unaligned =
&
raw
mut
packed.unaligned;
unsafe
{ std::ptr::write_unaligned(unaligned,
42
) };
assert_eq!
({packed.unaligned},
42
);
// `{...}` forces copying the field instead of creating a reference.
Accessing unaligned fields directly with e.g.
packed.unaligned
is safe however
(as can be seen in the
assert_eq!
above).
§
Examples
Write a
usize
value to a byte buffer:
fn
write_usize(x:
&mut
[u8], val: usize) {
assert!
(x.len() >= size_of::<usize>());
let
ptr = x.as_mut_ptr()
as
*mut
usize;
unsafe
{ ptr.write_unaligned(val) }
}