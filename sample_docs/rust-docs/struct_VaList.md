VaList in std::ffi - Rust
std
::
ffi
Struct
VaList
Copy item path
Source
pub struct VaList<'a, 'f>
where
    'f: 'a,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
c_variadic
#44930
)
Expand description
A wrapper for a
va_list
Methods from
Deref
<Target =
VaListImpl
<'f>>
§
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
impl<'a, 'f>
Debug
for
VaList
<'a, 'f>
where
    'f: 'a,
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
impl<'a, 'f>
Deref
for
VaList
<'a, 'f>
where
    'f: 'a,
Source
§
type
Target
=
VaListImpl
<'f>
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &
VaListImpl
<'f>
Dereferences the value.
Source
§
impl<'a, 'f>
DerefMut
for
VaList
<'a, 'f>
where
    'f: 'a,
Source
§
fn
deref_mut
(&mut self) -> &mut
VaListImpl
<'f>
Mutably dereferences the value.
Auto Trait Implementations
§
§
impl<'a, 'f>
Freeze
for
VaList
<'a, 'f>
§
impl<'a, 'f>
RefUnwindSafe
for
VaList
<'a, 'f>
§
impl<'a, 'f> !
Send
for
VaList
<'a, 'f>
§
impl<'a, 'f> !
Sync
for
VaList
<'a, 'f>
§
impl<'a, 'f>
Unpin
for
VaList
<'a, 'f>
§
impl<'a, 'f> !
UnwindSafe
for
VaList
<'a, 'f>
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