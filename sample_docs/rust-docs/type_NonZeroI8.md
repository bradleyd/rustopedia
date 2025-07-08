NonZeroI8 in std::num - Rust
std
::
num
Type Alias
NonZeroI8
Copy item path
1.34.0
·
Source
pub type NonZeroI8 =
NonZero
<
i8
>;
Expand description
An
i8
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroI8>
is the same size as
i8
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroI8>>(), size_of::<i8>());
§
Layout
NonZeroI8
is guaranteed to have the same layout and bit validity as
i8
with the exception that
0
is not a valid instance.
Option<NonZeroI8>
is guaranteed to be compatible with
i8
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroI8
and
Option<NonZeroI8>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroI8;
assert_eq!
(size_of::<NonZeroI8>(), size_of::<
Option
<NonZeroI8>>());
assert_eq!
(align_of::<NonZeroI8>(), align_of::<
Option
<NonZeroI8>>());
Aliased Type
§
struct NonZeroI8(
/* private fields */
);