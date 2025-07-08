hash in std::ptr - Rust
std
::
ptr
Function
hash
Copy item path
1.35.0
·
Source
pub fn hash<T, S>(hashee:
*const T
, into:
&mut S
)
where
    S:
Hasher
,
    T: ?
Sized
,
Expand description
Hash a raw pointer.
This can be used to hash a
&T
reference (which coerces to
*const T
implicitly)
by its address rather than the value it points to
(which is what the
Hash for &T
implementation does).
§
Examples
use
std::hash::{DefaultHasher, Hash, Hasher};
use
std::ptr;
let
five =
5
;
let
five_ref =
&
five;
let
mut
hasher = DefaultHasher::new();
ptr::hash(five_ref,
&mut
hasher);
let
actual = hasher.finish();
let
mut
hasher = DefaultHasher::new();
(five_ref
as
*const
i32).hash(
&mut
hasher);
let
expected = hasher.finish();
assert_eq!
(actual, expected);