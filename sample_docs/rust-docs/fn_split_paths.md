split_paths in std::env - Rust
std
::
env
Function
split_paths
Copy item path
1.0.0
·
Source
pub fn split_paths<T:
AsRef
<
OsStr
> + ?
Sized
>(unparsed:
&T
) ->
SplitPaths
<'_>
ⓘ
Expand description
Parses input according to platform conventions for the
PATH
environment variable.
Returns an iterator over the paths contained in
unparsed
. The iterator
element type is
PathBuf
.
On most Unix platforms, the separator is
:
and on Windows it is
;
. This
also performs unquoting on Windows.
join_paths
can be used to recombine elements.
§
Panics
This will panic on systems where there is no delimited
PATH
variable,
such as UEFI.
§
Examples
use
std::env;
let
key =
"PATH"
;
match
env::var_os(key) {
Some
(paths) => {
for
path
in
env::split_paths(
&
paths) {
println!
(
"'{}'"
, path.display());
        }
    }
None
=>
println!
(
"{key} is not defined in the environment."
)
}