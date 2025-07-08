masksizex64 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex64
Copy item path
Source
pub type masksizex64 =
Mask
<
isize
, 64>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with 64 elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 64]
.
Aliased Type
§
struct masksizex64(
/* private fields */
);