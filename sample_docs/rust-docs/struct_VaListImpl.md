VaListImpl in std::ffi - Rust
std
::
ffi
Struct
VaListImpl
Copy item path
Source
pub struct VaListImpl<'f> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
c_variadic
#44930
)
Expand description
Basic implementation of a
va_list
.
Implementations
§
Source
§
impl<'f>
VaListImpl
<'f>
Source
pub fn
as_va_list
<'a>(&'a mut self) ->
VaList
<'a, 'f>
🔬
This is a nightly-only experimental API. (
c_variadic
#44930
)
Converts a
VaListImpl
into a
VaList
that is binary-compatible with C’s
va_list
.
Source
§
impl<'f>
VaListImpl
<'f>
Source
pub unsafe fn
arg
<T>(&mut self) -> T
where
    T: VaArgSafe,
🔬
This is a nightly-only experimental API. (
c_variadic
#44930
)
Advance to the next arg.
Source
pub unsafe fn
with_copy
<F, R>(&self, f: F) -> R
where
    F: for<'copy>
FnOnce
(
VaList
<'copy, 'f>) -> R,
🔬
This is a nightly-only experimental API. (
c_variadic
#44930
)
Copies the
va_list
at the current location.
Trait Implementations
§
Source
§
impl<'f>
Clone
for
VaListImpl
<'f>
Source
§
fn
clone
(&self) ->
VaListImpl
<'f>
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
Source
§
impl<'f>
Debug
for
VaListImpl
<'f>
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
impl<'f>
Drop
for
VaListImpl
<'f>
Source
§
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
Auto Trait Implementations
§
§
impl<'f>
Freeze
for
VaListImpl
<'f>
§
impl<'f>
RefUnwindSafe
for
VaListImpl
<'f>
§
impl<'f> !
Send
for
VaListImpl
<'f>
§
impl<'f> !
Sync
for
VaListImpl
<'f>
§
impl<'f>
Unpin
for
VaListImpl
<'f>
§
impl<'f> !
UnwindSafe
for
VaListImpl
<'f>
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