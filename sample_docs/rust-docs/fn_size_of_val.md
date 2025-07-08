size_of_val in std::mem - Rust
std
::
mem
Function
size_of_val
Copy item path
1.0.0 (const: 1.85.0)
·
Source
pub const fn size_of_val<T>(val:
&T
) ->
usize
where
    T: ?
Sized
,
Expand description
Returns the size of the pointed-to value in bytes.
This is usually the same as
size_of::<T>()
. However, when
T
has
no
statically-known size, e.g., a slice
[T]
or a
trait object
,
then
size_of_val
can be used to get the dynamically-known size.
§
Examples
assert_eq!
(
4
, size_of_val(
&
5i32
));
let
x: [u8;
13
] = [
0
;
13
];
let
y:
&
[u8] =
&
x;
assert_eq!
(
13
, size_of_val(y));