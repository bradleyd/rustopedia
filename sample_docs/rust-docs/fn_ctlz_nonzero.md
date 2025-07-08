ctlz_nonzero in std::intrinsics - Rust
std
::
intrinsics
Function
ctlz_nonzero
Copy item path
Source
pub const unsafe fn ctlz_nonzero<T>(x: T) ->
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
ctlz
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
std::intrinsics::ctlz_nonzero;
let
x =
0b0001_1100_u8
;
let
num_leading =
unsafe
{ ctlz_nonzero(x) };
assert_eq!
(num_leading,
3
);