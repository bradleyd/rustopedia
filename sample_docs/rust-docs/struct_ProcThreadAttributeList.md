ProcThreadAttributeList in std::os::windows::process - Rust
std
::
os
::
windows
::
process
Struct
ProcThreadAttributeList
Copy item path
Source
pub struct ProcThreadAttributeList<'a> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Available on
Windows
only.
Expand description
A wrapper around windows
ProcThreadAttributeList
.
Implementations
§
Source
§
impl<'a>
ProcThreadAttributeList
<'a>
Source
pub fn
build
() ->
ProcThreadAttributeListBuilder
<'a>
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Creates a new builder for constructing a
ProcThreadAttributeList
.
Trait Implementations
§
Source
§
impl<'a>
Debug
for
ProcThreadAttributeList
<'a>
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
Source
§
impl<'a>
Drop
for
ProcThreadAttributeList
<'a>
Source
§
fn
drop
(&mut self)
Deletes the attribute list.
This method calls
DeleteProcThreadAttributeList
to delete the
underlying attribute list.
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
ProcThreadAttributeList
<'a>
§
impl<'a>
RefUnwindSafe
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Send
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Sync
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Unpin
for
ProcThreadAttributeList
<'a>
§
impl<'a>
UnwindSafe
for
ProcThreadAttributeList
<'a>
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