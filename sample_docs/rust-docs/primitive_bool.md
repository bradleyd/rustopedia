bool - Rust
Primitive Type
bool
Copy item path
1.0.0
Expand description
The boolean type.
The
bool
represents a value, which could only be either
true
or
false
. If you cast
a
bool
into an integer,
true
will be 1 and
false
will be 0.
§
Basic usage
bool
implements various traits, such as
BitAnd
,
BitOr
,
Not
, etc.,
which allow us to perform boolean operations using
&
,
|
and
!
.
if
requires a
bool
value as its conditional.
assert!
, which is an
important macro in testing, checks whether an expression is
true
and panics
if it isn’t.
let
bool_val =
true
&
false
|
false
;
assert!
(!bool_val);
§
Examples
A trivial example of the usage of
bool
:
let
praise_the_borrow_checker =
true
;
// using the `if` conditional
if
praise_the_borrow_checker {
println!
(
"oh, yeah!"
);
}
else
{
println!
(
"what?!!"
);
}
// ... or, a match pattern
match
praise_the_borrow_checker {
true
=>
println!
(
"keep praising!"
),
false
=>
println!
(
"you should praise!"
),
}
Also, since
bool
implements the
Copy
trait, we don’t
have to worry about the move semantics (just like the integer and float primitives).
Now an example of
bool
cast to integer type:
assert_eq!
(
true
as
i32,
1
);
assert_eq!
(
false
as
i32,
0
);
Implementations
§
Source
§
impl
bool
1.62.0
·
Source
pub fn
then_some
<T>(self, t: T) ->
Option
<T>
Returns
Some(t)
if the
bool
is
true
,
or
None
otherwise.
Arguments passed to
then_some
are eagerly evaluated; if you are
passing the result of a function call, it is recommended to use
then
, which is lazily evaluated.
§
Examples
assert_eq!
(
false
.then_some(
0
),
None
);
assert_eq!
(
true
.then_some(
0
),
Some
(
0
));
let
mut
a =
0
;
let
mut
function_with_side_effects = || { a +=
1
; };
true
.then_some(function_with_side_effects());
false
.then_some(function_with_side_effects());
// `a` is incremented twice because the value passed to `then_some` is
// evaluated eagerly.
assert_eq!
(a,
2
);
1.50.0
·
Source
pub fn
then
<T, F>(self, f: F) ->
Option
<T>
where
    F:
FnOnce
() -> T,
Returns
Some(f())
if the
bool
is
true
,
or
None
otherwise.
§
Examples
assert_eq!
(
false
.then(||
0
),
None
);
assert_eq!
(
true
.then(||
0
),
Some
(
0
));
let
mut
a =
0
;
true
.then(|| { a +=
1
; });
false
.then(|| { a +=
1
; });
// `a` is incremented once because the closure is evaluated lazily by
// `then`.
assert_eq!
(a,
1
);
Source
pub fn
select_unpredictable
<T>(self, true_val: T, false_val: T) -> T
🔬
This is a nightly-only experimental API. (
select_unpredictable
#133962
)
Returns either
true_val
or
false_val
depending on the value of
self
, with a hint to the compiler that
self
is unlikely
to be correctly predicted by a CPU’s branch predictor.
This method is functionally equivalent to
ⓘ
fn
select_unpredictable<T>(b: bool, true_val: T, false_val: T) -> T {
if
b { true_val }
else
{ false_val }
}
but might generate different assembly. In particular, on platforms with
a conditional move or select instruction (like
cmov
on x86 or
csel
on ARM) the optimizer might use these instructions to avoid branches,
which can benefit performance if the branch predictor is struggling
with predicting
condition
, such as in an implementation of  binary
search.
Note however that this lowering is not guaranteed (on any platform) and
should not be relied upon when trying to write constant-time code. Also
be aware that this lowering might
decrease
performance if
condition
is well-predictable. It is advisable to perform benchmarks to tell if
this function is useful.
§
Examples
Distribute values evenly between two buckets:
#![feature(select_unpredictable)]
use
std::hash::BuildHasher;
fn
append<H: BuildHasher>(hasher:
&
H, v: i32, bucket_one:
&mut
Vec<i32>, bucket_two:
&mut
Vec<i32>) {
let
hash = hasher.hash_one(
&
v);
let
bucket = (hash %
2
==
0
).select_unpredictable(bucket_one, bucket_two);
    bucket.push(v);
}
Trait Implementations
§
1.0.0
·
Source
§
impl
BitAnd
<&
bool
> for &
bool
Source
§
type
Output
= <
bool
as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
bool
) -> <
bool
as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
<&
bool
> for
bool
Source
§
type
Output
= <
bool
as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
bool
) -> <
bool
as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAnd
<
Mask
<T, N>> for
bool
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl<'a>
BitAnd
<
bool
> for &'a
bool
Source
§
type
Output
= <
bool
as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
bool
) -> <
bool
as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAnd
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
bool
) ->
Mask
<T, N>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
bool
Source
§
type
Output
=
bool
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
bool
) ->
bool
Performs the
&
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
bool
> for
bool
Source
§
fn
bitand_assign
(&mut self, other: &
bool
)
Performs the
&=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAndAssign
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitand_assign
(&mut self, rhs:
bool
)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
bool
Source
§
fn
bitand_assign
(&mut self, other:
bool
)
Performs the
&=
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
<&
bool
> for &
bool
Source
§
type
Output
= <
bool
as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
bool
) -> <
bool
as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
<&
bool
> for
bool
Source
§
type
Output
= <
bool
as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
bool
) -> <
bool
as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOr
<
Mask
<T, N>> for
bool
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl<'a>
BitOr
<
bool
> for &'a
bool
Source
§
type
Output
= <
bool
as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
bool
) -> <
bool
as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOr
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
bool
) ->
Mask
<T, N>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
bool
Source
§
type
Output
=
bool
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
bool
) ->
bool
Performs the
|
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
bool
> for
bool
Source
§
fn
bitor_assign
(&mut self, other: &
bool
)
Performs the
|=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOrAssign
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitor_assign
(&mut self, rhs:
bool
)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
bool
Source
§
fn
bitor_assign
(&mut self, other:
bool
)
Performs the
|=
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
<&
bool
> for &
bool
Source
§
type
Output
= <
bool
as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
bool
) -> <
bool
as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
<&
bool
> for
bool
Source
§
type
Output
= <
bool
as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
bool
) -> <
bool
as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXor
<
Mask
<T, N>> for
bool
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Mask
<T, N>) -> <
bool
as
BitXor
<
Mask
<T, N>>>::
Output
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl<'a>
BitXor
<
bool
> for &'a
bool
Source
§
type
Output
= <
bool
as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
bool
) -> <
bool
as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXor
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
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
Mask
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
bool
) -> <
Mask
<T, N> as
BitXor
<
bool
>>::
Output
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
bool
Source
§
type
Output
=
bool
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
bool
) ->
bool
Performs the
^
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
bool
> for
bool
Source
§
fn
bitxor_assign
(&mut self, other: &
bool
)
Performs the
^=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXorAssign
<
bool
> for
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitxor_assign
(&mut self, rhs:
bool
)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
bool
Source
§
fn
bitxor_assign
(&mut self, other:
bool
)
Performs the
^=
operation.
Read more
1.0.0
·
Source
§
impl
Clone
for
bool
Source
§
fn
clone
(&self) ->
bool
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
bool
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
bool
Source
§
fn
default
() ->
bool
Returns the default value of
false
Source
§
impl
DisjointBitOr
for
bool
Source
§
const unsafe fn
disjoint_bitor
(self, other:
bool
) ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
See
super::disjoint_bitor
; we just need the trait indirection to handle
different types since calling intrinsics with generics doesn’t work.
1.0.0
·
Source
§
impl
Display
for
bool
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
1.24.0
·
Source
§
impl
From
<
bool
> for
AtomicBool
Source
§
fn
from
(b:
bool
) ->
AtomicBool
Converts a
bool
into an
AtomicBool
.
§
Examples
use
std::sync::atomic::AtomicBool;
let
atomic_bool = AtomicBool::from(
true
);
assert_eq!
(
format!
(
"{atomic_bool:?}"
),
"true"
)
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
1.68.0
·
Source
§
impl
From
<
bool
> for
f16
Source
§
fn
from
(small:
bool
) ->
f16
Converts a
bool
to
f16
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
#![feature(f16)]
let
x: f16 =
false
.into();
assert_eq!
(x,
0.0
);
assert!
(x.is_sign_positive());
let
y: f16 =
true
.into();
assert_eq!
(y,
1.0
);
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
1.68.0
·
Source
§
impl
From
<
bool
> for
f64
Source
§
fn
from
(small:
bool
) ->
f64
Converts a
bool
to
f64
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
x: f64 =
false
.into();
assert_eq!
(x,
0.0
);
assert!
(x.is_sign_positive());
let
y: f64 =
true
.into();
assert_eq!
(y,
1.0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
i128
Source
§
fn
from
(small:
bool
) ->
i128
Converts a
bool
to
i128
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(i128::from(
true
),
1
);
assert_eq!
(i128::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
i16
Source
§
fn
from
(small:
bool
) ->
i16
Converts a
bool
to
i16
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(i16::from(
true
),
1
);
assert_eq!
(i16::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
i32
Source
§
fn
from
(small:
bool
) ->
i32
Converts a
bool
to
i32
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(i32::from(
true
),
1
);
assert_eq!
(i32::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
i64
Source
§
fn
from
(small:
bool
) ->
i64
Converts a
bool
to
i64
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(i64::from(
true
),
1
);
assert_eq!
(i64::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
i8
Source
§
fn
from
(small:
bool
) ->
i8
Converts a
bool
to
i8
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(i8::from(
true
),
1
);
assert_eq!
(i8::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
isize
Source
§
fn
from
(small:
bool
) ->
isize
Converts a
bool
to
isize
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(isize::from(
true
),
1
);
assert_eq!
(isize::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
u128
Source
§
fn
from
(small:
bool
) ->
u128
Converts a
bool
to
u128
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u128::from(
true
),
1
);
assert_eq!
(u128::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
u16
Source
§
fn
from
(small:
bool
) ->
u16
Converts a
bool
to
u16
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u16::from(
true
),
1
);
assert_eq!
(u16::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
u32
Source
§
fn
from
(small:
bool
) ->
u32
Converts a
bool
to
u32
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u32::from(
true
),
1
);
assert_eq!
(u32::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
u64
Source
§
fn
from
(small:
bool
) ->
u64
Converts a
bool
to
u64
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u64::from(
true
),
1
);
assert_eq!
(u64::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
u8
Source
§
fn
from
(small:
bool
) ->
u8
Converts a
bool
to
u8
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u8::from(
true
),
1
);
assert_eq!
(u8::from(
false
),
0
);
1.28.0
·
Source
§
impl
From
<
bool
> for
usize
Source
§
fn
from
(small:
bool
) ->
usize
Converts a
bool
to
usize
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(usize::from(
true
),
1
);
assert_eq!
(usize::from(
false
),
0
);
1.0.0
·
Source
§
impl
FromStr
for
bool
Source
§
fn
from_str
(s: &
str
) ->
Result
<
bool
,
ParseBoolError
>
Parse a
bool
from a string.
The only accepted values are
"true"
and
"false"
. Any other input
will return an error.
§
Examples
use
std::str::FromStr;
assert_eq!
(FromStr::from_str(
"true"
),
Ok
(
true
));
assert_eq!
(FromStr::from_str(
"false"
),
Ok
(
false
));
assert!
(<bool
as
FromStr>::from_str(
"not even a boolean"
).is_err());
Note, in many cases, the
.parse()
method on
str
is more proper.
assert_eq!
(
"true"
.parse(),
Ok
(
true
));
assert_eq!
(
"false"
.parse(),
Ok
(
false
));
assert!
(
"not even a boolean"
.parse::<bool>().is_err());
Source
§
type
Err
=
ParseBoolError
The associated error which can be returned from parsing.
1.0.0
·
Source
§
impl
Hash
for
bool
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.0.0
·
Source
§
impl
Not
for &
bool
Source
§
type
Output
= <
bool
as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
bool
as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
bool
Source
§
type
Output
=
bool
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
bool
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Ord
for
bool
Source
§
fn
cmp
(&self, other: &
bool
) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
Source
§
fn
min
(self, other:
bool
) ->
bool
Compares and returns the minimum of two values.
Read more
Source
§
fn
max
(self, other:
bool
) ->
bool
Compares and returns the maximum of two values.
Read more
Source
§
fn
clamp
(self, min:
bool
, max:
bool
) ->
bool
Restrict a value to a certain interval.
Read more
1.0.0
·
Source
§
impl
PartialEq
for
bool
Source
§
fn
eq
(&self, other: &
bool
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
bool
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
bool
Source
§
fn
partial_cmp
(&self, other: &
bool
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
bool
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
bool
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
bool
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
bool
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
Source
§
impl
Random
for
bool
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
bool
🔬
This is a nightly-only experimental API. (
random
#130703
)
Generates a random value.
Source
§
impl
ConstParamTy_
for
bool
1.0.0
·
Source
§
impl
Copy
for
bool
1.0.0
·
Source
§
impl
Eq
for
bool
Source
§
impl
StructuralPartialEq
for
bool
Source
§
impl
UnsizedConstParamTy
for
bool
Source
§
impl
UseCloned
for
bool
Auto Trait Implementations
§
§
impl
Freeze
for
bool
§
impl
RefUnwindSafe
for
bool
§
impl
Send
for
bool
§
impl
Sync
for
bool
§
impl
Unpin
for
bool
§
impl
UnwindSafe
for
bool
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