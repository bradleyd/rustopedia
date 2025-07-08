simd_as in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_as
Copy item path
Source
pub unsafe fn simd_as<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Numerically casts a vector, elementwise.
T
and
U
be a vectors of integers or floats, and must have the same length.
Like
simd_cast
, but saturates float-to-integer conversions (NaN becomes 0).
This matches regular
as
and is always safe.
When casting floats to integers, the result is truncated.
When casting integers to floats, the result is rounded.
Otherwise, truncates or extends the value, maintaining the sign for signed integers.