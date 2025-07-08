FileTypeExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
FileTypeExt
Copy item path
1.5.0
·
Source
pub trait FileTypeExt {
    // Required methods
    fn
is_block_device
(&self) ->
bool
;
fn
is_char_device
(&self) ->
bool
;
fn
is_fifo
(&self) ->
bool
;
fn
is_socket
(&self) ->
bool
;
}
Available on
Unix
only.
Expand description
Unix-specific extensions for
fs::FileType
.
Adds support for special Unix file types such as block/character devices,
pipes, and sockets.
Required Methods
§
1.5.0
·
Source
fn
is_block_device
(&self) ->
bool
Returns
true
if this file type is a block device.
§
Examples
use
std::fs;
use
std::os::unix::fs::FileTypeExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"block_device_file"
)
?
;
let
file_type = meta.file_type();
assert!
(file_type.is_block_device());
Ok
(())
}
1.5.0
·
Source
fn
is_char_device
(&self) ->
bool
Returns
true
if this file type is a char device.
§
Examples
use
std::fs;
use
std::os::unix::fs::FileTypeExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"char_device_file"
)
?
;
let
file_type = meta.file_type();
assert!
(file_type.is_char_device());
Ok
(())
}
1.5.0
·
Source
fn
is_fifo
(&self) ->
bool
Returns
true
if this file type is a fifo.
§
Examples
use
std::fs;
use
std::os::unix::fs::FileTypeExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"fifo_file"
)
?
;
let
file_type = meta.file_type();
assert!
(file_type.is_fifo());
Ok
(())
}
1.5.0
·
Source
fn
is_socket
(&self) ->
bool
Returns
true
if this file type is a socket.
§
Examples
use
std::fs;
use
std::os::unix::fs::FileTypeExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"unix.socket"
)
?
;
let
file_type = meta.file_type();
assert!
(file_type.is_socket());
Ok
(())
}
Implementors
§
1.5.0
·
Source
§
impl
FileTypeExt
for
FileType