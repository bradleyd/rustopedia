Add in std::ops - Rust
std
::
ops
Trait
Add
Copy item path
1.0.0 (const:
unstable
)
·
Source
pub trait Add<Rhs = Self> {
    type
Output
;

    // Required method
    const fn
add
(self, rhs: Rhs) -> Self::
Output
;
}
Expand description
The addition operator
+
.
Note that
Rhs
is
Self
by default, but this is not mandatory. For
example,
std::time::SystemTime
implements
Add<Duration>
, which permits
operations of the form
SystemTime = SystemTime + Duration
.
§
Examples
§
Add
able points
use
std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct
Point {
    x: i32,
    y: i32,
}
impl
Add
for
Point {
type
Output =
Self
;
fn
add(
self
, other:
Self
) ->
Self
{
Self
{
            x:
self
.x + other.x,
            y:
self
.y + other.y,
        }
    }
}
assert_eq!
(Point { x:
1
, y:
0
} + Point { x:
2
, y:
3
},
           Point { x:
3
, y:
3
});
§
Implementing
Add
with generics
Here is an example of the same
Point
struct implementing the
Add
trait
using generics.
use
std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct
Point<T> {
    x: T,
    y: T,
}
// Notice that the implementation uses the associated type `Output`.
impl
<T: Add<Output = T>> Add
for
Point<T> {
type
Output =
Self
;
fn
add(
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
.x + other.x,
            y:
self
.y + other.y,
        }
    }
}
assert_eq!
(Point { x:
1
, y:
0
} + Point { x:
2
, y:
3
},
           Point { x:
3
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
+
operator.
Required Methods
§
1.0.0
·
Source
const fn
add
(self, rhs: Rhs) -> Self::
Output
Performs the
+
operation.
§
Example
assert_eq!
(
12
+
1
,
13
);
Implementors
§
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
f16
Source
§
type
Output
=
f16
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
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
f64
Source
§
type
Output
=
f64
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
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
i8
Source
§
type
Output
=
i8
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
i16
Source
§
type
Output
=
i16
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
i32
Source
§
type
Output
=
i32
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
i64
Source
§
type
Output
=
i64
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
i128
Source
§
type
Output
=
i128
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
isize
Source
§
type
Output
=
isize
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u8
Source
§
type
Output
=
u8
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u16
Source
§
type
Output
=
u16
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u32
Source
§
type
Output
=
u32
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u64
Source
§
type
Output
=
u64
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u128
Source
§
type
Output
=
u128
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
for
Duration
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
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
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
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
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
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
<&
str
> for
String
Implements the
+
operator for concatenating two strings.
This consumes the
String
on the left-hand side and re-uses its buffer (growing it if
necessary). This is done to avoid allocating a new
String
and copying the entire contents on
every operation, which would lead to
O
(
n
^2) running time when building an
n
-byte string by
repeated concatenation.
The string on the right-hand side is only borrowed; its contents are copied into the returned
String
.
§
Examples
Concatenating two
String
s takes the first by value and borrows the second:
let
a = String::from(
"hello"
);
let
b = String::from(
" world"
);
let
c = a +
&
b;
// `a` is moved and can no longer be used here.
If you want to keep using the first
String
, you can clone it and append to the clone instead:
let
a = String::from(
"hello"
);
let
b = String::from(
" world"
);
let
c = a.clone() +
&
b;
// `a` is still valid here.
Concatenating
&str
slices can be done by converting the first to a
String
:
let
a =
"hello"
;
let
b =
" world"
;
let
c = a.to_string() + b;
Source
§
type
Output
=
String
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl
Add
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
Add
>::
Output
1.8.0
·
Source
§
impl
Add
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
Add
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
1.14.0
·
Source
§
impl<'a>
Add
for
Cow
<'a,
str
>
Source
§
type
Output
=
Cow
<'a,
str
>
1.14.0
·
Source
§
impl<'a>
Add
<&'a
str
> for
Cow
<'a,
str
>
Source
§
type
Output
=
Cow
<'a,
str
>
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
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
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.0.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.74.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
1.14.0
·
Source
§
impl<'a>
Add
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
Add
>::
Output
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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
Add
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