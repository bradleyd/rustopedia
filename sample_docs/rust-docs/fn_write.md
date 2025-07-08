write in std::fs - Rust
std
::
fs
Function
write
Copy item path
1.26.0
·
Source
pub fn write<P:
AsRef
<
Path
>, C:
AsRef
<[
u8
]>>(path: P, contents: C) ->
Result
<
()
>
Expand description
Writes a slice as the entire contents of a file.
This function will create a file if it does not exist,
and will entirely replace its contents if it does.
Depending on the platform, this function may fail if the
full directory path does not exist.
This is a convenience function for using
File::create
and
write_all
with fewer imports.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::write(
"foo.txt"
,
b"Lorem ipsum"
)
?
;
    fs::write(
"bar.txt"
,
"dolor sit"
)
?
;
Ok
(())
}