c_void in std::ffi - Rust
std
::
ffi
Enum
c_void
Copy item path
1.30.0
·
Source
#[repr(u8)]
pub enum c_void {
// some variants omitted
}
Expand description
Equivalent to C’s
void
type when used as a
pointer
.
In essence,
*const c_void
is equivalent to C’s
const void*
and
*mut c_void
is equivalent to C’s
void*
. That said, this is
not
the same as C’s
void
return type, which is Rust’s
()
type.
To model pointers to opaque types in FFI, until
extern type
is
stabilized, it is recommended to use a newtype wrapper around an empty
byte array. See the
Nomicon
for details.
One could use
std::os::raw::c_void
if they want to support old Rust
compiler down to 1.1.0. After Rust 1.30.0, it was re-exported by
this definition. For more information, please read
RFC 2521
.
Trait Implementations
§
1.16.0
·
Source
§
impl
Debug
for
c_void
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
Auto Trait Implementations
§
§
impl
Freeze
for
c_void
§
impl
RefUnwindSafe
for
c_void
§
impl
Send
for
c_void
§
impl
Sync
for
c_void
§
impl
Unpin
for
c_void
§
impl
UnwindSafe
for
c_void
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