DirEntryExt2 in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
DirEntryExt2
Copy item path
Source
pub trait DirEntryExt2: Sealed {
    // Required method
    fn
file_name_ref
(&self) -> &
OsStr
;
}
🔬
This is a nightly-only experimental API. (
dir_entry_ext2
#85573
)
Available on
Unix
only.
Expand description
Sealed Unix-specific extension methods for
fs::DirEntry
.
Required Methods
§
Source
fn
file_name_ref
(&self) -> &
OsStr
🔬
This is a nightly-only experimental API. (
dir_entry_ext2
#85573
)
Returns a reference to the underlying
OsStr
of this entry’s filename.
§
Examples
#![feature(dir_entry_ext2)]
use
std::os::unix::fs::DirEntryExt2;
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
.collect::<
Result
<Vec<
_
>, io::Error>>()
?
;
    entries.sort_unstable_by(|a, b| a.file_name_ref().cmp(b.file_name_ref()));
for
p
in
entries {
println!
(
"{p:?}"
);
    }
Ok
(())
}
Implementors
§
Source
§
impl
DirEntryExt2
for
DirEntry