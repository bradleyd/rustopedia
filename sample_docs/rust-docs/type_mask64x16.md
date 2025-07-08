mask64x16 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
mask64x16
Copy item path
Source
pub type mask64x16 =
Mask
<
i64
, 16>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with 16 elements for vectors with 64-bit element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[i64; 16]
.
Aliased Type
§
struct mask64x16(
/* private fields */
);