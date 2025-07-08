symlink in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Function
symlink
Copy item path
1.1.0
·
Source
pub fn symlink<P:
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
Available on
Unix
only.
Expand description
Creates a new symbolic link on the filesystem.
The
link
path will be a symbolic link pointing to the
original
path.
§
Examples
use
std::os::unix::fs;
fn
main() -> std::io::Result<()> {
    fs::symlink(
"a.txt"
,
"b.txt"
)
?
;
Ok
(())
}