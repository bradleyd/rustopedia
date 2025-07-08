align_of in std::mem - Rust
std
::
mem
Function
align_of
Copy item path
1.0.0 (const: 1.24.0)
·
Source
pub const fn align_of<T>() ->
usize
Expand description
Returns the
ABI
-required minimum alignment of a type in bytes.
Every reference to a value of the type
T
must be a multiple of this number.
This is the alignment used for struct fields. It may be smaller than the preferred alignment.
§
Examples
assert_eq!
(
4
, align_of::<i32>());