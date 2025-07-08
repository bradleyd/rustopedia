Rem in std::ops - Rust
std
::
ops
Trait
Rem
Copy item path
1.0.0
·
Source
pub trait Rem<Rhs = Self> {
    type
Output
;

    // Required method
    fn
rem
(self, rhs: Rhs) -> Self::
Output
;
}
Expand description
The remainder operator
%
.
Note that
Rhs
is
Self
by default, but this is not mandatory.
§
Examples
This example implements
Rem
on a
SplitSlice
object. After
Rem
is
implemented, one can use the
%
operator to find out what the remaining
elements of the slice would be after splitting it into equal slices of a
given length.
use
std::ops::Rem;
#[derive(PartialEq, Debug)]
struct
SplitSlice<
'a
, T> {
    slice:
&
'a
[T],
}
impl
<
'a
, T> Rem<usize>
for
SplitSlice<
'a
, T> {
type
Output =
Self
;
fn
rem(
self
, modulus: usize) ->
Self
::Output {
let
len =
self
.slice.len();
let
rem = len % modulus;
let
start = len - rem;
Self
{slice:
&
self
.slice[start..]}
    }
}
// If we were to divide &[0, 1, 2, 3, 4, 5, 6, 7] into slices of size 3,
// the remainder would be &[6, 7].
assert_eq!
(SplitSlice { slice:
&
[
0
,
1
,
2
,
3
,
4
,
5
,
6
,
7
] } %
3
,
           SplitSlice { slice:
&
[
6
,
7
] });
Required Associated Types
§
1.0.0
·
Source
type
Output
The resulting type after applying the
%
operator.
Required Methods
§
1.0.0
·
Source
fn
rem
(self, rhs: Rhs) -> Self::
Output
Performs the
%
operation.
§
Example
assert_eq!
(
12
%
10
,
2
);
Implementors
§
1.0.0
·
Source
§
impl
Rem
for
f16
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
f16
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
1.0.0
·
Source
§
impl
Rem
for
f64
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
f64
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
1.0.0
·
Source
§
impl
Rem
for
i8
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
i16
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
i32
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
i64
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
i128
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
isize
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
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
Rem
for
u8
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
for
u16
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
for
u32
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
for
u64
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
for
u128
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
for
usize
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
1.7.0
·
Source
§
impl
Rem
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
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
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
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
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
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl
Rem
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
Rem
>::
Output
1.51.0
·
Source
§
impl
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.0.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.74.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
1.14.0
·
Source
§
impl<'a>
Rem
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
Rem
>::
Output
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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
Rem
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