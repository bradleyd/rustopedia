mask8x2 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
mask8x2
Copy item path
Source
pub type mask8x2 =
Mask
<
i8
, 2>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with two elements for vectors with 8-bit element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[i8; 2]
.
Aliased Type
§
struct mask8x2(
/* private fields */
);