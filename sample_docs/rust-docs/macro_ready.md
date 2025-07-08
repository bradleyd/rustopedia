ready in std::task - Rust
std
::
task
Macro
ready
Copy item path
1.64.0
·
Source
pub macro ready($e:expr) {
    ...
}
Expand description
Extracts the successful type of a
Poll<T>
.
This macro bakes in propagation of
Pending
signals by returning early.
§
Examples
use
std::task::{ready, Context, Poll};
use
std::future::{
self
, Future};
use
std::pin::Pin;
pub fn
do_poll(cx:
&mut
Context<
'_
>) -> Poll<()> {
let
mut
fut = future::ready(
42
);
let
fut = Pin::new(
&mut
fut);
let
num =
ready!
(fut.poll(cx));
// ... use num
Poll::Ready(())
}
The
ready!
call expands to:
let
num =
match
fut.poll(cx) {
    Poll::Ready(t) => t,
    Poll::Pending =>
return
Poll::Pending,
};