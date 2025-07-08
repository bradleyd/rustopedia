char - Rust
Primitive Type
char
Copy item path
1.0.0
Expand description
A character type.
The
char
type represents a single character. More specifically, since
‘character’ isn’t a well-defined concept in Unicode,
char
is a ‘
Unicode
scalar value
’.
This documentation describes a number of methods and trait implementations on the
char
type. For technical reasons, there is additional, separate
documentation in
the
std::char
module
as well.
§
Validity and Layout
A
char
is a ‘
Unicode scalar value
’, which is any ‘
Unicode code point
’
other than a
surrogate code point
. This has a fixed numerical definition:
code points are in the range 0 to 0x10FFFF, inclusive.
Surrogate code points, used by UTF-16, are in the range 0xD800 to 0xDFFF.
No
char
may be constructed, whether as a literal or at runtime, that is not a
Unicode scalar value. Violating this rule causes undefined behavior.
ⓘ
// Each of these is a compiler error
[
'\u{D800}'
,
'\u{DFFF}'
,
'\u{110000}'
];
ⓘ
// Panics; from_u32 returns None.
char::from_u32(
0xDE01
).unwrap();
// Undefined behavior
let _
=
unsafe
{ char::from_u32_unchecked(
0x110000
) };
Unicode scalar values are also the exact set of values that may be encoded in UTF-8. Because
char
values are Unicode scalar values and functions may assume
incoming
str
values are
valid UTF-8
, it is safe to store any
char
in a
str
or read
any character from a
str
as a
char
.
The gap in valid
char
values is understood by the compiler, so in the
below example the two ranges are understood to cover the whole range of
possible
char
values and there is no error for a
non-exhaustive match
.
let
c: char =
'a'
;
match
c {
'\0'
..=
'\u{D7FF}'
=>
false
,
'\u{E000}'
..=
'\u{10FFFF}'
=>
true
,
};
All Unicode scalar values are valid
char
values, but not all of them represent a real
character. Many Unicode scalar values are not currently assigned to a character, but may be in
the future (“reserved”); some will never be a character (“noncharacters”); and some may be given
different meanings by different users (“private use”).
char
is guaranteed to have the same size, alignment, and function call ABI as
u32
on all
platforms.
use
std::alloc::Layout;
assert_eq!
(Layout::new::<char>(), Layout::new::<u32>());
§
Representation
char
is always four bytes in size. This is a different representation than
a given character would have as part of a
String
. For example:
let
v =
vec!
[
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
// five elements times four bytes for each element
assert_eq!
(
20
, v.len() * size_of::<char>());
let
s = String::from(
"hello"
);
// five elements times one byte per element
assert_eq!
(
5
, s.len() * size_of::<u8>());
As always, remember that a human intuition for ‘character’ might not map to
Unicode’s definitions. For example, despite looking similar, the ‘é’
character is one Unicode code point while ‘é’ is two Unicode code points:
let
mut
chars =
"é"
.chars();
// U+00e9: 'latin small letter e with acute'
assert_eq!
(
Some
(
'\u{00e9}'
), chars.next());
assert_eq!
(
None
, chars.next());
let
mut
chars =
"é"
.chars();
// U+0065: 'latin small letter e'
assert_eq!
(
Some
(
'\u{0065}'
), chars.next());
// U+0301: 'combining acute accent'
assert_eq!
(
Some
(
'\u{0301}'
), chars.next());
assert_eq!
(
None
, chars.next());
This means that the contents of the first string above
will
fit into a
char
while the contents of the second string
will not
. Trying to create
a
char
literal with the contents of the second string gives an error:
error: character literal may only contain one codepoint: 'é'
let c = 'é';
        ^^^
Another implication of the 4-byte fixed size of a
char
is that
per-
char
processing can end up using a lot more memory:
let
s = String::from(
"love: ❤️"
);
let
v: Vec<char> = s.chars().collect();
assert_eq!
(
12
, size_of_val(
&
s[..]));
assert_eq!
(
32
, size_of_val(
&
v[..]));
Implementations
§
Source
§
impl
char
1.83.0
·
Source
pub const
MIN
:
char
= '\0'
The lowest valid code point a
char
can have,
'\0'
.
Unlike integer types,
char
actually has a gap in the middle,
meaning that the range of possible
char
s is smaller than you
might expect. Ranges of
char
will automatically hop this gap
for you:
let
dist = u32::from(char::MAX) - u32::from(char::MIN);
let
size = (char::MIN..=char::MAX).count()
as
u32;
assert!
(size < dist);
Despite this gap, the
MIN
and
MAX
values can be used as bounds for
all
char
values.
§
Examples
let
c: char = something_which_returns_char();
assert!
(char::MIN <= c);
let
value_at_min = u32::from(char::MIN);
assert_eq!
(char::from_u32(value_at_min),
Some
(
'\0'
));
1.52.0
·
Source
pub const
MAX
:
char
= '\u{10ffff}'
The highest valid code point a
char
can have,
'\u{10FFFF}'
.
Unlike integer types,
char
actually has a gap in the middle,
meaning that the range of possible
char
s is smaller than you
might expect. Ranges of
char
will automatically hop this gap
for you:
let
dist = u32::from(char::MAX) - u32::from(char::MIN);
let
size = (char::MIN..=char::MAX).count()
as
u32;
assert!
(size < dist);
Despite this gap, the
MIN
and
MAX
values can be used as bounds for
all
char
values.
§
Examples
let
c: char = something_which_returns_char();
assert!
(c <= char::MAX);
let
value_at_max = u32::from(char::MAX);
assert_eq!
(char::from_u32(value_at_max),
Some
(
'\u{10FFFF}'
));
assert_eq!
(char::from_u32(value_at_max +
1
),
None
);
Source
pub const
MAX_LEN_UTF8
:
usize
= 4usize
🔬
This is a nightly-only experimental API. (
char_max_len
#121714
)
The maximum number of bytes required to
encode
a
char
to
UTF-8 encoding.
Source
pub const
MAX_LEN_UTF16
:
usize
= 2usize
🔬
This is a nightly-only experimental API. (
char_max_len
#121714
)
The maximum number of two-byte units required to
encode
a
char
to UTF-16 encoding.
1.52.0
·
Source
pub const
REPLACEMENT_CHARACTER
:
char
= '�'
U+FFFD REPLACEMENT CHARACTER
(�) is used in Unicode to represent a
decoding error.
It can occur, for example, when giving ill-formed UTF-8 bytes to
String::from_utf8_lossy
.
1.52.0
·
Source
pub const
UNICODE_VERSION
: (
u8
,
u8
,
u8
) = crate::unicode::UNICODE_VERSION
The version of
Unicode
that the Unicode parts of
char
and
str
methods are based on.
New versions of Unicode are released regularly and subsequently all methods
in the standard library depending on Unicode are updated. Therefore the
behavior of some
char
and
str
methods and the value of this constant
changes over time. This is
not
considered to be a breaking change.
The version numbering scheme is explained in
Unicode 11.0 or later, Section 3.1 Versions of the Unicode Standard
.
1.52.0
·
Source
pub fn
decode_utf16
<I>(iter: I) ->
DecodeUtf16
<<I as
IntoIterator
>::
IntoIter
>
ⓘ
where
    I:
IntoIterator
<Item =
u16
>,
Creates an iterator over the native endian UTF-16 encoded code points in
iter
,
returning unpaired surrogates as
Err
s.
§
Examples
Basic usage:
// 𝄞mus<invalid>ic<invalid>
let
v = [
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
,
];
assert_eq!
(
    char::decode_utf16(v)
        .map(|r| r.map_err(|e| e.unpaired_surrogate()))
        .collect::<Vec<
_
>>(),
vec!
[
Ok
(
'𝄞'
),
Ok
(
'm'
),
Ok
(
'u'
),
Ok
(
's'
),
Err
(
0xDD1E
),
Ok
(
'i'
),
Ok
(
'c'
),
Err
(
0xD834
)
    ]
);
A lossy decoder can be obtained by replacing
Err
results with the replacement character:
// 𝄞mus<invalid>ic<invalid>
let
v = [
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
,
];
assert_eq!
(
    char::decode_utf16(v)
       .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
       .collect::<String>(),
"𝄞mus�ic�"
);
1.52.0 (const: 1.67.0)
·
Source
pub const fn
from_u32
(i:
u32
) ->
Option
<
char
>
Converts a
u32
to a
char
.
Note that all
char
s are valid
u32
s, and can be cast to one with
as
:
let
c =
'💯'
;
let
i = c
as
u32;
assert_eq!
(
128175
, i);
However, the reverse is not true: not all valid
u32
s are valid
char
s.
from_u32()
will return
None
if the input is not a valid value
for a
char
.
For an unsafe version of this function which ignores these checks, see
from_u32_unchecked
.
§
Examples
Basic usage:
let
c = char::from_u32(
0x2764
);
assert_eq!
(
Some
(
'❤'
), c);
Returning
None
when the input is not a valid
char
:
let
c = char::from_u32(
0x110000
);
assert_eq!
(
None
, c);
1.52.0 (const: 1.81.0)
·
Source
pub const unsafe fn
from_u32_unchecked
(i:
u32
) ->
char
Converts a
u32
to a
char
, ignoring validity.
Note that all
char
s are valid
u32
s, and can be cast to one with
as
:
let
c =
'💯'
;
let
i = c
as
u32;
assert_eq!
(
128175
, i);
However, the reverse is not true: not all valid
u32
s are valid
char
s.
from_u32_unchecked()
will ignore this, and blindly cast to
char
, possibly creating an invalid one.
§
Safety
This function is unsafe, as it may construct invalid
char
values.
For a safe version of this function, see the
from_u32
function.
§
Examples
Basic usage:
let
c =
unsafe
{ char::from_u32_unchecked(
0x2764
) };
assert_eq!
(
'❤'
, c);
1.52.0 (const: 1.67.0)
·
Source
pub const fn
from_digit
(num:
u32
, radix:
u32
) ->
Option
<
char
>
Converts a digit in the given radix to a
char
.
A ‘radix’ here is sometimes also called a ‘base’. A radix of two
indicates a binary number, a radix of ten, decimal, and a radix of
sixteen, hexadecimal, to give some common values. Arbitrary
radices are supported.
from_digit()
will return
None
if the input is not a digit in
the given radix.
§
Panics
Panics if given a radix larger than 36.
§
Examples
Basic usage:
let
c = char::from_digit(
4
,
10
);
assert_eq!
(
Some
(
'4'
), c);
// Decimal 11 is a single digit in base 16
let
c = char::from_digit(
11
,
16
);
assert_eq!
(
Some
(
'b'
), c);
Returning
None
when the input is not a digit:
let
c = char::from_digit(
20
,
10
);
assert_eq!
(
None
, c);
Passing a large radix, causing a panic:
ⓘ
// this panics
let
_c = char::from_digit(
1
,
37
);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
is_digit
(self, radix:
u32
) ->
bool
Checks if a
char
is a digit in the given radix.
A ‘radix’ here is sometimes also called a ‘base’. A radix of two
indicates a binary number, a radix of ten, decimal, and a radix of
sixteen, hexadecimal, to give some common values. Arbitrary
radices are supported.
Compared to
is_numeric()
, this function only recognizes the characters
0-9
,
a-z
and
A-Z
.
‘Digit’ is defined to be only the following characters:
0-9
a-z
A-Z
For a more comprehensive understanding of ‘digit’, see
is_numeric()
.
§
Panics
Panics if given a radix smaller than 2 or larger than 36.
§
Examples
Basic usage:
assert!
(
'1'
.is_digit(
10
));
assert!
(
'f'
.is_digit(
16
));
assert!
(!
'f'
.is_digit(
10
));
Passing a large radix, causing a panic:
ⓘ
// this panics
'1'
.is_digit(
37
);
Passing a small radix, causing a panic:
ⓘ
// this panics
'1'
.is_digit(
1
);
1.0.0 (const: 1.67.0)
·
Source
pub const fn
to_digit
(self, radix:
u32
) ->
Option
<
u32
>
Converts a
char
to a digit in the given radix.
A ‘radix’ here is sometimes also called a ‘base’. A radix of two
indicates a binary number, a radix of ten, decimal, and a radix of
sixteen, hexadecimal, to give some common values. Arbitrary
radices are supported.
‘Digit’ is defined to be only the following characters:
0-9
a-z
A-Z
§
Errors
Returns
None
if the
char
does not refer to a digit in the given radix.
§
Panics
Panics if given a radix smaller than 2 or larger than 36.
§
Examples
Basic usage:
assert_eq!
(
'1'
.to_digit(
10
),
Some
(
1
));
assert_eq!
(
'f'
.to_digit(
16
),
Some
(
15
));
Passing a non-digit results in failure:
assert_eq!
(
'f'
.to_digit(
10
),
None
);
assert_eq!
(
'z'
.to_digit(
16
),
None
);
Passing a large radix, causing a panic:
ⓘ
// this panics
let _
=
'1'
.to_digit(
37
);
Passing a small radix, causing a panic:
ⓘ
// this panics
let _
=
'1'
.to_digit(
1
);
1.0.0
·
Source
pub fn
escape_unicode
(self) ->
EscapeUnicode
ⓘ
Returns an iterator that yields the hexadecimal Unicode escape of a
character as
char
s.
This will escape characters with the Rust syntax of the form
\u{NNNNNN}
where
NNNNNN
is a hexadecimal representation.
§
Examples
As an iterator:
for
c
in
'❤'
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
'❤'
.escape_unicode());
Both are equivalent to:
println!
(
"\\u{{2764}}"
);
Using
to_string
:
assert_eq!
(
'❤'
.escape_unicode().to_string(),
"\\u{2764}"
);
1.20.0
·
Source
pub fn
escape_debug
(self) ->
EscapeDebug
ⓘ
Returns an iterator that yields the literal escape code of a character
as
char
s.
This will escape the characters similar to the
Debug
implementations
of
str
or
char
.
§
Examples
As an iterator:
for
c
in
'\n'
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
'\n'
.escape_debug());
Both are equivalent to:
println!
(
"\\n"
);
Using
to_string
:
assert_eq!
(
'\n'
.escape_debug().to_string(),
"\\n"
);
1.0.0
·
Source
pub fn
escape_default
(self) ->
EscapeDefault
ⓘ
Returns an iterator that yields the literal escape code of a character
as
char
s.
The default is chosen with a bias toward producing literals that are
legal in a variety of languages, including C++11 and similar C-family
languages. The exact rules are:
Tab is escaped as
\t
.
Carriage return is escaped as
\r
.
Line feed is escaped as
\n
.
Single quote is escaped as
\'
.
Double quote is escaped as
\"
.
Backslash is escaped as
\\
.
Any character in the ‘printable ASCII’ range
0x20
..
0x7e
inclusive is not escaped.
All other characters are given hexadecimal Unicode escapes; see
escape_unicode
.
§
Examples
As an iterator:
for
c
in
'"'
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
'"'
.escape_default());
Both are equivalent to:
println!
(
"\\\""
);
Using
to_string
:
assert_eq!
(
'"'
.escape_default().to_string(),
"\\\""
);
1.0.0 (const: 1.52.0)
·
Source
pub const fn
len_utf8
(self) ->
usize
Returns the number of bytes this
char
would need if encoded in UTF-8.
That number of bytes is always between 1 and 4, inclusive.
§
Examples
Basic usage:
let
len =
'A'
.len_utf8();
assert_eq!
(len,
1
);
let
len =
'ß'
.len_utf8();
assert_eq!
(len,
2
);
let
len =
'ℝ'
.len_utf8();
assert_eq!
(len,
3
);
let
len =
'💣'
.len_utf8();
assert_eq!
(len,
4
);
The
&str
type guarantees that its contents are UTF-8, and so we can compare the length it
would take if each code point was represented as a
char
vs in the
&str
itself:
// as chars
let
eastern =
'東'
;
let
capital =
'京'
;
// both can be represented as three bytes
assert_eq!
(
3
, eastern.len_utf8());
assert_eq!
(
3
, capital.len_utf8());
// as a &str, these two are encoded in UTF-8
let
tokyo =
"東京"
;
let
len = eastern.len_utf8() + capital.len_utf8();
// we can see that they take six bytes total...
assert_eq!
(
6
, tokyo.len());
// ... just like the &str
assert_eq!
(len, tokyo.len());
1.0.0 (const: 1.52.0)
·
Source
pub const fn
len_utf16
(self) ->
usize
Returns the number of 16-bit code units this
char
would need if
encoded in UTF-16.
That number of code units is always either 1 or 2, for unicode scalar values in
the
basic multilingual plane
or
supplementary planes
respectively.
See the documentation for
len_utf8()
for more explanation of this
concept. This function is a mirror, but for UTF-16 instead of UTF-8.
§
Examples
Basic usage:
let
n =
'ß'
.len_utf16();
assert_eq!
(n,
1
);
let
len =
'💣'
.len_utf16();
assert_eq!
(len,
2
);
1.15.0 (const: 1.83.0)
·
Source
pub const fn
encode_utf8
(self, dst: &mut [
u8
]) -> &mut
str
Encodes this character as UTF-8 into the provided byte buffer,
and then returns the subslice of the buffer that contains the encoded character.
§
Panics
Panics if the buffer is not large enough.
A buffer of length four is large enough to encode any
char
.
§
Examples
In both of these examples, ‘ß’ takes two bytes to encode.
let
mut
b = [
0
;
2
];
let
result =
'ß'
.encode_utf8(
&mut
b);
assert_eq!
(result,
"ß"
);
assert_eq!
(result.len(),
2
);
A buffer that’s too small:
ⓘ
let
mut
b = [
0
;
1
];
// this panics
'ß'
.encode_utf8(
&mut
b);
1.15.0 (const: 1.84.0)
·
Source
pub const fn
encode_utf16
(self, dst: &mut [
u16
]) -> &mut [
u16
]
Encodes this character as native endian UTF-16 into the provided
u16
buffer,
and then returns the subslice of the buffer that contains the encoded character.
§
Panics
Panics if the buffer is not large enough.
A buffer of length 2 is large enough to encode any
char
.
§
Examples
In both of these examples, ‘𝕊’ takes two
u16
s to encode.
let
mut
b = [
0
;
2
];
let
result =
'𝕊'
.encode_utf16(
&mut
b);
assert_eq!
(result.len(),
2
);
A buffer that’s too small:
ⓘ
let
mut
b = [
0
;
1
];
// this panics
'𝕊'
.encode_utf16(
&mut
b);
1.0.0
·
Source
pub fn
is_alphabetic
(self) ->
bool
Returns
true
if this
char
has the
Alphabetic
property.
Alphabetic
is described in Chapter 4 (Character Properties) of the
Unicode Standard
and
specified in the
Unicode Character Database
DerivedCoreProperties.txt
.
§
Examples
Basic usage:
assert!
(
'a'
.is_alphabetic());
assert!
(
'京'
.is_alphabetic());
let
c =
'💝'
;
// love is many things, but it is not alphabetic
assert!
(!c.is_alphabetic());
1.0.0 (const: 1.84.0)
·
Source
pub const fn
is_lowercase
(self) ->
bool
Returns
true
if this
char
has the
Lowercase
property.
Lowercase
is described in Chapter 4 (Character Properties) of the
Unicode Standard
and
specified in the
Unicode Character Database
DerivedCoreProperties.txt
.
§
Examples
Basic usage:
assert!
(
'a'
.is_lowercase());
assert!
(
'δ'
.is_lowercase());
assert!
(!
'A'
.is_lowercase());
assert!
(!
'Δ'
.is_lowercase());
// The various Chinese scripts and punctuation do not have case, and so:
assert!
(!
'中'
.is_lowercase());
assert!
(!
' '
.is_lowercase());
In a const context:
const
CAPITAL_DELTA_IS_LOWERCASE: bool =
'Δ'
.is_lowercase();
assert!
(!CAPITAL_DELTA_IS_LOWERCASE);
1.0.0 (const: 1.84.0)
·
Source
pub const fn
is_uppercase
(self) ->
bool
Returns
true
if this
char
has the
Uppercase
property.
Uppercase
is described in Chapter 4 (Character Properties) of the
Unicode Standard
and
specified in the
Unicode Character Database
DerivedCoreProperties.txt
.
§
Examples
Basic usage:
assert!
(!
'a'
.is_uppercase());
assert!
(!
'δ'
.is_uppercase());
assert!
(
'A'
.is_uppercase());
assert!
(
'Δ'
.is_uppercase());
// The various Chinese scripts and punctuation do not have case, and so:
assert!
(!
'中'
.is_uppercase());
assert!
(!
' '
.is_uppercase());
In a const context:
const
CAPITAL_DELTA_IS_UPPERCASE: bool =
'Δ'
.is_uppercase();
assert!
(CAPITAL_DELTA_IS_UPPERCASE);
1.0.0 (const: 1.87.0)
·
Source
pub const fn
is_whitespace
(self) ->
bool
Returns
true
if this
char
has the
White_Space
property.
White_Space
is specified in the
Unicode Character Database
PropList.txt
.
§
Examples
Basic usage:
assert!
(
' '
.is_whitespace());
// line break
assert!
(
'\n'
.is_whitespace());
// a non-breaking space
assert!
(
'\u{A0}'
.is_whitespace());
assert!
(!
'越'
.is_whitespace());
1.0.0
·
Source
pub fn
is_alphanumeric
(self) ->
bool
Returns
true
if this
char
satisfies either
is_alphabetic()
or
is_numeric()
.
§
Examples
Basic usage:
assert!
(
'٣'
.is_alphanumeric());
assert!
(
'7'
.is_alphanumeric());
assert!
(
'৬'
.is_alphanumeric());
assert!
(
'¾'
.is_alphanumeric());
assert!
(
'①'
.is_alphanumeric());
assert!
(
'K'
.is_alphanumeric());
assert!
(
'و'
.is_alphanumeric());
assert!
(
'藏'
.is_alphanumeric());
1.0.0
·
Source
pub fn
is_control
(self) ->
bool
Returns
true
if this
char
has the general category for control codes.
Control codes (code points with the general category of
Cc
) are described in Chapter 4
(Character Properties) of the
Unicode Standard
and specified in the
Unicode Character
Database
UnicodeData.txt
.
§
Examples
Basic usage:
// U+009C, STRING TERMINATOR
assert!
(
''
.is_control());
assert!
(!
'q'
.is_control());
1.0.0
·
Source
pub fn
is_numeric
(self) ->
bool
Returns
true
if this
char
has one of the general categories for numbers.
The general categories for numbers (
Nd
for decimal digits,
Nl
for letter-like numeric
characters, and
No
for other numeric characters) are specified in the
Unicode Character
Database
UnicodeData.txt
.
This method doesn’t cover everything that could be considered a number, e.g. ideographic numbers like ‘三’.
If you want everything including characters with overlapping purposes then you might want to use
a unicode or language-processing library that exposes the appropriate character properties instead
of looking at the unicode categories.
If you want to parse ASCII decimal digits (0-9) or ASCII base-N, use
is_ascii_digit
or
is_digit
instead.
§
Examples
Basic usage:
assert!
(
'٣'
.is_numeric());
assert!
(
'7'
.is_numeric());
assert!
(
'৬'
.is_numeric());
assert!
(
'¾'
.is_numeric());
assert!
(
'①'
.is_numeric());
assert!
(!
'K'
.is_numeric());
assert!
(!
'و'
.is_numeric());
assert!
(!
'藏'
.is_numeric());
assert!
(!
'三'
.is_numeric());
1.0.0
·
Source
pub fn
to_lowercase
(self) ->
ToLowercase
ⓘ
Returns an iterator that yields the lowercase mapping of this
char
as one or more
char
s.
If this
char
does not have a lowercase mapping, the iterator yields the same
char
.
If this
char
has a one-to-one lowercase mapping given by the
Unicode Character
Database
UnicodeData.txt
, the iterator yields that
char
.
If this
char
requires special considerations (e.g. multiple
char
s) the iterator yields
the
char
(s) given by
SpecialCasing.txt
.
This operation performs an unconditional mapping without tailoring. That is, the conversion
is independent of context and language.
In the
Unicode Standard
, Chapter 4 (Character Properties) discusses case mapping in
general and Chapter 3 (Conformance) discusses the default algorithm for case conversion.
§
Examples
As an iterator:
for
c
in
'İ'
.to_lowercase() {
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
'İ'
.to_lowercase());
Both are equivalent to:
println!
(
"i\u{307}"
);
Using
to_string
:
assert_eq!
(
'C'
.to_lowercase().to_string(),
"c"
);
// Sometimes the result is more than one character:
assert_eq!
(
'İ'
.to_lowercase().to_string(),
"i\u{307}"
);
// Characters that do not have both uppercase and lowercase
// convert into themselves.
assert_eq!
(
'山'
.to_lowercase().to_string(),
"山"
);
1.0.0
·
Source
pub fn
to_uppercase
(self) ->
ToUppercase
ⓘ
Returns an iterator that yields the uppercase mapping of this
char
as one or more
char
s.
If this
char
does not have an uppercase mapping, the iterator yields the same
char
.
If this
char
has a one-to-one uppercase mapping given by the
Unicode Character
Database
UnicodeData.txt
, the iterator yields that
char
.
If this
char
requires special considerations (e.g. multiple
char
s) the iterator yields
the
char
(s) given by
SpecialCasing.txt
.
This operation performs an unconditional mapping without tailoring. That is, the conversion
is independent of context and language.
In the
Unicode Standard
, Chapter 4 (Character Properties) discusses case mapping in
general and Chapter 3 (Conformance) discusses the default algorithm for case conversion.
§
Examples
As an iterator:
for
c
in
'ß'
.to_uppercase() {
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
'ß'
.to_uppercase());
Both are equivalent to:
println!
(
"SS"
);
Using
to_string
:
assert_eq!
(
'c'
.to_uppercase().to_string(),
"C"
);
// Sometimes the result is more than one character:
assert_eq!
(
'ß'
.to_uppercase().to_string(),
"SS"
);
// Characters that do not have both uppercase and lowercase
// convert into themselves.
assert_eq!
(
'山'
.to_uppercase().to_string(),
"山"
);
§
Note on locale
In Turkish, the equivalent of ‘i’ in Latin has five forms instead of two:
‘Dotless’: I / ı, sometimes written ï
‘Dotted’: İ / i
Note that the lowercase dotted ‘i’ is the same as the Latin. Therefore:
let
upper_i =
'i'
.to_uppercase().to_string();
The value of
upper_i
here relies on the language of the text: if we’re
in
en-US
, it should be
"I"
, but if we’re in
tr_TR
, it should
be
"İ"
.
to_uppercase()
does not take this into account, and so:
let
upper_i =
'i'
.to_uppercase().to_string();
assert_eq!
(upper_i,
"I"
);
holds across languages.
1.23.0 (const: 1.32.0)
·
Source
pub const fn
is_ascii
(&self) ->
bool
Checks if the value is within the ASCII range.
§
Examples
let
ascii =
'a'
;
let
non_ascii =
'❤'
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
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Returns
Some
if the value is within the ASCII range,
or
None
if it’s not.
This is preferred to
Self::is_ascii
when you’re passing the value
along to something else that can take
ascii::Char
rather than
needing to check again for itself whether the value is in ASCII.
1.23.0 (const: 1.52.0)
·
Source
pub const fn
to_ascii_uppercase
(&self) ->
char
Makes a copy of the value in its ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
make_ascii_uppercase()
.
To uppercase ASCII characters in addition to non-ASCII characters, use
to_uppercase()
.
§
Examples
let
ascii =
'a'
;
let
non_ascii =
'❤'
;
assert_eq!
(
'A'
, ascii.to_ascii_uppercase());
assert_eq!
(
'❤'
, non_ascii.to_ascii_uppercase());
1.23.0 (const: 1.52.0)
·
Source
pub const fn
to_ascii_lowercase
(&self) ->
char
Makes a copy of the value in its ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
make_ascii_lowercase()
.
To lowercase ASCII characters in addition to non-ASCII characters, use
to_lowercase()
.
§
Examples
let
ascii =
'A'
;
let
non_ascii =
'❤'
;
assert_eq!
(
'a'
, ascii.to_ascii_lowercase());
assert_eq!
(
'❤'
, non_ascii.to_ascii_lowercase());
1.23.0 (const: 1.52.0)
·
Source
pub const fn
eq_ignore_ascii_case
(&self, other: &
char
) ->
bool
Checks that two values are an ASCII case-insensitive match.
Equivalent to
to_ascii_lowercase
(a) ==
to_ascii_lowercase
(b)
.
§
Examples
let
upper_a =
'A'
;
let
lower_a =
'a'
;
let
lower_z =
'z'
;
assert!
(upper_a.eq_ignore_ascii_case(
&
lower_a));
assert!
(upper_a.eq_ignore_ascii_case(
&
upper_a));
assert!
(!upper_a.eq_ignore_ascii_case(
&
lower_z));
1.23.0 (const: 1.84.0)
·
Source
pub const fn
make_ascii_uppercase
(&mut self)
Converts this type to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
to_ascii_uppercase()
.
§
Examples
let
mut
ascii =
'a'
;

ascii.make_ascii_uppercase();
assert_eq!
(
'A'
, ascii);
1.23.0 (const: 1.84.0)
·
Source
pub const fn
make_ascii_lowercase
(&mut self)
Converts this type to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
to_ascii_lowercase()
.
§
Examples
let
mut
ascii =
'A'
;

ascii.make_ascii_lowercase();
assert_eq!
(
'a'
, ascii);
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_alphabetic
(&self) ->
bool
Checks if the value is an ASCII alphabetic character:
U+0041 ‘A’ ..= U+005A ‘Z’, or
U+0061 ‘a’ ..= U+007A ‘z’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(uppercase_a.is_ascii_alphabetic());
assert!
(uppercase_g.is_ascii_alphabetic());
assert!
(a.is_ascii_alphabetic());
assert!
(g.is_ascii_alphabetic());
assert!
(!zero.is_ascii_alphabetic());
assert!
(!percent.is_ascii_alphabetic());
assert!
(!space.is_ascii_alphabetic());
assert!
(!lf.is_ascii_alphabetic());
assert!
(!esc.is_ascii_alphabetic());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_uppercase
(&self) ->
bool
Checks if the value is an ASCII uppercase character:
U+0041 ‘A’ ..= U+005A ‘Z’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(uppercase_a.is_ascii_uppercase());
assert!
(uppercase_g.is_ascii_uppercase());
assert!
(!a.is_ascii_uppercase());
assert!
(!g.is_ascii_uppercase());
assert!
(!zero.is_ascii_uppercase());
assert!
(!percent.is_ascii_uppercase());
assert!
(!space.is_ascii_uppercase());
assert!
(!lf.is_ascii_uppercase());
assert!
(!esc.is_ascii_uppercase());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_lowercase
(&self) ->
bool
Checks if the value is an ASCII lowercase character:
U+0061 ‘a’ ..= U+007A ‘z’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(!uppercase_a.is_ascii_lowercase());
assert!
(!uppercase_g.is_ascii_lowercase());
assert!
(a.is_ascii_lowercase());
assert!
(g.is_ascii_lowercase());
assert!
(!zero.is_ascii_lowercase());
assert!
(!percent.is_ascii_lowercase());
assert!
(!space.is_ascii_lowercase());
assert!
(!lf.is_ascii_lowercase());
assert!
(!esc.is_ascii_lowercase());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_alphanumeric
(&self) ->
bool
Checks if the value is an ASCII alphanumeric character:
U+0041 ‘A’ ..= U+005A ‘Z’, or
U+0061 ‘a’ ..= U+007A ‘z’, or
U+0030 ‘0’ ..= U+0039 ‘9’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(uppercase_a.is_ascii_alphanumeric());
assert!
(uppercase_g.is_ascii_alphanumeric());
assert!
(a.is_ascii_alphanumeric());
assert!
(g.is_ascii_alphanumeric());
assert!
(zero.is_ascii_alphanumeric());
assert!
(!percent.is_ascii_alphanumeric());
assert!
(!space.is_ascii_alphanumeric());
assert!
(!lf.is_ascii_alphanumeric());
assert!
(!esc.is_ascii_alphanumeric());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_digit
(&self) ->
bool
Checks if the value is an ASCII decimal digit:
U+0030 ‘0’ ..= U+0039 ‘9’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(!uppercase_a.is_ascii_digit());
assert!
(!uppercase_g.is_ascii_digit());
assert!
(!a.is_ascii_digit());
assert!
(!g.is_ascii_digit());
assert!
(zero.is_ascii_digit());
assert!
(!percent.is_ascii_digit());
assert!
(!space.is_ascii_digit());
assert!
(!lf.is_ascii_digit());
assert!
(!esc.is_ascii_digit());
Source
pub const fn
is_ascii_octdigit
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
is_ascii_octdigit
#101288
)
Checks if the value is an ASCII octal digit:
U+0030 ‘0’ ..= U+0037 ‘7’.
§
Examples
#![feature(is_ascii_octdigit)]
let
uppercase_a =
'A'
;
let
a =
'a'
;
let
zero =
'0'
;
let
seven =
'7'
;
let
nine =
'9'
;
let
percent =
'%'
;
let
lf =
'\n'
;
assert!
(!uppercase_a.is_ascii_octdigit());
assert!
(!a.is_ascii_octdigit());
assert!
(zero.is_ascii_octdigit());
assert!
(seven.is_ascii_octdigit());
assert!
(!nine.is_ascii_octdigit());
assert!
(!percent.is_ascii_octdigit());
assert!
(!lf.is_ascii_octdigit());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_hexdigit
(&self) ->
bool
Checks if the value is an ASCII hexadecimal digit:
U+0030 ‘0’ ..= U+0039 ‘9’, or
U+0041 ‘A’ ..= U+0046 ‘F’, or
U+0061 ‘a’ ..= U+0066 ‘f’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(uppercase_a.is_ascii_hexdigit());
assert!
(!uppercase_g.is_ascii_hexdigit());
assert!
(a.is_ascii_hexdigit());
assert!
(!g.is_ascii_hexdigit());
assert!
(zero.is_ascii_hexdigit());
assert!
(!percent.is_ascii_hexdigit());
assert!
(!space.is_ascii_hexdigit());
assert!
(!lf.is_ascii_hexdigit());
assert!
(!esc.is_ascii_hexdigit());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_punctuation
(&self) ->
bool
Checks if the value is an ASCII punctuation character:
U+0021 ..= U+002F
! " # $ % & ' ( ) * + , - . /
, or
U+003A ..= U+0040
: ; < = > ? @
, or
U+005B ..= U+0060
[ \ ] ^ _ `
, or
U+007B ..= U+007E
{ | } ~
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(!uppercase_a.is_ascii_punctuation());
assert!
(!uppercase_g.is_ascii_punctuation());
assert!
(!a.is_ascii_punctuation());
assert!
(!g.is_ascii_punctuation());
assert!
(!zero.is_ascii_punctuation());
assert!
(percent.is_ascii_punctuation());
assert!
(!space.is_ascii_punctuation());
assert!
(!lf.is_ascii_punctuation());
assert!
(!esc.is_ascii_punctuation());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_graphic
(&self) ->
bool
Checks if the value is an ASCII graphic character:
U+0021 ‘!’ ..= U+007E ‘~’.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(uppercase_a.is_ascii_graphic());
assert!
(uppercase_g.is_ascii_graphic());
assert!
(a.is_ascii_graphic());
assert!
(g.is_ascii_graphic());
assert!
(zero.is_ascii_graphic());
assert!
(percent.is_ascii_graphic());
assert!
(!space.is_ascii_graphic());
assert!
(!lf.is_ascii_graphic());
assert!
(!esc.is_ascii_graphic());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_whitespace
(&self) ->
bool
Checks if the value is an ASCII whitespace character:
U+0020 SPACE, U+0009 HORIZONTAL TAB, U+000A LINE FEED,
U+000C FORM FEED, or U+000D CARRIAGE RETURN.
Rust uses the WhatWG Infra Standard’s
definition of ASCII
whitespace
. There are several other definitions in
wide use. For instance,
the POSIX locale
includes
U+000B VERTICAL TAB as well as all the above characters,
but—from the very same specification—
the default rule for
“field splitting” in the Bourne shell
considers
only
SPACE, HORIZONTAL TAB, and LINE FEED as whitespace.
If you are writing a program that will process an existing
file format, check what that format’s definition of whitespace is
before using this function.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(!uppercase_a.is_ascii_whitespace());
assert!
(!uppercase_g.is_ascii_whitespace());
assert!
(!a.is_ascii_whitespace());
assert!
(!g.is_ascii_whitespace());
assert!
(!zero.is_ascii_whitespace());
assert!
(!percent.is_ascii_whitespace());
assert!
(space.is_ascii_whitespace());
assert!
(lf.is_ascii_whitespace());
assert!
(!esc.is_ascii_whitespace());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_control
(&self) ->
bool
Checks if the value is an ASCII control character:
U+0000 NUL ..= U+001F UNIT SEPARATOR, or U+007F DELETE.
Note that most ASCII whitespace characters are control
characters, but SPACE is not.
§
Examples
let
uppercase_a =
'A'
;
let
uppercase_g =
'G'
;
let
a =
'a'
;
let
g =
'g'
;
let
zero =
'0'
;
let
percent =
'%'
;
let
space =
' '
;
let
lf =
'\n'
;
let
esc =
'\x1b'
;
assert!
(!uppercase_a.is_ascii_control());
assert!
(!uppercase_g.is_ascii_control());
assert!
(!a.is_ascii_control());
assert!
(!g.is_ascii_control());
assert!
(!zero.is_ascii_control());
assert!
(!percent.is_ascii_control());
assert!
(!space.is_ascii_control());
assert!
(lf.is_ascii_control());
assert!
(esc.is_ascii_control());
Trait Implementations
§
1.0.0
·
Source
§
impl
AsciiExt
for
char
Source
§
type
Owned
=
char
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
Clone
for
char
Source
§
fn
clone
(&self) ->
char
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
char
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
char
Source
§
fn
default
() ->
char
Returns the default value of
\x00
1.0.0
·
Source
§
impl
Display
for
char
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
Source
§
impl
From
<
AsciiChar
> for
char
Source
§
fn
from
(chr:
AsciiChar
) ->
char
Converts to this type from the input type.
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
1.51.0
·
Source
§
impl
From
<
char
> for
u128
Source
§
fn
from
(c:
char
) ->
u128
Converts a
char
into a
u128
.
§
Examples
let
c =
'⚙'
;
let
u = u128::from(c);
assert!
(
16
== size_of_val(
&
u))
1.13.0
·
Source
§
impl
From
<
char
> for
u32
Source
§
fn
from
(c:
char
) ->
u32
Converts a
char
into a
u32
.
§
Examples
let
c =
'c'
;
let
u = u32::from(c);
assert!
(
4
== size_of_val(
&
u))
1.51.0
·
Source
§
impl
From
<
char
> for
u64
Source
§
fn
from
(c:
char
) ->
u64
Converts a
char
into a
u64
.
§
Examples
let
c =
'👤'
;
let
u = u64::from(c);
assert!
(
8
== size_of_val(
&
u))
1.13.0
·
Source
§
impl
From
<
u8
> for
char
Maps a byte in 0x00..=0xFF to a
char
whose code point has the same value, in U+0000..=U+00FF.
Unicode is designed such that this effectively decodes bytes
with the character encoding that IANA calls ISO-8859-1.
This encoding is compatible with ASCII.
Note that this is different from ISO/IEC 8859-1 a.k.a. ISO 8859-1 (with one less hyphen),
which leaves some “blanks”, byte values that are not assigned to any character.
ISO-8859-1 (the IANA one) assigns them to the C0 and C1 control codes.
Note that this is
also
different from Windows-1252 a.k.a. code page 1252,
which is a superset ISO/IEC 8859-1 that assigns some (not all!) blanks
to punctuation and various Latin characters.
To confuse things further,
on the Web
ascii
,
iso-8859-1
, and
windows-1252
are all aliases
for a superset of Windows-1252 that fills the remaining blanks with corresponding
C0 and C1 control codes.
Source
§
fn
from
(i:
u8
) ->
char
Converts a
u8
into a
char
.
§
Examples
let
u =
32
as
u8;
let
c = char::from(u);
assert!
(
4
== size_of_val(
&
c))
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
Source
§
impl
FromIterator
<
char
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
<Item =
char
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
char
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
char
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
1.20.0
·
Source
§
impl
FromStr
for
char
Source
§
type
Err
=
ParseCharError
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
char
, <
char
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
char
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
Ord
for
char
Source
§
fn
cmp
(&self, other: &
char
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
PartialEq
for
char
Source
§
fn
eq
(&self, other: &
char
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
char
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
char
Source
§
fn
partial_cmp
(&self, other: &
char
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
char
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
char
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
char
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
char
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
Pattern
for
char
Searches for chars that are equal to a given
char
.
§
Examples
assert_eq!
(
"Hello world"
.find(
'o'
),
Some
(
4
));
Source
§
type
Searcher
<'a> =
CharSearcher
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
<'a>(self, haystack: &'a
str
) -> <
char
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
    <
char
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
    <
char
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
Source
§
impl
RangePattern
for
char
Source
§
const
MIN
:
char
= '\0'
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
Trait version of the inherent
MIN
assoc const.
Source
§
const
MAX
:
char
= '\u{10ffff}'
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
Trait version of the inherent
MIN
assoc const.
Source
§
const fn
sub_one
(self) ->
char
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
A compile-time helper to subtract 1 for exclusive ranges.
Source
§
impl
Step
for
char
Source
§
fn
steps_between
(_: &
char
, _: &
char
) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the bounds on the number of
successor
steps required to get from
start
to
end
like
Iterator::size_hint()
.
Read more
Source
§
fn
forward_checked
(start:
char
, count:
usize
) ->
Option
<
char
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward_checked
(start:
char
, count:
usize
) ->
Option
<
char
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
unsafe fn
forward_unchecked
(start:
char
, count:
usize
) ->
char
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
unsafe fn
backward_unchecked
(start:
char
, count:
usize
) ->
char
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
fn
forward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
1.74.0
·
Source
§
impl
TryFrom
<
char
> for
u16
Maps a
char
with code point in U+0000..=U+FFFF to a
u16
in 0x0000..=0xFFFF with same value,
failing if the code point is greater than U+FFFF.
This corresponds to the UCS-2 encoding, as specified in ISO/IEC 10646:2003.
Source
§
fn
try_from
(c:
char
) ->
Result
<
u16
, <
u16
as
TryFrom
<
char
>>::
Error
>
Tries to convert a
char
into a
u16
.
§
Examples
let
trans_rights =
'⚧'
;
// U+26A7
let
ninjas =
'🥷'
;
// U+1F977
assert_eq!
(u16::try_from(trans_rights),
Ok
(
0x26A7_u16
));
assert!
(u16::try_from(ninjas).is_err());
Source
§
type
Error
=
TryFromCharError
The type returned in the event of a conversion error.
1.59.0
·
Source
§
impl
TryFrom
<
char
> for
u8
Maps a
char
with code point in U+0000..=U+00FF to a byte in 0x00..=0xFF with same value,
failing if the code point is greater than U+00FF.
See
impl From<u8> for char
for details on the encoding.
Source
§
fn
try_from
(c:
char
) ->
Result
<
u8
, <
u8
as
TryFrom
<
char
>>::
Error
>
Tries to convert a
char
into a
u8
.
§
Examples
let
a =
'ÿ'
;
// U+00FF
let
b =
'Ā'
;
// U+0100
assert_eq!
(u8::try_from(a),
Ok
(
0xFF_u8
));
assert!
(u8::try_from(b).is_err());
Source
§
type
Error
=
TryFromCharError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
u32
> for
char
Source
§
type
Error
=
CharTryFromError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(i:
u32
) ->
Result
<
char
, <
char
as
TryFrom
<
u32
>>::
Error
>
Performs the conversion.
Source
§
impl
ConstParamTy_
for
char
1.0.0
·
Source
§
impl
Copy
for
char
1.0.0
·
Source
§
impl
Eq
for
char
Source
§
impl
StructuralPartialEq
for
char
Source
§
impl
TrustedStep
for
char
Source
§
impl
UnsizedConstParamTy
for
char
Source
§
impl
UseCloned
for
char
Auto Trait Implementations
§
§
impl
Freeze
for
char
§
impl
RefUnwindSafe
for
char
§
impl
Send
for
char
§
impl
Sync
for
char
§
impl
Unpin
for
char
§
impl
UnwindSafe
for
char
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