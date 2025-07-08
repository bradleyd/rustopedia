absolute in std::path - Rust
std
::
path
Function
absolute
Copy item path
1.79.0
·
Source
pub fn absolute<P:
AsRef
<
Path
>>(path: P) ->
Result
<
PathBuf
>
Expand description
Makes the path absolute without accessing the filesystem.
If the path is relative, the current directory is used as the base directory.
All intermediate components will be resolved according to platform-specific
rules, but unlike
canonicalize
, this does not
resolve symlinks and may succeed even if the path does not exist.
If the
path
is empty or getting the
current directory
fails, then an error will be
returned.
§
Platform-specific behavior
On POSIX platforms, the path is resolved using
POSIX semantics
,
except that it stops short of resolving symlinks. This means it will keep
..
components and trailing slashes.
On Windows, for verbatim paths, this will simply return the path as given. For other
paths, this is currently equivalent to calling
GetFullPathNameW
.
Note that these
may change in the future
.
§
Errors
This function may return an error in the following situations:
If
path
is syntactically invalid; in particular, if it is empty.
If getting the
current directory
fails.
§
Examples
§
POSIX paths
fn
main() -> std::io::Result<()> {
use
std::path::{
self
, Path};
// Relative to absolute
let
absolute = path::absolute(
"foo/./bar"
)
?
;
assert!
(absolute.ends_with(
"foo/bar"
));
// Absolute to absolute
let
absolute = path::absolute(
"/foo//test/.././bar.rs"
)
?
;
assert_eq!
(absolute, Path::new(
"/foo/test/../bar.rs"
));
Ok
(())
}
§
Windows paths
fn
main() -> std::io::Result<()> {
use
std::path::{
self
, Path};
// Relative to absolute
let
absolute = path::absolute(
"foo/./bar"
)
?
;
assert!
(absolute.ends_with(
r"foo\bar"
));
// Absolute to absolute
let
absolute = path::absolute(
r"C:\foo//test\..\./bar.rs"
)
?
;
assert_eq!
(absolute, Path::new(
r"C:\foo\bar.rs"
));
Ok
(())
}
Note that this
may change in the future
.