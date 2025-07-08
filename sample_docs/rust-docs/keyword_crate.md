crate - Rust
Keyword
crate
Copy item path
Source
Expand description
A Rust binary or library.
The primary use of the
crate
keyword is as a part of
extern crate
declarations, which are
used to specify a dependency on a crate external to the one it’s declared in. Crates are the
fundamental compilation unit of Rust code, and can be seen as libraries or projects. More can
be read about crates in the
Reference
.
ⓘ
extern crate
rand;
extern crate
my_crate
as
thing;
extern crate
std;
// implicitly added to the root of every Rust project
The
as
keyword can be used to change what the crate is referred to as in your project. If a
crate name includes a dash, it is implicitly imported with the dashes replaced by underscores.
crate
can also be used as in conjunction with
pub
to signify that the item it’s attached to
is public only to other members of the same crate it’s in.
pub
(
crate
)
use
std::io::Error
as
IoError;
pub
(
crate
)
enum
CoolMarkerType { }
pub struct
PublicThing {
pub
(
crate
) semi_secret_thing: bool,
}
crate
is also used to represent the absolute path of a module, where
crate
refers to the
root of the current crate. For instance,
crate::foo::bar
refers to the name
bar
inside the
module
foo
, from anywhere else in the same crate.