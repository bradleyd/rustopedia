DebugStruct in std::fmt - Rust
std
::
fmt
Struct
DebugStruct
Copy item path
1.2.0
·
Source
pub struct DebugStruct<'a, 'b>
where
    'b: 'a,
{
/* private fields */
}
Expand description
A struct to help with
fmt::Debug
implementations.
This is useful when you wish to output a formatted struct as a part of your
Debug::fmt
implementation.
This can be constructed by the
Formatter::debug_struct
method.
§
Examples
use
std::fmt;
struct
Foo {
    bar: i32,
    baz: String,
}
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
        fmt.debug_struct(
"Foo"
)
           .field(
"bar"
,
&
self
.bar)
           .field(
"baz"
,
&
self
.baz)
           .finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo { bar:
10
, baz:
"Hello World"
.to_string() }),
r#"Foo { bar: 10, baz: "Hello World" }"#
,
);
Implementations
§
Source
§
impl<'a, 'b>
DebugStruct
<'a, 'b>
where
    'b: 'a,
1.2.0
·
Source
pub fn
field
(
    &mut self,
    name: &
str
,
    value: &dyn
Debug
,
) -> &mut
DebugStruct
<'a, 'b>
Adds a new field to the generated struct output.
§
Examples
use
std::fmt;
struct
Bar {
    bar: i32,
    another: String,
}
impl
fmt::Debug
for
Bar {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_struct(
"Bar"
)
           .field(
"bar"
,
&
self
.bar)
// We add `bar` field.
.field(
"another"
,
&
self
.another)
// We add `another` field.
           // We even add a field which doesn't exist (because why not?).
.field(
"nonexistent_field"
,
&
1
)
           .finish()
// We're good to go!
}
}
assert_eq!
(
format!
(
"{:?}"
, Bar { bar:
10
, another:
"Hello World"
.to_string() }),
r#"Bar { bar: 10, another: "Hello World", nonexistent_field: 1 }"#
,
);
Source
pub fn
field_with
<F>(
    &mut self,
    name: &
str
,
    value_fmt: F,
) -> &mut
DebugStruct
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
Adds a new field to the generated struct output.
This method is equivalent to
DebugStruct::field
, but formats the
value using a provided closure rather than by calling
Debug::fmt
.
1.53.0
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
Marks the struct as non-exhaustive, indicating to the reader that there are some other
fields that are not shown in the debug representation.
§
Examples
use
std::fmt;
struct
Bar {
    bar: i32,
    hidden: f32,
}
impl
fmt::Debug
for
Bar {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_struct(
"Bar"
)
           .field(
"bar"
,
&
self
.bar)
           .finish_non_exhaustive()
// Show that some other field(s) exist.
}
}
assert_eq!
(
format!
(
"{:?}"
, Bar { bar:
10
, hidden:
1.0
}),
"Bar { bar: 10, .. }"
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
Bar {
    bar: i32,
    baz: String,
}
impl
fmt::Debug
for
Bar {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_struct(
"Bar"
)
           .field(
"bar"
,
&
self
.bar)
           .field(
"baz"
,
&
self
.baz)
           .finish()
// You need to call it to "finish" the
                     // struct formatting.
}
}
assert_eq!
(
format!
(
"{:?}"
, Bar { bar:
10
, baz:
"Hello World"
.to_string() }),
r#"Bar { bar: 10, baz: "Hello World" }"#
,
);
Auto Trait Implementations
§
§
impl<'a, 'b>
Freeze
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugStruct
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