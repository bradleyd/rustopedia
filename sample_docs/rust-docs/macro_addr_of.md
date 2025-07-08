addr_of in std::ptr - Rust
std
::
ptr
Macro
addr_of
Copy item path
1.51.0
·
Source
pub macro addr_of($place:expr) {
    ...
}
Expand description
Creates a
const
raw pointer to a place, without creating an intermediate reference.
addr_of!(expr)
is equivalent to
&raw const expr
. The macro is
soft-deprecated
;
use
&raw const
instead.
It is still an open question under which conditions writing through an
addr_of!
-created
pointer is permitted. If the place
expr
evaluates to is based on a raw pointer, then the
result of
addr_of!
inherits all permissions from that raw pointer. However, if the place is
based on a reference, local variable, or
static
, then until all details are decided, the same
rules as for shared references apply: it is UB to write through a pointer created with this
operation, except for bytes located inside an
UnsafeCell
. Use
&raw mut
(or
addr_of_mut
)
to create a raw pointer that definitely permits mutation.
Creating a reference with
&
/
&mut
is only allowed if the pointer is properly aligned
and points to initialized data. For cases where those requirements do not hold,
raw pointers should be used instead. However,
&expr as *const _
creates a reference
before casting it to a raw pointer, and that reference is subject to the same rules
as all other references. This macro can create a raw pointer
without
creating
a reference first.
See
addr_of_mut
for how to create a pointer to uninitialized data.
Doing that with
addr_of
would not make much sense since one could only
read the data, and that would be Undefined Behavior.
§
Safety
The
expr
in
addr_of!(expr)
is evaluated as a place expression, but never loads from the
place or requires the place to be dereferenceable. This means that
addr_of!((*ptr).field)
still requires the projection to
field
to be in-bounds, using the same rules as
offset
.
However,
addr_of!(*ptr)
is defined behavior even if
ptr
is null, dangling, or misaligned.
Note that
Deref
/
Index
coercions (and their mutable counterparts) are applied inside
addr_of!
like everywhere else, in which case a reference is created to call
Deref::deref
or
Index::index
, respectively. The statements above only apply when no such coercions are
applied.
§
Example
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
packed = Packed { f1:
1
, f2:
2
};
// `&packed.f2` would create an unaligned reference, and thus be Undefined Behavior!
let
raw_f2 =
ptr::addr_of!
(packed.f2);
assert_eq!
(
unsafe
{ raw_f2.read_unaligned() },
2
);
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
*const
MyStruct = ptr::null();
let
fieldptr =
unsafe
{
ptr::addr_of!
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
addr_of!
. (In particular, it makes
no difference whether the pointer is null or dangling.)