FpCategory in std::num - Rust
std
::
num
Enum
FpCategory
Copy item path
1.0.0
·
Source
pub enum FpCategory {
    Nan,
    Infinite,
    Zero,
    Subnormal,
    Normal,
}
Expand description
A classification of floating point numbers.
This
enum
is used as the return type for
f32::classify
and
f64::classify
. See
their documentation for more.
§
Examples
use
std::num::FpCategory;
let
num =
12.4_f32
;
let
inf = f32::INFINITY;
let
zero =
0f32
;
let
sub: f32 =
1.1754942e-38
;
let
nan = f32::NAN;
assert_eq!
(num.classify(), FpCategory::Normal);
assert_eq!
(inf.classify(), FpCategory::Infinite);
assert_eq!
(zero.classify(), FpCategory::Zero);
assert_eq!
(sub.classify(), FpCategory::Subnormal);
assert_eq!
(nan.classify(), FpCategory::Nan);
Variants
§
§
1.0.0
Nan
NaN (not a number): this value results from calculations like
(-1.0).sqrt()
.
See
the documentation for
f32
for more information on the unusual properties
of NaN.
§
1.0.0
Infinite
Positive or negative infinity, which often results from dividing a nonzero number
by zero.
§
1.0.0
Zero
Positive or negative zero.
See
the documentation for
f32
for more information on the signedness of zeroes.
§
1.0.0
Subnormal
“Subnormal” or “denormal” floating point representation (less precise, relative to
their magnitude, than
Normal
).
Subnormal numbers are larger in magnitude than
Zero
but smaller in magnitude than all
Normal
numbers.
§
1.0.0
Normal
A regular floating point number, not any of the exceptional categories.
The smallest positive normal numbers are
f32::MIN_POSITIVE
and
f64::MIN_POSITIVE
,
and the largest positive normal numbers are
f32::MAX
and
f64::MAX
. (Unlike signed
integers, floating point numbers are symmetric in their range, so negating any of these
constants will produce their negative counterpart.)
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
FpCategory
Source
§
fn
clone
(&self) ->
FpCategory
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
FpCategory
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
PartialEq
for
FpCategory
Source
§
fn
eq
(&self, other: &
FpCategory
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
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
Copy
for
FpCategory
1.0.0
·
Source
§
impl
Eq
for
FpCategory
1.0.0
·
Source
§
impl
StructuralPartialEq
for
FpCategory
Auto Trait Implementations
§
§
impl
Freeze
for
FpCategory
§
impl
RefUnwindSafe
for
FpCategory
§
impl
Send
for
FpCategory
§
impl
Sync
for
FpCategory
§
impl
Unpin
for
FpCategory
§
impl
UnwindSafe
for
FpCategory
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