DebugList in std::fmt - Rust
std
::
fmt
Struct
DebugList
Copy item path
1.2.0
·
Source
pub struct DebugList<'a, 'b>
where
    'b: 'a,
{
/* private fields */
}
Expand description
A struct to help with
fmt::Debug
implementations.
This is useful when you wish to output a formatted list of items as a part
of your
Debug::fmt
implementation.
This can be constructed by the
Formatter::debug_list
method.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>);
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
        fmt.debug_list().entries(
self
.
0
.iter()).finish()
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
10
,
11
])),
"[10, 11]"
,
);
Implementations
§
Source
§
impl<'a, 'b>
DebugList
<'a, 'b>
where
    'b: 'a,
1.2.0
·
Source
pub fn
entry
(&mut self, entry: &dyn
Debug
) -> &mut
DebugList
<'a, 'b>
Adds a new entry to the list output.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>, Vec<u32>);
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
        fmt.debug_list()
           .entry(
&
self
.
0
)
// We add the first "entry".
.entry(
&
self
.
1
)
// We add the second "entry".
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
[
10
,
11
],
vec!
[
12
,
13
])),
"[[10, 11], [12, 13]]"
,
);
Source
pub fn
entry_with
<F>(&mut self, entry_fmt: F) -> &mut
DebugList
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
Adds a new entry to the list output.
This method is equivalent to
DebugList::entry
, but formats the
entry using a provided closure rather than by calling
Debug::fmt
.
1.2.0
·
Source
pub fn
entries
<D, I>(&mut self, entries: I) -> &mut
DebugList
<'a, 'b>
where
    D:
Debug
,
    I:
IntoIterator
<Item = D>,
Adds the contents of an iterator of entries to the list output.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>, Vec<u32>);
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
        fmt.debug_list()
           .entries(
self
.
0
.iter())
           .entries(
self
.
1
.iter())
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
[
10
,
11
],
vec!
[
12
,
13
])),
"[10, 11, 12, 13]"
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
Marks the list as non-exhaustive, indicating to the reader that there are some other
elements that are not shown in the debug representation.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>);
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
f = fmt.debug_list();
let
mut
f = f.entries(
self
.
0
.iter().take(
2
));
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
1
,
2
,
3
,
4
])),
"[1, 2, ..]"
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
Foo(Vec<i32>);
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
        fmt.debug_list()
           .entries(
self
.
0
.iter())
           .finish()
// Ends the list formatting.
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
10
,
11
])),
"[10, 11]"
,
);
Auto Trait Implementations
§
§
impl<'a, 'b>
Freeze
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugList
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugList
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