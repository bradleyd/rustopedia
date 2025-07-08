vars_os in std::env - Rust
std
::
env
Function
vars_os
Copy item path
1.0.0
·
Source
pub fn vars_os() ->
VarsOs
ⓘ
Expand description
Returns an iterator of (variable, value) pairs of OS strings, for all the
environment variables of the current process.
The returned iterator contains a snapshot of the process’s environment
variables at the time of this invocation. Modifications to environment
variables afterwards will not be reflected in the returned iterator.
Note that the returned iterator will not check if the environment variables
are valid Unicode. If you want to panic on invalid UTF-8,
use the
vars
function instead.
§
Examples
// Print all environment variables.
for
(key, value)
in
std::env::vars_os() {
println!
(
"{key:?}: {value:?}"
);
}