read in std::fs - Rust
std
::
fs
Function
read
Copy item path
1.26.0
·
Source
pub fn read<P:
AsRef
<
Path
>>(path: P) ->
Result
<
Vec
<
u8
>>
Expand description
Reads the entire contents of a file into a bytes vector.
This is a convenience function for using
File::open
and
read_to_end
with fewer imports and without an intermediate variable.
§
Errors
This function will return an error if
path
does not already exist.
Other errors may also be returned according to
OpenOptions::open
.
While reading from the file, this function handles
io::ErrorKind::Interrupted
with automatic retries. See
io::Read
documentation for details.
§
Examples
use
std::fs;
fn
main() ->
Result
<(), Box<
dyn
std::error::Error +
'static
>> {
let
data: Vec<u8> = fs::read(
"image.jpg"
)
?
;
assert_eq!
(data[
0
..
3
], [
0xFF
,
0xD8
,
0xFF
]);
Ok
(())
}