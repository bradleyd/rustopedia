set_alloc_error_hook in std::alloc - Rust
std
::
alloc
Function
set_alloc_error_hook
Copy item path
Source
pub fn set_alloc_error_hook(hook:
fn
(
Layout
))
🔬
This is a nightly-only experimental API. (
alloc_error_hook
#51245
)
Expand description
Registers a custom allocation error hook, replacing any that was previously registered.
The allocation error hook is invoked when an infallible memory allocation fails — that is,
as a consequence of calling
handle_alloc_error
— before the runtime aborts.
The allocation error hook is a global resource.
take_alloc_error_hook
may be used to
retrieve a previously registered hook and wrap or discard it.
§
What the provided
hook
function should expect
The hook function is provided with a
Layout
struct which contains information
about the allocation that failed.
The hook function may choose to panic or abort; in the event that it returns normally, this
will cause an immediate abort.
Since
take_alloc_error_hook
is a safe function that allows retrieving the hook, the hook
function must be
sound
to call even if no memory allocations were attempted.
§
The default hook
The default hook, used if
set_alloc_error_hook
is never called, prints a message to
standard error (and then returns, causing the runtime to abort the process).
Compiler options may cause it to panic instead, and the default behavior may be changed
to panicking in future versions of Rust.
§
Examples
#![feature(alloc_error_hook)]
use
std::alloc::{Layout, set_alloc_error_hook};
fn
custom_alloc_error_hook(layout: Layout) {
panic!
(
"memory allocation of {} bytes failed"
, layout.size());
}

set_alloc_error_hook(custom_alloc_error_hook);