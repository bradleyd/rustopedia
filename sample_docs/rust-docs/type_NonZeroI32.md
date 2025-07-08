NonZeroI32 in std::num - Rust
std
::
num
Type Alias
NonZeroI32
Copy item path
1.34.0
·
Source
pub type NonZeroI32 =
NonZero
<
i32
>;
Expand description
An
i32
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroI32>
is the same size as
i32
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroI32>>(), size_of::<i32>());
§
Layout
NonZeroI32
is guaranteed to have the same layout and bit validity as
i32
with the exception that
0
is not a valid instance.
Option<NonZeroI32>
is guaranteed to be compatible with
i32
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroI32
and
Option<NonZeroI32>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroI32;
assert_eq!
(size_of::<NonZeroI32>(), size_of::<
Option
<NonZeroI32>>());
assert_eq!
(align_of::<NonZeroI32>(), align_of::<
Option
<NonZeroI32>>());
Aliased Type
§
struct NonZeroI32(
/* private fields */
);