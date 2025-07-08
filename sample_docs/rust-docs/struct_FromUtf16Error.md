FromUtf16Error in std::string - Rust
std
::
string
Struct
FromUtf16Error
Copy item path
1.0.0
·
Source
pub struct FromUtf16Error(
/* private fields */
);
Expand description
A possible error value when converting a
String
from a UTF-16 byte slice.
This type is the error type for the
from_utf16
method on
String
.
§
Examples
// 𝄞mu<invalid>ic
let
v =
&
[
0xD834
,
0xDD1E
,
0x006d
,
0x0075
,
0xD800
,
0x0069
,
0x0063
];
assert!
(String::from_utf16(v).is_err());
Trait Implementations
§
1.0.0
·
Source
§
impl
Debug
for
FromUtf16Error
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
FromUtf16Error
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
FromUtf16Error
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
Auto Trait Implementations
§
§
impl
Freeze
for
FromUtf16Error
§
impl
RefUnwindSafe
for
FromUtf16Error
§
impl
Send
for
FromUtf16Error
§
impl
Sync
for
FromUtf16Error
§
impl
Unpin
for
FromUtf16Error
§
impl
UnwindSafe
for
FromUtf16Error
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