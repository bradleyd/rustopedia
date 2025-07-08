masksizex32 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
masksizex32
Copy item path
Source
pub type masksizex32 =
Mask
<
isize
, 32>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with 32 elements for vectors with pointer-sized element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[isize; 32]
.
Aliased Type
§
struct masksizex32(
/* private fields */
);