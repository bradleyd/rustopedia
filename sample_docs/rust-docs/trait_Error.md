Error in std::error - Rust
std
::
error
Trait
Error
Copy item path
1.0.0
·
Source
pub trait Error:
Debug
+
Display
{
    // Provided methods
    fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)> { ... }
fn
description
(&self) -> &
str
{ ... }
fn
cause
(&self) ->
Option
<&dyn
Error
> { ... }
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>) { ... }
}
Expand description
Error
is a trait representing the basic expectations for error values,
i.e., values of type
E
in
Result<T, E>
.
Errors must describe themselves through the
Display
and
Debug
traits. Error messages are typically concise lowercase sentences without
trailing punctuation:
let
err =
"NaN"
.parse::<u32>().unwrap_err();
assert_eq!
(err.to_string(),
"invalid digit found in string"
);
Errors may provide cause information.
Error::source()
is generally
used when errors cross “abstraction boundaries”. If one module must report
an error that is caused by an error from a lower-level module, it can allow
accessing that error via
Error::source()
. This makes it possible for the
high-level module to provide its own errors while also revealing some of the
implementation for debugging.
§
Example
Implementing the
Error
trait only requires that
Debug
and
Display
are implemented too.
use
std::error::Error;
use
std::fmt;
use
std::path::PathBuf;
#[derive(Debug)]
struct
ReadConfigError {
    path: PathBuf
}
impl
fmt::Display
for
ReadConfigError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
let
path =
self
.path.display();
write!
(f,
"unable to read configuration at {path}"
)
    }
}
impl
Error
for
ReadConfigError {}
Provided Methods
§
1.30.0
·
Source
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
§
Examples
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
SuperError {
    source: SuperErrorSideKick,
}
impl
fmt::Display
for
SuperError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"SuperError is here!"
)
    }
}
impl
Error
for
SuperError {
fn
source(
&
self
) ->
Option
<
&
(
dyn
Error +
'static
)> {
Some
(
&
self
.source)
    }
}
#[derive(Debug)]
struct
SuperErrorSideKick;
impl
fmt::Display
for
SuperErrorSideKick {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"SuperErrorSideKick is here!"
)
    }
}
impl
Error
for
SuperErrorSideKick {}
fn
get_super_error() ->
Result
<(), SuperError> {
Err
(SuperError { source: SuperErrorSideKick })
}
fn
main() {
match
get_super_error() {
Err
(e) => {
println!
(
"Error: {e}"
);
println!
(
"Caused by: {}"
, e.source().unwrap());
        }
_
=>
println!
(
"No error"
),
    }
}
1.0.0
·
Source
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
if let
Err
(e) =
"xc"
.parse::<u32>() {
// Print `e` itself, no need for description().
eprintln!
(
"Error: {e}"
);
}
1.0.0
·
Source
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
Used in conjunction with
Request::provide_value
and
Request::provide_ref
to extract
references to member variables from
dyn Error
trait objects.
§
Example
#![feature(error_generic_member_access)]
use
core::fmt;
use
core::error::{request_ref, Request};
#[derive(Debug)]
enum
MyLittleTeaPot {
    Empty,
}
#[derive(Debug)]
struct
MyBacktrace {
// ...
}
impl
MyBacktrace {
fn
new() -> MyBacktrace {
// ...
}
}
#[derive(Debug)]
struct
Error {
    backtrace: MyBacktrace,
}
impl
fmt::Display
for
Error {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"Example Error"
)
    }
}
impl
std::error::Error
for
Error {
fn
provide<
'a
>(
&
'a
self
, request:
&mut
Request<
'a
>) {
        request
            .provide_ref::<MyBacktrace>(
&
self
.backtrace);
    }
}
fn
main() {
let
backtrace = MyBacktrace::new();
let
error = Error { backtrace };
let
dyn_error =
&
error
as
&
dyn
std::error::Error;
let
backtrace_ref = request_ref::<MyBacktrace>(dyn_error).unwrap();
assert!
(core::ptr::eq(
&
error.backtrace, backtrace_ref));
assert!
(request_ref::<MyLittleTeaPot>(dyn_error).is_none());
}
Implementations
§
Source
§
impl dyn
Error
1.3.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Error
+ 'static,
Returns
true
if the inner type is the same as
T
.
1.3.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Error
+ 'static,
Returns some reference to the inner value if it is of type
T
, or
None
if it isn’t.
1.3.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Error
+ 'static,
Returns some mutable reference to the inner value if it is of type
T
, or
None
if it isn’t.
Source
§
impl dyn
Error
+
Send
1.3.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
1.3.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
1.3.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
Source
§
impl dyn
Error
+
Sync
+
Send
1.3.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
1.3.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
1.3.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Error
+ 'static,
Forwards to the method defined on the type
dyn Error
.
Source
§
impl dyn
Error
Source
pub fn
sources
(&self) ->
Source
<'_>
ⓘ
🔬
This is a nightly-only experimental API. (
error_iter
#58520
)
Returns an iterator starting with the current error and continuing with
recursively calling
Error::source
.
If you want to omit the current error and only use its sources,
use
skip(1)
.
§
Examples
#![feature(error_iter)]
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
A;
#[derive(Debug)]
struct
B(
Option
<Box<
dyn
Error +
'static
>>);
impl
fmt::Display
for
A {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"A"
)
    }
}
impl
fmt::Display
for
B {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"B"
)
    }
}
impl
Error
for
A {}
impl
Error
for
B {
fn
source(
&
self
) ->
Option
<
&
(
dyn
Error +
'static
)> {
self
.
0
.as_ref().map(|e| e.as_ref())
    }
}
let
b = B(
Some
(Box::new(A)));
// let err : Box<Error> = b.into(); // or
let
err =
&
b
as
&
(
dyn
Error);
let
mut
iter = err.sources();
assert_eq!
(
"B"
.to_string(), iter.next().unwrap().to_string());
assert_eq!
(
"A"
.to_string(), iter.next().unwrap().to_string());
assert!
(iter.next().is_none());
assert!
(iter.next().is_none());
Source
§
impl dyn
Error
1.3.0
·
Source
pub fn
downcast
<T>(self:
Box
<dyn
Error
>) ->
Result
<
Box
<T>,
Box
<dyn
Error
>>
where
    T:
Error
+ 'static,
Attempts to downcast the box to a concrete type.
Source
§
impl dyn
Error
+
Send
1.3.0
·
Source
pub fn
downcast
<T>(
    self:
Box
<dyn
Error
+
Send
>,
) ->
Result
<
Box
<T>,
Box
<dyn
Error
+
Send
>>
where
    T:
Error
+ 'static,
Attempts to downcast the box to a concrete type.
Source
§
impl dyn
Error
+
Sync
+
Send
1.3.0
·
Source
pub fn
downcast
<T>(
    self:
Box
<dyn
Error
+
Sync
+
Send
>,
) ->
Result
<
Box
<T>,
Box
<dyn
Error
+
Sync
+
Send
>>
where
    T:
Error
+ 'static,
Attempts to downcast the box to a concrete type.
Trait Implementations
§
1.6.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err: &
str
) ->
Box
<dyn
Error
+ 'a>
Converts a
str
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error>::from(a_str_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err: &
str
) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
str
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
let
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+ 'a>
Converts a
Cow
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error>::from(a_cow_str_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
Cow
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_cow_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+ 'a>
where
    E:
Error
+ 'a,
Source
§
fn
from
(err: E) ->
Box
<dyn
Error
+ 'a>
Converts a type of
Error
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
AnError;
impl
fmt::Display
for
AnError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"An error"
)
    }
}
impl
Error
for
AnError {}
let
an_error = AnError;
assert!
(
0
== size_of_val(
&
an_error));
let
a_boxed_error = Box::<
dyn
Error>::from(an_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
where
    E:
Error
+
Send
+
Sync
+ 'a,
Source
§
fn
from
(err: E) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a type of
Error
+
Send
+
Sync
into a box of
dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
AnError;
impl
fmt::Display
for
AnError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"An error"
)
    }
}
impl
Error
for
AnError {}
unsafe impl
Send
for
AnError {}
unsafe impl
Sync
for
AnError {}
let
an_error = AnError;
assert!
(
0
== size_of_val(
&
an_error));
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(an_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.6.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(str_err:
String
) ->
Box
<dyn
Error
+ 'a>
Converts a
String
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error>::from(a_string_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
String
) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
String
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_string_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
Implementors
§
1.65.0
·
Source
§
impl !
Error
for &
str
1.8.0
·
Source
§
impl
Error
for
Infallible
1.0.0
·
Source
§
impl
Error
for
VarError
1.17.0
·
Source
§
impl
Error
for
FromBytesWithNulError
1.86.0
·
Source
§
impl
Error
for
GetDisjointMutError
1.15.0
·
Source
§
impl
Error
for
RecvTimeoutError
1.0.0
·
Source
§
impl
Error
for
TryRecvError
Source
§
impl
Error
for
!
Source
§
impl
Error
for
AllocError
1.28.0
·
Source
§
impl
Error
for
LayoutError
1.34.0
·
Source
§
impl
Error
for
TryFromSliceError
1.13.0
·
Source
§
impl
Error
for
BorrowError
1.13.0
·
Source
§
impl
Error
for
BorrowMutError
1.34.0
·
Source
§
impl
Error
for
CharTryFromError
1.9.0
·
Source
§
impl
Error
for
DecodeUtf16Error
1.20.0
·
Source
§
impl
Error
for
ParseCharError
1.59.0
·
Source
§
impl
Error
for
TryFromCharError
Source
§
impl
Error
for
UnorderedKeyError
1.57.0
·
Source
§
impl
Error
for
TryReserveError
1.0.0
·
Source
§
impl
Error
for
JoinPathsError
1.69.0
·
Source
§
impl
Error
for
FromBytesUntilNulError
1.58.0
·
Source
§
impl
Error
for
FromVecWithNulError
1.7.0
·
Source
§
impl
Error
for
IntoStringError
1.0.0
·
Source
§
impl
Error
for
NulError
1.11.0
·
Source
§
impl
Error
for std::fmt::
Error
1.0.0
·
Source
§
impl
Error
for std::io::
Error
1.56.0
·
Source
§
impl
Error
for
WriterPanicked
1.4.0
·
Source
§
impl
Error
for
AddrParseError
1.0.0
·
Source
§
impl
Error
for
ParseFloatError
1.0.0
·
Source
§
impl
Error
for
ParseIntError
1.34.0
·
Source
§
impl
Error
for
TryFromIntError
1.63.0
·
Source
§
impl
Error
for
InvalidHandleError
Available on
Windows
only.
1.63.0
·
Source
§
impl
Error
for
NullHandleError
Available on
Windows
only.
1.7.0
·
Source
§
impl
Error
for
StripPrefixError
Source
§
impl
Error
for
ExitStatusError
1.0.0
·
Source
§
impl
Error
for
ParseBoolError
1.0.0
·
Source
§
impl
Error
for
Utf8Error
1.0.0
·
Source
§
impl
Error
for
FromUtf8Error
1.0.0
·
Source
§
impl
Error
for
FromUtf16Error
1.0.0
·
Source
§
impl
Error
for
RecvError
1.26.0
·
Source
§
impl
Error
for
AccessError
1.8.0
·
Source
§
impl
Error
for
SystemTimeError
1.66.0
·
Source
§
impl
Error
for
TryFromFloatSecsError
Source
§
impl<'a, K, V>
Error
for std::collections::btree_map::
OccupiedError
<'a, K, V>
where
    K:
Debug
+
Ord
,
    V:
Debug
,
Source
§
impl<'a, K:
Debug
, V:
Debug
>
Error
for std::collections::hash_map::
OccupiedError
<'a, K, V>
1.51.0
·
Source
§
impl<'a, T>
Error
for
&'a T
where
    T:
Error
+ ?
Sized
,
1.8.0
·
Source
§
impl<E>
Error
for
Box
<E>
where
    E:
Error
,
1.0.0
·
Source
§
impl<T>
Error
for
TryLockError
<T>
Source
§
impl<T>
Error
for
SendTimeoutError
<T>
1.0.0
·
Source
§
impl<T>
Error
for
TrySendError
<T>
Source
§
impl<T>
Error
for
ThinBox
<T>
where
    T:
Error
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Error
for
SendError
<T>
1.52.0
·
Source
§
impl<T>
Error
for
Arc
<T>
where
    T:
Error
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Error
for
PoisonError
<T>
1.0.0
·
Source
§
impl<W:
Send
+
Debug
>
Error
for
IntoInnerError
<W>