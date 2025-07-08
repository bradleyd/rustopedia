System in std::alloc - Rust
std
::
alloc
Struct
System
Copy item path
1.28.0
·
Source
pub struct System;
Expand description
The default memory allocator provided by the operating system.
This is based on
malloc
on Unix platforms and
HeapAlloc
on Windows,
plus related functions. However, it is not valid to mix use of the backing
system allocator with
System
, as this implementation may include extra
work, such as to serve alignment requests greater than the alignment
provided directly by the backing system allocator.
This type implements the
GlobalAlloc
trait. Currently the default
global allocator is unspecified. Libraries, however, like
cdylib
s and
staticlib
s are guaranteed to use the
System
by default and as such
work as if they had this definition:
use
std::alloc::System;
#[global_allocator]
static
A: System = System;
fn
main() {
let
a = Box::new(
4
);
// Allocates from the system allocator.
println!
(
"{a}"
);
}
You can also define your own wrapper around
System
if you’d like, such as
keeping track of the number of all bytes allocated:
use
std::alloc::{System, GlobalAlloc, Layout};
use
std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
struct
Counter;
static
ALLOCATED: AtomicUsize = AtomicUsize::new(
0
);
unsafe impl
GlobalAlloc
for
Counter {
unsafe fn
alloc(
&
self
, layout: Layout) ->
*mut
u8 {
let
ret =
unsafe
{ System.alloc(layout) };
if
!ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Relaxed);
        }
        ret
    }
unsafe fn
dealloc(
&
self
, ptr:
*mut
u8, layout: Layout) {
unsafe
{ System.dealloc(ptr, layout); }
        ALLOCATED.fetch_sub(layout.size(), Relaxed);
    }
}
#[global_allocator]
static
A: Counter = Counter;
fn
main() {
println!
(
"allocated bytes before main: {}"
, ALLOCATED.load(Relaxed));
}
It can also be used directly to allocate memory independently of whatever
global allocator has been selected for a Rust program. For example if a Rust
program opts in to using jemalloc as the global allocator,
System
will
still allocate memory using
malloc
and
HeapAlloc
.
Trait Implementations
§
Source
§
impl
Allocator
for
System
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
1.28.0
·
Source
§
impl
Clone
for
System
Source
§
fn
clone
(&self) ->
System
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
1.28.0
·
Source
§
impl
Debug
for
System
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
1.28.0
·
Source
§
impl
Default
for
System
Source
§
fn
default
() ->
System
Returns the “default value” for a type.
Read more
1.28.0
·
Source
§
impl
GlobalAlloc
for
System
Source
§
unsafe fn
alloc
(&self, layout:
Layout
) ->
*mut
u8
Allocates memory as described by the given
layout
.
Read more
Source
§
unsafe fn
alloc_zeroed
(&self, layout:
Layout
) ->
*mut
u8
Behaves like
alloc
, but also ensures that the contents
are set to zero before being returned.
Read more
Source
§
unsafe fn
dealloc
(&self, ptr:
*mut
u8
, _layout:
Layout
)
Deallocates the block of memory at the given
ptr
pointer with the given
layout
.
Read more
Source
§
unsafe fn
realloc
(
    &self,
    ptr:
*mut
u8
,
    layout:
Layout
,
    new_size:
usize
,
) ->
*mut
u8
Shrinks or grows a block of memory to the given
new_size
in bytes.
The block is described by the given
ptr
pointer and
layout
.
Read more
1.28.0
·
Source
§
impl
Copy
for
System
Auto Trait Implementations
§
§
impl
Freeze
for
System
§
impl
RefUnwindSafe
for
System
§
impl
Send
for
System
§
impl
Sync
for
System
§
impl
Unpin
for
System
§
impl
UnwindSafe
for
System
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