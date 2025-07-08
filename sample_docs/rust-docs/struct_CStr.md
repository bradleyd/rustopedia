CStr in std::ffi::c_str - Rust
std
::
ffi
::
c_str
Struct
CStr
Copy item path
Source
pub struct CStr {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Expand description
Representation of a borrowed C string.
This type represents a borrowed reference to a nul-terminated
array of bytes. It can be constructed safely from a
&[
u8
]
slice, or unsafely from a raw
*const c_char
. It can be expressed as a
literal in the form
c"Hello world"
.
The
CStr
can then be converted to a Rust
&
str
by performing
UTF-8 validation, or into an owned
CString
.
&CStr
is to
CString
as
&
str
is to
String
: the former
in each pair are borrowed references; the latter are owned
strings.
Note that this structure does
not
have a guaranteed layout (the
repr(transparent)
notwithstanding) and should not be placed in the signatures of FFI functions.
Instead, safe wrappers of FFI functions may leverage
CStr::as_ptr
and the unsafe
CStr::from_ptr
constructor to provide a safe interface to other consumers.
§
Examples
Inspecting a foreign C string:
use
std::ffi::CStr;
use
std::os::raw::c_char;
extern
"C"
{
fn
my_string() ->
*const
c_char; }
unsafe
{
let
slice = CStr::from_ptr(my_string());
println!
(
"string buffer size without nul terminator: {}"
, slice.to_bytes().len());
}
Passing a Rust-originating C string:
use
std::ffi::CStr;
use
std::os::raw::c_char;
fn
work(data:
&
CStr) {
unsafe extern
"C"
fn
work_with(s:
*const
c_char) {}
unsafe
{ work_with(data.as_ptr()) }
}
let
s =
c"Hello world!"
;
work(
&
s);
Converting a foreign C string into a Rust
String
:
use
std::ffi::CStr;
use
std::os::raw::c_char;
extern
"C"
{
fn
my_string() ->
*const
c_char; }
fn
my_string_safe() -> String {
let
cstr =
unsafe
{ CStr::from_ptr(my_string()) };
// Get copy-on-write Cow<'_, str>, then guarantee a freshly-owned String allocation
String::from_utf8_lossy(cstr.to_bytes()).to_string()
}
println!
(
"string: {}"
, my_string_safe());
Implementations
§
Source
§
impl
CStr
1.0.0 (const: 1.81.0)
·
Source
pub const unsafe fn
from_ptr
<'a>(ptr:
*const
i8
) -> &'a
CStr
Wraps a raw C string with a safe C string wrapper.
This function will wrap the provided
ptr
with a
CStr
wrapper, which
allows inspection and interoperation of non-owned C strings. The total
size of the terminated buffer must be smaller than
isize::MAX
bytes
in memory (a restriction from
slice::from_raw_parts
).
§
Safety
The memory pointed to by
ptr
must contain a valid nul terminator at the
end of the string.
ptr
must be
valid
for reads of bytes up to and including the nul terminator.
This means in particular:
The entire memory range of this
CStr
must be contained within a single allocated object!
ptr
must be non-null even for a zero-length cstr.
The memory referenced by the returned
CStr
must not be mutated for
the duration of lifetime
'a
.
The nul terminator must be within
isize::MAX
from
ptr
Note
: This operation is intended to be a 0-cost cast but it is
currently implemented with an up-front calculation of the length of
the string. This is not guaranteed to always be the case.
§
Caveat
The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse,
it’s suggested to tie the lifetime to whichever source lifetime is safe in the context,
such as by providing a helper function taking the lifetime of a host value for the slice,
or by explicit annotation.
§
Examples
use
std::ffi::{c_char, CStr};
fn
my_string() ->
*const
c_char {
c"hello"
.as_ptr()
}
unsafe
{
let
slice = CStr::from_ptr(my_string());
assert_eq!
(slice.to_str().unwrap(),
"hello"
);
}
use
std::ffi::{c_char, CStr};
const
HELLO_PTR:
*const
c_char = {
const
BYTES:
&
[u8] =
b"Hello, world!\0"
;
    BYTES.as_ptr().cast()
};
const
HELLO:
&
CStr =
unsafe
{ CStr::from_ptr(HELLO_PTR) };
assert_eq!
(
c"Hello, world!"
, HELLO);
1.69.0 (const: 1.69.0)
·
Source
pub const fn
from_bytes_until_nul
(
    bytes: &[
u8
],
) ->
Result
<&
CStr
,
FromBytesUntilNulError
>
Creates a C string wrapper from a byte slice with any number of nuls.
This method will create a
CStr
from any byte slice that contains at
least one nul byte. Unlike with
CStr::from_bytes_with_nul
, the caller
does not need to know where the nul byte is located.
If the first byte is a nul character, this method will return an
empty
CStr
. If multiple nul characters are present, the
CStr
will
end at the first one.
If the slice only has a single nul byte at the end, this method is
equivalent to
CStr::from_bytes_with_nul
.
§
Examples
use
std::ffi::CStr;
let
mut
buffer = [
0u8
;
16
];
unsafe
{
// Here we might call an unsafe C function that writes a string
    // into the buffer.
let
buf_ptr = buffer.as_mut_ptr();
    buf_ptr.write_bytes(
b'A'
,
8
);
}
// Attempt to extract a C nul-terminated string from the buffer.
let
c_str = CStr::from_bytes_until_nul(
&
buffer[..]).unwrap();
assert_eq!
(c_str.to_str().unwrap(),
"AAAAAAAA"
);
1.10.0 (const: 1.72.0)
·
Source
pub const fn
from_bytes_with_nul
(
    bytes: &[
u8
],
) ->
Result
<&
CStr
,
FromBytesWithNulError
>
Creates a C string wrapper from a byte slice with exactly one nul
terminator.
This function will cast the provided
bytes
to a
CStr
wrapper after ensuring that the byte slice is nul-terminated
and does not contain any interior nul bytes.
If the nul byte may not be at the end,
CStr::from_bytes_until_nul
can be used instead.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"hello\0"
);
assert_eq!
(cstr,
Ok
(
c"hello"
));
Creating a
CStr
without a trailing nul terminator is an error:
use
std::ffi::{CStr, FromBytesWithNulError};
let
cstr = CStr::from_bytes_with_nul(
b"hello"
);
assert_eq!
(cstr,
Err
(FromBytesWithNulError::NotNulTerminated));
Creating a
CStr
with an interior nul byte is an error:
use
std::ffi::{CStr, FromBytesWithNulError};
let
cstr = CStr::from_bytes_with_nul(
b"he\0llo\0"
);
assert_eq!
(cstr,
Err
(FromBytesWithNulError::InteriorNul { position:
2
}));
1.10.0 (const: 1.59.0)
·
Source
pub const unsafe fn
from_bytes_with_nul_unchecked
(bytes: &[
u8
]) -> &
CStr
Unsafely creates a C string wrapper from a byte slice.
This function will cast the provided
bytes
to a
CStr
wrapper without
performing any sanity checks.
§
Safety
The provided slice
must
be nul-terminated and not contain any interior
nul bytes.
§
Examples
use
std::ffi::CStr;
let
bytes =
b"Hello world!\0"
;
let
cstr =
unsafe
{ CStr::from_bytes_with_nul_unchecked(bytes) };
assert_eq!
(cstr.to_bytes_with_nul(), bytes);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
as_ptr
(&self) ->
*const
i8
Returns the inner pointer to this C string.
The returned pointer will be valid for as long as
self
is, and points
to a contiguous region of memory terminated with a 0 byte to represent
the end of the string.
The type of the returned pointer is
*const c_char
, and whether it’s
an alias for
*const i8
or
*const u8
is platform-specific.
WARNING
The returned pointer is read-only; writing to it (including passing it
to C code that writes to it) causes undefined behavior.
It is your responsibility to make sure that the underlying memory is not
freed too early. For example, the following code will cause undefined
behavior when
ptr
is used inside the
unsafe
block:
use
std::ffi::{CStr, CString};
// 💀 The meaning of this entire program is undefined,
// 💀 and nothing about its behavior is guaranteed,
// 💀 not even that its behavior resembles the code as written,
// 💀 just because it contains a single instance of undefined behavior!

// 🚨 creates a dangling pointer to a temporary `CString`
// 🚨 that is deallocated at the end of the statement
let
ptr = CString::new(
"Hi!"
.to_uppercase()).unwrap().as_ptr();
// without undefined behavior, you would expect that `ptr` equals:
dbg!
(CStr::from_bytes_with_nul(
b"HI!\0"
).unwrap());
// 🙏 Possibly the program behaved as expected so far,
// 🙏 and this just shows `ptr` is now garbage..., but
// 💀 this violates `CStr::from_ptr`'s safety contract
// 💀 leading to a dereference of a dangling pointer,
// 💀 which is immediate undefined behavior.
// 💀 *BOOM*, you're dead, you're entire program has no meaning.
dbg!
(
unsafe
{ CStr::from_ptr(ptr) });
This happens because, the pointer returned by
as_ptr
does not carry any
lifetime information, and the
CString
is deallocated immediately after
the expression that it is part of has been evaluated.
To fix the problem, bind the
CString
to a local variable:
use
std::ffi::{CStr, CString};
let
c_str = CString::new(
"Hi!"
.to_uppercase()).unwrap();
let
ptr = c_str.as_ptr();
assert_eq!
(
unsafe
{ CStr::from_ptr(ptr) },
c"HI!"
);
1.79.0 (const: 1.81.0)
·
Source
pub const fn
count_bytes
(&self) ->
usize
Returns the length of
self
. Like C’s
strlen
, this does not include the nul terminator.
Note
: This method is currently implemented as a constant-time
cast, but it is planned to alter its definition in the future to
perform the length calculation whenever this method is called.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
).unwrap();
assert_eq!
(cstr.count_bytes(),
3
);
let
cstr = CStr::from_bytes_with_nul(
b"\0"
).unwrap();
assert_eq!
(cstr.count_bytes(),
0
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
is_empty
(&self) ->
bool
Returns
true
if
self.to_bytes()
has a length of 0.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
)
?
;
assert!
(!cstr.is_empty());
let
empty_cstr = CStr::from_bytes_with_nul(
b"\0"
)
?
;
assert!
(empty_cstr.is_empty());
assert!
(
c""
.is_empty());
1.0.0 (const: 1.72.0)
·
Source
pub const fn
to_bytes
(&self) -> &[
u8
]
ⓘ
Converts this C string to a byte slice.
The returned slice will
not
contain the trailing nul terminator that this C
string has.
Note
: This method is currently implemented as a constant-time
cast, but it is planned to alter its definition in the future to
perform the length calculation whenever this method is called.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
).expect(
"CStr::from_bytes_with_nul failed"
);
assert_eq!
(cstr.to_bytes(),
b"foo"
);
1.0.0 (const: 1.72.0)
·
Source
pub const fn
to_bytes_with_nul
(&self) -> &[
u8
]
ⓘ
Converts this C string to a byte slice containing the trailing 0 byte.
This function is the equivalent of
CStr::to_bytes
except that it
will retain the trailing nul terminator instead of chopping it off.
Note
: This method is currently implemented as a 0-cost cast, but
it is planned to alter its definition in the future to perform the
length calculation whenever this method is called.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
).expect(
"CStr::from_bytes_with_nul failed"
);
assert_eq!
(cstr.to_bytes_with_nul(),
b"foo\0"
);
Source
pub fn
bytes
(&self) ->
Bytes
<'_>
ⓘ
🔬
This is a nightly-only experimental API. (
cstr_bytes
#112115
)
Iterates over the bytes in this C string.
The returned iterator will
not
contain the trailing nul terminator
that this C string has.
§
Examples
#![feature(cstr_bytes)]
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
).expect(
"CStr::from_bytes_with_nul failed"
);
assert!
(cstr.bytes().eq(
*
b"foo"
));
1.4.0 (const: 1.72.0)
·
Source
pub const fn
to_str
(&self) ->
Result
<&
str
,
Utf8Error
>
Yields a
&
str
slice if the
CStr
contains valid UTF-8.
If the contents of the
CStr
are valid UTF-8 data, this
function will return the corresponding
&
str
slice. Otherwise,
it will return an error with details of where UTF-8 validation failed.
§
Examples
use
std::ffi::CStr;
let
cstr = CStr::from_bytes_with_nul(
b"foo\0"
).expect(
"CStr::from_bytes_with_nul failed"
);
assert_eq!
(cstr.to_str(),
Ok
(
"foo"
));
Source
§
impl
CStr
1.4.0
·
Source
pub fn
to_string_lossy
(&self) ->
Cow
<'_,
str
>
Converts a
CStr
into a
Cow
<
str
>
.
If the contents of the
CStr
are valid UTF-8 data, this
function will return a
Cow
::
Borrowed
(&
str
)
with the corresponding
&
str
slice. Otherwise, it will
replace any invalid UTF-8 sequences with
U+FFFD REPLACEMENT CHARACTER
and return a
Cow
::
Owned
(&
str
)
with the result.
§
Examples
Calling
to_string_lossy
on a
CStr
containing valid UTF-8. The leading
c
on the string literal denotes a
CStr
.
use
std::borrow::Cow;
assert_eq!
(
c"Hello World"
.to_string_lossy(), Cow::Borrowed(
"Hello World"
));
Calling
to_string_lossy
on a
CStr
containing invalid UTF-8:
use
std::borrow::Cow;
assert_eq!
(
c"Hello \xF0\x90\x80World"
.to_string_lossy(),
    Cow::Owned(String::from(
"Hello �World"
))
as
Cow<
'_
, str>
);
1.20.0
·
Source
pub fn
into_c_string
(self:
Box
<
CStr
>) ->
CString
Converts a
Box
<
CStr
>
into a
CString
without copying or allocating.
§
Examples
use
std::ffi::{CStr, CString};
let
boxed: Box<CStr> = Box::from(
c"foo"
);
let
c_string: CString =
c"foo"
.to_owned();
assert_eq!
(boxed.into_c_string(), c_string);
Trait Implementations
§
1.7.0
·
Source
§
impl
AsRef
<
CStr
> for
CStr
Source
§
fn
as_ref
(&self) -> &
CStr
Converts this type into a shared reference of the (usually inferred) input type.
1.7.0
·
Source
§
impl
AsRef
<
CStr
> for
CString
Source
§
fn
as_ref
(&self) -> &
CStr
Converts this type into a shared reference of the (usually inferred) input type.
1.3.0
·
Source
§
impl
Borrow
<
CStr
> for
CString
Source
§
fn
borrow
(&self) -> &
CStr
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
CStr
>
Source
§
fn
clone
(&self) ->
Box
<
CStr
>
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
CStr
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
1.3.0
·
Source
§
impl
Debug
for
CStr
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
impl
Default
for &
CStr
Source
§
fn
default
() -> &
CStr
Returns the “default value” for a type.
Read more
1.17.0
·
Source
§
impl
Default
for
Box
<
CStr
>
Source
§
fn
default
() ->
Box
<
CStr
>
Returns the “default value” for a type.
Read more
1.24.0
·
Source
§
impl
From
<&
CStr
> for
Arc
<
CStr
>
Source
§
fn
from
(s: &
CStr
) ->
Arc
<
CStr
>
Converts a
&CStr
into a
Arc<CStr>
,
by copying the contents into a newly allocated
Arc
.
1.17.0
·
Source
§
impl
From
<&
CStr
> for
Box
<
CStr
>
Source
§
fn
from
(s: &
CStr
) ->
Box
<
CStr
>
Converts a
&CStr
into a
Box<CStr>
,
by copying the contents into a newly allocated
Box
.
1.7.0
·
Source
§
impl
From
<&
CStr
> for
CString
Source
§
fn
from
(s: &
CStr
) ->
CString
Converts a
&
CStr
into a
CString
by copying the contents into a new allocation.
1.28.0
·
Source
§
impl<'a>
From
<&'a
CStr
> for
Cow
<'a,
CStr
>
Source
§
fn
from
(s: &'a
CStr
) ->
Cow
<'a,
CStr
>
Converts a
CStr
into a borrowed
Cow
without copying or allocating.
1.24.0
·
Source
§
impl
From
<&
CStr
> for
Rc
<
CStr
>
Source
§
fn
from
(s: &
CStr
) ->
Rc
<
CStr
>
Converts a
&CStr
into a
Rc<CStr>
,
by copying the contents into a newly allocated
Rc
.
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Arc
<
CStr
>
Source
§
fn
from
(s: &mut
CStr
) ->
Arc
<
CStr
>
Converts a
&mut CStr
into a
Arc<CStr>
,
by copying the contents into a newly allocated
Arc
.
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Box
<
CStr
>
Source
§
fn
from
(s: &mut
CStr
) ->
Box
<
CStr
>
Converts a
&mut CStr
into a
Box<CStr>
,
by copying the contents into a newly allocated
Box
.
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Rc
<
CStr
>
Source
§
fn
from
(s: &mut
CStr
) ->
Rc
<
CStr
>
Converts a
&mut CStr
into a
Rc<CStr>
,
by copying the contents into a newly allocated
Rc
.
1.20.0
·
Source
§
impl
From
<
CString
> for
Box
<
CStr
>
Source
§
fn
from
(s:
CString
) ->
Box
<
CStr
>
Converts a
CString
into a
Box
<
CStr
>
without copying or allocating.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
CStr
>> for
Box
<
CStr
>
Source
§
fn
from
(cow:
Cow
<'_,
CStr
>) ->
Box
<
CStr
>
Converts a
Cow<'a, CStr>
into a
Box<CStr>
,
by copying the contents if they are borrowed.
1.64.0
·
Source
§
impl
Hash
for
CStr
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
1.47.0
·
Source
§
impl
Index
<
RangeFrom
<
usize
>> for
CStr
Source
§
type
Output
=
CStr
The returned type after indexing.
Source
§
fn
index
(&self, index:
RangeFrom
<
usize
>) -> &
CStr
Performs the indexing (
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
CStr
Source
§
fn
cmp
(&self, other: &
CStr
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
1.64.0
·
Source
§
impl
PartialEq
for
CStr
Source
§
fn
eq
(&self, other: &
CStr
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
PartialOrd
for
CStr
Source
§
fn
partial_cmp
(&self, other: &
CStr
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
1.3.0
·
Source
§
impl
ToOwned
for
CStr
Source
§
type
Owned
=
CString
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) ->
CString
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target: &mut
CString
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
1.64.0
·
Source
§
impl
Eq
for
CStr
1.64.0
·
Source
§
impl
StructuralPartialEq
for
CStr
Auto Trait Implementations
§
§
impl
Freeze
for
CStr
§
impl
RefUnwindSafe
for
CStr
§
impl
Send
for
CStr
§
impl !
Sized
for
CStr
§
impl
Sync
for
CStr
§
impl
Unpin
for
CStr
§
impl
UnwindSafe
for
CStr
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