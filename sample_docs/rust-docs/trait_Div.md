Div in std::ops - Rust
std
::
ops
Trait
Div
Copy item path
1.0.0
·
Source
pub trait Div<Rhs = Self> {
    type
Output
;

    // Required method
    fn
div
(self, rhs: Rhs) -> Self::
Output
;
}
Expand description
The division operator
/
.
Note that
Rhs
is
Self
by default, but this is not mandatory.
§
Examples
§
Div
idable rational numbers
use
std::ops::Div;
// By the fundamental theorem of arithmetic, rational numbers in lowest
// terms are unique. So, by keeping `Rational`s in reduced form, we can
// derive `Eq` and `PartialEq`.
#[derive(Debug, Eq, PartialEq)]
struct
Rational {
    numerator: usize,
    denominator: usize,
}
impl
Rational {
fn
new(numerator: usize, denominator: usize) ->
Self
{
if
denominator ==
0
{
panic!
(
"Zero is an invalid denominator!"
);
        }
// Reduce to lowest terms by dividing by the greatest common
        // divisor.
let
gcd = gcd(numerator, denominator);
Self
{
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}
impl
Div
for
Rational {
// The division of rational numbers is a closed operation.
type
Output =
Self
;
fn
div(
self
, rhs:
Self
) ->
Self
::Output {
if
rhs.numerator ==
0
{
panic!
(
"Cannot divide by zero-valued `Rational`!"
);
        }
let
numerator =
self
.numerator * rhs.denominator;
let
denominator =
self
.denominator * rhs.numerator;
Self
::new(numerator, denominator)
    }
}
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn
gcd(x: usize, y: usize) -> usize {
let
mut
x = x;
let
mut
y = y;
while
y !=
0
{
let
t = y;
        y = x % y;
        x = t;
    }
    x
}
assert_eq!
(Rational::new(
1
,
2
), Rational::new(
2
,
4
));
assert_eq!
(Rational::new(
1
,
2
) / Rational::new(
3
,
4
),
           Rational::new(
2
,
3
));
§
Dividing vectors by scalars as in linear algebra
use
std::ops::Div;
struct
Scalar { value: f32 }
#[derive(Debug, PartialEq)]
struct
Vector { value: Vec<f32> }
impl
Div<Scalar>
for
Vector {
type
Output =
Self
;
fn
div(
self
, rhs: Scalar) ->
Self
::Output {
Self
{ value:
self
.value.iter().map(|v| v / rhs.value).collect() }
    }
}
let
scalar = Scalar { value:
2f32
};
let
vector = Vector { value:
vec!
[
2f32
,
4f32
,
6f32
] };
assert_eq!
(vector / scalar, Vector { value:
vec!
[
1f32
,
2f32
,
3f32
] });
Required Associated Types
§
1.0.0
·
Source
type
Output
The resulting type after applying the
/
operator.
Required Methods
§
1.0.0
·
Source
fn
div
(self, rhs: Rhs) -> Self::
Output
Performs the
/
operation.
§
Example
assert_eq!
(
12
/
2
,
6
);
Implementors
§
1.0.0
·
Source
§
impl
Div
for
f16
Source
§
type
Output
=
f16
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
1.0.0
·
Source
§
impl
Div
for
f64
Source
§
type
Output
=
f64
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
1.0.0
·
Source
§
impl
Div
for
i8
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i8
1.0.0
·
Source
§
impl
Div
for
i16
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i16
1.0.0
·
Source
§
impl
Div
for
i32
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i32
1.0.0
·
Source
§
impl
Div
for
i64
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i64
1.0.0
·
Source
§
impl
Div
for
i128
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i128
1.0.0
·
Source
§
impl
Div
for
isize
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
isize
1.0.0
·
Source
§
impl
Div
for
u8
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u8
1.0.0
·
Source
§
impl
Div
for
u16
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u16
1.0.0
·
Source
§
impl
Div
for
u32
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u32
1.0.0
·
Source
§
impl
Div
for
u64
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u64
1.0.0
·
Source
§
impl
Div
for
u128
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u128
1.0.0
·
Source
§
impl
Div
for
usize
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
usize
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i8
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i8
), Saturating(
5i8
) / Saturating(
2
));
assert_eq!
(Saturating(i8::MAX), Saturating(i8::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i8::MIN), Saturating(i8::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i8
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
i8
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i16
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i16
), Saturating(
5i16
) / Saturating(
2
));
assert_eq!
(Saturating(i16::MAX), Saturating(i16::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i16::MIN), Saturating(i16::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i16
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
i16
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i32
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i32
), Saturating(
5i32
) / Saturating(
2
));
assert_eq!
(Saturating(i32::MAX), Saturating(i32::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i32::MIN), Saturating(i32::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i32
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
i32
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i64
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i64
), Saturating(
5i64
) / Saturating(
2
));
assert_eq!
(Saturating(i64::MAX), Saturating(i64::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i64::MIN), Saturating(i64::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i64
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
i64
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i128
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i128
), Saturating(
5i128
) / Saturating(
2
));
assert_eq!
(Saturating(i128::MAX), Saturating(i128::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i128::MIN), Saturating(i128::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i128
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
i128
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
isize
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2isize
), Saturating(
5isize
) / Saturating(
2
));
assert_eq!
(Saturating(isize::MAX), Saturating(isize::MAX) / Saturating(
1
));
assert_eq!
(Saturating(isize::MIN), Saturating(isize::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0isize
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
isize
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u8
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u8
), Saturating(
5u8
) / Saturating(
2
));
assert_eq!
(Saturating(u8::MAX), Saturating(u8::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u8::MIN), Saturating(u8::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u8
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
u8
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u16
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u16
), Saturating(
5u16
) / Saturating(
2
));
assert_eq!
(Saturating(u16::MAX), Saturating(u16::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u16::MIN), Saturating(u16::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u16
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
u16
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u32
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u32
), Saturating(
5u32
) / Saturating(
2
));
assert_eq!
(Saturating(u32::MAX), Saturating(u32::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u32::MIN), Saturating(u32::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u32
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
u32
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u64
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u64
), Saturating(
5u64
) / Saturating(
2
));
assert_eq!
(Saturating(u64::MAX), Saturating(u64::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u64::MIN), Saturating(u64::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u64
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
u64
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u128
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u128
), Saturating(
5u128
) / Saturating(
2
));
assert_eq!
(Saturating(u128::MAX), Saturating(u128::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u128::MIN), Saturating(u128::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u128
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
u128
>
1.74.0
·
Source
§
impl
Div
for
Saturating
<
usize
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2usize
), Saturating(
5usize
) / Saturating(
2
));
assert_eq!
(Saturating(usize::MAX), Saturating(usize::MAX) / Saturating(
1
));
assert_eq!
(Saturating(usize::MIN), Saturating(usize::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0usize
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
<
usize
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
1.0.0
·
Source
§
impl
Div
<&
f16
> for &
f16
Source
§
type
Output
= <
f16
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
f16
> for
f16
Source
§
type
Output
= <
f16
as
Div
>::
Output
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
1.0.0
·
Source
§
impl
Div
<&
f64
> for &
f64
Source
§
type
Output
= <
f64
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
f64
> for
f64
Source
§
type
Output
= <
f64
as
Div
>::
Output
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
1.0.0
·
Source
§
impl
Div
<&
i8
> for &
i8
Source
§
type
Output
= <
i8
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i8
> for
i8
Source
§
type
Output
= <
i8
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i16
> for &
i16
Source
§
type
Output
= <
i16
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i16
> for
i16
Source
§
type
Output
= <
i16
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i32
> for &
i32
Source
§
type
Output
= <
i32
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i32
> for
i32
Source
§
type
Output
= <
i32
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i64
> for &
i64
Source
§
type
Output
= <
i64
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i64
> for
i64
Source
§
type
Output
= <
i64
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i128
> for &
i128
Source
§
type
Output
= <
i128
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
i128
> for
i128
Source
§
type
Output
= <
i128
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
isize
> for &
isize
Source
§
type
Output
= <
isize
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
isize
> for
isize
Source
§
type
Output
= <
isize
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u16
> for &
u16
Source
§
type
Output
= <
u16
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u16
> for
u16
Source
§
type
Output
= <
u16
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u32
> for &
u32
Source
§
type
Output
= <
u32
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u32
> for
u32
Source
§
type
Output
= <
u32
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u64
> for &
u64
Source
§
type
Output
= <
u64
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u64
> for
u64
Source
§
type
Output
= <
u64
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u128
> for &
u128
Source
§
type
Output
= <
u128
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
u128
> for
u128
Source
§
type
Output
= <
u128
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
usize
> for &
usize
Source
§
type
Output
= <
usize
as
Div
>::
Output
1.0.0
·
Source
§
impl
Div
<&
usize
> for
usize
Source
§
type
Output
= <
usize
as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
<
i8
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
<
i8
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
<
i16
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
<
i16
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
<
i32
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
<
i32
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
<
i64
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
<
i64
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
<
i128
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
<
i128
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
<
isize
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
<
isize
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
<
u8
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
<
u8
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
<
u16
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
<
u16
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
<
u32
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
<
u32
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
<
u64
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
<
u64
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
<
u128
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
<
u128
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
<
usize
> as
Div
>::
Output
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
<
usize
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
1.3.0
·
Source
§
impl
Div
<
u32
> for
Duration
Source
§
type
Output
=
Duration
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u8
>> for
u8
Source
§
type
Output
=
u8
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u16
>> for
u16
Source
§
type
Output
=
u16
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u32
>> for
u32
Source
§
type
Output
=
u32
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u64
>> for
u64
Source
§
type
Output
=
u64
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u128
>> for
u128
Source
§
type
Output
=
u128
1.51.0
·
Source
§
impl
Div
<
NonZero
<
usize
>> for
usize
Source
§
type
Output
=
usize
1.0.0
·
Source
§
impl<'a>
Div
<
f16
> for &'a
f16
Source
§
type
Output
= <
f16
as
Div
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Div
<
f64
> for &'a
f64
Source
§
type
Output
= <
f64
as
Div
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Div
<
i8
> for &'a
i8
Source
§
type
Output
= <
i8
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
i16
> for &'a
i16
Source
§
type
Output
= <
i16
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
i32
> for &'a
i32
Source
§
type
Output
= <
i32
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
i64
> for &'a
i64
Source
§
type
Output
= <
i64
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
i128
> for &'a
i128
Source
§
type
Output
= <
i128
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
isize
> for &'a
isize
Source
§
type
Output
= <
isize
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
u16
> for &'a
u16
Source
§
type
Output
= <
u16
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
u32
> for &'a
u32
Source
§
type
Output
= <
u32
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
u64
> for &'a
u64
Source
§
type
Output
= <
u64
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
u128
> for &'a
u128
Source
§
type
Output
= <
u128
as
Div
>::
Output
1.0.0
·
Source
§
impl<'a>
Div
<
usize
> for &'a
usize
Source
§
type
Output
= <
usize
as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
<
i8
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
<
i16
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
<
i32
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
<
i64
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
<
i128
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
<
isize
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
<
u8
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
<
u16
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
<
u32
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
<
u64
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
<
u128
> as
Div
>::
Output
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
<
usize
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Div
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
Source
§
impl<T, const N:
usize
>
Div
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
Source
§
impl<T, const N:
usize
>
Div
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
Source
§
impl<const N:
usize
>
Div
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>