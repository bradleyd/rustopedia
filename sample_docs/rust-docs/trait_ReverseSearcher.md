ReverseSearcher in std::str::pattern - Rust
std
::
str
::
pattern
Trait
ReverseSearcher
Copy item path
Source
pub unsafe trait ReverseSearcher<'a>:
Searcher
<'a> {
    // Required method
    fn
next_back
(&mut self) ->
SearchStep
;

    // Provided methods
    fn
next_match_back
(&mut self) ->
Option
<(
usize
,
usize
)> { ... }
fn
next_reject_back
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
A reverse searcher for a string pattern.
This trait provides methods for searching for non-overlapping
matches of a pattern starting from the back (right) of a string.
It will be implemented by associated
Searcher
types of the
Pattern
trait if the pattern supports searching
for it from the back.
The index ranges returned by this trait are not required
to exactly match those of the forward search in reverse.
For the reason why this trait is marked unsafe, see the
parent trait
Searcher
.
Required Methods
§
Source
fn
next_back
(&mut self) ->
SearchStep
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Performs the next search step starting from the back.
Returns
Match(a, b)
if
haystack[a..b]
matches the pattern.
Returns
Reject(a, b)
if
haystack[a..b]
can not match the pattern, even partially.
Returns
Done
if every byte of the haystack
has been visited
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
[Reject(7, 8), Match(4, 7), Reject(1, 4), Reject(0, 1)]
.
Provided Methods
§
Source
fn
next_match_back
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
result.
See
next_back()
.
Source
fn
next_reject_back
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
result.
See
next_back()
.
Implementors
§
Source
§
impl<'a>
ReverseSearcher
<'a> for
CharSearcher
<'a>
Source
§
impl<'a, 'b>
ReverseSearcher
<'a> for
CharSliceSearcher
<'a, 'b>
Source
§
impl<'a, 'b>
ReverseSearcher
<'a> for
StrSearcher
<'a, 'b>
Source
§
impl<'a, 'b, const N:
usize
>
ReverseSearcher
<'a> for
CharArrayRefSearcher
<'a, 'b, N>
Source
§
impl<'a, F>
ReverseSearcher
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
ReverseSearcher
<'a> for
CharArraySearcher
<'a, N>