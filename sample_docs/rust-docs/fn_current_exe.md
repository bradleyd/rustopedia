current_exe in std::env - Rust
std
::
env
Function
current_exe
Copy item path
1.0.0
·
Source
pub fn current_exe() ->
Result
<
PathBuf
>
Expand description
Returns the full filesystem path of the current running executable.
§
Platform-specific behavior
If the executable was invoked through a symbolic link, some platforms will
return the path of the symbolic link and other platforms will return the
path of the symbolic link’s target.
If the executable is renamed while it is running, platforms may return the
path at the time it was loaded instead of the new path.
§
Errors
Acquiring the path of the current executable is a platform-specific operation
that can fail for a good number of reasons. Some errors can include, but not
be limited to, filesystem operations failing or general syscall failures.
§
Security
The output of this function should not be trusted for anything
that might have security implications. Basically, if users can run
the executable, they can change the output arbitrarily.
As an example, you can easily introduce a race condition. It goes
like this:
You get the path to the current executable using
current_exe()
, and
store it in a variable.
Time passes. A malicious actor removes the current executable, and
replaces it with a malicious one.
You then use the stored path to re-execute the current
executable.
You expected to safely execute the current executable, but you’re
instead executing something completely different. The code you
just executed run with your privileges.
This sort of behavior has been known to
lead to privilege escalation
when
used incorrectly.
§
Examples
use
std::env;
match
env::current_exe() {
Ok
(exe_path) =>
println!
(
"Path of this executable is: {}"
,
                             exe_path.display()),
Err
(e) =>
println!
(
"failed to get current exe path: {e}"
),
};