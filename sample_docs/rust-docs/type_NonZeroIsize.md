NonZeroIsize in std::num - Rust
std
::
num
Type Alias
NonZeroIsize
Copy item path
1.34.0
·
Source
pub type NonZeroIsize =
NonZero
<
isize
>;
Expand description
An
isize
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroIsize>
is the same size as
isize
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroIsize>>(), size_of::<isize>());
§
Layout
NonZeroIsize
is guaranteed to have the same layout and bit validity as
isize
with the exception that
0
is not a valid instance.
Option<NonZeroIsize>
is guaranteed to be compatible with
isize
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroIsize
and
Option<NonZeroIsize>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroIsize;
assert_eq!
(size_of::<NonZeroIsize>(), size_of::<
Option
<NonZeroIsize>>());
assert_eq!
(align_of::<NonZeroIsize>(), align_of::<
Option
<NonZeroIsize>>());
Aliased Type
§
struct NonZeroIsize(
/* private fields */
);