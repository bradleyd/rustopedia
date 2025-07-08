SimdInt in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdInt
Copy item path
Source
pub trait SimdInt:
Copy
+ Sealed {
    type
Mask
;
    type
Scalar
;
    type
Unsigned
;
    type
Cast
<T:
SimdElement
>;
Show 25 methods
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
saturating_add
(self, second: Self) -> Self;
fn
saturating_sub
(self, second: Self) -> Self;
fn
abs
(self) -> Self;
fn
abs_diff
(self, second: Self) -> Self::
Unsigned
;
fn
saturating_abs
(self) -> Self;
fn
saturating_neg
(self) -> Self;
fn
is_positive
(self) -> Self::
Mask
;
fn
is_negative
(self) -> Self::
Mask
;
fn
signum
(self) -> Self;
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
(self) -> Self::
Unsigned
;
fn
count_zeros
(self) -> Self::
Unsigned
;
fn
leading_zeros
(self) -> Self::
Unsigned
;
fn
trailing_zeros
(self) -> Self::
Unsigned
;
fn
leading_ones
(self) -> Self::
Unsigned
;
fn
trailing_ones
(self) -> Self::
Unsigned
;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Operations on SIMD vectors of signed integers.
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
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
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
core::i32::{MIN, MAX};
let
x = Simd::from_array([MIN,
0
,
1
, MAX]);
let
max = Simd::splat(MAX);
let
unsat = x + max;
let
sat = x.saturating_add(max);
assert_eq!
(unsat, Simd::from_array([-
1
, MAX, MIN, -
2
]));
assert_eq!
(sat, Simd::from_array([-
1
, MAX, MAX, MAX]));
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
core::i32::{MIN, MAX};
let
x = Simd::from_array([MIN, -
2
, -
1
, MAX]);
let
max = Simd::splat(MAX);
let
unsat = x - max;
let
sat = x.saturating_sub(max);
assert_eq!
(unsat, Simd::from_array([
1
, MAX, MIN,
0
]));
assert_eq!
(sat, Simd::from_array([MIN, MIN, MIN,
0
]));
Source
fn
abs
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
§
Examples
use
core::i32::{MIN, MAX};
let
xs = Simd::from_array([MIN, MIN +
1
, -
5
,
0
]);
assert_eq!
(xs.abs(), Simd::from_array([MIN, MAX,
5
,
0
]));
Source
fn
abs_diff
(self, second: Self) -> Self::
Unsigned
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
core::i32::{MIN, MAX};
let
a = Simd::from_array([MIN, MAX,
100
, -
100
]);
let
b = Simd::from_array([MAX, MIN, -
80
, -
120
]);
assert_eq!
(a.abs_diff(b), Simd::from_array([u32::MAX, u32::MAX,
180
,
20
]));
Source
fn
saturating_abs
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
§
Examples
use
core::i32::{MIN, MAX};
let
xs = Simd::from_array([MIN, -
2
,
0
,
3
]);
let
unsat = xs.abs();
let
sat = xs.saturating_abs();
assert_eq!
(unsat, Simd::from_array([MIN,
2
,
0
,
3
]));
assert_eq!
(sat, Simd::from_array([MAX,
2
,
0
,
3
]));
Source
fn
saturating_neg
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
§
Examples
use
core::i32::{MIN, MAX};
let
x = Simd::from_array([MIN, -
2
,
3
, MAX]);
let
unsat = -x;
let
sat = x.saturating_neg();
assert_eq!
(unsat, Simd::from_array([MIN,
2
, -
3
, MIN +
1
]));
assert_eq!
(sat, Simd::from_array([MAX,
2
, -
3
, MIN +
1
]));
Source
fn
is_positive
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
fn
is_negative
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
fn
signum
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
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
§
Examples
let
v = i32x4::from_array([
1
,
2
,
3
,
4
]);
assert_eq!
(v.reduce_sum(),
10
);
// SIMD integer addition is always wrapping
let
v = i32x4::from_array([i32::MAX,
1
,
0
,
0
]);
assert_eq!
(v.reduce_sum(), i32::MIN);
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
§
Examples
let
v = i32x4::from_array([
1
,
2
,
3
,
4
]);
assert_eq!
(v.reduce_product(),
24
);
// SIMD integer multiplication is always wrapping
let
v = i32x4::from_array([i32::MAX,
2
,
1
,
1
]);
assert!
(v.reduce_product() < i32::MAX);
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
§
Examples
let
v = i32x4::from_array([
1
,
2
,
3
,
4
]);
assert_eq!
(v.reduce_max(),
4
);
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
§
Examples
let
v = i32x4::from_array([
1
,
2
,
3
,
4
]);
assert_eq!
(v.reduce_min(),
1
);
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
(self) -> Self::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
fn
count_zeros
(self) -> Self::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
fn
leading_zeros
(self) -> Self::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
fn
trailing_zeros
(self) -> Self::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
fn
leading_ones
(self) -> Self::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
fn
trailing_ones
(self) -> Self::
Unsigned
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
SimdInt
for
Simd
<
i8
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
i8
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
i8
Source
§
type
Unsigned
=
Simd
<
u8
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
SimdInt
for
Simd
<
i16
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
i16
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
i16
Source
§
type
Unsigned
=
Simd
<
u16
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
SimdInt
for
Simd
<
i32
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
i32
Source
§
type
Unsigned
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
SimdInt
for
Simd
<
i64
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
i64
Source
§
type
Unsigned
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
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
isize
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
isize
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
isize
Source
§
type
Unsigned
=
Simd
<
usize
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