Exclusive in std::sync - Rust
std
::
sync
Struct
Exclusive
Copy item path
Source
pub struct Exclusive<T>
where
    T: ?
Sized
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Expand description
Exclusive
provides only
mutable
access, also referred to as
exclusive
access to the underlying value. It provides no
immutable
, or
shared
access to the underlying value.
While this may seem not very useful, it allows
Exclusive
to
unconditionally
implement
Sync
. Indeed, the safety requirements of
Sync
state that for
Exclusive
to be
Sync
, it must be sound to
share
across threads, that is, it must be sound
for
&Exclusive
to cross thread boundaries. By design, a
&Exclusive
has no API
whatsoever, making it useless, thus harmless, thus memory safe.
Certain constructs like
Future
s can only be used with
exclusive
access,
and are often
Send
but not
Sync
, so
Exclusive
can be used as hint to the
Rust compiler that something is
Sync
in practice.
§
Examples
Using a non-
Sync
future prevents the wrapping struct from being
Sync
ⓘ
use
core::cell::Cell;
async fn
other() {}
fn
assert_sync<T: Sync>(t: T) {}
struct
State<F> {
    future: F
}

assert_sync(State {
    future:
async
{
let
cell = Cell::new(
1
);
let
cell_ref =
&
cell;
        other().
await
;
let
value = cell_ref.get();
    }
});
Exclusive
ensures the struct is
Sync
without stripping the future of its
functionality.
#![feature(exclusive_wrapper)]
use
core::cell::Cell;
use
core::sync::Exclusive;
async fn
other() {}
fn
assert_sync<T: Sync>(t: T) {}
struct
State<F> {
    future: Exclusive<F>
}

assert_sync(State {
    future: Exclusive::new(
async
{
let
cell = Cell::new(
1
);
let
cell_ref =
&
cell;
        other().
await
;
let
value = cell_ref.get();
    })
});
§
Parallels with a mutex
In some sense,
Exclusive
can be thought of as a
compile-time
version of
a mutex, as the borrow-checker guarantees that only one
&mut
can exist
for any value. This is a parallel with the fact that
&
and
&mut
references together can be thought of as a
compile-time
version of a read-write lock.
Implementations
§
Source
§
impl<T>
Exclusive
<T>
Source
pub const fn
new
(t: T) ->
Exclusive
<T>
ⓘ
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Wrap a value in an
Exclusive
Source
pub const fn
into_inner
(self) -> T
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Unwrap the value contained in the
Exclusive
Source
§
impl<T>
Exclusive
<T>
where
    T: ?
Sized
,
Source
pub const fn
get_mut
(&mut self) ->
&mut T
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Gets exclusive access to the underlying value.
Source
pub const fn
get_pin_mut
(self:
Pin
<&mut
Exclusive
<T>>) ->
Pin
<
&mut T
>
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Gets pinned exclusive access to the underlying value.
Exclusive
is considered to
structurally pin
the underlying
value, which means
unpinned
Exclusive
s can produce
unpinned
access to the underlying value, but
pinned
Exclusive
s only
produce
pinned
access to the underlying value.
Source
pub const fn
from_mut
(r:
&mut T
) -> &mut
Exclusive
<T>
ⓘ
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Build a
mutable
reference to an
Exclusive<T>
from
a
mutable
reference to a
T
. This allows you to skip
building an
Exclusive
with
Exclusive::new
.
Source
pub const fn
from_pin_mut
(r:
Pin
<
&mut T
>) ->
Pin
<&mut
Exclusive
<T>>
🔬
This is a nightly-only experimental API. (
exclusive_wrapper
#98407
)
Build a
pinned mutable
reference to an
Exclusive<T>
from
a
pinned mutable
reference to a
T
. This allows you to skip
building an
Exclusive
with
Exclusive::new
.
Trait Implementations
§
Source
§
impl<R, G>
Coroutine
<R> for
Exclusive
<G>
where
    G:
Coroutine
<R> + ?
Sized
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
Read more
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
Read more
Source
§
fn
resume
(
    self:
Pin
<&mut
Exclusive
<G>>,
    arg: R,
) ->
CoroutineState
<<
Exclusive
<G> as
Coroutine
<R>>::
Yield
, <
Exclusive
<G> as
Coroutine
<R>>::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
Read more
Source
§
impl<T>
Debug
for
Exclusive
<T>
where
    T: ?
Sized
,
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
Source
§
impl<T>
Default
for
Exclusive
<T>
where
    T:
Default
+ ?
Sized
,
Source
§
fn
default
() ->
Exclusive
<T>
ⓘ
Returns the “default value” for a type.
Read more
Source
§
impl<F, Args>
FnMut
<Args> for
Exclusive
<F>
where
    F:
FnMut
<Args>,
    Args:
Tuple
,
Source
§
extern "rust-call" fn
call_mut
(
    &mut self,
    args: Args,
) -> <
Exclusive
<F> as
FnOnce
<Args>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
Source
§
impl<F, Args>
FnOnce
<Args> for
Exclusive
<F>
where
    F:
FnOnce
<Args>,
    Args:
Tuple
,
Source
§
type
Output
= <F as
FnOnce
<Args>>::
Output
The returned type after the call operator is used.
Source
§
extern "rust-call" fn
call_once
(
    self,
    args: Args,
) -> <
Exclusive
<F> as
FnOnce
<Args>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
Source
§
impl<T>
From
<T> for
Exclusive
<T>
Source
§
fn
from
(t: T) ->
Exclusive
<T>
ⓘ
Converts to this type from the input type.
Source
§
impl<T>
Future
for
Exclusive
<T>
where
    T:
Future
+ ?
Sized
,
Source
§
type
Output
= <T as
Future
>::
Output
The type of value produced on completion.
Source
§
fn
poll
(
    self:
Pin
<&mut
Exclusive
<T>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<<
Exclusive
<T> as
Future
>::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
Source
§
impl<T>
Sync
for
Exclusive
<T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Exclusive
<T>
where
    T:
Freeze
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
Exclusive
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
Send
for
Exclusive
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Unpin
for
Exclusive
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
Exclusive
<T>
where
    T:
UnwindSafe
+ ?
Sized
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
impl<F>
IntoFuture
for F
where
    F:
Future
,
Source
§
type
Output
= <F as
Future
>::
Output
The output that the future will produce on completion.
Source
§
type
IntoFuture
= F
Which kind of future are we turning this into?
Source
§
fn
into_future
(self) -> <F as
IntoFuture
>::
IntoFuture
Creates a future from a value.
Read more
Source
§
impl<F>
Pattern
for F
where
    F:
FnMut
(
char
) ->
bool
,
Source
§
type
Searcher
<'a> =
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
<'a>(self, haystack: &'a
str
) ->
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
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