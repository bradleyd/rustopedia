var in std::env - Rust
std
::
env
Function
var
Copy item path
1.0.0
·
Source
pub fn var<K:
AsRef
<
OsStr
>>(key: K) ->
Result
<
String
,
VarError
>
Expand description
Fetches the environment variable
key
from the current process.
§
Errors
Returns
VarError::NotPresent
if:
The variable is not set.
The variable’s name contains an equal sign or NUL (
'='
or
'\0'
).
Returns
VarError::NotUnicode
if the variable’s value is not valid
Unicode. If this is not desired, consider using
var_os
.
Use
env!
or
option_env!
instead if you want to check environment
variables at compile time.
§
Examples
use
std::env;
let
key =
"HOME"
;
match
env::var(key) {
Ok
(val) =>
println!
(
"{key}: {val:?}"
),
Err
(e) =>
println!
(
"couldn't interpret {key}: {e}"
),
}