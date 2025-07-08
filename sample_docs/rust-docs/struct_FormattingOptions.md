FormattingOptions in std::fmt - Rust
std
::
fmt
Struct
FormattingOptions
Copy item path
Source
pub struct FormattingOptions {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Expand description
Options for formatting.
FormattingOptions
is a
Formatter
without an attached
Write
trait.
It is mainly used to construct
Formatter
instances.
Implementations
§
Source
§
impl
FormattingOptions
Source
pub const fn
new
() ->
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Construct a new
FormatterBuilder
with the supplied
Write
trait
object for output that is equivalent to the
{}
formatting
specifier:
no flags,
filled with spaces,
no alignment,
no width,
no precision, and
no
DebugAsHex
output mode.
Source
pub fn
sign
(&mut self, sign:
Option
<
Sign
>) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or removes the sign (the
+
or the
-
flag).
+
: This is intended for numeric types and indicates that the sign
should always be printed. By default only the negative sign of signed
values is printed, and the sign of positive or unsigned values is
omitted. This flag indicates that the correct sign (+ or -) should
always be printed.
-
: Currently not used
Source
pub fn
sign_aware_zero_pad
(
    &mut self,
    sign_aware_zero_pad:
bool
,
) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or unsets the
0
flag.
This is used to indicate for integer formats that the padding to width should both be done with a 0 character as well as be sign-aware
Source
pub fn
alternate
(&mut self, alternate:
bool
) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or unsets the
#
flag.
This flag indicates that the “alternate” form of printing should be
used. The alternate forms are:
Debug
: pretty-print the
Debug
formatting (adds linebreaks and indentation)
LowerHex
as well as
UpperHex
- precedes the argument with a
0x
Octal
- precedes the argument with a
0b
Binary
- precedes the argument with a
0o
Source
pub fn
fill
(&mut self, fill:
char
) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets the fill character.
The optional fill character and alignment is provided normally in
conjunction with the width parameter. This indicates that if the value
being formatted is smaller than width some extra characters will be
printed around it.
Source
pub fn
align
(&mut self, align:
Option
<
Alignment
>) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or removes the alignment.
The alignment specifies how the value being formatted should be
positioned if it is smaller than the width of the formatter.
Source
pub fn
width
(&mut self, width:
Option
<
u16
>) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or removes the width.
This is a parameter for the “minimum width” that the format should take
up. If the value’s string does not fill up this many characters, then
the padding specified by
FormattingOptions::fill
/
FormattingOptions::align
will be used to take up the required space.
Source
pub fn
precision
(&mut self, precision:
Option
<
u16
>) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Sets or removes the precision.
For non-numeric types, this can be considered a “maximum width”. If
the resulting string is longer than this width, then it is truncated
down to this many characters and that truncated value is emitted with
proper fill, alignment and width if those parameters are set.
For integral types, this is ignored.
For floating-point types, this indicates how many digits after the
decimal point should be printed.
Source
pub fn
debug_as_hex
(
    &mut self,
    debug_as_hex:
Option
<
DebugAsHex
>,
) -> &mut
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Specifies whether the
Debug
trait should use lower-/upper-case
hexadecimal or normal integers
Source
pub const fn
get_sign
(&self) ->
Option
<
Sign
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current sign (the
+
or the
-
flag).
Source
pub const fn
get_sign_aware_zero_pad
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current
0
flag.
Source
pub const fn
get_alternate
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current
#
flag.
Source
pub const fn
get_fill
(&self) ->
char
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current fill character.
Source
pub const fn
get_align
(&self) ->
Option
<
Alignment
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current alignment.
Source
pub const fn
get_width
(&self) ->
Option
<
u16
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current width.
Source
pub const fn
get_precision
(&self) ->
Option
<
u16
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current precision.
Source
pub const fn
get_debug_as_hex
(&self) ->
Option
<
DebugAsHex
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the current precision.
Source
pub fn
create_formatter
<'a>(self, write: &'a mut dyn
Write
) ->
Formatter
<'a>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Creates a
Formatter
that writes its output to the given
Write
trait.
You may alternatively use
Formatter::new()
.
Trait Implementations
§
Source
§
impl
Clone
for
FormattingOptions
Source
§
fn
clone
(&self) ->
FormattingOptions
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
FormattingOptions
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
FormattingOptions
Source
§
fn
default
() ->
FormattingOptions
Same as
FormattingOptions::new()
.
Source
§
impl
PartialEq
for
FormattingOptions
Source
§
fn
eq
(&self, other: &
FormattingOptions
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
FormattingOptions
Source
§
impl
Eq
for
FormattingOptions
Source
§
impl
StructuralPartialEq
for
FormattingOptions
Auto Trait Implementations
§
§
impl
Freeze
for
FormattingOptions
§
impl
RefUnwindSafe
for
FormattingOptions
§
impl
Send
for
FormattingOptions
§
impl
Sync
for
FormattingOptions
§
impl
Unpin
for
FormattingOptions
§
impl
UnwindSafe
for
FormattingOptions
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