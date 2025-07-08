masksizex8 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex8
Copy item path
Source
pub type masksizex8 =
Mask
<
isize
, 8>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with eight elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 8]
.
Aliased Type
§
struct masksizex8(
/* private fields */
);