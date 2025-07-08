FnOnce in std::ops - Rust
std
::
ops
Trait
FnOnce
Copy item path
1.0.0
·
Source
pub trait FnOnce<Args>
where
    Args:
Tuple
,
{
    type
Output
;

    // Required method
    extern "rust-call" fn
call_once
(self, args: Args) -> Self::
Output
;
}
Expand description
The version of the call operator that takes a by-value receiver.
Instances of
FnOnce
can be called, but might not be callable multiple
times. Because of this, if the only thing known about a type is that it
implements
FnOnce
, it can only be called once.
FnOnce
is implemented automatically by closures that might consume captured
variables, as well as all types that implement
FnMut
, e.g., (safe)
function pointers
(since
FnOnce
is a supertrait of
FnMut
).
Since both
Fn
and
FnMut
are subtraits of
FnOnce
, any instance of
Fn
or
FnMut
can be used where a
FnOnce
is expected.
Use
FnOnce
as a bound when you want to accept a parameter of function-like
type and only need to call it once. If you need to call the parameter
repeatedly, use
FnMut
as a bound; if you also need it to not mutate
state, use
Fn
.
See the
chapter on closures in
The Rust Programming Language
for
some more information on this topic.
Also of note is the special syntax for
Fn
traits (e.g.
Fn(usize, bool) -> usize
). Those interested in the technical details of
this can refer to
the relevant section in the
Rustonomicon
.
§
Examples
§
Using a
FnOnce
parameter
fn
consume_with_relish<F>(func: F)
where
F: FnOnce() -> String
{
// `func` consumes its captured variables, so it cannot be run more
    // than once.
println!
(
"Consumed: {}"
, func());
println!
(
"Delicious!"
);
// Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}
let
x = String::from(
"x"
);
let
consume_and_return_x =
move
|| x;
consume_with_relish(consume_and_return_x);
// `consume_and_return_x` can no longer be invoked at this point
Required Associated Types
§
1.12.0
·
Source
type
Output
The returned type after the call operator is used.
Required Methods
§
Source
extern "rust-call" fn
call_once
(self, args: Args) -> Self::
Output
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
Implementors
§
1.0.0
·
Source
§
impl<A, F>
FnOnce
<A> for
&F
where
    A:
Tuple
,
    F:
Fn
<A> + ?
Sized
,
Source
§
type
Output
= <F as
FnOnce
<A>>::
Output
1.0.0
·
Source
§
impl<A, F>
FnOnce
<A> for
&mut F
where
    A:
Tuple
,
    F:
FnMut
<A> + ?
Sized
,
Source
§
type
Output
= <F as
FnOnce
<A>>::
Output
1.35.0
·
Source
§
impl<Args, F, A>
FnOnce
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
FnOnce
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
Output
= <F as
FnOnce
<Args>>::
Output
Source
§
impl<F, Args>
FnOnce
<Args> for
Exclusive
<F>
where
    F:
FnOnce
<Args>,
    Args:
Tuple
,
Source
§
type
Output
= <F as
FnOnce
<Args>>::
Output
1.9.0
·
Source
§
impl<R, F>
FnOnce
() for
AssertUnwindSafe
<F>
where
    F:
FnOnce
() -> R,
Source
§
type
Output
= R