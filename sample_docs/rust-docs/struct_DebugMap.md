DebugMap in std::fmt - Rust
std
::
fmt
Struct
DebugMap
Copy item path
1.2.0
·
Source
pub struct DebugMap<'a, 'b>
where
    'b: 'a,
{
/* private fields */
}
Expand description
A struct to help with
fmt::Debug
implementations.
This is useful when you wish to output a formatted map as a part of your
Debug::fmt
implementation.
This can be constructed by the
Formatter::debug_map
method.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map().entries(
self
.
0
.iter().map(|
&
(
ref
k,
ref
v)| (k, v))).finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"A": 10, "B": 11}"#
,
);
Implementations
§
Source
§
impl<'a, 'b>
DebugMap
<'a, 'b>
where
    'b: 'a,
1.2.0
·
Source
pub fn
entry
(
    &mut self,
    key: &dyn
Debug
,
    value: &dyn
Debug
,
) -> &mut
DebugMap
<'a, 'b>
Adds a new entry to the map output.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map()
           .entry(
&
"whole"
,
&
self
.
0
)
// We add the "whole" entry.
.finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"whole": [("A", 10), ("B", 11)]}"#
,
);
1.42.0
·
Source
pub fn
key
(&mut self, key: &dyn
Debug
) -> &mut
DebugMap
<'a, 'b>
Adds the key part of a new entry to the map output.
This method, together with
value
, is an alternative to
entry
that
can be used when the complete entry isn’t known upfront. Prefer the
entry
method when it’s possible to use.
§
Panics
key
must be called before
value
and each call to
key
must be followed
by a corresponding call to
value
. Otherwise this method will panic.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map()
           .key(
&
"whole"
).value(
&
self
.
0
)
// We add the "whole" entry.
.finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"whole": [("A", 10), ("B", 11)]}"#
,
);
Source
pub fn
key_with
<F>(&mut self, key_fmt: F) -> &mut
DebugMap
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
Adds the key part of a new entry to the map output.
This method is equivalent to
DebugMap::key
, but formats the
key using a provided closure rather than by calling
Debug::fmt
.
1.42.0
·
Source
pub fn
value
(&mut self, value: &dyn
Debug
) -> &mut
DebugMap
<'a, 'b>
Adds the value part of a new entry to the map output.
This method, together with
key
, is an alternative to
entry
that
can be used when the complete entry isn’t known upfront. Prefer the
entry
method when it’s possible to use.
§
Panics
key
must be called before
value
and each call to
key
must be followed
by a corresponding call to
value
. Otherwise this method will panic.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map()
           .key(
&
"whole"
).value(
&
self
.
0
)
// We add the "whole" entry.
.finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"whole": [("A", 10), ("B", 11)]}"#
,
);
Source
pub fn
value_with
<F>(&mut self, value_fmt: F) -> &mut
DebugMap
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
Adds the value part of a new entry to the map output.
This method is equivalent to
DebugMap::value
, but formats the
value using a provided closure rather than by calling
Debug::fmt
.
1.2.0
·
Source
pub fn
entries
<K, V, I>(&mut self, entries: I) -> &mut
DebugMap
<'a, 'b>
where
    K:
Debug
,
    V:
Debug
,
    I:
IntoIterator
<Item =
(K, V)
>,
Adds the contents of an iterator of entries to the map output.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map()
// We map our vec so each entries' first field will become
           // the "key".
.entries(
self
.
0
.iter().map(|
&
(
ref
k,
ref
v)| (k, v)))
           .finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"A": 10, "B": 11}"#
,
);
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
Marks the map as non-exhaustive, indicating to the reader that there are some other
entries that are not shown in the debug representation.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
// Print at most two elements, abbreviate the rest
let
mut
f = fmt.debug_map();
let
mut
f = f.entries(
self
.
0
.iter().take(
2
).map(|
&
(
ref
k,
ref
v)| (k, v)));
if
self
.
0
.len() >
2
{
            f.finish_non_exhaustive()
        }
else
{
            f.finish()
        }
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[
        (
"A"
.to_string(),
10
),
        (
"B"
.to_string(),
11
),
        (
"C"
.to_string(),
12
),
    ])),
r#"{"A": 10, "B": 11, ..}"#
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
Panics
key
must be called before
value
and each call to
key
must be followed
by a corresponding call to
value
. Otherwise this method will panic.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
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
        fmt.debug_map()
           .entries(
self
.
0
.iter().map(|
&
(
ref
k,
ref
v)| (k, v)))
           .finish()
// Ends the map formatting.
}
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"A": 10, "B": 11}"#
,
);
Auto Trait Implementations
§
§
impl<'a, 'b>
Freeze
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugMap
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugMap
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