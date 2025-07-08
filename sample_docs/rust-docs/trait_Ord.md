Ord in std::cmp - Rust
std
::
cmp
Trait
Ord
Copy item path
1.0.0
·
Source
pub trait Ord:
Eq
+
PartialOrd
{
    // Required method
    fn
cmp
(&self, other: &Self) ->
Ordering
;

    // Provided methods
    fn
max
(self, other: Self) -> Self
where Self:
Sized
{ ... }
fn
min
(self, other: Self) -> Self
where Self:
Sized
{ ... }
fn
clamp
(self, min: Self, max: Self) -> Self
where Self:
Sized
{ ... }
}
Expand description
Trait for types that form a
total order
.
Implementations must be consistent with the
PartialOrd
implementation, and ensure
max
,
min
, and
clamp
are consistent with
cmp
:
partial_cmp(a, b) == Some(cmp(a, b))
.
max(a, b) == max_by(a, b, cmp)
(ensured by the default implementation).
min(a, b) == min_by(a, b, cmp)
(ensured by the default implementation).
For
a.clamp(min, max)
, see the
method docs
(ensured by the default
implementation).
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
Corollaries
From the above and the requirements of
PartialOrd
, it follows that for all
a
,
b
and
c
:
exactly one of
a < b
,
a == b
or
a > b
is true; and
<
is transitive:
a < b
and
b < c
implies
a < c
. The same must hold for both
==
and
>
.
Mathematically speaking, the
<
operator defines a strict
weak order
. In cases where
==
conforms to mathematical equality, it also defines a strict
total order
.
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
d on enums, variants are ordered primarily by their discriminants. Secondarily,
they are ordered by their fields. By default, the discriminant is smallest for variants at the
top, and largest for variants at the bottom. Here’s an example:
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum
E {
    Top,
    Bottom,
}
assert!
(E::Top < E::Bottom);
However, manually setting the discriminants can override this default behavior:
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
Lexicographical comparison
Lexicographical comparison is an operation with the following properties:
Two sequences are compared element by element.
The first mismatching element defines which sequence is lexicographically less or greater
than the other.
If one sequence is a prefix of another, the shorter sequence is lexicographically less than
the other.
If two sequences have equivalent elements and are of the same length, then the sequences are
lexicographically equal.
An empty sequence is lexicographically less than any non-empty sequence.
Two empty sequences are lexicographically equal.
§
How can I implement
Ord
?
Ord
requires that the type also be
PartialOrd
,
PartialEq
, and
Eq
.
Because
Ord
implies a stronger ordering relationship than
PartialOrd
, and both
Ord
and
PartialOrd
must agree, you must choose how to implement
Ord
first
. You can choose to
derive it, or implement it manually. If you derive it, you should derive all four traits. If you
implement it manually, you should manually implement all four traits, based on the
implementation of
Ord
.
Here’s an example where you want to define the
Character
comparison by
health
and
experience
only, disregarding the field
mana
:
use
std::cmp::Ordering;
struct
Character {
    health: u32,
    experience: u32,
    mana: f32,
}
impl
Ord
for
Character {
fn
cmp(
&
self
, other:
&
Self
) -> Ordering {
self
.experience
            .cmp(
&
other.experience)
            .then(
self
.health.cmp(
&
other.health))
    }
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
.cmp(other))
    }
}
impl
PartialEq
for
Character {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.health == other.health &&
self
.experience == other.experience
    }
}
impl
Eq
for
Character {}
If all you need is to
slice::sort
a type by a field value, it can be simpler to use
slice::sort_by_key
.
§
Examples of incorrect
Ord
implementations
use
std::cmp::Ordering;
#[derive(Debug)]
struct
Character {
    health: f32,
}
impl
Ord
for
Character {
fn
cmp(
&
self
, other:
&
Self
) -> std::cmp::Ordering {
if
self
.health < other.health {
            Ordering::Less
        }
else if
self
.health > other.health {
            Ordering::Greater
        }
else
{
            Ordering::Equal
        }
    }
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
.cmp(other))
    }
}
impl
PartialEq
for
Character {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.health == other.health
    }
}
impl
Eq
for
Character {}
let
a = Character { health:
4.5
};
let
b = Character { health: f32::NAN };
// Mistake: floating-point values do not form a total order and using the built-in comparison
// operands to implement `Ord` irregardless of that reality does not change it. Use
// `f32::total_cmp` if you need a total order for floating-point values.

// Reflexivity requirement of `Ord` is not given.
assert!
(a == a);
assert!
(b != b);
// Antisymmetry requirement of `Ord` is not given. Only one of a < c and c < a is allowed to be
// true, not both or neither.
assert_eq!
((a < b)
as
u8 + (b < a)
as
u8,
0
);
use
std::cmp::Ordering;
#[derive(Debug)]
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
.cmp(other))
    }
}
impl
Ord
for
Character {
fn
cmp(
&
self
, other:
&
Self
) -> std::cmp::Ordering {
if
self
.health <
50
{
self
.health.cmp(
&
other.health)
        }
else
{
self
.experience.cmp(
&
other.experience)
        }
    }
}
// For performance reasons implementing `PartialEq` this way is not the idiomatic way, but it
// ensures consistent behavior between `PartialEq`, `PartialOrd` and `Ord` in this example.
impl
PartialEq
for
Character {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.cmp(other) == Ordering::Equal
    }
}
impl
Eq
for
Character {}
let
a = Character {
    health:
3
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
let
c = Character {
    health:
143
,
    experience:
2
,
};
// Mistake: The implementation of `Ord` compares different fields depending on the value of
// `self.health`, the resulting order is not total.

// Transitivity requirement of `Ord` is not given. If a is smaller than b and b is smaller than
// c, by transitive property a must also be smaller than c.
assert!
(a < b && b < c && c < a);
// Antisymmetry requirement of `Ord` is not given. Only one of a < c and c < a is allowed to be
// true, not both or neither.
assert_eq!
((a < c)
as
u8 + (c < a)
as
u8,
2
);
The documentation of
PartialOrd
contains further examples, for example it’s wrong for
PartialOrd
and
PartialEq
to disagree.
Required Methods
§
1.0.0
·
Source
fn
cmp
(&self, other: &Self) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
By convention,
self.cmp(&other)
returns the ordering matching the expression
self <operator> other
if true.
§
Examples
use
std::cmp::Ordering;
assert_eq!
(
5
.cmp(
&
10
), Ordering::Less);
assert_eq!
(
10
.cmp(
&
5
), Ordering::Greater);
assert_eq!
(
5
.cmp(
&
5
), Ordering::Equal);
Provided Methods
§
1.21.0
·
Source
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Returns the second argument if the comparison determines them to be equal.
§
Examples
assert_eq!
(
1
.max(
2
),
2
);
assert_eq!
(
2
.max(
2
),
2
);
use
std::cmp::Ordering;
#[derive(Eq)]
struct
Equal(
&
'static
str);
impl
PartialEq
for
Equal {
fn
eq(
&
self
, other:
&
Self
) -> bool {
true
}
}
impl
PartialOrd
for
Equal {
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
(Ordering::Equal) }
}
impl
Ord
for
Equal {
fn
cmp(
&
self
, other:
&
Self
) -> Ordering { Ordering::Equal }
}
assert_eq!
(Equal(
"self"
).max(Equal(
"other"
)).
0
,
"other"
);
1.21.0
·
Source
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Returns the first argument if the comparison determines them to be equal.
§
Examples
assert_eq!
(
1
.min(
2
),
1
);
assert_eq!
(
2
.min(
2
),
2
);
use
std::cmp::Ordering;
#[derive(Eq)]
struct
Equal(
&
'static
str);
impl
PartialEq
for
Equal {
fn
eq(
&
self
, other:
&
Self
) -> bool {
true
}
}
impl
PartialOrd
for
Equal {
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
(Ordering::Equal) }
}
impl
Ord
for
Equal {
fn
cmp(
&
self
, other:
&
Self
) -> Ordering { Ordering::Equal }
}
assert_eq!
(Equal(
"self"
).min(Equal(
"other"
)).
0
,
"self"
);
1.50.0
·
Source
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Returns
max
if
self
is greater than
max
, and
min
if
self
is
less than
min
. Otherwise this returns
self
.
§
Panics
Panics if
min > max
.
§
Examples
assert_eq!
((-
3
).clamp(-
2
,
1
), -
2
);
assert_eq!
(
0
.clamp(-
2
,
1
),
0
);
assert_eq!
(
2
.clamp(-
2
,
1
),
1
);
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
Ord
for
AsciiChar
1.34.0
·
Source
§
impl
Ord
for
Infallible
1.0.0
·
Source
§
impl
Ord
for
ErrorKind
1.7.0
·
Source
§
impl
Ord
for
IpAddr
1.0.0
·
Source
§
impl
Ord
for
SocketAddr
1.0.0
·
Source
§
impl
Ord
for
Ordering
1.0.0
·
Source
§
impl
Ord
for
bool
1.0.0
·
Source
§
impl
Ord
for
char
1.0.0
·
Source
§
impl
Ord
for
i8
1.0.0
·
Source
§
impl
Ord
for
i16
1.0.0
·
Source
§
impl
Ord
for
i32
1.0.0
·
Source
§
impl
Ord
for
i64
1.0.0
·
Source
§
impl
Ord
for
i128
1.0.0
·
Source
§
impl
Ord
for
isize
Source
§
impl
Ord
for
!
1.0.0
·
Source
§
impl
Ord
for
str
Implements ordering of strings.
Strings are ordered
lexicographically
by their byte values. This orders Unicode code
points based on their positions in the code charts. This is not necessarily the same as
“alphabetical” order, which varies by language and locale. Sorting strings according to
culturally-accepted standards requires locale-specific data that is outside the scope of
the
str
type.
1.0.0
·
Source
§
impl
Ord
for
u8
1.0.0
·
Source
§
impl
Ord
for
u16
1.0.0
·
Source
§
impl
Ord
for
u32
1.0.0
·
Source
§
impl
Ord
for
u64
1.0.0
·
Source
§
impl
Ord
for
u128
1.0.0
·
Source
§
impl
Ord
for
()
1.0.0
·
Source
§
impl
Ord
for
usize
1.0.0
·
Source
§
impl
Ord
for
TypeId
Source
§
impl
Ord
for
ByteStr
Source
§
impl
Ord
for
ByteString
1.0.0
·
Source
§
impl
Ord
for
CStr
1.64.0
·
Source
§
impl
Ord
for
CString
1.0.0
·
Source
§
impl
Ord
for
OsStr
1.0.0
·
Source
§
impl
Ord
for
OsString
1.0.0
·
Source
§
impl
Ord
for
Error
1.33.0
·
Source
§
impl
Ord
for
PhantomPinned
1.0.0
·
Source
§
impl
Ord
for
Ipv4Addr
1.0.0
·
Source
§
impl
Ord
for
Ipv6Addr
1.0.0
·
Source
§
impl
Ord
for
SocketAddrV4
1.0.0
·
Source
§
impl
Ord
for
SocketAddrV6
1.0.0
·
Source
§
impl
Ord
for
Components
<'_>
1.0.0
·
Source
§
impl
Ord
for
Path
1.0.0
·
Source
§
impl
Ord
for
PathBuf
1.0.0
·
Source
§
impl
Ord
for
PrefixComponent
<'_>
Source
§
impl
Ord
for
Alignment
1.0.0
·
Source
§
impl
Ord
for
String
1.3.0
·
Source
§
impl
Ord
for
Duration
1.8.0
·
Source
§
impl
Ord
for
Instant
1.8.0
·
Source
§
impl
Ord
for
SystemTime
1.0.0
·
Source
§
impl<'a>
Ord
for
Component
<'a>
1.0.0
·
Source
§
impl<'a>
Ord
for
Prefix
<'a>
Source
§
impl<'a>
Ord
for
PhantomContravariantLifetime
<'a>
Source
§
impl<'a>
Ord
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
Ord
for
PhantomInvariantLifetime
<'a>
1.10.0
·
Source
§
impl<'a>
Ord
for
Location
<'a>
1.0.0
·
Source
§
impl<A>
Ord
for
&A
where
    A:
Ord
+ ?
Sized
,
1.0.0
·
Source
§
impl<A>
Ord
for
&mut A
where
    A:
Ord
+ ?
Sized
,
1.0.0
·
Source
§
impl<B>
Ord
for
Cow
<'_, B>
where
    B:
Ord
+
ToOwned
+ ?
Sized
,
Source
§
impl<Dyn>
Ord
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
Ord
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
Ord
for
BTreeMap
<K, V, A>
where
    K:
Ord
,
    V:
Ord
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
Ord
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
Ord
,
1.0.0
·
Source
§
impl<T>
Ord
for
Option
<T>
where
    T:
Ord
,
1.36.0
·
Source
§
impl<T>
Ord
for
Poll
<T>
where
    T:
Ord
,
1.0.0
·
Source
§
impl<T>
Ord
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
Ord
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
Ord
for
[T]
where
    T:
Ord
,
Implements comparison of slices
lexicographically
.
1.0.0
·
Source
§
impl<T>
Ord
for
(T₁, T₂, …, Tₙ)
where
    T:
Ord
+ ?
Sized
,
This trait is implemented for tuples up to twelve items long.
1.10.0
·
Source
§
impl<T>
Ord
for
Cell
<T>
where
    T:
Ord
+
Copy
,
1.10.0
·
Source
§
impl<T>
Ord
for
RefCell
<T>
where
    T:
Ord
+ ?
Sized
,
Source
§
impl<T>
Ord
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
Ord
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
Ord
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
Ord
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
Ord
for
ManuallyDrop
<T>
where
    T:
Ord
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
Ord
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Ord
,
1.74.0
·
Source
§
impl<T>
Ord
for
Saturating
<T>
where
    T:
Ord
,
1.0.0
·
Source
§
impl<T>
Ord
for
Wrapping
<T>
where
    T:
Ord
,
1.25.0
·
Source
§
impl<T>
Ord
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
Ord
for
Reverse
<T>
where
    T:
Ord
,
1.0.0
·
Source
§
impl<T, A>
Ord
for
Box
<T, A>
where
    T:
Ord
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
Ord
for
BTreeSet
<T, A>
where
    T:
Ord
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
Ord
for
LinkedList
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Ord
for
VecDeque
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Ord
for
Rc
<T, A>
where
    T:
Ord
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
Ord
for
UniqueRc
<T, A>
where
    T:
Ord
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
Ord
for
Arc
<T, A>
where
    T:
Ord
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
Ord
for
Vec
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
Implements ordering of vectors,
lexicographically
.
1.0.0
·
Source
§
impl<T, E>
Ord
for
Result
<T, E>
where
    T:
Ord
,
    E:
Ord
,
1.0.0
·
Source
§
impl<T, const N:
usize
>
Ord
for
[T; N]
where
    T:
Ord
,
Implements comparison of arrays
lexicographically
.
Source
§
impl<T, const N:
usize
>
Ord
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
Ord
,
Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
Source
§
impl<Y, R>
Ord
for
CoroutineState
<Y, R>
where
    Y:
Ord
,
    R:
Ord
,