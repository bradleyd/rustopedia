IntErrorKind in std::num - Rust
std
::
num
Enum
IntErrorKind
Copy item path
1.55.0
·
Source
#[non_exhaustive]
pub enum IntErrorKind {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
}
Expand description
Enum to store the various types of errors that can cause parsing an integer to fail.
§
Example
if let
Err
(e) = i32::from_str_radix(
"a12"
,
10
) {
println!
(
"Failed conversion to i32: {:?}"
, e.kind());
}
Variants (Non-exhaustive)
§
This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
1.55.0
Empty
Value being parsed is empty.
This variant will be constructed when parsing an empty string.
§
1.55.0
InvalidDigit
Contains an invalid digit in its context.
Among other causes, this variant will be constructed when parsing a string that
contains a non-ASCII char.
This variant is also constructed when a
+
or
-
is misplaced within a string
either on its own or in the middle of a number.
§
1.55.0
PosOverflow
Integer is too large to store in target integer type.
§
1.55.0
NegOverflow
Integer is too small to store in target integer type.
§
1.55.0
Zero
Value was Zero
This variant will be emitted when the parsing string has a value of zero, which
would be illegal for non-zero types.
Trait Implementations
§
1.55.0
·
Source
§
impl
Clone
for
IntErrorKind
Source
§
fn
clone
(&self) ->
IntErrorKind
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
1.55.0
·
Source
§
impl
Debug
for
IntErrorKind
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
1.55.0
·
Source
§
impl
PartialEq
for
IntErrorKind
Source
§
fn
eq
(&self, other: &
IntErrorKind
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
1.55.0
·
Source
§
impl
Eq
for
IntErrorKind
1.55.0
·
Source
§
impl
StructuralPartialEq
for
IntErrorKind
Auto Trait Implementations
§
§
impl
Freeze
for
IntErrorKind
§
impl
RefUnwindSafe
for
IntErrorKind
§
impl
Send
for
IntErrorKind
§
impl
Sync
for
IntErrorKind
§
impl
Unpin
for
IntErrorKind
§
impl
UnwindSafe
for
IntErrorKind
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