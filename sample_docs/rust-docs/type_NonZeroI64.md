NonZeroI64 in std::num - Rust
std
::
num
Type Alias
NonZeroI64
Copy item path
1.34.0
·
Source
pub type NonZeroI64 =
NonZero
<
i64
>;
Expand description
An
i64
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroI64>
is the same size as
i64
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroI64>>(), size_of::<i64>());
§
Layout
NonZeroI64
is guaranteed to have the same layout and bit validity as
i64
with the exception that
0
is not a valid instance.
Option<NonZeroI64>
is guaranteed to be compatible with
i64
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroI64
and
Option<NonZeroI64>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroI64;
assert_eq!
(size_of::<NonZeroI64>(), size_of::<
Option
<NonZeroI64>>());
assert_eq!
(align_of::<NonZeroI64>(), align_of::<
Option
<NonZeroI64>>());
Aliased Type
§
struct NonZeroI64(
/* private fields */
);