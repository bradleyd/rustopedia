transmute_copy in std::mem - Rust
std
::
mem
Function
transmute_copy
Copy item path
1.0.0 (const: 1.74.0)
·
Source
pub const unsafe fn transmute_copy<Src, Dst>(src:
&Src
) -> Dst
Expand description
Interprets
src
as having type
&Dst
, and then reads
src
without moving
the contained value.
This function will unsafely assume the pointer
src
is valid for
size_of::<Dst>
bytes by transmuting
&Src
to
&Dst
and then reading the
&Dst
(except that this is done
in a way that is correct even when
&Dst
has stricter alignment requirements than
&Src
).
It will also unsafely create a copy of the contained value instead of moving out of
src
.
It is not a compile-time error if
Src
and
Dst
have different sizes, but it
is highly encouraged to only invoke this function where
Src
and
Dst
have the
same size. This function triggers
undefined behavior
if
Dst
is larger than
Src
.
§
Examples
use
std::mem;
#[repr(packed)]
struct
Foo {
    bar: u8,
}
let
foo_array = [
10u8
];
unsafe
{
// Copy the data from 'foo_array' and treat it as a 'Foo'
let
mut
foo_struct: Foo = mem::transmute_copy(
&
foo_array);
assert_eq!
(foo_struct.bar,
10
);
// Modify the copied data
foo_struct.bar =
20
;
assert_eq!
(foo_struct.bar,
20
);
}
// The contents of 'foo_array' should not have changed
assert_eq!
(foo_array, [
10
]);