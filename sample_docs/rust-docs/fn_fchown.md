fchown in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Function
fchown
Copy item path
1.73.0
·
Source
pub fn fchown<F:
AsFd
>(fd: F, uid:
Option
<
u32
>, gid:
Option
<
u32
>) ->
Result
<
()
>
Available on
Unix
only.
Expand description
Change the owner and group of the file referenced by the specified open file descriptor.
For semantics and required privileges, see
chown
.
§
Examples
use
std::os::unix::fs;
fn
main() -> std::io::Result<()> {
let
f = std::fs::File::open(
"/file"
)
?
;
    fs::fchown(
&
f,
Some
(
0
),
Some
(
0
))
?
;
Ok
(())
}