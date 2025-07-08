PartialEq in std::cmp - Rust
std
::
cmp
Trait
PartialEq
Copy item path
1.0.0
·
Source
pub trait PartialEq<Rhs = Self>
where
    Rhs: ?
Sized
,
{
    // Required method
    fn
eq
(&self, other:
&Rhs
) ->
bool
;

    // Provided method
    fn
ne
(&self, other:
&Rhs
) ->
bool
{ ... }
}
Expand description
Trait for comparisons using the equality operator.
Implementing this trait for types provides the
==
and
!=
operators for
those types.
x.eq(y)
can also be written
x == y
, and
x.ne(y)
can be written
x != y
.
We use the easier-to-read infix notation in the remainder of this documentation.
This trait allows for comparisons using the equality operator, for types
that do not have a full equivalence relation. For example, in floating point
numbers
NaN != NaN
, so floating point types implement
PartialEq
but not
Eq
. Formally speaking, when
Rhs == Self
, this trait corresponds
to a
partial equivalence relation
.
Implementations must ensure that
eq
and
ne
are consistent with each other:
a != b
if and only if
!(a == b)
.
The default implementation of
ne
provides this consistency and is almost
always sufficient. It should not be overridden without very good reason.
If
PartialOrd
or
Ord
are also implemented for
Self
and
Rhs
, their methods must also
be consistent with
PartialEq
(see the documentation of those traits for the exact
requirements). It’s easy to accidentally make them disagree by deriving some of the traits and
manually implementing others.
The equality relation
==
must satisfy the following conditions
(for all
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
Symmetry
: if
A: PartialEq<B>
and
B: PartialEq<A>
, then
a == b
implies
b == a
; and
Transitivity
: if
A: PartialEq<B>
and
B: PartialEq<C>
and
A: PartialEq<C>
, then
a == b
and
b == c
implies
a == c
.
This must also work for longer chains, such as when
A: PartialEq<B>
,
B: PartialEq<C>
,
C: PartialEq<D>
, and
A: PartialEq<D>
all exist.
Note that the
B: PartialEq<A>
(symmetric) and
A: PartialEq<C>
(transitive) impls are not forced to exist, but these requirements apply
whenever they do exist.
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
PartialEq
for a type of another crate (i.e., to allow comparing one of its own types with a type from the
standard library). The recommendation is to never implement this trait for a foreign type. In
other words, such a crate should do
impl PartialEq<ForeignType> for LocalType
, but it should
not
do
impl PartialEq<LocalType> for ForeignType
.
This avoids the problem of transitive chains that criss-cross crate boundaries: for all local
types
T
, you may assume that no other crate will add
impl
s that allow comparing
T == U
. In
other words, if other crates add
impl
s that allow building longer transitive chains
U1 == ... == T == V1 == ...
, then all the types that appear to the right of
T
must be types that the
crate defining
T
already knows about. This rules out transitive chains where downstream crates
can add new
impl
s that “stitch together” comparisons of foreign types in ways that violate
transitivity.
Not having such foreign
impl
s also avoids forward compatibility issues where one crate adding
more
PartialEq
implementations can cause build failures in downstream crates.
§
Derivable
This trait can be used with
#[derive]
. When
derive
d on structs, two
instances are equal if all fields are equal, and not equal if any fields
are not equal. When
derive
d on enums, two instances are equal if they
are the same variant and all fields are equal.
§
How can I implement
PartialEq
?
An example implementation for a domain in which two books are considered
the same book if their ISBN matches, even if the formats differ:
enum
BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
struct
Book {
    isbn: i32,
    format: BookFormat,
}
impl
PartialEq
for
Book {
fn
eq(
&
self
, other:
&
Self
) -> bool {
self
.isbn == other.isbn
    }
}
let
b1 = Book { isbn:
3
, format: BookFormat::Paperback };
let
b2 = Book { isbn:
3
, format: BookFormat::Ebook };
let
b3 = Book { isbn:
10
, format: BookFormat::Paperback };
assert!
(b1 == b2);
assert!
(b1 != b3);
§
How can I compare two different types?
The type you can compare with is controlled by
PartialEq
’s type parameter.
For example, let’s tweak our previous code a bit:
// The derive implements <BookFormat> == <BookFormat> comparisons
#[derive(PartialEq)]
enum
BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
struct
Book {
    isbn: i32,
    format: BookFormat,
}
// Implement <Book> == <BookFormat> comparisons
impl
PartialEq<BookFormat>
for
Book {
fn
eq(
&
self
, other:
&
BookFormat) -> bool {
self
.format ==
*
other
    }
}
// Implement <BookFormat> == <Book> comparisons
impl
PartialEq<Book>
for
BookFormat {
fn
eq(
&
self
, other:
&
Book) -> bool {
*
self
== other.format
    }
}
let
b1 = Book { isbn:
3
, format: BookFormat::Paperback };
assert!
(b1 == BookFormat::Paperback);
assert!
(BookFormat::Ebook != b1);
By changing
impl PartialEq for Book
to
impl PartialEq<BookFormat> for Book
,
we allow
BookFormat
s to be compared with
Book
s.
A comparison like the one above, which ignores some fields of the struct,
can be dangerous. It can easily lead to an unintended violation of the
requirements for a partial equivalence relation. For example, if we kept
the above implementation of
PartialEq<Book>
for
BookFormat
and added an
implementation of
PartialEq<Book>
for
Book
(either via a
#[derive]
or
via the manual implementation from the first example) then the result would
violate transitivity:
ⓘ
#[derive(PartialEq)]
enum
BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
#[derive(PartialEq)]
struct
Book {
    isbn: i32,
    format: BookFormat,
}
impl
PartialEq<BookFormat>
for
Book {
fn
eq(
&
self
, other:
&
BookFormat) -> bool {
self
.format ==
*
other
    }
}
impl
PartialEq<Book>
for
BookFormat {
fn
eq(
&
self
, other:
&
Book) -> bool {
*
self
== other.format
    }
}
fn
main() {
let
b1 = Book { isbn:
1
, format: BookFormat::Paperback };
let
b2 = Book { isbn:
2
, format: BookFormat::Paperback };
assert!
(b1 == BookFormat::Paperback);
assert!
(BookFormat::Paperback == b2);
// The following should hold by transitivity but doesn't.
assert!
(b1 == b2);
// <-- PANICS
}
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
(x == y,
false
);
assert_eq!
(x.eq(
&
y),
false
);
Required Methods
§
1.0.0
·
Source
fn
eq
(&self, other:
&Rhs
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Provided Methods
§
1.0.0
·
Source
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Implementors
§
Source
§
impl
PartialEq
for
AsciiChar
1.65.0
·
Source
§
impl
PartialEq
for
BacktraceStatus
Source
§
impl
PartialEq
for
TryReserveErrorKind
1.34.0
·
Source
§
impl
PartialEq
for
Infallible
1.0.0
·
Source
§
impl
PartialEq
for
VarError
1.64.0
·
Source
§
impl
PartialEq
for
FromBytesWithNulError
1.28.0
·
Source
§
impl
PartialEq
for std::fmt::
Alignment
Source
§
impl
PartialEq
for
DebugAsHex
Source
§
impl
PartialEq
for
Sign
1.0.0
·
Source
§
impl
PartialEq
for
ErrorKind
1.0.0
·
Source
§
impl
PartialEq
for
SeekFrom
1.7.0
·
Source
§
impl
PartialEq
for
IpAddr
Source
§
impl
PartialEq
for
Ipv6MulticastScope
1.0.0
·
Source
§
impl
PartialEq
for
Shutdown
1.0.0
·
Source
§
impl
PartialEq
for
SocketAddr
1.0.0
·
Source
§
impl
PartialEq
for
FpCategory
1.55.0
·
Source
§
impl
PartialEq
for
IntErrorKind
Source
§
impl
PartialEq
for
BacktraceStyle
1.86.0
·
Source
§
impl
PartialEq
for
GetDisjointMutError
Source
§
impl
PartialEq
for
SearchStep
1.0.0
·
Source
§
impl
PartialEq
for std::sync::atomic::
Ordering
1.12.0
·
Source
§
impl
PartialEq
for
RecvTimeoutError
1.0.0
·
Source
§
impl
PartialEq
for
TryRecvError
1.0.0
·
Source
§
impl
PartialEq
for std::cmp::
Ordering
1.0.0
·
Source
§
impl
PartialEq
for
bool
1.0.0
·
Source
§
impl
PartialEq
for
char
1.0.0
·
Source
§
impl
PartialEq
for
f16
1.0.0
·
Source
§
impl
PartialEq
for
f32
1.0.0
·
Source
§
impl
PartialEq
for
f64
1.0.0
·
Source
§
impl
PartialEq
for
f128
1.0.0
·
Source
§
impl
PartialEq
for
i8
1.0.0
·
Source
§
impl
PartialEq
for
i16
1.0.0
·
Source
§
impl
PartialEq
for
i32
1.0.0
·
Source
§
impl
PartialEq
for
i64
1.0.0
·
Source
§
impl
PartialEq
for
i128
1.0.0
·
Source
§
impl
PartialEq
for
isize
Source
§
impl
PartialEq
for
!
1.0.0
·
Source
§
impl
PartialEq
for
str
1.0.0
·
Source
§
impl
PartialEq
for
u8
1.0.0
·
Source
§
impl
PartialEq
for
u16
1.0.0
·
Source
§
impl
PartialEq
for
u32
1.0.0
·
Source
§
impl
PartialEq
for
u64
1.0.0
·
Source
§
impl
PartialEq
for
u128
1.0.0
·
Source
§
impl
PartialEq
for
()
1.0.0
·
Source
§
impl
PartialEq
for
usize
Source
§
impl
PartialEq
for
AllocError
1.28.0
·
Source
§
impl
PartialEq
for
Layout
1.50.0
·
Source
§
impl
PartialEq
for
LayoutError
1.0.0
·
Source
§
impl
PartialEq
for
TypeId
Source
§
impl
PartialEq
for
ByteStr
Source
§
impl
PartialEq
for
ByteString
1.34.0
·
Source
§
impl
PartialEq
for
CharTryFromError
1.9.0
·
Source
§
impl
PartialEq
for
DecodeUtf16Error
1.20.0
·
Source
§
impl
PartialEq
for
ParseCharError
1.59.0
·
Source
§
impl
PartialEq
for
TryFromCharError
Source
§
impl
PartialEq
for
UnorderedKeyError
1.57.0
·
Source
§
impl
PartialEq
for
TryReserveError
1.64.0
·
Source
§
impl
PartialEq
for
CStr
1.64.0
·
Source
§
impl
PartialEq
for
CString
1.69.0
·
Source
§
impl
PartialEq
for
FromBytesUntilNulError
1.64.0
·
Source
§
impl
PartialEq
for
FromVecWithNulError
1.64.0
·
Source
§
impl
PartialEq
for
IntoStringError
1.64.0
·
Source
§
impl
PartialEq
for
NulError
1.0.0
·
Source
§
impl
PartialEq
for
OsStr
1.0.0
·
Source
§
impl
PartialEq
for
OsString
1.0.0
·
Source
§
impl
PartialEq
for
Error
Source
§
impl
PartialEq
for
FormattingOptions
1.1.0
·
Source
§
impl
PartialEq
for
FileType
1.0.0
·
Source
§
impl
PartialEq
for
Permissions
1.33.0
·
Source
§
impl
PartialEq
for
PhantomPinned
Source
§
impl
PartialEq
for
Assume
1.0.0
·
Source
§
impl
PartialEq
for
AddrParseError
1.0.0
·
Source
§
impl
PartialEq
for
Ipv4Addr
1.0.0
·
Source
§
impl
PartialEq
for
Ipv6Addr
1.0.0
·
Source
§
impl
PartialEq
for
SocketAddrV4
1.0.0
·
Source
§
impl
PartialEq
for
SocketAddrV6
1.0.0
·
Source
§
impl
PartialEq
for
ParseFloatError
1.0.0
·
Source
§
impl
PartialEq
for
ParseIntError
1.34.0
·
Source
§
impl
PartialEq
for
TryFromIntError
1.0.0
·
Source
§
impl
PartialEq
for
RangeFull
Source
§
impl
PartialEq
for
UCred
Available on
Unix
only.
1.63.0
·
Source
§
impl
PartialEq
for
InvalidHandleError
Available on
Windows
only.
1.63.0
·
Source
§
impl
PartialEq
for
NullHandleError
Available on
Windows
only.
1.0.0
·
Source
§
impl
PartialEq
for
Path
1.0.0
·
Source
§
impl
PartialEq
for
PathBuf
1.7.0
·
Source
§
impl
PartialEq
for
StripPrefixError
1.61.0
·
Source
§
impl
PartialEq
for
ExitCode
1.0.0
·
Source
§
impl
PartialEq
for
ExitStatus
Source
§
impl
PartialEq
for
ExitStatusError
1.0.0
·
Source
§
impl
PartialEq
for
Output
Source
§
impl
PartialEq
for std::ptr::
Alignment
1.0.0
·
Source
§
impl
PartialEq
for
ParseBoolError
1.0.0
·
Source
§
impl
PartialEq
for
Utf8Error
1.0.0
·
Source
§
impl
PartialEq
for
FromUtf8Error
1.0.0
·
Source
§
impl
PartialEq
for
String
1.0.0
·
Source
§
impl
PartialEq
for
RecvError
1.5.0
·
Source
§
impl
PartialEq
for
WaitTimeoutResult
1.36.0
·
Source
§
impl
PartialEq
for
RawWaker
1.36.0
·
Source
§
impl
PartialEq
for
RawWakerVTable
1.26.0
·
Source
§
impl
PartialEq
for
AccessError
1.19.0
·
Source
§
impl
PartialEq
for
ThreadId
1.3.0
·
Source
§
impl
PartialEq
for
Duration
1.8.0
·
Source
§
impl
PartialEq
for
Instant
1.8.0
·
Source
§
impl
PartialEq
for
SystemTime
1.66.0
·
Source
§
impl
PartialEq
for
TryFromFloatSecsError
1.29.0
·
Source
§
impl
PartialEq
<&
str
> for
OsString
1.16.0
·
Source
§
impl
PartialEq
<
IpAddr
> for
Ipv4Addr
1.16.0
·
Source
§
impl
PartialEq
<
IpAddr
> for
Ipv6Addr
1.0.0
·
Source
§
impl
PartialEq
<
str
> for
OsStr
1.0.0
·
Source
§
impl
PartialEq
<
str
> for
OsString
1.0.0
·
Source
§
impl
PartialEq
<
OsStr
> for
str
1.8.0
·
Source
§
impl
PartialEq
<
OsStr
> for
Path
1.8.0
·
Source
§
impl
PartialEq
<
OsStr
> for
PathBuf
1.0.0
·
Source
§
impl
PartialEq
<
OsString
> for
str
1.8.0
·
Source
§
impl
PartialEq
<
OsString
> for
Path
1.8.0
·
Source
§
impl
PartialEq
<
OsString
> for
PathBuf
1.16.0
·
Source
§
impl
PartialEq
<
Ipv4Addr
> for
IpAddr
1.16.0
·
Source
§
impl
PartialEq
<
Ipv6Addr
> for
IpAddr
1.8.0
·
Source
§
impl
PartialEq
<
Path
> for
OsStr
1.8.0
·
Source
§
impl
PartialEq
<
Path
> for
OsString
1.6.0
·
Source
§
impl
PartialEq
<
Path
> for
PathBuf
1.8.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
OsStr
1.8.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
OsString
1.6.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
Path
1.0.0
·
Source
§
impl<'a>
PartialEq
for
Component
<'a>
1.0.0
·
Source
§
impl<'a>
PartialEq
for
Prefix
<'a>
Source
§
impl<'a>
PartialEq
for
Utf8Pattern
<'a>
Source
§
impl<'a>
PartialEq
for
PhantomContravariantLifetime
<'a>
Source
§
impl<'a>
PartialEq
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
PartialEq
for
PhantomInvariantLifetime
<'a>
1.10.0
·
Source
§
impl<'a>
PartialEq
for
Location
<'a>
1.0.0
·
Source
§
impl<'a>
PartialEq
for
Components
<'a>
1.0.0
·
Source
§
impl<'a>
PartialEq
for
PrefixComponent
<'a>
1.79.0
·
Source
§
impl<'a>
PartialEq
for
Utf8Chunk
<'a>
Source
§
impl<'a>
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
<&'a
OsStr
> for
Path
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
OsStr
> for
PathBuf
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
Path
> for
OsStr
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
Path
> for
OsString
1.6.0
·
Source
§
impl<'a>
PartialEq
<&'a
Path
> for
PathBuf
Source
§
impl<'a>
PartialEq
<&
str
> for
ByteStr
Source
§
impl<'a>
PartialEq
<&
str
> for
ByteString
Source
§
impl<'a>
PartialEq
<&
ByteStr
> for
ByteString
Source
§
impl<'a>
PartialEq
<&[
u8
]> for
ByteStr
Source
§
impl<'a>
PartialEq
<&[
u8
]> for
ByteString
Source
§
impl<'a>
PartialEq
<
Cow
<'_,
str
>> for
ByteString
Source
§
impl<'a>
PartialEq
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
impl<'a>
PartialEq
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
str
>> for &'a
ByteStr
Source
§
impl<'a>
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
<
Cow
<'a,
Path
>> for
OsString
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
Path
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
impl<'a>
PartialEq
<
Cow
<'a, [
u8
]>> for &'a
ByteStr
Source
§
impl<'a>
PartialEq
<
str
> for
ByteStr
Source
§
impl<'a>
PartialEq
<
str
> for
ByteString
Source
§
impl<'a>
PartialEq
<
ByteStr
> for &
str
Source
§
impl<'a>
PartialEq
<
ByteStr
> for &[
u8
]
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
str
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
ByteString
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
String
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
Vec
<
u8
>
Source
§
impl<'a>
PartialEq
<
ByteStr
> for [
u8
]
Source
§
impl<'a>
PartialEq
<
ByteString
> for &
str
Source
§
impl<'a>
PartialEq
<
ByteString
> for &
ByteStr
Source
§
impl<'a>
PartialEq
<
ByteString
> for &[
u8
]
Source
§
impl<'a>
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
<
ByteString
> for
str
Source
§
impl<'a>
PartialEq
<
ByteString
> for
ByteStr
Source
§
impl<'a>
PartialEq
<
ByteString
> for
String
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Vec
<
u8
>
Source
§
impl<'a>
PartialEq
<
ByteString
> for [
u8
]
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsStr
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsStr
> for
Cow
<'a,
Path
>
1.29.0
·
Source
§
impl<'a>
PartialEq
<
OsString
> for &'a
str
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsString
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialEq
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
PartialEq
<
Path
> for &'a
OsStr
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Path
> for
Cow
<'a,
OsStr
>
1.6.0
·
Source
§
impl<'a>
PartialEq
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
PartialEq
<
PathBuf
> for &'a
OsStr
1.6.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for &'a
Path
1.8.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
OsStr
>
1.6.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
impl<'a>
PartialEq
<
String
> for
ByteStr
Source
§
impl<'a>
PartialEq
<
String
> for
ByteString
Source
§
impl<'a>
PartialEq
<
Vec
<
u8
>> for
ByteStr
Source
§
impl<'a>
PartialEq
<
Vec
<
u8
>> for
ByteString
Source
§
impl<'a>
PartialEq
<[
u8
]> for
ByteStr
Source
§
impl<'a>
PartialEq
<[
u8
]> for
ByteString
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'a
str
> for
String
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'a
OsStr
> for
OsString
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'a
Path
> for
Cow
<'b,
OsStr
>
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
str
> for
Cow
<'a,
str
>
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
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
PartialEq
<&'b
OsStr
> for
Cow
<'a,
Path
>
1.6.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
Path
> for
Cow
<'a,
Path
>
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for &'b
str
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for
str
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for
String
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
<
Cow
<'a,
Path
>> for &'b
OsStr
1.6.0
·
Source
§
impl<'a, 'b>
PartialEq
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
PartialEq
<
Cow
<'b,
OsStr
>> for &'a
Path
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
str
> for
Cow
<'a,
str
>
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
str
> for
String
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
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
PartialEq
<
OsStr
> for
OsString
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsString
> for &'a
OsStr
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
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
PartialEq
<
OsString
> for
OsStr
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
String
> for &'a
str
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
String
> for
Cow
<'a,
str
>
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
String
> for
str
1.0.0
·
Source
§
impl<'a, 'b, B, C>
PartialEq
<
Cow
<'b, C>> for
Cow
<'a, B>
where
    B:
PartialEq
<C> +
ToOwned
+ ?
Sized
,
    C:
ToOwned
+ ?
Sized
,
1.0.0
·
Source
§
impl<A, B>
PartialEq
<
&B
> for
&A
where
    A:
PartialEq
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
PartialEq
<
&B
> for
&mut A
where
    A:
PartialEq
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
PartialEq
<
&mut B
> for
&A
where
    A:
PartialEq
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
PartialEq
<
&mut B
> for
&mut A
where
    A:
PartialEq
<B> + ?
Sized
,
    B: ?
Sized
,
1.55.0
·
Source
§
impl<B, C>
PartialEq
for
ControlFlow
<B, C>
where
    B:
PartialEq
,
    C:
PartialEq
,
Source
§
impl<Dyn>
PartialEq
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
PartialEq
for F
where
    F:
FnPtr
,
1.29.0
·
Source
§
impl<H>
PartialEq
for
BuildHasherDefault
<H>
1.0.0
·
Source
§
impl<Idx>
PartialEq
for std::ops::
Range
<Idx>
where
    Idx:
PartialEq
,
1.0.0
·
Source
§
impl<Idx>
PartialEq
for std::ops::
RangeFrom
<Idx>
where
    Idx:
PartialEq
,
1.26.0
·
Source
§
impl<Idx>
PartialEq
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
PartialEq
,
1.0.0
·
Source
§
impl<Idx>
PartialEq
for
RangeTo
<Idx>
where
    Idx:
PartialEq
,
1.26.0
·
Source
§
impl<Idx>
PartialEq
for
RangeToInclusive
<Idx>
where
    Idx:
PartialEq
,
Source
§
impl<Idx>
PartialEq
for std::range::
Range
<Idx>
where
    Idx:
PartialEq
,
Source
§
impl<Idx>
PartialEq
for std::range::
RangeFrom
<Idx>
where
    Idx:
PartialEq
,
Source
§
impl<Idx>
PartialEq
for std::range::
RangeInclusive
<Idx>
where
    Idx:
PartialEq
,
1.0.0
·
Source
§
impl<K, V, A>
PartialEq
for
BTreeMap
<K, V, A>
where
    K:
PartialEq
,
    V:
PartialEq
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
impl<K, V, S>
PartialEq
for
HashMap
<K, V, S>
where
    K:
Eq
+
Hash
,
    V:
PartialEq
,
    S:
BuildHasher
,
1.41.0
·
Source
§
impl<Ptr, Q>
PartialEq
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
PartialEq
<<Q as
Deref
>::
Target
>,
1.17.0
·
Source
§
impl<T>
PartialEq
for
Bound
<T>
where
    T:
PartialEq
,
1.0.0
·
Source
§
impl<T>
PartialEq
for
Option
<T>
where
    T:
PartialEq
,
1.36.0
·
Source
§
impl<T>
PartialEq
for
Poll
<T>
where
    T:
PartialEq
,
1.0.0
·
Source
§
impl<T>
PartialEq
for
*const T
where
    T: ?
Sized
,
Pointer equality is by address, as produced by the
<*const T>::addr
method.
1.0.0
·
Source
§
impl<T>
PartialEq
for
*mut T
where
    T: ?
Sized
,
Pointer equality is by address, as produced by the
<*mut T>::addr
method.
1.0.0
·
Source
§
impl<T>
PartialEq
for
(T₁, T₂, …, Tₙ)
where
    T:
PartialEq
+ ?
Sized
,
This trait is implemented for tuples up to twelve items long.
1.0.0
·
Source
§
impl<T>
PartialEq
for
Cell
<T>
where
    T:
PartialEq
+
Copy
,
1.70.0
·
Source
§
impl<T>
PartialEq
for
OnceCell
<T>
where
    T:
PartialEq
,
1.0.0
·
Source
§
impl<T>
PartialEq
for
RefCell
<T>
where
    T:
PartialEq
+ ?
Sized
,
Source
§
impl<T>
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
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
PartialEq
for
Discriminant
<T>
1.20.0
·
Source
§
impl<T>
PartialEq
for
ManuallyDrop
<T>
where
    T:
PartialEq
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
PartialEq
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
PartialEq
,
1.74.0
·
Source
§
impl<T>
PartialEq
for
Saturating
<T>
where
    T:
PartialEq
,
1.0.0
·
Source
§
impl<T>
PartialEq
for
Wrapping
<T>
where
    T:
PartialEq
,
1.25.0
·
Source
§
impl<T>
PartialEq
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
PartialEq
for
Reverse
<T>
where
    T:
PartialEq
,
1.0.0
·
Source
§
impl<T, A>
PartialEq
for
Box
<T, A>
where
    T:
PartialEq
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
PartialEq
for
BTreeSet
<T, A>
where
    T:
PartialEq
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
PartialEq
for
LinkedList
<T, A>
where
    T:
PartialEq
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
PartialEq
for
VecDeque
<T, A>
where
    T:
PartialEq
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
PartialEq
for
Rc
<T, A>
where
    T:
PartialEq
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
PartialEq
for
UniqueRc
<T, A>
where
    T:
PartialEq
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
PartialEq
for
Arc
<T, A>
where
    T:
PartialEq
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
PartialEq
for
Result
<T, E>
where
    T:
PartialEq
,
    E:
PartialEq
,
1.0.0
·
Source
§
impl<T, S>
PartialEq
for
HashSet
<T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
,
1.0.0
·
Source
§
impl<T, U>
PartialEq
<&
[U]
> for
Cow
<'_,
[T]
>
where
    T:
PartialEq
<U> +
Clone
,
1.0.0
·
Source
§
impl<T, U>
PartialEq
<&mut
[U]
> for
Cow
<'_,
[T]
>
where
    T:
PartialEq
<U> +
Clone
,
1.0.0
·
Source
§
impl<T, U>
PartialEq
<
[U]
> for
[T]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A1, A2>
PartialEq
<
Vec
<U, A2>> for
Vec
<T, A1>
where
    A1:
Allocator
,
    A2:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<&
[U]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A>
PartialEq
<&
[U]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<&mut
[U]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A>
PartialEq
<&mut
[U]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.48.0
·
Source
§
impl<T, U, A>
PartialEq
<
[U]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.46.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for &
[T]
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.46.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for &mut
[T]
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for
Cow
<'_,
[T]
>
where
    A:
Allocator
,
    T:
PartialEq
<U> +
Clone
,
1.48.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for
[T]
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&
[U; N]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&mut
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<
[U; N]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<&
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<&mut
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for &
[T]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for &mut
[T]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for
[T; N]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for
[T]
where
    T:
PartialEq
<U>,
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
Source
§
impl<T, const N:
usize
>
PartialEq
for
Mask
<T, N>
where
    T:
MaskElement
+
PartialEq
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
PartialEq
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
PartialEq
,
Source
§
impl<T:
PartialEq
>
PartialEq
for
SendTimeoutError
<T>
1.0.0
·
Source
§
impl<T:
PartialEq
>
PartialEq
for
TrySendError
<T>
1.0.0
·
Source
§
impl<T:
PartialEq
>
PartialEq
for
Cursor
<T>
1.0.0
·
Source
§
impl<T:
PartialEq
>
PartialEq
for
SendError
<T>
1.70.0
·
Source
§
impl<T:
PartialEq
>
PartialEq
for
OnceLock
<T>
Source
§
impl<Y, R>
PartialEq
for
CoroutineState
<Y, R>
where
    Y:
PartialEq
,
    R:
PartialEq
,
Source
§
impl<const N:
usize
>
PartialEq
<&[
u8
;
N
]> for
ByteStr
Source
§
impl<const N:
usize
>
PartialEq
<&[
u8
;
N
]> for
ByteString
Source
§
impl<const N:
usize
>
PartialEq
<
ByteStr
> for &[
u8
;
N
]
Source
§
impl<const N:
usize
>
PartialEq
<
ByteStr
> for [
u8
;
N
]
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for &[
u8
;
N
]
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for [
u8
;
N
]
Source
§
impl<const N:
usize
>
PartialEq
<[
u8
;
N
]> for
ByteStr
Source
§
impl<const N:
usize
>
PartialEq
<[
u8
;
N
]> for
ByteString