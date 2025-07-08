const_eval_select in std::intrinsics - Rust
std
::
intrinsics
Function
const_eval_select
Copy item path
Source
pub const fn const_eval_select<ARG, F, G, RET>(
    _arg: ARG,
    _called_in_const: F,
    _called_at_rt: G,
) -> RET
where
    ARG:
Tuple
,
    G:
FnOnce
<ARG, Output = RET>,
    F:
FnOnce
<ARG, Output = RET>,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Selects which function to call depending on the context.
If this function is evaluated at compile-time, then a call to this
intrinsic will be replaced with a call to
called_in_const
. It gets
replaced with a call to
called_at_rt
otherwise.
This function is safe to call, but note the stability concerns below.
§
Type Requirements
The two functions must be both function items. They cannot be function
pointers or closures. The first function must be a
const fn
.
arg
will be the tupled arguments that will be passed to either one of
the two functions, therefore, both functions must accept the same type of
arguments. Both functions must return RET.
§
Stability concerns
Rust has not yet decided that
const fn
are allowed to tell whether
they run at compile-time or at runtime. Therefore, when using this
intrinsic anywhere that can be reached from stable, it is crucial that
the end-to-end behavior of the stable
const fn
is the same for both
modes of execution. (Here, Undefined Behavior is considered “the same”
as any other behavior, so if the function exhibits UB at runtime then
it may do whatever it wants at compile-time.)
Here is an example of how this could cause a problem:
#![feature(const_eval_select)]
#![feature(core_intrinsics)]
use
std::intrinsics::const_eval_select;
// Standard library
pub const fn
inconsistent() -> i32 {
fn
runtime() -> i32 {
1
}
const fn
compiletime() -> i32 {
2
}
// ⚠ This code violates the required equivalence of `compiletime`
    // and `runtime`.
const_eval_select((), compiletime, runtime)
}
// User Crate
const
X: i32 = inconsistent();
let
x = inconsistent();
assert_eq!
(x, X);
Currently such an assertion would always succeed; until Rust decides
otherwise, that principle should not be violated.