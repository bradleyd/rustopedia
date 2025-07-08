chown in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Function
chown
Copy item path
1.73.0
·
Source
pub fn chown<P:
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
Change the owner and group of the specified path.
Specifying either the uid or gid as
None
will leave it unchanged.
Changing the owner typically requires privileges, such as root or a specific capability.
Changing the group typically requires either being the owner and a member of the group, or
having privileges.
Be aware that changing owner clears the
suid
and
sgid
permission bits in most cases
according to POSIX, usually even if the user is root. The sgid is not cleared when
the file is non-group-executable. See:
https://www.man7.org/linux/man-pages/man2/chown.2.html
This call may also clear file capabilities, if there was any.
If called on a symbolic link, this will change the owner and group of the link target. To
change the owner and group of the link itself, see
lchown
.
§
Examples
use
std::os::unix::fs;
fn
main() -> std::io::Result<()> {
    fs::chown(
"/sandbox"
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