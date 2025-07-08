symlink_path in std::os::wasi::fs - Rust
std
::
os
::
wasi
::
fs
Function
symlink_path
Copy item path
Source
pub fn symlink_path<P:
AsRef
<
Path
>, U:
AsRef
<
Path
>>(
    old_path: P,
    new_path: U,
) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Available on
WASI
only.
Expand description
Creates a symbolic link.
This is a convenience API similar to
std::os::unix::fs::symlink
and
std::os::windows::fs::symlink_file
and
std::os::windows::fs::symlink_dir
.