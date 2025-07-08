Global in std::alloc - Rust
std
::
alloc
Struct
Global
Copy item path
Source
pub struct Global;
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Expand description
The global memory allocator.
This type implements the
Allocator
trait by forwarding calls
to the allocator registered with the
#[global_allocator]
attribute
if there is one, or the
std
crate’s default.
Note: while this type is unstable, the functionality it provides can be
accessed through the
free functions in
alloc
.
Trait Implementations
§
Source
§
impl
Allocator
for
Global
Source
§
fn
allocate
(&self, layout:
Layout
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to allocate a block of memory.
Read more
Source
§
fn
allocate_zeroed
(&self, layout:
Layout
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Behaves like
allocate
, but also ensures that the returned memory is zero-initialized.
Read more
Source
§
unsafe fn
deallocate
(&self, ptr:
NonNull
<
u8
>, layout:
Layout
)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Deallocates the memory referenced by
ptr
.
Read more
Source
§
unsafe fn
grow
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to extend the memory block.
Read more
Source
§
unsafe fn
grow_zeroed
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Behaves like
grow
, but also ensures that the new contents are set to zero before being
returned.
Read more
Source
§
unsafe fn
shrink
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to shrink the memory block.
Read more
Source
§
fn
by_ref
(&self) -> &Self
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates a “by reference” adapter for this instance of
Allocator
.
Read more
Source
§
impl
Clone
for
Global
Source
§
fn
clone
(&self) ->
Global
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
impl
Debug
for
Global
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
impl
Default
for
Global
Source
§
fn
default
() ->
Global
Returns the “default value” for a type.
Read more
Source
§
impl
Copy
for
Global
Auto Trait Implementations
§
§
impl
Freeze
for
Global
§
impl
RefUnwindSafe
for
Global
§
impl
Send
for
Global
§
impl
Sync
for
Global
§
impl
Unpin
for
Global
§
impl
UnwindSafe
for
Global
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