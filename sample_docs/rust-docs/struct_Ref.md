Ref in std::cell - Rust
std
::
cell
Struct
Ref
Copy item path
1.0.0
·
Source
pub struct Ref<'b, T>
where
    T: 'b + ?
Sized
,
{
/* private fields */
}
Expand description
Wraps a borrowed reference to a value in a
RefCell
box.
A wrapper type for an immutably borrowed value from a
RefCell<T>
.
See the
module-level documentation
for more.
Implementations
§
Source
§
impl<'b, T>
Ref
<'b, T>
where
    T: ?
Sized
,
1.15.0
·
Source
pub fn
clone
(orig: &
Ref
<'b, T>) ->
Ref
<'b, T>
Copies a
Ref
.
The
RefCell
is already immutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
Ref::clone(...)
. A
Clone
implementation or a method would interfere
with the widespread use of
r.borrow().clone()
to clone the contents of
a
RefCell
.
1.8.0
·
Source
pub fn
map
<U, F>(orig:
Ref
<'b, T>, f: F) ->
Ref
<'b, U>
where
    F:
FnOnce
(
&T
) ->
&U
,
    U: ?
Sized
,
Makes a new
Ref
for a component of the borrowed data.
The
RefCell
is already immutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
Ref::map(...)
.
A method would interfere with methods of the same name on the contents
of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{RefCell, Ref};
let
c = RefCell::new((
5
,
'b'
));
let
b1: Ref<
'_
, (u32, char)> = c.borrow();
let
b2: Ref<
'_
, u32> = Ref::map(b1, |t|
&
t.
0
);
assert_eq!
(
*
b2,
5
)
1.63.0
·
Source
pub fn
filter_map
<U, F>(
    orig:
Ref
<'b, T>,
    f: F,
) ->
Result
<
Ref
<'b, U>,
Ref
<'b, T>>
where
    F:
FnOnce
(
&T
) ->
Option
<
&U
>,
    U: ?
Sized
,
Makes a new
Ref
for an optional component of the borrowed data. The
original guard is returned as an
Err(..)
if the closure returns
None
.
The
RefCell
is already immutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
Ref::filter_map(...)
. A method would interfere with methods of the same
name on the contents of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{RefCell, Ref};
let
c = RefCell::new(
vec!
[
1
,
2
,
3
]);
let
b1: Ref<
'_
, Vec<u32>> = c.borrow();
let
b2:
Result
<Ref<
'_
, u32>,
_
> = Ref::filter_map(b1, |v| v.get(
1
));
assert_eq!
(
*
b2.unwrap(),
2
);
1.35.0
·
Source
pub fn
map_split
<U, V, F>(orig:
Ref
<'b, T>, f: F) -> (
Ref
<'b, U>,
Ref
<'b, V>)
where
    F:
FnOnce
(
&T
) -> (
&U
,
&V
),
    U: ?
Sized
,
    V: ?
Sized
,
Splits a
Ref
into multiple
Ref
s for different components of the
borrowed data.
The
RefCell
is already immutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
Ref::map_split(...)
. A method would interfere with methods of the same
name on the contents of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{Ref, RefCell};
let
cell = RefCell::new([
1
,
2
,
3
,
4
]);
let
borrow = cell.borrow();
let
(begin, end) = Ref::map_split(borrow, |slice| slice.split_at(
2
));
assert_eq!
(
*
begin, [
1
,
2
]);
assert_eq!
(
*
end, [
3
,
4
]);
Source
pub fn
leak
(orig:
Ref
<'b, T>) ->
&'b T
🔬
This is a nightly-only experimental API. (
cell_leak
#69099
)
Converts into a reference to the underlying data.
The underlying
RefCell
can never be mutably borrowed from again and will always appear
already immutably borrowed. It is not a good idea to leak more than a constant number of
references. The
RefCell
can be immutably borrowed again if only a smaller number of leaks
have occurred in total.
This is an associated function that needs to be used as
Ref::leak(...)
. A method would interfere with methods of the
same name on the contents of a
RefCell
used through
Deref
.
§
Examples
#![feature(cell_leak)]
use
std::cell::{RefCell, Ref};
let
cell = RefCell::new(
0
);
let
value = Ref::leak(cell.borrow());
assert_eq!
(
*
value,
0
);
assert!
(cell.try_borrow().is_ok());
assert!
(cell.try_borrow_mut().is_err());
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Debug
for
Ref
<'_, T>
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
Deref
for
Ref
<'_, T>
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
Display
for
Ref
<'_, T>
where
    T:
Display
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
Source
§
impl<'b, T, U>
CoerceUnsized
<
Ref
<'b, U>> for
Ref
<'b, T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T>
DerefPure
for
Ref
<'_, T>
where
    T: ?
Sized
,
Source
§
impl<'b, T>
PinCoerceUnsized
for
Ref
<'b, T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<'b, T>
Freeze
for
Ref
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T> !
RefUnwindSafe
for
Ref
<'b, T>
§
impl<'b, T> !
Send
for
Ref
<'b, T>
§
impl<'b, T> !
Sync
for
Ref
<'b, T>
§
impl<'b, T>
Unpin
for
Ref
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T> !
UnwindSafe
for
Ref
<'b, T>
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
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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