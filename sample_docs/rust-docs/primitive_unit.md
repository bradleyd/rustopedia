unit - Rust
Primitive Type
unit
Copy item path
1.0.0
Expand description
The
()
type, also called “unit”.
The
()
type has exactly one value
()
, and is used when there
is no other meaningful value that could be returned.
()
is most
commonly seen implicitly: functions without a
-> ...
implicitly
have return type
()
, that is, these are equivalent:
fn
long() -> () {}
fn
short() {}
The semicolon
;
can be used to discard the result of an
expression at the end of a block, making the expression (and thus
the block) evaluate to
()
. For example,
fn
returns_i64() -> i64 {
1i64
}
fn
returns_unit() {
1i64
;
}
let
is_i64 = {
    returns_i64()
};
let
is_unit = {
    returns_i64();
};
Trait Implementations
§
1.0.0
·
Source
§
impl
Debug
for
()
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
()
Source
§
fn
default
()
Returns the default value of
()
1.28.0
·
Source
§
impl
Extend
<
()
> for
()
Source
§
fn
extend
<T>(&mut self, iter: T)
where
    T:
IntoIterator
<Item =
()
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, _item:
()
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
1.23.0
·
Source
§
impl
FromIterator
<
()
> for
()
Collapses all unit items from an iterator into one.
This is more useful when combined with higher-level abstractions, like
collecting to a
Result<(), E>
where you only care about errors:
use
std::io::
*
;
let
data =
vec!
[
1
,
2
,
3
,
4
,
5
];
let
res:
Result
<()> = data.iter()
    .map(|x|
writeln!
(stdout(),
"{x}"
))
    .collect();
assert!
(res.is_ok());
Source
§
fn
from_iter
<I>(iter: I)
where
    I:
IntoIterator
<Item =
()
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
Hash
for
()
Source
§
fn
hash
<H>(&self, _state:
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
Ord
for
()
Source
§
fn
cmp
(&self, _other: &
()
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
1.0.0
·
Source
§
impl
PartialEq
for
()
Source
§
fn
eq
(&self, _other: &
()
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
(&self, _other: &
()
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
()
Source
§
fn
partial_cmp
(&self, _: &
()
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
()
Source
§
fn
report
(self) ->
ExitCode
Is called to get the representation of the value as status code.
This status code is returned to the operating system.
Source
§
impl
ConstParamTy_
for
()
1.0.0
·
Source
§
impl
Eq
for
()
Source
§
impl
StructuralPartialEq
for
()
Source
§
impl
UnsizedConstParamTy
for
()
Auto Trait Implementations
§
§
impl
Freeze
for
()
§
impl
RefUnwindSafe
for
()
§
impl
Send
for
()
§
impl
Sync
for
()
§
impl
Unpin
for
()
§
impl
UnwindSafe
for
()
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