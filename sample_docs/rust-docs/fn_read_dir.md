read_dir in std::fs - Rust
std
::
fs
Function
read_dir
Copy item path
1.0.0
·
Source
pub fn read_dir<P:
AsRef
<
Path
>>(path: P) ->
Result
<
ReadDir
>
Expand description
Returns an iterator over the entries within a directory.
The iterator will yield instances of
io::Result
<
DirEntry
>
.
New errors may be encountered after an iterator is initially constructed.
Entries for the current and parent directories (typically
.
and
..
) are
skipped.
§
Platform-specific behavior
This function currently corresponds to the
opendir
function on Unix
and the
FindFirstFileEx
function on Windows. Advancing the iterator
currently corresponds to
readdir
on Unix and
FindNextFile
on Windows.
Note that, this
may change in the future
.
The order in which this iterator returns entries is platform and filesystem
dependent.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
The provided
path
doesn’t exist.
The process lacks permissions to view the contents.
The
path
points at a non-directory file.
§
Examples
use
std::io;
use
std::fs::{
self
, DirEntry};
use
std::path::Path;
// one possible implementation of walking a directory only visiting files
fn
visit_dirs(dir:
&
Path, cb:
&
dyn
Fn(
&
DirEntry)) -> io::Result<()> {
if
dir.is_dir() {
for
entry
in
fs::read_dir(dir)
?
{
let
entry = entry
?
;
let
path = entry.path();
if
path.is_dir() {
                visit_dirs(
&
path, cb)
?
;
            }
else
{
                cb(
&
entry);
            }
        }
    }
Ok
(())
}
use
std::{fs, io};
fn
main() -> io::Result<()> {
let
mut
entries = fs::read_dir(
"."
)
?
.map(|res| res.map(|e| e.path()))
        .collect::<
Result
<Vec<
_
>, io::Error>>()
?
;
// The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
entries.sort();
// The entries have now been sorted by their path.
Ok
(())
}