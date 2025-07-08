LazyCell in std::cell - Rust
std
::
cell
Struct
LazyCell
Copy item path
1.80.0
·
Source
pub struct LazyCell<T, F =
fn
() -> T> {
/* private fields */
}
Expand description
A value which is initialized on the first access.
For a thread-safe version of this struct, see
std::sync::LazyLock
.
§
Examples
use
std::cell::LazyCell;
let
lazy: LazyCell<i32> = LazyCell::new(|| {
println!
(
"initializing"
);
92
});
println!
(
"ready"
);
println!
(
"{}"
,
*
lazy);
println!
(
"{}"
,
*
lazy);
// Prints:
//   ready
//   initializing
//   92
//   92
Implementations
§
Source
§
impl<T, F>
LazyCell
<T, F>
where
    F:
FnOnce
() -> T,
1.80.0 (const: 1.80.0)
·
Source
pub const fn
new
(f: F) ->
LazyCell
<T, F>
Creates a new lazy value with the given initializing function.
§
Examples
use
std::cell::LazyCell;
let
hello =
"Hello, World!"
.to_string();
let
lazy = LazyCell::new(|| hello.to_uppercase());
assert_eq!
(
&*
lazy,
"HELLO, WORLD!"
);
Source
pub const fn
into_inner
(this:
LazyCell
<T, F>) ->
Result
<T, F>
🔬
This is a nightly-only experimental API. (
lazy_cell_into_inner
#125623
)
Consumes this
LazyCell
returning the stored value.
Returns
Ok(value)
if
Lazy
is initialized and
Err(f)
otherwise.
§
Examples
#![feature(lazy_cell_into_inner)]
use
std::cell::LazyCell;
let
hello =
"Hello, World!"
.to_string();
let
lazy = LazyCell::new(|| hello.to_uppercase());
assert_eq!
(
&*
lazy,
"HELLO, WORLD!"
);
assert_eq!
(LazyCell::into_inner(lazy).ok(),
Some
(
"HELLO, WORLD!"
.to_string()));
1.80.0
·
Source
pub fn
force
(this: &
LazyCell
<T, F>) ->
&T
Forces the evaluation of this lazy value and returns a reference to
the result.
This is equivalent to the
Deref
impl, but is explicit.
§
Examples
use
std::cell::LazyCell;
let
lazy = LazyCell::new(||
92
);
assert_eq!
(LazyCell::force(
&
lazy),
&
92
);
assert_eq!
(
&*
lazy,
&
92
);
Source
pub fn
force_mut
(this: &mut
LazyCell
<T, F>) ->
&mut T
🔬
This is a nightly-only experimental API. (
lazy_get
#129333
)
Forces the evaluation of this lazy value and returns a mutable reference to
the result.
§
Examples
#![feature(lazy_get)]
use
std::cell::LazyCell;
let
mut
lazy = LazyCell::new(||
92
);
let
p = LazyCell::force_mut(
&mut
lazy);
assert_eq!
(
*
p,
92
);
*
p =
44
;
assert_eq!
(
*
lazy,
44
);
Source
§
impl<T, F>
LazyCell
<T, F>
Source
pub fn
get_mut
(this: &mut
LazyCell
<T, F>) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
lazy_get
#129333
)
Returns a mutable reference to the value if initialized, or
None
if not.
§
Examples
#![feature(lazy_get)]
use
std::cell::LazyCell;
let
mut
lazy = LazyCell::new(||
92
);
assert_eq!
(LazyCell::get_mut(
&mut
lazy),
None
);
let _
= LazyCell::force(
&
lazy);
*
LazyCell::get_mut(
&mut
lazy).unwrap() =
44
;
assert_eq!
(
*
lazy,
44
);
Source
pub fn
get
(this: &
LazyCell
<T, F>) ->
Option
<
&T
>
🔬
This is a nightly-only experimental API. (
lazy_get
#129333
)
Returns a reference to the value if initialized, or
None
if not.
§
Examples
#![feature(lazy_get)]
use
std::cell::LazyCell;
let
lazy = LazyCell::new(||
92
);
assert_eq!
(LazyCell::get(
&
lazy),
None
);
let _
= LazyCell::force(
&
lazy);
assert_eq!
(LazyCell::get(
&
lazy),
Some
(
&
92
));
Trait Implementations
§
1.80.0
·
Source
§
impl<T, F>
Debug
for
LazyCell
<T, F>
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
1.80.0
·
Source
§
impl<T>
Default
for
LazyCell
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
LazyCell
<T>
Creates a new lazy value using
Default
as the initializing function.
1.80.0
·
Source
§
impl<T, F>
Deref
for
LazyCell
<T, F>
where
    F:
FnOnce
() -> T,
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
Auto Trait Implementations
§
§
impl<T, F =
fn
() -> T> !
Freeze
for
LazyCell
<T, F>
§
impl<T, F =
fn
() -> T> !
RefUnwindSafe
for
LazyCell
<T, F>
§
impl<T, F>
Send
for
LazyCell
<T, F>
where
    F:
Send
,
    T:
Send
,
§
impl<T, F =
fn
() -> T> !
Sync
for
LazyCell
<T, F>
§
impl<T, F>
Unpin
for
LazyCell
<T, F>
where
    F:
Unpin
,
    T:
Unpin
,
§
impl<T, F>
UnwindSafe
for
LazyCell
<T, F>
where
    F:
UnwindSafe
,
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