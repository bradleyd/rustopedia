UniqueRc in std::rc - Rust
std
::
rc
Struct
UniqueRc
Copy item path
Source
pub struct UniqueRc<T, A =
Global
>
where
    A:
Allocator
,
    T: ?
Sized
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
unique_rc_arc
#112566
)
Expand description
A uniquely owned
Rc
.
This represents an
Rc
that is known to be uniquely owned – that is, have exactly one strong
reference. Multiple weak pointers can be created, but attempts to upgrade those to strong
references will fail unless the
UniqueRc
they point to has been converted into a regular
Rc
.
Because they are uniquely owned, the contents of a
UniqueRc
can be freely mutated. A common
use case is to have an object be mutable during its initialization phase but then have it become
immutable and converted to a normal
Rc
.
This can be used as a flexible way to create cyclic data structures, as in the example below.
#![feature(unique_rc_arc)]
use
std::rc::{Rc, Weak, UniqueRc};
struct
Gadget {
#[allow(dead_code)]
me: Weak<Gadget>,
}
fn
create_gadget() ->
Option
<Rc<Gadget>> {
let
mut
rc = UniqueRc::new(Gadget {
        me: Weak::new(),
    });
    rc.me = UniqueRc::downgrade(
&
rc);
Some
(UniqueRc::into_rc(rc))
}

create_gadget().unwrap();
An advantage of using
UniqueRc
over
Rc::new_cyclic
to build cyclic data structures is that
Rc::new_cyclic
’s
data_fn
parameter cannot be async or return a
Result
. As shown in the
previous example,
UniqueRc
allows for more flexibility in the construction of cyclic data,
including fallible or async constructors.
Implementations
§
Source
§
impl<T>
UniqueRc
<T>
Source
pub fn
new
(value: T) ->
UniqueRc
<T>
🔬
This is a nightly-only experimental API. (
unique_rc_arc
#112566
)
Creates a new
UniqueRc
.
Weak references to this
UniqueRc
can be created with
UniqueRc::downgrade
. Upgrading
these weak references will fail before the
UniqueRc
has been converted into an
Rc
.
After converting the
UniqueRc
into an
Rc
, any weak references created beforehand will
point to the new
Rc
.
Source
§
impl<T, A>
UniqueRc
<T, A>
where
    A:
Allocator
,
Source
pub fn
new_in
(value: T, alloc: A) ->
UniqueRc
<T, A>
🔬
This is a nightly-only experimental API. (
unique_rc_arc
#112566
)
Creates a new
UniqueRc
in the provided allocator.
Weak references to this
UniqueRc
can be created with
UniqueRc::downgrade
. Upgrading
these weak references will fail before the
UniqueRc
has been converted into an
Rc
.
After converting the
UniqueRc
into an
Rc
, any weak references created beforehand will
point to the new
Rc
.
Source
§
impl<T, A>
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
pub fn
into_rc
(this:
UniqueRc
<T, A>) ->
Rc
<T, A>
🔬
This is a nightly-only experimental API. (
unique_rc_arc
#112566
)
Converts the
UniqueRc
into a regular
Rc
.
This consumes the
UniqueRc
and returns a regular
Rc
that contains the
value
that
is passed to
into_rc
.
Any weak references created before this method is called can now be upgraded to strong
references.
Source
§
impl<T, A>
UniqueRc
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
pub fn
downgrade
(this: &
UniqueRc
<T, A>) ->
Weak
<T, A>
🔬
This is a nightly-only experimental API. (
unique_rc_arc
#112566
)
Creates a new weak reference to the
UniqueRc
.
Attempting to upgrade this weak reference will fail before the
UniqueRc
has been converted
to a
Rc
using
UniqueRc::into_rc
.
Trait Implementations
§
Source
§
impl<T:
AsFd
+ ?
Sized
>
AsFd
for
UniqueRc
<T>
Source
§
fn
as_fd
(&self) ->
BorrowedFd
<'_>
Borrows the file descriptor.
Read more
Source
§
impl<T:
AsHandle
+ ?
Sized
>
AsHandle
for
UniqueRc
<T>
Available on
Windows
only.
Source
§
fn
as_handle
(&self) ->
BorrowedHandle
<'_>
Borrows the handle.
Read more
Source
§
impl<T, A>
AsMut
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
fn
as_mut
(&mut self) ->
&mut T
Converts this type into a mutable reference of the (usually inferred) input type.
Source
§
impl<T:
AsRawFd
+ ?
Sized
>
AsRawFd
for
UniqueRc
<T>
Source
§
fn
as_raw_fd
(&self) ->
RawFd
Extracts the raw file descriptor.
Read more
Source
§
impl<T, A>
AsRef
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
fn
as_ref
(&self) ->
&T
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl<T:
AsSocket
+ ?
Sized
>
AsSocket
for
UniqueRc
<T>
Available on
Windows
only.
Source
§
fn
as_socket
(&self) ->
BorrowedSocket
<'_>
Borrows the socket.
Source
§
impl<T, A>
Borrow
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
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
impl<T, A>
BorrowMut
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
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
impl<T, A>
Debug
for
UniqueRc
<T, A>
where
    T:
Debug
+ ?
Sized
,
    A:
Allocator
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
Source
§
impl<T, A>
Deref
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
type
Target
= T
The resulting type after dereferencing.
Source
§
fn
deref
(&self) ->
&T
Dereferences the value.
Source
§
impl<T, A>
DerefMut
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
fn
deref_mut
(&mut self) ->
&mut T
Mutably dereferences the value.
Source
§
impl<T, A>
Display
for
UniqueRc
<T, A>
where
    T:
Display
+ ?
Sized
,
    A:
Allocator
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
Source
§
impl<T, A>
Drop
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
Source
§
impl<T, A>
Hash
for
UniqueRc
<T, A>
where
    T:
Hash
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
Source
§
impl<T, A>
Ord
for
UniqueRc
<T, A>
where
    T:
Ord
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
cmp
(&self, other: &
UniqueRc
<T, A>) ->
Ordering
Comparison for two
UniqueRc
s.
The two are compared by calling
cmp()
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
use
std::cmp::Ordering;
let
five = UniqueRc::new(
5
);
assert_eq!
(Ordering::Less, five.cmp(
&
UniqueRc::new(
6
)));
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
Source
§
impl<T, A>
PartialEq
for
UniqueRc
<T, A>
where
    T:
PartialEq
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
eq
(&self, other: &
UniqueRc
<T, A>) ->
bool
Equality for two
UniqueRc
s.
Two
UniqueRc
s are equal if their inner values are equal.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five == UniqueRc::new(
5
));
Source
§
fn
ne
(&self, other: &
UniqueRc
<T, A>) ->
bool
Inequality for two
UniqueRc
s.
Two
UniqueRc
s are not equal if their inner values are not equal.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five != UniqueRc::new(
6
));
Source
§
impl<T, A>
PartialOrd
for
UniqueRc
<T, A>
where
    T:
PartialOrd
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
partial_cmp
(&self, other: &
UniqueRc
<T, A>) ->
Option
<
Ordering
>
Partial comparison for two
UniqueRc
s.
The two are compared by calling
partial_cmp()
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
use
std::cmp::Ordering;
let
five = UniqueRc::new(
5
);
assert_eq!
(
Some
(Ordering::Less), five.partial_cmp(
&
UniqueRc::new(
6
)));
Source
§
fn
lt
(&self, other: &
UniqueRc
<T, A>) ->
bool
Less-than comparison for two
UniqueRc
s.
The two are compared by calling
<
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five < UniqueRc::new(
6
));
Source
§
fn
le
(&self, other: &
UniqueRc
<T, A>) ->
bool
‘Less than or equal to’ comparison for two
UniqueRc
s.
The two are compared by calling
<=
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five <= UniqueRc::new(
5
));
Source
§
fn
gt
(&self, other: &
UniqueRc
<T, A>) ->
bool
Greater-than comparison for two
UniqueRc
s.
The two are compared by calling
>
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five > UniqueRc::new(
4
));
Source
§
fn
ge
(&self, other: &
UniqueRc
<T, A>) ->
bool
‘Greater than or equal to’ comparison for two
UniqueRc
s.
The two are compared by calling
>=
on their inner values.
§
Examples
#![feature(unique_rc_arc)]
use
std::rc::UniqueRc;
let
five = UniqueRc::new(
5
);
assert!
(five >= UniqueRc::new(
5
));
Source
§
impl<T, A>
Pointer
for
UniqueRc
<T, A>
where
    A:
Allocator
,
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
Source
§
impl<T, U, A>
CoerceUnsized
<
UniqueRc
<U, A>> for
UniqueRc
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, A>
DerefPure
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, U>
DispatchFromDyn
<
UniqueRc
<U>> for
UniqueRc
<T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, A>
Eq
for
UniqueRc
<T, A>
where
    T:
Eq
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
PinCoerceUnsized
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A> !
Send
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A> !
Sync
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A>
Unpin
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
UniqueRc
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A =
Global
> !
RefUnwindSafe
for
UniqueRc
<T, A>
§
impl<T, A =
Global
> !
UnwindSafe
for
UniqueRc
<T, A>
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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