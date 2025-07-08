str - Rust
Primitive Type
str
Copy item path
1.0.0
Expand description
String slices.
See also the
std::str
module
.
The
str
type, also called a ‘string slice’, is the most primitive string
type. It is usually seen in its borrowed form,
&str
. It is also the type
of string literals,
&'static str
.
§
Basic Usage
String literals are string slices:
let
hello_world =
"Hello, World!"
;
Here we have declared a string slice initialized with a string literal.
String literals have a static lifetime, which means the string
hello_world
is guaranteed to be valid for the duration of the entire program.
We can explicitly specify
hello_world
’s lifetime as well:
let
hello_world:
&
'static
str =
"Hello, world!"
;
§
Representation
A
&str
is made up of two components: a pointer to some bytes, and a
length. You can look at these with the
as_ptr
and
len
methods:
use
std::slice;
use
std::str;
let
story =
"Once upon a time..."
;
let
ptr = story.as_ptr();
let
len = story.len();
// story has nineteen bytes
assert_eq!
(
19
, len);
// We can re-build a str out of ptr and len. This is all unsafe because
// we are responsible for making sure the two components are valid:
let
s =
unsafe
{
// First, we build a &[u8]...
let
slice = slice::from_raw_parts(ptr, len);
// ... and then convert that slice into a string slice
str::from_utf8(slice)
};
assert_eq!
(s,
Ok
(story));
Note: This example shows the internals of
&str
.
unsafe
should not be
used to get a string slice under normal circumstances. Use
as_str
instead.
§
Invariant
Rust libraries may assume that string slices are always valid UTF-8.
Constructing a non-UTF-8 string slice is not immediate undefined behavior, but any function
called on a string slice may assume that it is valid UTF-8, which means that a non-UTF-8 string
slice can lead to undefined behavior down the road.
Implementations
§
Source
§
impl
str
1.0.0 (const: 1.39.0)
·
Source
pub const fn
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
1.0.0 (const: 1.39.0)
·
Source
pub const fn
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
1.87.0 (const: 1.87.0)
·
Source
pub const fn
from_utf8
(v: &[
u8
]) ->
Result
<&
str
,
Utf8Error
>
Converts a slice of bytes to a string slice.
A string slice (
&str
) is made of bytes (
u8
), and a byte slice
(
&[u8]
) is made of bytes, so this function converts between
the two. Not all byte slices are valid string slices, however:
&str
requires
that it is valid UTF-8.
from_utf8()
checks to ensure that the bytes are valid
UTF-8, and then does the conversion.
If you are sure that the byte slice is valid UTF-8, and you don’t want to
incur the overhead of the validity check, there is an unsafe version of
this function,
from_utf8_unchecked
, which has the same
behavior but skips the check.
If you need a
String
instead of a
&str
, consider
String::from_utf8
.
Because you can stack-allocate a
[u8; N]
, and you can take a
&[u8]
of it, this function is one way to have a
stack-allocated string. There is an example of this in the
examples section below.
§
Errors
Returns
Err
if the slice is not UTF-8 with a description as to why the
provided slice is not UTF-8.
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
// We can use the ? (try) operator to check if the bytes are valid
let
sparkle_heart = str::from_utf8(
&
sparkle_heart)
?
;
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
(str::from_utf8(
&
sparkle_heart).is_err());
See the docs for
Utf8Error
for more details on the kinds of
errors that can be returned.
A “stack allocated string”:
// some bytes, in a stack-allocated array
let
sparkle_heart = [
240
,
159
,
146
,
150
];
// We know these bytes are valid, so just use `unwrap()`.
let
sparkle_heart:
&
str = str::from_utf8(
&
sparkle_heart).unwrap();
assert_eq!
(
"💖"
, sparkle_heart);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
from_utf8_mut
(v: &mut [
u8
]) ->
Result
<&mut
str
,
Utf8Error
>
Converts a mutable slice of bytes to a mutable string slice.
§
Examples
Basic usage:
// "Hello, Rust!" as a mutable vector
let
mut
hellorust =
vec!
[
72
,
101
,
108
,
108
,
111
,
44
,
32
,
82
,
117
,
115
,
116
,
33
];
// As we know these bytes are valid, we can use `unwrap()`
let
outstr = str::from_utf8_mut(
&mut
hellorust).unwrap();
assert_eq!
(
"Hello, Rust!"
, outstr);
Incorrect bytes:
// Some invalid bytes in a mutable vector
let
mut
invalid =
vec!
[
128
,
223
];
assert!
(str::from_utf8_mut(
&mut
invalid).is_err());
See the docs for
Utf8Error
for more details on the kinds of
errors that can be returned.
1.87.0 (const: 1.87.0)
·
Source
pub const unsafe fn
from_utf8_unchecked
(v: &[
u8
]) -> &
str
Converts a slice of bytes to a string slice without checking
that the string contains valid UTF-8.
See the safe version,
from_utf8
, for more information.
§
Safety
The bytes passed in must be valid UTF-8.
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
sparkle_heart =
unsafe
{
    str::from_utf8_unchecked(
&
sparkle_heart)
};
assert_eq!
(
"💖"
, sparkle_heart);
1.87.0 (const: 1.87.0)
·
Source
pub const unsafe fn
from_utf8_unchecked_mut
(v: &mut [
u8
]) -> &mut
str
Converts a slice of bytes to a string slice without checking
that the string contains valid UTF-8; mutable version.
See the immutable version,
from_utf8_unchecked()
for more information.
§
Examples
Basic usage:
let
mut
heart =
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
heart =
unsafe
{ str::from_utf8_unchecked_mut(
&mut
heart) };
assert_eq!
(
"💖"
, heart);
1.9.0 (const: 1.86.0)
·
Source
pub const fn
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
1.0.0 (const: 1.39.0)
·
Source
pub const fn
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
1.20.0 (const: 1.83.0)
·
Source
pub const unsafe fn
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
1.0.0 (const: 1.32.0)
·
Source
pub const fn
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
1.36.0 (const: 1.83.0)
·
Source
pub const fn
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
1.4.0 (const: 1.86.0)
·
Source
pub const fn
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
1.4.0 (const: 1.86.0)
·
Source
pub const fn
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
1.80.0 (const: 1.86.0)
·
Source
pub const fn
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
1.80.0 (const: 1.86.0)
·
Source
pub const fn
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
1.23.0 (const: 1.74.0)
·
Source
pub const fn
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
pub const fn
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
1.23.0 (const:
unstable
)
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
1.23.0 (const: 1.84.0)
·
Source
pub const fn
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
1.23.0 (const: 1.84.0)
·
Source
pub const fn
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
1.80.0 (const: 1.80.0)
·
Source
pub const fn
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
1.80.0 (const: 1.80.0)
·
Source
pub const fn
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
1.80.0 (const: 1.80.0)
·
Source
pub const fn
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
Source
§
impl
str
Methods for string slices.
1.20.0
·
Source
pub fn
into_boxed_bytes
(self:
Box
<
str
>) ->
Box
<[
u8
]>
Converts a
Box<str>
into a
Box<[u8]>
without copying or allocating.
§
Examples
let
s =
"this is a string"
;
let
boxed_str = s.to_owned().into_boxed_str();
let
boxed_bytes = boxed_str.into_boxed_bytes();
assert_eq!
(
*
boxed_bytes,
*
s.as_bytes());
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
1.4.0
·
Source
pub fn
into_string
(self:
Box
<
str
>) ->
String
Converts a
Box<str>
into a
String
without copying or allocating.
§
Examples
let
string = String::from(
"birthday gift"
);
let
boxed_str = string.clone().into_boxed_str();
assert_eq!
(boxed_str.into_string(), string);
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
1.14.0
·
Source
§
impl<'a>
Add
<&'a
str
> for
Cow
<'a,
str
>
Source
§
type
Output
=
Cow
<'a,
str
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs: &'a
str
) -> <
Cow
<'a,
str
> as
Add
<&'a
str
>>::
Output
Performs the
+
operation.
Read more
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
1.14.0
·
Source
§
impl<'a>
AddAssign
<&'a
str
> for
Cow
<'a,
str
>
Source
§
fn
add_assign
(&mut self, rhs: &'a
str
)
Performs the
+=
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
1.51.0
·
Source
§
impl
AsMut
<
str
> for
str
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
str
Source
§
fn
as_ref
(&self) -> &[
u8
]
ⓘ
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl
AsRef
<
ByteStr
> for
str
Source
§
fn
as_ref
(&self) -> &
ByteStr
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
str
Source
§
fn
as_ref
(&self) -> &
Path
Converts this type into a shared reference of the (usually inferred) input type.
1.55.0
·
Source
§
impl<'a>
AsRef
<
str
> for
Drain
<'a>
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
AsRef
<
str
> for
str
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
AsciiExt
for
str
Source
§
type
Owned
=
String
👎
Deprecated since 1.26.0: use inherent methods instead
Container type for copied ASCII characters.
Source
§
fn
is_ascii
(&self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks if the value is within the ASCII range.
Read more
Source
§
fn
to_ascii_uppercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII upper case equivalent.
Read more
Source
§
fn
to_ascii_lowercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII lower case equivalent.
Read more
Source
§
fn
eq_ignore_ascii_case
(&self, o: &Self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks that two values are an ASCII case-insensitive match.
Read more
Source
§
fn
make_ascii_uppercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII upper case equivalent in-place.
Read more
Source
§
fn
make_ascii_lowercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII lower case equivalent in-place.
Read more
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
1.3.0
·
Source
§
impl
Clone
for
Box
<
str
>
Source
§
fn
clone
(&self) ->
Box
<
str
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
str
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
impl<S>
Concat
<
str
> for
[S]
where
    S:
Borrow
<
str
>,
Note:
str
in
Concat<str>
is not meaningful here.
This type parameter of the trait only exists to enable another impl.
Source
§
type
Output
=
String
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
The resulting type after concatenation
Source
§
fn
concat
(slice: &
[S]
) ->
String
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
Implementation of
[T]::concat
1.0.0
·
Source
§
impl
Debug
for
str
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
1.28.0
·
Source
§
impl
Default
for &mut
str
Source
§
fn
default
() -> &mut
str
Creates an empty mutable str
1.0.0
·
Source
§
impl
Default
for &
str
Source
§
fn
default
() -> &
str
Creates an empty str
1.17.0
·
Source
§
impl
Default
for
Box
<
str
>
Source
§
fn
default
() ->
Box
<
str
>
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl
Display
for
str
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
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Arc
<
str
>
Source
§
fn
from
(v: &mut
str
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
mut
original = String::from(
"eggplant"
);
let
original:
&mut
str =
&mut
original;
let
shared: Arc<str> = Arc::from(original);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Box
<
str
>
Source
§
fn
from
(s: &mut
str
) ->
Box
<
str
>
Converts a
&mut str
into a
Box<str>
This conversion allocates on the heap
and performs a copy of
s
.
§
Examples
let
mut
original = String::from(
"hello"
);
let
original:
&mut
str =
&mut
original;
let
boxed: Box<str> = Box::from(original);
println!
(
"{boxed}"
);
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Rc
<
str
>
Source
§
fn
from
(v: &mut
str
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
mut
original = String::from(
"statue"
);
let
original:
&mut
str =
&mut
original;
let
shared: Rc<str> = Rc::from(original);
assert_eq!
(
"statue"
,
&
shared[..]);
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
1.21.0
·
Source
§
impl
From
<&
str
> for
Arc
<
str
>
Source
§
fn
from
(v: &
str
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
shared: Arc<str> = Arc::from(
"eggplant"
);
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
<&
str
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err: &
str
) ->
Box
<dyn
Error
+ 'a>
Converts a
str
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error>::from(a_str_error);
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
<&
str
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
(err: &
str
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
str
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
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.17.0
·
Source
§
impl
From
<&
str
> for
Box
<
str
>
Source
§
fn
from
(s: &
str
) ->
Box
<
str
>
Converts a
&str
into a
Box<str>
This conversion allocates on the heap
and performs a copy of
s
.
§
Examples
let
boxed: Box<str> = Box::from(
"hello"
);
println!
(
"{boxed}"
);
1.0.0
·
Source
§
impl<'a>
From
<&'a
str
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
str
) ->
Cow
<'a,
str
>
Converts a string slice into a
Borrowed
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
assert_eq!
(Cow::from(
"eggplant"
), Cow::Borrowed(
"eggplant"
));
1.21.0
·
Source
§
impl
From
<&
str
> for
Rc
<
str
>
Source
§
fn
from
(v: &
str
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
shared: Rc<str> = Rc::from(
"statue"
);
assert_eq!
(
"statue"
,
&
shared[..]);
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
1.0.0
·
Source
§
impl
From
<&
str
> for
Vec
<
u8
>
Source
§
fn
from
(s: &
str
) ->
Vec
<
u8
>
ⓘ
Allocates a
Vec<u8>
and fills it with a UTF-8 string.
§
Examples
assert_eq!
(Vec::from(
"123"
),
vec!
[
b'1'
,
b'2'
,
b'3'
]);
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
str
>> for
Box
<
str
>
Source
§
fn
from
(cow:
Cow
<'_,
str
>) ->
Box
<
str
>
Converts a
Cow<'_, str>
into a
Box<str>
When
cow
is the
Cow::Borrowed
variant, this
conversion allocates on the heap and copies the
underlying
str
. Otherwise, it will try to reuse the owned
String
’s allocation.
§
Examples
use
std::borrow::Cow;
let
unboxed = Cow::Borrowed(
"hello"
);
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
let
unboxed = Cow::Owned(
"hello"
.to_string());
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
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
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
char
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
<Item = &'a
char
>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
str
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
<Item = &'a
str
>,
Creates a value from an iterator.
Read more
Source
§
impl<'a>
FromIterator
<&'a
str
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item = &'a
str
>,
Creates a value from an iterator.
Read more
1.12.0
·
Source
§
impl<'a, 'b>
FromIterator
<&'b
str
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
<Item = &'b
str
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
1.80.0
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
Box
<
str
>
where
    A:
Allocator
,
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
Box
<
str
, A>>,
Creates a value from an iterator.
Read more
1.80.0
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
1.80.0
·
Source
§
impl
FromIterator
<
char
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
char
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
Hash
for
str
Source
§
fn
hash
<H>(&self, state:
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
1.0.0
·
Source
§
impl<I>
Index
<I> for
str
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
str
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
Source
§
impl<S>
Join
<&
str
> for
[S]
where
    S:
Borrow
<
str
>,
Source
§
type
Output
=
String
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
(slice: &
[S]
, sep: &
str
) ->
String
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
str
Implements ordering of strings.
Strings are ordered
lexicographically
by their byte values. This orders Unicode code
points based on their positions in the code charts. This is not necessarily the same as
“alphabetical” order, which varies by language and locale. Sorting strings according to
culturally-accepted standards requires locale-specific data that is outside the scope of
the
str
type.
Source
§
fn
cmp
(&self, other: &
str
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
Source
§
impl<'a>
PartialEq
<&
str
> for
ByteStr
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
Source
§
impl<'a>
PartialEq
<&
str
> for
ByteString
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
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
str
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &&'b
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
(&self, other: &&'b
str
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
> for &
str
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
ByteStr
> for
str
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
> for &
str
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
Source
§
impl<'a>
PartialEq
<
ByteString
> for
str
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
>> for &'b
str
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
Cow
<'a,
str
>> for
str
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
Source
§
impl<'a>
PartialEq
<
str
> for
ByteStr
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
Source
§
impl<'a>
PartialEq
<
str
> for
ByteString
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
impl<'a, 'b>
PartialEq
<
str
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
str
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
str
Implements comparison operations on strings.
Strings are compared
lexicographically
by their byte values. This compares Unicode code
points based on their positions in the code charts. This is not necessarily the same as
“alphabetical” order, which varies by language and locale. Comparing strings according to
culturally-accepted standards requires locale-specific data that is outside the scope of
the
str
type.
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
Source
§
impl<'b>
Pattern
for &'b
str
Non-allocating substring search.
Will handle the pattern
""
as returning empty matches at each character
boundary.
§
Examples
assert_eq!
(
"Hello world"
.find(
"world"
),
Some
(
6
));
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
Checks whether the pattern matches at the front of the haystack.
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
str
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
Checks whether the pattern matches at the back of the haystack.
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
str
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
type
Searcher
<'a> =
StrSearcher
<'a, 'b>
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
) ->
StrSearcher
<'_, 'b>
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
1.73.0
·
Source
§
impl
SliceIndex
<
str
> for (
Bound
<
usize
>,
Bound
<
usize
>)
Implements substring slicing for arbitrary bounds.
Returns a slice of the given string bounded by the byte indices
provided by each bound.
This operation is
O
(1).
§
Panics
Panics if
begin
or
end
(if it exists and once adjusted for
inclusion/exclusion) does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
begin > end
, or if
end > len
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(self, slice: &
str
) ->
Option
<&
str
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(self, slice: &mut
str
) ->
Option
<&mut
str
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(self, slice:
*const
str
) ->
*const
str
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(self, slice:
*mut
str
) ->
*mut
str
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &
str
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(self, slice: &mut
str
) -> &mut
str
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
Range
<
usize
>
Implements substring slicing with syntax
&self[begin .. end]
or
&mut self[begin .. end]
.
Returns a slice of the given string from the byte range
[
begin
,
end
).
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
begin
or
end
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
begin > end
, or if
end > len
.
§
Examples
let
s =
"Löwe 老虎 Léopard"
;
assert_eq!
(
&
s[
0
..
1
],
"L"
);
assert_eq!
(
&
s[
1
..
9
],
"öwe 老"
);
// these will panic:
// byte 2 lies within `ö`:
// &s[2 ..3];

// byte 8 lies within `老`
// &s[1 .. 8];

// byte 100 is outside the string
// &s[3 .. 100];
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(self, slice: &
str
) ->
Option
<&<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
Source
§
impl
SliceIndex
<
str
> for
Range
<
usize
>
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(self, slice: &
str
) ->
Option
<&<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeFrom
<
usize
>
Implements substring slicing with syntax
&self[begin ..]
or
&mut self[begin ..]
.
Returns a slice of the given string from the byte range [
begin
,
len
).
Equivalent to
&self[begin .. len]
or
&mut self[begin .. len]
.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
begin
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), or if
begin > len
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
Source
§
impl
SliceIndex
<
str
> for
RangeFrom
<
usize
>
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeFrom
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeFull
Implements substring slicing with syntax
&self[..]
or
&mut self[..]
.
Returns a slice of the whole string, i.e., returns
&self
or
&mut self
. Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
. Unlike
other indexing operations, this can never panic.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(self, slice: &
str
) ->
Option
<&<
RangeFull
as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeFull
as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.26.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeInclusive
<
usize
>
Implements substring slicing with syntax
&self[begin ..= end]
or
&mut self[begin ..= end]
.
Returns a slice of the given string from the byte range
[
begin
,
end
]. Equivalent to
&self [begin .. end + 1]
or
&mut self[begin .. end + 1]
, except if
end
has the maximum value for
usize
.
This operation is
O
(1).
§
Panics
Panics if
begin
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
end
does not point
to the ending byte offset of a character (
end + 1
is either a starting
byte offset or equal to
len
), if
begin > end
, or if
end >= len
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(
    self,
    slice: &
str
,
) -> &<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
Source
§
impl
SliceIndex
<
str
> for
RangeInclusive
<
usize
>
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(
    self,
    slice: &
str
,
) -> &<
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeTo
<
usize
>
Implements substring slicing with syntax
&self[.. end]
or
&mut self[.. end]
.
Returns a slice of the given string from the byte range [0,
end
).
Equivalent to
&self[0 .. end]
or
&mut self[0 .. end]
.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
end
does not point to the starting byte offset of a
character (as defined by
is_char_boundary
), or if
end > len
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeTo
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.26.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeToInclusive
<
usize
>
Implements substring slicing with syntax
&self[..= end]
or
&mut self[..= end]
.
Returns a slice of the given string from the byte range [0,
end
].
Equivalent to
&self [0 .. end + 1]
, except if
end
has the maximum
value for
usize
.
This operation is
O
(1).
§
Panics
Panics if
end
does not point to the ending byte offset of a character
(
end + 1
is either a starting byte offset as defined by
is_char_boundary
, or equal to
len
), or if
end >= len
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(
    self,
    slice: &
str
,
) -> &<
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeToInclusive
<
usize
> as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.0.0
·
Source
§
impl
ToOwned
for
str
Source
§
type
Owned
=
String
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) ->
String
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target: &mut
String
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
1.0.0
·
Source
§
impl
ToSocketAddrs
for
str
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
> for &'a
str
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
<&'a
str
, <&'a
str
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
impl<'a>
TryFrom
<&'a
ByteString
> for &'a
str
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
ByteString
,
) ->
Result
<&'a
str
, <&'a
str
as
TryFrom
<&'a
ByteString
>>::
Error
>
Performs the conversion.
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
Source
§
impl<'a>
TryFrom
<&'a mut
ByteStr
> for &'a mut
str
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
    s: &'a mut
ByteStr
,
) ->
Result
<&'a mut
str
, <&'a mut
str
as
TryFrom
<&'a mut
ByteStr
>>::
Error
>
Performs the conversion.
1.0.0
·
Source
§
impl
Eq
for
str
1.65.0
·
Source
§
impl !
Error
for &
str
Source
§
impl
StructuralPartialEq
for
str
Source
§
impl
UnsizedConstParamTy
for
str
Auto Trait Implementations
§
§
impl
Freeze
for
str
§
impl
RefUnwindSafe
for
str
§
impl
Send
for
str
§
impl !
Sized
for
str
§
impl
Sync
for
str
§
impl
Unpin
for
str
§
impl
UnwindSafe
for
str
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