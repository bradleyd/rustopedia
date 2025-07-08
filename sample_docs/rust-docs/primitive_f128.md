f128 - Rust
Primitive Type
f128
Copy item path
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Expand description
A 128-bit floating-point type (specifically, the “binary128” type defined in IEEE 754-2008).
This type is very similar to
f32
and
f64
, but has increased precision by using twice
as many bits as
f64
. Please see
the documentation for
f32
or
Wikipedia on
quad-precision values
for more information.
Note that no platforms have hardware support for
f128
without enabling target specific features,
as for all instruction set architectures
f128
is considered an optional feature.  Only Power ISA
(“PowerPC”) and RISC-V (via the Q extension) specify it, and only certain microarchitectures
actually implement it. For x86-64 and AArch64, ISA support is not even specified, so it will always
be a software implementation significantly slower than
f64
.
Note:
f128
support is incomplete. Many platforms will not be able to link math functions. On
x86 in particular, these functions do link but their results are always incorrect.
See also the
std::f128::consts
module
.
Implementations
§
Source
§
impl
f128
Source
pub fn
floor
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the largest integer less than or equal to
self
.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
f =
3.7_f128
;
let
g =
3.0_f128
;
let
h = -
3.7_f128
;
assert_eq!
(f.floor(),
3.0
);
assert_eq!
(g.floor(),
3.0
);
assert_eq!
(h.floor(), -
4.0
);
Source
pub fn
ceil
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the smallest integer greater than or equal to
self
.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
f =
3.01_f128
;
let
g =
4.0_f128
;
assert_eq!
(f.ceil(),
4.0
);
assert_eq!
(g.ceil(),
4.0
);
Source
pub fn
round
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the nearest integer to
self
. If a value is half-way between two
integers, round away from
0.0
.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
f =
3.3_f128
;
let
g = -
3.3_f128
;
let
h = -
3.7_f128
;
let
i =
3.5_f128
;
let
j =
4.5_f128
;
assert_eq!
(f.round(),
3.0
);
assert_eq!
(g.round(), -
3.0
);
assert_eq!
(h.round(), -
4.0
);
assert_eq!
(i.round(),
4.0
);
assert_eq!
(j.round(),
5.0
);
Source
pub fn
round_ties_even
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the nearest integer to a number. Rounds half-way cases to the number
with an even least significant digit.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
f =
3.3_f128
;
let
g = -
3.3_f128
;
let
h =
3.5_f128
;
let
i =
4.5_f128
;
assert_eq!
(f.round_ties_even(),
3.0
);
assert_eq!
(g.round_ties_even(), -
3.0
);
assert_eq!
(h.round_ties_even(),
4.0
);
assert_eq!
(i.round_ties_even(),
4.0
);
Source
pub fn
trunc
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the integer part of
self
.
This means that non-integer numbers are always truncated towards zero.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
f =
3.7_f128
;
let
g =
3.0_f128
;
let
h = -
3.7_f128
;
assert_eq!
(f.trunc(),
3.0
);
assert_eq!
(g.trunc(),
3.0
);
assert_eq!
(h.trunc(), -
3.0
);
Source
pub fn
fract
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the fractional part of
self
.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
x =
3.6_f128
;
let
y = -
3.6_f128
;
let
abs_difference_x = (x.fract() -
0.6
).abs();
let
abs_difference_y = (y.fract() - (-
0.6
)).abs();
assert!
(abs_difference_x <= f128::EPSILON);
assert!
(abs_difference_y <= f128::EPSILON);
Source
pub fn
mul_add
(self, a:
f128
, b:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Fused multiply-add. Computes
(self * a) + b
with only one rounding
error, yielding a more accurate result than an unfused multiply-add.
Using
mul_add
may
be more performant than an unfused multiply-add if
the target architecture has a dedicated
fma
CPU instruction. However,
this is not always true, and will be heavily dependant on designing
algorithms with specific target hardware in mind.
§
Precision
The result of this operation is guaranteed to be the rounded
infinite-precision result. It is specified by IEEE 754 as
fusedMultiplyAdd
and guaranteed not to change.
§
Examples
#![feature(f128)]
let
m =
10.0_f128
;
let
x =
4.0_f128
;
let
b =
60.0_f128
;
assert_eq!
(m.mul_add(x, b),
100.0
);
assert_eq!
(m * x + b,
100.0
);
let
one_plus_eps =
1.0_f128
+ f128::EPSILON;
let
one_minus_eps =
1.0_f128
- f128::EPSILON;
let
minus_one = -
1.0_f128
;
// The exact result (1 + eps) * (1 - eps) = 1 - eps * eps.
assert_eq!
(one_plus_eps.mul_add(one_minus_eps, minus_one), -f128::EPSILON * f128::EPSILON);
// Different rounding with the non-fused multiply and add.
assert_eq!
(one_plus_eps * one_minus_eps + minus_one,
0.0
);
Source
pub fn
div_euclid
(self, rhs:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Calculates Euclidean division, the matching method for
rem_euclid
.
This computes the integer
n
such that
self = n * rhs + self.rem_euclid(rhs)
.
In other words, the result is
self / rhs
rounded to the integer
n
such that
self >= n * rhs
.
§
Precision
The result of this operation is guaranteed to be the rounded
infinite-precision result.
§
Examples
#![feature(f128)]
let
a: f128 =
7.0
;
let
b =
4.0
;
assert_eq!
(a.div_euclid(b),
1.0
);
// 7.0 > 4.0 * 1.0
assert_eq!
((-a).div_euclid(b), -
2.0
);
// -7.0 >= 4.0 * -2.0
assert_eq!
(a.div_euclid(-b), -
1.0
);
// 7.0 >= -4.0 * -1.0
assert_eq!
((-a).div_euclid(-b),
2.0
);
// -7.0 >= -4.0 * 2.0
Source
pub fn
rem_euclid
(self, rhs:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Calculates the least nonnegative remainder of
self (mod rhs)
.
In particular, the return value
r
satisfies
0.0 <= r < rhs.abs()
in
most cases. However, due to a floating point round-off error it can
result in
r == rhs.abs()
, violating the mathematical definition, if
self
is much smaller than
rhs.abs()
in magnitude and
self < 0.0
.
This result is not an element of the function’s codomain, but it is the
closest floating point number in the real numbers and thus fulfills the
property
self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)
approximately.
§
Precision
The result of this operation is guaranteed to be the rounded
infinite-precision result.
§
Examples
#![feature(f128)]
let
a: f128 =
7.0
;
let
b =
4.0
;
assert_eq!
(a.rem_euclid(b),
3.0
);
assert_eq!
((-a).rem_euclid(b),
1.0
);
assert_eq!
(a.rem_euclid(-b),
3.0
);
assert_eq!
((-a).rem_euclid(-b),
1.0
);
// limitation due to round-off error
assert!
((-f128::EPSILON).rem_euclid(
3.0
) !=
0.0
);
Source
pub fn
powi
(self, n:
i32
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Raises a number to an integer power.
Using this function is generally faster than using
powf
.
It might have a different sequence of rounding operations than
powf
,
so the results are not guaranteed to agree.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x =
2.0_f128
;
let
abs_difference = (x.powi(
2
) - (x * x)).abs();
assert!
(abs_difference <= f128::EPSILON);
assert_eq!
(f128::powi(f128::NAN,
0
),
1.0
);
Source
pub fn
powf
(self, n:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Raises a number to a floating point power.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x =
2.0_f128
;
let
abs_difference = (x.powf(
2.0
) - (x * x)).abs();
assert!
(abs_difference <= f128::EPSILON);
assert_eq!
(f128::powf(
1.0
, f128::NAN),
1.0
);
assert_eq!
(f128::powf(f128::NAN,
0.0
),
1.0
);
Source
pub fn
sqrt
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the square root of a number.
Returns NaN if
self
is a negative number other than
-0.0
.
§
Precision
The result of this operation is guaranteed to be the rounded
infinite-precision result. It is specified by IEEE 754 as
squareRoot
and guaranteed not to change.
§
Examples
#![feature(f128)]
let
positive =
4.0_f128
;
let
negative = -
4.0_f128
;
let
negative_zero = -
0.0_f128
;
assert_eq!
(positive.sqrt(),
2.0
);
assert!
(negative.sqrt().is_nan());
assert!
(negative_zero.sqrt() == negative_zero);
Source
pub fn
exp
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
e^(self)
, (the exponential function).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
one =
1.0f128
;
// e^1
let
e = one.exp();
// ln(e) - 1 == 0
let
abs_difference = (e.ln() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
exp2
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
2^(self)
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
f =
2.0f128
;
// 2^2 - 4 == 0
let
abs_difference = (f.exp2() -
4.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
ln
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the natural logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
one =
1.0f128
;
// e^1
let
e = one.exp();
// ln(e) - 1 == 0
let
abs_difference = (e.ln() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Non-positive values:
#![feature(f128)]
assert_eq!
(
0_f128
.ln(), f128::NEG_INFINITY);
assert!
((-
42_f128
).ln().is_nan());
Source
pub fn
log
(self, base:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the logarithm of the number with respect to an arbitrary base.
This returns NaN when the number is negative, and negative infinity when number is zero.
The result might not be correctly rounded owing to implementation details;
self.log2()
can produce more accurate results for base 2, and
self.log10()
can produce more accurate results for base 10.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
five =
5.0f128
;
// log5(5) - 1 == 0
let
abs_difference = (five.log(
5.0
) -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Non-positive values:
#![feature(f128)]
assert_eq!
(
0_f128
.log(
10.0
), f128::NEG_INFINITY);
assert!
((-
42_f128
).log(
10.0
).is_nan());
Source
pub fn
log2
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the base 2 logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
two =
2.0f128
;
// log2(2) - 1 == 0
let
abs_difference = (two.log2() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Non-positive values:
#![feature(f128)]
assert_eq!
(
0_f128
.log2(), f128::NEG_INFINITY);
assert!
((-
42_f128
).log2().is_nan());
Source
pub fn
log10
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the base 10 logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
ten =
10.0f128
;
// log10(10) - 1 == 0
let
abs_difference = (ten.log10() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Non-positive values:
#![feature(f128)]
assert_eq!
(
0_f128
.log10(), f128::NEG_INFINITY);
assert!
((-
42_f128
).log10().is_nan());
Source
pub fn
cbrt
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the cube root of a number.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
cbrtf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x =
8.0f128
;
// x^(1/3) - 2 == 0
let
abs_difference = (x.cbrt() -
2.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
hypot
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Compute the distance between the origin and a point (
x
,
y
) on the
Euclidean plane. Equivalently, compute the length of the hypotenuse of a
right-angle triangle with other sides having length
x.abs()
and
y.abs()
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
hypotf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x =
2.0f128
;
let
y =
3.0f128
;
// sqrt(x^2 + y^2)
let
abs_difference = (x.hypot(y) - (x.powi(
2
) + y.powi(
2
)).sqrt()).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
sin
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the sine of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x = std::f128::consts::FRAC_PI_2;
let
abs_difference = (x.sin() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
cos
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the cosine of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x =
2.0
* std::f128::consts::PI;
let
abs_difference = (x.cos() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
tan
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the tangent of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tanf128
from libc on Unix and
Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x = std::f128::consts::FRAC_PI_4;
let
abs_difference = (x.tan() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
asin
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the arcsine of a number. Return value is in radians in
the range [-pi/2, pi/2] or NaN if the number is outside the range
[-1, 1].
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
asinf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
f = std::f128::consts::FRAC_PI_2;
// asin(sin(pi/2))
let
abs_difference = (f.sin().asin() - std::f128::consts::FRAC_PI_2).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
acos
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the arccosine of a number. Return value is in radians in
the range [0, pi] or NaN if the number is outside the range
[-1, 1].
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
acosf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
f = std::f128::consts::FRAC_PI_4;
// acos(cos(pi/4))
let
abs_difference = (f.cos().acos() - std::f128::consts::FRAC_PI_4).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
atan
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the arctangent of a number. Return value is in radians in the
range [-pi/2, pi/2];
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
atanf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
f =
1.0f128
;
// atan(tan(1))
let
abs_difference = (f.tan().atan() -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
atan2
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the four quadrant arctangent of
self
(
y
) and
other
(
x
) in radians.
x = 0
,
y = 0
:
0
x >= 0
:
arctan(y/x)
->
[-pi/2, pi/2]
y >= 0
:
arctan(y/x) + pi
->
(pi/2, pi]
y < 0
:
arctan(y/x) - pi
->
(-pi, -pi/2)
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
atan2f128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
// Positive angles measured counter-clockwise
// from positive x axis
// -pi/4 radians (45 deg clockwise)
let
x1 =
3.0f128
;
let
y1 = -
3.0f128
;
// 3pi/4 radians (135 deg counter-clockwise)
let
x2 = -
3.0f128
;
let
y2 =
3.0f128
;
let
abs_difference_1 = (y1.atan2(x1) - (-std::f128::consts::FRAC_PI_4)).abs();
let
abs_difference_2 = (y2.atan2(x2) - (
3.0
* std::f128::consts::FRAC_PI_4)).abs();
assert!
(abs_difference_1 <= f128::EPSILON);
assert!
(abs_difference_2 <= f128::EPSILON);
Source
pub fn
sin_cos
(self) -> (
f128
,
f128
)
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Simultaneously computes the sine and cosine of the number,
x
. Returns
(sin(x), cos(x))
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
(f128::sin(x), f128::cos(x))
. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x = std::f128::consts::FRAC_PI_4;
let
f = x.sin_cos();
let
abs_difference_0 = (f.
0
- x.sin()).abs();
let
abs_difference_1 = (f.
1
- x.cos()).abs();
assert!
(abs_difference_0 <= f128::EPSILON);
assert!
(abs_difference_1 <= f128::EPSILON);
Source
pub fn
exp_m1
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
e^(self) - 1
in a way that is accurate even if the
number is close to zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
expm1f128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x =
1e-8_f128
;
// for very small x, e^x is approximately 1 + x + x^2 / 2
let
approx = x + x * x /
2.0
;
let
abs_difference = (x.exp_m1() - approx).abs();
assert!
(abs_difference <
1e-10
);
Source
pub fn
ln_1p
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
ln(1+n)
(natural logarithm) more accurately than if
the operations were performed separately.
This returns NaN when
n < -1.0
, and negative infinity when
n == -1.0
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
log1pf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
x =
1e-8_f128
;
// for very small x, ln(1 + x) is approximately x - x^2 / 2
let
approx = x - x * x /
2.0
;
let
abs_difference = (x.ln_1p() - approx).abs();
assert!
(abs_difference <
1e-10
);
Out-of-range values:
#![feature(f128)]
assert_eq!
((-
1.0_f128
).ln_1p(), f128::NEG_INFINITY);
assert!
((-
2.0_f128
).ln_1p().is_nan());
Source
pub fn
sinh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Hyperbolic sine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
sinhf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
e = std::f128::consts::E;
let
x =
1.0f128
;
let
f = x.sinh();
// Solving sinh() at 1 gives `(e^2-1)/(2e)`
let
g = ((e * e) -
1.0
) / (
2.0
* e);
let
abs_difference = (f - g).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
cosh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Hyperbolic cosine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
coshf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
e = std::f128::consts::E;
let
x =
1.0f128
;
let
f = x.cosh();
// Solving cosh() at 1 gives this result
let
g = ((e * e) +
1.0
) / (
2.0
* e);
let
abs_difference = (f - g).abs();
// Same result
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
tanh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Hyperbolic tangent function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tanhf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
let
e = std::f128::consts::E;
let
x =
1.0f128
;
let
f = x.tanh();
// Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
let
g = (
1.0
- e.powi(-
2
)) / (
1.0
+ e.powi(-
2
));
let
abs_difference = (f - g).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
asinh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Inverse hyperbolic sine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x =
1.0f128
;
let
f = x.sinh().asinh();
let
abs_difference = (f - x).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
acosh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Inverse hyperbolic cosine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
x =
1.0f128
;
let
f = x.cosh().acosh();
let
abs_difference = (f - x).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
atanh
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Inverse hyperbolic tangent function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
§
Examples
#![feature(f128)]
let
e = std::f128::consts::E;
let
f = e.tanh().atanh();
let
abs_difference = (f - e).abs();
assert!
(abs_difference <=
1e-5
);
Source
pub fn
gamma
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Gamma function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tgammaf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
#![feature(float_gamma)]
let
x =
5.0f128
;
let
abs_difference = (x.gamma() -
24.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
ln_gamma
(self) -> (
f128
,
i32
)
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Natural logarithm of the absolute value of the gamma function
The integer part of the tuple indicates the sign of the gamma function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
lgammaf128_r
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
#![feature(float_gamma)]
let
x =
2.0f128
;
let
abs_difference = (x.ln_gamma().
0
-
0.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub fn
erf
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Error function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
erff128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
#![feature(float_erf)]
/// The error function relates what percent of a normal distribution lies
/// within `x` standard deviations (scaled by `1/sqrt(2)`).
fn
within_standard_deviations(x: f128) -> f128 {
    (x * std::f128::consts::FRAC_1_SQRT_2).erf() *
100.0
}
// 68% of a normal distribution is within one standard deviation
assert!
((within_standard_deviations(
1.0
) -
68.269
).abs() <
0.01
);
// 95% of a normal distribution is within two standard deviations
assert!
((within_standard_deviations(
2.0
) -
95.450
).abs() <
0.01
);
// 99.7% of a normal distribution is within three standard deviations
assert!
((within_standard_deviations(
3.0
) -
99.730
).abs() <
0.01
);
Source
pub fn
erfc
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Complementary error function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
erfcf128
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(f128)]
#![feature(float_erf)]
let
x: f128 =
0.123
;
let
one = x.erf() + x.erfc();
let
abs_difference = (one -
1.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
§
impl
f128
Source
pub const
RADIX
:
u32
= 2u32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
The radix or base of the internal representation of
f128
.
Source
pub const
MANTISSA_DIGITS
:
u32
= 113u32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Number of significant digits in base 2.
Source
pub const
DIGITS
:
u32
= 33u32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Approximate number of significant digits in base 10.
This is the maximum
x
such that any decimal number with
x
significant digits can be converted to
f128
and back without loss.
Equal to floor(log
10
2
MANTISSA_DIGITS
− 1
).
Source
pub const
EPSILON
:
f128
= 1.92592994438723585305597794258492732E-34f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Machine epsilon
value for
f128
.
This is the difference between
1.0
and the next larger representable number.
Equal to 2
1 −
MANTISSA_DIGITS
.
Source
pub const
MIN
:
f128
= -1.18973149535723176508575932662800702E+4932f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Smallest finite
f128
value.
Equal to −
MAX
.
Source
pub const
MIN_POSITIVE
:
f128
= 3.3621031431120935062626778173217526E-4932f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Smallest positive normal
f128
value.
Equal to 2
MIN_EXP
− 1
.
Source
pub const
MAX
:
f128
= 1.18973149535723176508575932662800702E+4932f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Largest finite
f128
value.
Equal to
(1 − 2
−
MANTISSA_DIGITS
) 2
MAX_EXP
.
Source
pub const
MIN_EXP
:
i32
= -16_381i32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
One greater than the minimum possible normal power of 2 exponent.
If
x
=
MIN_EXP
, then normal numbers
≥ 0.5 × 2
x
.
Source
pub const
MAX_EXP
:
i32
= 16_384i32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Maximum possible power of 2 exponent.
If
x
=
MAX_EXP
, then normal numbers
< 1 × 2
x
.
Source
pub const
MIN_10_EXP
:
i32
= -4_931i32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Minimum
x
for which 10
x
is normal.
Equal to ceil(log
10
MIN_POSITIVE
).
Source
pub const
MAX_10_EXP
:
i32
= 4_932i32
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Maximum
x
for which 10
x
is normal.
Equal to floor(log
10
MAX
).
Source
pub const
NAN
:
f128
= NaN_f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Not a Number (NaN).
Note that IEEE 754 doesn’t define just a single NaN value;
a plethora of bit patterns are considered to be NaN.
Furthermore, the standard makes a difference
between a “signaling” and a “quiet” NaN,
and allows inspecting its “payload” (the unspecified bits in the bit pattern).
This constant isn’t guaranteed to equal to any specific NaN bitpattern,
and the stability of its representation over Rust versions
and target platforms isn’t guaranteed.
Source
pub const
INFINITY
:
f128
= +Inf_f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Infinity (∞).
Source
pub const
NEG_INFINITY
:
f128
= -Inf_f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Negative infinity (−∞).
Source
pub const fn
is_nan
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if this value is NaN.
#![feature(f128)]
let
nan = f128::NAN;
let
f =
7.0_f128
;
assert!
(nan.is_nan());
assert!
(!f.is_nan());
Source
pub const fn
is_infinite
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if this value is positive infinity or negative infinity, and
false
otherwise.
#![feature(f128)]
let
f =
7.0f128
;
let
inf = f128::INFINITY;
let
neg_inf = f128::NEG_INFINITY;
let
nan = f128::NAN;
assert!
(!f.is_infinite());
assert!
(!nan.is_infinite());
assert!
(inf.is_infinite());
assert!
(neg_inf.is_infinite());
Source
pub const fn
is_finite
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if this number is neither infinite nor NaN.
#![feature(f128)]
let
f =
7.0f128
;
let
inf: f128 = f128::INFINITY;
let
neg_inf: f128 = f128::NEG_INFINITY;
let
nan: f128 = f128::NAN;
assert!
(f.is_finite());
assert!
(!nan.is_finite());
assert!
(!inf.is_finite());
assert!
(!neg_inf.is_finite());
Source
pub const fn
is_subnormal
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if the number is
subnormal
.
#![feature(f128)]
let
min = f128::MIN_POSITIVE;
// 3.362103143e-4932f128
let
max = f128::MAX;
let
lower_than_min =
1.0e-4960_f128
;
let
zero =
0.0_f128
;
assert!
(!min.is_subnormal());
assert!
(!max.is_subnormal());
assert!
(!zero.is_subnormal());
assert!
(!f128::NAN.is_subnormal());
assert!
(!f128::INFINITY.is_subnormal());
// Values between `0` and `min` are Subnormal.
assert!
(lower_than_min.is_subnormal());
Source
pub const fn
is_normal
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if the number is neither zero, infinite,
subnormal
, or NaN.
#![feature(f128)]
let
min = f128::MIN_POSITIVE;
// 3.362103143e-4932f128
let
max = f128::MAX;
let
lower_than_min =
1.0e-4960_f128
;
let
zero =
0.0_f128
;
assert!
(min.is_normal());
assert!
(max.is_normal());
assert!
(!zero.is_normal());
assert!
(!f128::NAN.is_normal());
assert!
(!f128::INFINITY.is_normal());
// Values between `0` and `min` are Subnormal.
assert!
(!lower_than_min.is_normal());
Source
pub const fn
classify
(self) ->
FpCategory
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the floating point category of the number. If only one property
is going to be tested, it is generally faster to use the specific
predicate instead.
#![feature(f128)]
use
std::num::FpCategory;
let
num =
12.4_f128
;
let
inf = f128::INFINITY;
assert_eq!
(num.classify(), FpCategory::Normal);
assert_eq!
(inf.classify(), FpCategory::Infinite);
Source
pub const fn
is_sign_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if
self
has a positive sign, including
+0.0
, NaNs with
positive sign bit and positive infinity.
Note that IEEE 754 doesn’t assign any meaning to the sign bit in case of
a NaN, and as Rust doesn’t guarantee that the bit pattern of NaNs are
conserved over arithmetic operations, the result of
is_sign_positive
on
a NaN might produce an unexpected or non-portable result. See the
specification
of NaN bit patterns
for more info. Use
self.signum() == 1.0
if you need fully portable behavior (will return
false
for all NaNs).
#![feature(f128)]
let
f =
7.0_f128
;
let
g = -
7.0_f128
;
assert!
(f.is_sign_positive());
assert!
(!g.is_sign_positive());
Source
pub const fn
is_sign_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns
true
if
self
has a negative sign, including
-0.0
, NaNs with
negative sign bit and negative infinity.
Note that IEEE 754 doesn’t assign any meaning to the sign bit in case of
a NaN, and as Rust doesn’t guarantee that the bit pattern of NaNs are
conserved over arithmetic operations, the result of
is_sign_negative
on
a NaN might produce an unexpected or non-portable result. See the
specification
of NaN bit patterns
for more info. Use
self.signum() == -1.0
if you need fully portable behavior (will return
false
for all NaNs).
#![feature(f128)]
let
f =
7.0_f128
;
let
g = -
7.0_f128
;
assert!
(!f.is_sign_negative());
assert!
(g.is_sign_negative());
Source
pub const fn
next_up
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the least number greater than
self
.
Let
TINY
be the smallest representable positive
f128
. Then,
if
self.is_nan()
, this returns
self
;
if
self
is
NEG_INFINITY
, this returns
MIN
;
if
self
is
-TINY
, this returns -0.0;
if
self
is -0.0 or +0.0, this returns
TINY
;
if
self
is
MAX
or
INFINITY
, this returns
INFINITY
;
otherwise the unique least value greater than
self
is returned.
The identity
x.next_up() == -(-x).next_down()
holds for all non-NaN
x
. When
x
is finite
x == x.next_up().next_down()
also holds.
#![feature(f128)]
// f128::EPSILON is the difference between 1.0 and the next number up.
assert_eq!
(
1.0f128
.next_up(),
1.0
+ f128::EPSILON);
// But not for most numbers.
assert!
(
0.1f128
.next_up() <
0.1
+ f128::EPSILON);
assert_eq!
(
4611686018427387904f128
.next_up(),
4611686018427387904.000000000000001
);
This operation corresponds to IEEE-754
nextUp
.
Source
pub const fn
next_down
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the greatest number less than
self
.
Let
TINY
be the smallest representable positive
f128
. Then,
if
self.is_nan()
, this returns
self
;
if
self
is
INFINITY
, this returns
MAX
;
if
self
is
TINY
, this returns 0.0;
if
self
is -0.0 or +0.0, this returns
-TINY
;
if
self
is
MIN
or
NEG_INFINITY
, this returns
NEG_INFINITY
;
otherwise the unique greatest value less than
self
is returned.
The identity
x.next_down() == -(-x).next_up()
holds for all non-NaN
x
. When
x
is finite
x == x.next_down().next_up()
also holds.
#![feature(f128)]
let
x =
1.0f128
;
// Clamp value into range [0, 1).
let
clamped = x.clamp(
0.0
,
1.0f128
.next_down());
assert!
(clamped <
1.0
);
assert_eq!
(clamped.next_up(),
1.0
);
This operation corresponds to IEEE-754
nextDown
.
Source
pub const fn
recip
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Takes the reciprocal (inverse) of a number,
1/x
.
#![feature(f128)]
let
x =
2.0_f128
;
let
abs_difference = (x.recip() - (
1.0
/ x)).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub const fn
to_degrees
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Converts radians to degrees.
#![feature(f128)]
let
angle = std::f128::consts::PI;
let
abs_difference = (angle.to_degrees() -
180.0
).abs();
assert!
(abs_difference <= f128::EPSILON);
Source
pub const fn
to_radians
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Converts degrees to radians.
#![feature(f128)]
let
angle =
180.0f128
;
let
abs_difference = (angle.to_radians() - std::f128::consts::PI).abs();
assert!
(abs_difference <=
1e-30
);
Source
pub const fn
max
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the maximum of the two numbers, ignoring NaN.
If one of the arguments is NaN, then the other argument is returned.
This follows the IEEE 754-2008 semantics for maxNum, except for handling of signaling NaNs;
this function handles all NaNs the same way and avoids maxNum’s problems with associativity.
This also matches the behavior of libm’s fmax. In particular, if the inputs compare equal
(such as for the case of
+0.0
and
-0.0
), either input may be returned non-deterministically.
#![feature(f128)]
let
x =
1.0f128
;
let
y =
2.0f128
;
assert_eq!
(x.max(y), y);
Source
pub const fn
min
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the minimum of the two numbers, ignoring NaN.
If one of the arguments is NaN, then the other argument is returned.
This follows the IEEE 754-2008 semantics for minNum, except for handling of signaling NaNs;
this function handles all NaNs the same way and avoids minNum’s problems with associativity.
This also matches the behavior of libm’s fmin. In particular, if the inputs compare equal
(such as for the case of
+0.0
and
-0.0
), either input may be returned non-deterministically.
#![feature(f128)]
let
x =
1.0f128
;
let
y =
2.0f128
;
assert_eq!
(x.min(y), x);
Source
pub const fn
maximum
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the maximum of the two numbers, propagating NaN.
This returns NaN when
either
argument is NaN, as opposed to
f128::max
which only returns NaN when
both
arguments are NaN.
#![feature(f128)]
#![feature(float_minimum_maximum)]
let
x =
1.0f128
;
let
y =
2.0f128
;
assert_eq!
(x.maximum(y), y);
assert!
(x.maximum(f128::NAN).is_nan());
If one of the arguments is NaN, then NaN is returned. Otherwise this returns the greater
of the two numbers. For this operation, -0.0 is considered to be less than +0.0.
Note that this follows the semantics specified in IEEE 754-2019.
Also note that “propagation” of NaNs here doesn’t necessarily mean that the bitpattern of a NaN
operand is conserved; see the
specification of NaN bit patterns
for more info.
Source
pub const fn
minimum
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the minimum of the two numbers, propagating NaN.
This returns NaN when
either
argument is NaN, as opposed to
f128::min
which only returns NaN when
both
arguments are NaN.
#![feature(f128)]
#![feature(float_minimum_maximum)]
let
x =
1.0f128
;
let
y =
2.0f128
;
assert_eq!
(x.minimum(y), x);
assert!
(x.minimum(f128::NAN).is_nan());
If one of the arguments is NaN, then NaN is returned. Otherwise this returns the lesser
of the two numbers. For this operation, -0.0 is considered to be less than +0.0.
Note that this follows the semantics specified in IEEE 754-2019.
Also note that “propagation” of NaNs here doesn’t necessarily mean that the bitpattern of a NaN
operand is conserved; see the
specification of NaN bit patterns
for more info.
Source
pub const fn
midpoint
(self, other:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Calculates the middle point of
self
and
rhs
.
This returns NaN when
either
argument is NaN or if a combination of
+inf and -inf is provided as arguments.
§
Examples
#![feature(f128)]
assert_eq!
(
1f128
.midpoint(
4.0
),
2.5
);
assert_eq!
((-
5.5f128
).midpoint(
8.0
),
1.25
);
Source
pub unsafe fn
to_int_unchecked
<Int>(self) -> Int
where
f128
:
FloatToInt
<Int>,
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Rounds toward zero and converts to any primitive integer type,
assuming that the value is finite and fits in that type.
#![feature(f128)]
let
value =
4.6_f128
;
let
rounded =
unsafe
{ value.to_int_unchecked::<u16>() };
assert_eq!
(rounded,
4
);
let
value = -
128.9_f128
;
let
rounded =
unsafe
{ value.to_int_unchecked::<i8>() };
assert_eq!
(rounded, i8::MIN);
§
Safety
The value must:
Not be
NaN
Not be infinite
Be representable in the return type
Int
, after truncating off its fractional part
Source
pub const fn
to_bits
(self) ->
u128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Raw transmutation to
u128
.
This is currently identical to
transmute::<f128, u128>(self)
on all platforms.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
Note that this function is distinct from
as
casting, which attempts to
preserve the
numeric
value, and not the bitwise value.
#![feature(f128)]
assert_eq!
((
12.5f128
).to_bits(),
0x40029000000000000000000000000000
);
Source
pub const fn
from_bits
(v:
u128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Raw transmutation from
u128
.
This is currently identical to
transmute::<u128, f128>(v)
on all platforms.
It turns out this is incredibly portable, for two reasons:
Floats and Ints have the same endianness on all supported platforms.
IEEE 754 very precisely specifies the bit layout of floats.
However there is one caveat: prior to the 2008 version of IEEE 754, how
to interpret the NaN signaling bit wasn’t actually specified. Most platforms
(notably x86 and ARM) picked the interpretation that was ultimately
standardized in 2008, but some didn’t (notably MIPS). As a result, all
signaling NaNs on MIPS are quiet NaNs on x86, and vice-versa.
Rather than trying to preserve signaling-ness cross-platform, this
implementation favors preserving the exact bits. This means that
any payloads encoded in NaNs will be preserved even if the result of
this method is sent over the network from an x86 machine to a MIPS one.
If the results of this method are only manipulated by the same
architecture that produced them, then there is no portability concern.
If the input isn’t NaN, then there is no portability concern.
If you don’t care about signalingness (very likely), then there is no
portability concern.
Note that this function is distinct from
as
casting, which attempts to
preserve the
numeric
value, and not the bitwise value.
#![feature(f128)]
let
v = f128::from_bits(
0x40029000000000000000000000000000
);
assert_eq!
(v,
12.5
);
Source
pub const fn
to_be_bytes
(self) -> [
u8
;
16
]
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the memory representation of this floating point number as a byte array in
big-endian (network) byte order.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
bytes =
12.5f128
.to_be_bytes();
assert_eq!
(
    bytes,
    [
0x40
,
0x02
,
0x90
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
]
);
Source
pub const fn
to_le_bytes
(self) -> [
u8
;
16
]
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the memory representation of this floating point number as a byte array in
little-endian byte order.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
bytes =
12.5f128
.to_le_bytes();
assert_eq!
(
    bytes,
    [
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x90
,
0x02
,
0x40
]
);
Source
pub const fn
to_ne_bytes
(self) -> [
u8
;
16
]
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the memory representation of this floating point number as a byte array in
native byte order.
As the target platform’s native endianness is used, portable code
should use
to_be_bytes
or
to_le_bytes
, as appropriate, instead.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
bytes =
12.5f128
.to_ne_bytes();
assert_eq!
(
    bytes,
if
cfg!
(target_endian =
"big"
) {
        [
0x40
,
0x02
,
0x90
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
]
    }
else
{
        [
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x90
,
0x02
,
0x40
]
    }
);
Source
pub const fn
from_be_bytes
(bytes: [
u8
;
16
]) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Creates a floating point value from its representation as a byte array in big endian.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
value = f128::from_be_bytes(
    [
0x40
,
0x02
,
0x90
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
]
);
assert_eq!
(value,
12.5
);
Source
pub const fn
from_le_bytes
(bytes: [
u8
;
16
]) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Creates a floating point value from its representation as a byte array in little endian.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
value = f128::from_le_bytes(
    [
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x90
,
0x02
,
0x40
]
);
assert_eq!
(value,
12.5
);
Source
pub const fn
from_ne_bytes
(bytes: [
u8
;
16
]) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Creates a floating point value from its representation as a byte array in native endian.
As the target platform’s native endianness is used, portable code
likely wants to use
from_be_bytes
or
from_le_bytes
, as
appropriate instead.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
#![feature(f128)]
let
value = f128::from_ne_bytes(
if
cfg!
(target_endian =
"big"
) {
    [
0x40
,
0x02
,
0x90
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
]
}
else
{
    [
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x00
,
0x90
,
0x02
,
0x40
]
});
assert_eq!
(value,
12.5
);
Source
pub fn
total_cmp
(&self, other: &
f128
) ->
Ordering
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns the ordering between
self
and
other
.
Unlike the standard partial comparison between floating point numbers,
this comparison always produces an ordering in accordance to
the
totalOrder
predicate as defined in the IEEE 754 (2008 revision)
floating point standard. The values are ordered in the following sequence:
negative quiet NaN
negative signaling NaN
negative infinity
negative numbers
negative subnormal numbers
negative zero
positive zero
positive subnormal numbers
positive numbers
positive infinity
positive signaling NaN
positive quiet NaN.
The ordering established by this function does not always agree with the
PartialOrd
and
PartialEq
implementations of
f128
. For example,
they consider negative and positive zero equal, while
total_cmp
doesn’t.
The interpretation of the signaling NaN bit follows the definition in
the IEEE 754 standard, which may not match the interpretation by some of
the older, non-conformant (e.g. MIPS) hardware implementations.
§
Example
#![feature(f128)]
struct
GoodBoy {
    name:
&
'static
str,
    weight: f128,
}
let
mut
bois =
vec!
[
    GoodBoy { name:
"Pucci"
, weight:
0.1
},
    GoodBoy { name:
"Woofer"
, weight:
99.0
},
    GoodBoy { name:
"Yapper"
, weight:
10.0
},
    GoodBoy { name:
"Chonk"
, weight: f128::INFINITY },
    GoodBoy { name:
"Abs. Unit"
, weight: f128::NAN },
    GoodBoy { name:
"Floaty"
, weight: -
5.0
},
];

bois.sort_by(|a, b| a.weight.total_cmp(
&
b.weight));
// `f128::NAN` could be positive or negative, which will affect the sort order.
if
f128::NAN.is_sign_negative() {
    bois.into_iter().map(|b| b.weight)
        .zip([f128::NAN, -
5.0
,
0.1
,
10.0
,
99.0
, f128::INFINITY].iter())
        .for_each(|(a, b)|
assert_eq!
(a.to_bits(), b.to_bits()))
}
else
{
    bois.into_iter().map(|b| b.weight)
        .zip([-
5.0
,
0.1
,
10.0
,
99.0
, f128::INFINITY, f128::NAN].iter())
        .for_each(|(a, b)|
assert_eq!
(a.to_bits(), b.to_bits()))
}
Source
pub const fn
clamp
(self, min:
f128
, max:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Restrict a value to a certain interval unless it is NaN.
Returns
max
if
self
is greater than
max
, and
min
if
self
is
less than
min
. Otherwise this returns
self
.
Note that this function returns NaN if the initial value was NaN as
well.
§
Panics
Panics if
min > max
,
min
is NaN, or
max
is NaN.
§
Examples
#![feature(f128)]
assert!
((-
3.0f128
).clamp(-
2.0
,
1.0
) == -
2.0
);
assert!
((
0.0f128
).clamp(-
2.0
,
1.0
) ==
0.0
);
assert!
((
2.0f128
).clamp(-
2.0
,
1.0
) ==
1.0
);
assert!
((f128::NAN).clamp(-
2.0
,
1.0
).is_nan());
Source
pub const fn
abs
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Computes the absolute value of
self
.
This function always returns the precise result.
§
Examples
#![feature(f128)]
let
x =
3.5_f128
;
let
y = -
3.5_f128
;
assert_eq!
(x.abs(), x);
assert_eq!
(y.abs(), -y);
assert!
(f128::NAN.abs().is_nan());
Source
pub const fn
signum
(self) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns a number that represents the sign of
self
.
1.0
if the number is positive,
+0.0
or
INFINITY
-1.0
if the number is negative,
-0.0
or
NEG_INFINITY
NaN if the number is NaN
§
Examples
#![feature(f128)]
let
f =
3.5_f128
;
assert_eq!
(f.signum(),
1.0
);
assert_eq!
(f128::NEG_INFINITY.signum(), -
1.0
);
assert!
(f128::NAN.signum().is_nan());
Source
pub const fn
copysign
(self, sign:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
f128
#116909
)
Returns a number composed of the magnitude of
self
and the sign of
sign
.
Equal to
self
if the sign of
self
and
sign
are the same, otherwise equal to
-self
.
If
self
is a NaN, then a NaN with the same payload as
self
and the sign bit of
sign
is
returned.
If
sign
is a NaN, then this operation will still carry over its sign into the result. Note
that IEEE 754 doesn’t assign any meaning to the sign bit in case of a NaN, and as Rust
doesn’t guarantee that the bit pattern of NaNs are conserved over arithmetic operations, the
result of
copysign
with
sign
being a NaN might produce an unexpected or non-portable
result. See the
specification of NaN bit patterns
for more
info.
§
Examples
#![feature(f128)]
let
f =
3.5_f128
;
assert_eq!
(f.copysign(
0.42
),
3.5_f128
);
assert_eq!
(f.copysign(-
0.42
), -
3.5_f128
);
assert_eq!
((-f).copysign(
0.42
),
3.5_f128
);
assert_eq!
((-f).copysign(-
0.42
), -
3.5_f128
);
assert!
(f128::NAN.copysign(
1.0
).is_nan());
Trait Implementations
§
1.0.0
·
Source
§
impl
Add
<&
f128
> for &
f128
Source
§
type
Output
= <
f128
as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
f128
) -> <
f128
as
Add
>::
Output
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
<&
f128
> for
f128
Source
§
type
Output
= <
f128
as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
f128
) -> <
f128
as
Add
>::
Output
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Add
<
f128
> for &'a
f128
Source
§
type
Output
= <
f128
as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
f128
) -> <
f128
as
Add
>::
Output
Performs the
+
operation.
Read more
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
f128
Source
§
type
Output
=
f128
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
f128
) ->
f128
Performs the
+
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
f128
> for
f128
Source
§
fn
add_assign
(&mut self, other: &
f128
)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
f128
Source
§
fn
add_assign
(&mut self, other:
f128
)
Performs the
+=
operation.
Read more
1.0.0
·
Source
§
impl
Clone
for
f128
Source
§
fn
clone
(&self) ->
f128
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl
Debug
for
f128
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl
Default
for
f128
Source
§
fn
default
() ->
f128
Returns the default value of
0.0
1.0.0
·
Source
§
impl
Div
<&
f128
> for &
f128
Source
§
type
Output
= <
f128
as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
f128
) -> <
f128
as
Div
>::
Output
Performs the
/
operation.
Read more
1.0.0
·
Source
§
impl
Div
<&
f128
> for
f128
Source
§
type
Output
= <
f128
as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
f128
) -> <
f128
as
Div
>::
Output
Performs the
/
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Div
<
f128
> for &'a
f128
Source
§
type
Output
= <
f128
as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
f128
) -> <
f128
as
Div
>::
Output
Performs the
/
operation.
Read more
1.0.0
·
Source
§
impl
Div
for
f128
Source
§
type
Output
=
f128
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
f128
) ->
f128
Performs the
/
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
f128
> for
f128
Source
§
fn
div_assign
(&mut self, other: &
f128
)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
f128
Source
§
fn
div_assign
(&mut self, other:
f128
)
Performs the
/=
operation.
Read more
1.68.0
·
Source
§
impl
From
<
bool
> for
f128
Source
§
fn
from
(small:
bool
) ->
f128
Converts a
bool
to
f128
losslessly.
The resulting value is positive
0.0
for
false
and
1.0
for
true
values.
§
Examples
#![feature(f128)]
let
x: f128 =
false
.into();
assert_eq!
(x,
0.0
);
assert!
(x.is_sign_positive());
let
y: f128 =
true
.into();
assert_eq!
(y,
1.0
);
1.6.0
·
Source
§
impl
From
<
f16
> for
f128
Source
§
fn
from
(small:
f16
) ->
f128
Converts
f16
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
f32
> for
f128
Source
§
fn
from
(small:
f32
) ->
f128
Converts
f32
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
f64
> for
f128
Source
§
fn
from
(small:
f64
) ->
f128
Converts
f64
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
i16
> for
f128
Source
§
fn
from
(small:
i16
) ->
f128
Converts
i16
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
i32
> for
f128
Source
§
fn
from
(small:
i32
) ->
f128
Converts
i32
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
i8
> for
f128
Source
§
fn
from
(small:
i8
) ->
f128
Converts
i8
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
u16
> for
f128
Source
§
fn
from
(small:
u16
) ->
f128
Converts
u16
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
u32
> for
f128
Source
§
fn
from
(small:
u32
) ->
f128
Converts
u32
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
u8
> for
f128
Source
§
fn
from
(small:
u8
) ->
f128
Converts
u8
to
f128
losslessly.
1.0.0
·
Source
§
impl
Mul
<&
f128
> for &
f128
Source
§
type
Output
= <
f128
as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
f128
) -> <
f128
as
Mul
>::
Output
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
<&
f128
> for
f128
Source
§
type
Output
= <
f128
as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
f128
) -> <
f128
as
Mul
>::
Output
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Mul
<
f128
> for &'a
f128
Source
§
type
Output
= <
f128
as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
f128
) -> <
f128
as
Mul
>::
Output
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
f128
Source
§
type
Output
=
f128
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
f128
) ->
f128
Performs the
*
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
f128
> for
f128
Source
§
fn
mul_assign
(&mut self, other: &
f128
)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
f128
Source
§
fn
mul_assign
(&mut self, other:
f128
)
Performs the
*=
operation.
Read more
1.0.0
·
Source
§
impl
Neg
for &
f128
Source
§
type
Output
= <
f128
as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
f128
as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.0.0
·
Source
§
impl
Neg
for
f128
Source
§
type
Output
=
f128
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
f128
Performs the unary
-
operation.
Read more
1.0.0
·
Source
§
impl
PartialEq
for
f128
Source
§
fn
eq
(&self, other: &
f128
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
f128
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl
PartialOrd
for
f128
Source
§
fn
partial_cmp
(&self, other: &
f128
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
Source
§
fn
lt
(&self, other: &
f128
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
Source
§
fn
le
(&self, other: &
f128
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
Source
§
fn
gt
(&self, other: &
f128
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
Source
§
fn
ge
(&self, other: &
f128
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.0.0
·
Source
§
impl
Rem
<&
f128
> for &
f128
Source
§
type
Output
= <
f128
as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
f128
) -> <
f128
as
Rem
>::
Output
Performs the
%
operation.
Read more
1.0.0
·
Source
§
impl
Rem
<&
f128
> for
f128
Source
§
type
Output
= <
f128
as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
f128
) -> <
f128
as
Rem
>::
Output
Performs the
%
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Rem
<
f128
> for &'a
f128
Source
§
type
Output
= <
f128
as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
f128
) -> <
f128
as
Rem
>::
Output
Performs the
%
operation.
Read more
1.0.0
·
Source
§
impl
Rem
for
f128
The remainder from the division of two floats.
The remainder has the same sign as the dividend and is computed as:
x - (x / y).trunc() * y
.
§
Examples
let
x: f32 =
50.50
;
let
y: f32 =
8.125
;
let
remainder = x - (x / y).trunc() * y;
// The answer to both operations is 1.75
assert_eq!
(x % y, remainder);
Source
§
type
Output
=
f128
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
f128
) ->
f128
Performs the
%
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
f128
> for
f128
Source
§
fn
rem_assign
(&mut self, other: &
f128
)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
f128
Source
§
fn
rem_assign
(&mut self, other:
f128
)
Performs the
%=
operation.
Read more
1.0.0
·
Source
§
impl
Sub
<&
f128
> for &
f128
Source
§
type
Output
= <
f128
as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
f128
) -> <
f128
as
Sub
>::
Output
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
<&
f128
> for
f128
Source
§
type
Output
= <
f128
as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
f128
) -> <
f128
as
Sub
>::
Output
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Sub
<
f128
> for &'a
f128
Source
§
type
Output
= <
f128
as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
f128
) -> <
f128
as
Sub
>::
Output
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
f128
Source
§
type
Output
=
f128
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
f128
) ->
f128
Performs the
-
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
f128
> for
f128
Source
§
fn
sub_assign
(&mut self, other: &
f128
)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
f128
Source
§
fn
sub_assign
(&mut self, other:
f128
)
Performs the
-=
operation.
Read more
1.0.0
·
Source
§
impl
Copy
for
f128
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
f128
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
i8
> for
f128
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
u128
> for
f128
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
f128
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
u8
> for
f128
Source
§
impl
FloatToInt
<
usize
> for
f128
Source
§
impl
UseCloned
for
f128
Auto Trait Implementations
§
§
impl
Freeze
for
f128
§
impl
RefUnwindSafe
for
f128
§
impl
Send
for
f128
§
impl
Sync
for
f128
§
impl
Unpin
for
f128
§
impl
UnwindSafe
for
f128
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.