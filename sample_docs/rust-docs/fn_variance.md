variance in std::marker - Rust
std
::
marker
Function
variance
Copy item path
Source
pub const fn variance<T>() -> T
where
    T:
Variance
,
🔬
This is a nightly-only experimental API. (
phantom_variance_markers
#135806
)
Expand description
Construct a variance marker; equivalent to
Default::default
.
This type can be any of the following. You generally should not need to explicitly name the
type, however.
PhantomCovariant
PhantomContravariant
PhantomInvariant
PhantomCovariantLifetime
PhantomContravariantLifetime
PhantomInvariantLifetime
§
Example
#![feature(phantom_variance_markers)]
use
core::marker::{PhantomCovariant, variance};
struct
BoundFn<F, P, R>
where
F: Fn(P) -> R,
{
    function: F,
    parameter: P,
    return_value: PhantomCovariant<R>,
}
let
bound_fn = BoundFn {
    function: core::convert::identity,
    parameter:
5u8
,
    return_value: variance(),
};