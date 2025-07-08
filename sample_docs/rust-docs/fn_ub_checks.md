ub_checks in std::intrinsics - Rust
std
::
intrinsics
Function
ub_checks
Copy item path
Source
pub const fn ub_checks() ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns whether we should perform some UB-checking at runtime. This eventually evaluates to
cfg!(ub_checks)
, but behaves different from
cfg!
when mixing crates built with different
flags: if the crate has UB checks enabled or carries the
#[rustc_preserve_ub_checks]
attribute, evaluation is delayed until monomorphization (or until the call gets inlined into
a crate that does not delay evaluation further); otherwise it can happen any time.
The common case here is a user program built with ub_checks linked against the distributed
sysroot which is built without ub_checks but with
#[rustc_preserve_ub_checks]
.
For code that gets monomorphized in the user crate (i.e., generic functions and functions with
#[inline]
), gating assertions on
ub_checks()
rather than
cfg!(ub_checks)
means that
assertions are enabled whenever the
user crate
has UB checks enabled. However, if the
user has UB checks disabled, the checks will still get optimized out. This intrinsic is
primarily used by
ub_checks::assert_unsafe_precondition
.