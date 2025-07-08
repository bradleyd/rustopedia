Fn in std::ops - Rust
std
::
ops
Trait
Fn
Copy item path
1.0.0
·
Source
pub trait Fn<Args>:
FnMut
<Args>
where
    Args:
Tuple
,
{
    // Required method
    extern "rust-call" fn
call
(&self, args: Args) -> Self::
Output
;
}
Expand description
The version of the call operator that takes an immutable receiver.
Instances of
Fn
can be called repeatedly without mutating state.
This trait (
Fn
) is not to be confused with
function pointers
(
fn
).
Fn
is implemented automatically by closures which only take immutable
references to captured variables or don’t capture anything at all, as well
as (safe)
function pointers
(with some caveats, see their documentation
for more details). Additionally, for any type
F
that implements
Fn
,
&F
implements
Fn
, too.
Since both
FnMut
and
FnOnce
are supertraits of
Fn
, any
instance of
Fn
can be used as a parameter where a
FnMut
or
FnOnce
is expected.
Use
Fn
as a bound when you want to accept a parameter of function-like
type and need to call it repeatedly and without mutating state (e.g., when
calling it concurrently). If you do not need such strict requirements, use
FnMut
or
FnOnce
as bounds.
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
Calling a closure
let
square = |x| x * x;
assert_eq!
(square(
5
),
25
);
§
Using a
Fn
parameter
fn
call_with_one<F>(func: F) -> usize
where
F: Fn(usize) -> usize {
    func(
1
)
}
let
double = |x| x *
2
;
assert_eq!
(call_with_one(double),
2
);
Required Methods
§
Source
extern "rust-call" fn
call
(&self, args: Args) -> Self::
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
Fn
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
1.35.0
·
Source
§
impl<Args, F, A>
Fn
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
Fn
<Args> + ?
Sized
,
    A:
Allocator
,