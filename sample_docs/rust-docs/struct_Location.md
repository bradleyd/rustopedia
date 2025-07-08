Location in std::panic - Rust
std
::
panic
Struct
Location
Copy item path
1.10.0
·
Source
pub struct Location<'a> {
/* private fields */
}
Expand description
A struct containing information about the location of a panic.
This structure is created by
PanicHookInfo::location()
and
PanicInfo::location()
.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(location) = panic_info.location() {
println!
(
"panic occurred in file '{}' at line {}"
, location.file(), location.line());
    }
else
{
println!
(
"panic occurred but can't get location information..."
);
    }
}));
panic!
(
"Normal panic"
);
§
Comparisons
Comparisons for equality and ordering are made in file, line, then column priority.
Files are compared as strings, not
Path
, which could be unexpected.
See
Location::file
’s documentation for more discussion.
Implementations
§
Source
§
impl<'a>
Location
<'a>
1.46.0 (const: 1.79.0)
·
Source
pub const fn
caller
() -> &'static
Location
<'static>
Returns the source location of the caller of this function. If that function’s caller is
annotated then its call location will be returned, and so on up the stack to the first call
within a non-tracked function body.
§
Examples
use
std::panic::Location;
/// Returns the [`Location`] at which it is called.
#[track_caller]
fn
get_caller_location() ->
&
'static
Location<
'static
> {
    Location::caller()
}
/// Returns a [`Location`] from within this function's definition.
fn
get_just_one_location() ->
&
'static
Location<
'static
> {
    get_caller_location()
}
let
fixed_location = get_just_one_location();
assert_eq!
(fixed_location.file(),
file!
());
assert_eq!
(fixed_location.line(),
14
);
assert_eq!
(fixed_location.column(),
5
);
// running the same untracked function in a different location gives us the same result
let
second_fixed_location = get_just_one_location();
assert_eq!
(fixed_location.file(), second_fixed_location.file());
assert_eq!
(fixed_location.line(), second_fixed_location.line());
assert_eq!
(fixed_location.column(), second_fixed_location.column());
let
this_location = get_caller_location();
assert_eq!
(this_location.file(),
file!
());
assert_eq!
(this_location.line(),
28
);
assert_eq!
(this_location.column(),
21
);
// running the tracked function in a different location produces a different value
let
another_location = get_caller_location();
assert_eq!
(this_location.file(), another_location.file());
assert_ne!
(this_location.line(), another_location.line());
assert_ne!
(this_location.column(), another_location.column());
1.10.0 (const: 1.79.0)
·
Source
pub const fn
file
(&self) -> &
str
Returns the name of the source file from which the panic originated.
§
&str
, not
&Path
The returned name refers to a source path on the compiling system, but it isn’t valid to
represent this directly as a
&Path
. The compiled code may run on a different system with
a different
Path
implementation than the system providing the contents and this library
does not currently have a different “host path” type.
The most surprising behavior occurs when “the same” file is reachable via multiple paths in
the module system (usually using the
#[path = "..."]
attribute or similar), which can
cause what appears to be identical code to return differing values from this function.
§
Cross-compilation
This value is not suitable for passing to
Path::new
or similar constructors when the host
platform and target platform differ.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(location) = panic_info.location() {
println!
(
"panic occurred in file '{}'"
, location.file());
    }
else
{
println!
(
"panic occurred but can't get location information..."
);
    }
}));
panic!
(
"Normal panic"
);
1.10.0 (const: 1.79.0)
·
Source
pub const fn
line
(&self) ->
u32
Returns the line number from which the panic originated.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(location) = panic_info.location() {
println!
(
"panic occurred at line {}"
, location.line());
    }
else
{
println!
(
"panic occurred but can't get location information..."
);
    }
}));
panic!
(
"Normal panic"
);
1.25.0 (const: 1.79.0)
·
Source
pub const fn
column
(&self) ->
u32
Returns the column from which the panic originated.
§
Examples
ⓘ
use
std::panic;

panic::set_hook(Box::new(|panic_info| {
if let
Some
(location) = panic_info.location() {
println!
(
"panic occurred at column {}"
, location.column());
    }
else
{
println!
(
"panic occurred but can't get location information..."
);
    }
}));
panic!
(
"Normal panic"
);
Trait Implementations
§
1.10.0
·
Source
§
impl<'a>
Clone
for
Location
<'a>
Source
§
fn
clone
(&self) ->
Location
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
1.10.0
·
Source
§
impl<'a>
Debug
for
Location
<'a>
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
1.26.0
·
Source
§
impl
Display
for
Location
<'_>
Source
§
fn
fmt
(&self, formatter: &mut
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
impl<'a>
Hash
for
Location
<'a>
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
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
1.10.0
·
Source
§
impl<'a>
Ord
for
Location
<'a>
Source
§
fn
cmp
(&self, other: &
Location
<'a>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
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
1.10.0
·
Source
§
impl<'a>
PartialEq
for
Location
<'a>
Source
§
fn
eq
(&self, other: &
Location
<'a>) ->
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
1.10.0
·
Source
§
impl<'a>
PartialOrd
for
Location
<'a>
Source
§
fn
partial_cmp
(&self, other: &
Location
<'a>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.10.0
·
Source
§
impl<'a>
Copy
for
Location
<'a>
1.10.0
·
Source
§
impl<'a>
Eq
for
Location
<'a>
1.10.0
·
Source
§
impl<'a>
StructuralPartialEq
for
Location
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Location
<'a>
§
impl<'a>
RefUnwindSafe
for
Location
<'a>
§
impl<'a>
Send
for
Location
<'a>
§
impl<'a>
Sync
for
Location
<'a>
§
impl<'a>
Unpin
for
Location
<'a>
§
impl<'a>
UnwindSafe
for
Location
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