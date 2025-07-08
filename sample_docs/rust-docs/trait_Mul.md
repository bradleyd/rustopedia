Mul in std::ops - Rust
std
::
ops
Trait
Mul
Copy item path
1.0.0
·
Source
pub trait Mul<Rhs = Self> {
    type
Output
;

    // Required method
    fn
mul
(self, rhs: Rhs) -> Self::
Output
;
}
Expand description
The multiplication operator
*
.
Note that
Rhs
is
Self
by default, but this is not mandatory.
§
Examples
§
Mul
tipliable rational numbers
use
std::ops::Mul;
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
Mul
for
Rational {
// The multiplication of rational numbers is a closed operation.
type
Output =
Self
;
fn
mul(
self
, rhs:
Self
) ->
Self
{
let
numerator =
self
.numerator * rhs.numerator;
let
denominator =
self
.denominator * rhs.denominator;
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
2
,
3
) * Rational::new(
3
,
4
),
           Rational::new(
1
,
2
));
§
Multiplying vectors by scalars as in linear algebra
use
std::ops::Mul;
struct
Scalar { value: usize }
#[derive(Debug, PartialEq)]
struct
Vector { value: Vec<usize> }
impl
Mul<Scalar>
for
Vector {
type
Output =
Self
;
fn
mul(
self
, rhs: Scalar) ->
Self
::Output {
Self
{ value:
self
.value.iter().map(|v| v * rhs.value).collect() }
    }
}
let
vector = Vector { value:
vec!
[
2
,
4
,
6
] };
let
scalar = Scalar { value:
3
};
assert_eq!
(vector * scalar, Vector { value:
vec!
[
6
,
12
,
18
] });
Required Associated Types
§
1.0.0
·
Source
type
Output
The resulting type after applying the
*
operator.
Required Methods
§
1.0.0
·
Source
fn
mul
(self, rhs: Rhs) -> Self::
Output
Performs the
*
operation.
§
Example
assert_eq!
(
12
*
2
,
24
);
Implementors
§
1.0.0
·
Source
§
impl
Mul
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
Mul
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
Mul
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
Mul
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
Mul
for
i8
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
Mul
for
i16
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
Mul
for
i32
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
Mul
for
i64
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
Mul
for
i128
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
Mul
for
isize
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
Mul
for
u8
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
Mul
for
u16
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
Mul
for
u32
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
Mul
for
u64
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
Mul
for
u128
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
Mul
for
usize
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
Mul
for
Saturating
<
i8
>
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
Mul
for
Saturating
<
i16
>
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
Mul
for
Saturating
<
i32
>
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
Mul
for
Saturating
<
i64
>
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
Mul
for
Saturating
<
i128
>
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
Mul
for
Saturating
<
isize
>
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
Mul
for
Saturating
<
u8
>
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
Mul
for
Saturating
<
u16
>
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
Mul
for
Saturating
<
u32
>
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
Mul
for
Saturating
<
u64
>
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
Mul
for
Saturating
<
u128
>
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
Mul
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
<
usize
>
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
1.0.0
·
Source
§
impl
Mul
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
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
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
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
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
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl
Mul
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
Mul
>::
Output
1.3.0
·
Source
§
impl
Mul
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
1.31.0
·
Source
§
impl
Mul
<
Duration
> for
u32
Source
§
type
Output
=
Duration
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.0.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.74.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
1.14.0
·
Source
§
impl<'a>
Mul
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
Mul
>::
Output
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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
Mul
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