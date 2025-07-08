contract_check_requires in std::intrinsics - Rust
std
::
intrinsics
Function
contract_check_requires
Copy item path
Source
pub fn contract_check_requires<C>(cond: C)
where
    C:
Fn
() ->
bool
,
🔬
This is a nightly-only experimental API. (
contracts_internals
#128044
)
Expand description
Check if the pre-condition
cond
has been met.
By default, if
contract_checks
is enabled, this will panic with no unwind if the condition
returns false.