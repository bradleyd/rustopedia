NulError in std::ffi::c_str - Rust
std
::
ffi
::
c_str
Struct
NulError
Copy item path
Source
pub struct NulError(
/* private fields */
);
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Expand description
An error indicating that an interior nul byte was found.
While Rust strings may contain nul bytes in the middle, C strings
can’t, as that byte would effectively truncate the string.
This error is created by the
new
method on
CString
. See its documentation for more.
§
Examples
use
std::ffi::{CString, NulError};
let _
: NulError = CString::new(
b"f\0oo"
.to_vec()).unwrap_err();
Implementations
§
Source
§
impl
NulError
1.0.0
·
Source
pub fn
nul_position
(&self) ->
usize
Returns the position of the nul byte in the slice that caused
CString::new
to fail.
§
Examples
use
std::ffi::CString;
let
nul_error = CString::new(
"foo\0bar"
).unwrap_err();
assert_eq!
(nul_error.nul_position(),
3
);
let
nul_error = CString::new(
"foo bar\0"
).unwrap_err();
assert_eq!
(nul_error.nul_position(),
7
);
1.0.0
·
Source
pub fn
into_vec
(self) ->
Vec
<
u8
>
ⓘ
Consumes this error, returning the underlying vector of bytes which
generated the error in the first place.
§
Examples
use
std::ffi::CString;
let
nul_error = CString::new(
"foo\0bar"
).unwrap_err();
assert_eq!
(nul_error.into_vec(),
b"foo\0bar"
);
Trait Implementations
§
1.64.0
·
Source
§
impl
Clone
for
NulError
Source
§
fn
clone
(&self) ->
NulError
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
NulError
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
Display
for
NulError
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
Error
for
NulError
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
1.0.0
·
Source
§
impl
From
<
NulError
> for
Error
Source
§
fn
from
(_:
NulError
) ->
Error
Converts a
alloc::ffi::NulError
into a
Error
.
1.64.0
·
Source
§
impl
PartialEq
for
NulError
Source
§
fn
eq
(&self, other: &
NulError
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
NulError
1.64.0
·
Source
§
impl
StructuralPartialEq
for
NulError
Auto Trait Implementations
§
§
impl
Freeze
for
NulError
§
impl
RefUnwindSafe
for
NulError
§
impl
Send
for
NulError
§
impl
Sync
for
NulError
§
impl
Unpin
for
NulError
§
impl
UnwindSafe
for
NulError
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