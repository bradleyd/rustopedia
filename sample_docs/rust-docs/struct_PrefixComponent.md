PrefixComponent in std::path - Rust
std
::
path
Struct
PrefixComponent
Copy item path
1.0.0
·
Source
pub struct PrefixComponent<'a> {
/* private fields */
}
Expand description
A structure wrapping a Windows path prefix as well as its unparsed string
representation.
In addition to the parsed
Prefix
information returned by
kind
,
PrefixComponent
also holds the raw and unparsed
OsStr
slice,
returned by
as_os_str
.
Instances of this
struct
can be obtained by matching against the
Prefix
variant
on
Component
.
Does not occur on Unix.
§
Examples
use
std::path::{Component, Path, Prefix};
use
std::ffi::OsStr;
let
path = Path::new(
r"c:\you\later\"
);
match
path.components().next().unwrap() {
    Component::Prefix(prefix_component) => {
assert_eq!
(Prefix::Disk(
b'C'
), prefix_component.kind());
assert_eq!
(OsStr::new(
"c:"
), prefix_component.as_os_str());
    }
_
=>
unreachable!
(),
}
Implementations
§
Source
§
impl<'a>
PrefixComponent
<'a>
1.0.0
·
Source
pub fn
kind
(&self) ->
Prefix
<'a>
Returns the parsed prefix data.
See
Prefix
’s documentation for more information on the different
kinds of prefixes.
1.0.0
·
Source
pub fn
as_os_str
(&self) -> &'a
OsStr
Returns the raw
OsStr
slice for this prefix.
Trait Implementations
§
1.0.0
·
Source
§
impl<'a>
Clone
for
PrefixComponent
<'a>
Source
§
fn
clone
(&self) ->
PrefixComponent
<'a>
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
1.0.0
·
Source
§
impl<'a>
Debug
for
PrefixComponent
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
1.0.0
·
Source
§
impl
Hash
for
PrefixComponent
<'_>
Source
§
fn
hash
<H:
Hasher
>(&self, h:
&mut H
)
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
1.0.0
·
Source
§
impl
Ord
for
PrefixComponent
<'_>
Source
§
fn
cmp
(&self, other: &Self) ->
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
1.0.0
·
Source
§
impl<'a>
PartialEq
for
PrefixComponent
<'a>
Source
§
fn
eq
(&self, other: &
PrefixComponent
<'a>) ->
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
1.0.0
·
Source
§
impl<'a>
PartialOrd
for
PrefixComponent
<'a>
Source
§
fn
partial_cmp
(&self, other: &
PrefixComponent
<'a>) ->
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
1.0.0
·
Source
§
impl<'a>
Copy
for
PrefixComponent
<'a>
1.0.0
·
Source
§
impl<'a>
Eq
for
PrefixComponent
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
PrefixComponent
<'a>
§
impl<'a>
RefUnwindSafe
for
PrefixComponent
<'a>
§
impl<'a>
Send
for
PrefixComponent
<'a>
§
impl<'a>
Sync
for
PrefixComponent
<'a>
§
impl<'a>
Unpin
for
PrefixComponent
<'a>
§
impl<'a>
UnwindSafe
for
PrefixComponent
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