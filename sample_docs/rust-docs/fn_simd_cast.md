simd_cast in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_cast
Copy item path
Source
pub unsafe fn simd_cast<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Numerically casts a vector, elementwise.
T
and
U
must be vectors of integers or floats, and must have the same length.
When casting floats to integers, the result is truncated. Out-of-bounds result lead to UB.
When casting integers to floats, the result is rounded.
Otherwise, truncates or extends the value, maintaining the sign for signed integers.
§
Safety
Casting from integer types is always safe.
Casting between two float types is also always safe.
Casting floats to integers truncates, following the same rules as
to_int_unchecked
.
Specifically, each element must:
Not be
NaN
Not be infinite
Be representable in the return type, after truncating off its fractional part