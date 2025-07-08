OsString in std::ffi::os_str - Rust
std
::
ffi
::
os_str
Struct
OsString
Copy item path
1.87.0
·
Source
pub struct OsString {
/* private fields */
}
Expand description
A type that can represent owned, mutable platform-native strings, but is
cheaply inter-convertible with Rust strings.
The need for this type arises from the fact that:
On Unix systems, strings are often arbitrary sequences of non-zero
bytes, in many cases interpreted as UTF-8.
On Windows, strings are often arbitrary sequences of non-zero 16-bit
values, interpreted as UTF-16 when it is valid to do so.
In Rust, strings are always valid UTF-8, which may contain zeros.
OsString
and
OsStr
bridge this gap by simultaneously representing Rust
and platform-native string values, and in particular allowing a Rust string
to be converted into an “OS” string with no cost if possible. A consequence
of this is that
OsString
instances are
not
NUL
terminated; in order
to pass to e.g., Unix system call, you should create a
CStr
.
OsString
is to
&
OsStr
as
String
is to
&
str
: the former
in each pair are owned strings; the latter are borrowed
references.
Note,
OsString
and
OsStr
internally do not necessarily hold strings in
the form native to the platform; While on Unix, strings are stored as a
sequence of 8-bit values, on Windows, where strings are 16-bit value based
as just discussed, strings are also actually stored as a sequence of 8-bit
values, encoded in a less-strict variant of UTF-8. This is useful to
understand when handling capacity and length values.
§
Capacity of
OsString
Capacity uses units of UTF-8 bytes for OS strings which were created from valid unicode, and
uses units of bytes in an unspecified encoding for other contents. On a given target, all
OsString
and
OsStr
values use the same units for capacity, so the following will work:
use
std::ffi::{OsStr, OsString};
fn
concat_os_strings(a:
&
OsStr, b:
&
OsStr) -> OsString {
let
mut
ret = OsString::with_capacity(a.len() + b.len());
// This will allocate
ret.push(a);
// This will not allocate further
ret.push(b);
// This will not allocate further
ret
}
§
Creating an
OsString
From a Rust string
:
OsString
implements
From
<
String
>
, so you can use
my_string.
into
()
to
create an
OsString
from a normal Rust string.
From slices:
Just like you can start with an empty Rust
String
and then
String::push_str
some
&
str
sub-string slices into it, you can create an empty
OsString
with
the
OsString::new
method and then push string slices into it with the
OsString::push
method.
§
Extracting a borrowed reference to the whole OS string
You can use the
OsString::as_os_str
method to get an
&
OsStr
from
an
OsString
; this is effectively a borrowed reference to the
whole string.
§
Conversions
See the
module’s toplevel documentation about conversions
for a discussion on
the traits which
OsString
implements for
conversions
from/to native representations.
Implementations
§
Source
§
impl
OsString
1.0.0
·
Source
pub fn
new
() ->
OsString
Constructs a new empty
OsString
.
§
Examples
use
std::ffi::OsString;
let
os_string = OsString::new();
1.74.0
·
Source
pub unsafe fn
from_encoded_bytes_unchecked
(bytes:
Vec
<
u8
>) -> Self
Converts bytes to an
OsString
without checking that the bytes contains
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
OsString
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
as_os_str
(&self) -> &
OsStr
Converts to an
OsStr
slice.
§
Examples
use
std::ffi::{OsString, OsStr};
let
os_string = OsString::from(
"foo"
);
let
os_str = OsStr::new(
"foo"
);
assert_eq!
(os_string.as_os_str(), os_str);
1.74.0
·
Source
pub fn
into_encoded_bytes
(self) ->
Vec
<
u8
>
ⓘ
Converts the
OsString
into a byte vector.  To convert the byte vector back into an
OsString
, use the
OsString::from_encoded_bytes_unchecked
function.
The byte encoding is an unspecified, platform-specific, self-synchronizing superset of UTF-8.
By being a self-synchronizing superset of UTF-8, this encoding is also a superset of 7-bit
ASCII.
Note: As the encoding is unspecified, any sub-slice of bytes that is not valid UTF-8 should
be treated as opaque and only comparable within the same Rust version built for the same
target platform.  For example, sending the bytes over the network or storing it in a file
will likely result in incompatible data.  See
OsString
for more encoding details
and
std::ffi
for platform-specific, specified conversions.
1.0.0
·
Source
pub fn
into_string
(self) ->
Result
<
String
,
OsString
>
Converts the
OsString
into a
String
if it contains valid Unicode data.
On failure, ownership of the original
OsString
is returned.
§
Examples
use
std::ffi::OsString;
let
os_string = OsString::from(
"foo"
);
let
string = os_string.into_string();
assert_eq!
(string,
Ok
(String::from(
"foo"
)));
1.0.0
·
Source
pub fn
push
<T:
AsRef
<
OsStr
>>(&mut self, s: T)
Extends the string with the given
&
OsStr
slice.
§
Examples
use
std::ffi::OsString;
let
mut
os_string = OsString::from(
"foo"
);
os_string.push(
"bar"
);
assert_eq!
(
&
os_string,
"foobar"
);
1.9.0
·
Source
pub fn
with_capacity
(capacity:
usize
) ->
OsString
Creates a new
OsString
with at least the given capacity.
The string will be able to hold at least
capacity
length units of other
OS strings without reallocating. This method is allowed to allocate for
more units than
capacity
. If
capacity
is 0, the string will not
allocate.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
mut
os_string = OsString::with_capacity(
10
);
let
capacity = os_string.capacity();
// This push is done without reallocating
os_string.push(
"foo"
);
assert_eq!
(capacity, os_string.capacity());
1.9.0
·
Source
pub fn
clear
(&mut self)
Truncates the
OsString
to zero length.
§
Examples
use
std::ffi::OsString;
let
mut
os_string = OsString::from(
"foo"
);
assert_eq!
(
&
os_string,
"foo"
);

os_string.clear();
assert_eq!
(
&
os_string,
""
);
1.9.0
·
Source
pub fn
capacity
(&self) ->
usize
Returns the capacity this
OsString
can hold without reallocating.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
os_string = OsString::with_capacity(
10
);
assert!
(os_string.capacity() >=
10
);
1.9.0
·
Source
pub fn
reserve
(&mut self, additional:
usize
)
Reserves capacity for at least
additional
more capacity to be inserted
in the given
OsString
. Does nothing if the capacity is
already sufficient.
The collection may reserve more space to speculatively avoid frequent reallocations.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::new();
s.reserve(
10
);
assert!
(s.capacity() >=
10
);
1.63.0
·
Source
pub fn
try_reserve
(&mut self, additional:
usize
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve capacity for at least
additional
more length units
in the given
OsString
. The string may reserve more space to speculatively avoid
frequent reallocations. After calling
try_reserve
, capacity will be
greater than or equal to
self.len() + additional
if it returns
Ok(())
.
Does nothing if capacity is already sufficient. This method preserves
the contents even if an error occurs.
See the main
OsString
documentation information about encoding and capacity units.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::ffi::{OsStr, OsString};
use
std::collections::TryReserveError;
fn
process_data(data:
&
str) ->
Result
<OsString, TryReserveError> {
let
mut
s = OsString::new();
// Pre-reserve the memory, exiting if we can't
s.try_reserve(OsStr::new(data).len())
?
;
// Now we know this can't OOM in the middle of our complex work
s.push(data);
Ok
(s)
}
1.9.0
·
Source
pub fn
reserve_exact
(&mut self, additional:
usize
)
Reserves the minimum capacity for at least
additional
more capacity to
be inserted in the given
OsString
. Does nothing if the capacity is
already sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
reserve
if future insertions are expected.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::new();
s.reserve_exact(
10
);
assert!
(s.capacity() >=
10
);
1.63.0
·
Source
pub fn
try_reserve_exact
(
    &mut self,
    additional:
usize
,
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve the minimum capacity for at least
additional
more length units in the given
OsString
. After calling
try_reserve_exact
, capacity will be greater than or equal to
self.len() + additional
if it returns
Ok(())
.
Does nothing if the capacity is already sufficient.
Note that the allocator may give the
OsString
more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
try_reserve
if future insertions are expected.
See the main
OsString
documentation information about encoding and capacity units.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::ffi::{OsStr, OsString};
use
std::collections::TryReserveError;
fn
process_data(data:
&
str) ->
Result
<OsString, TryReserveError> {
let
mut
s = OsString::new();
// Pre-reserve the memory, exiting if we can't
s.try_reserve_exact(OsStr::new(data).len())
?
;
// Now we know this can't OOM in the middle of our complex work
s.push(data);
Ok
(s)
}
1.19.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Shrinks the capacity of the
OsString
to match its length.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::from(
"foo"
);

s.reserve(
100
);
assert!
(s.capacity() >=
100
);

s.shrink_to_fit();
assert_eq!
(
3
, s.capacity());
1.56.0
·
Source
pub fn
shrink_to
(&mut self, min_capacity:
usize
)
Shrinks the capacity of the
OsString
with a lower bound.
The capacity will remain at least as large as both the length
and the supplied value.
If the current capacity is less than the lower limit, this is a no-op.
See the main
OsString
documentation information about encoding and capacity units.
§
Examples
use
std::ffi::OsString;
let
mut
s = OsString::from(
"foo"
);

s.reserve(
100
);
assert!
(s.capacity() >=
100
);

s.shrink_to(
10
);
assert!
(s.capacity() >=
10
);
s.shrink_to(
0
);
assert!
(s.capacity() >=
3
);
1.20.0
·
Source
pub fn
into_boxed_os_str
(self) ->
Box
<
OsStr
>
Converts this
OsString
into a boxed
OsStr
.
§
Examples
use
std::ffi::{OsString, OsStr};
let
s = OsString::from(
"hello"
);
let
b: Box<OsStr> = s.into_boxed_os_str();
Source
pub fn
leak
<'a>(self) -> &'a mut
OsStr
🔬
This is a nightly-only experimental API. (
os_string_pathbuf_leak
#125965
)
Consumes and leaks the
OsString
, returning a mutable reference to the contents,
&'a mut OsStr
.
The caller has free choice over the returned lifetime, including ’static.
Indeed, this function is ideally used for data that lives for the remainder of
the program’s life, as dropping the returned reference will cause a memory leak.
It does not reallocate or shrink the
OsString
, so the leaked allocation may include
unused capacity that is not part of the returned slice. If you want to discard excess
capacity, call
into_boxed_os_str
, and then
Box::leak
instead.
However, keep in mind that trimming the capacity may result in a reallocation and copy.
Source
pub fn
truncate
(&mut self, len:
usize
)
🔬
This is a nightly-only experimental API. (
os_string_truncate
#133262
)
Truncate the
OsString
to the specified length.
§
Panics
Panics if
len
does not lie on a valid
OsStr
boundary
(as described in
OsStr::slice_encoded_bytes
).
Methods from
Deref
<Target =
OsStr
>
§
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
Path
> for
OsString
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
1.0.0
·
Source
§
impl
Clone
for
OsString
Source
§
fn
clone_from
(&mut self, source: &Self)
Clones the contents of
source
into
self
.
This method is preferred over simply assigning
source.clone()
to
self
,
as it avoids reallocation if possible.
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
impl
Debug
for
OsString
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
for
OsString
Source
§
fn
default
() ->
OsString
Constructs an empty
OsString
.
1.0.0
·
Source
§
impl
Deref
for
OsString
Source
§
type
Target
=
OsStr
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &
OsStr
Dereferences the value.
1.44.0
·
Source
§
impl
DerefMut
for
OsString
Source
§
fn
deref_mut
(&mut self) -> &mut
OsStr
Mutably dereferences the value.
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
1.52.0
·
Source
§
impl<'a>
Extend
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
extend
<T:
IntoIterator
<Item =
Cow
<'a,
OsStr
>>>(&mut self, iter: T)
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
1.52.0
·
Source
§
impl
Extend
<
OsString
> for
OsString
Source
§
fn
extend
<T:
IntoIterator
<Item =
OsString
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
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsString
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
OsString
) ->
Cow
<'a,
OsStr
>
Converts the string reference into a
Cow::Borrowed
.
1.0.0
·
Source
§
impl<T: ?
Sized
+
AsRef
<
OsStr
>>
From
<
&T
> for
OsString
Source
§
fn
from
(s:
&T
) ->
OsString
Copies any value implementing
AsRef
<
OsStr
>
into a newly allocated
OsString
.
1.18.0
·
Source
§
impl
From
<
Box
<
OsStr
>> for
OsString
Source
§
fn
from
(boxed:
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
without copying or
allocating.
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
from
(s:
Cow
<'a,
OsStr
>) -> Self
Converts a
Cow<'a, OsStr>
into an
OsString
,
by copying the contents if they are borrowed.
1.24.0
·
Source
§
impl
From
<
OsString
> for
Arc
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
Arc
<
OsStr
>
Converts an
OsString
into an
Arc
<
OsStr
>
by moving the
OsString
data into a new
Arc
buffer.
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
1.28.0
·
Source
§
impl<'a>
From
<
OsString
> for
Cow
<'a,
OsStr
>
Source
§
fn
from
(s:
OsString
) ->
Cow
<'a,
OsStr
>
Moves the string into a
Cow::Owned
.
1.0.0
·
Source
§
impl
From
<
OsString
> for
PathBuf
Source
§
fn
from
(s:
OsString
) ->
PathBuf
Converts an
OsString
into a
PathBuf
.
This conversion does not allocate or copy memory.
1.24.0
·
Source
§
impl
From
<
OsString
> for
Rc
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
Rc
<
OsStr
>
Converts an
OsString
into an
Rc
<
OsStr
>
by moving the
OsString
data into a new
Rc
buffer.
1.14.0
·
Source
§
impl
From
<
PathBuf
> for
OsString
Source
§
fn
from
(path_buf:
PathBuf
) ->
OsString
Converts a
PathBuf
into an
OsString
This conversion does not allocate or copy memory.
1.0.0
·
Source
§
impl
From
<
String
> for
OsString
Source
§
fn
from
(s:
String
) ->
OsString
Converts a
String
into an
OsString
.
This conversion does not allocate or copy memory.
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
1.52.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
from_iter
<I:
IntoIterator
<Item =
Cow
<'a,
OsStr
>>>(iter: I) -> Self
Creates a value from an iterator.
Read more
1.52.0
·
Source
§
impl
FromIterator
<
OsString
> for
OsString
Source
§
fn
from_iter
<I:
IntoIterator
<Item =
OsString
>>(iter: I) -> Self
Creates a value from an iterator.
Read more
1.45.0
·
Source
§
impl
FromStr
for
OsString
Source
§
type
Err
=
Infallible
The associated error which can be returned from parsing.
Source
§
fn
from_str
(s: &
str
) ->
Result
<Self, Self::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.0.0
·
Source
§
impl
Hash
for
OsString
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
1.0.0
·
Source
§
impl
Index
<
RangeFull
> for
OsString
Source
§
type
Output
=
OsStr
The returned type after indexing.
Source
§
fn
index
(&self, _index:
RangeFull
) -> &
OsStr
Performs the indexing (
container[index]
) operation.
Read more
1.44.0
·
Source
§
impl
IndexMut
<
RangeFull
> for
OsString
Source
§
fn
index_mut
(&mut self, _index:
RangeFull
) -> &mut
OsStr
Performs the mutable indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl
Ord
for
OsString
Source
§
fn
cmp
(&self, other: &
OsString
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
1.0.0
·
Source
§
impl
OsStringExt
for
OsString
Available on
Unix
only.
Source
§
fn
from_vec
(vec:
Vec
<
u8
>) ->
OsString
Creates an
OsString
from a byte vector.
Read more
Source
§
fn
into_vec
(self) ->
Vec
<
u8
>
ⓘ
Yields the underlying byte vector of this
OsString
.
Read more
1.0.0
·
Source
§
impl
OsStringExt
for
OsString
Available on
WASI
only.
Source
§
fn
from_vec
(vec:
Vec
<
u8
>) ->
OsString
Creates an
OsString
from a byte vector.
Read more
Source
§
fn
into_vec
(self) ->
Vec
<
u8
>
ⓘ
Yields the underlying byte vector of this
OsString
.
Read more
1.0.0
·
Source
§
impl
OsStringExt
for
OsString
Available on
Windows
only.
Source
§
fn
from_wide
(wide: &[
u16
]) ->
OsString
Creates an
OsString
from a potentially ill-formed UTF-16 slice of
16-bit code units.
Read more
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
Path
> for
OsString
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
1.29.0
·
Source
§
impl
PartialEq
<&
str
> for
OsString
Source
§
fn
eq
(&self, other: &&
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
OsString
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
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
OsString
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
impl<'a>
PartialEq
<
OsString
> for &'a
Path
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
1.29.0
·
Source
§
impl<'a>
PartialEq
<
OsString
> for &'a
str
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
Cow
<'a,
OsStr
>
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
OsString
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
impl
PartialEq
<
OsString
> for
Path
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
impl
PartialEq
<
OsString
> for
PathBuf
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
1.0.0
·
Source
§
impl
PartialEq
<
OsString
> for
str
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
impl
PartialEq
<
Path
> for
OsString
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
PathBuf
> for
OsString
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
OsString
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
OsString
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
Path
> for
OsString
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
>> for
OsString
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
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsString
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
impl<'a>
PartialOrd
<
OsString
> for &'a
Path
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
Cow
<'a,
OsStr
>
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
OsString
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
impl
PartialOrd
<
OsString
> for
Path
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
impl
PartialOrd
<
OsString
> for
PathBuf
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
impl
PartialOrd
<
Path
> for
OsString
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
PathBuf
> for
OsString
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
OsString
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
OsString
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
Source
§
fn
lt
(&self, other: &
OsString
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
OsString
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
OsString
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
OsString
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
1.64.0
·
Source
§
impl
Write
for
OsString
Source
§
fn
write_str
(&mut self, s: &
str
) ->
Result
Writes a string slice into this writer, returning whether the write
succeeded.
Read more
1.1.0
·
Source
§
fn
write_char
(&mut self, c:
char
) ->
Result
<
()
,
Error
>
Writes a
char
into this writer, returning whether the write succeeded.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
,
Error
>
Glue for usage of the
write!
macro with implementors of this trait.
Read more
1.0.0
·
Source
§
impl
Eq
for
OsString
Auto Trait Implementations
§
§
impl
Freeze
for
OsString
§
impl
RefUnwindSafe
for
OsString
§
impl
Send
for
OsString
§
impl
Sync
for
OsString
§
impl
Unpin
for
OsString
§
impl
UnwindSafe
for
OsString
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