request_value in std::error - Rust
std
::
error
Function
request_value
Copy item path
Source
pub fn request_value<'a, T>(err: &'a (impl
Error
+ ?
Sized
)) ->
Option
<T>
where
    T: 'static,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Expand description
Requests a value of type
T
from the given
impl Error
.
§
Examples
Get a string value from an error.
#![feature(error_generic_member_access)]
use
std::error::Error;
use
core::error::request_value;
fn
get_string(err:
&
impl
Error) -> String {
    request_value::<String>(err).unwrap()
}