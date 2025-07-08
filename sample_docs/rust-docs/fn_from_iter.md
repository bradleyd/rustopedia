from_iter in std::async_iter - Rust
std
::
async_iter
Function
from_iter
Copy item path
Source
pub fn from_iter<I>(iter: I) ->
FromIter
<<I as
IntoIterator
>::
IntoIter
>
where
    I:
IntoIterator
,
🔬
This is a nightly-only experimental API. (
async_iter_from_iter
#81798
)
Expand description
Converts an iterator into an async iterator.