f32 - Rust
Primitive Type
f32
Copy item path
1.0.0
Expand description
A 32-bit floating-point type (specifically, the “binary32” type defined in IEEE 754-2008).
This type can represent a wide range of decimal numbers, like
3.5
,
27
,
-113.75
,
0.0078125
,
34359738368
,
0
,
-1
. So unlike integer types
(such as
i32
), floating-point types can represent non-integer numbers,
too.
However, being able to represent this wide range of numbers comes at the
cost of precision: floats can only represent some of the real numbers and
calculation with floats round to a nearby representable number. For example,
5.0
and
1.0
can be exactly represented as
f32
, but
1.0 / 5.0
results
in
0.20000000298023223876953125
since
0.2
cannot be exactly represented
as
f32
. Note, however, that printing floats with
println
and friends will
often discard insignificant digits:
println!("{}", 1.0f32 / 5.0f32)
will
print
0.2
.
Additionally,
f32
can represent some special values:
−0.0: IEEE 754 floating-point numbers have a bit that indicates their sign, so −0.0 is a
possible value. For comparison −0.0 = +0.0, but floating-point operations can carry
the sign bit through arithmetic operations. This means −0.0 × +0.0 produces −0.0 and
a negative number rounded to a value smaller than a float can represent also produces −0.0.
∞
and
−∞
: these result from calculations
like
1.0 / 0.0
.
NaN (not a number)
: this value results from
calculations like
(-1.0).sqrt()
. NaN has some potentially unexpected
behavior:
It is not equal to any float, including itself! This is the reason
f32
doesn’t implement the
Eq
trait.
It is also neither smaller nor greater than any float, making it
impossible to sort by the default comparison operation, which is the
reason
f32
doesn’t implement the
Ord
trait.
It is also considered
infectious
as almost all calculations where one
of the operands is NaN will also result in NaN. The explanations on this
page only explicitly document behavior on NaN operands if this default
is deviated from.
Lastly, there are multiple bit patterns that are considered NaN.
Rust does not currently guarantee that the bit patterns of NaN are
preserved over arithmetic operations, and they are not guaranteed to be
portable or even fully deterministic! This means that there may be some
surprising results upon inspecting the bit patterns,
as the same calculations might produce NaNs with different bit patterns.
This also affects the sign of the NaN: checking
is_sign_positive
or
is_sign_negative
on
a NaN is the most common way to run into these surprising results.
(Checking
x >= 0.0
or
x <= 0.0
avoids those surprises, but also how negative/positive
zero are treated.)
See the section below for what exactly is guaranteed about the bit pattern of a NaN.
When a primitive operation (addition, subtraction, multiplication, or
division) is performed on this type, the result is rounded according to the
roundTiesToEven direction defined in IEEE 754-2008. That means:
The result is the representable value closest to the true value, if there
is a unique closest representable value.
If the true value is exactly half-way between two representable values,
the result is the one with an even least-significant binary digit.
If the true value’s magnitude is ≥
f32::MAX
+ 2
(
f32::MAX_EXP
−
f32::MANTISSA_DIGITS
− 1)
, the result is ∞ or −∞ (preserving the
true value’s sign).
If the result of a sum exactly equals zero, the outcome is +0.0 unless
both arguments were negative, then it is -0.0. Subtraction
a - b
is
regarded as a sum
a + (-b)
.
For more information on floating-point numbers, see
Wikipedia
.
See also the
std::f32::consts
module
.
§
NaN bit patterns
This section defines the possible NaN bit patterns returned by floating-point operations.
The bit pattern of a floating-point NaN value is defined by:
a sign bit.
a quiet/signaling bit. Rust assumes that the quiet/signaling bit being set to
1
indicates a
quiet NaN (QNaN), and a value of
0
indicates a signaling NaN (SNaN). In the following we
will hence just call it the “quiet bit”.
a payload, which makes up the rest of the significand (i.e., the mantissa) except for the
quiet bit.
The rules for NaN values differ between
arithmetic
and
non-arithmetic
(or “bitwise”)
operations. The non-arithmetic operations are unary
-
,
abs
,
copysign
,
signum
,
{to,from}_bits
,
{to,from}_{be,le,ne}_bytes
and
is_sign_{positive,negative}
. These
operations are guaranteed to exactly preserve the bit pattern of their input except for possibly
changing the sign bit.
The following rules apply when a NaN value is returned from an arithmetic operation:
The result has a non-deterministic sign.
The quiet bit and payload are non-deterministically chosen from
the following set of options:
Preferred NaN
: The quiet bit is set and the payload is all-zero.
Quieting NaN propagation
: The quiet bit is set and the payload is copied from any input
operand that is a NaN. If the inputs and outputs do not have the same payload size (i.e., for
as
casts), then
If the output is smaller than the input, low-order bits of the payload get dropped.
If the output is larger than the input, the payload gets filled up with 0s in the low-order
bits.
Unchanged NaN propagation
: The quiet bit and payload are copied from any input operand
that is a NaN. If the inputs and outputs do not have the same size (i.e., for
as
casts), the
same rules as for “quieting NaN propagation” apply, with one caveat: if the output is smaller
than the input, dropping the low-order bits may result in a payload of 0; a payload of 0 is not
possible with a signaling NaN (the all-0 significand encodes an infinity) so unchanged NaN
propagation cannot occur with some inputs.
Target-specific NaN
: The quiet bit is set and the payload is picked from a target-specific
set of “extra” possible NaN payloads. The set can depend on the input operand values.
See the table below for the concrete NaNs this set contains on various targets.
In particular, if all input NaNs are quiet (or if there are no input NaNs), then the output NaN
is definitely quiet. Signaling NaN outputs can only occur if they are provided as an input
value. Similarly, if all input NaNs are preferred (or if there are no input NaNs) and the target
does not have any “extra” NaN payloads, then the output NaN is guaranteed to be preferred.
The non-deterministic choice happens when the operation is executed; i.e., the result of a
NaN-producing floating-point operation is a stable bit pattern (looking at these bits multiple
times will yield consistent results), but running the same operation twice with the same inputs
can produce different results.
These guarantees are neither stronger nor weaker than those of IEEE 754: IEEE 754 guarantees
that an operation never returns a signaling NaN, whereas it is possible for operations like
SNAN * 1.0
to return a signaling NaN in Rust. Conversely, IEEE 754 makes no statement at all
about which quiet NaN is returned, whereas Rust restricts the set of possible results to the
ones listed above.
Unless noted otherwise, the same rules also apply to NaNs returned by other library functions
(e.g.
min
,
minimum
,
max
,
maximum
); other aspects of their semantics and which IEEE 754
operation they correspond to are documented with the respective functions.
When an arithmetic floating-point operation is executed in
const
context, the same rules
apply: no guarantee is made about which of the NaN bit patterns described above will be
returned. The result does not have to match what happens when executing the same code at
runtime, and the result can vary depending on factors such as compiler version and flags.
§
Target-specific “extra” NaN values
target_arch
Extra payloads possible on this platform
x86
,
x86_64
,
arm
,
aarch64
,
riscv32
,
riscv64
None
sparc
,
sparc64
The all-one payload
wasm32
,
wasm64
If all input NaNs are quiet with all-zero payload: None.
Otherwise: all possible payloads.
For targets not in this table, all payloads are possible.
Implementations
§
Source
§
impl
f32
1.0.0
·
Source
pub fn
floor
(self) ->
f32
Returns the largest integer less than or equal to
self
.
This function always returns the precise result.
§
Examples
let
f =
3.7_f32
;
let
g =
3.0_f32
;
let
h = -
3.7_f32
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
1.0.0
·
Source
pub fn
ceil
(self) ->
f32
Returns the smallest integer greater than or equal to
self
.
This function always returns the precise result.
§
Examples
let
f =
3.01_f32
;
let
g =
4.0_f32
;
assert_eq!
(f.ceil(),
4.0
);
assert_eq!
(g.ceil(),
4.0
);
1.0.0
·
Source
pub fn
round
(self) ->
f32
Returns the nearest integer to
self
. If a value is half-way between two
integers, round away from
0.0
.
This function always returns the precise result.
§
Examples
let
f =
3.3_f32
;
let
g = -
3.3_f32
;
let
h = -
3.7_f32
;
let
i =
3.5_f32
;
let
j =
4.5_f32
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
1.77.0
·
Source
pub fn
round_ties_even
(self) ->
f32
Returns the nearest integer to a number. Rounds half-way cases to the number
with an even least significant digit.
This function always returns the precise result.
§
Examples
let
f =
3.3_f32
;
let
g = -
3.3_f32
;
let
h =
3.5_f32
;
let
i =
4.5_f32
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
1.0.0
·
Source
pub fn
trunc
(self) ->
f32
Returns the integer part of
self
.
This means that non-integer numbers are always truncated towards zero.
This function always returns the precise result.
§
Examples
let
f =
3.7_f32
;
let
g =
3.0_f32
;
let
h = -
3.7_f32
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
1.0.0
·
Source
pub fn
fract
(self) ->
f32
Returns the fractional part of
self
.
This function always returns the precise result.
§
Examples
let
x =
3.6_f32
;
let
y = -
3.6_f32
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
(abs_difference_x <= f32::EPSILON);
assert!
(abs_difference_y <= f32::EPSILON);
1.0.0
·
Source
pub fn
mul_add
(self, a:
f32
, b:
f32
) ->
f32
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
let
m =
10.0_f32
;
let
x =
4.0_f32
;
let
b =
60.0_f32
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
1.0_f32
+ f32::EPSILON;
let
one_minus_eps =
1.0_f32
- f32::EPSILON;
let
minus_one = -
1.0_f32
;
// The exact result (1 + eps) * (1 - eps) = 1 - eps * eps.
assert_eq!
(one_plus_eps.mul_add(one_minus_eps, minus_one), -f32::EPSILON * f32::EPSILON);
// Different rounding with the non-fused multiply and add.
assert_eq!
(one_plus_eps * one_minus_eps + minus_one,
0.0
);
1.38.0
·
Source
pub fn
div_euclid
(self, rhs:
f32
) ->
f32
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
let
a: f32 =
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
1.38.0
·
Source
pub fn
rem_euclid
(self, rhs:
f32
) ->
f32
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
let
a: f32 =
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
((-f32::EPSILON).rem_euclid(
3.0
) !=
0.0
);
1.0.0
·
Source
pub fn
powi
(self, n:
i32
) ->
f32
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
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x =
2.0_f32
;
let
abs_difference = (x.powi(
2
) - (x * x)).abs();
assert!
(abs_difference <= f32::EPSILON);
assert_eq!
(f32::powi(f32::NAN,
0
),
1.0
);
1.0.0
·
Source
pub fn
powf
(self, n:
f32
) ->
f32
Raises a number to a floating point power.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x =
2.0_f32
;
let
abs_difference = (x.powf(
2.0
) - (x * x)).abs();
assert!
(abs_difference <= f32::EPSILON);
assert_eq!
(f32::powf(
1.0
, f32::NAN),
1.0
);
assert_eq!
(f32::powf(f32::NAN,
0.0
),
1.0
);
1.0.0
·
Source
pub fn
sqrt
(self) ->
f32
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
let
positive =
4.0_f32
;
let
negative = -
4.0_f32
;
let
negative_zero = -
0.0_f32
;
assert_eq!
(positive.sqrt(),
2.0
);
assert!
(negative.sqrt().is_nan());
assert!
(negative_zero.sqrt() == negative_zero);
1.0.0
·
Source
pub fn
exp
(self) ->
f32
Returns
e^(self)
, (the exponential function).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
one =
1.0f32
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
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
exp2
(self) ->
f32
Returns
2^(self)
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
f =
2.0f32
;
// 2^2 - 4 == 0
let
abs_difference = (f.exp2() -
4.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
ln
(self) ->
f32
Returns the natural logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
one =
1.0f32
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
(abs_difference <= f32::EPSILON);
Non-positive values:
assert_eq!
(
0_f32
.ln(), f32::NEG_INFINITY);
assert!
((-
42_f32
).ln().is_nan());
1.0.0
·
Source
pub fn
log
(self, base:
f32
) ->
f32
Returns the logarithm of the number with respect to an arbitrary base.
This returns NaN when the number is negative, and negative infinity when number is zero.
The result might not be correctly rounded owing to implementation details;
self.log2()
can produce more accurate results for base 2, and
self.log10()
can produce more accurate results for base 10.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
five =
5.0f32
;
// log5(5) - 1 == 0
let
abs_difference = (five.log(
5.0
) -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Non-positive values:
assert_eq!
(
0_f32
.log(
10.0
), f32::NEG_INFINITY);
assert!
((-
42_f32
).log(
10.0
).is_nan());
1.0.0
·
Source
pub fn
log2
(self) ->
f32
Returns the base 2 logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
two =
2.0f32
;
// log2(2) - 1 == 0
let
abs_difference = (two.log2() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Non-positive values:
assert_eq!
(
0_f32
.log2(), f32::NEG_INFINITY);
assert!
((-
42_f32
).log2().is_nan());
1.0.0
·
Source
pub fn
log10
(self) ->
f32
Returns the base 10 logarithm of the number.
This returns NaN when the number is negative, and negative infinity when number is zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
ten =
10.0f32
;
// log10(10) - 1 == 0
let
abs_difference = (ten.log10() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Non-positive values:
assert_eq!
(
0_f32
.log10(), f32::NEG_INFINITY);
assert!
((-
42_f32
).log10().is_nan());
1.0.0
·
Source
pub fn
abs_sub
(self, other:
f32
) ->
f32
👎
Deprecated since 1.10.0: you probably meant
(self - other).abs()
: this operation is
(self - other).max(0.0)
except that
abs_sub
also propagates NaNs (also known as
fdimf
in C). If you truly need the positive difference, consider using that expression or the C function
fdimf
, depending on how you wish to handle NaN (please consider filing an issue describing your use-case too).
The positive difference of two numbers.
If
self <= other
:
0.0
Else:
self - other
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
fdimf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
x =
3.0f32
;
let
y = -
3.0f32
;
let
abs_difference_x = (x.abs_sub(
1.0
) -
2.0
).abs();
let
abs_difference_y = (y.abs_sub(
1.0
) -
0.0
).abs();
assert!
(abs_difference_x <= f32::EPSILON);
assert!
(abs_difference_y <= f32::EPSILON);
1.0.0
·
Source
pub fn
cbrt
(self) ->
f32
Returns the cube root of a number.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
cbrtf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
x =
8.0f32
;
// x^(1/3) - 2 == 0
let
abs_difference = (x.cbrt() -
2.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
hypot
(self, other:
f32
) ->
f32
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
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
hypotf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
x =
2.0f32
;
let
y =
3.0f32
;
// sqrt(x^2 + y^2)
let
abs_difference = (x.hypot(y) - (x.powi(
2
) + y.powi(
2
)).sqrt()).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
sin
(self) ->
f32
Computes the sine of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x = std::f32::consts::FRAC_PI_2;
let
abs_difference = (x.sin() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
cos
(self) ->
f32
Computes the cosine of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x =
2.0
* std::f32::consts::PI;
let
abs_difference = (x.cos() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
tan
(self) ->
f32
Computes the tangent of a number (in radians).
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tanf
from libc on Unix and
Windows. Note that this might change in the future.
§
Examples
let
x = std::f32::consts::FRAC_PI_4;
let
abs_difference = (x.tan() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
asin
(self) ->
f32
Computes the arcsine of a number. Return value is in radians in
the range [-pi/2, pi/2] or NaN if the number is outside the range
[-1, 1].
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
asinf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
f = std::f32::consts::FRAC_PI_2;
// asin(sin(pi/2))
let
abs_difference = (f.sin().asin() - std::f32::consts::FRAC_PI_2).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
acos
(self) ->
f32
Computes the arccosine of a number. Return value is in radians in
the range [0, pi] or NaN if the number is outside the range
[-1, 1].
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
acosf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
f = std::f32::consts::FRAC_PI_4;
// acos(cos(pi/4))
let
abs_difference = (f.cos().acos() - std::f32::consts::FRAC_PI_4).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
atan
(self) ->
f32
Computes the arctangent of a number. Return value is in radians in the
range [-pi/2, pi/2];
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
atanf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
f =
1.0f32
;
// atan(tan(1))
let
abs_difference = (f.tan().atan() -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
atan2
(self, other:
f32
) ->
f32
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
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
atan2f
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
// Positive angles measured counter-clockwise
// from positive x axis
// -pi/4 radians (45 deg clockwise)
let
x1 =
3.0f32
;
let
y1 = -
3.0f32
;
// 3pi/4 radians (135 deg counter-clockwise)
let
x2 = -
3.0f32
;
let
y2 =
3.0f32
;
let
abs_difference_1 = (y1.atan2(x1) - (-std::f32::consts::FRAC_PI_4)).abs();
let
abs_difference_2 = (y2.atan2(x2) - (
3.0
* std::f32::consts::FRAC_PI_4)).abs();
assert!
(abs_difference_1 <= f32::EPSILON);
assert!
(abs_difference_2 <= f32::EPSILON);
1.0.0
·
Source
pub fn
sin_cos
(self) -> (
f32
,
f32
)
Simultaneously computes the sine and cosine of the number,
x
. Returns
(sin(x), cos(x))
.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
(f32::sin(x), f32::cos(x))
. Note that this might change in the future.
§
Examples
let
x = std::f32::consts::FRAC_PI_4;
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
(abs_difference_0 <= f32::EPSILON);
assert!
(abs_difference_1 <= f32::EPSILON);
1.0.0
·
Source
pub fn
exp_m1
(self) ->
f32
Returns
e^(self) - 1
in a way that is accurate even if the
number is close to zero.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
expm1f
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
x =
1e-8_f32
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
1.0.0
·
Source
pub fn
ln_1p
(self) ->
f32
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
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
log1pf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
x =
1e-8_f32
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
assert_eq!
((-
1.0_f32
).ln_1p(), f32::NEG_INFINITY);
assert!
((-
2.0_f32
).ln_1p().is_nan());
1.0.0
·
Source
pub fn
sinh
(self) ->
f32
Hyperbolic sine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
sinhf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
e = std::f32::consts::E;
let
x =
1.0f32
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
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
cosh
(self) ->
f32
Hyperbolic cosine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
coshf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
e = std::f32::consts::E;
let
x =
1.0f32
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
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
tanh
(self) ->
f32
Hyperbolic tangent function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tanhf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
let
e = std::f32::consts::E;
let
x =
1.0f32
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
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
asinh
(self) ->
f32
Inverse hyperbolic sine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x =
1.0f32
;
let
f = x.sinh().asinh();
let
abs_difference = (f - x).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
acosh
(self) ->
f32
Inverse hyperbolic cosine function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
x =
1.0f32
;
let
f = x.cosh().acosh();
let
abs_difference = (f - x).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0
·
Source
pub fn
atanh
(self) ->
f32
Inverse hyperbolic tangent function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
§
Examples
let
e = std::f32::consts::E;
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
f32
🔬
This is a nightly-only experimental API. (
float_gamma
#99842
)
Gamma function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
tgammaf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(float_gamma)]
let
x =
5.0f32
;
let
abs_difference = (x.gamma() -
24.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Source
pub fn
ln_gamma
(self) -> (
f32
,
i32
)
🔬
This is a nightly-only experimental API. (
float_gamma
#99842
)
Natural logarithm of the absolute value of the gamma function
The integer part of the tuple indicates the sign of the gamma function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform, Rust version, and
can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
lgamma_r
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(float_gamma)]
let
x =
2.0f32
;
let
abs_difference = (x.ln_gamma().
0
-
0.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Source
pub fn
erf
(self) ->
f32
🔬
This is a nightly-only experimental API. (
float_erf
#136321
)
Error function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
erff
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(float_erf)]
/// The error function relates what percent of a normal distribution lies
/// within `x` standard deviations (scaled by `1/sqrt(2)`).
fn
within_standard_deviations(x: f32) -> f32 {
    (x * std::f32::consts::FRAC_1_SQRT_2).erf() *
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
f32
🔬
This is a nightly-only experimental API. (
float_erf
#136321
)
Complementary error function.
§
Unspecified precision
The precision of this function is non-deterministic. This means it varies by platform,
Rust version, and can even differ within the same execution from one invocation to the next.
This function currently corresponds to the
erfcf
from libc on Unix
and Windows. Note that this might change in the future.
§
Examples
#![feature(float_erf)]
let
x: f32 =
0.123
;
let
one = x.erf() + x.erfc();
let
abs_difference = (one -
1.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
Source
§
impl
f32
1.43.0
·
Source
pub const
RADIX
:
u32
= 2u32
The radix or base of the internal representation of
f32
.
1.43.0
·
Source
pub const
MANTISSA_DIGITS
:
u32
= 24u32
Number of significant digits in base 2.
1.43.0
·
Source
pub const
DIGITS
:
u32
= 6u32
Approximate number of significant digits in base 10.
This is the maximum
x
such that any decimal number with
x
significant digits can be converted to
f32
and back without loss.
Equal to floor(log
10
2
MANTISSA_DIGITS
− 1
).
1.43.0
·
Source
pub const
EPSILON
:
f32
= 1.1920929E-7f32
Machine epsilon
value for
f32
.
This is the difference between
1.0
and the next larger representable number.
Equal to 2
1 −
MANTISSA_DIGITS
.
1.43.0
·
Source
pub const
MIN
:
f32
= -3.40282347E+38f32
Smallest finite
f32
value.
Equal to −
MAX
.
1.43.0
·
Source
pub const
MIN_POSITIVE
:
f32
= 1.17549435E-38f32
Smallest positive normal
f32
value.
Equal to 2
MIN_EXP
− 1
.
1.43.0
·
Source
pub const
MAX
:
f32
= 3.40282347E+38f32
Largest finite
f32
value.
Equal to
(1 − 2
−
MANTISSA_DIGITS
) 2
MAX_EXP
.
1.43.0
·
Source
pub const
MIN_EXP
:
i32
= -125i32
One greater than the minimum possible normal power of 2 exponent.
If
x
=
MIN_EXP
, then normal numbers
≥ 0.5 × 2
x
.
1.43.0
·
Source
pub const
MAX_EXP
:
i32
= 128i32
Maximum possible power of 2 exponent.
If
x
=
MAX_EXP
, then normal numbers
< 1 × 2
x
.
1.43.0
·
Source
pub const
MIN_10_EXP
:
i32
= -37i32
Minimum
x
for which 10
x
is normal.
Equal to ceil(log
10
MIN_POSITIVE
).
1.43.0
·
Source
pub const
MAX_10_EXP
:
i32
= 38i32
Maximum
x
for which 10
x
is normal.
Equal to floor(log
10
MAX
).
1.43.0
·
Source
pub const
NAN
:
f32
= NaN_f32
Not a Number (NaN).
Note that IEEE 754 doesn’t define just a single NaN value;
a plethora of bit patterns are considered to be NaN.
Furthermore, the standard makes a difference
between a “signaling” and a “quiet” NaN,
and allows inspecting its “payload” (the unspecified bits in the bit pattern).
This constant isn’t guaranteed to equal to any specific NaN bitpattern,
and the stability of its representation over Rust versions
and target platforms isn’t guaranteed.
1.43.0
·
Source
pub const
INFINITY
:
f32
= +Inf_f32
Infinity (∞).
1.43.0
·
Source
pub const
NEG_INFINITY
:
f32
= -Inf_f32
Negative infinity (−∞).
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_nan
(self) ->
bool
Returns
true
if this value is NaN.
let
nan = f32::NAN;
let
f =
7.0_f32
;
assert!
(nan.is_nan());
assert!
(!f.is_nan());
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_infinite
(self) ->
bool
Returns
true
if this value is positive infinity or negative infinity, and
false
otherwise.
let
f =
7.0f32
;
let
inf = f32::INFINITY;
let
neg_inf = f32::NEG_INFINITY;
let
nan = f32::NAN;
assert!
(!f.is_infinite());
assert!
(!nan.is_infinite());
assert!
(inf.is_infinite());
assert!
(neg_inf.is_infinite());
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_finite
(self) ->
bool
Returns
true
if this number is neither infinite nor NaN.
let
f =
7.0f32
;
let
inf = f32::INFINITY;
let
neg_inf = f32::NEG_INFINITY;
let
nan = f32::NAN;
assert!
(f.is_finite());
assert!
(!nan.is_finite());
assert!
(!inf.is_finite());
assert!
(!neg_inf.is_finite());
1.53.0 (const: 1.83.0)
·
Source
pub const fn
is_subnormal
(self) ->
bool
Returns
true
if the number is
subnormal
.
let
min = f32::MIN_POSITIVE;
// 1.17549435e-38f32
let
max = f32::MAX;
let
lower_than_min =
1.0e-40_f32
;
let
zero =
0.0_f32
;
assert!
(!min.is_subnormal());
assert!
(!max.is_subnormal());
assert!
(!zero.is_subnormal());
assert!
(!f32::NAN.is_subnormal());
assert!
(!f32::INFINITY.is_subnormal());
// Values between `0` and `min` are Subnormal.
assert!
(lower_than_min.is_subnormal());
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_normal
(self) ->
bool
Returns
true
if the number is neither zero, infinite,
subnormal
, or NaN.
let
min = f32::MIN_POSITIVE;
// 1.17549435e-38f32
let
max = f32::MAX;
let
lower_than_min =
1.0e-40_f32
;
let
zero =
0.0_f32
;
assert!
(min.is_normal());
assert!
(max.is_normal());
assert!
(!zero.is_normal());
assert!
(!f32::NAN.is_normal());
assert!
(!f32::INFINITY.is_normal());
// Values between `0` and `min` are Subnormal.
assert!
(!lower_than_min.is_normal());
1.0.0 (const: 1.83.0)
·
Source
pub const fn
classify
(self) ->
FpCategory
Returns the floating point category of the number. If only one property
is going to be tested, it is generally faster to use the specific
predicate instead.
use
std::num::FpCategory;
let
num =
12.4_f32
;
let
inf = f32::INFINITY;
assert_eq!
(num.classify(), FpCategory::Normal);
assert_eq!
(inf.classify(), FpCategory::Infinite);
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_sign_positive
(self) ->
bool
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
let
f =
7.0_f32
;
let
g = -
7.0_f32
;
assert!
(f.is_sign_positive());
assert!
(!g.is_sign_positive());
1.0.0 (const: 1.83.0)
·
Source
pub const fn
is_sign_negative
(self) ->
bool
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
let
f =
7.0f32
;
let
g = -
7.0f32
;
assert!
(!f.is_sign_negative());
assert!
(g.is_sign_negative());
1.86.0 (const: 1.86.0)
·
Source
pub const fn
next_up
(self) ->
f32
Returns the least number greater than
self
.
Let
TINY
be the smallest representable positive
f32
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
// f32::EPSILON is the difference between 1.0 and the next number up.
assert_eq!
(
1.0f32
.next_up(),
1.0
+ f32::EPSILON);
// But not for most numbers.
assert!
(
0.1f32
.next_up() <
0.1
+ f32::EPSILON);
assert_eq!
(
16777216f32
.next_up(),
16777218.0
);
This operation corresponds to IEEE-754
nextUp
.
1.86.0 (const: 1.86.0)
·
Source
pub const fn
next_down
(self) ->
f32
Returns the greatest number less than
self
.
Let
TINY
be the smallest representable positive
f32
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
let
x =
1.0f32
;
// Clamp value into range [0, 1).
let
clamped = x.clamp(
0.0
,
1.0f32
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
1.0.0 (const: 1.85.0)
·
Source
pub const fn
recip
(self) ->
f32
Takes the reciprocal (inverse) of a number,
1/x
.
let
x =
2.0_f32
;
let
abs_difference = (x.recip() - (
1.0
/ x)).abs();
assert!
(abs_difference <= f32::EPSILON);
1.7.0 (const: 1.85.0)
·
Source
pub const fn
to_degrees
(self) ->
f32
Converts radians to degrees.
let
angle = std::f32::consts::PI;
let
abs_difference = (angle.to_degrees() -
180.0
).abs();
assert!
(abs_difference <= f32::EPSILON);
1.7.0 (const: 1.85.0)
·
Source
pub const fn
to_radians
(self) ->
f32
Converts degrees to radians.
let
angle =
180.0f32
;
let
abs_difference = (angle.to_radians() - std::f32::consts::PI).abs();
assert!
(abs_difference <= f32::EPSILON);
1.0.0 (const: 1.85.0)
·
Source
pub const fn
max
(self, other:
f32
) ->
f32
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
let
x =
1.0f32
;
let
y =
2.0f32
;
assert_eq!
(x.max(y), y);
1.0.0 (const: 1.85.0)
·
Source
pub const fn
min
(self, other:
f32
) ->
f32
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
let
x =
1.0f32
;
let
y =
2.0f32
;
assert_eq!
(x.min(y), x);
Source
pub const fn
maximum
(self, other:
f32
) ->
f32
🔬
This is a nightly-only experimental API. (
float_minimum_maximum
#91079
)
Returns the maximum of the two numbers, propagating NaN.
This returns NaN when
either
argument is NaN, as opposed to
f32::max
which only returns NaN when
both
arguments are NaN.
#![feature(float_minimum_maximum)]
let
x =
1.0f32
;
let
y =
2.0f32
;
assert_eq!
(x.maximum(y), y);
assert!
(x.maximum(f32::NAN).is_nan());
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
f32
) ->
f32
🔬
This is a nightly-only experimental API. (
float_minimum_maximum
#91079
)
Returns the minimum of the two numbers, propagating NaN.
This returns NaN when
either
argument is NaN, as opposed to
f32::min
which only returns NaN when
both
arguments are NaN.
#![feature(float_minimum_maximum)]
let
x =
1.0f32
;
let
y =
2.0f32
;
assert_eq!
(x.minimum(y), x);
assert!
(x.minimum(f32::NAN).is_nan());
If one of the arguments is NaN, then NaN is returned. Otherwise this returns the lesser
of the two numbers. For this operation, -0.0 is considered to be less than +0.0.
Note that this follows the semantics specified in IEEE 754-2019.
Also note that “propagation” of NaNs here doesn’t necessarily mean that the bitpattern of a NaN
operand is conserved; see the
specification of NaN bit patterns
for more info.
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, other:
f32
) ->
f32
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
assert_eq!
(
1f32
.midpoint(
4.0
),
2.5
);
assert_eq!
((-
5.5f32
).midpoint(
8.0
),
1.25
);
1.44.0
·
Source
pub unsafe fn
to_int_unchecked
<Int>(self) -> Int
where
f32
:
FloatToInt
<Int>,
Rounds toward zero and converts to any primitive integer type,
assuming that the value is finite and fits in that type.
let
value =
4.6_f32
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
128.9_f32
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
1.20.0 (const: 1.83.0)
·
Source
pub const fn
to_bits
(self) ->
u32
Raw transmutation to
u32
.
This is currently identical to
transmute::<f32, u32>(self)
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
§
Examples
assert_ne!
((
1f32
).to_bits(),
1f32
as
u32);
// to_bits() is not casting!
assert_eq!
((
12.5f32
).to_bits(),
0x41480000
);
1.20.0 (const: 1.83.0)
·
Source
pub const fn
from_bits
(v:
u32
) ->
f32
Raw transmutation from
u32
.
This is currently identical to
transmute::<u32, f32>(v)
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
§
Examples
let
v = f32::from_bits(
0x41480000
);
assert_eq!
(v,
12.5
);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
to_be_bytes
(self) -> [
u8
;
4
]
Returns the memory representation of this floating point number as a byte array in
big-endian (network) byte order.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
let
bytes =
12.5f32
.to_be_bytes();
assert_eq!
(bytes, [
0x41
,
0x48
,
0x00
,
0x00
]);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
to_le_bytes
(self) -> [
u8
;
4
]
Returns the memory representation of this floating point number as a byte array in
little-endian byte order.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
let
bytes =
12.5f32
.to_le_bytes();
assert_eq!
(bytes, [
0x00
,
0x00
,
0x48
,
0x41
]);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
to_ne_bytes
(self) -> [
u8
;
4
]
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
let
bytes =
12.5f32
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
0x41
,
0x48
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
0x48
,
0x41
]
    }
);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
from_be_bytes
(bytes: [
u8
;
4
]) ->
f32
Creates a floating point value from its representation as a byte array in big endian.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
let
value = f32::from_be_bytes([
0x41
,
0x48
,
0x00
,
0x00
]);
assert_eq!
(value,
12.5
);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
from_le_bytes
(bytes: [
u8
;
4
]) ->
f32
Creates a floating point value from its representation as a byte array in little endian.
See
from_bits
for some discussion of the
portability of this operation (there are almost no issues).
§
Examples
let
value = f32::from_le_bytes([
0x00
,
0x00
,
0x48
,
0x41
]);
assert_eq!
(value,
12.5
);
1.40.0 (const: 1.83.0)
·
Source
pub const fn
from_ne_bytes
(bytes: [
u8
;
4
]) ->
f32
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
let
value = f32::from_ne_bytes(
if
cfg!
(target_endian =
"big"
) {
    [
0x41
,
0x48
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
0x48
,
0x41
]
});
assert_eq!
(value,
12.5
);
1.62.0
·
Source
pub fn
total_cmp
(&self, other: &
f32
) ->
Ordering
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
f32
. For example,
they consider negative and positive zero equal, while
total_cmp
doesn’t.
The interpretation of the signaling NaN bit follows the definition in
the IEEE 754 standard, which may not match the interpretation by some of
the older, non-conformant (e.g. MIPS) hardware implementations.
§
Example
struct
GoodBoy {
    name: String,
    weight: f32,
}
let
mut
bois =
vec!
[
    GoodBoy { name:
"Pucci"
.to_owned(), weight:
0.1
},
    GoodBoy { name:
"Woofer"
.to_owned(), weight:
99.0
},
    GoodBoy { name:
"Yapper"
.to_owned(), weight:
10.0
},
    GoodBoy { name:
"Chonk"
.to_owned(), weight: f32::INFINITY },
    GoodBoy { name:
"Abs. Unit"
.to_owned(), weight: f32::NAN },
    GoodBoy { name:
"Floaty"
.to_owned(), weight: -
5.0
},
];

bois.sort_by(|a, b| a.weight.total_cmp(
&
b.weight));
// `f32::NAN` could be positive or negative, which will affect the sort order.
if
f32::NAN.is_sign_negative() {
assert!
(bois.into_iter().map(|b| b.weight)
        .zip([f32::NAN, -
5.0
,
0.1
,
10.0
,
99.0
, f32::INFINITY].iter())
        .all(|(a, b)| a.to_bits() == b.to_bits()))
}
else
{
assert!
(bois.into_iter().map(|b| b.weight)
        .zip([-
5.0
,
0.1
,
10.0
,
99.0
, f32::INFINITY, f32::NAN].iter())
        .all(|(a, b)| a.to_bits() == b.to_bits()))
}
1.50.0 (const: 1.85.0)
·
Source
pub const fn
clamp
(self, min:
f32
, max:
f32
) ->
f32
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
assert!
((-
3.0f32
).clamp(-
2.0
,
1.0
) == -
2.0
);
assert!
((
0.0f32
).clamp(-
2.0
,
1.0
) ==
0.0
);
assert!
((
2.0f32
).clamp(-
2.0
,
1.0
) ==
1.0
);
assert!
((f32::NAN).clamp(-
2.0
,
1.0
).is_nan());
1.0.0 (const: 1.85.0)
·
Source
pub const fn
abs
(self) ->
f32
Computes the absolute value of
self
.
This function always returns the precise result.
§
Examples
let
x =
3.5_f32
;
let
y = -
3.5_f32
;
assert_eq!
(x.abs(), x);
assert_eq!
(y.abs(), -y);
assert!
(f32::NAN.abs().is_nan());
1.0.0 (const: 1.85.0)
·
Source
pub const fn
signum
(self) ->
f32
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
let
f =
3.5_f32
;
assert_eq!
(f.signum(),
1.0
);
assert_eq!
(f32::NEG_INFINITY.signum(), -
1.0
);
assert!
(f32::NAN.signum().is_nan());
1.35.0 (const: 1.85.0)
·
Source
pub const fn
copysign
(self, sign:
f32
) ->
f32
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
let
f =
3.5_f32
;
assert_eq!
(f.copysign(
0.42
),
3.5_f32
);
assert_eq!
(f.copysign(-
0.42
), -
3.5_f32
);
assert_eq!
((-f).copysign(
0.42
),
3.5_f32
);
assert_eq!
((-f).copysign(-
0.42
), -
3.5_f32
);
assert!
(f32::NAN.copysign(
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
f32
> for &
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for &'a
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
Source
§
type
Output
=
f32
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
f32
) ->
f32
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
f32
> for
f32
Source
§
fn
add_assign
(&mut self, other: &
f32
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
f32
Source
§
fn
add_assign
(&mut self, other:
f32
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
f32
Source
§
fn
clone
(&self) ->
f32
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
f32
Source
§
fn
fmt
(&self, fmt: &mut
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
f32
Source
§
fn
default
() ->
f32
Returns the default value of
0.0
1.0.0
·
Source
§
impl
Display
for
f32
Source
§
fn
fmt
(&self, fmt: &mut
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
Div
<&
f32
> for &
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for &'a
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
Source
§
type
Output
=
f32
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
f32
) ->
f32
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
f32
> for
f32
Source
§
fn
div_assign
(&mut self, other: &
f32
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
f32
Source
§
fn
div_assign
(&mut self, other:
f32
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
f32
Source
§
fn
from
(small:
bool
) ->
f32
Converts a
bool
to
f32
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
let
x: f32 =
false
.into();
assert_eq!
(x,
0.0
);
assert!
(x.is_sign_positive());
let
y: f32 =
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
f32
> for
f64
Source
§
fn
from
(small:
f32
) ->
f64
Converts
f32
to
f64
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
f32
Source
§
fn
from
(small:
i16
) ->
f32
Converts
i16
to
f32
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
f32
Source
§
fn
from
(small:
i8
) ->
f32
Converts
i8
to
f32
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
f32
Source
§
fn
from
(small:
u16
) ->
f32
Converts
u16
to
f32
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
f32
Source
§
fn
from
(small:
u8
) ->
f32
Converts
u8
to
f32
losslessly.
1.0.0
·
Source
§
impl
FromStr
for
f32
Source
§
fn
from_str
(src: &
str
) ->
Result
<
f32
,
ParseFloatError
>
Converts a string in base 10 to a float.
Accepts an optional decimal exponent.
This function accepts strings such as
‘3.14’
‘-3.14’
‘2.5E10’, or equivalently, ‘2.5e10’
‘2.5E-10’
‘5.’
‘.5’, or, equivalently, ‘0.5’
‘inf’, ‘-inf’, ‘+infinity’, ‘NaN’
Note that alphabetical characters are not case-sensitive.
Leading and trailing whitespace represent an error.
§
Grammar
All strings that adhere to the following
EBNF
grammar when
lowercased will result in an
Ok
being returned:
Float  ::= Sign? ( 'inf' | 'infinity' | 'nan' | Number )
Number ::= ( Digit+ |
             Digit+ '.' Digit* |
             Digit* '.' Digit+ ) Exp?
Exp    ::= 'e' Sign? Digit+
Sign   ::= [+-]
Digit  ::= [0-9]
§
Arguments
src - A string
§
Return value
Err(ParseFloatError)
if the string did not represent a valid
number. Otherwise,
Ok(n)
where
n
is the closest
representable floating-point number to the number represented
by
src
(following the same rules for rounding as for the
results of primitive operations).
Source
§
type
Err
=
ParseFloatError
The associated error which can be returned from parsing.
1.0.0
·
Source
§
impl
LowerExp
for
f32
Source
§
fn
fmt
(&self, fmt: &mut
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
Mul
<&
f32
> for &
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for &'a
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
Source
§
type
Output
=
f32
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
f32
) ->
f32
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
f32
> for
f32
Source
§
fn
mul_assign
(&mut self, other: &
f32
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
f32
Source
§
fn
mul_assign
(&mut self, other:
f32
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
f32
Source
§
type
Output
= <
f32
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
f32
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
f32
Source
§
type
Output
=
f32
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
f32
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
f32
Source
§
fn
eq
(&self, other: &
f32
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
f32
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
f32
Source
§
fn
partial_cmp
(&self, other: &
f32
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
f32
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
f32
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
f32
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
f32
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
1.12.0
·
Source
§
impl<'a>
Product
<&'a
f32
> for
f32
Source
§
fn
product
<I>(iter: I) ->
f32
where
    I:
Iterator
<Item = &'a
f32
>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.12.0
·
Source
§
impl
Product
for
f32
Source
§
fn
product
<I>(iter: I) ->
f32
where
    I:
Iterator
<Item =
f32
>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.0.0
·
Source
§
impl
Rem
<&
f32
> for &
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for &'a
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
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
f32
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
f32
) ->
f32
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
f32
> for
f32
Source
§
fn
rem_assign
(&mut self, other: &
f32
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
f32
Source
§
fn
rem_assign
(&mut self, other:
f32
)
Performs the
%=
operation.
Read more
Source
§
impl
SimdElement
for
f32
Source
§
type
Mask
=
i32
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask element type corresponding to this element type.
1.0.0
·
Source
§
impl
Sub
<&
f32
> for &
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
> for &'a
f32
Source
§
type
Output
= <
f32
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
f32
) -> <
f32
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
f32
Source
§
type
Output
=
f32
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
f32
) ->
f32
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
f32
> for
f32
Source
§
fn
sub_assign
(&mut self, other: &
f32
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
f32
Source
§
fn
sub_assign
(&mut self, other:
f32
)
Performs the
-=
operation.
Read more
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
f32
> for
f32
Source
§
fn
sum
<I>(iter: I) ->
f32
where
    I:
Iterator
<Item = &'a
f32
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.12.0
·
Source
§
impl
Sum
for
f32
Source
§
fn
sum
<I>(iter: I) ->
f32
where
    I:
Iterator
<Item =
f32
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.0.0
·
Source
§
impl
UpperExp
for
f32
Source
§
fn
fmt
(&self, fmt: &mut
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
Copy
for
f32
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
i16
> for
f32
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
i64
> for
f32
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
isize
> for
f32
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
u16
> for
f32
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
u64
> for
f32
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
usize
> for
f32
Source
§
impl
SimdCast
for
f32
Source
§
impl
UseCloned
for
f32
Auto Trait Implementations
§
§
impl
Freeze
for
f32
§
impl
RefUnwindSafe
for
f32
§
impl
Send
for
f32
§
impl
Sync
for
f32
§
impl
Unpin
for
f32
§
impl
UnwindSafe
for
f32
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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