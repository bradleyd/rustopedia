ExitCodeExt in std::os::windows::process - Rust
std
::
os
::
windows
::
process
Trait
ExitCodeExt
Copy item path
Source
pub trait ExitCodeExt: Sealed {
    // Required method
    fn
from_raw
(raw:
u32
) -> Self;
}
🔬
This is a nightly-only experimental API. (
windows_process_exit_code_from
#111688
)
Available on
Windows
only.
Expand description
Windows-specific extensions to
process::ExitCode
.
This trait is sealed: it cannot be implemented outside the standard library.
This is so that future additional methods are not breaking changes.
Required Methods
§
Source
fn
from_raw
(raw:
u32
) -> Self
🔬
This is a nightly-only experimental API. (
windows_process_exit_code_from
#111688
)
Creates a new
ExitCode
from the raw underlying
u32
return value of
a process.
The exit code should not be 259, as this conflicts with the
STILL_ACTIVE
macro returned from the
GetExitCodeProcess
function to signal that the
process has yet to run to completion.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
ExitCodeExt
for
ExitCode