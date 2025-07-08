cttz_nonzero in std::intrinsics - Rust
std
::
intrinsics
Function
cttz_nonzero
Copy item path
Source
pub const unsafe fn cttz_nonzero<T>(x: T) ->
u32
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Like
cttz
, but extra-unsafe as it returns
undef
when
given an
x
with value
0
.
This intrinsic does not have a stable counterpart.
§
Examples
#![feature(core_intrinsics)]
use
std::intrinsics::cttz_nonzero;
let
x =
0b0011_1000_u8
;
let
num_trailing =
unsafe
{ cttz_nonzero(x) };
assert_eq!
(num_trailing,
3
);