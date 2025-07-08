mask8x4 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
mask8x4
Copy item path
Source
pub type mask8x4 =
Mask
<
i8
, 4>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with four elements for vectors with 8-bit element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[i8; 4]
.
Aliased Type
§
struct mask8x4(
/* private fields */
);