Hasher in std::hash - Rust
std
::
hash
Trait
Hasher
Copy item path
1.0.0
·
Source
pub trait Hasher {
Show 16 methods
// Required methods
    fn
finish
(&self) ->
u64
;
fn
write
(&mut self, bytes: &[
u8
]);

    // Provided methods
    fn
write_u8
(&mut self, i:
u8
) { ... }
fn
write_u16
(&mut self, i:
u16
) { ... }
fn
write_u32
(&mut self, i:
u32
) { ... }
fn
write_u64
(&mut self, i:
u64
) { ... }
fn
write_u128
(&mut self, i:
u128
) { ... }
fn
write_usize
(&mut self, i:
usize
) { ... }
fn
write_i8
(&mut self, i:
i8
) { ... }
fn
write_i16
(&mut self, i:
i16
) { ... }
fn
write_i32
(&mut self, i:
i32
) { ... }
fn
write_i64
(&mut self, i:
i64
) { ... }
fn
write_i128
(&mut self, i:
i128
) { ... }
fn
write_isize
(&mut self, i:
isize
) { ... }
fn
write_length_prefix
(&mut self, len:
usize
) { ... }
fn
write_str
(&mut self, s: &
str
) { ... }
}
Expand description
A trait for hashing an arbitrary stream of bytes.
Instances of
Hasher
usually represent state that is changed while hashing
data.
Hasher
provides a fairly basic interface for retrieving the generated hash
(with
finish
), and writing integers as well as slices of bytes into an
instance (with
write
and
write_u8
etc.). Most of the time,
Hasher
instances are used in conjunction with the
Hash
trait.
This trait provides no guarantees about how the various
write_*
methods are
defined and implementations of
Hash
should not assume that they work one
way or another. You cannot assume, for example, that a
write_u32
call is
equivalent to four calls of
write_u8
.  Nor can you assume that adjacent
write
calls are merged, so it’s possible, for example, that
hasher.write(
&
[
1
,
2
]);
hasher.write(
&
[
3
,
4
,
5
,
6
]);
and
hasher.write(
&
[
1
,
2
,
3
,
4
]);
hasher.write(
&
[
5
,
6
]);
end up producing different hashes.
Thus to produce the same hash value,
Hash
implementations must ensure
for equivalent items that exactly the same sequence of calls is made – the
same methods with the same parameters in the same order.
§
Examples
use
std::hash::{DefaultHasher, Hasher};
let
mut
hasher = DefaultHasher::new();

hasher.write_u32(
1989
);
hasher.write_u8(
11
);
hasher.write_u8(
9
);
hasher.write(
b"Huh?"
);
println!
(
"Hash is {:x}!"
, hasher.finish());
Required Methods
§
1.0.0
·
Source
fn
finish
(&self) ->
u64
Returns the hash value for the values written so far.
Despite its name, the method does not reset the hasher’s internal
state. Additional
write
s will continue from the current value.
If you need to start a fresh hash value, you will have to create
a new hasher.
§
Examples
use
std::hash::{DefaultHasher, Hasher};
let
mut
hasher = DefaultHasher::new();
hasher.write(
b"Cool!"
);
println!
(
"Hash is {:x}!"
, hasher.finish());
1.0.0
·
Source
fn
write
(&mut self, bytes: &[
u8
])
Writes some data into this
Hasher
.
§
Examples
use
std::hash::{DefaultHasher, Hasher};
let
mut
hasher = DefaultHasher::new();
let
data = [
0x01
,
0x23
,
0x45
,
0x67
,
0x89
,
0xab
,
0xcd
,
0xef
];

hasher.write(
&
data);
println!
(
"Hash is {:x}!"
, hasher.finish());
§
Note to Implementers
You generally should not do length-prefixing as part of implementing
this method.  It’s up to the
Hash
implementation to call
Hasher::write_length_prefix
before sequences that need it.
Provided Methods
§
1.3.0
·
Source
fn
write_u8
(&mut self, i:
u8
)
Writes a single
u8
into this hasher.
1.3.0
·
Source
fn
write_u16
(&mut self, i:
u16
)
Writes a single
u16
into this hasher.
1.3.0
·
Source
fn
write_u32
(&mut self, i:
u32
)
Writes a single
u32
into this hasher.
1.3.0
·
Source
fn
write_u64
(&mut self, i:
u64
)
Writes a single
u64
into this hasher.
1.26.0
·
Source
fn
write_u128
(&mut self, i:
u128
)
Writes a single
u128
into this hasher.
1.3.0
·
Source
fn
write_usize
(&mut self, i:
usize
)
Writes a single
usize
into this hasher.
1.3.0
·
Source
fn
write_i8
(&mut self, i:
i8
)
Writes a single
i8
into this hasher.
1.3.0
·
Source
fn
write_i16
(&mut self, i:
i16
)
Writes a single
i16
into this hasher.
1.3.0
·
Source
fn
write_i32
(&mut self, i:
i32
)
Writes a single
i32
into this hasher.
1.3.0
·
Source
fn
write_i64
(&mut self, i:
i64
)
Writes a single
i64
into this hasher.
1.26.0
·
Source
fn
write_i128
(&mut self, i:
i128
)
Writes a single
i128
into this hasher.
1.3.0
·
Source
fn
write_isize
(&mut self, i:
isize
)
Writes a single
isize
into this hasher.
Source
fn
write_length_prefix
(&mut self, len:
usize
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a length prefix into this hasher, as part of being prefix-free.
If you’re implementing
Hash
for a custom collection, call this before
writing its contents to this
Hasher
.  That way
(collection![1, 2, 3], collection![4, 5])
and
(collection![1, 2], collection![3, 4, 5])
will provide different
sequences of values to the
Hasher
The
impl<T> Hash for [T]
includes a call to this method, so if you’re
hashing a slice (or array or vector) via its
Hash::hash
method,
you should
not
call this yourself.
This method is only for providing domain separation.  If you want to
hash a
usize
that represents part of the
data
, then it’s important
that you pass it to
Hasher::write_usize
instead of to this method.
§
Examples
#![feature(hasher_prefixfree_extras)]
use
std::hash::{Hash, Hasher};
impl
<T: Hash> Hash
for
MyCollection<T> {
fn
hash<H: Hasher>(
&
self
, state:
&mut
H) {
        state.write_length_prefix(
self
.len());
for
elt
in
self
{
            elt.hash(state);
        }
    }
}
§
Note to Implementers
If you’ve decided that your
Hasher
is willing to be susceptible to
Hash-DoS attacks, then you might consider skipping hashing some or all
of the
len
provided in the name of increased performance.
Source
fn
write_str
(&mut self, s: &
str
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a single
str
into this hasher.
If you’re implementing
Hash
, you generally do not need to call this,
as the
impl Hash for str
does, so you should prefer that instead.
This includes the domain separator for prefix-freedom, so you should
not
call
Self::write_length_prefix
before calling this.
§
Note to Implementers
There are at least two reasonable default ways to implement this.
Which one will be the default is not yet decided, so for now
you probably want to override it specifically.
§
The general answer
It’s always correct to implement this with a length prefix:
fn
write_str(
&mut
self
, s:
&
str) {
self
.write_length_prefix(s.len());
self
.write(s.as_bytes());
}
And, if your
Hasher
works in
usize
chunks, this is likely a very
efficient way to do it, as anything more complicated may well end up
slower than just running the round with the length.
§
If your
Hasher
works byte-wise
One nice thing about
str
being UTF-8 is that the
b'\xFF'
byte
never happens.  That means that you can append that to the byte stream
being hashed and maintain prefix-freedom:
fn
write_str(
&mut
self
, s:
&
str) {
self
.write(s.as_bytes());
self
.write_u8(
0xff
);
}
This does require that your implementation not add extra padding, and
thus generally requires that you maintain a buffer, running a round
only once that buffer is full (or
finish
is called).
That’s because if
write
pads data out to a fixed chunk size, it’s
likely that it does it in such a way that
"a"
and
"a\x00"
would
end up hashing the same sequence of things, introducing conflicts.
Implementors
§
1.13.0
·
Source
§
impl
Hasher
for
DefaultHasher
1.0.0
·
Source
§
impl
Hasher
for
SipHasher
1.22.0
·
Source
§
impl<H>
Hasher
for
&mut H
where
    H:
Hasher
+ ?
Sized
,
1.22.0
·
Source
§
impl<T, A>
Hasher
for
Box
<T, A>
where
    T:
Hasher
+ ?
Sized
,
    A:
Allocator
,