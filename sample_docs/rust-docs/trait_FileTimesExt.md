FileTimesExt in std::os::darwin::fs - Rust
std
::
os
::
darwin
::
fs
Trait
FileTimesExt
Copy item path
1.84.0
·
Source
pub trait FileTimesExt: Sealed {
    // Required method
    fn
set_created
(self, t:
SystemTime
) -> Self;
}
Available on
Apple
only.
Expand description
OS-specific extensions to
fs::FileTimes
.
Required Methods
§
1.75.0
·
Source
fn
set_created
(self, t:
SystemTime
) -> Self
Set the creation time of a file.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.75.0
·
Source
§
impl
FileTimesExt
for
FileTimes