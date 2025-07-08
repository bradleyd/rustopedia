SimdCast in std::simd - Rust
std
::
simd
Trait
SimdCast
Copy item path
Source
pub trait SimdCast: Sealed +
SimdElement
{ }
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Supporting trait for
Simd::cast
.  Typically doesn’t need to be used directly.
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
SimdCast
for
f32
Source
§
impl
SimdCast
for
f64
Source
§
impl
SimdCast
for
i8
Source
§
impl
SimdCast
for
i16
Source
§
impl
SimdCast
for
i32
Source
§
impl
SimdCast
for
i64
Source
§
impl
SimdCast
for
isize
Source
§
impl
SimdCast
for
u8
Source
§
impl
SimdCast
for
u16
Source
§
impl
SimdCast
for
u32
Source
§
impl
SimdCast
for
u64
Source
§
impl
SimdCast
for
usize