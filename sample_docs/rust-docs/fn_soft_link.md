soft_link in std::fs - Rust
std
::
fs
Function
soft_link
Copy item path
1.0.0
·
Source
pub fn soft_link<P:
AsRef
<
Path
>, Q:
AsRef
<
Path
>>(
    original: P,
    link: Q,
) ->
Result
<
()
>
👎
Deprecated since 1.1.0: replaced with std::os::unix::fs::symlink and std::os::windows::fs::{symlink_file, symlink_dir}
Expand description
Creates a new symbolic link on the filesystem.
The
link
path will be a symbolic link pointing to the
original
path.
On Windows, this will be a file symlink, not a directory symlink;
for this reason, the platform-specific
std::os::unix::fs::symlink
and
std::os::windows::fs::symlink_file
or
symlink_dir
should be
used instead to make the intent explicit.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::soft_link(
"a.txt"
,
"b.txt"
)
?
;
Ok
(())
}