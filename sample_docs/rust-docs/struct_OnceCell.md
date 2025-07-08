OnceCell in std::cell - Rust
std
::
cell
Struct
OnceCell
Copy item path
1.70.0
·
Source
pub struct OnceCell<T> {
/* private fields */
}
Expand description
A cell which can nominally be written to only once.
This allows obtaining a shared
&T
reference to its inner value without copying or replacing
it (unlike
Cell
), and without runtime borrow checks (unlike
RefCell
). However,
only immutable references can be obtained unless one has a mutable reference to the cell
itself. In the same vein, the cell can only be re-initialized with such a mutable reference.
A
OnceCell
can be thought of as a safe abstraction over uninitialized data that becomes
initialized once written.
For a thread-safe version of this struct, see
std::sync::OnceLock
.
§
Examples
use
std::cell::OnceCell;
let
cell = OnceCell::new();
assert!
(cell.get().is_none());
let
value:
&
String = cell.get_or_init(|| {
"Hello, World!"
.to_string()
});
assert_eq!
(value,
"Hello, World!"
);
assert!
(cell.get().is_some());
Implementations
§
Source
§
impl<T>
OnceCell
<T>
1.70.0 (const: 1.70.0)
·
Source
pub const fn
new
() ->
OnceCell
<T>
Creates a new uninitialized cell.
1.70.0
·
Source
pub fn
get
(&self) ->
Option
<
&T
>
Gets the reference to the underlying value.
Returns
None
if the cell is uninitialized.
1.70.0
·
Source
pub fn
get_mut
(&mut self) ->
Option
<
&mut T
>
Gets the mutable reference to the underlying value.
Returns
None
if the cell is uninitialized.
1.70.0
·
Source
pub fn
set
(&self, value: T) ->
Result
<
()
, T>
Initializes the contents of the cell to
value
.
§
Errors
This method returns
Ok(())
if the cell was uninitialized
and
Err(value)
if it was already initialized.
§
Examples
use
std::cell::OnceCell;
let
cell = OnceCell::new();
assert!
(cell.get().is_none());
assert_eq!
(cell.set(
92
),
Ok
(()));
assert_eq!
(cell.set(
62
),
Err
(
62
));
assert!
(cell.get().is_some());
Source
pub fn
try_insert
(&self, value: T) ->
Result
<
&T
, (
&T
, T)>
🔬
This is a nightly-only experimental API. (
once_cell_try_insert
#116693
)
Initializes the contents of the cell to
value
if the cell was
uninitialized, then returns a reference to it.
§
Errors
This method returns
Ok(&value)
if the cell was uninitialized
and
Err((&current_value, value))
if it was already initialized.
§
Examples
#![feature(once_cell_try_insert)]
use
std::cell::OnceCell;
let
cell = OnceCell::new();
assert!
(cell.get().is_none());
assert_eq!
(cell.try_insert(
92
),
Ok
(
&
92
));
assert_eq!
(cell.try_insert(
62
),
Err
((
&
92
,
62
)));
assert!
(cell.get().is_some());
1.70.0
·
Source
pub fn
get_or_init
<F>(&self, f: F) ->
&T
where
    F:
FnOnce
() -> T,
Gets the contents of the cell, initializing it to
f()
if the cell was uninitialized.
§
Panics
If
f()
panics, the panic is propagated to the caller, and the cell
remains uninitialized.
It is an error to reentrantly initialize the cell from
f
. Doing
so results in a panic.
§
Examples
use
std::cell::OnceCell;
let
cell = OnceCell::new();
let
value = cell.get_or_init(||
92
);
assert_eq!
(value,
&
92
);
let
value = cell.get_or_init(||
unreachable!
());
assert_eq!
(value,
&
92
);
Source
pub fn
get_mut_or_init
<F>(&mut self, f: F) ->
&mut T
where
    F:
FnOnce
() -> T,
🔬
This is a nightly-only experimental API. (
once_cell_get_mut
#121641
)
Gets the mutable reference of the contents of the cell,
initializing it to
f()
if the cell was uninitialized.
§
Panics
If
f()
panics, the panic is propagated to the caller, and the cell
remains uninitialized.
§
Examples
#![feature(once_cell_get_mut)]
use
std::cell::OnceCell;
let
mut
cell = OnceCell::new();
let
value = cell.get_mut_or_init(||
92
);
assert_eq!
(
*
value,
92
);
*
value +=
2
;
assert_eq!
(
*
value,
94
);
let
value = cell.get_mut_or_init(||
unreachable!
());
assert_eq!
(
*
value,
94
);
Source
pub fn
get_or_try_init
<F, E>(&self, f: F) ->
Result
<
&T
, E>
where
    F:
FnOnce
() ->
Result
<T, E>,
🔬
This is a nightly-only experimental API. (
once_cell_try
#109737
)
Gets the contents of the cell, initializing it to
f()
if
the cell was uninitialized. If the cell was uninitialized
and
f()
failed, an error is returned.
§
Panics
If
f()
panics, the panic is propagated to the caller, and the cell
remains uninitialized.
It is an error to reentrantly initialize the cell from
f
. Doing
so results in a panic.
§
Examples
#![feature(once_cell_try)]
use
std::cell::OnceCell;
let
cell = OnceCell::new();
assert_eq!
(cell.get_or_try_init(||
Err
(())),
Err
(()));
assert!
(cell.get().is_none());
let
value = cell.get_or_try_init(|| ->
Result
<i32, ()> {
Ok
(
92
)
});
assert_eq!
(value,
Ok
(
&
92
));
assert_eq!
(cell.get(),
Some
(
&
92
))
Source
pub fn
get_mut_or_try_init
<F, E>(&mut self, f: F) ->
Result
<
&mut T
, E>
where
    F:
FnOnce
() ->
Result
<T, E>,
🔬
This is a nightly-only experimental API. (
once_cell_get_mut
#121641
)
Gets the mutable reference of the contents of the cell, initializing
it to
f()
if the cell was uninitialized. If the cell was uninitialized
and
f()
failed, an error is returned.
§
Panics
If
f()
panics, the panic is propagated to the caller, and the cell
remains uninitialized.
§
Examples
#![feature(once_cell_get_mut)]
use
std::cell::OnceCell;
let
mut
cell: OnceCell<u32> = OnceCell::new();
// Failed attempts to initialize the cell do not change its contents
assert!
(cell.get_mut_or_try_init(||
"not a number!"
.parse()).is_err());
assert!
(cell.get().is_none());
let
value = cell.get_mut_or_try_init(||
"1234"
.parse());
assert_eq!
(value,
Ok
(
&mut
1234
));
let
Ok
(value) = value
else
{
return
; };
*
value +=
2
;
assert_eq!
(cell.get(),
Some
(
&
1236
))
1.70.0 (const: 1.83.0)
·
Source
pub const fn
into_inner
(self) ->
Option
<T>
Consumes the cell, returning the wrapped value.
Returns
None
if the cell was uninitialized.
§
Examples
use
std::cell::OnceCell;
let
cell: OnceCell<String> = OnceCell::new();
assert_eq!
(cell.into_inner(),
None
);
let
cell = OnceCell::new();
let _
= cell.set(
"hello"
.to_owned());
assert_eq!
(cell.into_inner(),
Some
(
"hello"
.to_owned()));
1.70.0
·
Source
pub fn
take
(&mut self) ->
Option
<T>
Takes the value out of this
OnceCell
, moving it back to an uninitialized state.
Has no effect and returns
None
if the
OnceCell
is uninitialized.
Safety is guaranteed by requiring a mutable reference.
§
Examples
use
std::cell::OnceCell;
let
mut
cell: OnceCell<String> = OnceCell::new();
assert_eq!
(cell.take(),
None
);
let
mut
cell = OnceCell::new();
let _
= cell.set(
"hello"
.to_owned());
assert_eq!
(cell.take(),
Some
(
"hello"
.to_owned()));
assert_eq!
(cell.get(),
None
);
Trait Implementations
§
1.70.0
·
Source
§
impl<T>
Clone
for
OnceCell
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
OnceCell
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
1.70.0
·
Source
§
impl<T>
Debug
for
OnceCell
<T>
where
    T:
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
1.70.0
·
Source
§
impl<T>
Default
for
OnceCell
<T>
Source
§
fn
default
() ->
OnceCell
<T>
Returns the “default value” for a type.
Read more
1.70.0
·
Source
§
impl<T>
From
<T> for
OnceCell
<T>
Source
§
fn
from
(value: T) ->
OnceCell
<T>
Creates a new
OnceCell<T>
which already contains the given
value
.
1.70.0
·
Source
§
impl<T>
PartialEq
for
OnceCell
<T>
where
    T:
PartialEq
,
Source
§
fn
eq
(&self, other: &
OnceCell
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
1.70.0
·
Source
§
impl<T>
Eq
for
OnceCell
<T>
where
    T:
Eq
,
1.70.0
·
Source
§
impl<T> !
Sync
for
OnceCell
<T>
Auto Trait Implementations
§
§
impl<T> !
Freeze
for
OnceCell
<T>
§
impl<T> !
RefUnwindSafe
for
OnceCell
<T>
§
impl<T>
Send
for
OnceCell
<T>
where
    T:
Send
,
§
impl<T>
Unpin
for
OnceCell
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
OnceCell
<T>
where
    T:
UnwindSafe
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