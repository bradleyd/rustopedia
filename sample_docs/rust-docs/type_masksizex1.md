masksizex1 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex1
Copy item path
Source
pub type masksizex1 =
Mask
<
isize
, 1>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with one element for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 1]
.
Aliased Type
§
struct masksizex1(
/* private fields */
);