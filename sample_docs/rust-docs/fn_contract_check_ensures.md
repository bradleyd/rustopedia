contract_check_ensures in std::intrinsics - Rust
std
::
intrinsics
Function
contract_check_ensures
Copy item path
Source
pub fn contract_check_ensures<'a, Ret, C>(ret:
&'a Ret
, cond: C)
where
    C:
Fn
(
&'a Ret
) ->
bool
,
🔬
This is a nightly-only experimental API. (
contracts_internals
#128044
)
Expand description
Check if the post-condition
cond
has been met.
By default, if
contract_checks
is enabled, this will panic with no unwind if the condition
returns false.