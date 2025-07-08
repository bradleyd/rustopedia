type_name_of_val in std::any - Rust
std
::
any
Function
type_name_of_val
Copy item path
1.76.0 (const:
unstable
)
·
Source
pub fn type_name_of_val<T>(_val:
&T
) -> &'static
str
where
    T: ?
Sized
,
Expand description
Returns the type name of the pointed-to value as a string slice.
This is the same as
type_name::<T>()
, but can be used where the type of a
variable is not easily available.
§
Note
Like
type_name
, this is intended for diagnostic use and the exact output is not
guaranteed. It provides a best-effort description, but the output may change between
versions of the compiler.
In short: use this for debugging, avoid using the output to affect program behavior. More
information is available at
type_name
.
Additionally, this function does not resolve trait objects. This means that
type_name_of_val(&7u32 as &dyn Debug)
may return
"dyn Debug"
, but will not return
"u32"
at this time.
§
Examples
Prints the default integer and float types.
use
std::any::type_name_of_val;
let
s =
"foo"
;
let
x: i32 =
1
;
let
y: f32 =
1.0
;
assert!
(type_name_of_val(
&
s).contains(
"str"
));
assert!
(type_name_of_val(
&
x).contains(
"i32"
));
assert!
(type_name_of_val(
&
y).contains(
"f32"
));