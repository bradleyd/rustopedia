min_align_of_val in std::mem - Rust
std
::
mem
Function
min_align_of_val
Copy item path
1.0.0
·
Source
pub fn min_align_of_val<T>(val:
&T
) ->
usize
where
    T: ?
Sized
,
👎
Deprecated since 1.2.0: use
align_of_val
instead
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
use
std::mem;
assert_eq!
(
4
, mem::min_align_of_val(
&
5i32
));