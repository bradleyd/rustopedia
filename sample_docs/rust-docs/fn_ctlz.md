ctlz in std::intrinsics - Rust
std
::
intrinsics
Function
ctlz
Copy item path
Source
pub const fn ctlz<T>(x: T) ->
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
Returns the number of leading unset bits (zeroes) in an integer type
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
leading_zeros
method. For example,
u32::leading_zeros
§
Examples
#![feature(core_intrinsics)]
use
std::intrinsics::ctlz;
let
x =
0b0001_1100_u8
;
let
num_leading = ctlz(x);
assert_eq!
(num_leading,
3
);
An
x
with value
0
will return the bit width of
T
.
#![feature(core_intrinsics)]
use
std::intrinsics::ctlz;
let
x =
0u16
;
let
num_leading = ctlz(x);
assert_eq!
(num_leading,
16
);