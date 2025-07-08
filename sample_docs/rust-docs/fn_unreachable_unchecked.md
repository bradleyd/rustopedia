unreachable_unchecked in std::hint - Rust
std
::
hint
Function
unreachable_unchecked
Copy item path
1.27.0 (const: 1.57.0)
·
Source
pub const unsafe fn unreachable_unchecked() ->
!
Expand description
Informs the compiler that the site which is calling this function is not
reachable, possibly enabling further optimizations.
§
Safety
Reaching this function is
Undefined Behavior
.
As the compiler assumes that all forms of Undefined Behavior can never
happen, it will eliminate all branches in the surrounding code that it can
determine will invariably lead to a call to
unreachable_unchecked()
.
If the assumptions embedded in using this function turn out to be wrong -
that is, if the site which is calling
unreachable_unchecked()
is actually
reachable at runtime - the compiler may have generated nonsensical machine
instructions for this situation, including in seemingly unrelated code,
causing difficult-to-debug problems.
Use this function sparingly. Consider using the
unreachable!
macro,
which may prevent some optimizations but will safely panic in case it is
actually reached at runtime. Benchmark your code to find out if using
unreachable_unchecked()
comes with a performance benefit.
§
Examples
unreachable_unchecked()
can be used in situations where the compiler
can’t prove invariants that were previously established. Such situations
have a higher chance of occurring if those invariants are upheld by
external code that the compiler can’t analyze.
fn
prepare_inputs(divisors:
&mut
Vec<u32>) {
// Note to future-self when making changes: The invariant established
    // here is NOT checked in `do_computation()`; if this changes, you HAVE
    // to change `do_computation()`.
divisors.retain(|divisor|
*
divisor !=
0
)
}
/// # Safety
/// All elements of `divisor` must be non-zero.
unsafe fn
do_computation(i: u32, divisors:
&
[u32]) -> u32 {
    divisors.iter().fold(i, |acc, divisor| {
// Convince the compiler that a division by zero can't happen here
        // and a check is not needed below.
if
*
divisor ==
0
{
// Safety: `divisor` can't be zero because of `prepare_inputs`,
            // but the compiler does not know about this. We *promise*
            // that we always call `prepare_inputs`.
unsafe
{ std::hint::unreachable_unchecked() }
        }
// The compiler would normally introduce a check here that prevents
        // a division by zero. However, if `divisor` was zero, the branch
        // above would reach what we explicitly marked as unreachable.
        // The compiler concludes that `divisor` can't be zero at this point
        // and removes the - now proven useless - check.
acc / divisor
    })
}
let
mut
divisors =
vec!
[
2
,
0
,
4
];
prepare_inputs(
&mut
divisors);
let
result =
unsafe
{
// Safety: prepare_inputs() guarantees that divisors is non-zero
do_computation(
100
,
&
divisors)
};
assert_eq!
(result,
12
);
While using
unreachable_unchecked()
is perfectly sound in the following
example, as the compiler is able to prove that a division by zero is not
possible, benchmarking reveals that
unreachable_unchecked()
provides
no benefit over using
unreachable!
, while the latter does not introduce
the possibility of Undefined Behavior.
fn
div_1(a: u32, b: u32) -> u32 {
use
std::hint::unreachable_unchecked;
// `b.saturating_add(1)` is always positive (not zero),
    // hence `checked_div` will never return `None`.
    // Therefore, the else branch is unreachable.
a.checked_div(b.saturating_add(
1
))
        .unwrap_or_else(||
unsafe
{ unreachable_unchecked() })
}
assert_eq!
(div_1(
7
,
0
),
7
);
assert_eq!
(div_1(
9
,
1
),
4
);
assert_eq!
(div_1(
11
, u32::MAX),
0
);