addr_of_mut in std::ptr - Rust
std
::
ptr
Macro
addr_of_mut
Copy item path
1.51.0
·
Source
pub macro addr_of_mut($place:expr) {
    ...
}
Expand description
Creates a
mut
raw pointer to a place, without creating an intermediate reference.
addr_of_mut!(expr)
is equivalent to
&raw mut expr
. The macro is
soft-deprecated
;
use
&raw mut
instead.
Creating a reference with
&
/
&mut
is only allowed if the pointer is properly aligned
and points to initialized data. For cases where those requirements do not hold,
raw pointers should be used instead. However,
&mut expr as *mut _
creates a reference
before casting it to a raw pointer, and that reference is subject to the same rules
as all other references. This macro can create a raw pointer
without
creating
a reference first.
§
Safety
The
expr
in
addr_of_mut!(expr)
is evaluated as a place expression, but never loads from the
place or requires the place to be dereferenceable. This means that
addr_of_mut!((*ptr).field)
still requires the projection to
field
to be in-bounds, using the same rules as
offset
.
However,
addr_of_mut!(*ptr)
is defined behavior even if
ptr
is null, dangling, or misaligned.
Note that
Deref
/
Index
coercions (and their mutable counterparts) are applied inside
addr_of_mut!
like everywhere else, in which case a reference is created to call
Deref::deref
or
Index::index
, respectively. The statements above only apply when no such coercions are
applied.
§
Examples
Correct usage: Creating a pointer to unaligned data
use
std::ptr;
#[repr(packed)]
struct
Packed {
    f1: u8,
    f2: u16,
}
let
mut
packed = Packed { f1:
1
, f2:
2
};
// `&mut packed.f2` would create an unaligned reference, and thus be Undefined Behavior!
let
raw_f2 =
ptr::addr_of_mut!
(packed.f2);
unsafe
{ raw_f2.write_unaligned(
42
); }
assert_eq!
({packed.f2},
42
);
// `{...}` forces copying the field instead of creating a reference.
Correct usage: Creating a pointer to uninitialized data
use
std::{ptr, mem::MaybeUninit};
struct
Demo {
    field: bool,
}
let
mut
uninit = MaybeUninit::<Demo>::uninit();
// `&uninit.as_mut().field` would create a reference to an uninitialized `bool`,
// and thus be Undefined Behavior!
let
f1_ptr =
unsafe
{
ptr::addr_of_mut!
((
*
uninit.as_mut_ptr()).field) };
unsafe
{ f1_ptr.write(
true
); }
let
init =
unsafe
{ uninit.assume_init() };
Incorrect usage: Out-of-bounds fields projection
use
std::ptr;
#[repr(C)]
struct
MyStruct {
    field1: i32,
    field2: i32,
}
let
ptr:
*mut
MyStruct = ptr::null_mut();
let
fieldptr =
unsafe
{
ptr::addr_of_mut!
((
*
ptr).field2) };
// Undefined Behavior ⚠️
The field projection
.field2
would offset the pointer by 4 bytes,
but the pointer is not in-bounds of an allocation for 4 bytes,
so this offset is Undefined Behavior.
See the
offset
docs for a full list of requirements for inbounds pointer arithmetic; the
same requirements apply to field projections, even inside
addr_of_mut!
. (In particular, it
makes no difference whether the pointer is null or dangling.)