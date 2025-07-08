Bound in std::range - Rust
std
::
range
Enum
Bound
Copy item path
Source
pub enum Bound<T> {
    Included(T),
    Excluded(T),
    Unbounded,
}
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
Expand description
An endpoint of a range of keys.
§
Examples
Bound
s are range endpoints:
use
std::ops::Bound::
*
;
use
std::ops::RangeBounds;
assert_eq!
((..
100
).start_bound(), Unbounded);
assert_eq!
((
1
..
12
).start_bound(), Included(
&
1
));
assert_eq!
((
1
..
12
).end_bound(), Excluded(
&
12
));
Using a tuple of
Bound
s as an argument to
BTreeMap::range
.
Note that in most cases, it’s better to use range syntax (
1..5
) instead.
use
std::collections::BTreeMap;
use
std::ops::Bound::{Excluded, Included, Unbounded};
let
mut
map = BTreeMap::new();
map.insert(
3
,
"a"
);
map.insert(
5
,
"b"
);
map.insert(
8
,
"c"
);
for
(key, value)
in
map.range((Excluded(
3
), Included(
8
))) {
println!
(
"{key}: {value}"
);
}
assert_eq!
(
Some
((
&
3
,
&
"a"
)), map.range((Unbounded, Included(
5
))).next());
Variants
§
§
Included(T)
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
An inclusive bound.
§
Excluded(T)
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
An exclusive bound.
§
Unbounded
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
An infinite endpoint. Indicates that there is no bound in this direction.
Implementations
§
Source
§
impl<T>
Bound
<T>
1.65.0
·
Source
pub fn
as_ref
(&self) ->
Bound
<
&T
>
Converts from
&Bound<T>
to
Bound<&T>
.
Source
pub fn
as_mut
(&mut self) ->
Bound
<
&mut T
>
🔬
This is a nightly-only experimental API. (
bound_as_ref
#80996
)
Converts from
&mut Bound<T>
to
Bound<&mut T>
.
1.77.0
·
Source
pub fn
map
<U, F>(self, f: F) ->
Bound
<U>
where
    F:
FnOnce
(T) -> U,
Maps a
Bound<T>
to a
Bound<U>
by applying a function to the contained value (including
both
Included
and
Excluded
), returning a
Bound
of the same kind.
§
Examples
use
std::ops::Bound::
*
;
let
bound_string = Included(
"Hello, World!"
);
assert_eq!
(bound_string.map(|s| s.len()), Included(
13
));
use
std::ops::Bound;
use
Bound::
*
;
let
unbounded_string: Bound<String> = Unbounded;
assert_eq!
(unbounded_string.map(|s| s.len()), Unbounded);
Source
§
impl<T>
Bound
<
&T
>
where
    T:
Clone
,
1.55.0
·
Source
pub fn
cloned
(self) ->
Bound
<T>
Map a
Bound<&T>
to a
Bound<T>
by cloning the contents of the bound.
§
Examples
use
std::ops::Bound::
*
;
use
std::ops::RangeBounds;
assert_eq!
((
1
..
12
).start_bound(), Included(
&
1
));
assert_eq!
((
1
..
12
).start_bound().cloned(), Included(
1
));
Trait Implementations
§
1.17.0
·
Source
§
impl<T>
Clone
for
Bound
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
Bound
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
1.17.0
·
Source
§
impl<T>
Debug
for
Bound
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
1.17.0
·
Source
§
impl<T>
Hash
for
Bound
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
1.17.0
·
Source
§
impl<T>
PartialEq
for
Bound
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
Bound
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
1.17.0
·
Source
§
impl<T>
Copy
for
Bound
<T>
where
    T:
Copy
,
1.17.0
·
Source
§
impl<T>
Eq
for
Bound
<T>
where
    T:
Eq
,
1.17.0
·
Source
§
impl<T>
StructuralPartialEq
for
Bound
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Bound
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Bound
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Bound
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Bound
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Bound
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Bound
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