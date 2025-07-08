lchown in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Function
lchown
Copy item path
1.73.0
·
Source
pub fn lchown<P:
AsRef
<
Path
>>(
    dir: P,
    uid:
Option
<
u32
>,
    gid:
Option
<
u32
>,
) ->
Result
<
()
>
Available on
Unix
only.
Expand description
Change the owner and group of the specified path, without dereferencing symbolic links.
Identical to
chown
, except that if called on a symbolic link, this will change the owner
and group of the link itself rather than the owner and group of the link target.
§
Examples
use
std::os::unix::fs;
fn
main() -> std::io::Result<()> {
    fs::lchown(
"/symlink"
,
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