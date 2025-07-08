Searcher in std::str::pattern - Rust
std
::
str
::
pattern
Trait
Searcher
Copy item path
Source
pub unsafe trait Searcher<'a> {
    // Required methods
    fn
haystack
(&self) -> &'a
str
;
fn
next
(&mut self) ->
SearchStep
;

    // Provided methods
    fn
next_match
(&mut self) ->
Option
<(
usize
,
usize
)> { ... }
fn
next_reject
(&mut self) ->
Option
<(
usize
,
usize
)> { ... }
}
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Expand description
A searcher for a string pattern.
This trait provides methods for searching for non-overlapping
matches of a pattern starting from the front (left) of a string.
It will be implemented by associated
Searcher
types of the
Pattern
trait.
The trait is marked unsafe because the indices returned by the
next()
methods are required to lie on valid utf8
boundaries in the haystack. This enables consumers of this trait to
slice the haystack without additional runtime checks.
Required Methods
§
Source
fn
haystack
(&self) -> &'a
str
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Getter for the underlying string to be searched in
Will always return the same
&str
.
Source
fn
next
(&mut self) ->
SearchStep
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Performs the next search step starting from the front.
Returns
Match(a, b)
if
haystack[a..b]
matches
the pattern.
Returns
Reject(a, b)
if
haystack[a..b]
can
not match the pattern, even partially.
Returns
Done
if every byte of the haystack has
been visited.
The stream of
Match
and
Reject
values up to a
Done
will contain index ranges that are adjacent, non-overlapping,
covering the whole haystack, and laying on utf8 boundaries.
A
Match
result needs to contain the whole matched
pattern, however
Reject
results may be split up
into arbitrary many adjacent fragments. Both ranges may have zero length.
As an example, the pattern
"aaa"
and the haystack
"cbaaaaab"
might produce the stream
[Reject(0, 1), Reject(1, 2), Match(2, 5), Reject(5, 8)]
Provided Methods
§
Source
fn
next_match
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Match
result. See
next()
.
Unlike
next()
, there is no guarantee that the returned ranges
of this and
next_reject
will overlap. This will return
(start_match, end_match)
, where start_match is the index of where
the match begins, and end_match is the index after the end of the match.
Source
fn
next_reject
(&mut self) ->
Option
<(
usize
,
usize
)>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Finds the next
Reject
result. See
next()
and
next_match()
.
Unlike
next()
, there is no guarantee that the returned ranges
of this and
next_match
will overlap.
Implementors
§
Source
§
impl<'a>
Searcher
<'a> for
CharSearcher
<'a>
Source
§
impl<'a, 'b>
Searcher
<'a> for
CharSliceSearcher
<'a, 'b>
Source
§
impl<'a, 'b>
Searcher
<'a> for
StrSearcher
<'a, 'b>
Source
§
impl<'a, 'b, const N:
usize
>
Searcher
<'a> for
CharArrayRefSearcher
<'a, 'b, N>
Source
§
impl<'a, F>
Searcher
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
Searcher
<'a> for
CharArraySearcher
<'a, N>