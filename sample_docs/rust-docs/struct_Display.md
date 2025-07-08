Display in std::ffi::os_str - Rust
std
::
ffi
::
os_str
Struct
Display
Copy item path
1.87.0
·
Source
pub struct Display<'a> {
/* private fields */
}
Expand description
Helper struct for safely printing an
OsStr
with
format!
and
{}
.
An
OsStr
might contain non-Unicode data. This
struct
implements the
Display
trait in a way that mitigates that. It is created by the
display
method on
OsStr
. This may perform lossy
conversion, depending on the platform. If you would like an implementation
which escapes the
OsStr
please use
Debug
instead.
§
Examples
use
std::ffi::OsStr;
let
s = OsStr::new(
"Hello, world!"
);
println!
(
"{}"
, s.display());
Trait Implementations
§
1.87.0
·
Source
§
impl
Debug
for
Display
<'_>
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.87.0
·
Source
§
impl
Display
for
Display
<'_>
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Display
<'a>
§
impl<'a>
RefUnwindSafe
for
Display
<'a>
§
impl<'a>
Send
for
Display
<'a>
§
impl<'a>
Sync
for
Display
<'a>
§
impl<'a>
Unpin
for
Display
<'a>
§
impl<'a>
UnwindSafe
for
Display
<'a>
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