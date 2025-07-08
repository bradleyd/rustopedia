FileTimes in std::fs - Rust
std
::
fs
Struct
FileTimes
Copy item path
1.75.0
·
Source
pub struct FileTimes(
/* private fields */
);
Expand description
Representation of the various timestamps on a file.
Implementations
§
Source
§
impl
FileTimes
1.75.0
·
Source
pub fn
new
() -> Self
Creates a new
FileTimes
with no times set.
Using the resulting
FileTimes
in
File::set_times
will not modify any timestamps.
1.75.0
·
Source
pub fn
set_accessed
(self, t:
SystemTime
) -> Self
Set the last access time of a file.
1.75.0
·
Source
pub fn
set_modified
(self, t:
SystemTime
) -> Self
Set the last modified time of a file.
Trait Implementations
§
1.75.0
·
Source
§
impl
Clone
for
FileTimes
Source
§
fn
clone
(&self) ->
FileTimes
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
1.75.0
·
Source
§
impl
Debug
for
FileTimes
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
1.75.0
·
Source
§
impl
Default
for
FileTimes
Source
§
fn
default
() ->
FileTimes
Returns the “default value” for a type.
Read more
1.75.0
·
Source
§
impl
FileTimesExt
for
FileTimes
Available on
Apple
only.
Source
§
fn
set_created
(self, t:
SystemTime
) -> Self
Set the creation time of a file.
1.75.0
·
Source
§
impl
FileTimesExt
for
FileTimes
Available on
Windows
only.
Source
§
fn
set_created
(self, t:
SystemTime
) -> Self
Set the creation time of a file.
1.75.0
·
Source
§
impl
Copy
for
FileTimes
Auto Trait Implementations
§
§
impl
Freeze
for
FileTimes
§
impl
RefUnwindSafe
for
FileTimes
§
impl
Send
for
FileTimes
§
impl
Sync
for
FileTimes
§
impl
Unpin
for
FileTimes
§
impl
UnwindSafe
for
FileTimes
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