FromUtf8Error in std::string - Rust
std
::
string
Struct
FromUtf8Error
Copy item path
1.0.0
·
Source
pub struct FromUtf8Error {
/* private fields */
}
Expand description
A possible error value when converting a
String
from a UTF-8 byte vector.
This type is the error type for the
from_utf8
method on
String
. It
is designed in such a way to carefully avoid reallocations: the
into_bytes
method will give back the byte vector that was used in the
conversion attempt.
The
Utf8Error
type provided by
std::str
represents an error that may
occur when converting a slice of
u8
s to a
&str
. In this sense, it’s
an analogue to
FromUtf8Error
, and you can get one from a
FromUtf8Error
through the
utf8_error
method.
§
Examples
// some invalid bytes, in a vector
let
bytes =
vec!
[
0
,
159
];
let
value = String::from_utf8(bytes);
assert!
(value.is_err());
assert_eq!
(
vec!
[
0
,
159
], value.unwrap_err().into_bytes());
Implementations
§
Source
§
impl
FromUtf8Error
1.26.0
·
Source
pub fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Returns a slice of
u8
s bytes that were attempted to convert to a
String
.
§
Examples
// some invalid bytes, in a vector
let
bytes =
vec!
[
0
,
159
];
let
value = String::from_utf8(bytes);
assert_eq!
(
&
[
0
,
159
], value.unwrap_err().as_bytes());
Source
pub fn
into_utf8_lossy
(self) ->
String
🔬
This is a nightly-only experimental API. (
string_from_utf8_lossy_owned
#129436
)
Converts the bytes into a
String
lossily, substituting invalid UTF-8
sequences with replacement characters.
See
String::from_utf8_lossy
for more details on replacement of
invalid sequences, and
String::from_utf8_lossy_owned
for the
String
function which corresponds to this function.
§
Examples
#![feature(string_from_utf8_lossy_owned)]
// some invalid bytes
let
input: Vec<u8> =
b"Hello \xF0\x90\x80World"
.into();
let
output = String::from_utf8(input).unwrap_or_else(|e| e.into_utf8_lossy());
assert_eq!
(String::from(
"Hello �World"
), output);
1.0.0
·
Source
pub fn
into_bytes
(self) ->
Vec
<
u8
>
ⓘ
Returns the bytes that were attempted to convert to a
String
.
This method is carefully constructed to avoid allocation. It will
consume the error, moving out the bytes, so that a copy of the bytes
does not need to be made.
§
Examples
// some invalid bytes, in a vector
let
bytes =
vec!
[
0
,
159
];
let
value = String::from_utf8(bytes);
assert_eq!
(
vec!
[
0
,
159
], value.unwrap_err().into_bytes());
1.0.0
·
Source
pub fn
utf8_error
(&self) ->
Utf8Error
Fetch a
Utf8Error
to get more details about the conversion failure.
The
Utf8Error
type provided by
std::str
represents an error that may
occur when converting a slice of
u8
s to a
&str
. In this sense, it’s
an analogue to
FromUtf8Error
. See its documentation for more details
on using it.
§
Examples
// some invalid bytes, in a vector
let
bytes =
vec!
[
0
,
159
];
let
error = String::from_utf8(bytes).unwrap_err().utf8_error();
// the first byte is invalid here
assert_eq!
(
1
, error.valid_up_to());
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
FromUtf8Error
Source
§
fn
clone
(&self) ->
FromUtf8Error
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
FromUtf8Error
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
FromUtf8Error
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
FromUtf8Error
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
PartialEq
for
FromUtf8Error
Source
§
fn
eq
(&self, other: &
FromUtf8Error
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
Eq
for
FromUtf8Error
1.0.0
·
Source
§
impl
StructuralPartialEq
for
FromUtf8Error
Auto Trait Implementations
§
§
impl
Freeze
for
FromUtf8Error
§
impl
RefUnwindSafe
for
FromUtf8Error
§
impl
Send
for
FromUtf8Error
§
impl
Sync
for
FromUtf8Error
§
impl
Unpin
for
FromUtf8Error
§
impl
UnwindSafe
for
FromUtf8Error
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