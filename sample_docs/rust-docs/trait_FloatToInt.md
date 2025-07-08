FloatToInt in std::convert - Rust
std
::
convert
Trait
FloatToInt
Copy item path
Source
pub trait FloatToInt<Int>:
Sized
+ Sealed { }
🔬
This is a nightly-only experimental API. (
convert_float_to_int
#67057
)
Expand description
Supporting trait for inherent methods of
f32
and
f64
such as
to_int_unchecked
.
Typically doesn’t need to be used directly.
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
FloatToInt
<
i8
> for
f16
Source
§
impl
FloatToInt
<
i8
> for
f32
Source
§
impl
FloatToInt
<
i8
> for
f64
Source
§
impl
FloatToInt
<
i8
> for
f128
Source
§
impl
FloatToInt
<
i16
> for
f16
Source
§
impl
FloatToInt
<
i16
> for
f32
Source
§
impl
FloatToInt
<
i16
> for
f64
Source
§
impl
FloatToInt
<
i16
> for
f128
Source
§
impl
FloatToInt
<
i32
> for
f16
Source
§
impl
FloatToInt
<
i32
> for
f32
Source
§
impl
FloatToInt
<
i32
> for
f64
Source
§
impl
FloatToInt
<
i32
> for
f128
Source
§
impl
FloatToInt
<
i64
> for
f16
Source
§
impl
FloatToInt
<
i64
> for
f32
Source
§
impl
FloatToInt
<
i64
> for
f64
Source
§
impl
FloatToInt
<
i64
> for
f128
Source
§
impl
FloatToInt
<
i128
> for
f16
Source
§
impl
FloatToInt
<
i128
> for
f32
Source
§
impl
FloatToInt
<
i128
> for
f64
Source
§
impl
FloatToInt
<
i128
> for
f128
Source
§
impl
FloatToInt
<
isize
> for
f16
Source
§
impl
FloatToInt
<
isize
> for
f32
Source
§
impl
FloatToInt
<
isize
> for
f64
Source
§
impl
FloatToInt
<
isize
> for
f128
Source
§
impl
FloatToInt
<
u8
> for
f16
Source
§
impl
FloatToInt
<
u8
> for
f32
Source
§
impl
FloatToInt
<
u8
> for
f64
Source
§
impl
FloatToInt
<
u8
> for
f128
Source
§
impl
FloatToInt
<
u16
> for
f16
Source
§
impl
FloatToInt
<
u16
> for
f32
Source
§
impl
FloatToInt
<
u16
> for
f64
Source
§
impl
FloatToInt
<
u16
> for
f128
Source
§
impl
FloatToInt
<
u32
> for
f16
Source
§
impl
FloatToInt
<
u32
> for
f32
Source
§
impl
FloatToInt
<
u32
> for
f64
Source
§
impl
FloatToInt
<
u32
> for
f128
Source
§
impl
FloatToInt
<
u64
> for
f16
Source
§
impl
FloatToInt
<
u64
> for
f32
Source
§
impl
FloatToInt
<
u64
> for
f64
Source
§
impl
FloatToInt
<
u64
> for
f128
Source
§
impl
FloatToInt
<
u128
> for
f16
Source
§
impl
FloatToInt
<
u128
> for
f32
Source
§
impl
FloatToInt
<
u128
> for
f64
Source
§
impl
FloatToInt
<
u128
> for
f128
Source
§
impl
FloatToInt
<
usize
> for
f16
Source
§
impl
FloatToInt
<
usize
> for
f32
Source
§
impl
FloatToInt
<
usize
> for
f64
Source
§
impl
FloatToInt
<
usize
> for
f128