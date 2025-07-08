SearchStep in std::str::pattern - Rust
std
::
str
::
pattern
Enum
SearchStep
Copy item path
Source
pub enum SearchStep {
    Match(
usize
,
usize
),
    Reject(
usize
,
usize
),
    Done,
}
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expand description
Result of calling
Searcher::next()
or
ReverseSearcher::next_back()
.
Variants
§
§
Match(
usize
,
usize
)
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expresses that a match of the pattern has been found at
haystack[a..b]
.
§
Reject(
usize
,
usize
)
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expresses that
haystack[a..b]
has been rejected as a possible match
of the pattern.
Note that there might be more than one
Reject
between two
Match
es,
there is no requirement for them to be combined into one.
§
Done
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expresses that every byte of the haystack has been visited, ending
the iteration.
Trait Implementations
§
Source
§
impl
Clone
for
SearchStep
Source
§
fn
clone
(&self) ->
SearchStep
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
Source
§
impl
Debug
for
SearchStep
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
impl
PartialEq
for
SearchStep
Source
§
fn
eq
(&self, other: &
SearchStep
) ->
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
Source
§
impl
Copy
for
SearchStep
Source
§
impl
Eq
for
SearchStep
Source
§
impl
StructuralPartialEq
for
SearchStep
Auto Trait Implementations
§
§
impl
Freeze
for
SearchStep
§
impl
RefUnwindSafe
for
SearchStep
§
impl
Send
for
SearchStep
§
impl
Sync
for
SearchStep
§
impl
Unpin
for
SearchStep
§
impl
UnwindSafe
for
SearchStep
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