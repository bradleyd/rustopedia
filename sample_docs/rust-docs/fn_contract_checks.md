contract_checks in std::intrinsics - Rust
std
::
intrinsics
Function
contract_checks
Copy item path
Source
pub const fn contract_checks() ->
bool
🔬
This is a nightly-only experimental API. (
contracts_internals
#128044
)
Expand description
Returns whether we should perform contract-checking at runtime.
This is meant to be similar to the ub_checks intrinsic, in terms
of not prematurely commiting at compile-time to whether contract
checking is turned on, so that we can specify contracts in libstd
and let an end user opt into turning them on.