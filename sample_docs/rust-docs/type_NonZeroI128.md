NonZeroI128 in std::num - Rust
std
::
num
Type Alias
NonZeroI128
Copy item path
1.34.0
·
Source
pub type NonZeroI128 =
NonZero
<
i128
>;
Expand description
An
i128
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroI128>
is the same size as
i128
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroI128>>(), size_of::<i128>());
§
Layout
NonZeroI128
is guaranteed to have the same layout and bit validity as
i128
with the exception that
0
is not a valid instance.
Option<NonZeroI128>
is guaranteed to be compatible with
i128
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroI128
and
Option<NonZeroI128>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroI128;
assert_eq!
(size_of::<NonZeroI128>(), size_of::<
Option
<NonZeroI128>>());
assert_eq!
(align_of::<NonZeroI128>(), align_of::<
Option
<NonZeroI128>>());
Aliased Type
§
struct NonZeroI128(
/* private fields */
);