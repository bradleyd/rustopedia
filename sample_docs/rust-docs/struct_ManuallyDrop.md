ManuallyDrop in std::mem - Rust
std
::
mem
Struct
ManuallyDrop
Copy item path
1.20.0
·
Source
pub struct ManuallyDrop<T>
where
    T: ?
Sized
,
{
/* private fields */
}
Expand description
A wrapper to inhibit the compiler from automatically calling
T
’s
destructor. This wrapper is 0-cost.
ManuallyDrop<T>
is guaranteed to have the same layout and bit validity as
T
, and is subject to the same layout optimizations as
T
. As a
consequence, it has
no effect
on the assumptions that the compiler makes
about its contents. For example, initializing a
ManuallyDrop<&mut T>
with
mem::zeroed
is undefined behavior. If you need to handle uninitialized
data, use
MaybeUninit<T>
instead.
Note that accessing the value inside a
ManuallyDrop<T>
is safe. This means
that a
ManuallyDrop<T>
whose content has been dropped must not be exposed
through a public safe API. Correspondingly,
ManuallyDrop::drop
is unsafe.
§
ManuallyDrop
and drop order
Rust has a well-defined
drop order
of values. To make sure that fields or
locals are dropped in a specific order, reorder the declarations such that
the implicit drop order is the correct one.
It is possible to use
ManuallyDrop
to control the drop order, but this
requires unsafe code and is hard to do correctly in the presence of
unwinding.
For example, if you want to make sure that a specific field is dropped after
the others, make it the last field of a struct:
struct
Context;
struct
Widget {
    children: Vec<Widget>,
// `context` will be dropped after `children`.
    // Rust guarantees that fields are dropped in the order of declaration.
context: Context,
}
§
Interaction with
Box
Currently, if you have a
ManuallyDrop<T>
, where the type
T
is a
Box
or
contains a
Box
inside, then dropping the
T
followed by moving the
ManuallyDrop<T>
is
considered to be undefined
behavior
.
That is, the following code causes undefined behavior:
use
std::mem::ManuallyDrop;
let
mut
x = ManuallyDrop::new(Box::new(
42
));
unsafe
{
    ManuallyDrop::drop(
&mut
x);
}
let
y = x;
// Undefined behavior!
This is
likely to change in the
future
. In the
meantime, consider using
MaybeUninit
instead.
§
Safety hazards when storing
ManuallyDrop
in a struct or an enum.
Special care is needed when all of the conditions below are met:
A struct or enum contains a
ManuallyDrop
.
The
ManuallyDrop
is not inside a
union
.
The struct or enum is part of public API, or is stored in a struct or an
enum that is part of public API.
There is code that drops the contents of the
ManuallyDrop
field, and
this code is outside the struct or enum’s
Drop
implementation.
In particular, the following hazards may occur:
§
Storing generic types
If the
ManuallyDrop
contains a client-supplied generic type, the client
might provide a
Box
as that type. This would cause undefined behavior when
the struct or enum is later moved, as mentioned in the previous section. For
example, the following code causes undefined behavior:
use
std::mem::ManuallyDrop;
pub struct
BadOption<T> {
// Invariant: Has been dropped iff `is_some` is false.
value: ManuallyDrop<T>,
    is_some: bool,
}
impl
<T> BadOption<T> {
pub fn
new(value: T) ->
Self
{
Self
{ value: ManuallyDrop::new(value), is_some:
true
}
    }
pub fn
change_to_none(
&mut
self
) {
if
self
.is_some {
self
.is_some =
false
;
unsafe
{
// SAFETY: `value` hasn't been dropped yet, as per the invariant
                // (This is actually unsound!)
ManuallyDrop::drop(
&mut
self
.value);
            }
        }
    }
}
// In another crate:
let
mut
option = BadOption::new(Box::new(
42
));
option.change_to_none();
let
option2 = option;
// Undefined behavior!
§
Deriving traits
Deriving
Debug
,
Clone
,
PartialEq
,
PartialOrd
,
Ord
, or
Hash
on
the struct or enum could be unsound, since the derived implementations of
these traits would access the
ManuallyDrop
field. For example, the
following code causes undefined behavior:
use
std::mem::ManuallyDrop;
// This derive is unsound in combination with the `ManuallyDrop::drop` call.
#[derive(Debug)]
pub struct
Foo {
    value: ManuallyDrop<String>,
}
impl
Foo {
pub fn
new() ->
Self
{
let
mut
temp =
Self
{
            value: ManuallyDrop::new(String::from(
"Unsafe rust is hard."
))
        };
unsafe
{
// SAFETY: `value` hasn't been dropped yet.
ManuallyDrop::drop(
&mut
temp.value);
        }
        temp
    }
}
// In another crate:
let
foo = Foo::new();
println!
(
"{:?}"
, foo);
// Undefined behavior!
Implementations
§
Source
§
impl<T>
ManuallyDrop
<T>
1.20.0 (const: 1.32.0)
·
Source
pub const fn
new
(value: T) ->
ManuallyDrop
<T>
Wrap a value to be manually dropped.
§
Examples
use
std::mem::ManuallyDrop;
let
mut
x = ManuallyDrop::new(String::from(
"Hello World!"
));
x.truncate(
5
);
// You can still safely operate on the value
assert_eq!
(
*
x,
"Hello"
);
// But `Drop` will not be run here
1.20.0 (const: 1.32.0)
·
Source
pub const fn
into_inner
(slot:
ManuallyDrop
<T>) -> T
Extracts the value from the
ManuallyDrop
container.
This allows the value to be dropped again.
§
Examples
use
std::mem::ManuallyDrop;
let
x = ManuallyDrop::new(Box::new(()));
let _
: Box<()> = ManuallyDrop::into_inner(x);
// This drops the `Box`.
1.42.0
·
Source
pub unsafe fn
take
(slot: &mut
ManuallyDrop
<T>) -> T
Takes the value from the
ManuallyDrop<T>
container out.
This method is primarily intended for moving out values in drop.
Instead of using
ManuallyDrop::drop
to manually drop the value,
you can use this method to take the value and use it however desired.
Whenever possible, it is preferable to use
into_inner
instead, which prevents duplicating the content of the
ManuallyDrop<T>
.
§
Safety
This function semantically moves out the contained value without preventing further usage,
leaving the state of this container unchanged.
It is your responsibility to ensure that this
ManuallyDrop
is not used again.
Source
§
impl<T>
ManuallyDrop
<T>
where
    T: ?
Sized
,
1.20.0
·
Source
pub unsafe fn
drop
(slot: &mut
ManuallyDrop
<T>)
Manually drops the contained value.
This is exactly equivalent to calling
ptr::drop_in_place
with a
pointer to the contained value. As such, unless the contained value is a
packed struct, the destructor will be called in-place without moving the
value, and thus can be used to safely drop
pinned
data.
If you have ownership of the value, you can use
ManuallyDrop::into_inner
instead.
§
Safety
This function runs the destructor of the contained value. Other than changes made by
the destructor itself, the memory is left unchanged, and so as far as the compiler is
concerned still holds a bit-pattern which is valid for the type
T
.
However, this “zombie” value should not be exposed to safe code, and this function
should not be called more than once. To use a value after it’s been dropped, or drop
a value multiple times, can cause Undefined Behavior (depending on what
drop
does).
This is normally prevented by the type system, but users of
ManuallyDrop
must
uphold those guarantees without assistance from the compiler.
Trait Implementations
§
1.20.0
·
Source
§
impl<T>
Clone
for
ManuallyDrop
<T>
where
    T:
Clone
+ ?
Sized
,
Source
§
fn
clone
(&self) ->
ManuallyDrop
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
1.20.0
·
Source
§
impl<T>
Debug
for
ManuallyDrop
<T>
where
    T:
Debug
+ ?
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
1.20.0
·
Source
§
impl<T>
Default
for
ManuallyDrop
<T>
where
    T:
Default
+ ?
Sized
,
Source
§
fn
default
() ->
ManuallyDrop
<T>
Returns the “default value” for a type.
Read more
1.20.0
·
Source
§
impl<T>
Deref
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
Source
§
type
Target
= T
The resulting type after dereferencing.
Source
§
fn
deref
(&self) ->
&T
Dereferences the value.
1.20.0
·
Source
§
impl<T>
DerefMut
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
Source
§
fn
deref_mut
(&mut self) ->
&mut T
Mutably dereferences the value.
1.20.0
·
Source
§
impl<T>
Hash
for
ManuallyDrop
<T>
where
    T:
Hash
+ ?
Sized
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
1.20.0
·
Source
§
impl<T>
Ord
for
ManuallyDrop
<T>
where
    T:
Ord
+ ?
Sized
,
Source
§
fn
cmp
(&self, other: &
ManuallyDrop
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
1.20.0
·
Source
§
impl<T>
PartialEq
for
ManuallyDrop
<T>
where
    T:
PartialEq
+ ?
Sized
,
Source
§
fn
eq
(&self, other: &
ManuallyDrop
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
1.20.0
·
Source
§
impl<T>
PartialOrd
for
ManuallyDrop
<T>
where
    T:
PartialOrd
+ ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
ManuallyDrop
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
1.20.0
·
Source
§
impl<T>
Copy
for
ManuallyDrop
<T>
where
    T:
Copy
+ ?
Sized
,
Source
§
impl<T>
DerefPure
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
1.20.0
·
Source
§
impl<T>
Eq
for
ManuallyDrop
<T>
where
    T:
Eq
+ ?
Sized
,
1.20.0
·
Source
§
impl<T>
StructuralPartialEq
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T>
Freeze
for
ManuallyDrop
<T>
where
    T:
Freeze
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
ManuallyDrop
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
ManuallyDrop
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
ManuallyDrop
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
ManuallyDrop
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
ManuallyDrop
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
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
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