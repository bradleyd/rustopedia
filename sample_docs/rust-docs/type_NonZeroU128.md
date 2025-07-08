NonZeroU128 in std::num - Rust
std
::
num
Type Alias
NonZeroU128
Copy item path
1.28.0
·
Source
pub type NonZeroU128 =
NonZero
<
u128
>;
Expand description
A
u128
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroU128>
is the same size as
u128
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroU128>>(), size_of::<u128>());
§
Layout
NonZeroU128
is guaranteed to have the same layout and bit validity as
u128
with the exception that
0
is not a valid instance.
Option<NonZeroU128>
is guaranteed to be compatible with
u128
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroU128
and
Option<NonZeroU128>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroU128;
assert_eq!
(size_of::<NonZeroU128>(), size_of::<
Option
<NonZeroU128>>());
assert_eq!
(align_of::<NonZeroU128>(), align_of::<
Option
<NonZeroU128>>());
Aliased Type
§
struct NonZeroU128(
/* private fields */
);