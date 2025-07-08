NonZeroUsize in std::num - Rust
std
::
num
Type Alias
NonZeroUsize
Copy item path
1.28.0
·
Source
pub type NonZeroUsize =
NonZero
<
usize
>;
Expand description
A
usize
that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZeroUsize>
is the same size as
usize
:
assert_eq!
(size_of::<
Option
<core::num::NonZeroUsize>>(), size_of::<usize>());
§
Layout
NonZeroUsize
is guaranteed to have the same layout and bit validity as
usize
with the exception that
0
is not a valid instance.
Option<NonZeroUsize>
is guaranteed to be compatible with
usize
,
including in FFI.
Thanks to the
null pointer optimization
,
NonZeroUsize
and
Option<NonZeroUsize>
are guaranteed to have the same size and alignment:
use
std::num::NonZeroUsize;
assert_eq!
(size_of::<NonZeroUsize>(), size_of::<
Option
<NonZeroUsize>>());
assert_eq!
(align_of::<NonZeroUsize>(), align_of::<
Option
<NonZeroUsize>>());
Aliased Type
§
struct NonZeroUsize(
/* private fields */
);