remove_dir_all in std::fs - Rust
std
::
fs
Function
remove_dir_all
Copy item path
1.0.0
·
Source
pub fn remove_dir_all<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Removes a directory at this path, after removing all its contents. Use
carefully!
This function does
not
follow symbolic links and it will simply remove the
symbolic link itself.
§
Platform-specific behavior
This function currently corresponds to
openat
,
fdopendir
,
unlinkat
and
lstat
functions
on Unix (except for REDOX) and the
CreateFileW
,
GetFileInformationByHandleEx
,
SetFileInformationByHandle
, and
NtCreateFile
functions on Windows. Note that, this
may change in the future
.
On REDOX, as well as when running in Miri for any target, this function is not protected against
time-of-check to time-of-use (TOCTOU) race conditions, and should not be used in
security-sensitive code on those platforms. All other platforms are protected.
§
Errors
See
fs::remove_file
and
fs::remove_dir
.
remove_dir_all
will fail if
remove_dir
or
remove_file
fail on
any
constituent
paths,
including
the root
path
. Consequently,
The directory you are deleting
must
exist, meaning that this function is
not idempotent
.
remove_dir_all
will fail if the
path
is
not
a directory.
Consider ignoring the error if validating the removal is not required for your use case.
io::ErrorKind::NotFound
is only returned if no removal occurs.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::remove_dir_all(
"/some/dir"
)
?
;
Ok
(())
}