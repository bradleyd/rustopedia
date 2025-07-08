AsciiExt in std::ascii - Rust
std
::
ascii
Trait
AsciiExt
Copy item path
1.0.0
·
Source
pub trait AsciiExt {
    type
Owned
;

    // Required methods
    fn
is_ascii
(&self) ->
bool
;
fn
to_ascii_uppercase
(&self) -> Self::
Owned
;
fn
to_ascii_lowercase
(&self) -> Self::
Owned
;
fn
eq_ignore_ascii_case
(&self, other: &Self) ->
bool
;
fn
make_ascii_uppercase
(&mut self);
fn
make_ascii_lowercase
(&mut self);
}
👎
Deprecated since 1.26.0: use inherent methods instead
Expand description
Extension methods for ASCII-subset only operations.
Be aware that operations on seemingly non-ASCII characters can sometimes
have unexpected results. Consider this example:
use
std::ascii::AsciiExt;
assert_eq!
(AsciiExt::to_ascii_uppercase(
"café"
),
"CAFÉ"
);
assert_eq!
(AsciiExt::to_ascii_uppercase(
"café"
),
"CAFé"
);
In the first example, the lowercased string is represented
"cafe\u{301}"
(the last character is an acute accent
combining character
). Unlike the
other characters in the string, the combining character will not get mapped
to an uppercase variant, resulting in
"CAFE\u{301}"
. In the second
example, the lowercased string is represented
"caf\u{e9}"
(the last
character is a single Unicode character representing an ‘e’ with an acute
accent). Since the last character is defined outside the scope of ASCII,
it will not get mapped to an uppercase variant, resulting in
"CAF\u{e9}"
.
Required Associated Types
§
1.0.0
·
Source
type
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Container type for copied ASCII characters.
Required Methods
§
1.0.0
·
Source
fn
is_ascii
(&self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks if the value is within the ASCII range.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
1.0.0
·
Source
fn
to_ascii_uppercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
make_ascii_uppercase
.
To uppercase ASCII characters in addition to non-ASCII characters, use
str::to_uppercase
.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
1.0.0
·
Source
fn
to_ascii_lowercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
make_ascii_lowercase
.
To lowercase ASCII characters in addition to non-ASCII characters, use
str::to_lowercase
.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
1.0.0
·
Source
fn
eq_ignore_ascii_case
(&self, other: &Self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks that two values are an ASCII case-insensitive match.
Same as
to_ascii_lowercase(a) == to_ascii_lowercase(b)
,
but without allocating and copying temporaries.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
1.9.0
·
Source
fn
make_ascii_uppercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
to_ascii_uppercase
.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
1.9.0
·
Source
fn
make_ascii_lowercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
to_ascii_lowercase
.
§
Note
This method is deprecated in favor of the identically-named
inherent methods on
u8
,
char
,
[u8]
and
str
.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
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
1.0.0
·
Source
§
impl
AsciiExt
for
u8
Source
§
type
Owned
=
u8
1.0.0
·
Source
§
impl
AsciiExt
for [
u8
]
Source
§
type
Owned
=
Vec
<
u8
>