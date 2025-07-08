pin in std::pin - Rust
std
::
pin
Macro
pin
Copy item path
1.68.0
·
Source
pub macro pin($value:expr $(,)?) {
    ...
}
Expand description
Constructs a
Pin
<
&mut
T>
, by pinning a
value: T
locally.
Unlike
Box::pin
, this does not create a new heap allocation. As explained
below, the element might still end up on the heap however.
The local pinning performed by this macro is usually dubbed “stack”-pinning.
Outside of
async
contexts locals do indeed get stored on the stack. In
async
functions or blocks however, any locals crossing an
.await
point
are part of the state captured by the
Future
, and will use the storage of
those. That storage can either be on the heap or on the stack. Therefore,
local pinning is a more accurate term.
If the type of the given value does not implement
Unpin
, then this macro
pins the value in memory in a way that prevents moves. On the other hand,
if the type does implement
Unpin
,
Pin
<
&mut
T>
behaves
like
&mut
T
, and operations such as
mem::replace()
or
mem::take()
will allow moves of the value.
See
the
Unpin
section of the
pin
module
for details.
§
Examples
§
Basic usage
use
core::pin::{pin, Pin};
fn
stuff(foo: Pin<
&mut
Foo>) {
// …
}
let
pinned_foo =
pin!
(Foo {
/* … */
});
stuff(pinned_foo);
// or, directly:
stuff(
pin!
(Foo {
/* … */
}));
§
Manually polling a
Future
(without
Unpin
bounds)
use
std::{
    future::Future,
    pin::pin,
    task::{Context, Poll},
    thread,
};
/// Runs a future to completion.
fn
block_on<Fut: Future>(fut: Fut) -> Fut::Output {
let
waker_that_unparks_thread =
// …
let
mut
cx = Context::from_waker(
&
waker_that_unparks_thread);
// Pin the future so it can be polled.
let
mut
pinned_fut =
pin!
(fut);
loop
{
match
pinned_fut.as_mut().poll(
&mut
cx) {
            Poll::Pending => thread::park(),
            Poll::Ready(res) =>
return
res,
        }
    }
}
§
With
Coroutine
s
#![feature(coroutines)]
#![feature(coroutine_trait)]
use
core::{
    ops::{Coroutine, CoroutineState},
    pin::pin,
};
fn
coroutine_fn() ->
impl
Coroutine<Yield = usize, Return = ()>
/* not Unpin */
{
// Allow coroutine to be self-referential (not `Unpin`)
 // vvvvvv        so that locals can cross yield points.
#[coroutine]
static
|| {
let
foo = String::from(
"foo"
);
let
foo_ref =
&
foo;
// ------+
yield
0
;
// | <- crosses yield point!
println!
(
"{foo_ref}"
);
// <--+
yield
foo.len();
    }
}
fn
main() {
let
mut
coroutine =
pin!
(coroutine_fn());
match
coroutine.as_mut().resume(()) {
        CoroutineState::Yielded(
0
) => {},
_
=>
unreachable!
(),
    }
match
coroutine.as_mut().resume(()) {
        CoroutineState::Yielded(
3
) => {},
_
=>
unreachable!
(),
    }
match
coroutine.resume(()) {
        CoroutineState::Yielded(
_
) =>
unreachable!
(),
        CoroutineState::Complete(()) => {},
    }
}
§
Remarks
Precisely because a value is pinned to local storage, the resulting
Pin
<
&mut
T>
reference ends up borrowing a local tied to that block: it can’t escape it.
The following, for instance, fails to compile:
ⓘ
use
core::pin::{pin, Pin};
let
x: Pin<
&mut
Foo> = {
let
x: Pin<
&mut
Foo> =
pin!
(Foo {
/* … */
});
    x
};
// <- Foo is dropped
stuff(x);
// Error: use of dropped value
Error message
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:9:28
   |
8  | let x: Pin<&mut Foo> = {
   |     - borrow later stored here
9  |     let x: Pin<&mut Foo> = pin!(Foo { /* … */ });
   |                            ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
10 |     x
11 | }; // <- Foo is dropped
   | - temporary value is freed at the end of this statement
   |
   = note: consider using a `let` binding to create a longer lived value
This makes
pin!
unsuitable to pin values when intending to
return
them
. Instead, the
value is expected to be passed around
unpinned
until the point where it is to be consumed,
where it is then useful and even sensible to pin the value locally using
pin!
.
If you really need to return a pinned value, consider using
Box::pin
instead.
On the other hand, local pinning using
pin!
is likely to be cheaper than
pinning into a fresh heap allocation using
Box::pin
. Moreover, by virtue of not
requiring an allocator,
pin!
is the main non-
unsafe
#![no_std]
-compatible
Pin
constructor.