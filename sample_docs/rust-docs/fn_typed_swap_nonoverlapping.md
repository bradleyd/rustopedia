typed_swap_nonoverlapping in std::intrinsics - Rust
std
::
intrinsics
Function
typed_swap_nonoverlapping
Copy item path
Source
pub const unsafe fn typed_swap_nonoverlapping<T>(x:
*mut T
, y:
*mut T
)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Non-overlapping
typed
swap of a single value.
The codegen backends will replace this with a better implementation when
T
is a simple type that can be loaded and stored as an immediate.
The stabilized form of this intrinsic is
crate::mem::swap
.
§
Safety
Behavior is undefined if any of the following conditions are violated:
Both
x
and
y
must be
valid
for both reads and writes.
Both
x
and
y
must be properly aligned.
The region of memory beginning at
x
must
not
overlap with the region of memory
beginning at
y
.
The memory pointed by
x
and
y
must both contain values of type
T
.