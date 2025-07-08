CharPredicateSearcher in std::str::pattern - Rust
std
::
str
::
pattern
Struct
CharPredicateSearcher
Copy item path
Source
pub struct CharPredicateSearcher<'a, F>(
/* private fields */
)
where
    F:
FnMut
(
char
) ->
bool
;
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expand description
Associated type for
<F as Pattern>::Searcher<'a>
.
Trait Implementations
§
Source
§
impl<'a, F>
Clone
for
CharPredicateSearcher
<'a, F>
where
    F:
Clone
+
FnMut
(
char
) ->
bool
,
Source
§
fn
clone
(&self) ->
CharPredicateSearcher
<'a, F>
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
impl<F>
Debug
for
CharPredicateSearcher
<'_, F>
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
impl<'a, F>
ReverseSearcher
<'a> for
CharPredicateSearcher
<'a, F>
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
fn
next_back
(&mut self) ->
SearchStep
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Performs the next search step starting from the back.
Read more
Source
§
fn
next_match_back
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Match
result.
See
next_back()
.
Source
§
fn
next_reject_back
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Reject
result.
See
next_back()
.
Source
§
impl<'a, F>
Searcher
<'a> for
CharPredicateSearcher
<'a, F>
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
fn
haystack
(&self) -> &'a
str
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Getter for the underlying string to be searched in
Read more
Source
§
fn
next
(&mut self) ->
SearchStep
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Performs the next search step starting from the front.
Read more
Source
§
fn
next_match
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Match
result. See
next()
.
Read more
Source
§
fn
next_reject
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Reject
result. See
next()
and
next_match()
.
Read more
Source
§
impl<'a, F>
DoubleEndedSearcher
<'a> for
CharPredicateSearcher
<'a, F>
where
    F:
FnMut
(
char
) ->
bool
,
Auto Trait Implementations
§
§
impl<'a, F>
Freeze
for
CharPredicateSearcher
<'a, F>
where
    F:
Freeze
,
§
impl<'a, F>
RefUnwindSafe
for
CharPredicateSearcher
<'a, F>
where
    F:
RefUnwindSafe
,
§
impl<'a, F>
Send
for
CharPredicateSearcher
<'a, F>
where
    F:
Send
,
§
impl<'a, F>
Sync
for
CharPredicateSearcher
<'a, F>
where
    F:
Sync
,
§
impl<'a, F>
Unpin
for
CharPredicateSearcher
<'a, F>
where
    F:
Unpin
,
§
impl<'a, F>
UnwindSafe
for
CharPredicateSearcher
<'a, F>
where
    F:
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