ToOwned in std::borrow - Rust
std
::
borrow
Trait
ToOwned
Copy item path
1.0.0
·
Source
pub trait ToOwned {
    type
Owned
:
Borrow
<Self>;

    // Required method
    fn
to_owned
(&self) -> Self::
Owned
;

    // Provided method
    fn
clone_into
(&self, target: &mut Self::
Owned
) { ... }
}
Expand description
A generalization of
Clone
to borrowed data.
Some types make it possible to go from borrowed to owned, usually by
implementing the
Clone
trait. But
Clone
works only for going from
&T
to
T
. The
ToOwned
trait generalizes
Clone
to construct owned data
from any borrow of a given type.
Required Associated Types
§
1.0.0
·
Source
type
Owned
:
Borrow
<Self>
The resulting type after obtaining ownership.
Required Methods
§
1.0.0
·
Source
fn
to_owned
(&self) -> Self::
Owned
Creates owned data from borrowed data, usually by cloning.
§
Examples
Basic usage:
let
s:
&
str =
"a"
;
let
ss: String = s.to_owned();
let
v:
&
[i32] =
&
[
1
,
2
];
let
vv: Vec<i32> = v.to_owned();
Provided Methods
§
1.63.0
·
Source
fn
clone_into
(&self, target: &mut Self::
Owned
)
Uses borrowed data to replace owned data, usually by cloning.
This is borrow-generalized version of
Clone::clone_from
.
§
Examples
Basic usage:
let
mut
s: String = String::new();
"hello"
.clone_into(
&mut
s);
let
mut
v: Vec<i32> = Vec::new();
[
1
,
2
][..].clone_into(
&mut
v);
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.0.0
·
Source
§
impl
ToOwned
for
str
Source
§
type
Owned
=
String
Source
§
impl
ToOwned
for
ByteStr
Source
§
type
Owned
=
ByteString
1.3.0
·
Source
§
impl
ToOwned
for
CStr
Source
§
type
Owned
=
CString
1.0.0
·
Source
§
impl
ToOwned
for
OsStr
Source
§
type
Owned
=
OsString
1.0.0
·
Source
§
impl
ToOwned
for
Path
Source
§
type
Owned
=
PathBuf
1.0.0
·
Source
§
impl<T>
ToOwned
for
[T]
where
    T:
Clone
,
Source
§
type
Owned
=
Vec
<T>
1.0.0
·
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T