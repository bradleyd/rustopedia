write in std - Rust
std
Macro
write
Copy item path
1.0.0
·
Source
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => { ... };
}
Expand description
Writes formatted data into a buffer.
This macro accepts a ‘writer’, a format string, and a list of arguments. Arguments will be
formatted according to the specified format string and the result will be passed to the writer.
The writer may be any value with a
write_fmt
method; generally this comes from an
implementation of either the
fmt::Write
or the
io::Write
trait. The macro
returns whatever the
write_fmt
method returns; commonly a
fmt::Result
, or an
io::Result
.
See
std::fmt
for more information on the format string syntax.
§
Examples
use
std::io::Write;
fn
main() -> std::io::Result<()> {
let
mut
w = Vec::new();
write!
(
&mut
w,
"test"
)
?
;
write!
(
&mut
w,
"formatted {}"
,
"arguments"
)
?
;
assert_eq!
(w,
b"testformatted arguments"
);
Ok
(())
}
A module can import both
std::fmt::Write
and
std::io::Write
and call
write!
on objects
implementing either, as objects do not typically implement both. However, the module must
avoid conflict between the trait names, such as by importing them as
_
or otherwise renaming
them:
use
std::fmt::Write
as _
;
use
std::io::Write
as _
;
fn
main() ->
Result
<(), Box<
dyn
std::error::Error>> {
let
mut
s = String::new();
let
mut
v = Vec::new();
write!
(
&mut
s,
"{} {}"
,
"abc"
,
123
)
?
;
// uses fmt::Write::write_fmt
write!
(
&mut
v,
"s = {:?}"
, s)
?
;
// uses io::Write::write_fmt
assert_eq!
(v,
b"s = \"abc 123\""
);
Ok
(())
}
If you also need the trait names themselves, such as to implement one or both on your types,
import the containing module and then name them with a prefix:
use
std::fmt::{
self
, Write
as _
};
use
std::io::{
self
, Write
as _
};
struct
Example;
impl
fmt::Write
for
Example {
fn
write_str(
&mut
self
, _s:
&
str) -> core::fmt::Result {
unimplemented!
();
    }
}
Note: This macro can be used in
no_std
setups as well.
In a
no_std
setup you are responsible for the implementation details of the components.
use
core::fmt::Write;
struct
Example;
impl
Write
for
Example {
fn
write_str(
&mut
self
, _s:
&
str) -> core::fmt::Result {
unimplemented!
();
    }
}
let
mut
m = Example{};
write!
(
&mut
m,
"Hello World"
).expect(
"Not written"
);