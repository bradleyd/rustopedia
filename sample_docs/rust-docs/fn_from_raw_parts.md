from_raw_parts in std::str - Rust
std
::
str
Function
from_raw_parts
Copy item path
Source
pub const unsafe fn from_raw_parts<'a>(ptr:
*const
u8
, len:
usize
) -> &'a
str
🔬
This is a nightly-only experimental API. (
str_from_raw_parts
#119206
)
Expand description
Creates a
&str
from a pointer and a length.
The pointed-to bytes must be valid UTF-8.
If this might not be the case, use
str::from_utf8(slice::from_raw_parts(ptr, len))
,
which will return an
Err
if the data isn’t valid UTF-8.
This function is the
str
equivalent of
slice::from_raw_parts
.
See that function’s documentation for safety concerns and examples.
The mutable version of this function is
from_raw_parts_mut
.