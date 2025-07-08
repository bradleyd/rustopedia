BuildHasher in std::hash - Rust
std
::
hash
Trait
BuildHasher
Copy item path
1.7.0
·
Source
pub trait BuildHasher {
    type
Hasher
:
Hasher
;

    // Required method
    fn
build_hasher
(&self) -> Self::
Hasher
;

    // Provided method
    fn
hash_one
<T>(&self, x: T) ->
u64
where T:
Hash
,
             Self:
Sized
,
             Self::
Hasher
:
Hasher
{ ... }
}
Expand description
A trait for creating instances of
Hasher
.
A
BuildHasher
is typically used (e.g., by
HashMap
) to create
Hasher
s for each key such that they are hashed independently of one
another, since
Hasher
s contain state.
For each instance of
BuildHasher
, the
Hasher
s created by
build_hasher
should be identical. That is, if the same stream of bytes
is fed into each hasher, the same output will also be generated.
§
Examples
use
std::hash::{BuildHasher, Hasher, RandomState};
let
s = RandomState::new();
let
mut
hasher_1 = s.build_hasher();
let
mut
hasher_2 = s.build_hasher();

hasher_1.write_u32(
8128
);
hasher_2.write_u32(
8128
);
assert_eq!
(hasher_1.finish(), hasher_2.finish());
Required Associated Types
§
1.7.0
·
Source
type
Hasher
:
Hasher
Type of the hasher that will be created.
Required Methods
§
1.7.0
·
Source
fn
build_hasher
(&self) -> Self::
Hasher
Creates a new hasher.
Each call to
build_hasher
on the same instance should produce identical
Hasher
s.
§
Examples
use
std::hash::{BuildHasher, RandomState};
let
s = RandomState::new();
let
new_s = s.build_hasher();
Provided Methods
§
1.71.0
·
Source
fn
hash_one
<T>(&self, x: T) ->
u64
where
    T:
Hash
,
    Self:
Sized
,
    Self::
Hasher
:
Hasher
,
Calculates the hash of a single value.
This is intended as a convenience for code which
consumes
hashes, such
as the implementation of a hash table or in unit tests that check
whether a custom
Hash
implementation behaves as expected.
This must not be used in any code which
creates
hashes, such as in an
implementation of
Hash
.  The way to create a combined hash of
multiple values is to call
Hash::hash
multiple times using the same
Hasher
, not to call this method repeatedly and combine the results.
§
Example
use
std::cmp::{max, min};
use
std::hash::{BuildHasher, Hash, Hasher};
struct
OrderAmbivalentPair<T: Ord>(T, T);
impl
<T: Ord + Hash> Hash
for
OrderAmbivalentPair<T> {
fn
hash<H: Hasher>(
&
self
, hasher:
&mut
H) {
        min(
&
self
.
0
,
&
self
.
1
).hash(hasher);
        max(
&
self
.
0
,
&
self
.
1
).hash(hasher);
    }
}
// Then later, in a `#[test]` for the type...
let
bh = std::hash::RandomState::new();
assert_eq!
(
    bh.hash_one(OrderAmbivalentPair(
1
,
2
)),
    bh.hash_one(OrderAmbivalentPair(
2
,
1
))
);
assert_eq!
(
    bh.hash_one(OrderAmbivalentPair(
10
,
2
)),
    bh.hash_one(
&
OrderAmbivalentPair(
2
,
10
))
);
Implementors
§
1.7.0
·
Source
§
impl
BuildHasher
for
RandomState
Source
§
type
Hasher
=
DefaultHasher
1.7.0
·
Source
§
impl<H>
BuildHasher
for
BuildHasherDefault
<H>
where
    H:
Default
+
Hasher
,
Source
§
type
Hasher
= H