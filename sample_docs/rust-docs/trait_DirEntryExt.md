DirEntryExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
DirEntryExt
Copy item path
1.1.0
·
Source
pub trait DirEntryExt {
    // Required method
    fn
ino
(&self) ->
u64
;
}
Available on
Unix
only.
Expand description
Unix-specific extension methods for
fs::DirEntry
.
Required Methods
§
1.1.0
·
Source
fn
ino
(&self) ->
u64
Returns the underlying
d_ino
field in the contained
dirent
structure.
§
Examples
use
std::fs;
use
std::os::unix::fs::DirEntryExt;
if let
Ok
(entries) = fs::read_dir(
"."
) {
for
entry
in
entries {
if let
Ok
(entry) = entry {
// Here, `entry` is a `DirEntry`.
println!
(
"{:?}: {}"
, entry.file_name(), entry.ino());
        }
    }
}
Implementors
§
1.1.0
·
Source
§
impl
DirEntryExt
for
DirEntry