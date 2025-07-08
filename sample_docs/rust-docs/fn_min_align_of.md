min_align_of in std::mem - Rust
std
::
mem
Function
min_align_of
Copy item path
1.0.0
·
Source
pub fn min_align_of<T>() ->
usize
👎
Deprecated since 1.2.0: use
align_of
instead
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
use
std::mem;
assert_eq!
(
4
, mem::min_align_of::<i32>());