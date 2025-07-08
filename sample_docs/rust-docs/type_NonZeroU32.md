NonZeroU32 in std::num - Rust
std
::
num
Type Alias
NonZeroU32
Copy item path
1.28.0
·
Source
pub type NonZeroU32 =
NonZero
<
u32
>;
Expand description
A
u32
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroU32>
is the same size as
u32
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroU32>>(), size_of::<u32>());
§
Layout
NonZeroU32
is guaranteed to have the same layout and bit validity as
u32
with the exception that
0
is not a valid instance.
Option<NonZeroU32>
is guaranteed to be compatible with
u32
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroU32
and
Option<NonZeroU32>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroU32;
assert_eq!
(size_of::<NonZeroU32>(), size_of::<
Option
<NonZeroU32>>());
assert_eq!
(align_of::<NonZeroU32>(), align_of::<
Option
<NonZeroU32>>());
Aliased Type
§
struct NonZeroU32(
/* private fields */
);