request_ref in std::error - Rust
std
::
error
Function
request_ref
Copy item path
Source
pub fn request_ref<'a, T>(err: &'a (impl
Error
+ ?
Sized
)) ->
Option
<
&'a T
>
where
    T: 'static + ?
Sized
,
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Expand description
Requests a reference of type
T
from the given
impl Error
.
§
Examples
Get a string reference from an error.
#![feature(error_generic_member_access)]
use
core::error::Error;
use
core::error::request_ref;
fn
get_str(err:
&
impl
Error) ->
&
str {
    request_ref::<str>(err).unwrap()
}