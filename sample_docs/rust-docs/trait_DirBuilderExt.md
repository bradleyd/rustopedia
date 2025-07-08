DirBuilderExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
DirBuilderExt
Copy item path
1.6.0
·
Source
pub trait DirBuilderExt {
    // Required method
    fn
mode
(&mut self, mode:
u32
) -> &mut Self;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
fs::DirBuilder
.
Required Methods
§
1.6.0
·
Source
fn
mode
(&mut self, mode:
u32
) -> &mut Self
Sets the mode to create new directories with. This option defaults to
0o777.
§
Examples
use
std::fs::DirBuilder;
use
std::os::unix::fs::DirBuilderExt;
let
mut
builder = DirBuilder::new();
builder.mode(
0o755
);
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.6.0
·
Source
§
impl
DirBuilderExt
for
DirBuilder