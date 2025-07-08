FileExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
FileExt
Copy item path
1.15.0
·
Source
pub trait FileExt {
    // Required methods
    fn
read_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
usize
>;
fn
write_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
usize
>;

    // Provided methods
    fn
read_vectored_at
(
        &self,
        bufs: &mut [
IoSliceMut
<'_>],
        offset:
u64
,
    ) ->
Result
<
usize
> { ... }
fn
read_exact_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
()
> { ... }
fn
write_vectored_at
(
        &self,
        bufs: &[
IoSlice
<'_>],
        offset:
u64
,
    ) ->
Result
<
usize
> { ... }
fn
write_all_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
()
> { ... }
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
fs::File
.
Required Methods
§
1.15.0
·
Source
fn
read_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
usize
>
Reads a number of bytes starting from a given offset.
Returns the number of bytes read.
The offset is relative to the start of the file and thus independent
from the current cursor.
The current file cursor is not affected by this function.
Note that similar to
File::read
, it is not an error to return with a
short read.
§
Examples
use
std::io;
use
std::fs::File;
use
std::os::unix::prelude::FileExt;
fn
main() -> io::Result<()> {
let
mut
buf = [
0u8
;
8
];
let
file = File::open(
"foo.txt"
)
?
;
// We now read 8 bytes from the offset 10.
let
num_bytes_read = file.read_at(
&mut
buf,
10
)
?
;
println!
(
"read {num_bytes_read} bytes: {buf:?}"
);
Ok
(())
}
1.15.0
·
Source
fn
write_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
usize
>
Writes a number of bytes starting from a given offset.
Returns the number of bytes written.
The offset is relative to the start of the file and thus independent
from the current cursor.
The current file cursor is not affected by this function.
When writing beyond the end of the file, the file is appropriately
extended and the intermediate bytes are initialized with the value 0.
Note that similar to
File::write
, it is not an error to return a
short write.
§
Bug
On some systems,
write_at
utilises
pwrite64
to write to files.
However, this syscall has a
bug
where files opened with the
O_APPEND
flag fail to respect the offset parameter, always appending to the end
of the file instead.
It is possible to inadvertently set this flag, like in the example below.
Therefore, it is important to be vigilant while changing options to mitigate
unexpected behavior.
use
std::fs::File;
use
std::io;
use
std::os::unix::prelude::FileExt;
fn
main() -> io::Result<()> {
// Open a file with the append option (sets the `O_APPEND` flag)
let
file = File::options().append(
true
).open(
"foo.txt"
)
?
;
// We attempt to write at offset 10; instead appended to EOF
file.write_at(
b"sushi"
,
10
)
?
;
// foo.txt is 5 bytes long instead of 15
Ok
(())
}
§
Examples
use
std::fs::File;
use
std::io;
use
std::os::unix::prelude::FileExt;
fn
main() -> io::Result<()> {
let
file = File::create(
"foo.txt"
)
?
;
// We now write at the offset 10.
file.write_at(
b"sushi"
,
10
)
?
;
Ok
(())
}
Provided Methods
§
Source
fn
read_vectored_at
(
    &self,
    bufs: &mut [
IoSliceMut
<'_>],
    offset:
u64
,
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
unix_file_vectored_at
#89517
)
Like
read_at
, except that it reads into a slice of buffers.
Data is copied to fill each buffer in order, with the final buffer
written to possibly being only partially filled. This method must behave
equivalently to a single call to read with concatenated buffers.
1.33.0
·
Source
fn
read_exact_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
from the given offset.
The offset is relative to the start of the file and thus independent
from the current cursor.
The current file cursor is not affected by this function.
Similar to
io::Read::read_exact
but uses
read_at
instead of
read
.
§
Errors
If this function encounters an error of the kind
io::ErrorKind::Interrupted
then the error is ignored and the operation
will continue.
If this function encounters an “end of file” before completely filling
the buffer, it returns an error of the kind
io::ErrorKind::UnexpectedEof
.
The contents of
buf
are unspecified in this case.
If any other read error is encountered then this function immediately
returns. The contents of
buf
are unspecified in this case.
If this function returns an error, it is unspecified how many bytes it
has read, but it will never read more than would be necessary to
completely fill the buffer.
§
Examples
use
std::io;
use
std::fs::File;
use
std::os::unix::prelude::FileExt;
fn
main() -> io::Result<()> {
let
mut
buf = [
0u8
;
8
];
let
file = File::open(
"foo.txt"
)
?
;
// We now read exactly 8 bytes from the offset 10.
file.read_exact_at(
&mut
buf,
10
)
?
;
println!
(
"read {} bytes: {:?}"
, buf.len(), buf);
Ok
(())
}
Source
fn
write_vectored_at
(&self, bufs: &[
IoSlice
<'_>], offset:
u64
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
unix_file_vectored_at
#89517
)
Like
write_at
, except that it writes from a slice of buffers.
Data is copied from each buffer in order, with the final buffer read
from possibly being only partially consumed. This method must behave as
a call to
write_at
with the buffers concatenated would.
1.33.0
·
Source
fn
write_all_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
()
>
Attempts to write an entire buffer starting from a given offset.
The offset is relative to the start of the file and thus independent
from the current cursor.
The current file cursor is not affected by this function.
This method will continuously call
write_at
until there is no more data
to be written or an error of non-
io::ErrorKind::Interrupted
kind is
returned. This method will not return until the entire buffer has been
successfully written or such an error occurs. The first error that is
not of
io::ErrorKind::Interrupted
kind generated from this method will be
returned.
§
Errors
This function will return the first error of
non-
io::ErrorKind::Interrupted
kind that
write_at
returns.
§
Examples
use
std::fs::File;
use
std::io;
use
std::os::unix::prelude::FileExt;
fn
main() -> io::Result<()> {
let
file = File::open(
"foo.txt"
)
?
;
// We now write at the offset 10.
file.write_all_at(
b"sushi"
,
10
)
?
;
Ok
(())
}
Implementors
§
1.15.0
·
Source
§
impl
FileExt
for
File