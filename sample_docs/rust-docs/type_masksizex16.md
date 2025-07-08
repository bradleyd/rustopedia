masksizex16 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex16
Copy item path
Source
pub type masksizex16 =
Mask
<
isize
, 16>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with 16 elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 16]
.
Aliased Type
§
struct masksizex16(
/* private fields */
);