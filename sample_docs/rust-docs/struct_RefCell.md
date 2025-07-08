RefCell in std::cell - Rust
std
::
cell
Struct
RefCell
Copy item path
1.0.0
·
Source
pub struct RefCell<T>
where
    T: ?
Sized
,
{
/* private fields */
}
Expand description
A mutable memory location with dynamically checked borrow rules
See the
module-level documentation
for more.
Implementations
§
Source
§
impl<T>
RefCell
<T>
1.0.0 (const: 1.24.0)
·
Source
pub const fn
new
(value: T) ->
RefCell
<T>
Creates a new
RefCell
containing
value
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
1.0.0 (const: 1.83.0)
·
Source
pub const fn
into_inner
(self) -> T
Consumes the
RefCell
, returning the wrapped value.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
five = c.into_inner();
1.24.0
·
Source
pub fn
replace
(&self, t: T) -> T
Replaces the wrapped value with a new one, returning the old value,
without deinitializing either one.
This function corresponds to
std::mem::replace
.
§
Panics
Panics if the value is currently borrowed.
§
Examples
use
std::cell::RefCell;
let
cell = RefCell::new(
5
);
let
old_value = cell.replace(
6
);
assert_eq!
(old_value,
5
);
assert_eq!
(cell, RefCell::new(
6
));
1.35.0
·
Source
pub fn
replace_with
<F>(&self, f: F) -> T
where
    F:
FnOnce
(
&mut T
) -> T,
Replaces the wrapped value with a new one computed from
f
, returning
the old value, without deinitializing either one.
§
Panics
Panics if the value is currently borrowed.
§
Examples
use
std::cell::RefCell;
let
cell = RefCell::new(
5
);
let
old_value = cell.replace_with(|
&mut
old| old +
1
);
assert_eq!
(old_value,
5
);
assert_eq!
(cell, RefCell::new(
6
));
1.24.0
·
Source
pub fn
swap
(&self, other: &
RefCell
<T>)
Swaps the wrapped value of
self
with the wrapped value of
other
,
without deinitializing either one.
This function corresponds to
std::mem::swap
.
§
Panics
Panics if the value in either
RefCell
is currently borrowed, or
if
self
and
other
point to the same
RefCell
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
d = RefCell::new(
6
);
c.swap(
&
d);
assert_eq!
(c, RefCell::new(
6
));
assert_eq!
(d, RefCell::new(
5
));
Source
§
impl<T>
RefCell
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
pub fn
borrow
(&self) ->
Ref
<'_, T>
Immutably borrows the wrapped value.
The borrow lasts until the returned
Ref
exits scope. Multiple
immutable borrows can be taken out at the same time.
§
Panics
Panics if the value is currently mutably borrowed. For a non-panicking variant, use
try_borrow
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
borrowed_five = c.borrow();
let
borrowed_five2 = c.borrow();
An example of panic:
ⓘ
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
m = c.borrow_mut();
let
b = c.borrow();
// this causes a panic
1.13.0
·
Source
pub fn
try_borrow
(&self) ->
Result
<
Ref
<'_, T>,
BorrowError
>
Immutably borrows the wrapped value, returning an error if the value is currently mutably
borrowed.
The borrow lasts until the returned
Ref
exits scope. Multiple immutable borrows can be
taken out at the same time.
This is the non-panicking variant of
borrow
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);

{
let
m = c.borrow_mut();
assert!
(c.try_borrow().is_err());
}

{
let
m = c.borrow();
assert!
(c.try_borrow().is_ok());
}
1.0.0
·
Source
pub fn
borrow_mut
(&self) ->
RefMut
<'_, T>
Mutably borrows the wrapped value.
The borrow lasts until the returned
RefMut
or all
RefMut
s derived
from it exit scope. The value cannot be borrowed while this borrow is
active.
§
Panics
Panics if the value is currently borrowed. For a non-panicking variant, use
try_borrow_mut
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
"hello"
.to_owned());
*
c.borrow_mut() =
"bonjour"
.to_owned();
assert_eq!
(
&*
c.borrow(),
"bonjour"
);
An example of panic:
ⓘ
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
m = c.borrow();
let
b = c.borrow_mut();
// this causes a panic
1.13.0
·
Source
pub fn
try_borrow_mut
(&self) ->
Result
<
RefMut
<'_, T>,
BorrowMutError
>
Mutably borrows the wrapped value, returning an error if the value is currently borrowed.
The borrow lasts until the returned
RefMut
or all
RefMut
s derived
from it exit scope. The value cannot be borrowed while this borrow is
active.
This is the non-panicking variant of
borrow_mut
.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);

{
let
m = c.borrow();
assert!
(c.try_borrow_mut().is_err());
}
assert!
(c.try_borrow_mut().is_ok());
1.12.0
·
Source
pub fn
as_ptr
(&self) ->
*mut T
Returns a raw pointer to the underlying data in this cell.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
ptr = c.as_ptr();
1.11.0
·
Source
pub fn
get_mut
(&mut self) ->
&mut T
Returns a mutable reference to the underlying data.
Since this method borrows
RefCell
mutably, it is statically guaranteed
that no borrows to the underlying data exist. The dynamic checks inherent
in
borrow_mut
and most other methods of
RefCell
are therefore
unnecessary.
This method can only be called if
RefCell
can be mutably borrowed,
which in general is only the case directly after the
RefCell
has
been created. In these situations, skipping the aforementioned dynamic
borrowing checks may yield better ergonomics and runtime-performance.
In most situations where
RefCell
is used, it can’t be borrowed mutably.
Use
borrow_mut
to get mutable access to the underlying data then.
§
Examples
use
std::cell::RefCell;
let
mut
c = RefCell::new(
5
);
*
c.get_mut() +=
1
;
assert_eq!
(c, RefCell::new(
6
));
Source
pub fn
undo_leak
(&mut self) ->
&mut T
🔬
This is a nightly-only experimental API. (
cell_leak
#69099
)
Undo the effect of leaked guards on the borrow state of the
RefCell
.
This call is similar to
get_mut
but more specialized. It borrows
RefCell
mutably to
ensure no borrows exist and then resets the state tracking shared borrows. This is relevant
if some
Ref
or
RefMut
borrows have been leaked.
§
Examples
#![feature(cell_leak)]
use
std::cell::RefCell;
let
mut
c = RefCell::new(
0
);
std::mem::forget(c.borrow_mut());
assert!
(c.try_borrow().is_err());
c.undo_leak();
assert!
(c.try_borrow().is_ok());
1.37.0
·
Source
pub unsafe fn
try_borrow_unguarded
(&self) ->
Result
<
&T
,
BorrowError
>
Immutably borrows the wrapped value, returning an error if the value is
currently mutably borrowed.
§
Safety
Unlike
RefCell::borrow
, this method is unsafe because it does not
return a
Ref
, thus leaving the borrow flag untouched. Mutably
borrowing the
RefCell
while the reference returned by this method
is alive is undefined behavior.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);

{
let
m = c.borrow_mut();
assert!
(
unsafe
{ c.try_borrow_unguarded() }.is_err());
}

{
let
m = c.borrow();
assert!
(
unsafe
{ c.try_borrow_unguarded() }.is_ok());
}
Source
§
impl<T>
RefCell
<T>
where
    T:
Default
,
1.50.0
·
Source
pub fn
take
(&self) -> T
Takes the wrapped value, leaving
Default::default()
in its place.
§
Panics
Panics if the value is currently borrowed.
§
Examples
use
std::cell::RefCell;
let
c = RefCell::new(
5
);
let
five = c.take();
assert_eq!
(five,
5
);
assert_eq!
(c.into_inner(),
0
);
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
RefCell
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
RefCell
<T>
§
Panics
Panics if the value is currently mutably borrowed.
Source
§
fn
clone_from
(&mut self, source: &
RefCell
<T>)
§
Panics
Panics if
source
is currently mutably borrowed.
1.0.0
·
Source
§
impl<T>
Debug
for
RefCell
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
1.0.0
·
Source
§
impl<T>
Default
for
RefCell
<T>
where
    T:
Default
,
Source
§
fn
default
() ->
RefCell
<T>
Creates a
RefCell<T>
, with the
Default
value for T.
1.12.0
·
Source
§
impl<T>
From
<T> for
RefCell
<T>
Source
§
fn
from
(t: T) ->
RefCell
<T>
Creates a new
RefCell<T>
containing the given value.
1.10.0
·
Source
§
impl<T>
Ord
for
RefCell
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
RefCell
<T>) ->
Ordering
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
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
RefCell
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
RefCell
<T>) ->
bool
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
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
1.10.0
·
Source
§
impl<T>
PartialOrd
for
RefCell
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
RefCell
<T>) ->
Option
<
Ordering
>
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
Source
§
fn
lt
(&self, other: &
RefCell
<T>) ->
bool
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
Source
§
fn
le
(&self, other: &
RefCell
<T>) ->
bool
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
Source
§
fn
gt
(&self, other: &
RefCell
<T>) ->
bool
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
Source
§
fn
ge
(&self, other: &
RefCell
<T>) ->
bool
§
Panics
Panics if the value in either
RefCell
is currently mutably borrowed.
Source
§
impl<T, U>
CoerceUnsized
<
RefCell
<U>> for
RefCell
<T>
where
    T:
CoerceUnsized
<U>,
1.2.0
·
Source
§
impl<T>
Eq
for
RefCell
<T>
where
    T:
Eq
+ ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
RefCell
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
Send
for
RefCell
<T>
where
    T:
Send
+ ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Sync
for
RefCell
<T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T> !
Freeze
for
RefCell
<T>
§
impl<T> !
RefUnwindSafe
for
RefCell
<T>
§
impl<T>
Unpin
for
RefCell
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
RefCell
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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