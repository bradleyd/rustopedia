simd_select_bitmask in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_select_bitmask
Copy item path
Source
pub unsafe fn simd_select_bitmask<M, T>(m: M, yes: T, no: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Selects elements from a bitmask.
M
must be an unsigned integer or array of
u8
, matching
simd_bitmask
.
T
must be a vector.
For each element, if the bit in
mask
is
1
, select the element from
if_true
.  If the corresponding bit in
mask
is
0
, select the element from
if_false
.
The bitmask bit order matches
simd_bitmask
.
§
Safety
Padding bits must be all zero.