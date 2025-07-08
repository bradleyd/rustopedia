masksizex4 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex4
Copy item path
Source
pub type masksizex4 =
Mask
<
isize
, 4>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with four elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 4]
.
Aliased Type
§
struct masksizex4(
/* private fields */
);