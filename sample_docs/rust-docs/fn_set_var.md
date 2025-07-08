set_var in std::env - Rust
std
::
env
Function
set_var
Copy item path
1.0.0
·
Source
pub unsafe fn set_var<K:
AsRef
<
OsStr
>, V:
AsRef
<
OsStr
>>(key: K, value: V)
Expand description
Sets the environment variable
key
to the value
value
for the currently running
process.
§
Safety
This function is safe to call in a single-threaded program.
This function is also always safe to call on Windows, in single-threaded
and multi-threaded programs.
In multi-threaded programs on other operating systems, the only safe option is
to not use
set_var
or
remove_var
at all.
The exact requirement is: you
must ensure that there are no other threads concurrently writing or
reading
(!) the environment through functions or global variables other
than the ones in this module. The problem is that these operating systems
do not provide a thread-safe way to read the environment, and most C
libraries, including libc itself, do not advertise which functions read
from the environment. Even functions from the Rust standard library may
read the environment without going through this module, e.g. for DNS
lookups from
std::net::ToSocketAddrs
. No stable guarantee is made about
which functions may read from the environment in future versions of a
library. All this makes it not practically possible for you to guarantee
that no other thread will read the environment, so the only safe option is
to not use
set_var
or
remove_var
in multi-threaded programs at all.
Discussion of this unsafety on Unix may be found in:
Austin Group Bugzilla (for POSIX)
GNU C library Bugzilla
To pass an environment variable to a child process, you can instead use
Command::env
.
§
Panics
This function may panic if
key
is empty, contains an ASCII equals sign
'='
or the NUL character
'\0'
, or when
value
contains the NUL character.
§
Examples
use
std::env;
let
key =
"KEY"
;
unsafe
{
    env::set_var(key,
"VALUE"
);
}
assert_eq!
(env::var(key),
Ok
(
"VALUE"
.to_string()));