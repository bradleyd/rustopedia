vars in std::env - Rust
std
::
env
Function
vars
Copy item path
1.0.0
·
Source
pub fn vars() ->
Vars
ⓘ
Expand description
Returns an iterator of (variable, value) pairs of strings, for all the
environment variables of the current process.
The returned iterator contains a snapshot of the process’s environment
variables at the time of this invocation. Modifications to environment
variables afterwards will not be reflected in the returned iterator.
§
Panics
While iterating, the returned iterator will panic if any key or value in the
environment is not valid unicode. If this is not desired, consider using
env::vars_os()
.
§
Examples
// Print all environment variables.
for
(key, value)
in
std::env::vars() {
println!
(
"{key}: {value}"
);
}