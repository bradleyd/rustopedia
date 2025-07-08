SimdUint in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdUint
Copy item path
Source
pub trait SimdUint:
Copy
+ Sealed {
    type
Scalar
;
    type
Cast
<T:
SimdElement
>;
Show 20 methods
// Required methods
    fn
cast
<T>(self) -> Self::
Cast
<T>
where T:
SimdCast
;
fn
wrapping_neg
(self) -> Self;
fn
saturating_add
(self, second: Self) -> Self;
fn
saturating_sub
(self, second: Self) -> Self;
fn
abs_diff
(self, second: Self) -> Self;
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
fn
reduce_and
(self) -> Self::
Scalar
;
fn
reduce_or
(self) -> Self::
Scalar
;
fn
reduce_xor
(self) -> Self::
Scalar
;
fn
swap_bytes
(self) -> Self;
fn
reverse_bits
(self) -> Self;
fn
count_ones
(self) -> Self;
fn
count_zeros
(self) -> Self;
fn
leading_zeros
(self) -> Self;
fn
trailing_zeros
(self) -> Self;
fn
leading_ones
(self) -> Self;
fn
trailing_ones
(self) -> Self;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Operations on SIMD vectors of unsigned integers.
Required Associated Types
§
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
conversion for casting integers (wrapping to
other integer types, and saturating to float types).
Source
fn
wrapping_neg
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Like
u32::wrapping_neg
, all applications of this function will wrap, with the exception
of
-0
.
Source
fn
saturating_add
(self, second: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
§
Examples
use
core::u32::MAX;
let
x = Simd::from_array([
2
,
1
,
0
, MAX]);
let
max = Simd::splat(MAX);
let
unsat = x + max;
let
sat = x.saturating_add(max);
assert_eq!
(unsat, Simd::from_array([
1
,
0
, MAX, MAX -
1
]));
assert_eq!
(sat, max);
Source
fn
saturating_sub
(self, second: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
§
Examples
use
core::u32::MAX;
let
x = Simd::from_array([
2
,
1
,
0
, MAX]);
let
max = Simd::splat(MAX);
let
unsat = x - max;
let
sat = x.saturating_sub(max);
assert_eq!
(unsat, Simd::from_array([
3
,
2
,
1
,
0
]));
assert_eq!
(sat, Simd::splat(
0
));
Source
fn
abs_diff
(self, second: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
§
Examples
use
core::u32::MAX;
let
a = Simd::from_array([
0
, MAX,
100
,
20
]);
let
b = Simd::from_array([MAX,
0
,
80
,
200
]);
assert_eq!
(a.abs_diff(b), Simd::from_array([MAX, MAX,
20
,
180
]));
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
Returns the sum of the elements of the vector, with wrapping addition.
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
Returns the product of the elements of the vector, with wrapping multiplication.
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
Source
fn
reduce_and
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
fn
reduce_or
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
fn
reduce_xor
(self) -> Self::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
fn
swap_bytes
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
fn
reverse_bits
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
fn
count_ones
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
fn
count_zeros
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
fn
leading_zeros
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
fn
trailing_zeros
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
fn
leading_ones
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
fn
trailing_ones
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
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
SimdUint
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u8
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
SimdUint
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u16
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
SimdUint
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u32
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
SimdUint
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u64
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
SimdUint
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
usize
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>