hard_link in std::fs - Rust
std
::
fs
Function
hard_link
Copy item path
1.0.0
·
Source
pub fn hard_link<P:
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
Expand description
Creates a new hard link on the filesystem.
The
link
path will be a link pointing to the
original
path. Note that
systems often require these two paths to both be located on the same
filesystem.
If
original
names a symbolic link, it is platform-specific whether the
symbolic link is followed. On platforms where it’s possible to not follow
it, it is not followed, and the created hard link points to the symbolic
link itself.
§
Platform-specific behavior
This function currently corresponds the
CreateHardLink
function on Windows.
On most Unix systems, it corresponds to the
linkat
function with no flags.
On Android, VxWorks, and Redox, it instead corresponds to the
link
function.
On MacOS, it uses the
linkat
function if it is available, but on very old
systems where
linkat
is not available,
link
is selected at runtime instead.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
The
original
path is not a file or doesn’t exist.
The ‘link’ path already exists.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::hard_link(
"a.txt"
,
"b.txt"
)
?
;
// Hard link a.txt to b.txt
Ok
(())
}