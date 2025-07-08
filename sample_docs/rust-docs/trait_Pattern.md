Pattern in std::str::pattern - Rust
std
::
str
::
pattern
Trait
Pattern
Copy item path
Source
pub trait Pattern:
Sized
{
    type
Searcher
<'a>:
Searcher
<'a>;

    // Required method
    fn
into_searcher
(self, haystack: &
str
) -> Self::
Searcher
<'_>;

    // Provided methods
    fn
is_contained_in
(self, haystack: &
str
) ->
bool
{ ... }
fn
is_prefix_of
(self, haystack: &
str
) ->
bool
{ ... }
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where Self::
Searcher
<'a>:
ReverseSearcher
<'a>
{ ... }
fn
strip_prefix_of
(self, haystack: &
str
) ->
Option
<&
str
> { ... }
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where Self::
Searcher
<'a>:
ReverseSearcher
<'a>
{ ... }
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>> { ... }
}
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expand description
A string pattern.
A
Pattern
expresses that the implementing type
can be used as a string pattern for searching in a
&str
.
For example, both
'a'
and
"aa"
are patterns that
would match at index
1
in the string
"baaaab"
.
The trait itself acts as a builder for an associated
Searcher
type, which does the actual work of finding
occurrences of the pattern in a string.
Depending on the type of the pattern, the behavior of methods like
str::find
and
str::contains
can change. The table below describes
some of those behaviors.
Pattern type
Match condition
&str
is substring
char
is contained in string
&[char]
any char in slice is contained in string
F: FnMut(char) -> bool
F
returns
true
for a char in string
&&str
is substring
&String
is substring
§
Examples
// &str
assert_eq!
(
"abaaa"
.find(
"ba"
),
Some
(
1
));
assert_eq!
(
"abaaa"
.find(
"bac"
),
None
);
// char
assert_eq!
(
"abaaa"
.find(
'a'
),
Some
(
0
));
assert_eq!
(
"abaaa"
.find(
'b'
),
Some
(
1
));
assert_eq!
(
"abaaa"
.find(
'c'
),
None
);
// &[char; N]
assert_eq!
(
"ab"
.find(
&
[
'b'
,
'a'
]),
Some
(
0
));
assert_eq!
(
"abaaa"
.find(
&
[
'a'
,
'z'
]),
Some
(
0
));
assert_eq!
(
"abaaa"
.find(
&
[
'c'
,
'd'
]),
None
);
// &[char]
assert_eq!
(
"ab"
.find(
&
[
'b'
,
'a'
][..]),
Some
(
0
));
assert_eq!
(
"abaaa"
.find(
&
[
'a'
,
'z'
][..]),
Some
(
0
));
assert_eq!
(
"abaaa"
.find(
&
[
'c'
,
'd'
][..]),
None
);
// FnMut(char) -> bool
assert_eq!
(
"abcdef_z"
.find(|ch| ch >
'd'
&& ch <
'y'
),
Some
(
4
));
assert_eq!
(
"abcddd_z"
.find(|ch| ch >
'd'
&& ch <
'y'
),
None
);
Required Associated Types
§
Source
type
Searcher
<'a>:
Searcher
<'a>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Required Methods
§
Source
fn
into_searcher
(self, haystack: &
str
) -> Self::
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
Provided Methods
§
Source
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
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
    Self::
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
    Self::
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
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
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
type
Searcher
<'a> =
StrSearcher
<'a, 'b>
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
Source
§
impl<'b>
Pattern
for &'b [
char
]
Searches for chars that are equal to any of the
char
s in the slice.
§
Examples
assert_eq!
(
"Hello world"
.find(
&
[
'o'
,
'l'
][..]),
Some
(
2
));
assert_eq!
(
"Hello world"
.find(
&
[
'h'
,
'w'
][..]),
Some
(
6
));
Source
§
type
Searcher
<'a> =
CharSliceSearcher
<'a, 'b>
Source
§
impl<'b, 'c>
Pattern
for &'c &'b
str
Delegates to the
&str
impl.
Source
§
type
Searcher
<'a> =
StrSearcher
<'a, 'b>
Source
§
impl<'b, const N:
usize
>
Pattern
for &'b [
char
;
N
]
Searches for chars that are equal to any of the
char
s in the array.
§
Examples
assert_eq!
(
"Hello world"
.find(
&
[
'o'
,
'l'
]),
Some
(
2
));
assert_eq!
(
"Hello world"
.find(
&
[
'h'
,
'w'
]),
Some
(
6
));
Source
§
type
Searcher
<'a> =
CharArrayRefSearcher
<'a, 'b, N>
Source
§
impl<F>
Pattern
for F
where
    F:
FnMut
(
char
) ->
bool
,
Searches for
char
s that match the given predicate.
§
Examples
assert_eq!
(
"Hello world"
.find(char::is_uppercase),
Some
(
0
));
assert_eq!
(
"Hello world"
.find(|c|
"aeiou"
.contains(c)),
Some
(
1
));
Source
§
type
Searcher
<'a> =
CharPredicateSearcher
<'a, F>
Source
§
impl<const N:
usize
>
Pattern
for [
char
;
N
]
Searches for chars that are equal to any of the
char
s in the array.
§
Examples
assert_eq!
(
"Hello world"
.find([
'o'
,
'l'
]),
Some
(
2
));
assert_eq!
(
"Hello world"
.find([
'h'
,
'w'
]),
Some
(
6
));
Source
§
type
Searcher
<'a> =
CharArraySearcher
<'a, N>