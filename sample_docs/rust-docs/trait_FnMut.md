FnMut in std::ops - Rust
std
::
ops
Trait
FnMut
Copy item path
1.0.0
·
Source
pub trait FnMut<Args>:
FnOnce
<Args>
where
    Args:
Tuple
,
{
    // Required method
    extern "rust-call" fn
call_mut
(
        &mut self,
        args: Args,
    ) -> Self::
Output
;
}
Expand description
The version of the call operator that takes a mutable receiver.
Instances of
FnMut
can be called repeatedly and may mutate state.
FnMut
is implemented automatically by closures which take mutable
references to captured variables, as well as all types that implement
Fn
, e.g., (safe)
function pointers
(since
FnMut
is a supertrait of
Fn
). Additionally, for any type
F
that implements
FnMut
,
&mut F
implements
FnMut
, too.
Since
FnOnce
is a supertrait of
FnMut
, any instance of
FnMut
can be
used where a
FnOnce
is expected, and since
Fn
is a subtrait of
FnMut
, any instance of
Fn
can be used where
FnMut
is expected.
Use
FnMut
as a bound when you want to accept a parameter of function-like
type and need to call it repeatedly, while allowing it to mutate state.
If you don’t want the parameter to mutate state, use
Fn
as a
bound; if you don’t need to call it repeatedly, use
FnOnce
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
Calling a mutably capturing closure
let
mut
x =
5
;
{
let
mut
square_x = || x
*
= x;
    square_x();
}
assert_eq!
(x,
25
);
§
Using a
FnMut
parameter
fn
do_twice<F>(
mut
func: F)
where
F: FnMut()
{
    func();
    func();
}
let
mut
x: usize =
1
;
{
let
add_two_to_x = || x +=
2
;
    do_twice(add_two_to_x);
}
assert_eq!
(x,
5
);
Required Methods
§
Source
extern "rust-call" fn
call_mut
(&mut self, args: Args) -> Self::
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
FnMut
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
1.0.0
·
Source
§
impl<A, F>
FnMut
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
1.35.0
·
Source
§
impl<Args, F, A>
FnMut
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
FnMut
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
impl<F, Args>
FnMut
<Args> for
Exclusive
<F>
where
    F:
FnMut
<Args>,
    Args:
Tuple
,