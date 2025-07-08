PartialOrd in std::cmp - Rust
std
::
cmp
Trait
PartialOrd
Copy item path
1.0.0
·
Source
pub trait PartialOrd<Rhs = Self>:
PartialEq
<Rhs>
where
    Rhs: ?
Sized
,
{
    // Required method
    fn
partial_cmp
(&self, other:
&Rhs
) ->
Option
<
Ordering
>;

    // Provided methods
    fn
lt
(&self, other:
&Rhs
) ->
bool
{ ... }
fn
le
(&self, other:
&Rhs
) ->
bool
{ ... }
fn
gt
(&self, other:
&Rhs
) ->
bool
{ ... }
fn
ge
(&self, other:
&Rhs
) ->
bool
{ ... }
}
Expand description
Trait for types that form a
partial order
.
The
lt
,
le
,
gt
, and
ge
methods of this trait can be called using the
<
,
<=
,
>
, and
>=
operators, respectively.
This trait should
only
contain the comparison logic for a type
if one plans on only
implementing
PartialOrd
but not
Ord
. Otherwise the comparison logic should be in
Ord
and this trait implemented with
Some(self.cmp(other))
.
The methods of this trait must be consistent with each other and with those of
PartialEq
.
The following conditions must hold:
a == b
if and only if
partial_cmp(a, b) == Some(Equal)
.
a < b
if and only if
partial_cmp(a, b) == Some(Less)
a > b
if and only if
partial_cmp(a, b) == Some(Greater)
a <= b
if and only if
a < b || a == b
a >= b
if and only if
a > b || a == b
a != b
if and only if
!(a == b)
.
Conditions 2–5 above are ensured by the default implementation. Condition 6 is already ensured
by
PartialEq
.
If
Ord
is also implemented for
Self
and
Rhs
, it must also be consistent with
partial_cmp
(see the documentation of that trait for the exact requirements). It’s easy to
accidentally make them disagree by deriving some of the traits and manually implementing others.
The comparison relations must satisfy the following conditions (for all
a
,
b
,
c
of type
A
,
B
,
C
):
Transitivity
: if
A: PartialOrd<B>
and
B: PartialOrd<C>
and
A: PartialOrd<C>
, then
a < b
and
b < c
implies
a < c
. The same must hold for both
==
and
>
. This must also
work for longer chains, such as when
A: PartialOrd<B>
,
B: PartialOrd<C>
,
C: PartialOrd<D>
, and
A: PartialOrd<D>
all exist.
Duality
: if
A: PartialOrd<B>
and
B: PartialOrd<A>
, then
a < b
if and only if
b > a
.
Note that the
B: PartialOrd<A>
(dual) and
A: PartialOrd<C>
(transitive) impls are not forced
to exist, but these requirements apply whenever they do exist.
Violating these requirements is a logic error. The behavior resulting from a logic error is not
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
Cross-crate considerations
Upholding the requirements stated above can become tricky when one crate implements
PartialOrd
for a type of another crate (i.e., to allow comparing one of its own types with a type from the
standard library). The recommendation is to never implement this trait for a foreign type. In
other words, such a crate should do
impl PartialOrd<ForeignType> for LocalType
, but it should
not
do
impl PartialOrd<LocalType> for ForeignType
.
This avoids the problem of transitive chains that criss-cross crate boundaries: for all local
types
T
, you may assume that no other crate will add
impl
s that allow comparing
T < U
. In
other words, if other crates add
impl
s that allow building longer transitive chains
U1 < ... < T < V1 < ...
, then all the types that appear to the right of
T
must be types that the crate
defining
T
already knows about. This rules out transitive chains where downstream crates can
add new
impl
s that “stitch together” comparisons of foreign types in ways that violate
transitivity.
Not having such foreign
impl
s also avoids forward compatibility issues where one crate adding
more
PartialOrd
implementations can cause build failures in downstream crates.
§
Corollaries
The following corollaries follow from the above requirements:
irreflexivity of
<
and
>
:
!(a < a)
,
!(a > a)
transitivity of
>
: if
a > b
and
b > c
then
a > c
duality of
partial_cmp
:
partial_cmp(a, b) == partial_cmp(b, a).map(Ordering::reverse)
§
Strict and non-strict partial orders
The
<
and
>
operators behave according to a
strict
partial order. However,
<=
and
>=
do
not
behave according to a
non-strict
partial order. That is because mathematically, a
non-strict partial order would require reflexivity, i.e.
a <= a
would need to be true for
every
a
. This isn’t always the case for types that implement
PartialOrd
, for example:
let
a = f64::sqrt(-
1.0
);
assert_eq!
(a <= a,
false
);
§
Derivable
This trait can be used with
#[derive]
.
When
derive
d on structs, it will produce a
lexicographic
ordering based on the
top-to-bottom declaration order of the struct’s members.
When
derive
d on enums, variants are primarily ordered by their discriminants. Secondarily,
they are ordered by their fields. By default, the discriminant is smallest for variants at the
top, and largest for variants at the bottom. Here’s an example:
#[derive(PartialEq, PartialOrd)]
enum
E {
    Top,
    Bottom,
}
assert!
(E::Top < E::Bottom);
However, manually setting the discriminants can override this default behavior:
#[derive(PartialEq, PartialOrd)]
enum
E {
    Top =
2
,
    Bottom =
1
,
}
assert!
(E::Bottom < E::Top);
§
How can I implement
PartialOrd
?
PartialOrd
only requires implementation of the
partial_cmp
method, with the others
generated from default implementations.
However it remains possible to implement the others separately for types which do not have a
total order. For example, for floating point numbers,
NaN < 0 == false
and
NaN >= 0 == false
(cf. IEEE 754-2008 section 5.11).
PartialOrd
requires your type to be
PartialEq
.
If your type is
Ord
, you can implement
partial_cmp
by using
cmp
:
use
std::cmp::Ordering;
struct
Person {
    id: u32,
    name: String,
    height: u32,
}
impl
PartialOrd
for
Person {
fn
partial_cmp(
&
self
, other:
&
Self
) ->
Option
<Ordering> {
Some
(
self
.cmp(other))
    }
}
impl
Ord
for
Person {
fn
cmp(
&
self
, other:
&
Self
) -> Ordering {
self
.height.cmp(
&
other.height)
    }
}
impl
PartialEq
for
Person {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.height == other.height
    }
}
impl
Eq
for
Person {}
You may also find it useful to use
partial_cmp
on your type’s fields. Here is an example of
Person
types who have a floating-point
height
field that is the only field to be used for
sorting:
use
std::cmp::Ordering;
struct
Person {
    id: u32,
    name: String,
    height: f64,
}
impl
PartialOrd
for
Person {
fn
partial_cmp(
&
self
, other:
&
Self
) ->
Option
<Ordering> {
self
.height.partial_cmp(
&
other.height)
    }
}
impl
PartialEq
for
Person {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.height == other.height
    }
}
§
Examples of incorrect
PartialOrd
implementations
use
std::cmp::Ordering;
#[derive(PartialEq, Debug)]
struct
Character {
    health: u32,
    experience: u32,
}
impl
PartialOrd
for
Character {
fn
partial_cmp(
&
self
, other:
&
Self
) ->
Option
<Ordering> {
Some
(
self
.health.cmp(
&
other.health))
    }
}
let
a = Character {
    health:
10
,
    experience:
5
,
};
let
b = Character {
    health:
10
,
    experience:
77
,
};
// Mistake: `PartialEq` and `PartialOrd` disagree with each other.
assert_eq!
(a.partial_cmp(
&
b).unwrap(), Ordering::Equal);
// a == b according to `PartialOrd`.
assert_ne!
(a, b);
// a != b according to `PartialEq`.
§
Examples
let
x: u32 =
0
;
let
y: u32 =
1
;
assert_eq!
(x < y,
true
);
assert_eq!
(x.lt(
&
y),
true
);
Required Methods
§
1.0.0
·
Source
fn
partial_cmp
(&self, other:
&Rhs
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
§
Examples
use
std::cmp::Ordering;
let
result =
1.0
.partial_cmp(
&
2.0
);
assert_eq!
(result,
Some
(Ordering::Less));
let
result =
1.0
.partial_cmp(
&
1.0
);
assert_eq!
(result,
Some
(Ordering::Equal));
let
result =
2.0
.partial_cmp(
&
1.0
);
assert_eq!
(result,
Some
(Ordering::Greater));
When comparison is impossible:
let
result = f64::NAN.partial_cmp(
&
1.0
);
assert_eq!
(result,
None
);
Provided Methods
§
1.0.0
·
Source
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
§
Examples
assert_eq!
(
1.0
<
1.0
,
false
);
assert_eq!
(
1.0
<
2.0
,
true
);
assert_eq!
(
2.0
<
1.0
,
false
);
1.0.0
·
Source
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
§
Examples
assert_eq!
(
1.0
<=
1.0
,
true
);
assert_eq!
(
1.0
<=
2.0
,
true
);
assert_eq!
(
2.0
<=
1.0
,
false
);
1.0.0
·
Source
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
§
Examples
assert_eq!
(
1.0
>
1.0
,
false
);
assert_eq!
(
1.0
>
2.0
,
false
);
assert_eq!
(
2.0
>
1.0
,
true
);
1.0.0
·
Source
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
§
Examples
assert_eq!
(
1.0
>=
1.0
,
true
);
assert_eq!
(
1.0
>=
2.0
,
false
);
assert_eq!
(
2.0
>=
1.0
,
true
);
Implementors
§
Source
§
impl
PartialOrd
for
AsciiChar
1.34.0
·
Source
§
impl
PartialOrd
for
Infallible
1.0.0
·
Source
§
impl
PartialOrd
for
ErrorKind
1.7.0
·
Source
§
impl
PartialOrd
for
IpAddr
1.0.0
·
Source
§
impl
PartialOrd
for
SocketAddr
1.0.0
·
Source
§
impl
PartialOrd
for
Ordering
1.0.0
·
Source
§
impl
PartialOrd
for
bool
1.0.0
·
Source
§
impl
PartialOrd
for
char
1.0.0
·
Source
§
impl
PartialOrd
for
f16
1.0.0
·
Source
§
impl
PartialOrd
for
f32
1.0.0
·
Source
§
impl
PartialOrd
for
f64
1.0.0
·
Source
§
impl
PartialOrd
for
f128
1.0.0
·
Source
§
impl
PartialOrd
for
i8
1.0.0
·
Source
§
impl
PartialOrd
for
i16
1.0.0
·
Source
§
impl
PartialOrd
for
i32
1.0.0
·
Source
§
impl
PartialOrd
for
i64
1.0.0
·
Source
§
impl
PartialOrd
for
i128
1.0.0
·
Source
§
impl
PartialOrd
for
isize
Source
§
impl
PartialOrd
for
!
1.0.0
·
Source
§
impl
PartialOrd
for
str
Implements comparison operations on strings.
Strings are compared
lexicographically
by their byte values. This compares Unicode code
points based on their positions in the code charts. This is not necessarily the same as
“alphabetical” order, which varies by language and locale. Comparing strings according to
culturally-accepted standards requires locale-specific data that is outside the scope of
the
str
type.
1.0.0
·
Source
§
impl
PartialOrd
for
u8
1.0.0
·
Source
§
impl
PartialOrd
for
u16
1.0.0
·
Source
§
impl
PartialOrd
for
u32
1.0.0
·
Source
§
impl
PartialOrd
for
u64
1.0.0
·
Source
§
impl
PartialOrd
for
u128
1.0.0
·
Source
§
impl
PartialOrd
for
()
1.0.0
·
Source
§
impl
PartialOrd
for
usize
1.0.0
·
Source
§
impl
PartialOrd
for
TypeId
Source
§
impl
PartialOrd
for
ByteStr
Source
§
impl
PartialOrd
for
ByteString
1.0.0
·
Source
§
impl
PartialOrd
for
CStr
1.64.0
·
Source
§
impl
PartialOrd
for
CString
1.0.0
·
Source
§
impl
PartialOrd
for
OsStr
1.0.0
·
Source
§
impl
PartialOrd
for
OsString
1.0.0
·
Source
§
impl
PartialOrd
for
Error
1.33.0
·
Source
§
impl
PartialOrd
for
PhantomPinned
1.0.0
·
Source
§
impl
PartialOrd
for
Ipv4Addr
1.0.0
·
Source
§
impl
PartialOrd
for
Ipv6Addr
1.0.0
·
Source
§
impl
PartialOrd
for
SocketAddrV4
1.0.0
·
Source
§
impl
PartialOrd
for
SocketAddrV6
1.0.0
·
Source
§
impl
PartialOrd
for
Path
1.0.0
·
Source
§
impl
PartialOrd
for
PathBuf
Source
§
impl
PartialOrd
for
Alignment
1.0.0
·
Source
§
impl
PartialOrd
for
String
1.3.0
·
Source
§
impl
PartialOrd
for
Duration
1.8.0
·
Source
§
impl
PartialOrd
for
Instant
1.8.0
·
Source
§
impl
PartialOrd
for
SystemTime
1.16.0
·
Source
§
impl
PartialOrd
<
IpAddr
> for
Ipv4Addr
1.16.0
·
Source
§
impl
PartialOrd
<
IpAddr
> for
Ipv6Addr
1.0.0
·
Source
§
impl
PartialOrd
<
str
> for
OsStr
1.0.0
·
Source
§
impl
PartialOrd
<
str
> for
OsString
1.8.0
·
Source
§
impl
PartialOrd
<
OsStr
> for
Path
1.8.0
·
Source
§
impl
PartialOrd
<
OsStr
> for
PathBuf
1.8.0
·
Source
§
impl
PartialOrd
<
OsString
> for
Path
1.8.0
·
Source
§
impl
PartialOrd
<
OsString
> for
PathBuf
1.16.0
·
Source
§
impl
PartialOrd
<
Ipv4Addr
> for
IpAddr
1.16.0
·
Source
§
impl
PartialOrd
<
Ipv6Addr
> for
IpAddr
1.8.0
·
Source
§
impl
PartialOrd
<
Path
> for
OsStr
1.8.0
·
Source
§
impl
PartialOrd
<
Path
> for
OsString
1.8.0
·
Source
§
impl
PartialOrd
<
Path
> for
PathBuf
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
OsStr
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
OsString
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
Path
1.0.0
·
Source
§
impl<'a>
PartialOrd
for
Component
<'a>
1.0.0
·
Source
§
impl<'a>
PartialOrd
for
Prefix
<'a>
Source
§
impl<'a>
PartialOrd
for
PhantomContravariantLifetime
<'a>
Source
§
impl<'a>
PartialOrd
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
PartialOrd
for
PhantomInvariantLifetime
<'a>
1.10.0
·
Source
§
impl<'a>
PartialOrd
for
Location
<'a>
1.0.0
·
Source
§
impl<'a>
PartialOrd
for
Components
<'a>
1.0.0
·
Source
§
impl<'a>
PartialOrd
for
PrefixComponent
<'a>
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a,
str
>
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a,
ByteStr
>
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a, [
u8
]>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
OsStr
> for
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
OsStr
> for
PathBuf
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
Path
> for
OsStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
Path
> for
OsString
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
Path
> for
PathBuf
Source
§
impl<'a>
PartialOrd
<&
ByteStr
> for
ByteString
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
str
>> for
ByteString
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
impl<'a>
PartialOrd
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
str
>> for &'a
ByteStr
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
ByteStr
>> for &'a
ByteStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
OsStr
>> for
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
OsStr
>> for
PathBuf
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsString
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
impl<'a>
PartialOrd
<
Cow
<'a, [
u8
]>> for &'a
ByteStr
Source
§
impl<'a>
PartialOrd
<
ByteStr
> for
ByteString
Source
§
impl<'a>
PartialOrd
<
ByteString
> for &
ByteStr
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
str
>
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
ByteStr
>
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_, [
u8
]>
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
ByteStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsStr
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsStr
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsString
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsString
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for &'a
OsStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for
Cow
<'a,
OsStr
>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for &'a
OsStr
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
OsStr
>
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'a
OsStr
> for
OsString
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'a
Path
> for
Cow
<'b,
OsStr
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
OsStr
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
Path
> for
Cow
<'a,
Path
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for &'b
OsStr
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for
OsStr
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for
OsString
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
Path
>> for &'b
OsStr
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
Path
>> for &'b
Path
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'b,
OsStr
>> for &'a
Path
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsStr
> for
Cow
<'a,
OsStr
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsStr
> for
OsString
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for &'a
OsStr
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for
Cow
<'a,
OsStr
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for
OsStr
1.0.0
·
Source
§
impl<'a, B>
PartialOrd
for
Cow
<'a, B>
where
    B:
PartialOrd
+
ToOwned
+ ?
Sized
,
1.0.0
·
Source
§
impl<A, B>
PartialOrd
<
&B
> for
&A
where
    A:
PartialOrd
<B> + ?
Sized
,
    B: ?
Sized
,
1.0.0
·
Source
§
impl<A, B>
PartialOrd
<
&mut B
> for
&mut A
where
    A:
PartialOrd
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
impl<Dyn>
PartialOrd
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
PartialOrd
for F
where
    F:
FnPtr
,
1.0.0
·
Source
§
impl<K, V, A>
PartialOrd
for
BTreeMap
<K, V, A>
where
    K:
PartialOrd
,
    V:
PartialOrd
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
impl<Ptr, Q>
PartialOrd
<
Pin
<Q>> for
Pin
<Ptr>
where
    Ptr:
Deref
,
    Q:
Deref
,
    <Ptr as
Deref
>::
Target
:
PartialOrd
<<Q as
Deref
>::
Target
>,
1.0.0
·
Source
§
impl<T>
PartialOrd
for
Option
<T>
where
    T:
PartialOrd
,
1.36.0
·
Source
§
impl<T>
PartialOrd
for
Poll
<T>
where
    T:
PartialOrd
,
1.0.0
·
Source
§
impl<T>
PartialOrd
for
*const T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
[
<*const T>::addr
](pointer::addr)
method.
1.0.0
·
Source
§
impl<T>
PartialOrd
for
*mut T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
<*mut T>::addr
method.
1.0.0
·
Source
§
impl<T>
PartialOrd
for
[T]
where
    T:
PartialOrd
,
Implements comparison of slices
lexicographically
.
1.0.0
·
Source
§
impl<T>
PartialOrd
for
(T₁, T₂, …, Tₙ)
where
    T:
PartialOrd
+ ?
Sized
,
This trait is implemented for tuples up to twelve items long.
1.10.0
·
Source
§
impl<T>
PartialOrd
for
Cell
<T>
where
    T:
PartialOrd
+
Copy
,
1.10.0
·
Source
§
impl<T>
PartialOrd
for
RefCell
<T>
where
    T:
PartialOrd
+ ?
Sized
,
Source
§
impl<T>
PartialOrd
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
PartialOrd
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
PartialOrd
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
PartialOrd
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
1.20.0
·
Source
§
impl<T>
PartialOrd
for
ManuallyDrop
<T>
where
    T:
PartialOrd
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
PartialOrd
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
PartialOrd
,
1.74.0
·
Source
§
impl<T>
PartialOrd
for
Saturating
<T>
where
    T:
PartialOrd
,
1.0.0
·
Source
§
impl<T>
PartialOrd
for
Wrapping
<T>
where
    T:
PartialOrd
,
1.25.0
·
Source
§
impl<T>
PartialOrd
for
NonNull
<T>
where
    T: ?
Sized
,
1.19.0
·
Source
§
impl<T>
PartialOrd
for
Reverse
<T>
where
    T:
PartialOrd
,
1.0.0
·
Source
§
impl<T, A1, A2>
PartialOrd
<
Vec
<T, A2>> for
Vec
<T, A1>
where
    T:
PartialOrd
,
    A1:
Allocator
,
    A2:
Allocator
,
Implements comparison of vectors,
lexicographically
.
1.0.0
·
Source
§
impl<T, A>
PartialOrd
for
Box
<T, A>
where
    T:
PartialOrd
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
PartialOrd
for
BTreeSet
<T, A>
where
    T:
PartialOrd
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
PartialOrd
for
LinkedList
<T, A>
where
    T:
PartialOrd
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
PartialOrd
for
VecDeque
<T, A>
where
    T:
PartialOrd
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
PartialOrd
for
Rc
<T, A>
where
    T:
PartialOrd
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
PartialOrd
for
UniqueRc
<T, A>
where
    T:
PartialOrd
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
PartialOrd
for
Arc
<T, A>
where
    T:
PartialOrd
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
impl<T, E>
PartialOrd
for
Result
<T, E>
where
    T:
PartialOrd
,
    E:
PartialOrd
,
1.0.0
·
Source
§
impl<T, const N:
usize
>
PartialOrd
for
[T; N]
where
    T:
PartialOrd
,
Implements comparison of arrays
lexicographically
.
Source
§
impl<T, const N:
usize
>
PartialOrd
for
Mask
<T, N>
where
    T:
MaskElement
+
PartialOrd
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<T, const N:
usize
>
PartialOrd
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
PartialOrd
,
Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
Source
§
impl<Y, R>
PartialOrd
for
CoroutineState
<Y, R>
where
    Y:
PartialOrd
,
    R:
PartialOrd
,