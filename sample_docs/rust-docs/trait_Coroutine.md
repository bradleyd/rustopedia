Coroutine in std::ops - Rust
std
::
ops
Trait
Coroutine
Copy item path
Source
pub trait Coroutine<R =
()
> {
    type
Yield
;
    type
Return
;

    // Required method
    fn
resume
(
        self:
Pin
<&mut Self>,
        arg: R,
    ) ->
CoroutineState
<Self::
Yield
, Self::
Return
>;
}
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Expand description
The trait implemented by builtin coroutine types.
Coroutines are currently an
experimental language feature in Rust. Added in
RFC 2033
coroutines are
currently intended to primarily provide a building block for async/await
syntax but will likely extend to also providing an ergonomic definition for
iterators and other primitives.
The syntax and semantics for coroutines is unstable and will require a
further RFC for stabilization. At this time, though, the syntax is
closure-like:
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
use
std::ops::{Coroutine, CoroutineState};
use
std::pin::Pin;
fn
main() {
let
mut
coroutine =
#[coroutine]
|| {
yield
1
;
"foo"
};
match
Pin::new(
&mut
coroutine).resume(()) {
        CoroutineState::Yielded(
1
) => {}
_
=>
panic!
(
"unexpected return from resume"
),
    }
match
Pin::new(
&mut
coroutine).resume(()) {
        CoroutineState::Complete(
"foo"
) => {}
_
=>
panic!
(
"unexpected return from resume"
),
    }
}
More documentation of coroutines can be found in the
unstable book
.
Required Associated Types
§
Source
type
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
This associated type corresponds to the
yield
expression and the
values which are allowed to be returned each time a coroutine yields.
For example an iterator-as-a-coroutine would likely have this type as
T
, the type being iterated over.
Source
type
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
This corresponds to the type returned from a coroutine either with a
return
statement or implicitly as the last expression of a coroutine
literal. For example futures would use this as
Result<T, E>
as it
represents a completed future.
Required Methods
§
Source
fn
resume
(
    self:
Pin
<&mut Self>,
    arg: R,
) ->
CoroutineState
<Self::
Yield
, Self::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
This function will resume execution of the coroutine or start execution
if it hasn’t already. This call will return back into the coroutine’s
last suspension point, resuming execution from the latest
yield
. The
coroutine will continue executing until it either yields or returns, at
which point this function will return.
§
Return value
The
CoroutineState
enum returned from this function indicates what
state the coroutine is in upon returning. If the
Yielded
variant is
returned then the coroutine has reached a suspension point and a value
has been yielded out. Coroutines in this state are available for
resumption at a later point.
If
Complete
is returned then the coroutine has completely finished
with the value provided. It is invalid for the coroutine to be resumed
again.
§
Panics
This function may panic if it is called after the
Complete
variant has
been returned previously. While coroutine literals in the language are
guaranteed to panic on resuming after
Complete
, this is not guaranteed
for all implementations of the
Coroutine
trait.
Implementors
§
Source
§
impl<G, R>
Coroutine
<R> for
&mut G
where
    G:
Coroutine
<R> +
Unpin
+ ?
Sized
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
Source
§
impl<G, R>
Coroutine
<R> for
Pin
<
&mut G
>
where
    G:
Coroutine
<R> + ?
Sized
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
Source
§
impl<G, R, A>
Coroutine
<R> for
Box
<G, A>
where
    G:
Coroutine
<R> +
Unpin
+ ?
Sized
,
    A:
Allocator
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
Source
§
impl<G, R, A>
Coroutine
<R> for
Pin
<
Box
<G, A>>
where
    G:
Coroutine
<R> + ?
Sized
,
    A:
Allocator
+ 'static,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
Source
§
impl<R, G>
Coroutine
<R> for
Exclusive
<G>
where
    G:
Coroutine
<R> + ?
Sized
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return