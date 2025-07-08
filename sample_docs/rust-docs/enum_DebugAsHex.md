DebugAsHex in std::fmt - Rust
std
::
fmt
Enum
DebugAsHex
Copy item path
Source
pub enum DebugAsHex {
    Lower,
    Upper,
}
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Expand description
Specifies whether the
Debug
trait should use lower-/upper-case
hexadecimal or normal integers.
Variants
§
§
Lower
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Use lower-case hexadecimal integers for the
Debug
trait (like
the
x?
type
).
§
Upper
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Use upper-case hexadecimal integers for the
Debug
trait (like
the
X?
type
).
Trait Implementations
§
Source
§
impl
Clone
for
DebugAsHex
Source
§
fn
clone
(&self) ->
DebugAsHex
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
DebugAsHex
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
PartialEq
for
DebugAsHex
Source
§
fn
eq
(&self, other: &
DebugAsHex
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl
Copy
for
DebugAsHex
Source
§
impl
Eq
for
DebugAsHex
Source
§
impl
StructuralPartialEq
for
DebugAsHex
Auto Trait Implementations
§
§
impl
Freeze
for
DebugAsHex
§
impl
RefUnwindSafe
for
DebugAsHex
§
impl
Send
for
DebugAsHex
§
impl
Sync
for
DebugAsHex
§
impl
Unpin
for
DebugAsHex
§
impl
UnwindSafe
for
DebugAsHex
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