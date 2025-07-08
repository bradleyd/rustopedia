NonZeroU16 in std::num - Rust
std
::
num
Type Alias
NonZeroU16
Copy item path
1.28.0
·
Source
pub type NonZeroU16 =
NonZero
<
u16
>;
Expand description
A
u16
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroU16>
is the same size as
u16
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroU16>>(), size_of::<u16>());
§
Layout
NonZeroU16
is guaranteed to have the same layout and bit validity as
u16
with the exception that
0
is not a valid instance.
Option<NonZeroU16>
is guaranteed to be compatible with
u16
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroU16
and
Option<NonZeroU16>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroU16;
assert_eq!
(size_of::<NonZeroU16>(), size_of::<
Option
<NonZeroU16>>());
assert_eq!
(align_of::<NonZeroU16>(), align_of::<
Option
<NonZeroU16>>());
Aliased Type
§
struct NonZeroU16(
/* private fields */
);