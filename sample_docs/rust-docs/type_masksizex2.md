masksizex2 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex2
Copy item path
Source
pub type masksizex2 =
Mask
<
isize
, 2>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with two elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 2]
.
Aliased Type
§
struct masksizex2(
/* private fields */
);