FromBytesWithNulError in std::ffi::c_str - Rust
std
::
ffi
::
c_str
Enum
FromBytesWithNulError
Copy item path
Source
pub enum FromBytesWithNulError {
    InteriorNul {
        position:
usize
,
    },
    NotNulTerminated,
}
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Expand description
An error indicating that a nul byte was not in the expected position.
The slice used to create a
CStr
must have one and only one nul byte,
positioned at the end.
This error is created by the
CStr::from_bytes_with_nul
method.
See its documentation for more.
§
Examples
use
std::ffi::{CStr, FromBytesWithNulError};
let _
: FromBytesWithNulError = CStr::from_bytes_with_nul(
b"f\0oo"
).unwrap_err();
Variants
§
§
InteriorNul
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Data provided contains an interior nul byte at byte
position
.
Fields
§
position:
usize
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
The position of the interior nul byte.
§
NotNulTerminated
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Data provided is not nul terminated.
Trait Implementations
§
1.64.0
·
Source
§
impl
Clone
for
FromBytesWithNulError
Source
§
fn
clone
(&self) ->
FromBytesWithNulError
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
1.64.0
·
Source
§
impl
Debug
for
FromBytesWithNulError
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
1.17.0
·
Source
§
impl
Display
for
FromBytesWithNulError
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
1.17.0
·
Source
§
impl
Error
for
FromBytesWithNulError
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
1.64.0
·
Source
§
impl
PartialEq
for
FromBytesWithNulError
Source
§
fn
eq
(&self, other: &
FromBytesWithNulError
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
1.64.0
·
Source
§
impl
Copy
for
FromBytesWithNulError
1.64.0
·
Source
§
impl
Eq
for
FromBytesWithNulError
1.64.0
·
Source
§
impl
StructuralPartialEq
for
FromBytesWithNulError
Auto Trait Implementations
§
§
impl
Freeze
for
FromBytesWithNulError
§
impl
RefUnwindSafe
for
FromBytesWithNulError
§
impl
Send
for
FromBytesWithNulError
§
impl
Sync
for
FromBytesWithNulError
§
impl
Unpin
for
FromBytesWithNulError
§
impl
UnwindSafe
for
FromBytesWithNulError
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