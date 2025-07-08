set_permissions in std::fs - Rust
std
::
fs
Function
set_permissions
Copy item path
1.1.0
·
Source
pub fn set_permissions<P:
AsRef
<
Path
>>(path: P, perm:
Permissions
) ->
Result
<
()
>
Expand description
Changes the permissions found on a file or a directory.
§
Platform-specific behavior
This function currently corresponds to the
chmod
function on Unix
and the
SetFileAttributes
function on Windows.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
path
does not exist.
The user lacks the permission to change attributes of the file.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
mut
perms = fs::metadata(
"foo.txt"
)
?
.permissions();
    perms.set_readonly(
true
);
    fs::set_permissions(
"foo.txt"
, perms)
?
;
Ok
(())
}