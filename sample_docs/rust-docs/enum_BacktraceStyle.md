BacktraceStyle in std::panic - Rust
std
::
panic
Enum
BacktraceStyle
Copy item path
Source
#[non_exhaustive]
pub enum BacktraceStyle {
    Short,
    Full,
    Off,
}
🔬
This is a nightly-only experimental API. (
panic_backtrace_config
#93346
)
Expand description
The configuration for whether and how the default panic hook will capture
and display the backtrace.
Variants (Non-exhaustive)
§
This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
Short
🔬
This is a nightly-only experimental API. (
panic_backtrace_config
#93346
)
Prints a terser backtrace which ideally only contains relevant
information.
§
Full
🔬
This is a nightly-only experimental API. (
panic_backtrace_config
#93346
)
Prints a backtrace with all possible information.
§
Off
🔬
This is a nightly-only experimental API. (
panic_backtrace_config
#93346
)
Disable collecting and displaying backtraces.
Trait Implementations
§
Source
§
impl
Clone
for
BacktraceStyle
Source
§
fn
clone
(&self) ->
BacktraceStyle
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
BacktraceStyle
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
impl
PartialEq
for
BacktraceStyle
Source
§
fn
eq
(&self, other: &
BacktraceStyle
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
BacktraceStyle
Source
§
impl
Eq
for
BacktraceStyle
Source
§
impl
StructuralPartialEq
for
BacktraceStyle
Auto Trait Implementations
§
§
impl
Freeze
for
BacktraceStyle
§
impl
RefUnwindSafe
for
BacktraceStyle
§
impl
Send
for
BacktraceStyle
§
impl
Sync
for
BacktraceStyle
§
impl
Unpin
for
BacktraceStyle
§
impl
UnwindSafe
for
BacktraceStyle
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