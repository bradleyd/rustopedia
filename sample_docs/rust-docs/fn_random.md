random in std::random - Rust
std
::
random
Function
random
Copy item path
Source
pub fn random<T:
Random
>() -> T
🔬
This is a nightly-only experimental API. (
random
#130703
)
Expand description
Generates a random value with the default random source.
This is a convenience function for
T::random(&mut DefaultRandomSource)
and
will sample according to the same distribution as the underlying
Random
trait implementation. See
DefaultRandomSource
for more information about
how randomness is sourced.
Warning:
Be careful when manipulating random values! The
random
method on integers samples them with a uniform
distribution, so a value of 1 is just as likely as
i32::MAX
. By using
modulo operations, some of the resulting values can become more likely than
others. Use audited crates when in doubt.
§
Examples
Generating a
version 4/variant 1 UUID
represented as text:
#![feature(random)]
use
std::random::random;
let
bits: u128 = random();
let
g1 = (bits >>
96
)
as
u32;
let
g2 = (bits >>
80
)
as
u16;
let
g3 = (
0x4000
| (bits >>
64
) &
0x0fff
)
as
u16;
let
g4 = (
0x8000
| (bits >>
48
) &
0x3fff
)
as
u16;
let
g5 = (bits &
0xffffffffffff
)
as
u64;
let
uuid =
format!
(
"{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}"
);
println!
(
"{uuid}"
);