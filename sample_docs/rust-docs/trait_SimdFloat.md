SimdFloat in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdFloat
Copy item path
Source
pub trait SimdFloat:
Copy
+ Sealed {
    type
Mask
;
    type
Scalar
;
    type
Bits
;
    type
Cast
<T:
SimdElement
>;
Show 24 methods
// Required methods
    fn
cast
<T>(self) -> Self::
Cast
<T>
where T:
SimdCast
;
unsafe fn
to_int_unchecked
<I>(self) -> Self::
Cast
<I>
where I:
SimdCast
,
             Self::
Scalar
:
FloatToInt
<I>
;
fn
to_bits
(self) -> Self::
Bits
;
fn
from_bits
(bits: Self::
Bits
) -> Self;
fn
abs
(self) -> Self;
fn
recip
(self) -> Self;
fn
to_degrees
(self) -> Self;
fn
to_radians
(self) -> Self;
fn
is_sign_positive
(self) -> Self::
Mask
;
fn
is_sign_negative
(self) -> Self::
Mask
;
fn
is_nan
(self) -> Self::
Mask
;
fn
is_infinite
(self) -> Self::
Mask
;
fn
is_finite
(self) -> Self::
Mask
;
fn
is_subnormal
(self) -> Self::
Mask
;
fn
is_normal
(self) -> Self::
Mask
;
fn
signum
(self) -> Self;
fn
copysign
(self, sign: Self) -> Self;
fn
simd_min
(self, other: Self) -> Self;
fn
simd_max
(self, other: Self) -> Self;
fn
simd_clamp
(self, min: Self, max: Self) -> Self;
fn
reduce_sum
(self) -> Self::
Scalar
;
fn
reduce_product
(self) -> Self::
Scalar
;
fn
reduce_max
(self) -> Self::
Scalar
;
fn
reduce_min
(self) -> Self::
Scalar
;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Operations on SIMD vectors of floats.
Required Associated Types
§
Source
type
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
type
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
type
Bits
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Bit representation of this SIMD vector type.
Source
type
Cast
<T:
SimdElement
>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Required Methods
§
Source
fn
cast
<T>(self) -> Self::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
This follows the semantics of Rust’s
as
conversion for floats (truncating or saturating
at the limits) for each element.
§
Example
let
floats: Simd<f32,
4
> = Simd::from_array([
1.9
, -
4.5
, f32::INFINITY, f32::NAN]);
let
ints = floats.cast::<i32>();
assert_eq!
(ints, Simd::from_array([
1
, -
4
, i32::MAX,
0
]));
// Formally equivalent, but `Simd::cast` can optimize better.
assert_eq!
(ints, Simd::from_array(floats.to_array().map(|x| x
as
i32)));
// The float conversion does not round-trip.
let
floats_again = ints.cast();
assert_ne!
(floats, floats_again);
assert_eq!
(floats_again, Simd::from_array([
1.0
, -
4.0
,
2147483647.0
,
0.0
]));
Source
unsafe fn
to_int_unchecked
<I>(self) -> Self::
Cast
<I>
where
    I:
SimdCast
,
    Self::
Scalar
:
FloatToInt
<I>,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds toward zero and converts to the same-width integer type, assuming that
the value is finite and fits in that type.
§
Safety
The value must:
Not be NaN
Not be infinite
Be representable in the return type, after truncating off its fractional part
If these requirements are infeasible or costly, consider using the safe function
cast
,
which saturates on conversion.
Source
fn
to_bits
(self) -> Self::
Bits
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation to an unsigned integer vector type with the
same size and number of elements.
Source
fn
from_bits
(bits: Self::
Bits
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation from an unsigned integer vector type with the
same size and number of elements.
Source
fn
abs
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the absolute value of the
equivalently-indexed element in
self
.
Source
fn
recip
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Takes the reciprocal (inverse) of each element,
1/x
.
Source
fn
to_degrees
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from radians to degrees.
Source
fn
to_radians
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from degrees to radians.
Source
fn
is_sign_positive
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a positive sign, including
+0.0
,
NaN
s with positive sign bit and positive infinity.
Source
fn
is_sign_negative
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a negative sign, including
-0.0
,
NaN
s with negative sign bit and negative infinity.
Source
fn
is_nan
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is
NaN
.
Source
fn
is_infinite
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is positive infinity or negative infinity.
Source
fn
is_finite
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither infinite nor
NaN
.
Source
fn
is_subnormal
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is subnormal.
Source
fn
is_normal
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither zero, infinite,
subnormal, nor
NaN
.
Source
fn
signum
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Replaces each element with a number that represents its sign.
1.0
if the number is positive,
+0.0
, or
INFINITY
-1.0
if the number is negative,
-0.0
, or
NEG_INFINITY
NAN
if the number is
NAN
Source
fn
copysign
(self, sign: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns each element with the magnitude of
self
and the sign of
sign
.
For any element containing a
NAN
, a
NAN
with the sign of
sign
is returned.
Source
fn
simd_min
(self, other: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum of each element.
If one of the values is
NAN
, then the other value is returned.
Source
fn
simd_max
(self, other: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum of each element.
If one of the values is
NAN
, then the other value is returned.
Source
fn
simd_clamp
(self, min: Self, max: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval unless it is NaN.
For each element in
self
, returns the corresponding element in
max
if the element is
greater than
max
, and the corresponding element in
min
if the element is less
than
min
.  Otherwise returns the element in
self
.
Source
fn
reduce_sum
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector.
§
Examples
let
v = f32x2::from_array([
1.
,
2.
]);
assert_eq!
(v.reduce_sum(),
3.
);
Source
fn
reduce_product
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reducing multiply.  Returns the product of the elements of the vector.
§
Examples
let
v = f32x2::from_array([
3.
,
4.
]);
assert_eq!
(v.reduce_product(),
12.
);
Source
fn
reduce_max
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Returns values based on equality, so a vector containing both
0.
and
-0.
may
return either.
This function will not return
NaN
unless all elements are
NaN
.
§
Examples
let
v = f32x2::from_array([
1.
,
2.
]);
assert_eq!
(v.reduce_max(),
2.
);
// NaN values are skipped...
let
v = f32x2::from_array([
1.
, f32::NAN]);
assert_eq!
(v.reduce_max(),
1.
);
// ...unless all values are NaN
let
v = f32x2::from_array([f32::NAN, f32::NAN]);
assert!
(v.reduce_max().is_nan());
Source
fn
reduce_min
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Returns values based on equality, so a vector containing both
0.
and
-0.
may
return either.
This function will not return
NaN
unless all elements are
NaN
.
§
Examples
let
v = f32x2::from_array([
3.
,
7.
]);
assert_eq!
(v.reduce_min(),
3.
);
// NaN values are skipped...
let
v = f32x2::from_array([
1.
, f32::NAN]);
assert_eq!
(v.reduce_min(),
1.
);
// ...unless all values are NaN
let
v = f32x2::from_array([f32::NAN, f32::NAN]);
assert!
(v.reduce_min().is_nan());
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
impl<const N:
usize
>
SimdFloat
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i32
as
SimdElement
>::
Mask
, N>
Source
§
type
Scalar
=
f32
Source
§
type
Bits
=
Simd
<
u32
, N>
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
Source
§
impl<const N:
usize
>
SimdFloat
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i64
as
SimdElement
>::
Mask
, N>
Source
§
type
Scalar
=
f64
Source
§
type
Bits
=
Simd
<
u64
, N>
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>