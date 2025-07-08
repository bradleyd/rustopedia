args in std::env - Rust
std
::
env
Function
args
Copy item path
1.0.0
·
Source
pub fn args() ->
Args
ⓘ
Expand description
Returns the arguments that this program was started with (normally passed
via the command line).
The first element is traditionally the path of the executable, but it can be
set to arbitrary text, and might not even exist. This means this property should
not be relied upon for security purposes.
On Unix systems the shell usually expands unquoted arguments with glob patterns
(such as
*
and
?
). On Windows this is not done, and such arguments are
passed as-is.
On glibc Linux systems, arguments are retrieved by placing a function in
.init_array
.
glibc passes
argc
,
argv
, and
envp
to functions in
.init_array
, as a non-standard
extension. This allows
std::env::args
to work even in a
cdylib
or
staticlib
, as it
does on macOS and Windows.
§
Panics
The returned iterator will panic during iteration if any argument to the
process is not valid Unicode. If this is not desired,
use the
args_os
function instead.
§
Examples
use
std::env;
// Prints each argument on a separate line
for
argument
in
env::args() {
println!
(
"{argument}"
);
}