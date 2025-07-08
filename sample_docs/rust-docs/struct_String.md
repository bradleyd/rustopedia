String in std::string - Rust
std
::
string
Struct
String
Copy item path
1.0.0
·
Source
pub struct String {
/* private fields */
}
Expand description
A UTF-8–encoded, growable string.
String
is the most common string type. It has ownership over the contents
of the string, stored in a heap-allocated buffer (see
Representation
).
It is closely related to its borrowed counterpart, the primitive
str
.
§
Examples
You can create a
String
from
a literal string
with
String::from
:
let
hello = String::from(
"Hello, world!"
);
You can append a
char
to a
String
with the
push
method, and
append a
&str
with the
push_str
method:
let
mut
hello = String::from(
"Hello, "
);

hello.push(
'w'
);
hello.push_str(
"orld!"
);
If you have a vector of UTF-8 bytes, you can create a
String
from it with
the
from_utf8
method:
// some bytes, in a vector
let
sparkle_heart =
vec!
[
240
,
159
,
146
,
150
];
// We know these bytes are valid, so we'll use `unwrap()`.
let
sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
assert_eq!
(
"💖"
, sparkle_heart);
§
UTF-8
String
s are always valid UTF-8. If you need a non-UTF-8 string, consider
OsString
. It is similar, but without the UTF-8 constraint. Because UTF-8
is a variable width encoding,
String
s are typically smaller than an array of
the same
char
s:
// `s` is ASCII which represents each `char` as one byte
let
s =
"hello"
;
assert_eq!
(s.len(),
5
);
// A `char` array with the same contents would be longer because
// every `char` is four bytes
let
s = [
'h'
,
'e'
,
'l'
,
'l'
,
'o'
];
let
size: usize = s.into_iter().map(|c| size_of_val(
&
c)).sum();
assert_eq!
(size,
20
);
// However, for non-ASCII strings, the difference will be smaller
// and sometimes they are the same
let
s =
"💖💖💖💖💖"
;
assert_eq!
(s.len(),
20
);
let
s = [
'💖'
,
'💖'
,
'💖'
,
'💖'
,
'💖'
];
let
size: usize = s.into_iter().map(|c| size_of_val(
&
c)).sum();
assert_eq!
(size,
20
);
This raises interesting questions as to how
s[i]
should work.
What should
i
be here? Several options include byte indices and
char
indices but, because of UTF-8 encoding, only byte indices
would provide constant time indexing. Getting the
i
th
char
, for
example, is available using
chars
:
let
s =
"hello"
;
let
third_character = s.chars().nth(
2
);
assert_eq!
(third_character,
Some
(
'l'
));
let
s =
"💖💖💖💖💖"
;
let
third_character = s.chars().nth(
2
);
assert_eq!
(third_character,
Some
(
'💖'
));
Next, what should
s[i]
return? Because indexing returns a reference
to underlying data it could be
&u8
,
&[u8]
, or something else similar.
Since we’re only providing one index,
&u8
makes the most sense but that
might not be what the user expects and can be explicitly achieved with
as_bytes()
:
// The first byte is 104 - the byte value of `'h'`
let
s =
"hello"
;
assert_eq!
(s.as_bytes()[
0
],
104
);
// or
assert_eq!
(s.as_bytes()[
0
],
b'h'
);
// The first byte is 240 which isn't obviously useful
let
s =
"💖💖💖💖💖"
;
assert_eq!
(s.as_bytes()[
0
],
240
);
Due to these ambiguities/restrictions, indexing with a
usize
is simply
forbidden:
ⓘ
let
s =
"hello"
;
// The following will not compile!
println!
(
"The first letter of s is {}"
, s[
0
]);
It is more clear, however, how
&s[i..j]
should work (that is,
indexing with a range). It should accept byte indices (to be constant-time)
and return a
&str
which is UTF-8 encoded. This is also called “string slicing”.
Note this will panic if the byte indices provided are not character
boundaries - see
is_char_boundary
for more details. See the implementations
for
SliceIndex<str>
for more details on string slicing. For a non-panicking
version of string slicing, see
get
.
The
bytes
and
chars
methods return iterators over the bytes and
codepoints of the string, respectively. To iterate over codepoints along
with byte indices, use
char_indices
.
§
Deref
String
implements
Deref
<Target =
str
>
, and so inherits all of
str
’s
methods. In addition, this means that you can pass a
String
to a
function which takes a
&str
by using an ampersand (
&
):
fn
takes_str(s:
&
str) { }
let
s = String::from(
"Hello"
);

takes_str(
&
s);
This will create a
&str
from the
String
and pass it in. This
conversion is very inexpensive, and so generally, functions will accept
&str
s as arguments unless they need a
String
for some specific
reason.
In certain cases Rust doesn’t have enough information to make this
conversion, known as
Deref
coercion. In the following example a string
slice
&'a str
implements the trait
TraitExample
, and the function
example_func
takes anything that implements the trait. In this case Rust
would need to make two implicit conversions, which Rust doesn’t have the
means to do. For that reason, the following example will not compile.
ⓘ
trait
TraitExample {}
impl
<
'a
> TraitExample
for
&
'a
str {}
fn
example_func<A: TraitExample>(example_arg: A) {}
let
example_string = String::from(
"example_string"
);
example_func(
&
example_string);
There are two options that would work instead. The first would be to
change the line
example_func(&example_string);
to
example_func(example_string.as_str());
, using the method
as_str()
to explicitly extract the string slice containing the string. The second
way changes
example_func(&example_string);
to
example_func(&*example_string);
. In this case we are dereferencing a
String
to a
str
, then referencing the
str
back to
&str
. The second way is more idiomatic, however both work to do the
conversion explicitly rather than relying on the implicit conversion.
§
Representation
A
String
is made up of three components: a pointer to some bytes, a
length, and a capacity. The pointer points to the internal buffer which
String
uses to store its data. The length is the number of bytes currently stored
in the buffer, and the capacity is the size of the buffer in bytes. As such,
the length will always be less than or equal to the capacity.
This buffer is always stored on the heap.
You can look at these with the
as_ptr
,
len
, and
capacity
methods:
use
std::mem;
let
story = String::from(
"Once upon a time..."
);
// Prevent automatically dropping the String's data
let
mut
story = mem::ManuallyDrop::new(story);
let
ptr = story.as_mut_ptr();
let
len = story.len();
let
capacity = story.capacity();
// story has nineteen bytes
assert_eq!
(
19
, len);
// We can re-build a String out of ptr, len, and capacity. This is all
// unsafe because we are responsible for making sure the components are
// valid:
let
s =
unsafe
{ String::from_raw_parts(ptr, len, capacity) } ;
assert_eq!
(String::from(
"Once upon a time..."
), s);
If a
String
has enough capacity, adding elements to it will not
re-allocate. For example, consider this program:
let
mut
s = String::new();
println!
(
"{}"
, s.capacity());
for _ in
0
..
5
{
    s.push_str(
"hello"
);
println!
(
"{}"
, s.capacity());
}
This will output the following:
0
8
16
16
32
32
At first, we have no memory allocated at all, but as we append to the
string, it increases its capacity appropriately. If we instead use the
with_capacity
method to allocate the correct capacity initially:
let
mut
s = String::with_capacity(
25
);
println!
(
"{}"
, s.capacity());
for _ in
0
..
5
{
    s.push_str(
"hello"
);
println!
(
"{}"
, s.capacity());
}
We end up with a different output:
25
25
25
25
25
25
Here, there’s no need to allocate more memory inside the loop.
Implementations
§
Source
§
impl
String
1.0.0 (const: 1.39.0)
·
Source
pub const fn
new
() ->
String
Creates a new empty
String
.
Given that the
String
is empty, this will not allocate any initial
buffer. While that means that this initial operation is very
inexpensive, it may cause excessive allocation later when you add
data. If you have an idea of how much data the
String
will hold,
consider the
with_capacity
method to prevent excessive
re-allocation.
§
Examples
let
s = String::new();
1.0.0
·
Source
pub fn
with_capacity
(capacity:
usize
) ->
String
Creates a new empty
String
with at least the specified capacity.
String
s have an internal buffer to hold their data. The capacity is
the length of that buffer, and can be queried with the
capacity
method. This method creates an empty
String
, but one with an initial
buffer that can hold at least
capacity
bytes. This is useful when you
may be appending a bunch of data to the
String
, reducing the number of
reallocations it needs to do.
If the given capacity is
0
, no allocation will occur, and this method
is identical to the
new
method.
§
Examples
let
mut
s = String::with_capacity(
10
);
// The String contains no chars, even though it has capacity for more
assert_eq!
(s.len(),
0
);
// These are all done without reallocating...
let
cap = s.capacity();
for _ in
0
..
10
{
    s.push(
'a'
);
}
assert_eq!
(s.capacity(), cap);
// ...but this may make the string reallocate
s.push(
'a'
);
Source
pub fn
try_with_capacity
(capacity:
usize
) ->
Result
<
String
,
TryReserveError
>
🔬
This is a nightly-only experimental API. (
try_with_capacity
#91913
)
Creates a new empty
String
with at least the specified capacity.
§
Errors
Returns
Err
if the capacity exceeds
isize::MAX
bytes,
or if the memory allocator reports failure.
1.0.0
·
Source
pub fn
from_utf8
(vec:
Vec
<
u8
>) ->
Result
<
String
,
FromUtf8Error
>
Converts a vector of bytes to a
String
.
A string (
String
) is made of bytes (
u8
), and a vector of bytes
(
Vec<u8>
) is made of bytes, so this function converts between the
two. Not all byte slices are valid
String
s, however:
String
requires that it is valid UTF-8.
from_utf8()
checks to ensure that
the bytes are valid UTF-8, and then does the conversion.
If you are sure that the byte slice is valid UTF-8, and you don’t want
to incur the overhead of the validity check, there is an unsafe version
of this function,
from_utf8_unchecked
, which has the same behavior
but skips the check.
This method will take care to not copy the vector, for efficiency’s
sake.
If you need a
&str
instead of a
String
, consider
str::from_utf8
.
The inverse of this method is
into_bytes
.
§
Errors
Returns
Err
if the slice is not UTF-8 with a description as to why the
provided bytes are not UTF-8. The vector you moved in is also included.
§
Examples
Basic usage:
// some bytes, in a vector
let
sparkle_heart =
vec!
[
240
,
159
,
146
,
150
];
// We know these bytes are valid, so we'll use `unwrap()`.
let
sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
assert_eq!
(
"💖"
, sparkle_heart);
Incorrect bytes:
// some invalid bytes, in a vector
let
sparkle_heart =
vec!
[
0
,
159
,
146
,
150
];
assert!
(String::from_utf8(sparkle_heart).is_err());
See the docs for
FromUtf8Error
for more details on what you can do
with this error.
1.0.0
·
Source
pub fn
from_utf8_lossy
(v: &[
u8
]) ->
Cow
<'_,
str
>
Converts a slice of bytes to a string, including invalid characters.
Strings are made of bytes (
u8
), and a slice of bytes
(
&[u8]
) is made of bytes, so this function converts
between the two. Not all byte slices are valid strings, however: strings
are required to be valid UTF-8. During this conversion,
from_utf8_lossy()
will replace any invalid UTF-8 sequences with
U+FFFD REPLACEMENT CHARACTER
, which looks like this: �
If you are sure that the byte slice is valid UTF-8, and you don’t want
to incur the overhead of the conversion, there is an unsafe version
of this function,
from_utf8_unchecked
, which has the same behavior
but skips the checks.
This function returns a
Cow<'a, str>
. If our byte slice is invalid
UTF-8, then we need to insert the replacement characters, which will
change the size of the string, and hence, require a
String
. But if
it’s already valid UTF-8, we don’t need a new allocation. This return
type allows us to handle both cases.
§
Examples
Basic usage:
// some bytes, in a vector
let
sparkle_heart =
vec!
[
240
,
159
,
146
,
150
];
let
sparkle_heart = String::from_utf8_lossy(
&
sparkle_heart);
assert_eq!
(
"💖"
, sparkle_heart);
Incorrect bytes:
// some invalid bytes
let
input =
b"Hello \xF0\x90\x80World"
;
let
output = String::from_utf8_lossy(input);
assert_eq!
(
"Hello �World"
, output);
Source
pub fn
from_utf8_lossy_owned
(v:
Vec
<
u8
>) ->
String
🔬
This is a nightly-only experimental API. (
string_from_utf8_lossy_owned
#129436
)
Converts a
Vec<u8>
to a
String
, substituting invalid UTF-8
sequences with replacement characters.
See
from_utf8_lossy
for more details.
Note that this function does not guarantee reuse of the original
Vec
allocation.
§
Examples
Basic usage:
#![feature(string_from_utf8_lossy_owned)]
// some bytes, in a vector
let
sparkle_heart =
vec!
[
240
,
159
,
146
,
150
];
let
sparkle_heart = String::from_utf8_lossy_owned(sparkle_heart);
assert_eq!
(String::from(
"💖"
), sparkle_heart);
Incorrect bytes:
#![feature(string_from_utf8_lossy_owned)]
// some invalid bytes
let
input: Vec<u8> =
b"Hello \xF0\x90\x80World"
.into();
let
output = String::from_utf8_lossy_owned(input);
assert_eq!
(String::from(
"Hello �World"
), output);
1.0.0
·
Source
pub fn
from_utf16
(v: &[
u16
]) ->
Result
<
String
,
FromUtf16Error
>
Decode a native endian UTF-16–encoded vector
v
into a
String
,
returning
Err
if
v
contains any invalid data.
§
Examples
// 𝄞music
let
v =
&
[
0xD834
,
0xDD1E
,
0x006d
,
0x0075
,
0x0073
,
0x0069
,
0x0063
];
assert_eq!
(String::from(
"𝄞music"
),
           String::from_utf16(v).unwrap());
// 𝄞mu<invalid>ic
let
v =
&
[
0xD834
,
0xDD1E
,
0x006d
,
0x0075
,
0xD800
,
0x0069
,
0x0063
];
assert!
(String::from_utf16(v).is_err());
1.0.0
·
Source
pub fn
from_utf16_lossy
(v: &[
u16
]) ->
String
Decode a native endian UTF-16–encoded slice
v
into a
String
,
replacing invalid data with
the replacement character (
U+FFFD
)
.
Unlike
from_utf8_lossy
which returns a
Cow<'a, str>
,
from_utf16_lossy
returns a
String
since the UTF-16 to UTF-8
conversion requires a memory allocation.
§
Examples
// 𝄞mus<invalid>ic<invalid>
let
v =
&
[
0xD834
,
0xDD1E
,
0x006d
,
0x0075
,
0x0073
,
0xDD1E
,
0x0069
,
0x0063
,
0xD834
];
assert_eq!
(String::from(
"𝄞mus\u{FFFD}ic\u{FFFD}"
),
           String::from_utf16_lossy(v));
Source
pub fn
from_utf16le
(v: &[
u8
]) ->
Result
<
String
,
FromUtf16Error
>
🔬
This is a nightly-only experimental API. (
str_from_utf16_endian
#116258
)
Decode a UTF-16LE–encoded vector
v
into a
String
,
returning
Err
if
v
contains any invalid data.
§
Examples
Basic usage:
#![feature(str_from_utf16_endian)]
// 𝄞music
let
v =
&
[
0x34
,
0xD8
,
0x1E
,
0xDD
,
0x6d
,
0x00
,
0x75
,
0x00
,
0x73
,
0x00
,
0x69
,
0x00
,
0x63
,
0x00
];
assert_eq!
(String::from(
"𝄞music"
),
           String::from_utf16le(v).unwrap());
// 𝄞mu<invalid>ic
let
v =
&
[
0x34
,
0xD8
,
0x1E
,
0xDD
,
0x6d
,
0x00
,
0x75
,
0x00
,
0x00
,
0xD8
,
0x69
,
0x00
,
0x63
,
0x00
];
assert!
(String::from_utf16le(v).is_err());
Source
pub fn
from_utf16le_lossy
(v: &[
u8
]) ->
String
🔬
This is a nightly-only experimental API. (
str_from_utf16_endian
#116258
)
Decode a UTF-16LE–encoded slice
v
into a
String
, replacing
invalid data with
the replacement character (
U+FFFD
)
.
Unlike
from_utf8_lossy
which returns a
Cow<'a, str>
,
from_utf16le_lossy
returns a
String
since the UTF-16 to UTF-8
conversion requires a memory allocation.
§
Examples
Basic usage:
#![feature(str_from_utf16_endian)]
// 𝄞mus<invalid>ic<invalid>
let
v =
&
[
0x34
,
0xD8
,
0x1E
,
0xDD
,
0x6d
,
0x00
,
0x75
,
0x00
,
0x73
,
0x00
,
0x1E
,
0xDD
,
0x69
,
0x00
,
0x63
,
0x00
,
0x34
,
0xD8
];
assert_eq!
(String::from(
"𝄞mus\u{FFFD}ic\u{FFFD}"
),
           String::from_utf16le_lossy(v));
Source
pub fn
from_utf16be
(v: &[
u8
]) ->
Result
<
String
,
FromUtf16Error
>
🔬
This is a nightly-only experimental API. (
str_from_utf16_endian
#116258
)
Decode a UTF-16BE–encoded vector
v
into a
String
,
returning
Err
if
v
contains any invalid data.
§
Examples
Basic usage:
#![feature(str_from_utf16_endian)]
// 𝄞music
let
v =
&
[
0xD8
,
0x34
,
0xDD
,
0x1E
,
0x00
,
0x6d
,
0x00
,
0x75
,
0x00
,
0x73
,
0x00
,
0x69
,
0x00
,
0x63
];
assert_eq!
(String::from(
"𝄞music"
),
           String::from_utf16be(v).unwrap());
// 𝄞mu<invalid>ic
let
v =
&
[
0xD8
,
0x34
,
0xDD
,
0x1E
,
0x00
,
0x6d
,
0x00
,
0x75
,
0xD8
,
0x00
,
0x00
,
0x69
,
0x00
,
0x63
];
assert!
(String::from_utf16be(v).is_err());
Source
pub fn
from_utf16be_lossy
(v: &[
u8
]) ->
String
🔬
This is a nightly-only experimental API. (
str_from_utf16_endian
#116258
)
Decode a UTF-16BE–encoded slice
v
into a
String
, replacing
invalid data with
the replacement character (
U+FFFD
)
.
Unlike
from_utf8_lossy
which returns a
Cow<'a, str>
,
from_utf16le_lossy
returns a
String
since the UTF-16 to UTF-8
conversion requires a memory allocation.
§
Examples
Basic usage:
#![feature(str_from_utf16_endian)]
// 𝄞mus<invalid>ic<invalid>
let
v =
&
[
0xD8
,
0x34
,
0xDD
,
0x1E
,
0x00
,
0x6d
,
0x00
,
0x75
,
0x00
,
0x73
,
0xDD
,
0x1E
,
0x00
,
0x69
,
0x00
,
0x63
,
0xD8
,
0x34
];
assert_eq!
(String::from(
"𝄞mus\u{FFFD}ic\u{FFFD}"
),
           String::from_utf16be_lossy(v));
Source
pub fn
into_raw_parts
(self) -> (
*mut
u8
,
usize
,
usize
)
🔬
This is a nightly-only experimental API. (
vec_into_raw_parts
#65816
)
Decomposes a
String
into its raw components:
(pointer, length, capacity)
.
Returns the raw pointer to the underlying data, the length of
the string (in bytes), and the allocated capacity of the data
(in bytes). These are the same arguments in the same order as
the arguments to
from_raw_parts
.
After calling this function, the caller is responsible for the
memory previously managed by the
String
. The only way to do
this is to convert the raw pointer, length, and capacity back
into a
String
with the
from_raw_parts
function, allowing
the destructor to perform the cleanup.
§
Examples
#![feature(vec_into_raw_parts)]
let
s = String::from(
"hello"
);
let
(ptr, len, cap) = s.into_raw_parts();
let
rebuilt =
unsafe
{ String::from_raw_parts(ptr, len, cap) };
assert_eq!
(rebuilt,
"hello"
);
1.0.0
·
Source
pub unsafe fn
from_raw_parts
(
    buf:
*mut
u8
,
    length:
usize
,
    capacity:
usize
,
) ->
String
Creates a new
String
from a pointer, a length and a capacity.
§
Safety
This is highly unsafe, due to the number of invariants that aren’t
checked:
all safety requirements for
Vec::<u8>::from_raw_parts
.
all safety requirements for
String::from_utf8_unchecked
.
Violating these may cause problems like corrupting the allocator’s
internal data structures. For example, it is normally
not
safe to
build a
String
from a pointer to a C
char
array containing UTF-8
unless
you are certain that array was originally allocated by the
Rust standard library’s allocator.
The ownership of
buf
is effectively transferred to the
String
which may then deallocate, reallocate or change the
contents of memory pointed to by the pointer at will. Ensure
that nothing else uses the pointer after calling this
function.
§
Examples
use
std::mem;
unsafe
{
let
s = String::from(
"hello"
);
// Prevent automatically dropping the String's data
let
mut
s = mem::ManuallyDrop::new(s);
let
ptr = s.as_mut_ptr();
let
len = s.len();
let
capacity = s.capacity();
let
s = String::from_raw_parts(ptr, len, capacity);
assert_eq!
(String::from(
"hello"
), s);
}
1.0.0
·
Source
pub unsafe fn
from_utf8_unchecked
(bytes:
Vec
<
u8
>) ->
String
Converts a vector of bytes to a
String
without checking that the
string contains valid UTF-8.
See the safe version,
from_utf8
, for more details.
§
Safety
This function is unsafe because it does not check that the bytes passed
to it are valid UTF-8. If this constraint is violated, it may cause
memory unsafety issues with future users of the
String
, as the rest of
the standard library assumes that
String
s are valid UTF-8.
§
Examples
// some bytes, in a vector
let
sparkle_heart =
vec!
[
240
,
159
,
146
,
150
];
let
sparkle_heart =
unsafe
{
    String::from_utf8_unchecked(sparkle_heart)
};
assert_eq!
(
"💖"
, sparkle_heart);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
into_bytes
(self) ->
Vec
<
u8
>
ⓘ
Converts a
String
into a byte vector.
This consumes the
String
, so we do not need to copy its contents.
§
Examples
let
s = String::from(
"hello"
);
let
bytes = s.into_bytes();
assert_eq!
(
&
[
104
,
101
,
108
,
108
,
111
][..],
&
bytes[..]);
1.7.0 (const: 1.87.0)
·
Source
pub const fn
as_str
(&self) -> &
str
Extracts a string slice containing the entire
String
.
§
Examples
let
s = String::from(
"foo"
);
assert_eq!
(
"foo"
, s.as_str());
1.7.0 (const: 1.87.0)
·
Source
pub const fn
as_mut_str
(&mut self) -> &mut
str
Converts a
String
into a mutable string slice.
§
Examples
let
mut
s = String::from(
"foobar"
);
let
s_mut_str = s.as_mut_str();

s_mut_str.make_ascii_uppercase();
assert_eq!
(
"FOOBAR"
, s_mut_str);
1.0.0
·
Source
pub fn
push_str
(&mut self, string: &
str
)
Appends a given string slice onto the end of this
String
.
§
Examples
let
mut
s = String::from(
"foo"
);

s.push_str(
"bar"
);
assert_eq!
(
"foobar"
, s);
1.87.0
·
Source
pub fn
extend_from_within
<R>(&mut self, src: R)
where
    R:
RangeBounds
<
usize
>,
Copies elements from
src
range to the end of the string.
§
Panics
Panics if the starting point or end point do not lie on a
char
boundary, or if they’re out of bounds.
§
Examples
let
mut
string = String::from(
"abcde"
);

string.extend_from_within(
2
..);
assert_eq!
(string,
"abcdecde"
);

string.extend_from_within(..
2
);
assert_eq!
(string,
"abcdecdeab"
);

string.extend_from_within(
4
..
8
);
assert_eq!
(string,
"abcdecdeabecde"
);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
capacity
(&self) ->
usize
Returns this
String
’s capacity, in bytes.
§
Examples
let
s = String::with_capacity(
10
);
assert!
(s.capacity() >=
10
);
1.0.0
·
Source
pub fn
reserve
(&mut self, additional:
usize
)
Reserves capacity for at least
additional
bytes more than the
current length. The allocator may reserve more space to speculatively
avoid frequent allocations. After calling
reserve
,
capacity will be greater than or equal to
self.len() + additional
.
Does nothing if capacity is already sufficient.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
Basic usage:
let
mut
s = String::new();

s.reserve(
10
);
assert!
(s.capacity() >=
10
);
This might not actually increase the capacity:
let
mut
s = String::with_capacity(
10
);
s.push(
'a'
);
s.push(
'b'
);
// s now has a length of 2 and a capacity of at least 10
let
capacity = s.capacity();
assert_eq!
(
2
, s.len());
assert!
(capacity >=
10
);
// Since we already have at least an extra 8 capacity, calling this...
s.reserve(
8
);
// ... doesn't actually increase.
assert_eq!
(capacity, s.capacity());
1.0.0
·
Source
pub fn
reserve_exact
(&mut self, additional:
usize
)
Reserves the minimum capacity for at least
additional
bytes more than
the current length. Unlike
reserve
, this will not
deliberately over-allocate to speculatively avoid frequent allocations.
After calling
reserve_exact
, capacity will be greater than or equal to
self.len() + additional
. Does nothing if the capacity is already
sufficient.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
Basic usage:
let
mut
s = String::new();

s.reserve_exact(
10
);
assert!
(s.capacity() >=
10
);
This might not actually increase the capacity:
let
mut
s = String::with_capacity(
10
);
s.push(
'a'
);
s.push(
'b'
);
// s now has a length of 2 and a capacity of at least 10
let
capacity = s.capacity();
assert_eq!
(
2
, s.len());
assert!
(capacity >=
10
);
// Since we already have at least an extra 8 capacity, calling this...
s.reserve_exact(
8
);
// ... doesn't actually increase.
assert_eq!
(capacity, s.capacity());
1.57.0
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
bytes more than the
current length. The allocator may reserve more space to speculatively
avoid frequent allocations. After calling
try_reserve
, capacity will be
greater than or equal to
self.len() + additional
if it returns
Ok(())
. Does nothing if capacity is already sufficient. This method
preserves the contents even if an error occurs.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
fn
process_data(data:
&
str) ->
Result
<String, TryReserveError> {
let
mut
output = String::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
output.push_str(data);
Ok
(output)
}
1.57.0
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
bytes
more than the current length. Unlike
try_reserve
, this will not
deliberately over-allocate to speculatively avoid frequent allocations.
After calling
try_reserve_exact
, capacity will be greater than or
equal to
self.len() + additional
if it returns
Ok(())
.
Does nothing if the capacity is already sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
try_reserve
if future insertions are expected.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
fn
process_data(data:
&
str) ->
Result
<String, TryReserveError> {
let
mut
output = String::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve_exact(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
output.push_str(data);
Ok
(output)
}
1.0.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Shrinks the capacity of this
String
to match its length.
§
Examples
let
mut
s = String::from(
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
Shrinks the capacity of this
String
with a lower bound.
The capacity will remain at least as large as both the length
and the supplied value.
If the current capacity is less than the lower limit, this is a no-op.
§
Examples
let
mut
s = String::from(
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
1.0.0
·
Source
pub fn
push
(&mut self, ch:
char
)
Appends the given
char
to the end of this
String
.
§
Examples
let
mut
s = String::from(
"abc"
);

s.push(
'1'
);
s.push(
'2'
);
s.push(
'3'
);
assert_eq!
(
"abc123"
, s);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Returns a byte slice of this
String
’s contents.
The inverse of this method is
from_utf8
.
§
Examples
let
s = String::from(
"hello"
);
assert_eq!
(
&
[
104
,
101
,
108
,
108
,
111
], s.as_bytes());
1.0.0
·
Source
pub fn
truncate
(&mut self, new_len:
usize
)
Shortens this
String
to the specified length.
If
new_len
is greater than or equal to the string’s current length, this has no
effect.
Note that this method has no effect on the allocated capacity
of the string
§
Panics
Panics if
new_len
does not lie on a
char
boundary.
§
Examples
let
mut
s = String::from(
"hello"
);

s.truncate(
2
);
assert_eq!
(
"he"
, s);
1.0.0
·
Source
pub fn
pop
(&mut self) ->
Option
<
char
>
Removes the last character from the string buffer and returns it.
Returns
None
if this
String
is empty.
§
Examples
let
mut
s = String::from(
"abč"
);
assert_eq!
(s.pop(),
Some
(
'č'
));
assert_eq!
(s.pop(),
Some
(
'b'
));
assert_eq!
(s.pop(),
Some
(
'a'
));
assert_eq!
(s.pop(),
None
);
1.0.0
·
Source
pub fn
remove
(&mut self, idx:
usize
) ->
char
Removes a
char
from this
String
at a byte position and returns it.
This is an
O
(
n
) operation, as it requires copying every element in the
buffer.
§
Panics
Panics if
idx
is larger than or equal to the
String
’s length,
or if it does not lie on a
char
boundary.
§
Examples
let
mut
s = String::from(
"abç"
);
assert_eq!
(s.remove(
0
),
'a'
);
assert_eq!
(s.remove(
1
),
'ç'
);
assert_eq!
(s.remove(
0
),
'b'
);
Source
pub fn
remove_matches
<P>(&mut self, pat: P)
where
    P:
Pattern
,
🔬
This is a nightly-only experimental API. (
string_remove_matches
#72826
)
Remove all matches of pattern
pat
in the
String
.
§
Examples
#![feature(string_remove_matches)]
let
mut
s = String::from(
"Trees are not green, the sky is not blue."
);
s.remove_matches(
"not "
);
assert_eq!
(
"Trees are green, the sky is blue."
, s);
Matches will be detected and removed iteratively, so in cases where
patterns overlap, only the first pattern will be removed:
#![feature(string_remove_matches)]
let
mut
s = String::from(
"banana"
);
s.remove_matches(
"ana"
);
assert_eq!
(
"bna"
, s);
1.26.0
·
Source
pub fn
retain
<F>(&mut self, f: F)
where
    F:
FnMut
(
char
) ->
bool
,
Retains only the characters specified by the predicate.
In other words, remove all characters
c
such that
f(c)
returns
false
.
This method operates in place, visiting each character exactly once in the
original order, and preserves the order of the retained characters.
§
Examples
let
mut
s = String::from(
"f_o_ob_ar"
);

s.retain(|c| c !=
'_'
);
assert_eq!
(s,
"foobar"
);
Because the elements are visited exactly once in the original order,
external state may be used to decide which elements to keep.
let
mut
s = String::from(
"abcde"
);
let
keep = [
false
,
true
,
true
,
false
,
true
];
let
mut
iter = keep.iter();
s.retain(|
_
|
*
iter.next().unwrap());
assert_eq!
(s,
"bce"
);
1.0.0
·
Source
pub fn
insert
(&mut self, idx:
usize
, ch:
char
)
Inserts a character into this
String
at a byte position.
This is an
O
(
n
) operation as it requires copying every element in the
buffer.
§
Panics
Panics if
idx
is larger than the
String
’s length, or if it does not
lie on a
char
boundary.
§
Examples
let
mut
s = String::with_capacity(
3
);

s.insert(
0
,
'f'
);
s.insert(
1
,
'o'
);
s.insert(
2
,
'o'
);
assert_eq!
(
"foo"
, s);
1.16.0
·
Source
pub fn
insert_str
(&mut self, idx:
usize
, string: &
str
)
Inserts a string slice into this
String
at a byte position.
This is an
O
(
n
) operation as it requires copying every element in the
buffer.
§
Panics
Panics if
idx
is larger than the
String
’s length, or if it does not
lie on a
char
boundary.
§
Examples
let
mut
s = String::from(
"bar"
);

s.insert_str(
0
,
"foo"
);
assert_eq!
(
"foobar"
, s);
1.0.0 (const: 1.87.0)
·
Source
pub const unsafe fn
as_mut_vec
(&mut self) -> &mut
Vec
<
u8
>
ⓘ
Returns a mutable reference to the contents of this
String
.
§
Safety
This function is unsafe because the returned
&mut Vec
allows writing
bytes which are not valid UTF-8. If this constraint is violated, using
the original
String
after dropping the
&mut Vec
may violate memory
safety, as the rest of the standard library assumes that
String
s are
valid UTF-8.
§
Examples
let
mut
s = String::from(
"hello"
);
unsafe
{
let
vec = s.as_mut_vec();
assert_eq!
(
&
[
104
,
101
,
108
,
108
,
111
][..],
&
vec[..]);

    vec.reverse();
}
assert_eq!
(s,
"olleh"
);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
len
(&self) ->
usize
Returns the length of this
String
, in bytes, not
char
s or
graphemes. In other words, it might not be what a human considers the
length of the string.
§
Examples
let
a = String::from(
"foo"
);
assert_eq!
(a.len(),
3
);
let
fancy_f = String::from(
"ƒoo"
);
assert_eq!
(fancy_f.len(),
4
);
assert_eq!
(fancy_f.chars().count(),
3
);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
is_empty
(&self) ->
bool
Returns
true
if this
String
has a length of zero, and
false
otherwise.
§
Examples
let
mut
v = String::new();
assert!
(v.is_empty());

v.push(
'a'
);
assert!
(!v.is_empty());
1.16.0
·
Source
pub fn
split_off
(&mut self, at:
usize
) ->
String
Splits the string into two at the given byte index.
Returns a newly allocated
String
.
self
contains bytes
[0, at)
, and
the returned
String
contains bytes
[at, len)
.
at
must be on the
boundary of a UTF-8 code point.
Note that the capacity of
self
does not change.
§
Panics
Panics if
at
is not on a
UTF-8
code point boundary, or if it is beyond the last
code point of the string.
§
Examples
let
mut
hello = String::from(
"Hello, World!"
);
let
world = hello.split_off(
7
);
assert_eq!
(hello,
"Hello, "
);
assert_eq!
(world,
"World!"
);
1.0.0
·
Source
pub fn
clear
(&mut self)
Truncates this
String
, removing all contents.
While this means the
String
will have a length of zero, it does not
touch its capacity.
§
Examples
let
mut
s = String::from(
"foo"
);

s.clear();
assert!
(s.is_empty());
assert_eq!
(
0
, s.len());
assert_eq!
(
3
, s.capacity());
1.6.0
·
Source
pub fn
drain
<R>(&mut self, range: R) ->
Drain
<'_>
ⓘ
where
    R:
RangeBounds
<
usize
>,
Removes the specified range from the string in bulk, returning all
removed characters as an iterator.
The returned iterator keeps a mutable borrow on the string to optimize
its implementation.
§
Panics
Panics if the starting point or end point do not lie on a
char
boundary, or if they’re out of bounds.
§
Leaking
If the returned iterator goes out of scope without being dropped (due to
core::mem::forget
, for example), the string may still contain a copy
of any drained characters, or may have lost characters arbitrarily,
including characters outside the range.
§
Examples
let
mut
s = String::from(
"α is alpha, β is beta"
);
let
beta_offset = s.find(
'β'
).unwrap_or(s.len());
// Remove the range up until the β from the string
let
t: String = s.drain(..beta_offset).collect();
assert_eq!
(t,
"α is alpha, "
);
assert_eq!
(s,
"β is beta"
);
// A full range clears the string, like `clear()` does
s.drain(..);
assert_eq!
(s,
""
);
Source
pub fn
into_chars
(self) ->
IntoChars
ⓘ
🔬
This is a nightly-only experimental API. (
string_into_chars
#133125
)
Converts a
String
into an iterator over the
char
s of the string.
As a string consists of valid UTF-8, we can iterate through a string
by
char
. This method returns such an iterator.
It’s important to remember that
char
represents a Unicode Scalar
Value, and might not match your idea of what a ‘character’ is. Iteration
over grapheme clusters may be what you actually want. That functionality
is not provided by Rust’s standard library, check crates.io instead.
§
Examples
Basic usage:
#![feature(string_into_chars)]
let
word = String::from(
"goodbye"
);
let
mut
chars = word.into_chars();
assert_eq!
(
Some
(
'g'
), chars.next());
assert_eq!
(
Some
(
'o'
), chars.next());
assert_eq!
(
Some
(
'o'
), chars.next());
assert_eq!
(
Some
(
'd'
), chars.next());
assert_eq!
(
Some
(
'b'
), chars.next());
assert_eq!
(
Some
(
'y'
), chars.next());
assert_eq!
(
Some
(
'e'
), chars.next());
assert_eq!
(
None
, chars.next());
Remember,
char
s might not match your intuition about characters:
#![feature(string_into_chars)]
let
y = String::from(
"y̆"
);
let
mut
chars = y.into_chars();
assert_eq!
(
Some
(
'y'
), chars.next());
// not 'y̆'
assert_eq!
(
Some
(
'\u{0306}'
), chars.next());
assert_eq!
(
None
, chars.next());
1.27.0
·
Source
pub fn
replace_range
<R>(&mut self, range: R, replace_with: &
str
)
where
    R:
RangeBounds
<
usize
>,
Removes the specified range in the string,
and replaces it with the given string.
The given string doesn’t need to be the same length as the range.
§
Panics
Panics if the starting point or end point do not lie on a
char
boundary, or if they’re out of bounds.
§
Examples
let
mut
s = String::from(
"α is alpha, β is beta"
);
let
beta_offset = s.find(
'β'
).unwrap_or(s.len());
// Replace the range up until the β from the string
s.replace_range(..beta_offset,
"Α is capital alpha; "
);
assert_eq!
(s,
"Α is capital alpha; β is beta"
);
1.4.0
·
Source
pub fn
into_boxed_str
(self) ->
Box
<
str
>
Converts this
String
into a
Box
<
str
>
.
Before doing the conversion, this method discards excess capacity like
shrink_to_fit
.
Note that this call may reallocate and copy the bytes of the string.
§
Examples
let
s = String::from(
"hello"
);
let
b = s.into_boxed_str();
1.72.0
·
Source
pub fn
leak
<'a>(self) -> &'a mut
str
Consumes and leaks the
String
, returning a mutable reference to the contents,
&'a mut str
.
The caller has free choice over the returned lifetime, including
'static
. Indeed,
this function is ideally used for data that lives for the remainder of the program’s life,
as dropping the returned reference will cause a memory leak.
It does not reallocate or shrink the
String
, so the leaked allocation may include unused
capacity that is not part of the returned slice. If you want to discard excess capacity,
call
into_boxed_str
, and then
Box::leak
instead. However, keep in mind that
trimming the capacity may result in a reallocation and copy.
§
Examples
let
x = String::from(
"bucket"
);
let
static_ref:
&
'static
mut
str = x.leak();
assert_eq!
(static_ref,
"bucket"
);
Methods from
Deref
<Target =
str
>
§
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the length of
self
.
This length is in bytes, not
char
s or graphemes. In other words,
it might not be what a human considers the length of the string.
§
Examples
let
len =
"foo"
.len();
assert_eq!
(
3
, len);
assert_eq!
(
"ƒoo"
.len(),
4
);
// fancy f!
assert_eq!
(
"ƒoo"
.chars().count(),
3
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if
self
has a length of zero bytes.
§
Examples
let
s =
""
;
assert!
(s.is_empty());
let
s =
"not empty"
;
assert!
(!s.is_empty());
1.9.0
·
Source
pub fn
is_char_boundary
(&self, index:
usize
) ->
bool
Checks that
index
-th byte is the first byte in a UTF-8 code point
sequence or the end of the string.
The start and end of the string (when
index == self.len()
) are
considered to be boundaries.
Returns
false
if
index
is greater than
self.len()
.
§
Examples
let
s =
"Löwe 老虎 Léopard"
;
assert!
(s.is_char_boundary(
0
));
// start of `老`
assert!
(s.is_char_boundary(
6
));
assert!
(s.is_char_boundary(s.len()));
// second byte of `ö`
assert!
(!s.is_char_boundary(
2
));
// third byte of `老`
assert!
(!s.is_char_boundary(
8
));
Source
pub fn
floor_char_boundary
(&self, index:
usize
) ->
usize
🔬
This is a nightly-only experimental API. (
round_char_boundary
#93743
)
Finds the closest
x
not exceeding
index
where
is_char_boundary(x)
is
true
.
This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t
exceed a given number of bytes. Note that this is done purely at the character level
and can still visually split graphemes, even though the underlying characters aren’t
split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only
includes 🧑 (person) instead.
§
Examples
#![feature(round_char_boundary)]
let
s =
"❤️🧡💛💚💙💜"
;
assert_eq!
(s.len(),
26
);
assert!
(!s.is_char_boundary(
13
));
let
closest = s.floor_char_boundary(
13
);
assert_eq!
(closest,
10
);
assert_eq!
(
&
s[..closest],
"❤️🧡"
);
Source
pub fn
ceil_char_boundary
(&self, index:
usize
) ->
usize
🔬
This is a nightly-only experimental API. (
round_char_boundary
#93743
)
Finds the closest
x
not below
index
where
is_char_boundary(x)
is
true
.
If
index
is greater than the length of the string, this returns the length of the string.
This method is the natural complement to
floor_char_boundary
. See that method
for more details.
§
Examples
#![feature(round_char_boundary)]
let
s =
"❤️🧡💛💚💙💜"
;
assert_eq!
(s.len(),
26
);
assert!
(!s.is_char_boundary(
13
));
let
closest = s.ceil_char_boundary(
13
);
assert_eq!
(closest,
14
);
assert_eq!
(
&
s[..closest],
"❤️🧡💛"
);
1.0.0
·
Source
pub fn
as_bytes
(&self) -> &[
u8
]
ⓘ
Converts a string slice to a byte slice. To convert the byte slice back
into a string slice, use the
from_utf8
function.
§
Examples
let
bytes =
"bors"
.as_bytes();
assert_eq!
(
b"bors"
, bytes);
1.20.0
·
Source
pub unsafe fn
as_bytes_mut
(&mut self) -> &mut [
u8
]
ⓘ
Converts a mutable string slice to a mutable byte slice.
§
Safety
The caller must ensure that the content of the slice is valid UTF-8
before the borrow ends and the underlying
str
is used.
Use of a
str
whose contents are not valid UTF-8 is undefined behavior.
§
Examples
Basic usage:
let
mut
s = String::from(
"Hello"
);
let
bytes =
unsafe
{ s.as_bytes_mut() };
assert_eq!
(
b"Hello"
, bytes);
Mutability:
let
mut
s = String::from(
"🗻∈🌏"
);
unsafe
{
let
bytes = s.as_bytes_mut();

    bytes[
0
] =
0xF0
;
    bytes[
1
] =
0x9F
;
    bytes[
2
] =
0x8D
;
    bytes[
3
] =
0x94
;
}
assert_eq!
(
"🍔∈🌏"
, s);
1.0.0
·
Source
pub fn
as_ptr
(&self) ->
*const
u8
Converts a string slice to a raw pointer.
As string slices are a slice of bytes, the raw pointer points to a
u8
. This pointer will be pointing to the first byte of the string
slice.
The caller must ensure that the returned pointer is never written to.
If you need to mutate the contents of the string slice, use
as_mut_ptr
.
§
Examples
let
s =
"Hello"
;
let
ptr = s.as_ptr();
1.36.0
·
Source
pub fn
as_mut_ptr
(&mut self) ->
*mut
u8
Converts a mutable string slice to a raw pointer.
As string slices are a slice of bytes, the raw pointer points to a
u8
. This pointer will be pointing to the first byte of the string
slice.
It is your responsibility to make sure that the string slice only gets
modified in a way that it remains valid UTF-8.
1.20.0
·
Source
pub fn
get
<I>(&self, i: I) ->
Option
<&<I as
SliceIndex
<
str
>>::
Output
>
where
    I:
SliceIndex
<
str
>,
Returns a subslice of
str
.
This is the non-panicking alternative to indexing the
str
. Returns
None
whenever equivalent indexing operation would panic.
§
Examples
let
v = String::from(
"🗻∈🌏"
);
assert_eq!
(
Some
(
"🗻"
), v.get(
0
..
4
));
// indices not on UTF-8 sequence boundaries
assert!
(v.get(
1
..).is_none());
assert!
(v.get(..
8
).is_none());
// out of bounds
assert!
(v.get(..
42
).is_none());
1.20.0
·
Source
pub fn
get_mut
<I>(
    &mut self,
    i: I,
) ->
Option
<&mut <I as
SliceIndex
<
str
>>::
Output
>
where
    I:
SliceIndex
<
str
>,
Returns a mutable subslice of
str
.
This is the non-panicking alternative to indexing the
str
. Returns
None
whenever equivalent indexing operation would panic.
§
Examples
let
mut
v = String::from(
"hello"
);
// correct length
assert!
(v.get_mut(
0
..
5
).is_some());
// out of bounds
assert!
(v.get_mut(..
42
).is_none());
assert_eq!
(
Some
(
"he"
), v.get_mut(
0
..
2
).map(|v|
&*
v));
assert_eq!
(
"hello"
, v);
{
let
s = v.get_mut(
0
..
2
);
let
s = s.map(|s| {
        s.make_ascii_uppercase();
&*
s
    });
assert_eq!
(
Some
(
"HE"
), s);
}
assert_eq!
(
"HEllo"
, v);
1.20.0
·
Source
pub unsafe fn
get_unchecked
<I>(&self, i: I) -> &<I as
SliceIndex
<
str
>>::
Output
where
    I:
SliceIndex
<
str
>,
Returns an unchecked subslice of
str
.
This is the unchecked alternative to indexing the
str
.
§
Safety
Callers of this function are responsible that these preconditions are
satisfied:
The starting index must not exceed the ending index;
Indexes must be within bounds of the original slice;
Indexes must lie on UTF-8 sequence boundaries.
Failing that, the returned string slice may reference invalid memory or
violate the invariants communicated by the
str
type.
§
Examples
let
v =
"🗻∈🌏"
;
unsafe
{
assert_eq!
(
"🗻"
, v.get_unchecked(
0
..
4
));
assert_eq!
(
"∈"
, v.get_unchecked(
4
..
7
));
assert_eq!
(
"🌏"
, v.get_unchecked(
7
..
11
));
}
1.20.0
·
Source
pub unsafe fn
get_unchecked_mut
<I>(
    &mut self,
    i: I,
) -> &mut <I as
SliceIndex
<
str
>>::
Output
where
    I:
SliceIndex
<
str
>,
Returns a mutable, unchecked subslice of
str
.
This is the unchecked alternative to indexing the
str
.
§
Safety
Callers of this function are responsible that these preconditions are
satisfied:
The starting index must not exceed the ending index;
Indexes must be within bounds of the original slice;
Indexes must lie on UTF-8 sequence boundaries.
Failing that, the returned string slice may reference invalid memory or
violate the invariants communicated by the
str
type.
§
Examples
let
mut
v = String::from(
"🗻∈🌏"
);
unsafe
{
assert_eq!
(
"🗻"
, v.get_unchecked_mut(
0
..
4
));
assert_eq!
(
"∈"
, v.get_unchecked_mut(
4
..
7
));
assert_eq!
(
"🌏"
, v.get_unchecked_mut(
7
..
11
));
}
1.0.0
·
Source
pub unsafe fn
slice_unchecked
(&self, begin:
usize
, end:
usize
) -> &
str
👎
Deprecated since 1.29.0: use
get_unchecked(begin..end)
instead
Creates a string slice from another string slice, bypassing safety
checks.
This is generally not recommended, use with caution! For a safe
alternative see
str
and
Index
.
This new slice goes from
begin
to
end
, including
begin
but
excluding
end
.
To get a mutable string slice instead, see the
slice_mut_unchecked
method.
§
Safety
Callers of this function are responsible that three preconditions are
satisfied:
begin
must not exceed
end
.
begin
and
end
must be byte positions within the string slice.
begin
and
end
must lie on UTF-8 sequence boundaries.
§
Examples
let
s =
"Löwe 老虎 Léopard"
;
unsafe
{
assert_eq!
(
"Löwe 老虎 Léopard"
, s.slice_unchecked(
0
,
21
));
}
let
s =
"Hello, world!"
;
unsafe
{
assert_eq!
(
"world"
, s.slice_unchecked(
7
,
12
));
}
1.5.0
·
Source
pub unsafe fn
slice_mut_unchecked
(
    &mut self,
    begin:
usize
,
    end:
usize
,
) -> &mut
str
👎
Deprecated since 1.29.0: use
get_unchecked_mut(begin..end)
instead
Creates a string slice from another string slice, bypassing safety
checks.
This is generally not recommended, use with caution! For a safe
alternative see
str
and
IndexMut
.
This new slice goes from
begin
to
end
, including
begin
but
excluding
end
.
To get an immutable string slice instead, see the
slice_unchecked
method.
§
Safety
Callers of this function are responsible that three preconditions are
satisfied:
begin
must not exceed
end
.
begin
and
end
must be byte positions within the string slice.
begin
and
end
must lie on UTF-8 sequence boundaries.
1.4.0
·
Source
pub fn
split_at
(&self, mid:
usize
) -> (&
str
, &
str
)
Divides one string slice into two at an index.
The argument,
mid
, should be a byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point.
The two slices returned go from the start of the string slice to
mid
,
and from
mid
to the end of the string slice.
To get mutable string slices instead, see the
split_at_mut
method.
§
Panics
Panics if
mid
is not on a UTF-8 code point boundary, or if it is past
the end of the last code point of the string slice.  For a non-panicking
alternative see
split_at_checked
.
§
Examples
let
s =
"Per Martin-Löf"
;
let
(first, last) = s.split_at(
3
);
assert_eq!
(
"Per"
, first);
assert_eq!
(
" Martin-Löf"
, last);
1.4.0
·
Source
pub fn
split_at_mut
(&mut self, mid:
usize
) -> (&mut
str
, &mut
str
)
Divides one mutable string slice into two at an index.
The argument,
mid
, should be a byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point.
The two slices returned go from the start of the string slice to
mid
,
and from
mid
to the end of the string slice.
To get immutable string slices instead, see the
split_at
method.
§
Panics
Panics if
mid
is not on a UTF-8 code point boundary, or if it is past
the end of the last code point of the string slice.  For a non-panicking
alternative see
split_at_mut_checked
.
§
Examples
let
mut
s =
"Per Martin-Löf"
.to_string();
{
let
(first, last) = s.split_at_mut(
3
);
    first.make_ascii_uppercase();
assert_eq!
(
"PER"
, first);
assert_eq!
(
" Martin-Löf"
, last);
}
assert_eq!
(
"PER Martin-Löf"
, s);
1.80.0
·
Source
pub fn
split_at_checked
(&self, mid:
usize
) ->
Option
<(&
str
, &
str
)>
Divides one string slice into two at an index.
The argument,
mid
, should be a valid byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point. The
method returns
None
if that’s not the case.
The two slices returned go from the start of the string slice to
mid
,
and from
mid
to the end of the string slice.
To get mutable string slices instead, see the
split_at_mut_checked
method.
§
Examples
let
s =
"Per Martin-Löf"
;
let
(first, last) = s.split_at_checked(
3
).unwrap();
assert_eq!
(
"Per"
, first);
assert_eq!
(
" Martin-Löf"
, last);
assert_eq!
(
None
, s.split_at_checked(
13
));
// Inside “ö”
assert_eq!
(
None
, s.split_at_checked(
16
));
// Beyond the string length
1.80.0
·
Source
pub fn
split_at_mut_checked
(
    &mut self,
    mid:
usize
,
) ->
Option
<(&mut
str
, &mut
str
)>
Divides one mutable string slice into two at an index.
The argument,
mid
, should be a valid byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point. The
method returns
None
if that’s not the case.
The two slices returned go from the start of the string slice to
mid
,
and from
mid
to the end of the string slice.
To get immutable string slices instead, see the
split_at_checked
method.
§
Examples
let
mut
s =
"Per Martin-Löf"
.to_string();
if let
Some
((first, last)) = s.split_at_mut_checked(
3
) {
    first.make_ascii_uppercase();
assert_eq!
(
"PER"
, first);
assert_eq!
(
" Martin-Löf"
, last);
}
assert_eq!
(
"PER Martin-Löf"
, s);
assert_eq!
(
None
, s.split_at_mut_checked(
13
));
// Inside “ö”
assert_eq!
(
None
, s.split_at_mut_checked(
16
));
// Beyond the string length
1.0.0
·
Source
pub fn
chars
(&self) ->
Chars
<'_>
ⓘ
Returns an iterator over the
char
s of a string slice.
As a string slice consists of valid UTF-8, we can iterate through a
string slice by
char
. This method returns such an iterator.
It’s important to remember that
char
represents a Unicode Scalar
Value, and might not match your idea of what a ‘character’ is. Iteration
over grapheme clusters may be what you actually want. This functionality
is not provided by Rust’s standard library, check crates.io instead.
§
Examples
Basic usage:
let
word =
"goodbye"
;
let
count = word.chars().count();
assert_eq!
(
7
, count);
let
mut
chars = word.chars();
assert_eq!
(
Some
(
'g'
), chars.next());
assert_eq!
(
Some
(
'o'
), chars.next());
assert_eq!
(
Some
(
'o'
), chars.next());
assert_eq!
(
Some
(
'd'
), chars.next());
assert_eq!
(
Some
(
'b'
), chars.next());
assert_eq!
(
Some
(
'y'
), chars.next());
assert_eq!
(
Some
(
'e'
), chars.next());
assert_eq!
(
None
, chars.next());
Remember,
char
s might not match your intuition about characters:
let
y =
"y̆"
;
let
mut
chars = y.chars();
assert_eq!
(
Some
(
'y'
), chars.next());
// not 'y̆'
assert_eq!
(
Some
(
'\u{0306}'
), chars.next());
assert_eq!
(
None
, chars.next());
1.0.0
·
Source
pub fn
char_indices
(&self) ->
CharIndices
<'_>
ⓘ
Returns an iterator over the
char
s of a string slice, and their
positions.
As a string slice consists of valid UTF-8, we can iterate through a
string slice by
char
. This method returns an iterator of both
these
char
s, as well as their byte positions.
The iterator yields tuples. The position is first, the
char
is
second.
§
Examples
Basic usage:
let
word =
"goodbye"
;
let
count = word.char_indices().count();
assert_eq!
(
7
, count);
let
mut
char_indices = word.char_indices();
assert_eq!
(
Some
((
0
,
'g'
)), char_indices.next());
assert_eq!
(
Some
((
1
,
'o'
)), char_indices.next());
assert_eq!
(
Some
((
2
,
'o'
)), char_indices.next());
assert_eq!
(
Some
((
3
,
'd'
)), char_indices.next());
assert_eq!
(
Some
((
4
,
'b'
)), char_indices.next());
assert_eq!
(
Some
((
5
,
'y'
)), char_indices.next());
assert_eq!
(
Some
((
6
,
'e'
)), char_indices.next());
assert_eq!
(
None
, char_indices.next());
Remember,
char
s might not match your intuition about characters:
let
yes =
"y̆es"
;
let
mut
char_indices = yes.char_indices();
assert_eq!
(
Some
((
0
,
'y'
)), char_indices.next());
// not (0, 'y̆')
assert_eq!
(
Some
((
1
,
'\u{0306}'
)), char_indices.next());
// note the 3 here - the previous character took up two bytes
assert_eq!
(
Some
((
3
,
'e'
)), char_indices.next());
assert_eq!
(
Some
((
4
,
's'
)), char_indices.next());
assert_eq!
(
None
, char_indices.next());
1.0.0
·
Source
pub fn
bytes
(&self) ->
Bytes
<'_>
ⓘ
Returns an iterator over the bytes of a string slice.
As a string slice consists of a sequence of bytes, we can iterate
through a string slice by byte. This method returns such an iterator.
§
Examples
let
mut
bytes =
"bors"
.bytes();
assert_eq!
(
Some
(
b'b'
), bytes.next());
assert_eq!
(
Some
(
b'o'
), bytes.next());
assert_eq!
(
Some
(
b'r'
), bytes.next());
assert_eq!
(
Some
(
b's'
), bytes.next());
assert_eq!
(
None
, bytes.next());
1.1.0
·
Source
pub fn
split_whitespace
(&self) ->
SplitWhitespace
<'_>
ⓘ
Splits a string slice by whitespace.
The iterator returned will return string slices that are sub-slices of
the original string slice, separated by any amount of whitespace.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
. If you only want to split on ASCII whitespace
instead, use
split_ascii_whitespace
.
§
Examples
Basic usage:
let
mut
iter =
"A few words"
.split_whitespace();
assert_eq!
(
Some
(
"A"
), iter.next());
assert_eq!
(
Some
(
"few"
), iter.next());
assert_eq!
(
Some
(
"words"
), iter.next());
assert_eq!
(
None
, iter.next());
All kinds of whitespace are considered:
let
mut
iter =
" Mary   had\ta\u{2009}little  \n\t lamb"
.split_whitespace();
assert_eq!
(
Some
(
"Mary"
), iter.next());
assert_eq!
(
Some
(
"had"
), iter.next());
assert_eq!
(
Some
(
"a"
), iter.next());
assert_eq!
(
Some
(
"little"
), iter.next());
assert_eq!
(
Some
(
"lamb"
), iter.next());
assert_eq!
(
None
, iter.next());
If the string is empty or all whitespace, the iterator yields no string slices:
assert_eq!
(
""
.split_whitespace().next(),
None
);
assert_eq!
(
"   "
.split_whitespace().next(),
None
);
1.34.0
·
Source
pub fn
split_ascii_whitespace
(&self) ->
SplitAsciiWhitespace
<'_>
ⓘ
Splits a string slice by ASCII whitespace.
The iterator returned will return string slices that are sub-slices of
the original string slice, separated by any amount of ASCII whitespace.
To split by Unicode
Whitespace
instead, use
split_whitespace
.
§
Examples
Basic usage:
let
mut
iter =
"A few words"
.split_ascii_whitespace();
assert_eq!
(
Some
(
"A"
), iter.next());
assert_eq!
(
Some
(
"few"
), iter.next());
assert_eq!
(
Some
(
"words"
), iter.next());
assert_eq!
(
None
, iter.next());
All kinds of ASCII whitespace are considered:
let
mut
iter =
" Mary   had\ta little  \n\t lamb"
.split_ascii_whitespace();
assert_eq!
(
Some
(
"Mary"
), iter.next());
assert_eq!
(
Some
(
"had"
), iter.next());
assert_eq!
(
Some
(
"a"
), iter.next());
assert_eq!
(
Some
(
"little"
), iter.next());
assert_eq!
(
Some
(
"lamb"
), iter.next());
assert_eq!
(
None
, iter.next());
If the string is empty or all ASCII whitespace, the iterator yields no string slices:
assert_eq!
(
""
.split_ascii_whitespace().next(),
None
);
assert_eq!
(
"   "
.split_ascii_whitespace().next(),
None
);
1.0.0
·
Source
pub fn
lines
(&self) ->
Lines
<'_>
ⓘ
Returns an iterator over the lines of a string, as string slices.
Lines are split at line endings that are either newlines (
\n
) or
sequences of a carriage return followed by a line feed (
\r\n
).
Line terminators are not included in the lines returned by the iterator.
Note that any carriage return (
\r
) not immediately followed by a
line feed (
\n
) does not split a line. These carriage returns are
thereby included in the produced lines.
The final line ending is optional. A string that ends with a final line
ending will return the same lines as an otherwise identical string
without a final line ending.
§
Examples
Basic usage:
let
text =
"foo\r\nbar\n\nbaz\r"
;
let
mut
lines = text.lines();
assert_eq!
(
Some
(
"foo"
), lines.next());
assert_eq!
(
Some
(
"bar"
), lines.next());
assert_eq!
(
Some
(
""
), lines.next());
// Trailing carriage return is included in the last line
assert_eq!
(
Some
(
"baz\r"
), lines.next());
assert_eq!
(
None
, lines.next());
The final line does not require any ending:
let
text =
"foo\nbar\n\r\nbaz"
;
let
mut
lines = text.lines();
assert_eq!
(
Some
(
"foo"
), lines.next());
assert_eq!
(
Some
(
"bar"
), lines.next());
assert_eq!
(
Some
(
""
), lines.next());
assert_eq!
(
Some
(
"baz"
), lines.next());
assert_eq!
(
None
, lines.next());
1.0.0
·
Source
pub fn
lines_any
(&self) ->
LinesAny
<'_>
ⓘ
👎
Deprecated since 1.4.0: use lines() instead now
Returns an iterator over the lines of a string.
1.8.0
·
Source
pub fn
encode_utf16
(&self) ->
EncodeUtf16
<'_>
ⓘ
Returns an iterator of
u16
over the string encoded
as native endian UTF-16 (without byte-order mark).
§
Examples
let
text =
"Zażółć gęślą jaźń"
;
let
utf8_len = text.len();
let
utf16_len = text.encode_utf16().count();
assert!
(utf16_len <= utf8_len);
1.0.0
·
Source
pub fn
contains
<P>(&self, pat: P) ->
bool
where
    P:
Pattern
,
Returns
true
if the given pattern matches a sub-slice of
this string slice.
Returns
false
if it does not.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
let
bananas =
"bananas"
;
assert!
(bananas.contains(
"nana"
));
assert!
(!bananas.contains(
"apples"
));
1.0.0
·
Source
pub fn
starts_with
<P>(&self, pat: P) ->
bool
where
    P:
Pattern
,
Returns
true
if the given pattern matches a prefix of this
string slice.
Returns
false
if it does not.
The
pattern
can be a
&str
, in which case this function will return true if
the
&str
is a prefix of this string slice.
The
pattern
can also be a
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
These will only be checked against the first character of this string slice.
Look at the second example below regarding behavior for slices of
char
s.
§
Examples
let
bananas =
"bananas"
;
assert!
(bananas.starts_with(
"bana"
));
assert!
(!bananas.starts_with(
"nana"
));
let
bananas =
"bananas"
;
// Note that both of these assert successfully.
assert!
(bananas.starts_with(
&
[
'b'
,
'a'
,
'n'
,
'a'
]));
assert!
(bananas.starts_with(
&
[
'a'
,
'b'
,
'c'
,
'd'
]));
1.0.0
·
Source
pub fn
ends_with
<P>(&self, pat: P) ->
bool
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns
true
if the given pattern matches a suffix of this
string slice.
Returns
false
if it does not.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
let
bananas =
"bananas"
;
assert!
(bananas.ends_with(
"anas"
));
assert!
(!bananas.ends_with(
"nana"
));
1.0.0
·
Source
pub fn
find
<P>(&self, pat: P) ->
Option
<
usize
>
where
    P:
Pattern
,
Returns the byte index of the first character of this string slice that
matches the pattern.
Returns
None
if the pattern doesn’t match.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
Simple patterns:
let
s =
"Löwe 老虎 Léopard Gepardi"
;
assert_eq!
(s.find(
'L'
),
Some
(
0
));
assert_eq!
(s.find(
'é'
),
Some
(
14
));
assert_eq!
(s.find(
"pard"
),
Some
(
17
));
More complex patterns using point-free style and closures:
let
s =
"Löwe 老虎 Léopard"
;
assert_eq!
(s.find(char::is_whitespace),
Some
(
5
));
assert_eq!
(s.find(char::is_lowercase),
Some
(
1
));
assert_eq!
(s.find(|c: char| c.is_whitespace() || c.is_lowercase()),
Some
(
1
));
assert_eq!
(s.find(|c: char| (c <
'o'
) && (c >
'a'
)),
Some
(
4
));
Not finding the pattern:
let
s =
"Löwe 老虎 Léopard"
;
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(s.find(x),
None
);
1.0.0
·
Source
pub fn
rfind
<P>(&self, pat: P) ->
Option
<
usize
>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns the byte index for the first character of the last match of the pattern in
this string slice.
Returns
None
if the pattern doesn’t match.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
Simple patterns:
let
s =
"Löwe 老虎 Léopard Gepardi"
;
assert_eq!
(s.rfind(
'L'
),
Some
(
13
));
assert_eq!
(s.rfind(
'é'
),
Some
(
14
));
assert_eq!
(s.rfind(
"pard"
),
Some
(
24
));
More complex patterns with closures:
let
s =
"Löwe 老虎 Léopard"
;
assert_eq!
(s.rfind(char::is_whitespace),
Some
(
12
));
assert_eq!
(s.rfind(char::is_lowercase),
Some
(
20
));
Not finding the pattern:
let
s =
"Löwe 老虎 Léopard"
;
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(s.rfind(x),
None
);
1.0.0
·
Source
pub fn
split
<P>(&self, pat: P) ->
Split
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over substrings of this string slice, separated by
characters matched by a pattern.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator will be a
DoubleEndedIterator
if the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,
char
, but not for
&str
.
If the pattern allows a reverse search but its results might differ
from a forward search, the
rsplit
method can be used.
§
Examples
Simple patterns:
let
v: Vec<
&
str> =
"Mary had a little lamb"
.split(
' '
).collect();
assert_eq!
(v, [
"Mary"
,
"had"
,
"a"
,
"little"
,
"lamb"
]);
let
v: Vec<
&
str> =
""
.split(
'X'
).collect();
assert_eq!
(v, [
""
]);
let
v: Vec<
&
str> =
"lionXXtigerXleopard"
.split(
'X'
).collect();
assert_eq!
(v, [
"lion"
,
""
,
"tiger"
,
"leopard"
]);
let
v: Vec<
&
str> =
"lion::tiger::leopard"
.split(
"::"
).collect();
assert_eq!
(v, [
"lion"
,
"tiger"
,
"leopard"
]);
let
v: Vec<
&
str> =
"abc1def2ghi"
.split(char::is_numeric).collect();
assert_eq!
(v, [
"abc"
,
"def"
,
"ghi"
]);
let
v: Vec<
&
str> =
"lionXtigerXleopard"
.split(char::is_uppercase).collect();
assert_eq!
(v, [
"lion"
,
"tiger"
,
"leopard"
]);
If the pattern is a slice of chars, split on each occurrence of any of the characters:
let
v: Vec<
&
str> =
"2020-11-03 23:59"
.split(
&
[
'-'
,
' '
,
':'
,
'@'
][..]).collect();
assert_eq!
(v, [
"2020"
,
"11"
,
"03"
,
"23"
,
"59"
]);
A more complex pattern, using a closure:
let
v: Vec<
&
str> =
"abc1defXghi"
.split(|c| c ==
'1'
|| c ==
'X'
).collect();
assert_eq!
(v, [
"abc"
,
"def"
,
"ghi"
]);
If a string contains multiple contiguous separators, you will end up
with empty strings in the output:
let
x =
"||||a||b|c"
.to_string();
let
d: Vec<
_
> = x.split(
'|'
).collect();
assert_eq!
(d,
&
[
""
,
""
,
""
,
""
,
"a"
,
""
,
"b"
,
"c"
]);
Contiguous separators are separated by the empty string.
let
x =
"(///)"
.to_string();
let
d: Vec<
_
> = x.split(
'/'
).collect();
assert_eq!
(d,
&
[
"("
,
""
,
""
,
")"
]);
Separators at the start or end of a string are neighbored
by empty strings.
let
d: Vec<
_
> =
"010"
.split(
"0"
).collect();
assert_eq!
(d,
&
[
""
,
"1"
,
""
]);
When the empty string is used as a separator, it separates
every character in the string, along with the beginning
and end of the string.
let
f: Vec<
_
> =
"rust"
.split(
""
).collect();
assert_eq!
(f,
&
[
""
,
"r"
,
"u"
,
"s"
,
"t"
,
""
]);
Contiguous separators can lead to possibly surprising behavior
when whitespace is used as the separator. This code is correct:
let
x =
"    a  b c"
.to_string();
let
d: Vec<
_
> = x.split(
' '
).collect();
assert_eq!
(d,
&
[
""
,
""
,
""
,
""
,
"a"
,
""
,
"b"
,
"c"
]);
It does
not
give you:
ⓘ
assert_eq!
(d,
&
[
"a"
,
"b"
,
"c"
]);
Use
split_whitespace
for this behavior.
1.51.0
·
Source
pub fn
split_inclusive
<P>(&self, pat: P) ->
SplitInclusive
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over substrings of this string slice, separated by
characters matched by a pattern.
Differs from the iterator produced by
split
in that
split_inclusive
leaves the matched part as the terminator of the substring.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
let
v: Vec<
&
str> =
"Mary had a little lamb\nlittle lamb\nlittle lamb."
.split_inclusive(
'\n'
).collect();
assert_eq!
(v, [
"Mary had a little lamb\n"
,
"little lamb\n"
,
"little lamb."
]);
If the last element of the string is matched,
that element will be considered the terminator of the preceding substring.
That substring will be the last item returned by the iterator.
let
v: Vec<
&
str> =
"Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
.split_inclusive(
'\n'
).collect();
assert_eq!
(v, [
"Mary had a little lamb\n"
,
"little lamb\n"
,
"little lamb.\n"
]);
1.0.0
·
Source
pub fn
rsplit
<P>(&self, pat: P) ->
RSplit
<'_, P>
ⓘ
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns an iterator over substrings of the given string slice, separated
by characters matched by a pattern and yielded in reverse order.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator requires that the pattern supports a reverse
search, and it will be a
DoubleEndedIterator
if a forward/reverse
search yields the same elements.
For iterating from the front, the
split
method can be used.
§
Examples
Simple patterns:
let
v: Vec<
&
str> =
"Mary had a little lamb"
.rsplit(
' '
).collect();
assert_eq!
(v, [
"lamb"
,
"little"
,
"a"
,
"had"
,
"Mary"
]);
let
v: Vec<
&
str> =
""
.rsplit(
'X'
).collect();
assert_eq!
(v, [
""
]);
let
v: Vec<
&
str> =
"lionXXtigerXleopard"
.rsplit(
'X'
).collect();
assert_eq!
(v, [
"leopard"
,
"tiger"
,
""
,
"lion"
]);
let
v: Vec<
&
str> =
"lion::tiger::leopard"
.rsplit(
"::"
).collect();
assert_eq!
(v, [
"leopard"
,
"tiger"
,
"lion"
]);
A more complex pattern, using a closure:
let
v: Vec<
&
str> =
"abc1defXghi"
.rsplit(|c| c ==
'1'
|| c ==
'X'
).collect();
assert_eq!
(v, [
"ghi"
,
"def"
,
"abc"
]);
1.0.0
·
Source
pub fn
split_terminator
<P>(&self, pat: P) ->
SplitTerminator
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over substrings of the given string slice, separated
by characters matched by a pattern.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
Equivalent to
split
, except that the trailing substring
is skipped if empty.
This method can be used for string data that is
terminated
,
rather than
separated
by a pattern.
§
Iterator behavior
The returned iterator will be a
DoubleEndedIterator
if the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,
char
, but not for
&str
.
If the pattern allows a reverse search but its results might differ
from a forward search, the
rsplit_terminator
method can be used.
§
Examples
let
v: Vec<
&
str> =
"A.B."
.split_terminator(
'.'
).collect();
assert_eq!
(v, [
"A"
,
"B"
]);
let
v: Vec<
&
str> =
"A..B.."
.split_terminator(
"."
).collect();
assert_eq!
(v, [
"A"
,
""
,
"B"
,
""
]);
let
v: Vec<
&
str> =
"A.B:C.D"
.split_terminator(
&
[
'.'
,
':'
][..]).collect();
assert_eq!
(v, [
"A"
,
"B"
,
"C"
,
"D"
]);
1.0.0
·
Source
pub fn
rsplit_terminator
<P>(&self, pat: P) ->
RSplitTerminator
<'_, P>
ⓘ
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns an iterator over substrings of
self
, separated by characters
matched by a pattern and yielded in reverse order.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
Equivalent to
split
, except that the trailing substring is
skipped if empty.
This method can be used for string data that is
terminated
,
rather than
separated
by a pattern.
§
Iterator behavior
The returned iterator requires that the pattern supports a
reverse search, and it will be double ended if a forward/reverse
search yields the same elements.
For iterating from the front, the
split_terminator
method can be
used.
§
Examples
let
v: Vec<
&
str> =
"A.B."
.rsplit_terminator(
'.'
).collect();
assert_eq!
(v, [
"B"
,
"A"
]);
let
v: Vec<
&
str> =
"A..B.."
.rsplit_terminator(
"."
).collect();
assert_eq!
(v, [
""
,
"B"
,
""
,
"A"
]);
let
v: Vec<
&
str> =
"A.B:C.D"
.rsplit_terminator(
&
[
'.'
,
':'
][..]).collect();
assert_eq!
(v, [
"D"
,
"C"
,
"B"
,
"A"
]);
1.0.0
·
Source
pub fn
splitn
<P>(&self, n:
usize
, pat: P) ->
SplitN
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over substrings of the given string slice, separated
by a pattern, restricted to returning at most
n
items.
If
n
substrings are returned, the last substring (the
n
th substring)
will contain the remainder of the string.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator will not be double ended, because it is
not efficient to support.
If the pattern allows a reverse search, the
rsplitn
method can be
used.
§
Examples
Simple patterns:
let
v: Vec<
&
str> =
"Mary had a little lambda"
.splitn(
3
,
' '
).collect();
assert_eq!
(v, [
"Mary"
,
"had"
,
"a little lambda"
]);
let
v: Vec<
&
str> =
"lionXXtigerXleopard"
.splitn(
3
,
"X"
).collect();
assert_eq!
(v, [
"lion"
,
""
,
"tigerXleopard"
]);
let
v: Vec<
&
str> =
"abcXdef"
.splitn(
1
,
'X'
).collect();
assert_eq!
(v, [
"abcXdef"
]);
let
v: Vec<
&
str> =
""
.splitn(
1
,
'X'
).collect();
assert_eq!
(v, [
""
]);
A more complex pattern, using a closure:
let
v: Vec<
&
str> =
"abc1defXghi"
.splitn(
2
, |c| c ==
'1'
|| c ==
'X'
).collect();
assert_eq!
(v, [
"abc"
,
"defXghi"
]);
1.0.0
·
Source
pub fn
rsplitn
<P>(&self, n:
usize
, pat: P) ->
RSplitN
<'_, P>
ⓘ
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns an iterator over substrings of this string slice, separated by a
pattern, starting from the end of the string, restricted to returning at
most
n
items.
If
n
substrings are returned, the last substring (the
n
th substring)
will contain the remainder of the string.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator will not be double ended, because it is not
efficient to support.
For splitting from the front, the
splitn
method can be used.
§
Examples
Simple patterns:
let
v: Vec<
&
str> =
"Mary had a little lamb"
.rsplitn(
3
,
' '
).collect();
assert_eq!
(v, [
"lamb"
,
"little"
,
"Mary had a"
]);
let
v: Vec<
&
str> =
"lionXXtigerXleopard"
.rsplitn(
3
,
'X'
).collect();
assert_eq!
(v, [
"leopard"
,
"tiger"
,
"lionX"
]);
let
v: Vec<
&
str> =
"lion::tiger::leopard"
.rsplitn(
2
,
"::"
).collect();
assert_eq!
(v, [
"leopard"
,
"lion::tiger"
]);
A more complex pattern, using a closure:
let
v: Vec<
&
str> =
"abc1defXghi"
.rsplitn(
2
, |c| c ==
'1'
|| c ==
'X'
).collect();
assert_eq!
(v, [
"ghi"
,
"abc1def"
]);
1.52.0
·
Source
pub fn
split_once
<P>(&self, delimiter: P) ->
Option
<(&
str
, &
str
)>
where
    P:
Pattern
,
Splits the string on the first occurrence of the specified delimiter and
returns prefix before delimiter and suffix after delimiter.
§
Examples
assert_eq!
(
"cfg"
.split_once(
'='
),
None
);
assert_eq!
(
"cfg="
.split_once(
'='
),
Some
((
"cfg"
,
""
)));
assert_eq!
(
"cfg=foo"
.split_once(
'='
),
Some
((
"cfg"
,
"foo"
)));
assert_eq!
(
"cfg=foo=bar"
.split_once(
'='
),
Some
((
"cfg"
,
"foo=bar"
)));
1.52.0
·
Source
pub fn
rsplit_once
<P>(&self, delimiter: P) ->
Option
<(&
str
, &
str
)>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Splits the string on the last occurrence of the specified delimiter and
returns prefix before delimiter and suffix after delimiter.
§
Examples
assert_eq!
(
"cfg"
.rsplit_once(
'='
),
None
);
assert_eq!
(
"cfg=foo"
.rsplit_once(
'='
),
Some
((
"cfg"
,
"foo"
)));
assert_eq!
(
"cfg=foo=bar"
.rsplit_once(
'='
),
Some
((
"cfg=foo"
,
"bar"
)));
1.2.0
·
Source
pub fn
matches
<P>(&self, pat: P) ->
Matches
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over the disjoint matches of a pattern within the
given string slice.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator will be a
DoubleEndedIterator
if the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,
char
, but not for
&str
.
If the pattern allows a reverse search but its results might differ
from a forward search, the
rmatches
method can be used.
§
Examples
let
v: Vec<
&
str> =
"abcXXXabcYYYabc"
.matches(
"abc"
).collect();
assert_eq!
(v, [
"abc"
,
"abc"
,
"abc"
]);
let
v: Vec<
&
str> =
"1abc2abc3"
.matches(char::is_numeric).collect();
assert_eq!
(v, [
"1"
,
"2"
,
"3"
]);
1.2.0
·
Source
pub fn
rmatches
<P>(&self, pat: P) ->
RMatches
<'_, P>
ⓘ
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns an iterator over the disjoint matches of a pattern within this
string slice, yielded in reverse order.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator requires that the pattern supports a reverse
search, and it will be a
DoubleEndedIterator
if a forward/reverse
search yields the same elements.
For iterating from the front, the
matches
method can be used.
§
Examples
let
v: Vec<
&
str> =
"abcXXXabcYYYabc"
.rmatches(
"abc"
).collect();
assert_eq!
(v, [
"abc"
,
"abc"
,
"abc"
]);
let
v: Vec<
&
str> =
"1abc2abc3"
.rmatches(char::is_numeric).collect();
assert_eq!
(v, [
"3"
,
"2"
,
"1"
]);
1.5.0
·
Source
pub fn
match_indices
<P>(&self, pat: P) ->
MatchIndices
<'_, P>
ⓘ
where
    P:
Pattern
,
Returns an iterator over the disjoint matches of a pattern within this string
slice as well as the index that the match starts at.
For matches of
pat
within
self
that overlap, only the indices
corresponding to the first match are returned.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator will be a
DoubleEndedIterator
if the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,
char
, but not for
&str
.
If the pattern allows a reverse search but its results might differ
from a forward search, the
rmatch_indices
method can be used.
§
Examples
let
v: Vec<
_
> =
"abcXXXabcYYYabc"
.match_indices(
"abc"
).collect();
assert_eq!
(v, [(
0
,
"abc"
), (
6
,
"abc"
), (
12
,
"abc"
)]);
let
v: Vec<
_
> =
"1abcabc2"
.match_indices(
"abc"
).collect();
assert_eq!
(v, [(
1
,
"abc"
), (
4
,
"abc"
)]);
let
v: Vec<
_
> =
"ababa"
.match_indices(
"aba"
).collect();
assert_eq!
(v, [(
0
,
"aba"
)]);
// only the first `aba`
1.5.0
·
Source
pub fn
rmatch_indices
<P>(&self, pat: P) ->
RMatchIndices
<'_, P>
ⓘ
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns an iterator over the disjoint matches of a pattern within
self
,
yielded in reverse order along with the index of the match.
For matches of
pat
within
self
that overlap, only the indices
corresponding to the last match are returned.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Iterator behavior
The returned iterator requires that the pattern supports a reverse
search, and it will be a
DoubleEndedIterator
if a forward/reverse
search yields the same elements.
For iterating from the front, the
match_indices
method can be used.
§
Examples
let
v: Vec<
_
> =
"abcXXXabcYYYabc"
.rmatch_indices(
"abc"
).collect();
assert_eq!
(v, [(
12
,
"abc"
), (
6
,
"abc"
), (
0
,
"abc"
)]);
let
v: Vec<
_
> =
"1abcabc2"
.rmatch_indices(
"abc"
).collect();
assert_eq!
(v, [(
4
,
"abc"
), (
1
,
"abc"
)]);
let
v: Vec<
_
> =
"ababa"
.rmatch_indices(
"aba"
).collect();
assert_eq!
(v, [(
2
,
"aba"
)]);
// only the last `aba`
1.0.0
·
Source
pub fn
trim
(&self) -> &
str
Returns a string slice with leading and trailing whitespace removed.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
, which includes newlines.
§
Examples
let
s =
"\n Hello\tworld\t\n"
;
assert_eq!
(
"Hello\tworld"
, s.trim());
1.30.0
·
Source
pub fn
trim_start
(&self) -> &
str
Returns a string slice with leading whitespace removed.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
, which includes newlines.
§
Text directionality
A string is a sequence of bytes.
start
in this context means the first
position of that byte string; for a left-to-right language like English or
Russian, this will be left side, and for right-to-left languages like
Arabic or Hebrew, this will be the right side.
§
Examples
Basic usage:
let
s =
"\n Hello\tworld\t\n"
;
assert_eq!
(
"Hello\tworld\t\n"
, s.trim_start());
Directionality:
let
s =
"  English  "
;
assert!
(
Some
(
'E'
) == s.trim_start().chars().next());
let
s =
"  עברית  "
;
assert!
(
Some
(
'ע'
) == s.trim_start().chars().next());
1.30.0
·
Source
pub fn
trim_end
(&self) -> &
str
Returns a string slice with trailing whitespace removed.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
, which includes newlines.
§
Text directionality
A string is a sequence of bytes.
end
in this context means the last
position of that byte string; for a left-to-right language like English or
Russian, this will be right side, and for right-to-left languages like
Arabic or Hebrew, this will be the left side.
§
Examples
Basic usage:
let
s =
"\n Hello\tworld\t\n"
;
assert_eq!
(
"\n Hello\tworld"
, s.trim_end());
Directionality:
let
s =
"  English  "
;
assert!
(
Some
(
'h'
) == s.trim_end().chars().rev().next());
let
s =
"  עברית  "
;
assert!
(
Some
(
'ת'
) == s.trim_end().chars().rev().next());
1.0.0
·
Source
pub fn
trim_left
(&self) -> &
str
👎
Deprecated since 1.33.0: superseded by
trim_start
Returns a string slice with leading whitespace removed.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
.
§
Text directionality
A string is a sequence of bytes. ‘Left’ in this context means the first
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
the
right
side, not the left.
§
Examples
Basic usage:
let
s =
" Hello\tworld\t"
;
assert_eq!
(
"Hello\tworld\t"
, s.trim_left());
Directionality:
let
s =
"  English"
;
assert!
(
Some
(
'E'
) == s.trim_left().chars().next());
let
s =
"  עברית"
;
assert!
(
Some
(
'ע'
) == s.trim_left().chars().next());
1.0.0
·
Source
pub fn
trim_right
(&self) -> &
str
👎
Deprecated since 1.33.0: superseded by
trim_end
Returns a string slice with trailing whitespace removed.
‘Whitespace’ is defined according to the terms of the Unicode Derived
Core Property
White_Space
.
§
Text directionality
A string is a sequence of bytes. ‘Right’ in this context means the last
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
the
left
side, not the right.
§
Examples
Basic usage:
let
s =
" Hello\tworld\t"
;
assert_eq!
(
" Hello\tworld"
, s.trim_right());
Directionality:
let
s =
"English  "
;
assert!
(
Some
(
'h'
) == s.trim_right().chars().rev().next());
let
s =
"עברית  "
;
assert!
(
Some
(
'ת'
) == s.trim_right().chars().rev().next());
1.0.0
·
Source
pub fn
trim_matches
<P>(&self, pat: P) -> &
str
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
DoubleEndedSearcher
<'a>,
Returns a string slice with all prefixes and suffixes that match a
pattern repeatedly removed.
The
pattern
can be a
char
, a slice of
char
s, or a function
or closure that determines if a character matches.
§
Examples
Simple patterns:
assert_eq!
(
"11foo1bar11"
.trim_matches(
'1'
),
"foo1bar"
);
assert_eq!
(
"123foo1bar123"
.trim_matches(char::is_numeric),
"foo1bar"
);
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(
"12foo1bar12"
.trim_matches(x),
"foo1bar"
);
A more complex pattern, using a closure:
assert_eq!
(
"1foo1barXX"
.trim_matches(|c| c ==
'1'
|| c ==
'X'
),
"foo1bar"
);
1.30.0
·
Source
pub fn
trim_start_matches
<P>(&self, pat: P) -> &
str
where
    P:
Pattern
,
Returns a string slice with all prefixes that match a pattern
repeatedly removed.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Text directionality
A string is a sequence of bytes.
start
in this context means the first
position of that byte string; for a left-to-right language like English or
Russian, this will be left side, and for right-to-left languages like
Arabic or Hebrew, this will be the right side.
§
Examples
assert_eq!
(
"11foo1bar11"
.trim_start_matches(
'1'
),
"foo1bar11"
);
assert_eq!
(
"123foo1bar123"
.trim_start_matches(char::is_numeric),
"foo1bar123"
);
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(
"12foo1bar12"
.trim_start_matches(x),
"foo1bar12"
);
1.45.0
·
Source
pub fn
strip_prefix
<P>(&self, prefix: P) ->
Option
<&
str
>
where
    P:
Pattern
,
Returns a string slice with the prefix removed.
If the string starts with the pattern
prefix
, returns the substring after the prefix,
wrapped in
Some
. Unlike
trim_start_matches
, this method removes the prefix exactly once.
If the string does not start with
prefix
, returns
None
.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
assert_eq!
(
"foo:bar"
.strip_prefix(
"foo:"
),
Some
(
"bar"
));
assert_eq!
(
"foo:bar"
.strip_prefix(
"bar"
),
None
);
assert_eq!
(
"foofoo"
.strip_prefix(
"foo"
),
Some
(
"foo"
));
1.45.0
·
Source
pub fn
strip_suffix
<P>(&self, suffix: P) ->
Option
<&
str
>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns a string slice with the suffix removed.
If the string ends with the pattern
suffix
, returns the substring before the suffix,
wrapped in
Some
.  Unlike
trim_end_matches
, this method removes the suffix exactly once.
If the string does not end with
suffix
, returns
None
.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Examples
assert_eq!
(
"bar:foo"
.strip_suffix(
":foo"
),
Some
(
"bar"
));
assert_eq!
(
"bar:foo"
.strip_suffix(
"bar"
),
None
);
assert_eq!
(
"foofoo"
.strip_suffix(
"foo"
),
Some
(
"foo"
));
1.30.0
·
Source
pub fn
trim_end_matches
<P>(&self, pat: P) -> &
str
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
Returns a string slice with all suffixes that match a pattern
repeatedly removed.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Text directionality
A string is a sequence of bytes.
end
in this context means the last
position of that byte string; for a left-to-right language like English or
Russian, this will be right side, and for right-to-left languages like
Arabic or Hebrew, this will be the left side.
§
Examples
Simple patterns:
assert_eq!
(
"11foo1bar11"
.trim_end_matches(
'1'
),
"11foo1bar"
);
assert_eq!
(
"123foo1bar123"
.trim_end_matches(char::is_numeric),
"123foo1bar"
);
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(
"12foo1bar12"
.trim_end_matches(x),
"12foo1bar"
);
A more complex pattern, using a closure:
assert_eq!
(
"1fooX"
.trim_end_matches(|c| c ==
'1'
|| c ==
'X'
),
"1foo"
);
1.0.0
·
Source
pub fn
trim_left_matches
<P>(&self, pat: P) -> &
str
where
    P:
Pattern
,
👎
Deprecated since 1.33.0: superseded by
trim_start_matches
Returns a string slice with all prefixes that match a pattern
repeatedly removed.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Text directionality
A string is a sequence of bytes. ‘Left’ in this context means the first
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
the
right
side, not the left.
§
Examples
assert_eq!
(
"11foo1bar11"
.trim_left_matches(
'1'
),
"foo1bar11"
);
assert_eq!
(
"123foo1bar123"
.trim_left_matches(char::is_numeric),
"foo1bar123"
);
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(
"12foo1bar12"
.trim_left_matches(x),
"foo1bar12"
);
1.0.0
·
Source
pub fn
trim_right_matches
<P>(&self, pat: P) -> &
str
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>: for<'a>
ReverseSearcher
<'a>,
👎
Deprecated since 1.33.0: superseded by
trim_end_matches
Returns a string slice with all suffixes that match a pattern
repeatedly removed.
The
pattern
can be a
&str
,
char
, a slice of
char
s, or a
function or closure that determines if a character matches.
§
Text directionality
A string is a sequence of bytes. ‘Right’ in this context means the last
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
the
left
side, not the right.
§
Examples
Simple patterns:
assert_eq!
(
"11foo1bar11"
.trim_right_matches(
'1'
),
"11foo1bar"
);
assert_eq!
(
"123foo1bar123"
.trim_right_matches(char::is_numeric),
"123foo1bar"
);
let
x:
&
[
_
] =
&
[
'1'
,
'2'
];
assert_eq!
(
"12foo1bar12"
.trim_right_matches(x),
"12foo1bar"
);
A more complex pattern, using a closure:
assert_eq!
(
"1fooX"
.trim_right_matches(|c| c ==
'1'
|| c ==
'X'
),
"1foo"
);
1.0.0
·
Source
pub fn
parse
<F>(&self) ->
Result
<F, <F as
FromStr
>::
Err
>
where
    F:
FromStr
,
Parses this string slice into another type.
Because
parse
is so general, it can cause problems with type
inference. As such,
parse
is one of the few times you’ll see
the syntax affectionately known as the ‘turbofish’:
::<>
. This
helps the inference algorithm understand specifically which type
you’re trying to parse into.
parse
can parse into any type that implements the
FromStr
trait.
§
Errors
Will return
Err
if it’s not possible to parse this string slice into
the desired type.
§
Examples
Basic usage:
let
four: u32 =
"4"
.parse().unwrap();
assert_eq!
(
4
, four);
Using the ‘turbofish’ instead of annotating
four
:
let
four =
"4"
.parse::<u32>();
assert_eq!
(
Ok
(
4
), four);
Failing to parse:
let
nope =
"j"
.parse::<u32>();
assert!
(nope.is_err());
1.23.0
·
Source
pub fn
is_ascii
(&self) ->
bool
Checks if all characters in this string are within the ASCII range.
§
Examples
let
ascii =
"hello!\n"
;
let
non_ascii =
"Grüße, Jürgen ❤"
;
assert!
(ascii.is_ascii());
assert!
(!non_ascii.is_ascii());
Source
pub fn
as_ascii
(&self) ->
Option
<&[
AsciiChar
]>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
If this string slice
is_ascii
, returns it as a slice
of
ASCII characters
, otherwise returns
None
.
1.23.0
·
Source
pub fn
eq_ignore_ascii_case
(&self, other: &
str
) ->
bool
Checks that two strings are an ASCII case-insensitive match.
Same as
to_ascii_lowercase(a) == to_ascii_lowercase(b)
,
but without allocating and copying temporaries.
§
Examples
assert!
(
"Ferris"
.eq_ignore_ascii_case(
"FERRIS"
));
assert!
(
"Ferrös"
.eq_ignore_ascii_case(
"FERRöS"
));
assert!
(!
"Ferrös"
.eq_ignore_ascii_case(
"FERRÖS"
));
1.23.0
·
Source
pub fn
make_ascii_uppercase
(&mut self)
Converts this string to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
to_ascii_uppercase()
.
§
Examples
let
mut
s = String::from(
"Grüße, Jürgen ❤"
);

s.make_ascii_uppercase();
assert_eq!
(
"GRüßE, JüRGEN ❤"
, s);
1.23.0
·
Source
pub fn
make_ascii_lowercase
(&mut self)
Converts this string to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
to_ascii_lowercase()
.
§
Examples
let
mut
s = String::from(
"GRÜßE, JÜRGEN ❤"
);

s.make_ascii_lowercase();
assert_eq!
(
"grÜße, jÜrgen ❤"
, s);
1.80.0
·
Source
pub fn
trim_ascii_start
(&self) -> &
str
Returns a string slice with leading ASCII whitespace removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
" \t \u{3000}hello world\n"
.trim_ascii_start(),
"\u{3000}hello world\n"
);
assert_eq!
(
"  "
.trim_ascii_start(),
""
);
assert_eq!
(
""
.trim_ascii_start(),
""
);
1.80.0
·
Source
pub fn
trim_ascii_end
(&self) -> &
str
Returns a string slice with trailing ASCII whitespace removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
"\r hello world\u{3000}\n "
.trim_ascii_end(),
"\r hello world\u{3000}"
);
assert_eq!
(
"  "
.trim_ascii_end(),
""
);
assert_eq!
(
""
.trim_ascii_end(),
""
);
1.80.0
·
Source
pub fn
trim_ascii
(&self) -> &
str
Returns a string slice with leading and trailing ASCII whitespace
removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
"\r hello world\n "
.trim_ascii(),
"hello world"
);
assert_eq!
(
"  "
.trim_ascii(),
""
);
assert_eq!
(
""
.trim_ascii(),
""
);
1.34.0
·
Source
pub fn
escape_debug
(&self) ->
EscapeDebug
<'_>
ⓘ
Returns an iterator that escapes each char in
self
with
char::escape_debug
.
Note: only extended grapheme codepoints that begin the string will be
escaped.
§
Examples
As an iterator:
for
c
in
"❤\n!"
.escape_debug() {
print!
(
"{c}"
);
}
println!
();
Using
println!
directly:
println!
(
"{}"
,
"❤\n!"
.escape_debug());
Both are equivalent to:
println!
(
"❤\\n!"
);
Using
to_string
:
assert_eq!
(
"❤\n!"
.escape_debug().to_string(),
"❤\\n!"
);
1.34.0
·
Source
pub fn
escape_default
(&self) ->
EscapeDefault
<'_>
ⓘ
Returns an iterator that escapes each char in
self
with
char::escape_default
.
§
Examples
As an iterator:
for
c
in
"❤\n!"
.escape_default() {
print!
(
"{c}"
);
}
println!
();
Using
println!
directly:
println!
(
"{}"
,
"❤\n!"
.escape_default());
Both are equivalent to:
println!
(
"\\u{{2764}}\\n!"
);
Using
to_string
:
assert_eq!
(
"❤\n!"
.escape_default().to_string(),
"\\u{2764}\\n!"
);
1.34.0
·
Source
pub fn
escape_unicode
(&self) ->
EscapeUnicode
<'_>
ⓘ
Returns an iterator that escapes each char in
self
with
char::escape_unicode
.
§
Examples
As an iterator:
for
c
in
"❤\n!"
.escape_unicode() {
print!
(
"{c}"
);
}
println!
();
Using
println!
directly:
println!
(
"{}"
,
"❤\n!"
.escape_unicode());
Both are equivalent to:
println!
(
"\\u{{2764}}\\u{{a}}\\u{{21}}"
);
Using
to_string
:
assert_eq!
(
"❤\n!"
.escape_unicode().to_string(),
"\\u{2764}\\u{a}\\u{21}"
);
Source
pub fn
substr_range
(&self, substr: &
str
) ->
Option
<
Range
<
usize
>>
🔬
This is a nightly-only experimental API. (
substr_range
#126769
)
Returns the range that a substring points to.
Returns
None
if
substr
does not point within
self
.
Unlike
str::find
,
this does not search through the string
.
Instead, it uses pointer arithmetic to find where in the string
substr
is derived from.
This is useful for extending
str::split
and similar methods.
Note that this method may return false positives (typically either
Some(0..0)
or
Some(self.len()..self.len())
) if
substr
is a
zero-length
str
that points at the beginning or end of another,
independent,
str
.
§
Examples
#![feature(substr_range)]
let
data =
"a, b, b, a"
;
let
mut
iter = data.split(
", "
).map(|s| data.substr_range(s).unwrap());
assert_eq!
(iter.next(),
Some
(
0
..
1
));
assert_eq!
(iter.next(),
Some
(
3
..
4
));
assert_eq!
(iter.next(),
Some
(
6
..
7
));
assert_eq!
(iter.next(),
Some
(
9
..
10
));
Source
pub fn
as_str
(&self) -> &
str
🔬
This is a nightly-only experimental API. (
str_as_str
#130366
)
Returns the same string as a string slice
&str
.
This method is redundant when used directly on
&str
, but
it helps dereferencing other string-like types to string slices,
for example references to
Box<str>
or
Arc<str>
.
1.0.0
·
Source
pub fn
replace
<P>(&self, from: P, to: &
str
) ->
String
where
    P:
Pattern
,
Replaces all matches of a pattern with another string.
replace
creates a new
String
, and copies the data from this string slice into it.
While doing so, it attempts to find matches of a pattern. If it finds any, it
replaces them with the replacement string slice.
§
Examples
Basic usage:
let
s =
"this is old"
;
assert_eq!
(
"this is new"
, s.replace(
"old"
,
"new"
));
assert_eq!
(
"than an old"
, s.replace(
"is"
,
"an"
));
When the pattern doesn’t match, it returns this string slice as
String
:
let
s =
"this is old"
;
assert_eq!
(s, s.replace(
"cookie monster"
,
"little lamb"
));
1.16.0
·
Source
pub fn
replacen
<P>(&self, pat: P, to: &
str
, count:
usize
) ->
String
where
    P:
Pattern
,
Replaces first N matches of a pattern with another string.
replacen
creates a new
String
, and copies the data from this string slice into it.
While doing so, it attempts to find matches of a pattern. If it finds any, it
replaces them with the replacement string slice at most
count
times.
§
Examples
Basic usage:
let
s =
"foo foo 123 foo"
;
assert_eq!
(
"new new 123 foo"
, s.replacen(
"foo"
,
"new"
,
2
));
assert_eq!
(
"faa fao 123 foo"
, s.replacen(
'o'
,
"a"
,
3
));
assert_eq!
(
"foo foo new23 foo"
, s.replacen(char::is_numeric,
"new"
,
1
));
When the pattern doesn’t match, it returns this string slice as
String
:
let
s =
"this is old"
;
assert_eq!
(s, s.replacen(
"cookie monster"
,
"little lamb"
,
10
));
1.2.0
·
Source
pub fn
to_lowercase
(&self) ->
String
Returns the lowercase equivalent of this string slice, as a new
String
.
‘Lowercase’ is defined according to the terms of the Unicode Derived Core Property
Lowercase
.
Since some characters can expand into multiple characters when changing
the case, this function returns a
String
instead of modifying the
parameter in-place.
§
Examples
Basic usage:
let
s =
"HELLO"
;
assert_eq!
(
"hello"
, s.to_lowercase());
A tricky example, with sigma:
let
sigma =
"Σ"
;
assert_eq!
(
"σ"
, sigma.to_lowercase());
// but at the end of a word, it's ς, not σ:
let
odysseus =
"ὈΔΥΣΣΕΎΣ"
;
assert_eq!
(
"ὀδυσσεύς"
, odysseus.to_lowercase());
Languages without case are not changed:
let
new_year =
"农历新年"
;
assert_eq!
(new_year, new_year.to_lowercase());
1.2.0
·
Source
pub fn
to_uppercase
(&self) ->
String
Returns the uppercase equivalent of this string slice, as a new
String
.
‘Uppercase’ is defined according to the terms of the Unicode Derived Core Property
Uppercase
.
Since some characters can expand into multiple characters when changing
the case, this function returns a
String
instead of modifying the
parameter in-place.
§
Examples
Basic usage:
let
s =
"hello"
;
assert_eq!
(
"HELLO"
, s.to_uppercase());
Scripts without case are not changed:
let
new_year =
"农历新年"
;
assert_eq!
(new_year, new_year.to_uppercase());
One character can become multiple:
let
s =
"tschüß"
;
assert_eq!
(
"TSCHÜSS"
, s.to_uppercase());
1.16.0
·
Source
pub fn
repeat
(&self, n:
usize
) ->
String
Creates a new
String
by repeating a string
n
times.
§
Panics
This function will panic if the capacity would overflow.
§
Examples
Basic usage:
assert_eq!
(
"abc"
.repeat(
4
), String::from(
"abcabcabcabc"
));
A panic upon overflow:
ⓘ
// this will panic at runtime
let
huge =
"0123456789abcdef"
.repeat(usize::MAX);
1.23.0
·
Source
pub fn
to_ascii_uppercase
(&self) ->
String
Returns a copy of this string where each character is mapped to its
ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
make_ascii_uppercase
.
To uppercase ASCII characters in addition to non-ASCII characters, use
to_uppercase
.
§
Examples
let
s =
"Grüße, Jürgen ❤"
;
assert_eq!
(
"GRüßE, JüRGEN ❤"
, s.to_ascii_uppercase());
1.23.0
·
Source
pub fn
to_ascii_lowercase
(&self) ->
String
Returns a copy of this string where each character is mapped to its
ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
make_ascii_lowercase
.
To lowercase ASCII characters in addition to non-ASCII characters, use
to_lowercase
.
§
Examples
let
s =
"Grüße, Jürgen ❤"
;
assert_eq!
(
"grüße, jürgen ❤"
, s.to_ascii_lowercase());
Trait Implementations
§
1.0.0
·
Source
§
impl
Add
<&
str
> for
String
Implements the
+
operator for concatenating two strings.
This consumes the
String
on the left-hand side and re-uses its buffer (growing it if
necessary). This is done to avoid allocating a new
String
and copying the entire contents on
every operation, which would lead to
O
(
n
^2) running time when building an
n
-byte string by
repeated concatenation.
The string on the right-hand side is only borrowed; its contents are copied into the returned
String
.
§
Examples
Concatenating two
String
s takes the first by value and borrows the second:
let
a = String::from(
"hello"
);
let
b = String::from(
" world"
);
let
c = a +
&
b;
// `a` is moved and can no longer be used here.
If you want to keep using the first
String
, you can clone it and append to the clone instead:
let
a = String::from(
"hello"
);
let
b = String::from(
" world"
);
let
c = a.clone() +
&
b;
// `a` is still valid here.
Concatenating
&str
slices can be done by converting the first to a
String
:
let
a =
"hello"
;
let
b =
" world"
;
let
c = a.to_string() + b;
Source
§
type
Output
=
String
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
str
) ->
String
Performs the
+
operation.
Read more
1.12.0
·
Source
§
impl
AddAssign
<&
str
> for
String
Implements the
+=
operator for appending to a
String
.
This has the same behavior as the
push_str
method.
Source
§
fn
add_assign
(&mut self, other: &
str
)
Performs the
+=
operation.
Read more
1.43.0
·
Source
§
impl
AsMut
<
str
> for
String
Source
§
fn
as_mut
(&mut self) -> &mut
str
Converts this type into a mutable reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<[
u8
]> for
String
Source
§
fn
as_ref
(&self) -> &[
u8
]
ⓘ
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
Path
> for
String
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
AsRef
<
str
> for
String
Source
§
fn
as_ref
(&self) -> &
str
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
Borrow
<
str
> for
String
Source
§
fn
borrow
(&self) -> &
str
Immutably borrows from an owned value.
Read more
1.36.0
·
Source
§
impl
BorrowMut
<
str
> for
String
Source
§
fn
borrow_mut
(&mut self) -> &mut
str
Mutably borrows from an owned value.
Read more
1.0.0
·
Source
§
impl
Clone
for
String
Source
§
fn
clone_from
(&mut self, source: &
String
)
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
(&self) ->
String
Returns a copy of the value.
Read more
1.0.0
·
Source
§
impl
Debug
for
String
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
1.0.0
·
Source
§
impl
Default
for
String
Source
§
fn
default
() ->
String
Creates an empty
String
.
1.0.0
·
Source
§
impl
Deref
for
String
Source
§
type
Target
=
str
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &
str
Dereferences the value.
1.3.0
·
Source
§
impl
DerefMut
for
String
Source
§
fn
deref_mut
(&mut self) -> &mut
str
Mutably dereferences the value.
1.0.0
·
Source
§
impl
Display
for
String
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
impl<'a>
Extend
<&'a
AsciiChar
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = &'a
AsciiChar
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, c: &'a
AsciiChar
)
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
1.2.0
·
Source
§
impl<'a>
Extend
<&'a
char
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = &'a
char
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, _: &'a
char
)
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
1.0.0
·
Source
§
impl<'a>
Extend
<&'a
str
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = &'a
str
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, s: &'a
str
)
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
1.45.0
·
Source
§
impl<A>
Extend
<
Box
<
str
, A>> for
String
where
    A:
Allocator
,
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
Box
<
str
, A>>,
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
Source
§
impl
Extend
<
AsciiChar
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
AsciiChar
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, c:
AsciiChar
)
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
1.19.0
·
Source
§
impl<'a>
Extend
<
Cow
<'a,
str
>> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, s:
Cow
<'a,
str
>)
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
1.4.0
·
Source
§
impl
Extend
<
String
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
String
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, s:
String
)
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
1.0.0
·
Source
§
impl
Extend
<
char
> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
char
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, c:
char
)
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
String
> for
Cow
<'a,
str
>
Source
§
fn
from
(s: &'a
String
) ->
Cow
<'a,
str
>
Converts a
String
reference into a
Borrowed
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
let
s =
"eggplant"
.to_string();
assert_eq!
(Cow::from(
&
s), Cow::Borrowed(
"eggplant"
));
1.35.0
·
Source
§
impl
From
<&
String
> for
String
Source
§
fn
from
(s: &
String
) ->
String
Converts a
&String
into a
String
.
This clones
s
and returns the clone.
1.44.0
·
Source
§
impl
From
<&mut
str
> for
String
Source
§
fn
from
(s: &mut
str
) ->
String
Converts a
&mut str
into a
String
.
The result is allocated on the heap.
1.0.0
·
Source
§
impl
From
<&
str
> for
String
Source
§
fn
from
(s: &
str
) ->
String
Converts a
&str
into a
String
.
The result is allocated on the heap.
1.18.0
·
Source
§
impl
From
<
Box
<
str
>> for
String
Source
§
fn
from
(s:
Box
<
str
>) ->
String
Converts the given boxed
str
slice to a
String
.
It is notable that the
str
slice is owned.
§
Examples
let
s1: String = String::from(
"hello world"
);
let
s2: Box<str> = s1.into_boxed_str();
let
s3: String = String::from(s2);
assert_eq!
(
"hello world"
, s3)
1.14.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
str
>> for
String
Source
§
fn
from
(s:
Cow
<'a,
str
>) ->
String
Converts a clone-on-write string to an owned
instance of
String
.
This extracts the owned string,
clones the string if it is not already owned.
§
Example
// If the string is not owned...
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
// It will allocate on the heap and copy the string.
let
owned: String = String::from(cow);
assert_eq!
(
&
owned[..],
"eggplant"
);
1.21.0
·
Source
§
impl
From
<
String
> for
Arc
<
str
>
Source
§
fn
from
(v:
String
) ->
Arc
<
str
>
Allocates a reference-counted
str
and copies
v
into it.
§
Example
let
unique: String =
"eggplant"
.to_owned();
let
shared: Arc<str> = Arc::from(unique);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.6.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(str_err:
String
) ->
Box
<dyn
Error
+ 'a>
Converts a
String
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error>::from(a_string_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
String
) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
String
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_string_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.20.0
·
Source
§
impl
From
<
String
> for
Box
<
str
>
Source
§
fn
from
(s:
String
) ->
Box
<
str
>
Converts the given
String
to a boxed
str
slice that is owned.
§
Examples
let
s1: String = String::from(
"hello world"
);
let
s2: Box<str> = Box::from(s1);
let
s3: String = String::from(s2);
assert_eq!
(
"hello world"
, s3)
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Cow
<'a,
str
>
Source
§
fn
from
(s:
String
) ->
Cow
<'a,
str
>
Converts a
String
into an
Owned
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
let
s =
"eggplant"
.to_string();
let
s2 =
"eggplant"
.to_string();
assert_eq!
(Cow::from(s), Cow::<
'static
, str>::Owned(s2));
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
1.0.0
·
Source
§
impl
From
<
String
> for
PathBuf
Source
§
fn
from
(s:
String
) ->
PathBuf
Converts a
String
into a
PathBuf
This conversion does not allocate or copy memory.
1.21.0
·
Source
§
impl
From
<
String
> for
Rc
<
str
>
Source
§
fn
from
(v:
String
) ->
Rc
<
str
>
Allocates a reference-counted string slice and copies
v
into it.
§
Example
let
original: String =
"statue"
.to_owned();
let
shared: Rc<str> = Rc::from(original);
assert_eq!
(
"statue"
,
&
shared[..]);
1.14.0
·
Source
§
impl
From
<
String
> for
Vec
<
u8
>
Source
§
fn
from
(string:
String
) ->
Vec
<
u8
>
ⓘ
Converts the given
String
to a vector
Vec
that holds values of type
u8
.
§
Examples
let
s1 = String::from(
"hello world"
);
let
v1 = Vec::from(s1);
for
b
in
v1 {
println!
(
"{b}"
);
}
1.46.0
·
Source
§
impl
From
<
char
> for
String
Source
§
fn
from
(c:
char
) ->
String
Allocates an owned
String
from a single character.
§
Example
let
c: char =
'a'
;
let
s: String = String::from(c);
assert_eq!
(
"a"
,
&
s[..]);
1.17.0
·
Source
§
impl<'a>
FromIterator
<&'a
char
> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item = &'a
char
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl<'a>
FromIterator
<&'a
str
> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item = &'a
str
>,
Creates a value from an iterator.
Read more
1.45.0
·
Source
§
impl<A>
FromIterator
<
Box
<
str
, A>> for
String
where
    A:
Allocator
,
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
Box
<
str
, A>>,
Creates a value from an iterator.
Read more
1.19.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl
FromIterator
<
String
> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
String
>,
Creates a value from an iterator.
Read more
1.12.0
·
Source
§
impl<'a>
FromIterator
<
String
> for
Cow
<'a,
str
>
Source
§
fn
from_iter
<I>(it: I) ->
Cow
<'a,
str
>
where
    I:
IntoIterator
<Item =
String
>,
Creates a value from an iterator.
Read more
1.4.0
·
Source
§
impl
FromIterator
<
String
> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
String
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
FromIterator
<
char
> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
char
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
FromStr
for
String
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
<
String
, <
String
as
FromStr
>::
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
String
Source
§
fn
hash
<H>(&self, hasher:
&mut H
)
where
    H:
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
1.0.0
·
Source
§
impl<I>
Index
<I> for
String
where
    I:
SliceIndex
<
str
>,
Source
§
type
Output
= <I as
SliceIndex
<
str
>>::
Output
The returned type after indexing.
Source
§
fn
index
(&self, index: I) -> &<I as
SliceIndex
<
str
>>::
Output
Performs the indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl<I>
IndexMut
<I> for
String
where
    I:
SliceIndex
<
str
>,
Source
§
fn
index_mut
(&mut self, index: I) -> &mut <I as
SliceIndex
<
str
>>::
Output
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
String
Source
§
fn
cmp
(&self, other: &
String
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
impl<'a, 'b>
PartialEq
<&'a
str
> for
String
Source
§
fn
eq
(&self, other: &&'a
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
Source
§
fn
ne
(&self, other: &&'a
str
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
String
Source
§
fn
eq
(&self, other: &
ByteStr
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
impl<'a>
PartialEq
<
ByteString
> for
String
Source
§
fn
eq
(&self, other: &
ByteString
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
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for
String
Source
§
fn
eq
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
String
> for &'a
str
Source
§
fn
eq
(&self, other: &
String
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
String
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
String
> for
ByteStr
Source
§
fn
eq
(&self, other: &
String
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
impl<'a>
PartialEq
<
String
> for
ByteString
Source
§
fn
eq
(&self, other: &
String
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
impl<'a, 'b>
PartialEq
<
String
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &
String
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
String
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
impl<'a, 'b>
PartialEq
<
String
> for
str
Source
§
fn
eq
(&self, other: &
String
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
String
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
impl<'a, 'b>
PartialEq
<
str
> for
String
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
Source
§
fn
ne
(&self, other: &
str
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
String
Source
§
fn
eq
(&self, other: &
String
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
String
Source
§
fn
partial_cmp
(&self, other: &
String
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
impl<'b>
Pattern
for &'b
String
A convenience impl that delegates to the impl for
&str
.
§
Examples
assert_eq!
(String::from(
"Hello world"
).find(
"world"
),
Some
(
6
));
Source
§
type
Searcher
<'a> = <&'b
str
as
Pattern
>::
Searcher
<'a>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
(self, haystack: &
str
) -> <&'b
str
as
Pattern
>::
Searcher
<'_>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
(self, haystack: &
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
(self, haystack: &
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
(self, haystack: &
str
) ->
Option
<&
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
    <&'b
String
as
Pattern
>::
Searcher
<'a>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
    <&'b
String
as
Pattern
>::
Searcher
<'a>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
1.16.0
·
Source
§
impl
ToSocketAddrs
for
String
Source
§
type
Iter
=
IntoIter
<
SocketAddr
>
Returned iterator over socket addresses which this type may correspond
to.
Source
§
fn
to_socket_addrs
(&self) ->
Result
<
IntoIter
<
SocketAddr
>>
Converts this object to an iterator of resolved
SocketAddr
s.
Read more
Source
§
impl<'a>
TryFrom
<&'a
ByteStr
> for
String
Source
§
type
Error
=
Utf8Error
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    s: &'a
ByteStr
,
) ->
Result
<
String
, <
String
as
TryFrom
<&'a
ByteStr
>>::
Error
>
Performs the conversion.
Source
§
impl
TryFrom
<
ByteString
> for
String
Source
§
type
Error
=
FromUtf8Error
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    s:
ByteString
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
ByteString
>>::
Error
>
Performs the conversion.
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
1.87.0
·
Source
§
impl
TryFrom
<
Vec
<
u8
>> for
String
Source
§
fn
try_from
(
    bytes:
Vec
<
u8
>,
) ->
Result
<
String
, <
String
as
TryFrom
<
Vec
<
u8
>>>::
Error
>
Converts the given
Vec<u8>
into a
String
if it contains valid UTF-8 data.
§
Examples
let
s1 =
b"hello world"
.to_vec();
let
v1 = String::try_from(s1).unwrap();
assert_eq!
(v1,
"hello world"
);
Source
§
type
Error
=
FromUtf8Error
The type returned in the event of a conversion error.
1.0.0
·
Source
§
impl
Write
for
String
Source
§
fn
write_str
(&mut self, s: &
str
) ->
Result
<
()
,
Error
>
Writes a string slice into this writer, returning whether the write
succeeded.
Read more
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
Source
§
impl
DerefPure
for
String
1.0.0
·
Source
§
impl
Eq
for
String
1.0.0
·
Source
§
impl
StructuralPartialEq
for
String
Auto Trait Implementations
§
§
impl
Freeze
for
String
§
impl
RefUnwindSafe
for
String
§
impl
Send
for
String
§
impl
Sync
for
String
§
impl
Unpin
for
String
§
impl
UnwindSafe
for
String
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