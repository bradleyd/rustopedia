take_alloc_error_hook in std::alloc - Rust
std
::
alloc
Function
take_alloc_error_hook
Copy item path
Source
pub fn take_alloc_error_hook() ->
fn
(
Layout
)
🔬
This is a nightly-only experimental API. (
alloc_error_hook
#51245
)
Expand description
Unregisters the current allocation error hook, returning it.
See also the function
set_alloc_error_hook
.
If no custom hook is registered, the default hook will be returned.