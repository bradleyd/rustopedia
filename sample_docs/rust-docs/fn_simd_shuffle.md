simd_shuffle in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_shuffle
Copy item path
Source
pub unsafe fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Shuffles two vectors by const indices.
T
must be a vector.
U
must be a
const
vector of
u32
s. This means it must either refer to a named
const or be given as an inline const expression (
const { ... }
).
V
must be a vector with the same element type as
T
and the same length as
U
.
Returns a new vector such that element
i
is selected from
xy[idx[i]]
, where
xy
is the concatenation of
x
and
y
. It is a compile-time error if
idx[i]
is out-of-bounds
of
xy
.