Arguments in std::fmt - Rust
std
::
fmt
Struct
Arguments
Copy item path
1.0.0
·
Source
pub struct Arguments<'a> {
/* private fields */
}
Expand description
This structure represents a safely precompiled version of a format string
and its arguments. This cannot be generated at runtime because it cannot
safely be done, so no constructors are given and the fields are private
to prevent modification.
The
format_args!
macro will safely create an instance of this structure.
The macro validates the format string at compile-time so usage of the
write()
and
format()
functions can be safely performed.
You can use the
Arguments<'a>
that
format_args!
returns in
Debug
and
Display
contexts as seen below. The example also shows that
Debug
and
Display
format to the same thing: the interpolated format string
in
format_args!
.
let
debug =
format!
(
"{:?}"
,
format_args!
(
"{} foo {:?}"
,
1
,
2
));
let
display =
format!
(
"{}"
,
format_args!
(
"{} foo {:?}"
,
1
,
2
));
assert_eq!
(
"1 foo 2"
, display);
assert_eq!
(display, debug);
Implementations
§
Source
§
impl<'a>
Arguments
<'a>
1.52.0 (const: 1.84.0)
·
Source
pub const fn
as_str
(&self) ->
Option
<&'static
str
>
Gets the formatted string, if it has no arguments to be formatted at runtime.
This can be used to avoid allocations in some cases.
§
Guarantees
For
format_args!("just a literal")
, this function is guaranteed to
return
Some("just a literal")
.
For most cases with placeholders, this function will return
None
.
However, the compiler may perform optimizations that can cause this
function to return
Some(_)
even if the format string contains
placeholders. For example,
format_args!("Hello, {}!", "world")
may be
optimized to
format_args!("Hello, world!")
, such that
as_str()
returns
Some("Hello, world!")
.
The behavior for anything but the trivial case (without placeholders)
is not guaranteed, and should not be relied upon for anything other
than optimization.
§
Examples
use
std::fmt::Arguments;
fn
write_str(
_
:
&
str) {
/* ... */
}
fn
write_fmt(args:
&
Arguments<
'_
>) {
if let
Some
(s) = args.as_str() {
        write_str(s)
    }
else
{
        write_str(
&
args.to_string());
    }
}
assert_eq!
(
format_args!
(
"hello"
).as_str(),
Some
(
"hello"
));
assert_eq!
(
format_args!
(
""
).as_str(),
Some
(
""
));
assert_eq!
(
format_args!
(
"{:?}"
, std::env::current_dir()).as_str(),
None
);
Trait Implementations
§
1.0.0
·
Source
§
impl<'a>
Clone
for
Arguments
<'a>
Source
§
fn
clone
(&self) ->
Arguments
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
1.0.0
·
Source
§
impl
Debug
for
Arguments
<'_>
Source
§
fn
fmt
(&self, fmt: &mut
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
1.0.0
·
Source
§
impl
Display
for
Arguments
<'_>
Source
§
fn
fmt
(&self, fmt: &mut
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
1.0.0
·
Source
§
impl<'a>
Copy
for
Arguments
<'a>
1.0.0
·
Source
§
impl !
Send
for
Arguments
<'_>
1.0.0
·
Source
§
impl !
Sync
for
Arguments
<'_>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Arguments
<'a>
§
impl<'a>
RefUnwindSafe
for
Arguments
<'a>
§
impl<'a>
Unpin
for
Arguments
<'a>
§
impl<'a>
UnwindSafe
for
Arguments
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