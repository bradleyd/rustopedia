Prefix in std::path - Rust
std
::
path
Enum
Prefix
Copy item path
1.0.0
·
Source
pub enum Prefix<'a> {
    Verbatim(&'a
OsStr
),
    VerbatimUNC(&'a
OsStr
, &'a
OsStr
),
    VerbatimDisk(
u8
),
    DeviceNS(&'a
OsStr
),
    UNC(&'a
OsStr
, &'a
OsStr
),
    Disk(
u8
),
}
Expand description
Windows path prefixes, e.g.,
C:
or
\\server\share
.
Windows uses a variety of path prefix styles, including references to drive
volumes (like
C:
), network shared folders (like
\\server\share
), and
others. In addition, some path prefixes are “verbatim” (i.e., prefixed with
\\?\
), in which case
/
is
not
treated as a separator and essentially
no normalization is performed.
§
Examples
use
std::path::{Component, Path, Prefix};
use
std::path::Prefix::
*
;
use
std::ffi::OsStr;
fn
get_path_prefix(s:
&
str) -> Prefix<
'_
> {
let
path = Path::new(s);
match
path.components().next().unwrap() {
        Component::Prefix(prefix_component) => prefix_component.kind(),
_
=>
panic!
(),
    }
}
assert_eq!
(Verbatim(OsStr::new(
"pictures"
)),
           get_path_prefix(
r"\\?\pictures\kittens"
));
assert_eq!
(VerbatimUNC(OsStr::new(
"server"
), OsStr::new(
"share"
)),
           get_path_prefix(
r"\\?\UNC\server\share"
));
assert_eq!
(VerbatimDisk(
b'C'
), get_path_prefix(
r"\\?\c:\"
));
assert_eq!
(DeviceNS(OsStr::new(
"BrainInterface"
)),
           get_path_prefix(
r"\\.\BrainInterface"
));
assert_eq!
(UNC(OsStr::new(
"server"
), OsStr::new(
"share"
)),
           get_path_prefix(
r"\\server\share"
));
assert_eq!
(Disk(
b'C'
), get_path_prefix(
r"C:\Users\Rust\Pictures\Ferris"
));
Variants
§
§
1.0.0
Verbatim(&'a
OsStr
)
Verbatim prefix, e.g.,
\\?\cat_pics
.
Verbatim prefixes consist of
\\?\
immediately followed by the given
component.
§
1.0.0
VerbatimUNC(&'a
OsStr
, &'a
OsStr
)
Verbatim prefix using Windows’
U
niform
N
aming
C
onvention
,
e.g.,
\\?\UNC\server\share
.
Verbatim UNC prefixes consist of
\\?\UNC\
immediately followed by the
server’s hostname and a share name.
§
1.0.0
VerbatimDisk(
u8
)
Verbatim disk prefix, e.g.,
\\?\C:
.
Verbatim disk prefixes consist of
\\?\
immediately followed by the
drive letter and
:
.
§
1.0.0
DeviceNS(&'a
OsStr
)
Device namespace prefix, e.g.,
\\.\COM42
.
Device namespace prefixes consist of
\\.\
(possibly using
/
instead of
\
), immediately followed by the device name.
§
1.0.0
UNC(&'a
OsStr
, &'a
OsStr
)
Prefix using Windows’
U
niform
N
aming
C
onvention
, e.g.
\\server\share
.
UNC prefixes consist of the server’s hostname and a share name.
§
1.0.0
Disk(
u8
)
Prefix
C:
for the given disk drive.
Implementations
§
Source
§
impl<'a>
Prefix
<'a>
1.0.0
·
Source
pub fn
is_verbatim
(&self) ->
bool
Determines if the prefix is verbatim, i.e., begins with
\\?\
.
§
Examples
use
std::path::Prefix::
*
;
use
std::ffi::OsStr;
assert!
(Verbatim(OsStr::new(
"pictures"
)).is_verbatim());
assert!
(VerbatimUNC(OsStr::new(
"server"
), OsStr::new(
"share"
)).is_verbatim());
assert!
(VerbatimDisk(
b'C'
).is_verbatim());
assert!
(!DeviceNS(OsStr::new(
"BrainInterface"
)).is_verbatim());
assert!
(!UNC(OsStr::new(
"server"
), OsStr::new(
"share"
)).is_verbatim());
assert!
(!Disk(
b'C'
).is_verbatim());
Trait Implementations
§
1.0.0
·
Source
§
impl<'a>
Clone
for
Prefix
<'a>
Source
§
fn
clone
(&self) ->
Prefix
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
Prefix
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
Prefix
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
Prefix
<'a>
Source
§
fn
cmp
(&self, other: &
Prefix
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
Prefix
<'a>
Source
§
fn
eq
(&self, other: &
Prefix
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
Prefix
<'a>
Source
§
fn
partial_cmp
(&self, other: &
Prefix
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
Prefix
<'a>
1.0.0
·
Source
§
impl<'a>
Eq
for
Prefix
<'a>
1.0.0
·
Source
§
impl<'a>
StructuralPartialEq
for
Prefix
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Prefix
<'a>
§
impl<'a>
RefUnwindSafe
for
Prefix
<'a>
§
impl<'a>
Send
for
Prefix
<'a>
§
impl<'a>
Sync
for
Prefix
<'a>
§
impl<'a>
Unpin
for
Prefix
<'a>
§
impl<'a>
UnwindSafe
for
Prefix
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