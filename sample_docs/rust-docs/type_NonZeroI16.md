NonZeroI16 in std::num - Rust
std
::
num
Type Alias
NonZeroI16
Copy item path
1.34.0
·
Source
pub type NonZeroI16 =
NonZero
<
i16
>;
Expand description
An
i16
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroI16>
is the same size as
i16
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroI16>>(), size_of::<i16>());
§
Layout
NonZeroI16
is guaranteed to have the same layout and bit validity as
i16
with the exception that
0
is not a valid instance.
Option<NonZeroI16>
is guaranteed to be compatible with
i16
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroI16
and
Option<NonZeroI16>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroI16;
assert_eq!
(size_of::<NonZeroI16>(), size_of::<
Option
<NonZeroI16>>());
assert_eq!
(align_of::<NonZeroI16>(), align_of::<
Option
<NonZeroI16>>());
Aliased Type
§
struct NonZeroI16(
/* private fields */
);