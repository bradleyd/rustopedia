junction_point in std::os::windows::fs - Rust
std
::
os
::
windows
::
fs
Function
junction_point
Copy item path
Source
pub fn junction_point<P:
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
🔬
This is a nightly-only experimental API. (
junction_point
#121709
)
Available on
Windows
only.
Expand description
Creates a junction point.
The
link
path will be a directory junction pointing to the original path.
If
link
is a relative path then it will be made absolute prior to creating the junction point.
The
original
path must be a directory or a link to a directory, otherwise the junction point will be broken.
If either path is not a local file path then this will fail.