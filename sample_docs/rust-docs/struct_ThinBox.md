ThinBox in std::boxed - Rust
std
::
boxed
Struct
ThinBox
Copy item path
Source
pub struct ThinBox<T>
where
    T: ?
Sized
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
thin_box
#92791
)
Expand description
ThinBox.
A thin pointer for heap allocation, regardless of T.
§
Examples
#![feature(thin_box)]
use
std::boxed::ThinBox;
let
five = ThinBox::new(
5
);
let
thin_slice = ThinBox::<[i32]>::new_unsize([
1
,
2
,
3
,
4
]);
let
size_of_ptr = size_of::<
*const
()>();
assert_eq!
(size_of_ptr, size_of_val(
&
five));
assert_eq!
(size_of_ptr, size_of_val(
&
thin_slice));
Implementations
§
Source
§
impl<T>
ThinBox
<T>
Source
pub fn
new
(value: T) ->
ThinBox
<T>
🔬
This is a nightly-only experimental API. (
thin_box
#92791
)
Moves a type to the heap with its
Metadata
stored in the heap allocation instead of on
the stack.
§
Examples
#![feature(thin_box)]
use
std::boxed::ThinBox;
let
five = ThinBox::new(
5
);
Source
pub fn
try_new
(value: T) ->
Result
<
ThinBox
<T>,
AllocError
>
🔬
This is a nightly-only experimental API. (
thin_box
#92791
)
Moves a type to the heap with its
Metadata
stored in the heap allocation instead of on
the stack. Returns an error if allocation fails, instead of aborting.
§
Examples
#![feature(allocator_api)]
#![feature(thin_box)]
use
std::boxed::ThinBox;
let
five = ThinBox::try_new(
5
)
?
;
Source
§
impl<Dyn>
ThinBox
<Dyn>
where
    Dyn: ?
Sized
,
Source
pub fn
new_unsize
<T>(value: T) ->
ThinBox
<Dyn>
where
    T:
Unsize
<Dyn>,
🔬
This is a nightly-only experimental API. (
thin_box
#92791
)
Moves a type to the heap with its
Metadata
stored in the heap allocation instead of on
the stack.
§
Examples
#![feature(thin_box)]
use
std::boxed::ThinBox;
let
thin_slice = ThinBox::<[i32]>::new_unsize([
1
,
2
,
3
,
4
]);
Trait Implementations
§
Source
§
impl<T>
Debug
for
ThinBox
<T>
where
    T:
Debug
+ ?
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
impl<T>
Deref
for
ThinBox
<T>
where
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
impl<T>
DerefMut
for
ThinBox
<T>
where
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
impl<T>
Display
for
ThinBox
<T>
where
    T:
Display
+ ?
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
impl<T>
Drop
for
ThinBox
<T>
where
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
impl<T>
Error
for
ThinBox
<T>
where
    T:
Error
+ ?
Sized
,
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
1.0.0
·
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0
·
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
Source
§
impl<T>
Send
for
ThinBox
<T>
where
    T:
Send
+ ?
Sized
,
ThinBox<T>
is
Send
if
T
is
Send
because the data is owned.
Source
§
impl<T>
Sync
for
ThinBox
<T>
where
    T:
Sync
+ ?
Sized
,
ThinBox<T>
is
Sync
if
T
is
Sync
because the data is owned.
Auto Trait Implementations
§
§
impl<T>
Freeze
for
ThinBox
<T>
where
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
ThinBox
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
Unpin
for
ThinBox
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
ThinBox
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