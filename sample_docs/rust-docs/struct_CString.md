CString in std::ffi::c_str - Rust
std
::
ffi
::
c_str
Struct
CString
Copy item path
Source
pub struct CString {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
c_str_module
#112134
)
Expand description
A type representing an owned, C-compatible, nul-terminated string with no nul bytes in the
middle.
This type serves the purpose of being able to safely generate a
C-compatible string from a Rust byte slice or vector. An instance of this
type is a static guarantee that the underlying bytes contain no interior 0
bytes (“nul characters”) and that the final byte is 0 (“nul terminator”).
CString
is to
&
CStr
as
String
is to
&
str
: the former
in each pair are owned strings; the latter are borrowed
references.
§
Creating a
CString
A
CString
is created from either a byte slice or a byte vector,
or anything that implements
Into
<
Vec
<
u8
>>
(for
example, you can build a
CString
straight out of a
String
or
a
&
str
, since both implement that trait).
You can create a
CString
from a literal with
CString::from(c"Text")
.
The
CString::new
method will actually check that the provided
&[
u8
]
does not have 0 bytes in the middle, and return an error if it
finds one.
§
Extracting a raw pointer to the whole C string
CString
implements an
as_ptr
method through the
Deref
trait. This method will give you a
*const c_char
which you can
feed directly to extern functions that expect a nul-terminated
string, like C’s
strdup()
. Notice that
as_ptr
returns a
read-only pointer; if the C code writes to it, that causes
undefined behavior.
§
Extracting a slice of the whole C string
Alternatively, you can obtain a
&[
u8
]
slice from a
CString
with the
CString::as_bytes
method. Slices produced in this
way do
not
contain the trailing nul terminator. This is useful
when you will be calling an extern function that takes a
*const u8
argument which is not necessarily nul-terminated, plus another
argument with the length of the string — like C’s
strndup()
.
You can of course get the slice’s length with its
len
method.
If you need a
&[
u8
]
slice
with
the nul terminator, you
can use
CString::as_bytes_with_nul
instead.
Once you have the kind of slice you need (with or without a nul
terminator), you can call the slice’s own
as_ptr
method to get a read-only raw pointer to pass to
extern functions. See the documentation for that function for a
discussion on ensuring the lifetime of the raw pointer.
§
Examples
ⓘ
use
std::ffi::CString;
use
std::os::raw::c_char;
extern
"C"
{
fn
my_printer(s:
*const
c_char);
}
// We are certain that our string doesn't have 0 bytes in the middle,
// so we can .expect()
let
c_to_print = CString::new(
"Hello, world!"
).expect(
"CString::new failed"
);
unsafe
{
    my_printer(c_to_print.as_ptr());
}
§
Safety
CString
is intended for working with traditional C-style strings
(a sequence of non-nul bytes terminated by a single nul byte); the
primary use case for these kinds of strings is interoperating with C-like
code. Often you will need to transfer ownership to/from that external
code. It is strongly recommended that you thoroughly read through the
documentation of
CString
before use, as improper ownership management
of
CString
instances can lead to invalid memory accesses, memory leaks,
and other memory errors.
Implementations
§
Source
§
impl
CString
1.0.0
·
Source
pub fn
new
<T>(t: T) ->
Result
<
CString
,
NulError
>
where
    T:
Into
<
Vec
<
u8
>>,
Creates a new C-compatible string from a container of bytes.
This function will consume the provided data and use the
underlying bytes to construct a new string, ensuring that
there is a trailing 0 byte. This trailing 0 byte will be
appended by this function; the provided data should
not
contain any 0 bytes in it.
§
Examples
ⓘ
use
std::ffi::CString;
use
std::os::raw::c_char;
extern
"C"
{
fn
puts(s:
*const
c_char); }
let
to_print = CString::new(
"Hello!"
).expect(
"CString::new failed"
);
unsafe
{
    puts(to_print.as_ptr());
}
§
Errors
This function will return an error if the supplied bytes contain an
internal 0 byte. The
NulError
returned will contain the bytes as well as
the position of the nul byte.
1.0.0
·
Source
pub unsafe fn
from_vec_unchecked
(v:
Vec
<
u8
>) ->
CString
Creates a C-compatible string by consuming a byte vector,
without checking for interior 0 bytes.
Trailing 0 byte will be appended by this function.
This method is equivalent to
CString::new
except that no runtime
assertion is made that
v
contains no 0 bytes, and it requires an
actual byte vector, not anything that can be converted to one with Into.
§
Examples
use
std::ffi::CString;
let
raw =
b"foo"
.to_vec();
unsafe
{
let
c_string = CString::from_vec_unchecked(raw);
}
1.4.0
·
Source
pub unsafe fn
from_raw
(ptr:
*mut
i8
) ->
CString
Retakes ownership of a
CString
that was transferred to C via
CString::into_raw
.
Additionally, the length of the string will be recalculated from the pointer.
§
Safety
This should only ever be called with a pointer that was earlier
obtained by calling
CString::into_raw
. Other usage (e.g., trying to take
ownership of a string that was allocated by foreign code) is likely to lead
to undefined behavior or allocator corruption.
It should be noted that the length isn’t just “recomputed,” but that
the recomputed length must match the original length from the
CString::into_raw
call. This means the
CString::into_raw
/
from_raw
methods should not be used when passing the string to C functions that can
modify the string’s length.
Note:
If you need to borrow a string that was allocated by
foreign code, use
CStr
. If you need to take ownership of
a string that was allocated by foreign code, you will need to
make your own provisions for freeing it appropriately, likely
with the foreign code’s API to do that.
§
Examples
Creates a
CString
, pass ownership to an
extern
function (via raw pointer), then retake
ownership with
from_raw
:
ⓘ
use
std::ffi::CString;
use
std::os::raw::c_char;
extern
"C"
{
fn
some_extern_function(s:
*mut
c_char);
}
let
c_string = CString::from(
c"Hello!"
);
let
raw = c_string.into_raw();
unsafe
{
    some_extern_function(raw);
let
c_string = CString::from_raw(raw);
}
1.4.0
·
Source
pub fn
into_raw
(self) ->
*mut
i8
Consumes the
CString
and transfers ownership of the string to a C caller.
The pointer which this function returns must be returned to Rust and reconstituted using
CString::from_raw
to be properly deallocated. Specifically, one
should
not
use the standard C
free()
function to deallocate
this string.
Failure to call
CString::from_raw
will lead to a memory leak.
The C side must
not
modify the length of the string (by writing a
nul byte somewhere inside the string or removing the final one) before
it makes it back into Rust using
CString::from_raw
. See the safety section
in
CString::from_raw
.
§
Examples
use
std::ffi::CString;
let
c_string = CString::from(
c"foo"
);
let
ptr = c_string.into_raw();
unsafe
{
assert_eq!
(
b'f'
,
*
ptr
as
u8);
assert_eq!
(
b'o'
,
*
ptr.add(
1
)
as
u8);
assert_eq!
(
b'o'
,
*
ptr.add(
2
)
as
u8);
assert_eq!
(
b'\0'
,
*
ptr.add(
3
)
as
u8);
// retake pointer to free memory
let _
= CString::from_raw(ptr);
}
1.7.0
·
Source
pub fn
into_string
(self) ->
Result
<
String
,
IntoStringError
>
Converts the
CString
into a
String
if it contains valid UTF-8 data.
On failure, ownership of the original
CString
is returned.
§
Examples
use
std::ffi::CString;
let
valid_utf8 =
vec!
[
b'f'
,
b'o'
,
b'o'
];
let
cstring = CString::new(valid_utf8).expect(
"CString::new failed"
);
assert_eq!
(cstring.into_string().expect(
"into_string() call failed"
),
"foo"
);
let
invalid_utf8 =
vec!
[
b'f'
,
0xff
,
b'o'
,
b'o'
];
let
cstring = CString::new(invalid_utf8).expect(
"CString::new failed"
);
let
err = cstring.into_string().err().expect(
"into_string().err() failed"
);
assert_eq!
(err.utf8_error().valid_up_to(),
1
);
1.7.0
·
Source
pub fn
into_bytes
(self) ->
Vec
<
u8
>
ⓘ
Consumes the
CString
and returns the underlying byte buffer.
The returned buffer does
not
contain the trailing nul
terminator, and it is guaranteed to not have any interior nul
bytes.
§
Examples
use
std::ffi::CString;
let
c_string = CString::from(
c"foo"
);
let
bytes = c_string.into_bytes();
assert_eq!
(bytes,
vec!
[
b'f'
,
b'o'
,
b'o'
]);
1.7.0
·
Source
pub fn
into_bytes_with_nul
(self) ->
Vec
<
u8
>
ⓘ
Equivalent to
CString::into_bytes()
except that the
returned vector includes the trailing nul terminator.
§
Examples
use
std::ffi::CString;
let
c_string = CString::from(
c"foo"
);
let
bytes = c_string.into_bytes_with_nul();
assert_eq!
(bytes,
vec!
[
b'f'
,
b'o'
,
b'o'
,
b'\0'
]);
1.0.0
·
Source
pub fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Returns the contents of this
CString
as a slice of bytes.
The returned slice does
not
contain the trailing nul
terminator, and it is guaranteed to not have any interior nul
bytes. If you need the nul terminator, use
CString::as_bytes_with_nul
instead.
§
Examples
use
std::ffi::CString;
let
c_string = CString::from(
c"foo"
);
let
bytes = c_string.as_bytes();
assert_eq!
(bytes,
&
[
b'f'
,
b'o'
,
b'o'
]);
1.0.0
·
Source
pub fn
as_bytes_with_nul
(&self) -> &[
u8
]
ⓘ
Equivalent to
CString::as_bytes()
except that the
returned slice includes the trailing nul terminator.
§
Examples
use
std::ffi::CString;
let
c_string = CString::from(
c"foo"
);
let
bytes = c_string.as_bytes_with_nul();
assert_eq!
(bytes,
&
[
b'f'
,
b'o'
,
b'o'
,
b'\0'
]);
1.20.0
·
Source
pub fn
as_c_str
(&self) -> &
CStr
Extracts a
CStr
slice containing the entire string.
§
Examples
use
std::ffi::{CString, CStr};
let
c_string = CString::from(
c"foo"
);
let
cstr = c_string.as_c_str();
assert_eq!
(cstr,
           CStr::from_bytes_with_nul(
b"foo\0"
).expect(
"CStr::from_bytes_with_nul failed"
));
1.20.0
·
Source
pub fn
into_boxed_c_str
(self) ->
Box
<
CStr
>
Converts this
CString
into a boxed
CStr
.
§
Examples
let
c_string =
c"foo"
.to_owned();
let
boxed = c_string.into_boxed_c_str();
assert_eq!
(boxed.to_bytes_with_nul(),
b"foo\0"
);
1.58.0
·
Source
pub unsafe fn
from_vec_with_nul_unchecked
(v:
Vec
<
u8
>) ->
CString
Converts a
Vec
<
u8
>
to a
CString
without checking the
invariants on the given
Vec
.
§
Safety
The given
Vec
must
have one nul byte as its last element.
This means it cannot be empty nor have any other nul byte anywhere else.
§
Example
use
std::ffi::CString;
assert_eq!
(
unsafe
{ CString::from_vec_with_nul_unchecked(
b"abc\0"
.to_vec()) },
unsafe
{ CString::from_vec_unchecked(
b"abc"
.to_vec()) }
);
1.58.0
·
Source
pub fn
from_vec_with_nul
(v:
Vec
<
u8
>) ->
Result
<
CString
,
FromVecWithNulError
>
Attempts to converts a
Vec
<
u8
>
to a
CString
.
Runtime checks are present to ensure there is only one nul byte in the
Vec
, its last element.
§
Errors
If a nul byte is present and not the last element or no nul bytes
is present, an error will be returned.
§
Examples
A successful conversion will produce the same result as
CString::new
when called without the ending nul byte.
use
std::ffi::CString;
assert_eq!
(
    CString::from_vec_with_nul(
b"abc\0"
.to_vec())
        .expect(
"CString::from_vec_with_nul failed"
),
c"abc"
.to_owned()
);
An incorrectly formatted
Vec
will produce an error.
use
std::ffi::{CString, FromVecWithNulError};
// Interior nul byte
let _
: FromVecWithNulError = CString::from_vec_with_nul(
b"a\0bc"
.to_vec()).unwrap_err();
// No nul byte
let _
: FromVecWithNulError = CString::from_vec_with_nul(
b"abc"
.to_vec()).unwrap_err();
Methods from
Deref
<Target =
CStr
>
§
1.0.0
·
Source
pub fn
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
1.79.0
·
Source
pub fn
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
1.71.0
·
Source
pub fn
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
1.0.0
·
Source
pub fn
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
1.0.0
·
Source
pub fn
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
1.4.0
·
Source
pub fn
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
1.64.0
·
Source
§
impl
Clone
for
CString
Source
§
fn
clone
(&self) ->
CString
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
CString
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
for
CString
Source
§
fn
default
() ->
CString
Creates an empty
CString
.
1.0.0
·
Source
§
impl
Deref
for
CString
Source
§
type
Target
=
CStr
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &
CStr
Dereferences the value.
1.13.0
·
Source
§
impl
Drop
for
CString
Source
§
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
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
CString
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
CString
) ->
Cow
<'a,
CStr
>
Converts a
&
CString
into a borrowed
Cow
without copying or allocating.
1.18.0
·
Source
§
impl
From
<
Box
<
CStr
>> for
CString
Source
§
fn
from
(s:
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
1.24.0
·
Source
§
impl
From
<
CString
> for
Arc
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
Arc
<
CStr
>
Converts a
CString
into an
Arc
<
CStr
>
by moving the
CString
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
1.28.0
·
Source
§
impl<'a>
From
<
CString
> for
Cow
<'a,
CStr
>
Source
§
fn
from
(s:
CString
) ->
Cow
<'a,
CStr
>
Converts a
CString
into an owned
Cow
without copying or allocating.
1.24.0
·
Source
§
impl
From
<
CString
> for
Rc
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
Rc
<
CStr
>
Converts a
CString
into an
Rc
<
CStr
>
by moving the
CString
data into a new
Rc
buffer.
1.7.0
·
Source
§
impl
From
<
CString
> for
Vec
<
u8
>
Source
§
fn
from
(s:
CString
) ->
Vec
<
u8
>
ⓘ
Converts a
CString
into a
Vec
<
u8
>
.
The conversion consumes the
CString
, and removes the terminating NUL byte.
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
CStr
>> for
CString
Source
§
fn
from
(s:
Cow
<'a,
CStr
>) ->
CString
Converts a
Cow<'a, CStr>
into a
CString
, by copying the contents if they are
borrowed.
1.43.0
·
Source
§
impl
From
<
Vec
<
NonZero
<
u8
>>> for
CString
Source
§
fn
from
(v:
Vec
<
NonZero
<
u8
>>) ->
CString
Converts a
Vec
<
NonZero
<
u8
>>
into a
CString
without
copying nor checking for inner nul bytes.
Source
§
impl
FromStr
for
CString
Source
§
fn
from_str
(s: &
str
) ->
Result
<
CString
, <
CString
as
FromStr
>::
Err
>
Converts a string
s
into a
CString
.
This method is equivalent to
CString::new
.
Source
§
type
Err
=
NulError
The associated error which can be returned from parsing.
1.64.0
·
Source
§
impl
Hash
for
CString
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
1.7.0
·
Source
§
impl
Index
<
RangeFull
> for
CString
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
(&self, _index:
RangeFull
) -> &
CStr
Performs the indexing (
container[index]
) operation.
Read more
1.64.0
·
Source
§
impl
Ord
for
CString
Source
§
fn
cmp
(&self, other: &
CString
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
1.64.0
·
Source
§
impl
PartialEq
for
CString
Source
§
fn
eq
(&self, other: &
CString
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
1.64.0
·
Source
§
impl
PartialOrd
for
CString
Source
§
fn
partial_cmp
(&self, other: &
CString
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
Source
§
impl
TryFrom
<
CString
> for
String
Source
§
fn
try_from
(
    value:
CString
,
) ->
Result
<
String
, <
String
as
TryFrom
<
CString
>>::
Error
>
Converts a
CString
into a
String
if it contains valid UTF-8 data.
This method is equivalent to
CString::into_string
.
Source
§
type
Error
=
IntoStringError
The type returned in the event of a conversion error.
1.64.0
·
Source
§
impl
Eq
for
CString
1.64.0
·
Source
§
impl
StructuralPartialEq
for
CString
Auto Trait Implementations
§
§
impl
Freeze
for
CString
§
impl
RefUnwindSafe
for
CString
§
impl
Send
for
CString
§
impl
Sync
for
CString
§
impl
Unpin
for
CString
§
impl
UnwindSafe
for
CString
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