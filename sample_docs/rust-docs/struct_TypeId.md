TypeId in std::any - Rust
std
::
any
Struct
TypeId
Copy item path
1.0.0
·
Source
pub struct TypeId {
/* private fields */
}
Expand description
A
TypeId
represents a globally unique identifier for a type.
Each
TypeId
is an opaque object which does not allow inspection of what’s
inside but does allow basic operations such as cloning, comparison,
printing, and showing.
A
TypeId
is currently only available for types which ascribe to
'static
,
but this limitation may be removed in the future.
While
TypeId
implements
Hash
,
PartialOrd
, and
Ord
, it is worth
noting that the hashes and ordering will vary between Rust releases. Beware
of relying on them inside of your code!
§
Danger of Improper Variance
You might think that subtyping is impossible between two static types,
but this is false; there exists a static type with a static subtype.
To wit,
fn(&str)
, which is short for
for<'any> fn(&'any str)
, and
fn(&'static str)
, are two distinct, static types, and yet,
fn(&str)
is a subtype of
fn(&'static str)
, since any value of type
fn(&str)
can be used where a value of type
fn(&'static str)
is needed.
This means that abstractions around
TypeId
, despite its
'static
bound on arguments, still need to worry about unnecessary
and improper variance: it is advisable to strive for invariance
first. The usability impact will be negligible, while the reduction
in the risk of unsoundness will be most welcome.
§
Examples
Suppose
SubType
is a subtype of
SuperType
, that is,
a value of type
SubType
can be used wherever
a value of type
SuperType
is expected.
Suppose also that
CoVar<T>
is a generic type, which is covariant over
T
(like many other types, including
PhantomData<T>
and
Vec<T>
).
Then, by covariance,
CoVar<SubType>
is a subtype of
CoVar<SuperType>
,
that is, a value of type
CoVar<SubType>
can be used wherever
a value of type
CoVar<SuperType>
is expected.
Then if
CoVar<SuperType>
relies on
TypeId::of::<SuperType>()
to uphold any invariants,
those invariants may be broken because a value of type
CoVar<SuperType>
can be created
without going through any of its methods, like so:
type
SubType =
fn
(
&
());
type
SuperType =
fn
(
&
'static
());
type
CoVar<T> = Vec<T>;
// imagine something more complicated
let
sub: CoVar<SubType> = CoVar::new();
// we have a `CoVar<SuperType>` instance without
// *ever* having called `CoVar::<SuperType>::new()`!
let
fake_super: CoVar<SuperType> = sub;
The following is an example program that tries to use
TypeId::of
to
implement a generic type
Unique<T>
that guarantees unique instances for each
Unique<T>
,
that is, and for each type
T
there can be at most one value of type
Unique<T>
at any time.
mod
unique {
use
std::any::TypeId;
use
std::collections::BTreeSet;
use
std::marker::PhantomData;
use
std::sync::Mutex;
static
ID_SET: Mutex<BTreeSet<TypeId>> = Mutex::new(BTreeSet::new());
// TypeId has only covariant uses, which makes Unique covariant over TypeAsId 🚨
#[derive(Debug, PartialEq)]
pub struct
Unique<TypeAsId:
'static
>(
// private field prevents creation without `new` outside this module
PhantomData<TypeAsId>,
    );
impl
<TypeAsId:
'static
> Unique<TypeAsId> {
pub fn
new() ->
Option
<
Self
> {
let
mut
set = ID_SET.lock().unwrap();
            (set.insert(TypeId::of::<TypeAsId>())).then(||
Self
(PhantomData))
        }
    }
impl
<TypeAsId:
'static
> Drop
for
Unique<TypeAsId> {
fn
drop(
&mut
self
) {
let
mut
set = ID_SET.lock().unwrap();
            (!set.remove(
&
TypeId::of::<TypeAsId>())).then(||
panic!
(
"duplicity detected"
));
        }
    }
}
use
unique::Unique;
// `OtherRing` is a subtype of `TheOneRing`. Both are 'static, and thus have a TypeId.
type
TheOneRing =
fn
(
&
'static
());
type
OtherRing =
fn
(
&
());
fn
main() {
let
the_one_ring: Unique<TheOneRing> = Unique::new().unwrap();
assert_eq!
(Unique::<TheOneRing>::new(),
None
);
let
other_ring: Unique<OtherRing> = Unique::new().unwrap();
// Use that `Unique<OtherRing>` is a subtype of `Unique<TheOneRing>` 🚨
let
fake_one_ring: Unique<TheOneRing> = other_ring;
assert_eq!
(fake_one_ring, the_one_ring);

    std::mem::forget(fake_one_ring);
}
Implementations
§
Source
§
impl
TypeId
1.0.0 (const:
unstable
)
·
Source
pub fn
of
<T>() ->
TypeId
where
    T: 'static + ?
Sized
,
Returns the
TypeId
of the generic type parameter.
§
Examples
use
std::any::{Any, TypeId};
fn
is_string<T:
?
Sized + Any>(_s:
&
T) -> bool {
    TypeId::of::<String>() == TypeId::of::<T>()
}
assert_eq!
(is_string(
&
0
),
false
);
assert_eq!
(is_string(
&
"cookie monster"
.to_string()),
true
);
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
TypeId
Source
§
fn
clone
(&self) ->
TypeId
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
impl
Debug
for
TypeId
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
1.0.0
·
Source
§
impl
Hash
for
TypeId
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
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
1.0.0
·
Source
§
impl
Ord
for
TypeId
Source
§
fn
cmp
(&self, other: &
TypeId
) ->
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
impl
PartialEq
for
TypeId
Source
§
fn
eq
(&self, other: &
TypeId
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
1.0.0
·
Source
§
impl
PartialOrd
for
TypeId
Source
§
fn
partial_cmp
(&self, other: &
TypeId
) ->
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
impl
Copy
for
TypeId
1.0.0
·
Source
§
impl
Eq
for
TypeId
Auto Trait Implementations
§
§
impl
Freeze
for
TypeId
§
impl
RefUnwindSafe
for
TypeId
§
impl
Send
for
TypeId
§
impl
Sync
for
TypeId
§
impl
Unpin
for
TypeId
§
impl
UnwindSafe
for
TypeId
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