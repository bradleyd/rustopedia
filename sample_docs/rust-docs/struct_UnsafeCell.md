UnsafeCell in std::cell - Rust
std
::
cell
Struct
UnsafeCell
Copy item path
1.0.0
·
Source
pub struct UnsafeCell<T>
where
    T: ?
Sized
,
{
/* private fields */
}
Expand description
The core primitive for interior mutability in Rust.
If you have a reference
&T
, then normally in Rust the compiler performs optimizations based on
the knowledge that
&T
points to immutable data. Mutating that data, for example through an
alias or by transmuting a
&T
into a
&mut T
, is considered undefined behavior.
UnsafeCell<T>
opts-out of the immutability guarantee for
&T
: a shared reference
&UnsafeCell<T>
may point to data that is being mutated. This is called “interior mutability”.
All other types that allow internal mutability, such as
Cell<T>
and
RefCell<T>
, internally
use
UnsafeCell
to wrap their data.
Note that only the immutability guarantee for shared references is affected by
UnsafeCell
. The
uniqueness guarantee for mutable references is unaffected. There is
no
legal way to obtain
aliasing
&mut
, not even with
UnsafeCell<T>
.
UnsafeCell
does nothing to avoid data races; they are still undefined behavior. If multiple
threads have access to the same
UnsafeCell
, they must follow the usual rules of the
concurrent memory model
: conflicting non-synchronized accesses must be done via the APIs in
core::sync::atomic
.
The
UnsafeCell
API itself is technically very simple:
.get()
gives you a raw pointer
*mut T
to its contents. It is up to
you
as the abstraction designer to use that raw pointer
correctly.
The precise Rust aliasing rules are somewhat in flux, but the main points are not contentious:
If you create a safe reference with lifetime
'a
(either a
&T
or
&mut T
reference), then
you must not access the data in any way that contradicts that reference for the remainder of
'a
. For example, this means that if you take the
*mut T
from an
UnsafeCell<T>
and cast it
to an
&T
, then the data in
T
must remain immutable (modulo any
UnsafeCell
data found
within
T
, of course) until that reference’s lifetime expires. Similarly, if you create a
&mut T
reference that is released to safe code, then you must not access the data within the
UnsafeCell
until that reference expires.
For both
&T
without
UnsafeCell<_>
and
&mut T
, you must also not deallocate the data
until the reference expires. As a special exception, given an
&T
, any part of it that is
inside an
UnsafeCell<_>
may be deallocated during the lifetime of the reference, after the
last time the reference is used (dereferenced or reborrowed). Since you cannot deallocate a part
of what a reference points to, this means the memory an
&T
points to can be deallocated only if
every part of it
(including padding) is inside an
UnsafeCell
.
However, whenever a
&UnsafeCell<T>
is constructed or dereferenced, it must still point to
live memory and the compiler is allowed to insert spurious reads if it can prove that this
memory has not yet been deallocated.
To assist with proper design, the following scenarios are explicitly declared legal
for single-threaded code:
A
&T
reference can be released to safe code and there it can co-exist with other
&T
references, but not with a
&mut T
A
&mut T
reference may be released to safe code provided neither other
&mut T
nor
&T
co-exist with it. A
&mut T
must always be unique.
Note that whilst mutating the contents of an
&UnsafeCell<T>
(even while other
&UnsafeCell<T>
references alias the cell) is
ok (provided you enforce the above invariants some other way), it is still undefined behavior
to have multiple
&mut UnsafeCell<T>
aliases. That is,
UnsafeCell
is a wrapper
designed to have a special interaction with
shared
accesses (
i.e.
, through an
&UnsafeCell<_>
reference); there is no magic whatsoever when dealing with
exclusive
accesses (
e.g.
, through a
&mut UnsafeCell<_>
): neither the cell nor the wrapped value
may be aliased for the duration of that
&mut
borrow.
This is showcased by the
.get_mut()
accessor, which is a
safe
getter that yields
a
&mut T
.
§
Memory layout
UnsafeCell<T>
has the same in-memory representation as its inner type
T
. A consequence
of this guarantee is that it is possible to convert between
T
and
UnsafeCell<T>
.
Special care has to be taken when converting a nested
T
inside of an
Outer<T>
type
to an
Outer<UnsafeCell<T>>
type: this is not sound when the
Outer<T>
type enables
niche
optimizations. For example, the type
Option<NonNull<u8>>
is typically 8 bytes large on
64-bit platforms, but the type
Option<UnsafeCell<NonNull<u8>>>
takes up 16 bytes of space.
Therefore this is not a valid conversion, despite
NonNull<u8>
and
UnsafeCell<NonNull<u8>>>
having the same memory layout. This is because
UnsafeCell
disables niche optimizations in
order to avoid its interior mutability property from spreading from
T
into the
Outer
type,
thus this can cause distortions in the type size in these cases.
Note that the only valid way to obtain a
*mut T
pointer to the contents of a
shared
UnsafeCell<T>
is through
.get()
or
.raw_get()
. A
&mut T
reference
can be obtained by either dereferencing this pointer or by calling
.get_mut()
on an
exclusive
UnsafeCell<T>
. Even though
T
and
UnsafeCell<T>
have the
same memory layout, the following is not allowed and undefined behavior:
ⓘ
unsafe fn
not_allowed<T>(ptr:
&
UnsafeCell<T>) ->
&mut
T {
let
t = ptr
as
*const
UnsafeCell<T>
as
*mut
T;
// This is undefined behavior, because the `*mut T` pointer
  // was not obtained through `.get()` nor `.raw_get()`:
unsafe
{
&mut *
t }
}
Instead, do this:
// Safety: the caller must ensure that there are no references that
// point to the *contents* of the `UnsafeCell`.
unsafe fn
get_mut<T>(ptr:
&
UnsafeCell<T>) ->
&mut
T {
unsafe
{
&mut *
ptr.get() }
}
Converting in the other direction from a
&mut T
to an
&UnsafeCell<T>
is allowed:
fn
get_shared<T>(ptr:
&mut
T) ->
&
UnsafeCell<T> {
let
t = ptr
as
*mut
T
as
*const
UnsafeCell<T>;
// SAFETY: `T` and `UnsafeCell<T>` have the same memory layout
unsafe
{
&*
t }
}
§
Examples
Here is an example showcasing how to soundly mutate the contents of an
UnsafeCell<_>
despite
there being multiple references aliasing the cell:
use
std::cell::UnsafeCell;
let
x: UnsafeCell<i32> =
42
.into();
// Get multiple / concurrent / shared references to the same `x`.
let
(p1, p2): (
&
UnsafeCell<i32>,
&
UnsafeCell<i32>) = (
&
x,
&
x);
unsafe
{
// SAFETY: within this scope there are no other references to `x`'s contents,
    // so ours is effectively unique.
let
p1_exclusive:
&mut
i32 =
&mut *
p1.get();
// -- borrow --+
*
p1_exclusive +=
27
;
//                                     |
}
// <---------- cannot go beyond this point -------------------+
unsafe
{
// SAFETY: within this scope nobody expects to have exclusive access to `x`'s contents,
    // so we can have multiple shared accesses concurrently.
let
p2_shared:
&
i32 =
&*
p2.get();
assert_eq!
(
*
p2_shared,
42
+
27
);
let
p1_shared:
&
i32 =
&*
p1.get();
assert_eq!
(
*
p1_shared,
*
p2_shared);
}
The following example showcases the fact that exclusive access to an
UnsafeCell<T>
implies exclusive access to its
T
:
#![forbid(unsafe_code)]
// with exclusive accesses,
                        // `UnsafeCell` is a transparent no-op wrapper,
                        // so no need for `unsafe` here.
use
std::cell::UnsafeCell;
let
mut
x: UnsafeCell<i32> =
42
.into();
// Get a compile-time-checked unique reference to `x`.
let
p_unique:
&mut
UnsafeCell<i32> =
&mut
x;
// With an exclusive reference, we can mutate the contents for free.
*
p_unique.get_mut() =
0
;
// Or, equivalently:
x = UnsafeCell::new(
0
);
// When we own the value, we can extract the contents for free.
let
contents: i32 = x.into_inner();
assert_eq!
(contents,
0
);
Implementations
§
Source
§
impl<T>
UnsafeCell
<T>
1.0.0 (const: 1.32.0)
·
Source
pub const fn
new
(value: T) ->
UnsafeCell
<T>
Constructs a new instance of
UnsafeCell
which will wrap the specified
value.
All access to the inner value through
&UnsafeCell<T>
requires
unsafe
code.
§
Examples
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
1.0.0 (const: 1.83.0)
·
Source
pub const fn
into_inner
(self) -> T
Unwraps the value, consuming the cell.
§
Examples
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
let
five = uc.into_inner();
Source
pub const unsafe fn
replace
(&self, value: T) -> T
🔬
This is a nightly-only experimental API. (
unsafe_cell_access
#136327
)
Replace the value in this
UnsafeCell
and return the old value.
§
Safety
The caller must take care to avoid aliasing and data races.
It is Undefined Behavior to allow calls to race with
any other access to the wrapped value.
It is Undefined Behavior to call this while any other
reference(s) to the wrapped value are alive.
§
Examples
#![feature(unsafe_cell_access)]
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
let
old =
unsafe
{ uc.replace(
10
) };
assert_eq!
(old,
5
);
Source
§
impl<T>
UnsafeCell
<T>
where
    T: ?
Sized
,
1.84.0 (const: 1.84.0)
·
Source
pub const fn
from_mut
(value:
&mut T
) -> &mut
UnsafeCell
<T>
Converts from
&mut T
to
&mut UnsafeCell<T>
.
§
Examples
use
std::cell::UnsafeCell;
let
mut
val =
42
;
let
uc = UnsafeCell::from_mut(
&mut
val);
*
uc.get_mut() -=
1
;
assert_eq!
(
*
uc.get_mut(),
41
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
get
(&self) ->
*mut T
Gets a mutable pointer to the wrapped value.
This can be cast to a pointer of any kind.
Ensure that the access is unique (no active references, mutable or not)
when casting to
&mut T
, and ensure that there are no mutations
or mutable aliases going on when casting to
&T
§
Examples
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
let
five = uc.get();
1.50.0 (const: 1.83.0)
·
Source
pub const fn
get_mut
(&mut self) ->
&mut T
Returns a mutable reference to the underlying data.
This call borrows the
UnsafeCell
mutably (at compile-time) which
guarantees that we possess the only reference.
§
Examples
use
std::cell::UnsafeCell;
let
mut
c = UnsafeCell::new(
5
);
*
c.get_mut() +=
1
;
assert_eq!
(
*
c.get_mut(),
6
);
1.56.0 (const: 1.56.0)
·
Source
pub const fn
raw_get
(this:
*const
UnsafeCell
<T>) ->
*mut T
Gets a mutable pointer to the wrapped value.
The difference from
get
is that this function accepts a raw pointer,
which is useful to avoid the creation of temporary references.
The result can be cast to a pointer of any kind.
Ensure that the access is unique (no active references, mutable or not)
when casting to
&mut T
, and ensure that there are no mutations
or mutable aliases going on when casting to
&T
.
§
Examples
Gradual initialization of an
UnsafeCell
requires
raw_get
, as
calling
get
would require creating a reference to uninitialized data:
use
std::cell::UnsafeCell;
use
std::mem::MaybeUninit;
let
m = MaybeUninit::<UnsafeCell<i32>>::uninit();
unsafe
{ UnsafeCell::raw_get(m.as_ptr()).write(
5
); }
// avoid below which references to uninitialized data
// unsafe { UnsafeCell::get(&*m.as_ptr()).write(5); }
let
uc =
unsafe
{ m.assume_init() };
assert_eq!
(uc.into_inner(),
5
);
Source
pub const unsafe fn
as_ref_unchecked
(&self) ->
&T
🔬
This is a nightly-only experimental API. (
unsafe_cell_access
#136327
)
Get a shared reference to the value within the
UnsafeCell
.
§
Safety
It is Undefined Behavior to call this while any mutable
reference to the wrapped value is alive.
Mutating the wrapped value while the returned
reference is alive is Undefined Behavior.
§
Examples
#![feature(unsafe_cell_access)]
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
let
val =
unsafe
{ uc.as_ref_unchecked() };
assert_eq!
(val,
&
5
);
Source
pub const unsafe fn
as_mut_unchecked
(&self) ->
&mut T
🔬
This is a nightly-only experimental API. (
unsafe_cell_access
#136327
)
Get an exclusive reference to the value within the
UnsafeCell
.
§
Safety
It is Undefined Behavior to call this while any other
reference(s) to the wrapped value are alive.
Mutating the wrapped value through other means while the
returned reference is alive is Undefined Behavior.
§
Examples
#![feature(unsafe_cell_access)]
use
std::cell::UnsafeCell;
let
uc = UnsafeCell::new(
5
);
unsafe
{
*
uc.as_mut_unchecked() +=
1
; }
assert_eq!
(uc.into_inner(),
6
);
Trait Implementations
§
1.9.0
·
Source
§
impl<T>
Debug
for
UnsafeCell
<T>
where
    T: ?
Sized
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
1.10.0
·
Source
§
impl<T>
Default
for
UnsafeCell
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
UnsafeCell
<T>
Creates an
UnsafeCell
, with the
Default
value for T.
1.12.0
·
Source
§
impl<T>
From
<T> for
UnsafeCell
<T>
Source
§
fn
from
(t: T) ->
UnsafeCell
<T>
Creates a new
UnsafeCell<T>
containing the given value.
Source
§
impl<T, U>
CoerceUnsized
<
UnsafeCell
<U>> for
UnsafeCell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
DispatchFromDyn
<
UnsafeCell
<U>> for
UnsafeCell
<T>
where
    T:
DispatchFromDyn
<U>,
Source
§
impl<T> !
Freeze
for
UnsafeCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
UnsafeCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PointerLike
for
UnsafeCell
<T>
where
    T:
PointerLike
,
1.9.0
·
Source
§
impl<T> !
RefUnwindSafe
for
UnsafeCell
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Sync
for
UnsafeCell
<T>
where
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T>
Send
for
UnsafeCell
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Unpin
for
UnsafeCell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
UnsafeCell
<T>
where
    T:
UnwindSafe
+ ?
Sized
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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