Component in std::path - Rust
std
::
path
Enum
Component
Copy item path
1.0.0
·
Source
pub enum Component<'a> {
    Prefix(
PrefixComponent
<'a>),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a
OsStr
),
}
Expand description
A single component of a path.
A
Component
roughly corresponds to a substring between path separators
(
/
or
\
).
This
enum
is created by iterating over
Components
, which in turn is
created by the
components
method on
Path
.
§
Examples
use
std::path::{Component, Path};
let
path = Path::new(
"/tmp/foo/bar.txt"
);
let
components = path.components().collect::<Vec<
_
>>();
assert_eq!
(
&
components,
&
[
    Component::RootDir,
    Component::Normal(
"tmp"
.as_ref()),
    Component::Normal(
"foo"
.as_ref()),
    Component::Normal(
"bar.txt"
.as_ref()),
]);
Variants
§
§
1.0.0
Prefix(
PrefixComponent
<'a>)
A Windows path prefix, e.g.,
C:
or
\\server\share
.
There is a large variety of prefix types, see
Prefix
’s documentation
for more.
Does not occur on Unix.
§
1.0.0
RootDir
The root directory component, appears after any prefix and before anything else.
It represents a separator that designates that a path starts from root.
§
1.0.0
CurDir
A reference to the current directory, i.e.,
.
.
§
1.0.0
ParentDir
A reference to the parent directory, i.e.,
..
.
§
1.0.0
Normal(&'a
OsStr
)
A normal component, e.g.,
a
and
b
in
a/b
.
This variant is the most common one, it represents references to files
or directories.
Implementations
§
Source
§
impl<'a>
Component
<'a>
1.0.0
·
Source
pub fn
as_os_str
(self) -> &'a
OsStr
Extracts the underlying
OsStr
slice.
§
Examples
use
std::path::Path;
let
path = Path::new(
"./tmp/foo/bar.txt"
);
let
components: Vec<
_
> = path.components().map(|comp| comp.as_os_str()).collect();
assert_eq!
(
&
components,
&
[
"."
,
"tmp"
,
"foo"
,
"bar.txt"
]);
Trait Implementations
§
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Component
<'_>
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.25.0
·
Source
§
impl
AsRef
<
Path
> for
Component
<'_>
Source
§
fn
as_ref
(&self) -> &
Path
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl<'a>
Clone
for
Component
<'a>
Source
§
fn
clone
(&self) ->
Component
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
Component
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
impl<'a>
Hash
for
Component
<'a>
Source
§
fn
hash
<__H:
Hasher
>(&self, state:
&mut __H
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
impl<'a>
Ord
for
Component
<'a>
Source
§
fn
cmp
(&self, other: &
Component
<'a>) ->
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
Component
<'a>
Source
§
fn
eq
(&self, other: &
Component
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
Component
<'a>
Source
§
fn
partial_cmp
(&self, other: &
Component
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
Component
<'a>
1.0.0
·
Source
§
impl<'a>
Eq
for
Component
<'a>
1.0.0
·
Source
§
impl<'a>
StructuralPartialEq
for
Component
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Component
<'a>
§
impl<'a>
RefUnwindSafe
for
Component
<'a>
§
impl<'a>
Send
for
Component
<'a>
§
impl<'a>
Sync
for
Component
<'a>
§
impl<'a>
Unpin
for
Component
<'a>
§
impl<'a>
UnwindSafe
for
Component
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