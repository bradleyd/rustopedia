DebugTuple in std::fmt - Rust
std
::
fmt
Struct
DebugTuple
Copy item path
1.2.0
·
Source
pub struct DebugTuple<'a, 'b>
where
    'b: 'a,
{
/* private fields */
}
Expand description
A struct to help with
fmt::Debug
implementations.
This is useful when you wish to output a formatted tuple as a part of your
Debug::fmt
implementation.
This can be constructed by the
Formatter::debug_tuple
method.
§
Examples
use
std::fmt;
struct
Foo(i32, String);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_tuple(
"Foo"
)
           .field(
&
self
.
0
)
           .field(
&
self
.
1
)
           .finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
10
,
"Hello World"
.to_string())),
r#"Foo(10, "Hello World")"#
,
);
Implementations
§
Source
§
impl<'a, 'b>
DebugTuple
<'a, 'b>
where
    'b: 'a,
1.2.0
·
Source
pub fn
field
(&mut self, value: &dyn
Debug
) -> &mut
DebugTuple
<'a, 'b>
Adds a new field to the generated tuple struct output.
§
Examples
use
std::fmt;
struct
Foo(i32, String);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_tuple(
"Foo"
)
           .field(
&
self
.
0
)
// We add the first field.
.field(
&
self
.
1
)
// We add the second field.
.finish()
// We're good to go!
}
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
10
,
"Hello World"
.to_string())),
r#"Foo(10, "Hello World")"#
,
);
Source
pub fn
field_with
<F>(&mut self, value_fmt: F) -> &mut
DebugTuple
<'a, 'b>
where
    F:
FnOnce
(&mut
Formatter
<'_>) ->
Result
<
()
,
Error
>,
🔬
This is a nightly-only experimental API. (
debug_closure_helpers
#117729
)
Adds a new field to the generated tuple struct output.
This method is equivalent to
DebugTuple::field
, but formats the
value using a provided closure rather than by calling
Debug::fmt
.
1.83.0
·
Source
pub fn
finish_non_exhaustive
(&mut self) ->
Result
<
()
,
Error
>
Marks the tuple struct as non-exhaustive, indicating to the reader that there are some
other fields that are not shown in the debug representation.
§
Examples
use
std::fmt;
struct
Foo(i32, String);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_tuple(
"Foo"
)
           .field(
&
self
.
0
)
           .finish_non_exhaustive()
// Show that some other field(s) exist.
}
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
10
,
"secret!"
.to_owned())),
"Foo(10, ..)"
,
);
1.2.0
·
Source
pub fn
finish
(&mut self) ->
Result
<
()
,
Error
>
Finishes output and returns any error encountered.
§
Examples
use
std::fmt;
struct
Foo(i32, String);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_tuple(
"Foo"
)
           .field(
&
self
.
0
)
           .field(
&
self
.
1
)
           .finish()
// You need to call it to "finish" the
                     // tuple formatting.
}
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
10
,
"Hello World"
.to_string())),
r#"Foo(10, "Hello World")"#
,
);
Auto Trait Implementations
§
§
impl<'a, 'b>
Freeze
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugTuple
<'a, 'b>
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