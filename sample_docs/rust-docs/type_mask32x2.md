mask32x2 in std::simd::prelude - Rust
std
::
simd
::
prelude
Type Alias
mask32x2
Copy item path
Source
pub type mask32x2 =
Mask
<
i32
, 2>;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD mask with two elements for vectors with 32-bit element types.
The layout of this type is unspecified, and may change between platforms and/or Rust versions, and code should not assume that it is equivalent to
[i32; 2]
.
Aliased Type
§
struct mask32x2(
/* private fields */
);