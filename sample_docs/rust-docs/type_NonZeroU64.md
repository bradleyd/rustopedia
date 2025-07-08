NonZeroU64 in std::num - Rust
std
::
num
Type Alias
NonZeroU64
Copy item path
1.28.0
·
Source
pub type NonZeroU64 =
NonZero
<
u64
>;
Expand description
A
u64
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroU64>
is the same size as
u64
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroU64>>(), size_of::<u64>());
§
Layout
NonZeroU64
is guaranteed to have the same layout and bit validity as
u64
with the exception that
0
is not a valid instance.
Option<NonZeroU64>
is guaranteed to be compatible with
u64
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroU64
and
Option<NonZeroU64>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroU64;
assert_eq!
(size_of::<NonZeroU64>(), size_of::<
Option
<NonZeroU64>>());
assert_eq!
(align_of::<NonZeroU64>(), align_of::<
Option
<NonZeroU64>>());
Aliased Type
§
struct NonZeroU64(
/* private fields */
);