link in std::os::wasi::fs - Rust
std
::
os
::
wasi
::
fs
Function
link
Copy item path
Source
pub fn link<P:
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
    old_flags:
u32
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
Creates a hard link.
This corresponds to the
path_link
syscall.