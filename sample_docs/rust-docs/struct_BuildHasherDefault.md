BuildHasherDefault in std::hash - Rust
std
::
hash
Struct
BuildHasherDefault
Copy item path
1.7.0
·
Source
pub struct BuildHasherDefault<H>(
/* private fields */
);
Expand description
Used to create a default
BuildHasher
instance for types that implement
Hasher
and
Default
.
BuildHasherDefault<H>
can be used when a type
H
implements
Hasher
and
Default
, and you need a corresponding
BuildHasher
instance, but none is
defined.
Any
BuildHasherDefault
is
zero-sized
. It can be created with
default
. When using
BuildHasherDefault
with
HashMap
or
HashSet
, this doesn’t need to be done, since they implement appropriate
Default
instances themselves.
§
Examples
Using
BuildHasherDefault
to specify a custom
BuildHasher
for
HashMap
:
use
std::collections::HashMap;
use
std::hash::{BuildHasherDefault, Hasher};
#[derive(Default)]
struct
MyHasher;
impl
Hasher
for
MyHasher {
fn
write(
&mut
self
, bytes:
&
[u8]) {
// Your hashing algorithm goes here!
unimplemented!
()
    }
fn
finish(
&
self
) -> u64 {
// Your hashing algorithm goes here!
unimplemented!
()
    }
}
type
MyBuildHasher = BuildHasherDefault<MyHasher>;
let
hash_map = HashMap::<u32, u32, MyBuildHasher>::default();
Implementations
§
Source
§
impl<H>
BuildHasherDefault
<H>
1.85.0 (const: 1.85.0)
·
Source
pub const fn
new
() ->
BuildHasherDefault
<H>
Creates a new BuildHasherDefault for Hasher
H
.
Trait Implementations
§
1.7.0
·
Source
§
impl<H>
BuildHasher
for
BuildHasherDefault
<H>
where
    H:
Default
+
Hasher
,
Source
§
type
Hasher
= H
Type of the hasher that will be created.
Source
§
fn
build_hasher
(&self) -> H
Creates a new hasher.
Read more
1.71.0
·
Source
§
fn
hash_one
<T>(&self, x: T) ->
u64
where
    T:
Hash
,
    Self:
Sized
,
    Self::
Hasher
:
Hasher
,
Calculates the hash of a single value.
Read more
1.7.0
·
Source
§
impl<H>
Clone
for
BuildHasherDefault
<H>
Source
§
fn
clone
(&self) ->
BuildHasherDefault
<H>
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
1.9.0
·
Source
§
impl<H>
Debug
for
BuildHasherDefault
<H>
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
1.7.0
·
Source
§
impl<H>
Default
for
BuildHasherDefault
<H>
Source
§
fn
default
() ->
BuildHasherDefault
<H>
Returns the “default value” for a type.
Read more
1.29.0
·
Source
§
impl<H>
PartialEq
for
BuildHasherDefault
<H>
Source
§
fn
eq
(&self, _other: &
BuildHasherDefault
<H>) ->
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
1.29.0
·
Source
§
impl<H>
Eq
for
BuildHasherDefault
<H>
Auto Trait Implementations
§
§
impl<H>
Freeze
for
BuildHasherDefault
<H>
§
impl<H>
RefUnwindSafe
for
BuildHasherDefault
<H>
§
impl<H>
Send
for
BuildHasherDefault
<H>
§
impl<H>
Sync
for
BuildHasherDefault
<H>
§
impl<H>
Unpin
for
BuildHasherDefault
<H>
§
impl<H>
UnwindSafe
for
BuildHasherDefault
<H>
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