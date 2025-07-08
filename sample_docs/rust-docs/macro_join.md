join in std::future - Rust
std
::
future
Macro
join
Copy item path
Source
pub macro join($($fut:expr),+ $(,)?) {
    ...
}
🔬
This is a nightly-only experimental API. (
future_join
#91642
)
Expand description
Polls multiple futures simultaneously, returning a tuple
of all results once complete.
While
join!(a, b).await
is similar to
(a.await, b.await)
,
join!
polls both futures concurrently and is therefore more efficient.
§
Examples
#![feature(future_join)]
use
std::future::join;
async fn
one() -> usize {
1
}
async fn
two() -> usize {
2
}
let
x =
join!
(one(), two()).
await
;
assert_eq!
(x, (
1
,
2
));
join!
is variadic, so you can pass any number of futures:
#![feature(future_join)]
use
std::future::join;
async fn
one() -> usize {
1
}
async fn
two() -> usize {
2
}
async fn
three() -> usize {
3
}
let
x =
join!
(one(), two(), three()).
await
;
assert_eq!
(x, (
1
,
2
,
3
));