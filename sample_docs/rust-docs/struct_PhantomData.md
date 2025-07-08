PhantomData in std::marker - Rust
std
::
marker
Struct
PhantomData
Copy item path
1.0.0
·
Source
pub struct PhantomData<T>
where
    T: ?
Sized
;
Expand description
Zero-sized type used to mark things that “act like” they own a
T
.
Adding a
PhantomData<T>
field to your type tells the compiler that your
type acts as though it stores a value of type
T
, even though it doesn’t
really. This information is used when computing certain safety properties.
For a more in-depth explanation of how to use
PhantomData<T>
, please see
the Nomicon
.
§
A ghastly note 👻👻👻
Though they both have scary names,
PhantomData
and ‘phantom types’ are
related, but not identical. A phantom type parameter is simply a type
parameter which is never used. In Rust, this often causes the compiler to
complain, and the solution is to add a “dummy” use by way of
PhantomData
.
§
Examples
§
Unused lifetime parameters
Perhaps the most common use case for
PhantomData
is a struct that has an
unused lifetime parameter, typically as part of some unsafe code. For
example, here is a struct
Slice
that has two pointers of type
*const T
,
presumably pointing into an array somewhere:
ⓘ
struct
Slice<
'a
, T> {
    start:
*const
T,
    end:
*const
T,
}
The intention is that the underlying data is only valid for the
lifetime
'a
, so
Slice
should not outlive
'a
. However, this
intent is not expressed in the code, since there are no uses of
the lifetime
'a
and hence it is not clear what data it applies
to. We can correct this by telling the compiler to act
as if
the
Slice
struct contained a reference
&'a T
:
use
std::marker::PhantomData;
struct
Slice<
'a
, T> {
    start:
*const
T,
    end:
*const
T,
    phantom: PhantomData<
&
'a
T>,
}
This also in turn infers the lifetime bound
T: 'a
, indicating
that any references in
T
are valid over the lifetime
'a
.
When initializing a
Slice
you simply provide the value
PhantomData
for the field
phantom
:
fn
borrow_vec<T>(vec:
&
Vec<T>) -> Slice<
'_
, T> {
let
ptr = vec.as_ptr();
    Slice {
        start: ptr,
        end:
unsafe
{ ptr.add(vec.len()) },
        phantom: PhantomData,
    }
}
§
Unused type parameters
It sometimes happens that you have unused type parameters which
indicate what type of data a struct is “tied” to, even though that
data is not actually found in the struct itself. Here is an
example where this arises with
FFI
. The foreign interface uses
handles of type
*mut ()
to refer to Rust values of different
types. We track the Rust type using a phantom type parameter on
the struct
ExternalResource
which wraps a handle.
use
std::marker::PhantomData;
struct
ExternalResource<R> {
   resource_handle:
*mut
(),
   resource_type: PhantomData<R>,
}
impl
<R: ResType> ExternalResource<R> {
fn
new() ->
Self
{
let
size_of_res = size_of::<R>();
Self
{
            resource_handle: foreign_lib::new(size_of_res),
            resource_type: PhantomData,
        }
    }
fn
do_stuff(
&
self
, param: ParamType) {
let
foreign_params = convert_params(param);
        foreign_lib::do_stuff(
self
.resource_handle, foreign_params);
    }
}
§
Ownership and the drop check
The exact interaction of
PhantomData
with drop check
may change in the future
.
Currently, adding a field of type
PhantomData<T>
indicates that your type
owns
data of type
T
in very rare circumstances. This in turn has effects on the Rust compiler’s
drop check
analysis. For the exact rules, see the
drop check
documentation.
§
Layout
For all
T
, the following are guaranteed:
size_of::<PhantomData<T>>() == 0
align_of::<PhantomData<T>>() == 1
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
clone
(&self) ->
PhantomData
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
1.0.0
·
Source
§
impl<T>
Debug
for
PhantomData
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
1.0.0
·
Source
§
impl<T>
Default
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
default
() ->
PhantomData
<T>
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl<T>
Hash
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
hash
<H>(&self, _:
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
impl<T>
Ord
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
cmp
(&self, _other: &
PhantomData
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
1.0.0
·
Source
§
impl<T>
PartialEq
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
eq
(&self, _other: &
PhantomData
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
1.0.0
·
Source
§
impl<T>
PartialOrd
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
fn
partial_cmp
(&self, _other: &
PhantomData
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
1.0.0
·
Source
§
impl<T>
Copy
for
PhantomData
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Eq
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
StructuralPartialEq
for
PhantomData
<T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T>
RefUnwindSafe
for
PhantomData
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
PhantomData
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Sync
for
PhantomData
<T>
where
    T:
Sync
+ ?
Sized
,
§
impl<T>
Unpin
for
PhantomData
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
PhantomData
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