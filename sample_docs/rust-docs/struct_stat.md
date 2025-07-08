stat in std::os::macos::raw - Rust
std
::
os
::
macos
::
raw
Struct
stat
Copy item path
1.1.0
·
Source
#[repr(C)]
pub struct stat {
Show 22 fields
pub st_dev:
i32
,
    pub st_mode:
u16
,
    pub st_nlink:
u16
,
    pub st_ino:
u64
,
    pub st_uid:
u32
,
    pub st_gid:
u32
,
    pub st_rdev:
i32
,
    pub st_atime:
c_long
,
    pub st_atime_nsec:
c_long
,
    pub st_mtime:
c_long
,
    pub st_mtime_nsec:
c_long
,
    pub st_ctime:
c_long
,
    pub st_ctime_nsec:
c_long
,
    pub st_birthtime:
c_long
,
    pub st_birthtime_nsec:
c_long
,
    pub st_size:
i64
,
    pub st_blocks:
i64
,
    pub st_blksize:
i32
,
    pub st_flags:
u32
,
    pub st_gen:
u32
,
    pub st_lspare:
i32
,
    pub st_qspare: [
i64
;
2
],
}
Available on
Apple
only.
Fields
§
§
st_dev:
i32
§
st_mode:
u16
§
st_nlink:
u16
§
st_ino:
u64
§
st_uid:
u32
§
st_gid:
u32
§
st_rdev:
i32
§
st_atime:
c_long
§
st_atime_nsec:
c_long
§
st_mtime:
c_long
§
st_mtime_nsec:
c_long
§
st_ctime:
c_long
§
st_ctime_nsec:
c_long
§
st_birthtime:
c_long
§
st_birthtime_nsec:
c_long
§
st_size:
i64
§
st_blocks:
i64
§
st_blksize:
i32
§
st_flags:
u32
§
st_gen:
u32
§
st_lspare:
i32
§
st_qspare: [
i64
;
2
]
Trait Implementations
§
1.1.0
·
Source
§
impl
Clone
for
stat
Source
§
fn
clone
(&self) ->
stat
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
Auto Trait Implementations
§
§
impl
Freeze
for
stat
§
impl
RefUnwindSafe
for
stat
§
impl
Send
for
stat
§
impl
Sync
for
stat
§
impl
Unpin
for
stat
§
impl
UnwindSafe
for
stat
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