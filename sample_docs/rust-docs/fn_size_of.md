size_of in std::mem - Rust
std
::
mem
Function
size_of
Copy item path
1.0.0 (const: 1.24.0)
·
Source
pub const fn size_of<T>() ->
usize
Expand description
Returns the size of a type in bytes.
More specifically, this is the offset in bytes between successive elements
in an array with that item type including alignment padding. Thus, for any
type
T
and length
n
,
[T; n]
has a size of
n * size_of::<T>()
.
In general, the size of a type is not stable across compilations, but
specific types such as primitives are.
The following table gives the size for primitives.
Type
size_of::<Type>()
()
0
bool
1
u8
1
u16
2
u32
4
u64
8
u128
16
i8
1
i16
2
i32
4
i64
8
i128
16
f32
4
f64
8
char
4
Furthermore,
usize
and
isize
have the same size.
The types
*const T
,
&T
,
Box<T>
,
Option<&T>
, and
Option<Box<T>>
all have
the same size. If
T
is
Sized
, all of those types have the same size as
usize
.
The mutability of a pointer does not change its size. As such,
&T
and
&mut T
have the same size. Likewise for
*const T
and
*mut T
.
§
Size of
#[repr(C)]
items
The
C
representation for items has a defined layout. With this layout,
the size of items is also stable as long as all fields have a stable size.
§
Size of Structs
For
struct
s, the size is determined by the following algorithm.
For each field in the struct ordered by declaration order:
Add the size of the field.
Round up the current size to the nearest multiple of the next field’s
alignment
.
Finally, round the size of the struct to the nearest multiple of its
alignment
.
The alignment of the struct is usually the largest alignment of all its
fields; this can be changed with the use of
repr(align(N))
.
Unlike
C
, zero sized structs are not rounded up to one byte in size.
§
Size of Enums
Enums that carry no data other than the discriminant have the same size as C enums
on the platform they are compiled for.
§
Size of Unions
The size of a union is the size of its largest field.
Unlike
C
, zero sized unions are not rounded up to one byte in size.
§
Examples
// Some primitives
assert_eq!
(
4
, size_of::<i32>());
assert_eq!
(
8
, size_of::<f64>());
assert_eq!
(
0
, size_of::<()>());
// Some arrays
assert_eq!
(
8
, size_of::<[i32;
2
]>());
assert_eq!
(
12
, size_of::<[i32;
3
]>());
assert_eq!
(
0
, size_of::<[i32;
0
]>());
// Pointer size equality
assert_eq!
(size_of::<
&
i32>(), size_of::<
*const
i32>());
assert_eq!
(size_of::<
&
i32>(), size_of::<Box<i32>>());
assert_eq!
(size_of::<
&
i32>(), size_of::<
Option
<
&
i32>>());
assert_eq!
(size_of::<Box<i32>>(), size_of::<
Option
<Box<i32>>>());
Using
#[repr(C)]
.
#[repr(C)]
struct
FieldStruct {
    first: u8,
    second: u16,
    third: u8
}
// The size of the first field is 1, so add 1 to the size. Size is 1.
// The alignment of the second field is 2, so add 1 to the size for padding. Size is 2.
// The size of the second field is 2, so add 2 to the size. Size is 4.
// The alignment of the third field is 1, so add 0 to the size for padding. Size is 4.
// The size of the third field is 1, so add 1 to the size. Size is 5.
// Finally, the alignment of the struct is 2 (because the largest alignment amongst its
// fields is 2), so add 1 to the size for padding. Size is 6.
assert_eq!
(
6
, size_of::<FieldStruct>());
#[repr(C)]
struct
TupleStruct(u8, u16, u8);
// Tuple structs follow the same rules.
assert_eq!
(
6
, size_of::<TupleStruct>());
// Note that reordering the fields can lower the size. We can remove both padding bytes
// by putting `third` before `second`.
#[repr(C)]
struct
FieldStructOptimized {
    first: u8,
    third: u8,
    second: u16
}
assert_eq!
(
4
, size_of::<FieldStructOptimized>());
// Union size is the size of the largest field.
#[repr(C)]
union
ExampleUnion {
    smaller: u8,
    larger: u16
}
assert_eq!
(
2
, size_of::<ExampleUnion>());