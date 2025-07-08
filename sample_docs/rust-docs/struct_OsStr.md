OsStr in std::ffi::os_str - Rust
std
::
ffi
::
os_str
Struct
OsStr
Copy item path
1.87.0
·
Source
pub struct OsStr {
/* private fields */
}
Expand description
Borrowed reference to an OS string (see
OsString
).
This type represents a borrowed reference to a string in the operating system’s preferred
representation.
&OsStr
is to
OsString
as
&
str
is to
String
: the
former in each pair are borrowed references; the latter are owned strings.
See the
module’s toplevel documentation about conversions
for a discussion on
the traits which
OsStr
implements for
conversions
from/to native representations.
Implementations
§
Source
§
impl
OsStr
1.0.0
·
Source
pub fn
new
<S:
AsRef
<
OsStr
> + ?
Sized
>(s:
&S
) -> &
OsStr
Coerces into an
OsStr
slice.
§
Examples
use
std::ffi::OsStr;
let
os_str = OsStr::new(
"foo"
);
1.74.0
·
Source
pub unsafe fn
from_encoded_bytes_unchecked
(bytes: &[
u8
]) -> &Self
Converts a slice of bytes to an OS string slice without checking that the string contains
valid
OsStr
-encoded data.
The byte encoding is an unspecified, platform-specific, self-synchronizing superset of UTF-8.
By being a self-synchronizing superset of UTF-8, this encoding is also a superset of 7-bit
ASCII.
See the
module’s toplevel documentation about conversions
for safe,
cross-platform
conversions
from/to native representations.
§
Safety
As the encoding is unspecified, callers must pass in bytes that originated as a mixture of
validated UTF-8 and bytes from
OsStr::as_encoded_bytes
from within the same Rust version
built for the same target platform.  For example, reconstructing an
OsStr
from bytes sent
over the network or stored in a file will likely violate these safety rules.
Due to the encoding being self-synchronizing, the bytes from
OsStr::as_encoded_bytes
can be
split either immediately before or immediately after any valid non-empty UTF-8 substring.
§
Example
use
std::ffi::OsStr;
let
os_str = OsStr::new(
"Mary had a little lamb"
);
let
bytes = os_str.as_encoded_bytes();
let
words = bytes.split(|b|
*
b ==
b' '
);
let
words: Vec<
&
OsStr> = words.map(|word| {
// SAFETY:
    // - Each `word` only contains content that originated from `OsStr::as_encoded_bytes`
    // - Only split with ASCII whitespace which is a non-empty UTF-8 substring
unsafe
{ OsStr::from_encoded_bytes_unchecked(word) }
}).collect();
1.0.0
·
Source
pub fn
to_str
(&self) ->
Option
<&
str
>
Yields a
&
str
slice if the
OsStr
is valid Unicode.
This conversion may entail doing a check for UTF-8 validity.
§
Examples
use
std::ffi::OsStr;
let
os_str = OsStr::new(
"foo"
);
assert_eq!
(os_str.to_str(),
Some
(
"foo"
));
1.0.0
·
Source
pub fn
to_string_lossy
(&self) ->
Cow
<'_,
str
>
Converts an
OsStr
to a
Cow
<
str
>
.
Any non-UTF-8 sequences are replaced with
U+FFFD REPLACEMENT CHARACTER
.
§
Examples
Calling
to_string_lossy
on an
OsStr
with invalid unicode:
// Note, due to differences in how Unix and Windows represent strings,
// we are forced to complicate this example, setting up example `OsStr`s
// with different source data and via different platform extensions.
// Understand that in reality you could end up with such example invalid
// sequences simply through collecting user command line arguments, for
// example.
#[cfg(unix)]
{
use
std::ffi::OsStr;
use
std::os::unix::ffi::OsStrExt;
// Here, the values 0x66 and 0x6f correspond to 'f' and 'o'
    // respectively. The value 0x80 is a lone continuation byte, invalid
    // in a UTF-8 sequence.
let
source = [
0x66
,
0x6f
,
0x80
,
0x6f
];
let
os_str = OsStr::from_bytes(
&
source[..]);
assert_eq!
(os_str.to_string_lossy(),
"fo�o"
);
}
#[cfg(windows)]
{
use
std::ffi::OsString;
use
std::os::windows::prelude::
*
;
// Here the values 0x0066 and 0x006f correspond to 'f' and 'o'
    // respectively. The value 0xD800 is a lone surrogate half, invalid
    // in a UTF-16 sequence.
let
source = [
0x0066
,
0x006f
,
0xD800
,
0x006f
];
let
os_string = OsString::from_wide(
&
source[..]);
let
os_str = os_string.as_os_str();
assert_eq!
(os_str.to_string_lossy(),
"fo�o"
);
}
1.0.0
·
Source
pub fn
to_os_string
(&self) ->
OsString
Copies the slice into an owned
OsString
.
§
Examples
use
std::ffi::{OsStr, OsString};
let
os_str = OsStr::new(
"foo"
);
let
os_string = os_str.to_os_string();
assert_eq!
(os_string, OsString::from(
"foo"
));
1.9.0
·
Source
pub fn
is_empty
(&self) ->
bool
Checks whether the
OsStr
is empty.
§
Examples
use
std::ffi::OsStr;
let
os_str = OsStr::new(
""
);
assert!
(os_str.is_empty());
let
os_str = OsStr::new(
"foo"
);
assert!
(!os_str.is_empty());
1.9.0
·
Source
pub fn
len
(&self) ->
usize
Returns the length of this
OsStr
.
Note that this does
not
return the number of bytes in the string in
OS string form.
The length returned is that of the underlying storage used by
OsStr
.
As discussed in the
OsString
introduction,
OsString
and
OsStr
store strings in a form best suited for cheap inter-conversion between
native-platform and Rust string forms, which may differ significantly
from both of them, including in storage size and encoding.
This number is simply useful for passing to other methods, like
OsString::with_capacity
to avoid reallocations.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsStr;
let
os_str = OsStr::new(
""
);
assert_eq!
(os_str.len(),
0
);
let
os_str = OsStr::new(
"foo"
);
assert_eq!
(os_str.len(),
3
);
1.20.0
·
Source
pub fn
into_os_string
(self:
Box
<
OsStr
>) ->
OsString
Converts a
Box
<
OsStr
>
into an
OsString
without copying or allocating.
1.74.0
·
Source
pub fn
as_encoded_bytes
(&self) -> &[
u8
]
ⓘ
Converts an OS string slice to a byte slice.  To convert the byte slice back into an OS
string slice, use the
OsStr::from_encoded_bytes_unchecked
function.
The byte encoding is an unspecified, platform-specific, self-synchronizing superset of UTF-8.
By being a self-synchronizing superset of UTF-8, this encoding is also a superset of 7-bit
ASCII.
Note: As the encoding is unspecified, any sub-slice of bytes that is not valid UTF-8 should
be treated as opaque and only comparable within the same Rust version built for the same
target platform.  For example, sending the slice over the network or storing it in a file
will likely result in incompatible byte slices.  See
OsString
for more encoding details
and
std::ffi
for platform-specific, specified conversions.
Source
pub fn
slice_encoded_bytes
<R:
RangeBounds
<
usize
>>(&self, range: R) -> &Self
🔬
This is a nightly-only experimental API. (
os_str_slice
#118485
)
Takes a substring based on a range that corresponds to the return value of
OsStr::as_encoded_bytes
.
The range’s start and end must lie on valid
OsStr
boundaries.
A valid
OsStr
boundary is one of:
The start of the string
The end of the string
Immediately before a valid non-empty UTF-8 substring
Immediately after a valid non-empty UTF-8 substring
§
Panics
Panics if
range
does not lie on valid
OsStr
boundaries or if it
exceeds the end of the string.
§
Example
#![feature(os_str_slice)]
use
std::ffi::OsStr;
let
os_str = OsStr::new(
"foo=bar"
);
let
bytes = os_str.as_encoded_bytes();
if let
Some
(index) = bytes.iter().position(|b|
*
b ==
b'='
) {
let
key = os_str.slice_encoded_bytes(..index);
let
value = os_str.slice_encoded_bytes(index +
1
..);
assert_eq!
(key,
"foo"
);
assert_eq!
(value,
"bar"
);
}
1.53.0
·
Source
pub fn
make_ascii_lowercase
(&mut self)
Converts this string to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
OsStr::to_ascii_lowercase
.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::from(
"GRÜßE, JÜRGEN ❤"
);

s.make_ascii_lowercase();
assert_eq!
(
"grÜße, jÜrgen ❤"
, s);
1.53.0
·
Source
pub fn
make_ascii_uppercase
(&mut self)
Converts this string to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
OsStr::to_ascii_uppercase
.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::from(
"Grüße, Jürgen ❤"
);

s.make_ascii_uppercase();
assert_eq!
(
"GRüßE, JüRGEN ❤"
, s);
1.53.0
·
Source
pub fn
to_ascii_lowercase
(&self) ->
OsString
Returns a copy of this string where each character is mapped to its
ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
OsStr::make_ascii_lowercase
.
§
Examples
use
std::ffi::OsString;
let
s = OsString::from(
"Grüße, Jürgen ❤"
);
assert_eq!
(
"grüße, jürgen ❤"
, s.to_ascii_lowercase());
1.53.0
·
Source
pub fn
to_ascii_uppercase
(&self) ->
OsString
Returns a copy of this string where each character is mapped to its
ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
OsStr::make_ascii_uppercase
.
§
Examples
use
std::ffi::OsString;
let
s = OsString::from(
"Grüße, Jürgen ❤"
);
assert_eq!
(
"GRüßE, JüRGEN ❤"
, s.to_ascii_uppercase());
1.53.0
·
Source
pub fn
is_ascii
(&self) ->
bool
Checks if all characters in this string are within the ASCII range.
§
Examples
use
std::ffi::OsString;
let
ascii = OsString::from(
"hello!\n"
);
let
non_ascii = OsString::from(
"Grüße, Jürgen ❤"
);
assert!
(ascii.is_ascii());
assert!
(!non_ascii.is_ascii());
1.53.0
·
Source
pub fn
eq_ignore_ascii_case
<S:
AsRef
<
OsStr
>>(&self, other: S) ->
bool
Checks that two strings are an ASCII case-insensitive match.
Same as
to_ascii_lowercase(a) == to_ascii_lowercase(b)
,
but without allocating and copying temporaries.
§
Examples
use
std::ffi::OsString;
assert!
(OsString::from(
"Ferris"
).eq_ignore_ascii_case(
"FERRIS"
));
assert!
(OsString::from(
"Ferrös"
).eq_ignore_ascii_case(
"FERRöS"
));
assert!
(!OsString::from(
"Ferrös"
).eq_ignore_ascii_case(
"FERRÖS"
));
1.87.0
·
Source
pub fn
display
(&self) ->
Display
<'_>
Returns an object that implements
Display
for safely printing an
OsStr
that may contain non-Unicode data. This may perform lossy
conversion, depending on the platform.  If you would like an
implementation which escapes the
OsStr
please use
Debug
instead.
§
Examples
use
std::ffi::OsStr;
let
s = OsStr::new(
"Hello, world!"
);
println!
(
"{}"
, s.display());
Trait Implementations
§
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Component
<'_>
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Components
<'_>
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Iter
<'_>
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
OsStr
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
OsString
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Path
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
PathBuf
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
String
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
str
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
OsStr
Source
§
fn
as_ref
(&self) -> &
Path
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
Borrow
<
OsStr
> for
OsString
Source
§
fn
borrow
(&self) -> &
OsStr
Immutably borrows from an owned value.
Read more
1.29.0
·
Source
§
impl
Clone
for
Box
<
OsStr
>
Source
§
fn
clone
(&self) -> Self
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
CloneToUninit
for
OsStr
Source
§
unsafe fn
clone_to_uninit
(&self, dst:
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
1.0.0
·
Source
§
impl
Debug
for
OsStr
Source
§
fn
fmt
(&self, formatter: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.9.0
·
Source
§
impl
Default
for &
OsStr
Source
§
fn
default
() -> Self
Creates an empty
OsStr
.
1.17.0
·
Source
§
impl
Default
for
Box
<
OsStr
>
Source
§
fn
default
() ->
Box
<
OsStr
>
Returns the “default value” for a type.
Read more
1.52.0
·
Source
§
impl<'a>
Extend
<&'a
OsStr
> for
OsString
Source
§
fn
extend
<T:
IntoIterator
<Item = &'a
OsStr
>>(&mut self, iter: T)
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, item: A)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
1.24.0
·
Source
§
impl
From
<&
OsStr
> for
Arc
<
OsStr
>
Source
§
fn
from
(s: &
OsStr
) ->
Arc
<
OsStr
>
Copies the string into a newly allocated
Arc
<
OsStr
>
.
1.17.0
·
Source
§
impl
From
<&
OsStr
> for
Box
<
OsStr
>
Source
§
fn
from
(s: &
OsStr
) ->
Box
<
OsStr
>
Copies the string into a newly allocated
Box
<
OsStr
>
.
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
from
(s: &'a
OsStr
) ->
Cow
<'a,
OsStr
>
Converts the string reference into a
Cow::Borrowed
.
1.24.0
·
Source
§
impl
From
<&
OsStr
> for
Rc
<
OsStr
>
Source
§
fn
from
(s: &
OsStr
) ->
Rc
<
OsStr
>
Copies the string into a newly allocated
Rc
<
OsStr
>
.
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Arc
<
OsStr
>
Source
§
fn
from
(s: &mut
OsStr
) ->
Arc
<
OsStr
>
Copies the string into a newly allocated
Arc
<
OsStr
>
.
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Box
<
OsStr
>
Source
§
fn
from
(s: &mut
OsStr
) ->
Box
<
OsStr
>
Copies the string into a newly allocated
Box
<
OsStr
>
.
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Rc
<
OsStr
>
Source
§
fn
from
(s: &mut
OsStr
) ->
Rc
<
OsStr
>
Copies the string into a newly allocated
Rc
<
OsStr
>
.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
OsStr
>> for
Box
<
OsStr
>
Source
§
fn
from
(cow:
Cow
<'_,
OsStr
>) ->
Box
<
OsStr
>
Converts a
Cow<'a, OsStr>
into a
Box
<
OsStr
>
,
by copying the contents if they are borrowed.
1.20.0
·
Source
§
impl
From
<
OsString
> for
Box
<
OsStr
>
Source
§
fn
from
(s:
OsString
) ->
Box
<
OsStr
>
Converts an
OsString
into a
Box
<
OsStr
>
without copying or allocating.
1.52.0
·
Source
§
impl<'a>
FromIterator
<&'a
OsStr
> for
OsString
Source
§
fn
from_iter
<I:
IntoIterator
<Item = &'a
OsStr
>>(iter: I) -> Self
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
Hash
for
OsStr
Source
§
fn
hash
<H:
Hasher
>(&self, state:
&mut H
)
Feeds this value into the given
Hasher
.
Read more
Source
§
impl<S:
Borrow
<
OsStr
>>
Join
<&
OsStr
> for
[S]
Source
§
type
Output
=
OsString
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
The resulting type after concatenation
Source
§
fn
join
(slice: &Self, sep: &
OsStr
) ->
OsString
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
Implementation of
[T]::join
1.0.0
·
Source
§
impl
Ord
for
OsStr
Source
§
fn
cmp
(&self, other: &
OsStr
) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.0.0
·
Source
§
impl
OsStrExt
for
OsStr
Available on
Unix
only.
Source
§
fn
from_bytes
(slice: &[
u8
]) -> &
OsStr
Creates an
OsStr
from a byte slice.
Read more
Source
§
fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Gets the underlying byte view of the
OsStr
slice.
Read more
1.0.0
·
Source
§
impl
OsStrExt
for
OsStr
Available on
WASI
only.
Source
§
fn
from_bytes
(slice: &[
u8
]) -> &
OsStr
Creates an
OsStr
from a byte slice.
Read more
Source
§
fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Gets the underlying byte view of the
OsStr
slice.
Read more
1.0.0
·
Source
§
impl
OsStrExt
for
OsStr
Available on
Windows
only.
Source
§
fn
encode_wide
(&self) ->
EncodeWide
<'_>
ⓘ
Re-encodes an
OsStr
as a wide character sequence, i.e., potentially
ill-formed UTF-16.
Read more
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &&'b
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &&'b
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'a
OsStr
> for
OsString
Source
§
fn
eq
(&self, other: &&'a
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
OsStr
> for
Path
Source
§
fn
eq
(&self, other: &&'a
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
OsStr
> for
PathBuf
Source
§
fn
eq
(&self, other: &&'a
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
Path
> for
OsStr
Source
§
fn
eq
(&self, other: &&'a
Path
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
OsStr
>> for &'b
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
OsStr
>> for
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
Path
>> for &'b
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsStr
> for &'a
Path
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsStr
> for
OsString
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl
PartialEq
<
OsStr
> for
Path
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl
PartialEq
<
OsStr
> for
PathBuf
Source
§
fn
eq
(&self, other: &
OsStr
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
1.0.0
·
Source
§
impl
PartialEq
<
OsStr
> for
str
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsString
> for &'a
OsStr
Source
§
fn
eq
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsString
> for
OsStr
Source
§
fn
eq
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Path
> for &'a
OsStr
Source
§
fn
eq
(&self, other: &
Path
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
1.8.0
·
Source
§
impl
PartialEq
<
Path
> for
OsStr
Source
§
fn
eq
(&self, other: &
Path
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for &'a
OsStr
Source
§
fn
eq
(&self, other: &
PathBuf
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
1.8.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
OsStr
Source
§
fn
eq
(&self, other: &
PathBuf
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
1.0.0
·
Source
§
impl
PartialEq
<
str
> for
OsStr
Source
§
fn
eq
(&self, other: &
str
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
1.0.0
·
Source
§
impl
PartialEq
for
OsStr
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &&'b
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &&'b
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'a
OsStr
> for
OsString
Source
§
fn
partial_cmp
(&self, other: &&'a
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
OsStr
> for
Path
Source
§
fn
partial_cmp
(&self, other: &&'a
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
OsStr
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &&'a
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
Path
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &&'a
Path
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for &'b
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
Path
>> for &'b
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsStr
> for &'a
Path
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsStr
> for
OsString
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl
PartialOrd
<
OsStr
> for
Path
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl
PartialOrd
<
OsStr
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for &'a
OsStr
Source
§
fn
partial_cmp
(&self, other: &
OsString
) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
OsString
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for &'a
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Path
) ->
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
1.8.0
·
Source
§
impl
PartialOrd
<
Path
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Path
) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for &'a
OsStr
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
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
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
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
1.0.0
·
Source
§
impl
PartialOrd
<
str
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
str
) ->
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
1.0.0
·
Source
§
impl
PartialOrd
for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
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
Source
§
fn
lt
(&self, other: &
OsStr
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
Source
§
fn
le
(&self, other: &
OsStr
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
Source
§
fn
gt
(&self, other: &
OsStr
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
Source
§
fn
ge
(&self, other: &
OsStr
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
1.0.0
·
Source
§
impl
ToOwned
for
OsStr
Source
§
type
Owned
=
OsString
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) ->
OsString
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target: &mut
OsString
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
1.72.0
·
Source
§
impl<'a>
TryFrom
<&'a
OsStr
> for &'a
str
Source
§
fn
try_from
(value: &'a
OsStr
) ->
Result
<Self, Self::
Error
>
Tries to convert an
&OsStr
to a
&str
.
use
std::ffi::OsStr;
let
os_str = OsStr::new(
"foo"
);
let
as_str = <
&
str>::try_from(os_str).unwrap();
assert_eq!
(as_str,
"foo"
);
Source
§
type
Error
=
Utf8Error
The type returned in the event of a conversion error.
1.0.0
·
Source
§
impl
Eq
for
OsStr
Auto Trait Implementations
§
§
impl
Freeze
for
OsStr
§
impl
RefUnwindSafe
for
OsStr
§
impl
Send
for
OsStr
§
impl !
Sized
for
OsStr
§
impl
Sync
for
OsStr
§
impl
Unpin
for
OsStr
§
impl
UnwindSafe
for
OsStr
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