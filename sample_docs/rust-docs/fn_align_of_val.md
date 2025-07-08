align_of_val in std::mem - Rust
std
::
mem
Function
align_of_val
Copy item path
1.0.0 (const: 1.85.0)
·
Source
pub const fn align_of_val<T>(val:
&T
) ->
usize
where
    T: ?
Sized
,
Expand description
Returns the
ABI
-required minimum alignment of the type of the value that
val
points to in
bytes.
Every reference to a value of the type
T
must be a multiple of this number.
§
Examples
assert_eq!
(
4
, align_of_val(
&
5i32
));