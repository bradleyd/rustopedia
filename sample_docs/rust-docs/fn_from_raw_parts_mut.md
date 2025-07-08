from_raw_parts_mut in std::str - Rust
std
::
str
Function
from_raw_parts_mut
Copy item path
Source
pub const unsafe fn from_raw_parts_mut<'a>(
    ptr:
*mut
u8
,
    len:
usize
,
) -> &'a mut
str
🔬
This is a nightly-only experimental API. (
str_from_raw_parts
#119206
)
Expand description
Creates a
&mut str
from a pointer and a length.
The pointed-to bytes must be valid UTF-8.
If this might not be the case, use
str::from_utf8_mut(slice::from_raw_parts_mut(ptr, len))
,
which will return an
Err
if the data isn’t valid UTF-8.
This function is the
str
equivalent of
slice::from_raw_parts_mut
.
See that function’s documentation for safety concerns and examples.
The immutable version of this function is
from_raw_parts
.