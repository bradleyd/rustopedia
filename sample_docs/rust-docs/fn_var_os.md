var_os in std::env - Rust
std
::
env
Function
var_os
Copy item path
1.0.0
·
Source
pub fn var_os<K:
AsRef
<
OsStr
>>(key: K) ->
Option
<
OsString
>
Expand description
Fetches the environment variable
key
from the current process, returning
None
if the variable isn’t set or if there is another error.
It may return
None
if the environment variable’s name contains
the equal sign character (
=
) or the NUL character.
Note that this function will not check if the environment variable
is valid Unicode. If you want to have an error on invalid UTF-8,
use the
var
function instead.
§
Examples
use
std::env;
let
key =
"HOME"
;
match
env::var_os(key) {
Some
(val) =>
println!
(
"{key}: {val:?}"
),
None
=>
println!
(
"{key} is not defined in the environment."
)
}
If expecting a delimited variable (such as
PATH
),
split_paths
can be used to separate items.