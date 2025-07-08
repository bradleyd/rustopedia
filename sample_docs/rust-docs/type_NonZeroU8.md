NonZeroU8 in std::num - Rust
std
::
num
Type Alias
NonZeroU8
Copy item path
1.28.0
·
Source
pub type NonZeroU8 =
NonZero
<
u8
>;
Expand description
A
u8
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroU8>
is the same size as
u8
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroU8>>(), size_of::<u8>());
§
Layout
NonZeroU8
is guaranteed to have the same layout and bit validity as
u8
with the exception that
0
is not a valid instance.
Option<NonZeroU8>
is guaranteed to be compatible with
u8
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroU8
and
Option<NonZeroU8>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroU8;
assert_eq!
(size_of::<NonZeroU8>(), size_of::<
Option
<NonZeroU8>>());
assert_eq!
(align_of::<NonZeroU8>(), align_of::<
Option
<NonZeroU8>>());
Aliased Type
§
struct NonZeroU8(
/* private fields */
);