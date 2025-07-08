rename in std::os::wasi::fs - Rust
std
::
os
::
wasi
::
fs
Function
rename
Copy item path
Source
pub fn rename<P:
AsRef
<
Path
>, U:
AsRef
<
Path
>>(
    old_fd: &
File
,
    old_path: P,
    new_fd: &
File
,
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
Renames a file or directory.
This corresponds to the
path_rename
syscall.