Utf8Error in std::str - Rust
std
::
str
Struct
Utf8Error
Copy item path
1.0.0
·
Source
pub struct Utf8Error {
/* private fields */
}
Expand description
Errors which can occur when attempting to interpret a sequence of
u8
as a string.
As such, the
from_utf8
family of functions and methods for both
String
s
and
&str
s make use of this error, for example.
§
Examples
This error type’s methods can be used to create functionality
similar to
String::from_utf8_lossy
without allocating heap memory:
fn
from_utf8_lossy<F>(
mut
input:
&
[u8],
mut
push: F)
where
F: FnMut(
&
str) {
loop
{
match
std::str::from_utf8(input) {
Ok
(valid) => {
                push(valid);
break
}
Err
(error) => {
let
(valid, after_valid) = input.split_at(error.valid_up_to());
unsafe
{
                    push(std::str::from_utf8_unchecked(valid))
                }
                push(
"\u{FFFD}"
);
if let
Some
(invalid_sequence_length) = error.error_len() {
                    input =
&
after_valid[invalid_sequence_length..]
                }
else
{
break
}
            }
        }
    }
}
Implementations
§
Source
§
impl
Utf8Error
1.5.0 (const: 1.63.0)
·
Source
pub const fn
valid_up_to
(&self) ->
usize
Returns the index in the given string up to which valid UTF-8 was
verified.
It is the maximum index such that
from_utf8(&input[..index])
would return
Ok(_)
.
§
Examples
Basic usage:
use
std::str;
// some invalid bytes, in a vector
let
sparkle_heart =
vec!
[
0
,
159
,
146
,
150
];
// std::str::from_utf8 returns a Utf8Error
let
error = str::from_utf8(
&
sparkle_heart).unwrap_err();
// the second byte is invalid here
assert_eq!
(
1
, error.valid_up_to());
1.20.0 (const: 1.63.0)
·
Source
pub const fn
error_len
(&self) ->
Option
<
usize
>
Provides more information about the failure:
None
: the end of the input was reached unexpectedly.
self.valid_up_to()
is 1 to 3 bytes from the end of the input.
If a byte stream (such as a file or a network socket) is being decoded incrementally,
this could be a valid
char
whose UTF-8 byte sequence is spanning multiple chunks.
Some(len)
: an unexpected byte was encountered.
The length provided is that of the invalid byte sequence
that starts at the index given by
valid_up_to()
.
Decoding should resume after that sequence
(after inserting a
U+FFFD REPLACEMENT CHARACTER
) in case of
lossy decoding.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
Utf8Error
Source
§
fn
clone
(&self) ->
Utf8Error
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
Utf8Error
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
Utf8Error
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
Utf8Error
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
Utf8Error
Source
§
fn
eq
(&self, other: &
Utf8Error
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
Utf8Error
1.0.0
·
Source
§
impl
Eq
for
Utf8Error
1.0.0
·
Source
§
impl
StructuralPartialEq
for
Utf8Error
Auto Trait Implementations
§
§
impl
Freeze
for
Utf8Error
§
impl
RefUnwindSafe
for
Utf8Error
§
impl
Send
for
Utf8Error
§
impl
Sync
for
Utf8Error
§
impl
Unpin
for
Utf8Error
§
impl
UnwindSafe
for
Utf8Error
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