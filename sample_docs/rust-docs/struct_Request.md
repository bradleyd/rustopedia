Request in std::error - Rust
std
::
error
Struct
Request
Copy item path
Source
pub struct Request<'a>(
/* private fields */
);
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Expand description
Request
supports generic, type-driven access to data. Its use is currently restricted to the
standard library in cases where trait authors wish to allow trait implementors to share generic
information across trait boundaries. The motivating and prototypical use case is
core::error::Error
which would otherwise require a method per concrete type (eg.
std::backtrace::Backtrace
instance that implementors want to expose to users).
§
Data flow
To describe the intended data flow for Request objects, let’s consider two conceptual users
separated by API boundaries:
Consumer - the consumer requests objects using a Request instance; eg a crate that offers
fancy
Error
/
Result
reporting to users wants to request a Backtrace from a given
dyn Error
.
Producer - the producer provides objects when requested via Request; eg. a library with an
an
Error
implementation that automatically captures backtraces at the time instances are
created.
The consumer only needs to know where to submit their request and are expected to handle the
request not being fulfilled by the use of
Option<T>
in the responses offered by the producer.
A Producer initializes the value of one of its fields of a specific type. (or is otherwise
prepared to generate a value requested). eg,
backtrace::Backtrace
or
std::backtrace::Backtrace
A Consumer requests an object of a specific type (say
std::backtrace::Backtrace
). In the
case of a
dyn Error
trait object (the Producer), there are functions called
request_ref
and
request_value
to simplify obtaining an
Option<T>
for a given type.
The Producer, when requested, populates the given Request object which is given as a mutable
reference.
The Consumer extracts a value or reference to the requested type from the
Request
object
wrapped in an
Option<T>
; in the case of
dyn Error
the aforementioned
request_ref
and
request_value
methods mean that
dyn Error
users don’t have to deal with the
Request
type at
all (but
Error
implementors do). The
None
case of the
Option
suggests only that the
Producer cannot currently offer an instance of the requested type, not it can’t or never will.
§
Examples
The best way to demonstrate this is using an example implementation of
Error
’s
provide
trait
method:
#![feature(error_generic_member_access)]
use
core::fmt;
use
core::error::Request;
use
core::error::request_ref;
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
impl<'a>
Request
<'a>
Source
pub fn
provide_value
<T>(&mut self, value: T) -> &mut
Request
<'a>
where
    T: 'static,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides a value or other type with only static lifetimes.
§
Examples
Provides an
u8
.
#![feature(error_generic_member_access)]
use
core::error::Request;
#[derive(Debug)]
struct
SomeConcreteType { field: u8 }
impl
std::fmt::Display
for
SomeConcreteType {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"{} failed"
,
self
.field)
    }
}
impl
std::error::Error
for
SomeConcreteType {
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
        request.provide_value::<u8>(
self
.field);
    }
}
Source
pub fn
provide_value_with
<T>(
    &mut self,
    fulfil: impl
FnOnce
() -> T,
) -> &mut
Request
<'a>
where
    T: 'static,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides a value or other type with only static lifetimes computed using a closure.
§
Examples
Provides a
String
by cloning.
#![feature(error_generic_member_access)]
use
core::error::Request;
#[derive(Debug)]
struct
SomeConcreteType { field: String }
impl
std::fmt::Display
for
SomeConcreteType {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"{} failed"
,
self
.field)
    }
}
impl
std::error::Error
for
SomeConcreteType {
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
        request.provide_value_with::<String>(||
self
.field.clone());
    }
}
Source
pub fn
provide_ref
<T>(&mut self, value:
&'a T
) -> &mut
Request
<'a>
where
    T: 'static + ?
Sized
,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides a reference. The referee type must be bounded by
'static
,
but may be unsized.
§
Examples
Provides a reference to a field as a
&str
.
#![feature(error_generic_member_access)]
use
core::error::Request;
#[derive(Debug)]
struct
SomeConcreteType { field: String }
impl
std::fmt::Display
for
SomeConcreteType {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"{} failed"
,
self
.field)
    }
}
impl
std::error::Error
for
SomeConcreteType {
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
        request.provide_ref::<str>(
&
self
.field);
    }
}
Source
pub fn
provide_ref_with
<T>(
    &mut self,
    fulfil: impl
FnOnce
() ->
&'a T
,
) -> &mut
Request
<'a>
where
    T: 'static + ?
Sized
,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides a reference computed using a closure. The referee type
must be bounded by
'static
, but may be unsized.
§
Examples
Provides a reference to a field as a
&str
.
#![feature(error_generic_member_access)]
use
core::error::Request;
#[derive(Debug)]
struct
SomeConcreteType { business: String, party: String }
fn
today_is_a_weekday() -> bool {
true
}
impl
std::fmt::Display
for
SomeConcreteType {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"{} failed"
,
self
.business)
    }
}
impl
std::error::Error
for
SomeConcreteType {
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
        request.provide_ref_with::<str>(|| {
if
today_is_a_weekday() {
&
self
.business
            }
else
{
&
self
.party
            }
        });
    }
}
Source
pub fn
would_be_satisfied_by_value_of
<T>(&self) ->
bool
where
    T: 'static,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Checks if the
Request
would be satisfied if provided with a
value of the specified type. If the type does not match or has
already been provided, returns false.
§
Examples
Checks if a
u8
still needs to be provided and then provides
it.
#![feature(error_generic_member_access)]
use
core::error::Request;
use
core::error::request_value;
#[derive(Debug)]
struct
Parent(
Option
<u8>);
impl
std::fmt::Display
for
Parent {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"a parent failed"
)
    }
}
impl
std::error::Error
for
Parent {
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
if let
Some
(v) =
self
.
0
{
            request.provide_value::<u8>(v);
        }
    }
}
#[derive(Debug)]
struct
Child {
    parent: Parent,
}
impl
Child {
// Pretend that this takes a lot of resources to evaluate.
fn
an_expensive_computation(
&
self
) ->
Option
<u8> {
Some
(
99
)
    }
}
impl
std::fmt::Display
for
Child {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"child failed: \n  because of parent: {}"
,
self
.parent)
    }
}
impl
std::error::Error
for
Child {
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
// In general, we don't know if this call will provide
        // an `u8` value or not...
self
.parent.provide(request);
// ...so we check to see if the `u8` is needed before
        // we run our expensive computation.
if
request.would_be_satisfied_by_value_of::<u8>() {
if let
Some
(v) =
self
.an_expensive_computation() {
                request.provide_value::<u8>(v);
            }
        }
// The request will be satisfied now, regardless of if
        // the parent provided the value or we did.
assert!
(!request.would_be_satisfied_by_value_of::<u8>());
    }
}
let
parent = Parent(
Some
(
42
));
let
child = Child { parent };
assert_eq!
(
Some
(
42
), request_value::<u8>(
&
child));
let
parent = Parent(
None
);
let
child = Child { parent };
assert_eq!
(
Some
(
99
), request_value::<u8>(
&
child));
Source
pub fn
would_be_satisfied_by_ref_of
<T>(&self) ->
bool
where
    T: 'static + ?
Sized
,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Checks if the
Request
would be satisfied if provided with a
reference to a value of the specified type.
If the type does not match or has already been provided, returns false.
§
Examples
Checks if a
&str
still needs to be provided and then provides
it.
#![feature(error_generic_member_access)]
use
core::error::Request;
use
core::error::request_ref;
#[derive(Debug)]
struct
Parent(
Option
<String>);
impl
std::fmt::Display
for
Parent {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"a parent failed"
)
    }
}
impl
std::error::Error
for
Parent {
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
if let
Some
(v) =
&
self
.
0
{
            request.provide_ref::<str>(v);
        }
    }
}
#[derive(Debug)]
struct
Child {
    parent: Parent,
    name: String,
}
impl
Child {
// Pretend that this takes a lot of resources to evaluate.
fn
an_expensive_computation(
&
self
) ->
Option
<
&
str> {
Some
(
&
self
.name)
    }
}
impl
std::fmt::Display
for
Child {
fn
fmt(
&
self
, f:
&mut
std::fmt::Formatter<
'_
>) -> std::fmt::Result {
write!
(f,
"{} failed: \n  {}"
,
self
.name,
self
.parent)
    }
}
impl
std::error::Error
for
Child {
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
// In general, we don't know if this call will provide
        // a `str` reference or not...
self
.parent.provide(request);
// ...so we check to see if the `&str` is needed before
        // we run our expensive computation.
if
request.would_be_satisfied_by_ref_of::<str>() {
if let
Some
(v) =
self
.an_expensive_computation() {
                request.provide_ref::<str>(v);
            }
        }
// The request will be satisfied now, regardless of if
        // the parent provided the reference or we did.
assert!
(!request.would_be_satisfied_by_ref_of::<str>());
    }
}
let
parent = Parent(
Some
(
"parent"
.into()));
let
child = Child { parent, name:
"child"
.into() };
assert_eq!
(
Some
(
"parent"
), request_ref::<str>(
&
child));
let
parent = Parent(
None
);
let
child = Child { parent, name:
"child"
.into() };
assert_eq!
(
Some
(
"child"
), request_ref::<str>(
&
child));
Trait Implementations
§
Source
§
impl<'a>
Debug
for
Request
<'a>
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
Auto Trait Implementations
§
§
impl<'a> !
Freeze
for
Request
<'a>
§
impl<'a> !
RefUnwindSafe
for
Request
<'a>
§
impl<'a> !
Send
for
Request
<'a>
§
impl<'a> !
Sized
for
Request
<'a>
§
impl<'a> !
Sync
for
Request
<'a>
§
impl<'a> !
Unpin
for
Request
<'a>
§
impl<'a> !
UnwindSafe
for
Request
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