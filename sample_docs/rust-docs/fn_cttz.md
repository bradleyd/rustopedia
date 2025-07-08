cttz in std::intrinsics - Rust
std
::
intrinsics
Function
cttz
Copy item path
Source
pub const fn cttz<T>(x: T) ->
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
Returns the number of trailing unset bits (zeroes) in an integer type
T
.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
trailing_zeros
method. For example,
u32::trailing_zeros
§
Examples
#![feature(core_intrinsics)]
use
std::intrinsics::cttz;
let
x =
0b0011_1000_u8
;
let
num_trailing = cttz(x);
assert_eq!
(num_trailing,
3
);
An
x
with value
0
will return the bit width of
T
:
#![feature(core_intrinsics)]
use
std::intrinsics::cttz;
let
x =
0u16
;
let
num_trailing = cttz(x);
assert_eq!
(num_trailing,
16
);