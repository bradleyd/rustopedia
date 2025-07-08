Poll in std::task - Rust
std
::
task
Enum
Poll
Copy item path
1.36.0
·
Source
pub enum Poll<T> {
    Ready(T),
    Pending,
}
Expand description
Indicates whether a value is available or if the current task has been
scheduled to receive a wakeup instead.
This is returned by
Future::poll
.
Variants
§
§
1.36.0
Ready(T)
Represents that a value is immediately ready.
§
1.36.0
Pending
Represents that a value is not ready yet.
When a function returns
Pending
, the function
must
also
ensure that the current task is scheduled to be awoken when
progress can be made.
Implementations
§
Source
§
impl<T>
Poll
<T>
1.36.0
·
Source
pub fn
map
<U, F>(self, f: F) ->
Poll
<U>
where
    F:
FnOnce
(T) -> U,
Maps a
Poll<T>
to
Poll<U>
by applying a function to a contained value.
§
Examples
Converts a
Poll<
String
>
into a
Poll<
usize
>
, consuming
the original:
let
poll_some_string = Poll::Ready(String::from(
"Hello, World!"
));
// `Poll::map` takes self *by value*, consuming `poll_some_string`
let
poll_some_len = poll_some_string.map(|s| s.len());
assert_eq!
(poll_some_len, Poll::Ready(
13
));
1.36.0 (const: 1.49.0)
·
Source
pub const fn
is_ready
(&self) ->
bool
Returns
true
if the poll is a
Poll::Ready
value.
§
Examples
let
x: Poll<u32> = Poll::Ready(
2
);
assert_eq!
(x.is_ready(),
true
);
let
x: Poll<u32> = Poll::Pending;
assert_eq!
(x.is_ready(),
false
);
1.36.0 (const: 1.49.0)
·
Source
pub const fn
is_pending
(&self) ->
bool
Returns
true
if the poll is a
Pending
value.
§
Examples
let
x: Poll<u32> = Poll::Ready(
2
);
assert_eq!
(x.is_pending(),
false
);
let
x: Poll<u32> = Poll::Pending;
assert_eq!
(x.is_pending(),
true
);
Source
§
impl<T, E>
Poll
<
Result
<T, E>>
1.36.0
·
Source
pub fn
map_ok
<U, F>(self, f: F) ->
Poll
<
Result
<U, E>>
where
    F:
FnOnce
(T) -> U,
Maps a
Poll<Result<T, E>>
to
Poll<Result<U, E>>
by applying a
function to a contained
Poll::Ready(Ok)
value, leaving all other
variants untouched.
This function can be used to compose the results of two functions.
§
Examples
let
res: Poll<
Result
<u8,
_
>> = Poll::Ready(
"12"
.parse());
let
squared = res.map_ok(|n| n * n);
assert_eq!
(squared, Poll::Ready(
Ok
(
144
)));
1.36.0
·
Source
pub fn
map_err
<U, F>(self, f: F) ->
Poll
<
Result
<T, U>>
where
    F:
FnOnce
(E) -> U,
Maps a
Poll::Ready<Result<T, E>>
to
Poll::Ready<Result<T, F>>
by
applying a function to a contained
Poll::Ready(Err)
value, leaving all other
variants untouched.
This function can be used to pass through a successful result while handling
an error.
§
Examples
let
res: Poll<
Result
<u8,
_
>> = Poll::Ready(
"oops"
.parse());
let
res = res.map_err(|
_
|
0_u8
);
assert_eq!
(res, Poll::Ready(
Err
(
0
)));
Source
§
impl<T, E>
Poll
<
Option
<
Result
<T, E>>>
1.51.0
·
Source
pub fn
map_ok
<U, F>(self, f: F) ->
Poll
<
Option
<
Result
<U, E>>>
where
    F:
FnOnce
(T) -> U,
Maps a
Poll<Option<Result<T, E>>>
to
Poll<Option<Result<U, E>>>
by
applying a function to a contained
Poll::Ready(Some(Ok))
value,
leaving all other variants untouched.
This function can be used to compose the results of two functions.
§
Examples
let
res: Poll<
Option
<
Result
<u8,
_
>>> = Poll::Ready(
Some
(
"12"
.parse()));
let
squared = res.map_ok(|n| n * n);
assert_eq!
(squared, Poll::Ready(
Some
(
Ok
(
144
))));
1.51.0
·
Source
pub fn
map_err
<U, F>(self, f: F) ->
Poll
<
Option
<
Result
<T, U>>>
where
    F:
FnOnce
(E) -> U,
Maps a
Poll::Ready<Option<Result<T, E>>>
to
Poll::Ready<Option<Result<T, F>>>
by applying a function to a
contained
Poll::Ready(Some(Err))
value, leaving all other variants
untouched.
This function can be used to pass through a successful result while handling
an error.
§
Examples
let
res: Poll<
Option
<
Result
<u8,
_
>>> = Poll::Ready(
Some
(
"oops"
.parse()));
let
res = res.map_err(|
_
|
0_u8
);
assert_eq!
(res, Poll::Ready(
Some
(
Err
(
0
))));
Trait Implementations
§
1.36.0
·
Source
§
impl<T>
Clone
for
Poll
<T>
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
Poll
<T>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.36.0
·
Source
§
impl<T>
Debug
for
Poll
<T>
where
    T:
Debug
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
1.36.0
·
Source
§
impl<T>
From
<T> for
Poll
<T>
Source
§
fn
from
(t: T) ->
Poll
<T>
Moves the value into a
Poll::Ready
to make a
Poll<T>
.
§
Example
assert_eq!
(Poll::from(
true
), Poll::Ready(
true
));
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Option
<
Result
<T, F>>>
where
    F:
From
<E>,
Source
§
fn
from_residual
(x:
Result
<
Infallible
, E>) ->
Poll
<
Option
<
Result
<T, F>>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Result
<T, F>>
where
    F:
From
<E>,
Source
§
fn
from_residual
(x:
Result
<
Infallible
, E>) ->
Poll
<
Result
<T, F>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
1.36.0
·
Source
§
impl<T>
Hash
for
Poll
<T>
where
    T:
Hash
,
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.36.0
·
Source
§
impl<T>
Ord
for
Poll
<T>
where
    T:
Ord
,
Source
§
fn
cmp
(&self, other: &
Poll
<T>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.36.0
·
Source
§
impl<T>
PartialEq
for
Poll
<T>
where
    T:
PartialEq
,
Source
§
fn
eq
(&self, other: &
Poll
<T>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.36.0
·
Source
§
impl<T>
PartialOrd
for
Poll
<T>
where
    T:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
Poll
<T>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<T, E>
Try
for
Poll
<
Option
<
Result
<T, E>>>
Source
§
type
Output
=
Poll
<
Option
<T>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value produced by
?
when
not
short-circuiting.
Source
§
type
Residual
=
Result
<
Infallible
, E>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value passed to
FromResidual::from_residual
as part of
?
when short-circuiting.
Read more
Source
§
fn
from_output
(
    c: <
Poll
<
Option
<
Result
<T, E>>> as
Try
>::
Output
,
) ->
Poll
<
Option
<
Result
<T, E>>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from its
Output
type.
Read more
Source
§
fn
branch
(
    self,
) ->
ControlFlow
<<
Poll
<
Option
<
Result
<T, E>>> as
Try
>::
Residual
, <
Poll
<
Option
<
Result
<T, E>>> as
Try
>::
Output
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Used in
?
to decide whether the operator should produce a value
(because this returned
ControlFlow::Continue
)
or propagate a value back to the caller
(because this returned
ControlFlow::Break
).
Read more
Source
§
impl<T, E>
Try
for
Poll
<
Result
<T, E>>
Source
§
type
Output
=
Poll
<T>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value produced by
?
when
not
short-circuiting.
Source
§
type
Residual
=
Result
<
Infallible
, E>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value passed to
FromResidual::from_residual
as part of
?
when short-circuiting.
Read more
Source
§
fn
from_output
(c: <
Poll
<
Result
<T, E>> as
Try
>::
Output
) ->
Poll
<
Result
<T, E>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from its
Output
type.
Read more
Source
§
fn
branch
(
    self,
) ->
ControlFlow
<<
Poll
<
Result
<T, E>> as
Try
>::
Residual
, <
Poll
<
Result
<T, E>> as
Try
>::
Output
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Used in
?
to decide whether the operator should produce a value
(because this returned
ControlFlow::Continue
)
or propagate a value back to the caller
(because this returned
ControlFlow::Break
).
Read more
1.36.0
·
Source
§
impl<T>
Copy
for
Poll
<T>
where
    T:
Copy
,
1.36.0
·
Source
§
impl<T>
Eq
for
Poll
<T>
where
    T:
Eq
,
1.36.0
·
Source
§
impl<T>
StructuralPartialEq
for
Poll
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Poll
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Poll
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Poll
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Poll
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Poll
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Poll
<T>
where
    T:
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
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
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
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
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