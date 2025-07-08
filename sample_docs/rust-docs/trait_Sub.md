Sub in std::ops - Rust
std
::
ops
Trait
Sub
Copy item path
1.0.0
·
Source
pub trait Sub<Rhs = Self> {
    type
Output
;

    // Required method
    fn
sub
(self, rhs: Rhs) -> Self::
Output
;
}
Expand description
The subtraction operator
-
.
Note that
Rhs
is
Self
by default, but this is not mandatory. For
example,
std::time::SystemTime
implements
Sub<Duration>
, which permits
operations of the form
SystemTime = SystemTime - Duration
.
§
Examples
§
Sub
tractable points
use
std::ops::Sub;
#[derive(Debug, Copy, Clone, PartialEq)]
struct
Point {
    x: i32,
    y: i32,
}
impl
Sub
for
Point {
type
Output =
Self
;
fn
sub(
self
, other:
Self
) ->
Self
::Output {
Self
{
            x:
self
.x - other.x,
            y:
self
.y - other.y,
        }
    }
}
assert_eq!
(Point { x:
3
, y:
3
} - Point { x:
2
, y:
3
},
           Point { x:
1
, y:
0
});
§
Implementing
Sub
with generics
Here is an example of the same
Point
struct implementing the
Sub
trait
using generics.
use
std::ops::Sub;
#[derive(Debug, PartialEq)]
struct
Point<T> {
    x: T,
    y: T,
}
// Notice that the implementation uses the associated type `Output`.
impl
<T: Sub<Output = T>> Sub
for
Point<T> {
type
Output =
Self
;
fn
sub(
self
, other:
Self
) ->
Self
::Output {
        Point {
            x:
self
.x - other.x,
            y:
self
.y - other.y,
        }
    }
}
assert_eq!
(Point { x:
2
, y:
3
} - Point { x:
1
, y:
0
},
           Point { x:
1
, y:
3
});
Required Associated Types
§
1.0.0
·
Source
type
Output
The resulting type after applying the
-
operator.
Required Methods
§
1.0.0
·
Source
fn
sub
(self, rhs: Rhs) -> Self::
Output
Performs the
-
operation.
§
Example
assert_eq!
(
12
-
1
,
11
);
Implementors
§
1.0.0
·
Source
§
impl
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
for
usize
Source
§
type
Output
=
usize
Source
§
impl
Sub
for
Assume
Source
§
type
Output
=
Assume
1.74.0
·
Source
§
impl
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
1.3.0
·
Source
§
impl
Sub
for
Duration
Source
§
type
Output
=
Duration
1.8.0
·
Source
§
impl
Sub
for
Instant
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
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
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
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
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
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl
Sub
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
Sub
>::
Output
1.8.0
·
Source
§
impl
Sub
<
Duration
> for
Instant
Source
§
type
Output
=
Instant
1.8.0
·
Source
§
impl
Sub
<
Duration
> for
SystemTime
Source
§
type
Output
=
SystemTime
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.0.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.74.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
1.14.0
·
Source
§
impl<'a>
Sub
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
Sub
>::
Output
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Sub
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
Sub
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
1.0.0
·
Source
§
impl<T, A>
Sub
<&
BTreeSet
<T, A>> for &
BTreeSet
<T, A>
where
    T:
Ord
+
Clone
,
    A:
Allocator
+
Clone
,
Source
§
type
Output
=
BTreeSet
<T, A>
1.0.0
·
Source
§
impl<T, S>
Sub
<&
HashSet
<T, S>> for &
HashSet
<T, S>
where
    T:
Eq
+
Hash
+
Clone
,
    S:
BuildHasher
+
Default
,
Source
§
type
Output
=
HashSet
<T, S>
Source
§
impl<T, const N:
usize
>
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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
Sub
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