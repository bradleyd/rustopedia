Hash in std::hash - Rust
std
::
hash
Trait
Hash
Copy item path
1.0.0
·
Source
pub trait Hash {
    // Required method
    fn
hash
<H>(&self, state:
&mut H
)
where H:
Hasher
;

    // Provided method
    fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where H:
Hasher
,
             Self:
Sized
{ ... }
}
Expand description
A hashable type.
Types implementing
Hash
are able to be
hash
ed with an instance of
Hasher
.
§
Implementing
Hash
You can derive
Hash
with
#[derive(Hash)]
if all fields implement
Hash
.
The resulting hash will be the combination of the values from calling
hash
on each field.
#[derive(Hash)]
struct
Rustacean {
    name: String,
    country: String,
}
If you need more control over how a value is hashed, you can of course
implement the
Hash
trait yourself:
use
std::hash::{Hash, Hasher};
struct
Person {
    id: u32,
    name: String,
    phone: u64,
}
impl
Hash
for
Person {
fn
hash<H: Hasher>(
&
self
, state:
&mut
H) {
self
.id.hash(state);
self
.phone.hash(state);
    }
}
§
Hash
and
Eq
When implementing both
Hash
and
Eq
, it is important that the following
property holds:
k1 == k2 -> hash(k1) == hash(k2)
In other words, if two keys are equal, their hashes must also be equal.
HashMap
and
HashSet
both rely on this behavior.
Thankfully, you won’t need to worry about upholding this property when
deriving both
Eq
and
Hash
with
#[derive(PartialEq, Eq, Hash)]
.
Violating this property is a logic error. The behavior resulting from a logic error is not
specified, but users of the trait must ensure that such logic errors do
not
result in
undefined behavior. This means that
unsafe
code
must not
rely on the correctness of these
methods.
§
Prefix collisions
Implementations of
hash
should ensure that the data they
pass to the
Hasher
are prefix-free. That is,
values which are not equal should cause two different sequences of values to be written,
and neither of the two sequences should be a prefix of the other.
For example, the standard implementation of
Hash
for
&str
passes an extra
0xFF
byte to the
Hasher
so that the values
("ab", "c")
and
("a", "bc")
hash differently.
§
Portability
Due to differences in endianness and type sizes, data fed by
Hash
to a
Hasher
should not be considered portable across platforms. Additionally the data passed by most
standard library types should not be considered stable between compiler versions.
This means tests shouldn’t probe hard-coded hash values or data fed to a
Hasher
and
instead should check consistency with
Eq
.
Serialization formats intended to be portable between platforms or compiler versions should
either avoid encoding hashes or only rely on
Hash
and
Hasher
implementations that
provide additional guarantees.
Required Methods
§
1.0.0
·
Source
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
§
Examples
use
std::hash::{DefaultHasher, Hash, Hasher};
let
mut
hasher = DefaultHasher::new();
7920
.hash(
&mut
hasher);
println!
(
"Hash is {:x}!"
, hasher.finish());
Provided Methods
§
1.3.0
·
Source
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
This method is meant as a convenience, but its implementation is
also explicitly left unspecified. It isn’t guaranteed to be
equivalent to repeated calls of
hash
and implementations of
Hash
should keep that in mind and call
hash
themselves
if the slice isn’t treated as a whole unit in the
PartialEq
implementation.
For example, a
VecDeque
implementation might naïvely call
as_slices
and then
hash_slice
on each slice, but this
is wrong since the two slices can change with a call to
make_contiguous
without affecting the
PartialEq
result. Since these slices aren’t treated as singular
units, and instead part of a larger deque, this method cannot
be used.
§
Examples
use
std::hash::{DefaultHasher, Hash, Hasher};
let
mut
hasher = DefaultHasher::new();
let
numbers = [
6
,
28
,
496
,
8128
];
Hash::hash_slice(
&
numbers,
&mut
hasher);
println!
(
"Hash is {:x}!"
, hasher.finish());
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
Hash
for
AsciiChar
1.0.0
·
Source
§
impl
Hash
for std::cmp::
Ordering
1.44.0
·
Source
§
impl
Hash
for
Infallible
1.0.0
·
Source
§
impl
Hash
for
ErrorKind
1.7.0
·
Source
§
impl
Hash
for
IpAddr
Source
§
impl
Hash
for
Ipv6MulticastScope
1.0.0
·
Source
§
impl
Hash
for
SocketAddr
1.0.0
·
Source
§
impl
Hash
for std::sync::atomic::
Ordering
1.0.0
·
Source
§
impl
Hash
for
bool
1.0.0
·
Source
§
impl
Hash
for
char
1.0.0
·
Source
§
impl
Hash
for
i8
1.0.0
·
Source
§
impl
Hash
for
i16
1.0.0
·
Source
§
impl
Hash
for
i32
1.0.0
·
Source
§
impl
Hash
for
i64
1.0.0
·
Source
§
impl
Hash
for
i128
1.0.0
·
Source
§
impl
Hash
for
isize
1.29.0
·
Source
§
impl
Hash
for
!
1.0.0
·
Source
§
impl
Hash
for
str
1.0.0
·
Source
§
impl
Hash
for
u8
1.0.0
·
Source
§
impl
Hash
for
u16
1.0.0
·
Source
§
impl
Hash
for
u32
1.0.0
·
Source
§
impl
Hash
for
u64
1.0.0
·
Source
§
impl
Hash
for
u128
1.0.0
·
Source
§
impl
Hash
for
()
1.0.0
·
Source
§
impl
Hash
for
usize
1.28.0
·
Source
§
impl
Hash
for
Layout
1.0.0
·
Source
§
impl
Hash
for
TypeId
Source
§
impl
Hash
for
ByteStr
Source
§
impl
Hash
for
ByteString
1.64.0
·
Source
§
impl
Hash
for
CStr
1.64.0
·
Source
§
impl
Hash
for
CString
1.0.0
·
Source
§
impl
Hash
for
OsStr
1.0.0
·
Source
§
impl
Hash
for
OsString
1.0.0
·
Source
§
impl
Hash
for
Error
1.1.0
·
Source
§
impl
Hash
for
FileType
1.33.0
·
Source
§
impl
Hash
for
PhantomPinned
1.0.0
·
Source
§
impl
Hash
for
Ipv4Addr
1.0.0
·
Source
§
impl
Hash
for
Ipv6Addr
1.0.0
·
Source
§
impl
Hash
for
SocketAddrV4
1.0.0
·
Source
§
impl
Hash
for
SocketAddrV6
1.0.0
·
Source
§
impl
Hash
for
RangeFull
Source
§
impl
Hash
for
UCred
Available on
Unix
only.
1.0.0
·
Source
§
impl
Hash
for
Path
1.0.0
·
Source
§
impl
Hash
for
PathBuf
1.0.0
·
Source
§
impl
Hash
for
PrefixComponent
<'_>
Source
§
impl
Hash
for
Alignment
1.0.0
·
Source
§
impl
Hash
for
String
1.19.0
·
Source
§
impl
Hash
for
ThreadId
1.3.0
·
Source
§
impl
Hash
for
Duration
1.8.0
·
Source
§
impl
Hash
for
Instant
1.8.0
·
Source
§
impl
Hash
for
SystemTime
1.0.0
·
Source
§
impl<'a>
Hash
for
Component
<'a>
1.0.0
·
Source
§
impl<'a>
Hash
for
Prefix
<'a>
Source
§
impl<'a>
Hash
for
PhantomContravariantLifetime
<'a>
Source
§
impl<'a>
Hash
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
Hash
for
PhantomInvariantLifetime
<'a>
1.10.0
·
Source
§
impl<'a>
Hash
for
Location
<'a>
1.0.0
·
Source
§
impl<B>
Hash
for
Cow
<'_, B>
where
    B:
Hash
+
ToOwned
+ ?
Sized
,
1.55.0
·
Source
§
impl<B, C>
Hash
for
ControlFlow
<B, C>
where
    B:
Hash
,
    C:
Hash
,
Source
§
impl<Dyn>
Hash
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
1.4.0
·
Source
§
impl<F>
Hash
for F
where
    F:
FnPtr
,
1.0.0
·
Source
§
impl<Idx>
Hash
for std::ops::
Range
<Idx>
where
    Idx:
Hash
,
1.0.0
·
Source
§
impl<Idx>
Hash
for std::ops::
RangeFrom
<Idx>
where
    Idx:
Hash
,
1.26.0
·
Source
§
impl<Idx>
Hash
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
Hash
,
1.0.0
·
Source
§
impl<Idx>
Hash
for
RangeTo
<Idx>
where
    Idx:
Hash
,
1.26.0
·
Source
§
impl<Idx>
Hash
for
RangeToInclusive
<Idx>
where
    Idx:
Hash
,
Source
§
impl<Idx>
Hash
for std::range::
Range
<Idx>
where
    Idx:
Hash
,
Source
§
impl<Idx>
Hash
for std::range::
RangeFrom
<Idx>
where
    Idx:
Hash
,
Source
§
impl<Idx>
Hash
for std::range::
RangeInclusive
<Idx>
where
    Idx:
Hash
,
1.0.0
·
Source
§
impl<K, V, A>
Hash
for
BTreeMap
<K, V, A>
where
    K:
Hash
,
    V:
Hash
,
    A:
Allocator
+
Clone
,
1.41.0
·
Source
§
impl<Ptr>
Hash
for
Pin
<Ptr>
where
    Ptr:
Deref
,
    <Ptr as
Deref
>::
Target
:
Hash
,
1.17.0
·
Source
§
impl<T>
Hash
for
Bound
<T>
where
    T:
Hash
,
1.0.0
·
Source
§
impl<T>
Hash
for
Option
<T>
where
    T:
Hash
,
1.36.0
·
Source
§
impl<T>
Hash
for
Poll
<T>
where
    T:
Hash
,
1.0.0
·
Source
§
impl<T>
Hash
for
*const T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Hash
for
*mut T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Hash
for
&T
where
    T:
Hash
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Hash
for
&mut T
where
    T:
Hash
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Hash
for
[T]
where
    T:
Hash
,
1.0.0
·
Source
§
impl<T>
Hash
for
(T₁, T₂, …, Tₙ)
where
    T:
Hash
+ ?
Sized
,
This trait is implemented for tuples up to twelve items long.
1.19.0
·
Source
§
impl<T>
Hash
for
Reverse
<T>
where
    T:
Hash
,
Source
§
impl<T>
Hash
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Hash
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Hash
for
PhantomData
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Hash
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
1.21.0
·
Source
§
impl<T>
Hash
for
Discriminant
<T>
1.20.0
·
Source
§
impl<T>
Hash
for
ManuallyDrop
<T>
where
    T:
Hash
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
Hash
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Hash
,
1.74.0
·
Source
§
impl<T>
Hash
for
Saturating
<T>
where
    T:
Hash
,
1.0.0
·
Source
§
impl<T>
Hash
for
Wrapping
<T>
where
    T:
Hash
,
1.25.0
·
Source
§
impl<T>
Hash
for
NonNull
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
Box
<T, A>
where
    T:
Hash
+ ?
Sized
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
BTreeSet
<T, A>
where
    T:
Hash
,
    A:
Allocator
+
Clone
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
LinkedList
<T, A>
where
    T:
Hash
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
VecDeque
<T, A>
where
    T:
Hash
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
Rc
<T, A>
where
    T:
Hash
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
Hash
for
UniqueRc
<T, A>
where
    T:
Hash
+ ?
Sized
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
Arc
<T, A>
where
    T:
Hash
+ ?
Sized
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Hash
for
Vec
<T, A>
where
    T:
Hash
,
    A:
Allocator
,
The hash of a vector is the same as that of the corresponding slice,
as required by the
core::borrow::Borrow
implementation.
use
std::hash::BuildHasher;
let
b = std::hash::RandomState::new();
let
v: Vec<u8> =
vec!
[
0xa8
,
0x3c
,
0x09
];
let
s:
&
[u8] =
&
[
0xa8
,
0x3c
,
0x09
];
assert_eq!
(b.hash_one(v), b.hash_one(s));
1.0.0
·
Source
§
impl<T, E>
Hash
for
Result
<T, E>
where
    T:
Hash
,
    E:
Hash
,
1.0.0
·
Source
§
impl<T, const N:
usize
>
Hash
for
[T; N]
where
    T:
Hash
,
The hash of an array is the same as that of the corresponding slice,
as required by the
Borrow
implementation.
use
std::hash::BuildHasher;
let
b = std::hash::RandomState::new();
let
a: [u8;
3
] = [
0xa8
,
0x3c
,
0x09
];
let
s:
&
[u8] =
&
[
0xa8
,
0x3c
,
0x09
];
assert_eq!
(b.hash_one(a), b.hash_one(s));
Source
§
impl<T, const N:
usize
>
Hash
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Hash
,
Source
§
impl<Y, R>
Hash
for
CoroutineState
<Y, R>
where
    Y:
Hash
,
    R:
Hash
,