PanicHookInfo in std::panic - Rust
std
::
panic
Struct
PanicHookInfo
Copy item path
1.81.0
·
Source
pub struct PanicHookInfo<'a> {
/* private fields */
}
Expand description
A struct providing information about a panic.
PanicHookInfo
structure is passed to a panic hook set by the
set_hook
function.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
println!
(
"panic occurred: {panic_info}"
);
}));
panic!
(
"critical system failure"
);
Implementations
§
Source
§
impl<'a>
PanicHookInfo
<'a>
1.10.0
·
Source
pub fn
payload
(&self) -> &(dyn
Any
+
Send
)
Returns the payload associated with the panic.
This will commonly, but not always, be a
&'static str
or
String
.
A invocation of the
panic!()
macro in Rust 2021 or later will always result in a
panic payload of type
&'static str
or
String
.
Only an invocation of
panic_any
(or, in Rust 2018 and earlier,
panic!(x)
where
x
is something other than a string)
can result in a panic payload other than a
&'static str
or
String
.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(s) = panic_info.payload().downcast_ref::<
&
str>() {
println!
(
"panic occurred: {s:?}"
);
    }
else if let
Some
(s) = panic_info.payload().downcast_ref::<String>() {
println!
(
"panic occurred: {s:?}"
);
    }
else
{
println!
(
"panic occurred"
);
    }
}));
panic!
(
"Normal panic"
);
Source
pub fn
payload_as_str
(&self) ->
Option
<&
str
>
🔬
This is a nightly-only experimental API. (
panic_payload_as_str
#125175
)
Returns the payload associated with the panic, if it is a string.
This returns the payload if it is of type
&'static str
or
String
.
A invocation of the
panic!()
macro in Rust 2021 or later will always result in a
panic payload where
payload_as_str
returns
Some
.
Only an invocation of
panic_any
(or, in Rust 2018 and earlier,
panic!(x)
where
x
is something other than a string)
can result in a panic payload where
payload_as_str
returns
None
.
§
Example
ⓘ
#![feature(panic_payload_as_str)]
std::panic::set_hook(Box::new(|panic_info| {
if let
Some
(s) = panic_info.payload_as_str() {
println!
(
"panic occurred: {s:?}"
);
    }
else
{
println!
(
"panic occurred"
);
    }
}));
panic!
(
"Normal panic"
);
1.10.0
·
Source
pub fn
location
(&self) ->
Option
<&
Location
<'_>>
Returns information about the location from which the panic originated,
if available.
This method will currently always return
Some
, but this may change
in future versions.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(location) = panic_info.location() {
println!
(
"panic occurred in file '{}' at line {}"
,
            location.file(),
            location.line(),
        );
    }
else
{
println!
(
"panic occurred but can't get location information..."
);
    }
}));
panic!
(
"Normal panic"
);
Source
pub fn
can_unwind
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
panic_can_unwind
#92988
)
Returns whether the panic handler is allowed to unwind the stack from
the point where the panic occurred.
This is true for most kinds of panics with the exception of panics
caused by trying to unwind out of a
Drop
implementation or a function
whose ABI does not support unwinding.
It is safe for a panic handler to unwind even when this function returns
false, however this will simply cause the panic handler to be called
again.
Trait Implementations
§
1.81.0
·
Source
§
impl<'a>
Debug
for
PanicHookInfo
<'a>
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
1.26.0
·
Source
§
impl
Display
for
PanicHookInfo
<'_>
Source
§
fn
fmt
(&self, formatter: &mut
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
PanicHookInfo
<'a>
§
impl<'a> !
RefUnwindSafe
for
PanicHookInfo
<'a>
§
impl<'a> !
Send
for
PanicHookInfo
<'a>
§
impl<'a> !
Sync
for
PanicHookInfo
<'a>
§
impl<'a>
Unpin
for
PanicHookInfo
<'a>
§
impl<'a> !
UnwindSafe
for
PanicHookInfo
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