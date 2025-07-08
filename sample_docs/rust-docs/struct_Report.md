Report in std::error - Rust
std
::
error
Struct
Report
Copy item path
Source
pub struct Report<E =
Box
<dyn
Error
>> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
error_reporter
#90172
)
Expand description
An error reporter that prints an error and its sources.
Report also exposes configuration options for formatting the error sources, either entirely on a
single line, or in multi-line format with each source on a new line.
Report
only requires that the wrapped error implement
Error
. It doesn’t require that the
wrapped error be
Send
,
Sync
, or
'static
.
§
Examples
#![feature(error_reporter)]
use
std::error::{Error, Report};
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
(e) =>
println!
(
"Error: {}"
, Report::new(e)),
_
=>
println!
(
"No error"
),
    }
}
This example produces the following output:
Error: SuperError is here!: SuperErrorSideKick is here!
§
Output consistency
Report prints the same output via
Display
and
Debug
, so it works well with
Result::unwrap
/
Result::expect
which print their
Err
variant via
Debug
:
ⓘ
#![feature(error_reporter)]
use
std::error::Report;

get_super_error().map_err(Report::new).unwrap();
This example produces the following output:
thread 'main' panicked at src/error.rs:34:40:
called `Result::unwrap()` on an `Err` value: SuperError is here!: SuperErrorSideKick is here!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
§
Return from
main
Report
also implements
From
for all types that implement
Error
; this when combined with
the
Debug
output means
Report
is an ideal starting place for formatting errors returned
from
main
.
ⓘ
#![feature(error_reporter)]
use
std::error::Report;
fn
main() ->
Result
<(), Report<SuperError>> {
    get_super_error()
?
;
Ok
(())
}
This example produces the following output:
Error: SuperError is here!: SuperErrorSideKick is here!
Note
:
Report
s constructed via
?
and
From
will be configured to use the single line
output format. If you want to make sure your
Report
s are pretty printed and include backtrace
you will need to manually convert and enable those flags.
ⓘ
#![feature(error_reporter)]
use
std::error::Report;
fn
main() ->
Result
<(), Report<SuperError>> {
    get_super_error()
        .map_err(Report::from)
        .map_err(|r| r.pretty(
true
).show_backtrace(
true
))
?
;
Ok
(())
}
This example produces the following output:
Error: SuperError is here!

Caused by:
      SuperErrorSideKick is here!
Implementations
§
Source
§
impl<E>
Report
<E>
where
Report
<E>:
From
<E>,
Source
pub fn
new
(error: E) ->
Report
<E>
🔬
This is a nightly-only experimental API. (
error_reporter
#90172
)
Creates a new
Report
from an input error.
Source
§
impl<E>
Report
<E>
Source
pub fn
pretty
(self, pretty:
bool
) -> Self
🔬
This is a nightly-only experimental API. (
error_reporter
#90172
)
Enable pretty-printing the report across multiple lines.
§
Examples
#![feature(error_reporter)]
use
std::error::Report;
let
error = SuperError { source: SuperErrorSideKick };
let
report = Report::new(error).pretty(
true
);
eprintln!
(
"Error: {report:?}"
);
This example produces the following output:
Error: SuperError is here!

Caused by:
      SuperErrorSideKick is here!
When there are multiple source errors the causes will be numbered in order of iteration
starting from the outermost error.
#![feature(error_reporter)]
use
std::error::Report;
let
source = SuperErrorSideKickSideKick;
let
source = SuperErrorSideKick { source };
let
error = SuperError { source };
let
report = Report::new(error).pretty(
true
);
eprintln!
(
"Error: {report:?}"
);
This example produces the following output:
Error: SuperError is here!

Caused by:
   0: SuperErrorSideKick is here!
   1: SuperErrorSideKickSideKick is here!
Source
pub fn
show_backtrace
(self, show_backtrace:
bool
) -> Self
🔬
This is a nightly-only experimental API. (
error_reporter
#90172
)
Display backtrace if available when using pretty output format.
§
Examples
Note
: Report will search for the first
Backtrace
it can find starting from the
outermost error. In this example it will display the backtrace from the second error in the
sources,
SuperErrorSideKick
.
#![feature(error_reporter)]
#![feature(error_generic_member_access)]
use
std::error::Request;
use
std::error::Report;
use
std::backtrace::Backtrace;
#[derive(Debug)]
struct
SuperErrorSideKick {
    backtrace: Backtrace,
}
impl
SuperErrorSideKick {
fn
new() -> SuperErrorSideKick {
        SuperErrorSideKick { backtrace: Backtrace::force_capture() }
    }
}
impl
Error
for
SuperErrorSideKick {
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
        request.provide_ref::<Backtrace>(
&
self
.backtrace);
    }
}
// The rest of the example is unchanged ...
let
source = SuperErrorSideKick::new();
let
error = SuperError { source };
let
report = Report::new(error).pretty(
true
).show_backtrace(
true
);
eprintln!
(
"Error: {report:?}"
);
This example produces something similar to the following output:
Error: SuperError is here!

Caused by:
      SuperErrorSideKick is here!

Stack backtrace:
   0: rust_out::main::_doctest_main_src_error_rs_1158_0::SuperErrorSideKick::new
   1: rust_out::main::_doctest_main_src_error_rs_1158_0
   2: rust_out::main
   3: core::ops::function::FnOnce::call_once
   4: std::sys::backtrace::__rust_begin_short_backtrace
   5: std::rt::lang_start::{{closure}}
   6: std::panicking::try
   7: std::rt::lang_start_internal
   8: std::rt::lang_start
   9: main
  10: __libc_start_main
  11: _start
Trait Implementations
§
Source
§
impl<E>
Debug
for
Report
<E>
where
Report
<E>:
Display
,
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
Source
§
impl<E>
Display
for
Report
<E>
where
    E:
Error
,
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
Source
§
impl<E>
From
<E> for
Report
<E>
where
    E:
Error
,
Source
§
fn
from
(error: E) -> Self
Converts to this type from the input type.
Auto Trait Implementations
§
§
impl<E>
Freeze
for
Report
<E>
where
    E:
Freeze
,
§
impl<E>
RefUnwindSafe
for
Report
<E>
where
    E:
RefUnwindSafe
,
§
impl<E>
Send
for
Report
<E>
where
    E:
Send
,
§
impl<E>
Sync
for
Report
<E>
where
    E:
Sync
,
§
impl<E>
Unpin
for
Report
<E>
where
    E:
Unpin
,
§
impl<E>
UnwindSafe
for
Report
<E>
where
    E:
UnwindSafe
,
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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