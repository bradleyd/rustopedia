from_coroutine in std::iter - Rust
std
::
iter
Function
from_coroutine
Copy item path
Source
pub fn from_coroutine<G>(coroutine: G) ->
FromCoroutine
<G>
ⓘ
where
    G:
Coroutine
<Return =
()
> +
Unpin
,
🔬
This is a nightly-only experimental API. (
iter_from_coroutine
#43122
)
Expand description
Creates a new iterator where each iteration calls the provided coroutine.
Similar to
iter::from_fn
.
§
Examples
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
let
it = std::iter::from_coroutine(
#[coroutine]
|| {
yield
1
;
yield
2
;
yield
3
;
});
let
v: Vec<
_
> = it.collect();
assert_eq!
(v, [
1
,
2
,
3
]);