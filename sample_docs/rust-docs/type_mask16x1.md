mask16x1 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
mask16x1
Copy item path
Source
pub type mask16x1 =
Mask
<
i16
, 1>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with one element for vectors with 16-bit element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[i16; 1]
.
Aliased Type
§
struct mask16x1(
/* private fields */
);