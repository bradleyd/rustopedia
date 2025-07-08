Infallible in std::convert - Rust
std
::
convert
Enum
Infallible
Copy item path
1.34.0
·
Source
pub enum Infallible {}
Expand description
The error type for errors that can never happen.
Since this enum has no variant, a value of this type can never actually exist.
This can be useful for generic APIs that use
Result
and parameterize the error type,
to indicate that the result is always
Ok
.
For example, the
TryFrom
trait (conversion that returns a
Result
)
has a blanket implementation for all types where a reverse
Into
implementation exists.
ⓘ
impl
<T, U> TryFrom<U>
for
T
where
U: Into<T> {
type
Error = Infallible;
fn
try_from(value: U) ->
Result
<
Self
, Infallible> {
Ok
(U::into(value))
// Never returns `Err`
}
}
§
Future compatibility
This enum has the same role as
the
!
“never” type
,
which is unstable in this version of Rust.
When
!
is stabilized, we plan to make
Infallible
a type alias to it:
ⓘ
pub type
Infallible = !;
… and eventually deprecate
Infallible
.
However there is one case where
!
syntax can be used
before
!
is stabilized as a full-fledged type: in the position of a function’s return type.
Specifically, it is possible to have implementations for two different function pointer types:
trait
MyTrait {}
impl
MyTrait
for fn
() -> ! {}
impl
MyTrait
for fn
() -> std::convert::Infallible {}
With
Infallible
being an enum, this code is valid.
However when
Infallible
becomes an alias for the never type,
the two
impl
s will start to overlap
and therefore will be disallowed by the language’s trait coherence rules.
Trait Implementations
§
1.34.0
·
Source
§
impl
Clone
for
Infallible
Source
§
fn
clone
(&self) ->
Infallible
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
1.34.0
·
Source
§
impl
Debug
for
Infallible
Source
§
fn
fmt
(&self, _: &mut
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
1.34.0
·
Source
§
impl
Display
for
Infallible
Source
§
fn
fmt
(&self, _: &mut
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
1.8.0
·
Source
§
impl
Error
for
Infallible
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.30.0
·
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
1.0.0
·
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.34.0
·
Source
§
impl
From
<
!
> for
Infallible
Source
§
fn
from
(x:
!
) ->
Infallible
Converts to this type from the input type.
1.34.0
·
Source
§
impl
From
<
Infallible
> for
TryFromIntError
Source
§
fn
from
(x:
Infallible
) ->
TryFromIntError
Converts to this type from the input type.
1.36.0
·
Source
§
impl
From
<
Infallible
> for
TryFromSliceError
Source
§
fn
from
(x:
Infallible
) ->
TryFromSliceError
Converts to this type from the input type.
1.44.0
·
Source
§
impl
Hash
for
Infallible
Source
§
fn
hash
<H>(&self, _:
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
1.34.0
·
Source
§
impl
Ord
for
Infallible
Source
§
fn
cmp
(&self, _other: &
Infallible
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
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.34.0
·
Source
§
impl
PartialEq
for
Infallible
Source
§
fn
eq
(&self, _: &
Infallible
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
1.34.0
·
Source
§
impl
PartialOrd
for
Infallible
Source
§
fn
partial_cmp
(&self, _other: &
Infallible
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
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
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
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
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
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
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
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
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
1.61.0
·
Source
§
impl
Termination
for
Infallible
Source
§
fn
report
(self) ->
ExitCode
Is called to get the representation of the value as status code.
This status code is returned to the operating system.
1.34.0
·
Source
§
impl
Copy
for
Infallible
1.34.0
·
Source
§
impl
Eq
for
Infallible
Auto Trait Implementations
§
§
impl
Freeze
for
Infallible
§
impl
RefUnwindSafe
for
Infallible
§
impl
Send
for
Infallible
§
impl
Sync
for
Infallible
§
impl
Unpin
for
Infallible
§
impl
UnwindSafe
for
Infallible
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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