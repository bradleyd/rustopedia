Cell in std::cell - Rust
std
::
cell
Struct
Cell
Copy item path
1.0.0
·
Source
pub struct Cell<T>
where
    T: ?
Sized
,
{
/* private fields */
}
Expand description
A mutable memory location.
§
Memory layout
Cell<T>
has the same
memory layout and caveats as
UnsafeCell<T>
. In particular, this means that
Cell<T>
has the same in-memory representation as its inner type
T
.
§
Examples
In this example, you can see that
Cell<T>
enables mutation inside an
immutable struct. In other words, it enables “interior mutability”.
use
std::cell::Cell;
struct
SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}
let
my_struct = SomeStruct {
    regular_field:
0
,
    special_field: Cell::new(
1
),
};
let
new_value =
100
;
// ERROR: `my_struct` is immutable
// my_struct.regular_field = new_value;

// WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
// which can always be mutated
my_struct.special_field.set(new_value);
assert_eq!
(my_struct.special_field.get(), new_value);
See the
module-level documentation
for more.
Implementations
§
Source
§
impl<T>
Cell
<T>
1.0.0 (const: 1.24.0)
·
Source
pub const fn
new
(value: T) ->
Cell
<T>
Creates a new
Cell
containing the given value.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
5
);
1.0.0
·
Source
pub fn
set
(&self, val: T)
Sets the contained value.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
5
);

c.set(
10
);
1.17.0
·
Source
pub fn
swap
(&self, other: &
Cell
<T>)
Swaps the values of two
Cell
s.
The difference with
std::mem::swap
is that this function doesn’t
require a
&mut
reference.
§
Panics
This function will panic if
self
and
other
are different
Cell
s that partially overlap.
(Using just standard library methods, it is impossible to create such partially overlapping
Cell
s.
However, unsafe code is allowed to e.g. create two
&Cell<[i32; 2]>
that partially overlap.)
§
Examples
use
std::cell::Cell;
let
c1 = Cell::new(
5i32
);
let
c2 = Cell::new(
10i32
);
c1.swap(
&
c2);
assert_eq!
(
10
, c1.get());
assert_eq!
(
5
, c2.get());
1.17.0 (const:
unstable
)
·
Source
pub fn
replace
(&self, val: T) -> T
Replaces the contained value with
val
, and returns the old contained value.
§
Examples
use
std::cell::Cell;
let
cell = Cell::new(
5
);
assert_eq!
(cell.get(),
5
);
assert_eq!
(cell.replace(
10
),
5
);
assert_eq!
(cell.get(),
10
);
1.17.0 (const: 1.83.0)
·
Source
pub const fn
into_inner
(self) -> T
Unwraps the value, consuming the cell.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
5
);
let
five = c.into_inner();
assert_eq!
(five,
5
);
Source
§
impl<T>
Cell
<T>
where
    T:
Copy
,
1.0.0 (const:
unstable
)
·
Source
pub fn
get
(&self) -> T
Returns a copy of the contained value.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
5
);
let
five = c.get();
Source
pub fn
update
<F>(&self, f: F) -> T
where
    F:
FnOnce
(T) -> T,
🔬
This is a nightly-only experimental API. (
cell_update
#50186
)
Updates the contained value using a function and returns the new value.
§
Examples
#![feature(cell_update)]
use
std::cell::Cell;
let
c = Cell::new(
5
);
let
new = c.update(|x| x +
1
);
assert_eq!
(new,
6
);
assert_eq!
(c.get(),
6
);
Source
§
impl<T>
Cell
<T>
where
    T: ?
Sized
,
1.12.0 (const: 1.32.0)
·
Source
pub const fn
as_ptr
(&self) ->
*mut T
Returns a raw pointer to the underlying data in this cell.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
5
);
let
ptr = c.as_ptr();
1.11.0 (const:
unstable
)
·
Source
pub fn
get_mut
(&mut self) ->
&mut T
Returns a mutable reference to the underlying data.
This call borrows
Cell
mutably (at compile-time) which guarantees
that we possess the only reference.
However be cautious: this method expects
self
to be mutable, which is
generally not the case when using a
Cell
. If you require interior
mutability by reference, consider using
RefCell
which provides
run-time checked mutable borrows through its
borrow_mut
method.
§
Examples
use
std::cell::Cell;
let
mut
c = Cell::new(
5
);
*
c.get_mut() +=
1
;
assert_eq!
(c.get(),
6
);
1.37.0 (const:
unstable
)
·
Source
pub fn
from_mut
(t:
&mut T
) -> &
Cell
<T>
Returns a
&Cell<T>
from a
&mut T
§
Examples
use
std::cell::Cell;
let
slice:
&mut
[i32] =
&mut
[
1
,
2
,
3
];
let
cell_slice:
&
Cell<[i32]> = Cell::from_mut(slice);
let
slice_cell:
&
[Cell<i32>] = cell_slice.as_slice_of_cells();
assert_eq!
(slice_cell.len(),
3
);
Source
§
impl<T>
Cell
<T>
where
    T:
Default
,
1.17.0
·
Source
pub fn
take
(&self) -> T
Takes the value of the cell, leaving
Default::default()
in its place.
§
Examples
use
std::cell::Cell;
let
c = Cell::new(
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
Source
§
impl<T>
Cell
<
[T]
>
1.37.0 (const:
unstable
)
·
Source
pub fn
as_slice_of_cells
(&self) -> &[
Cell
<T>]
Returns a
&[Cell<T>]
from a
&Cell<[T]>
§
Examples
use
std::cell::Cell;
let
slice:
&mut
[i32] =
&mut
[
1
,
2
,
3
];
let
cell_slice:
&
Cell<[i32]> = Cell::from_mut(slice);
let
slice_cell:
&
[Cell<i32>] = cell_slice.as_slice_of_cells();
assert_eq!
(slice_cell.len(),
3
);
Source
§
impl<T, const N:
usize
>
Cell
<
[T; N]
>
Source
pub const fn
as_array_of_cells
(&self) -> &[
Cell
<T>;
N
]
🔬
This is a nightly-only experimental API. (
as_array_of_cells
#88248
)
Returns a
&[Cell<T>; N]
from a
&Cell<[T; N]>
§
Examples
#![feature(as_array_of_cells)]
use
std::cell::Cell;
let
mut
array: [i32;
3
] = [
1
,
2
,
3
];
let
cell_array:
&
Cell<[i32;
3
]> = Cell::from_mut(
&mut
array);
let
array_cell:
&
[Cell<i32>;
3
] = cell_array.as_array_of_cells();
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
Cell
<T>
where
    T:
Copy
,
Source
§
fn
clone
(&self) ->
Cell
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
Cell
<T>
where
    T:
Copy
+
Debug
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
Cell
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
Cell
<T>
Creates a
Cell<T>
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
Cell
<T>
Source
§
fn
from
(t: T) ->
Cell
<T>
Creates a new
Cell<T>
containing the given value.
1.10.0
·
Source
§
impl<T>
Ord
for
Cell
<T>
where
    T:
Ord
+
Copy
,
Source
§
fn
cmp
(&self, other: &
Cell
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
Cell
<T>
where
    T:
PartialEq
+
Copy
,
Source
§
fn
eq
(&self, other: &
Cell
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
1.10.0
·
Source
§
impl<T>
PartialOrd
for
Cell
<T>
where
    T:
PartialOrd
+
Copy
,
Source
§
fn
partial_cmp
(&self, other: &
Cell
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
Source
§
fn
lt
(&self, other: &
Cell
<T>) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
Source
§
fn
le
(&self, other: &
Cell
<T>) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
Source
§
fn
gt
(&self, other: &
Cell
<T>) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
Source
§
fn
ge
(&self, other: &
Cell
<T>) ->
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
Source
§
impl<T, U>
CoerceUnsized
<
Cell
<U>> for
Cell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
DispatchFromDyn
<
Cell
<U>> for
Cell
<T>
where
    T:
DispatchFromDyn
<U>,
1.2.0
·
Source
§
impl<T>
Eq
for
Cell
<T>
where
    T:
Eq
+
Copy
,
Source
§
impl<T>
PinCoerceUnsized
for
Cell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PointerLike
for
Cell
<T>
where
    T:
PointerLike
,
1.0.0
·
Source
§
impl<T>
Send
for
Cell
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
Cell
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
Cell
<T>
§
impl<T> !
RefUnwindSafe
for
Cell
<T>
§
impl<T>
Unpin
for
Cell
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
Cell
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