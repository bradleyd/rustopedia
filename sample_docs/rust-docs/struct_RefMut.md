RefMut in std::cell - Rust
std
::
cell
Struct
RefMut
Copy item path
1.0.0
·
Source
pub struct RefMut<'b, T>
where
    T: 'b + ?
Sized
,
{
/* private fields */
}
Expand description
A wrapper type for a mutably borrowed value from a
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
RefMut
<'b, T>
where
    T: ?
Sized
,
1.8.0
·
Source
pub fn
map
<U, F>(orig:
RefMut
<'b, T>, f: F) ->
RefMut
<'b, U>
where
    F:
FnOnce
(
&mut T
) ->
&mut U
,
    U: ?
Sized
,
Makes a new
RefMut
for a component of the borrowed data, e.g., an enum
variant.
The
RefCell
is already mutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
RefMut::map(...)
. A method would interfere with methods of the same
name on the contents of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{RefCell, RefMut};
let
c = RefCell::new((
5
,
'b'
));
{
let
b1: RefMut<
'_
, (u32, char)> = c.borrow_mut();
let
mut
b2: RefMut<
'_
, u32> = RefMut::map(b1, |t|
&mut
t.
0
);
assert_eq!
(
*
b2,
5
);
*
b2 =
42
;
}
assert_eq!
(
*
c.borrow(), (
42
,
'b'
));
1.63.0
·
Source
pub fn
filter_map
<U, F>(
    orig:
RefMut
<'b, T>,
    f: F,
) ->
Result
<
RefMut
<'b, U>,
RefMut
<'b, T>>
where
    F:
FnOnce
(
&mut T
) ->
Option
<
&mut U
>,
    U: ?
Sized
,
Makes a new
RefMut
for an optional component of the borrowed data. The
original guard is returned as an
Err(..)
if the closure returns
None
.
The
RefCell
is already mutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
RefMut::filter_map(...)
. A method would interfere with methods of the
same name on the contents of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{RefCell, RefMut};
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

{
let
b1: RefMut<
'_
, Vec<u32>> = c.borrow_mut();
let
mut
b2:
Result
<RefMut<
'_
, u32>,
_
> = RefMut::filter_map(b1, |v| v.get_mut(
1
));
if let
Ok
(
mut
b2) = b2 {
*
b2 +=
2
;
    }
}
assert_eq!
(
*
c.borrow(),
vec!
[
1
,
4
,
3
]);
1.35.0
·
Source
pub fn
map_split
<U, V, F>(
    orig:
RefMut
<'b, T>,
    f: F,
) -> (
RefMut
<'b, U>,
RefMut
<'b, V>)
where
    F:
FnOnce
(
&mut T
) -> (
&mut U
,
&mut V
),
    U: ?
Sized
,
    V: ?
Sized
,
Splits a
RefMut
into multiple
RefMut
s for different components of the
borrowed data.
The underlying
RefCell
will remain mutably borrowed until both
returned
RefMut
s go out of scope.
The
RefCell
is already mutably borrowed, so this cannot fail.
This is an associated function that needs to be used as
RefMut::map_split(...)
. A method would interfere with methods of the
same name on the contents of a
RefCell
used through
Deref
.
§
Examples
use
std::cell::{RefCell, RefMut};
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
borrow = cell.borrow_mut();
let
(
mut
begin,
mut
end) = RefMut::map_split(borrow, |slice| slice.split_at_mut(
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
begin.copy_from_slice(
&
[
4
,
3
]);
end.copy_from_slice(
&
[
2
,
1
]);
Source
pub fn
leak
(orig:
RefMut
<'b, T>) ->
&'b mut T
🔬
This is a nightly-only experimental API. (
cell_leak
#69099
)
Converts into a mutable reference to the underlying data.
The underlying
RefCell
can not be borrowed from again and will always appear already
mutably borrowed, making the returned reference the only to the interior.
This is an associated function that needs to be used as
RefMut::leak(...)
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
std::cell::{RefCell, RefMut};
let
cell = RefCell::new(
0
);
let
value = RefMut::leak(cell.borrow_mut());
assert_eq!
(
*
value,
0
);
*
value =
1
;
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
RefMut
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
RefMut
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
1.0.0
·
Source
§
impl<T>
DerefMut
for
RefMut
<'_, T>
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
Display
for
RefMut
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
RefMut
<'b, U>> for
RefMut
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
RefMut
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
RefMut
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
RefMut
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T> !
RefUnwindSafe
for
RefMut
<'b, T>
§
impl<'b, T> !
Send
for
RefMut
<'b, T>
§
impl<'b, T> !
Sync
for
RefMut
<'b, T>
§
impl<'b, T>
Unpin
for
RefMut
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T> !
UnwindSafe
for
RefMut
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