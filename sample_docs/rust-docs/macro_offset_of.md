offset_of in std::mem - Rust
std
::
mem
Macro
offset_of
Copy item path
1.77.0
·
Source
pub macro offset_of($Container:ty, $($fields:expr)+ $(,)?) {
    ...
}
Expand description
Expands to the offset in bytes of a field from the beginning of the given type.
The type may be a
struct
,
enum
,
union
, or tuple.
The field may be a nested field (
field1.field2
), but not an array index.
The field must be visible to the call site.
The offset is returned as a
usize
.
§
Offsets of, and in, dynamically sized types
The field’s type must be
Sized
, but it may be located in a
dynamically sized
container.
If the field type is dynamically sized, then you cannot use
offset_of!
(since the field’s
alignment, and therefore its offset, may also be dynamic) and must take the offset from an
actual pointer to the container instead.
#[repr(C)]
pub struct
Struct<T:
?
Sized> {
    a: u8,
    b: T,
}
#[derive(Debug)]
#[repr(C, align(
4
))]
struct
Align4(u32);
assert_eq!
(
mem::offset_of!
(Struct<
dyn
Debug>, a),
0
);
// OK — Sized field
assert_eq!
(
mem::offset_of!
(Struct<Align4>, b),
4
);
// OK — not DST

// assert_eq!(mem::offset_of!(Struct<dyn Debug>, b), 1);
// ^^^ error[E0277]: ... cannot be known at compilation time

// To obtain the offset of a !Sized field, examine a concrete value
// instead of using offset_of!.
let
value: Struct<Align4> = Struct { a:
1
, b: Align4(
2
) };
let
ref_unsized:
&
Struct<
dyn
Debug> =
&
value;
let
offset_of_b =
unsafe
{
    (
&
raw
const
ref_unsized.b).byte_offset_from_unsigned(ref_unsized)
};
assert_eq!
(offset_of_b,
4
);
If you need to obtain the offset of a field of a
!Sized
type, then, since the offset may
depend on the particular value being stored (in particular,
dyn Trait
values have a
dynamically-determined alignment), you must retrieve the offset from a specific reference
or pointer, and so you cannot use
offset_of!
to work without one.
§
Layout is subject to change
Note that type layout is, in general,
subject to change and
platform-specific
. If
layout stability is required, consider using an
explicit
repr
attribute
.
Rust guarantees that the offset of a given field within a given type will not
change over the lifetime of the program. However, two different compilations of
the same program may result in different layouts. Also, even within a single
program execution, no guarantees are made about types which are
similar
but
not
identical
, e.g.:
struct
Wrapper<T, U>(T, U);
type
A = Wrapper<u8, u8>;
type
B = Wrapper<u8, i8>;
// Not necessarily identical even though `u8` and `i8` have the same layout!
// assert_eq!(mem::offset_of!(A, 1), mem::offset_of!(B, 1));
#[repr(transparent)]
struct
U8(u8);
type
C = Wrapper<u8, U8>;
// Not necessarily identical even though `u8` and `U8` have the same layout!
// assert_eq!(mem::offset_of!(A, 1), mem::offset_of!(C, 1));
struct
Empty<T>(core::marker::PhantomData<T>);
// Not necessarily identical even though `PhantomData` always has the same layout!
// assert_eq!(mem::offset_of!(Empty<u8>, 0), mem::offset_of!(Empty<i8>, 0));
§
Unstable features
The following unstable features expand the functionality of
offset_of!
:
offset_of_enum
— allows
enum
variants to be traversed as if they were fields.
offset_of_slice
— allows getting the offset of a field of type
[T]
.
§
Examples
use
std::mem;
#[repr(C)]
struct
FieldStruct {
    first: u8,
    second: u16,
    third: u8
}
assert_eq!
(
mem::offset_of!
(FieldStruct, first),
0
);
assert_eq!
(
mem::offset_of!
(FieldStruct, second),
2
);
assert_eq!
(
mem::offset_of!
(FieldStruct, third),
4
);
#[repr(C)]
struct
NestedA {
    b: NestedB
}
#[repr(C)]
struct
NestedB(u8);
assert_eq!
(
mem::offset_of!
(NestedA, b.
0
),
0
);