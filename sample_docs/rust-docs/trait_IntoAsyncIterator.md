IntoAsyncIterator in std::async_iter - Rust
std
::
async_iter
Trait
IntoAsyncIterator
Copy item path
Source
pub trait IntoAsyncIterator {
    type
Item
;
    type
IntoAsyncIter
:
AsyncIterator
<Item = Self::
Item
>;

    // Required method
    fn
into_async_iter
(self) -> Self::
IntoAsyncIter
;
}
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Expand description
Converts something into an async iterator
Required Associated Types
§
Source
type
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the item yielded by the iterator
Source
type
IntoAsyncIter
:
AsyncIterator
<Item = Self::
Item
>
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the resulting iterator
Required Methods
§
Source
fn
into_async_iter
(self) -> Self::
IntoAsyncIter
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Converts
self
into an async iterator
Implementors
§
Source
§
impl<I>
IntoAsyncIterator
for I
where
    I:
AsyncIterator
,
Source
§
type
Item
= <I as
AsyncIterator
>::
Item
Source
§
type
IntoAsyncIter
= I