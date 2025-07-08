ProcThreadAttributeListBuilder in std::os::windows::process - Rust
std
::
os
::
windows
::
process
Struct
ProcThreadAttributeListBuilder
Copy item path
Source
pub struct ProcThreadAttributeListBuilder<'a> {
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
Builder for constructing a
ProcThreadAttributeList
.
Implementations
§
Source
§
impl<'a>
ProcThreadAttributeListBuilder
<'a>
Source
pub fn
attribute
<T>(self, attribute:
usize
, value:
&'a T
) -> Self
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Sets an attribute on the attribute list.
The
attribute
parameter specifies the raw attribute to be set, while
the
value
parameter holds the value associated with that attribute.
Please refer to the
Windows documentation
for a list of valid attributes.
§
Note
The maximum number of attributes is the value of
u32::MAX
. If this
limit is exceeded, the call to
Self::finish
will return an
Error
indicating that the maximum number of attributes has been exceeded.
§
Safety Note
Remember that improper use of attributes can lead to undefined behavior
or security vulnerabilities. Always consult the documentation and ensure
proper attribute values are used.
Source
pub unsafe fn
raw_attribute
<T>(
    self,
    attribute:
usize
,
    value_ptr:
*const T
,
    value_size:
usize
,
) -> Self
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Sets a raw attribute on the attribute list.
This function is useful for setting attributes with pointers or sizes
that cannot be derived directly from their values.
§
Safety
This function is marked as
unsafe
because it deals with raw pointers
and sizes. It is the responsibility of the caller to ensure the value
lives longer than the resulting
ProcThreadAttributeList
as well as
the validity of the size parameter.
§
Example
#![feature(windows_process_extensions_raw_attribute)]
use
std::ffi::c_void;
use
std::os::windows::process::{CommandExt, ProcThreadAttributeList};
use
std::os::windows::raw::HANDLE;
use
std::process::Command;
#[repr(C)]
pub struct
COORD {
pub
X: i16,
pub
Y: i16,
}
unsafe extern
"system"
{
fn
CreatePipe(
        hreadpipe:
*mut
HANDLE,
        hwritepipe:
*mut
HANDLE,
        lppipeattributes:
*const
c_void,
        nsize: u32,
    ) -> i32;
fn
CreatePseudoConsole(
        size: COORD,
        hinput: HANDLE,
        houtput: HANDLE,
        dwflags: u32,
        phpc:
*mut
isize,
    ) -> i32;
fn
CloseHandle(hobject: HANDLE) -> i32;
}
let
[
mut
input_read_side,
mut
output_write_side,
mut
output_read_side,
mut
input_write_side] =
    [
unsafe
{ std::mem::zeroed::<HANDLE>() };
4
];
unsafe
{
    CreatePipe(
&mut
input_read_side,
&mut
input_write_side, std::ptr::null(),
0
);
    CreatePipe(
&mut
output_read_side,
&mut
output_write_side, std::ptr::null(),
0
);
}
let
size = COORD { X:
60
, Y:
40
};
let
mut
h_pc =
unsafe
{ std::mem::zeroed() };
unsafe
{ CreatePseudoConsole(size, input_read_side, output_write_side,
0
,
&mut
h_pc) };
unsafe
{ CloseHandle(input_read_side) };
unsafe
{ CloseHandle(output_write_side) };
const
PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: usize =
131094
;
let
attribute_list =
unsafe
{
    ProcThreadAttributeList::build()
        .raw_attribute(
            PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE,
            h_pc
as
*const
c_void,
            size_of::<isize>(),
        )
        .finish()
?
};
let
mut
child = Command::new(
"cmd"
).spawn_with_attributes(
&
attribute_list)
?
;
Source
pub fn
finish
(&self) ->
Result
<
ProcThreadAttributeList
<'a>>
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Finalizes the construction of the
ProcThreadAttributeList
.
§
Errors
Returns an error if the maximum number of attributes is exceeded
or if there is an I/O error during initialization.
Trait Implementations
§
Source
§
impl<'a>
Clone
for
ProcThreadAttributeListBuilder
<'a>
Source
§
fn
clone
(&self) ->
ProcThreadAttributeListBuilder
<'a>
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
impl<'a>
Debug
for
ProcThreadAttributeListBuilder
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
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
RefUnwindSafe
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a> !
Send
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a> !
Sync
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
Unpin
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
UnwindSafe
for
ProcThreadAttributeListBuilder
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