MaskElement in std::simd - Rust
std
::
simd
Trait
MaskElement
Copy item path
Source
pub unsafe trait MaskElement:
SimdElement
<Mask = Self>
    +
SimdCast
+ Sealed { }
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Marker trait for types that may be used as SIMD mask elements.
§
Safety
Type must be a signed integer.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
MaskElement
for
i8
Source
§
impl
MaskElement
for
i16
Source
§
impl
MaskElement
for
i32
Source
§
impl
MaskElement
for
i64
Source
§
impl
MaskElement
for
isize