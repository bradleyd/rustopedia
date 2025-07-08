handle_alloc_error in std::alloc - Rust
std
::
alloc
Function
handle_alloc_error
Copy item path
1.28.0 (const:
unstable
)
·
Source
pub fn handle_alloc_error(layout:
Layout
) ->
!
Expand description
Signals a memory allocation error.
Callers of memory allocation APIs wishing to cease execution
in response to an allocation error are encouraged to call this function,
rather than directly invoking
panic!
or similar.
This function is guaranteed to diverge (not return normally with a value), but depending on
global configuration, it may either panic (resulting in unwinding or aborting as per
configuration for all panics), or abort the process (with no unwinding).
The default behavior is:
If the binary links against
std
(typically the case), then
print a message to standard error and abort the process.
This behavior can be replaced with
set_alloc_error_hook
and
take_alloc_error_hook
.
Future versions of Rust may panic by default instead.
If the binary does not link against
std
(all of its crates are marked
#![no_std]
), then call
panic!
with a message.
The panic handler
applies as to any panic.