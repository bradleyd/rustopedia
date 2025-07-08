poll_fn in std::future - Rust
std
::
future
Function
poll_fn
Copy item path
1.64.0
·
Source
pub fn poll_fn<T, F>(f: F) ->
PollFn
<F>
ⓘ
where
    F:
FnMut
(&mut
Context
<'_>) ->
Poll
<T>,
Expand description
Creates a future that wraps a function returning
Poll
.
Polling the future delegates to the wrapped function. If the returned future is pinned, then the
captured environment of the wrapped function is also pinned in-place, so as long as the closure
does not move out of its captures it can soundly create pinned references to them.
§
Examples
use
core::future::poll_fn;
use
std::task::{Context, Poll};
fn
read_line(_cx:
&mut
Context<
'_
>) -> Poll<String> {
    Poll::Ready(
"Hello, World!"
.into())
}
let
read_future = poll_fn(read_line);
assert_eq!
(read_future.
await
,
"Hello, World!"
.to_owned());
§
Capturing a pinned state
Example of a closure wrapping inner futures:
use
core::future::{
self
, Future};
use
core::task::Poll;
/// Resolves to the first future that completes. In the event of a tie, `a` wins.
fn
naive_select<T>(
    a:
impl
Future<Output = T>,
    b:
impl
Future<Output = T>,
) ->
impl
Future<Output = T>
{
let
(
mut
a,
mut
b) = (Box::pin(a), Box::pin(b));
    future::poll_fn(
move
|cx| {
if let
Poll::Ready(r) = a.as_mut().poll(cx) {
            Poll::Ready(r)
        }
else if let
Poll::Ready(r) = b.as_mut().poll(cx) {
            Poll::Ready(r)
        }
else
{
            Poll::Pending
        }
    })
}
let
a =
async
{
42
};
let
b = future::pending();
let
v = naive_select(a, b).
await
;
assert_eq!
(v,
42
);
let
a = future::pending();
let
b =
async
{
27
};
let
v = naive_select(a, b).
await
;
assert_eq!
(v,
27
);
let
a =
async
{
42
};
let
b =
async
{
27
};
let
v = naive_select(a, b).
await
;
assert_eq!
(v,
42
);
// biased towards `a` in case of tie!
This time without
Box::pin
ning:
use
core::future::{
self
, Future};
use
core::pin::pin;
use
core::task::Poll;
/// Resolves to the first future that completes. In the event of a tie, `a` wins.
fn
naive_select<T>(
    a:
impl
Future<Output = T>,
    b:
impl
Future<Output = T>,
) ->
impl
Future<Output = T>
{
async
{
let
(
mut
a,
mut
b) = (
pin!
(a),
pin!
(b));
        future::poll_fn(
move
|cx| {
if let
Poll::Ready(r) = a.as_mut().poll(cx) {
                Poll::Ready(r)
            }
else if let
Poll::Ready(r) = b.as_mut().poll(cx) {
                Poll::Ready(r)
            }
else
{
                Poll::Pending
            }
        }).
await
}
}
let
a =
async
{
42
};
let
b = future::pending();
let
v = naive_select(a, b).
await
;
assert_eq!
(v,
42
);
Notice how, by virtue of being in an
async
context, we have been able to make the
pin!
macro work, thereby avoiding any need for the
unsafe
Pin::new_unchecked
(&mut fut)
constructor.