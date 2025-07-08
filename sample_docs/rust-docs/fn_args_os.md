args_os in std::env - Rust
std
::
env
Function
args_os
Copy item path
1.0.0
·
Source
pub fn args_os() ->
ArgsOs
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
std::env::args_os
to work even in a
cdylib
or
staticlib
, as it
does on macOS and Windows.
Note that the returned iterator will not check if the arguments to the
process are valid Unicode. If you want to panic on invalid UTF-8,
use the
args
function instead.
§
Examples
use
std::env;
// Prints each argument on a separate line
for
argument
in
env::args_os() {
println!
(
"{argument:?}"
);
}