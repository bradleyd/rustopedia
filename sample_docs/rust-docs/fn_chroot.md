chroot in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Function
chroot
Copy item path
1.56.0
·
Source
pub fn chroot<P:
AsRef
<
Path
>>(dir: P) ->
Result
<
()
>
Available on
Unix
only.
Expand description
Change the root directory of the current process to the specified path.
This typically requires privileges, such as root or a specific capability.
This does not change the current working directory; you should call
std::env::set_current_dir
afterwards.
§
Examples
use
std::os::unix::fs;
fn
main() -> std::io::Result<()> {
    fs::chroot(
"/sandbox"
)
?
;
    std::env::set_current_dir(
"/"
)
?
;
// continue working in sandbox
Ok
(())
}