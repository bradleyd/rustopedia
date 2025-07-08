IntoStringError in std::ffi::c_str - Rust
std
::
ffi
::
c_str
Struct
IntoStringError
Copy item path
Source
pub struct IntoStringError {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Expand description
An error indicating invalid UTF-8 when converting a
CString
into a
String
.
CString
is just a wrapper over a buffer of bytes with a nul terminator;
CString::into_string
performs UTF-8 validation on those bytes and may
return this error.
This
struct
is created by
CString::into_string()
. See
its documentation for more.
Implementations
§
Source
§
impl
IntoStringError
1.7.0
·
Source
pub fn
into_cstring
(self) ->
CString
Consumes this error, returning original
CString
which generated the
error.
1.7.0
·
Source
pub fn
utf8_error
(&self) ->
Utf8Error
Access the underlying UTF-8 error that was the cause of this error.
Trait Implementations
§
1.64.0
·
Source
§
impl
Clone
for
IntoStringError
Source
§
fn
clone
(&self) ->
IntoStringError
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
IntoStringError
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
1.7.0
·
Source
§
impl
Display
for
IntoStringError
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
1.7.0
·
Source
§
impl
Error
for
IntoStringError
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
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
IntoStringError
Source
§
fn
eq
(&self, other: &
IntoStringError
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
Eq
for
IntoStringError
1.64.0
·
Source
§
impl
StructuralPartialEq
for
IntoStringError
Auto Trait Implementations
§
§
impl
Freeze
for
IntoStringError
§
impl
RefUnwindSafe
for
IntoStringError
§
impl
Send
for
IntoStringError
§
impl
Sync
for
IntoStringError
§
impl
Unpin
for
IntoStringError
§
impl
UnwindSafe
for
IntoStringError
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