DoubleEndedSearcher in std::str::pattern - Rust
std
::
str
::
pattern
Trait
DoubleEndedSearcher
Copy item path
Source
pub trait DoubleEndedSearcher<'a>:
ReverseSearcher
<'a> { }
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expand description
A marker trait to express that a
ReverseSearcher
can be used for a
DoubleEndedIterator
implementation.
For this, the impl of
Searcher
and
ReverseSearcher
need
to follow these conditions:
All results of
next()
need to be identical
to the results of
next_back()
in reverse order.
next()
and
next_back()
need to behave as
the two ends of a range of values, that is they
can not “walk past each other”.
§
Examples
char::Searcher
is a
DoubleEndedSearcher
because searching for a
char
only requires looking at one at a time, which behaves the same
from both ends.
(&str)::Searcher
is not a
DoubleEndedSearcher
because
the pattern
"aa"
in the haystack
"aaa"
matches as either
"[aa]a"
or
"a[aa]"
, depending on which side it is searched.
Implementors
§
Source
§
impl<'a>
DoubleEndedSearcher
<'a> for
CharSearcher
<'a>
Source
§
impl<'a, 'b>
DoubleEndedSearcher
<'a> for
CharSliceSearcher
<'a, 'b>
Source
§
impl<'a, 'b, const N:
usize
>
DoubleEndedSearcher
<'a> for
CharArrayRefSearcher
<'a, 'b, N>
Source
§
impl<'a, F>
DoubleEndedSearcher
<'a> for
CharPredicateSearcher
<'a, F>
where
    F:
FnMut
(
char
) ->
bool
,
Source
§
impl<'a, const N:
usize
>
DoubleEndedSearcher
<'a> for
CharArraySearcher
<'a, N>