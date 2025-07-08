From in std::convert - Rust
std
::
convert
Trait
From
Copy item path
1.0.0
·
Source
pub trait From<T>:
Sized
{
    // Required method
    fn
from
(value: T) -> Self;
}
Expand description
Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
Into
.
One should always prefer implementing
From
over
Into
because implementing
From
automatically provides one with an implementation of
Into
thanks to the blanket implementation in the standard library.
Only implement
Into
when targeting a version prior to Rust 1.41 and converting to a type
outside the current crate.
From
was not able to do these types of conversions in earlier versions because of Rust’s
orphaning rules.
See
Into
for more details.
Prefer using
Into
over using
From
when specifying trait bounds on a generic function.
This way, types that directly implement
Into
can be used as arguments as well.
The
From
trait is also very useful when performing error handling. When constructing a function
that is capable of failing, the return type will generally be of the form
Result<T, E>
.
From
simplifies error handling by allowing a function to return a single error type
that encapsulates multiple error types. See the “Examples” section and
the book
for more
details.
Note: This trait must not fail
. The
From
trait is intended for perfect conversions.
If the conversion can fail or is not perfect, use
TryFrom
.
§
Generic Implementations
From<T> for U
implies
Into
<U> for T
From
is reflexive, which means that
From<T> for T
is implemented
§
When to implement
From
While there’s no technical restrictions on which conversions can be done using
a
From
implementation, the general expectation is that the conversions
should typically be restricted as follows:
The conversion is
infallible
: if the conversion can fail, use
TryFrom
instead; don’t provide a
From
impl that panics.
The conversion is
lossless
: semantically, it should not lose or discard
information. For example,
i32: From<u16>
exists, where the original
value can be recovered using
u16: TryFrom<i32>
.  And
String: From<&str>
exists, where you can get something equivalent to the original value via
Deref
.  But
From
cannot be used to convert from
u32
to
u16
, since
that cannot succeed in a lossless way.  (There’s some wiggle room here for
information not considered semantically relevant.  For example,
Box<[T]>: From<Vec<T>>
exists even though it might not preserve capacity,
like how two vectors can be equal despite differing capacities.)
The conversion is
value-preserving
: the conceptual kind and meaning of
the resulting value is the same, even though the Rust type and technical
representation might be different.  For example
-1_i8 as u8
is
lossless
,
since
as
casting back can recover the original value, but that conversion
is
not
available via
From
because
-1
and
255
are different conceptual
values (despite being identical bit patterns technically).  But
f32: From<i16>
is
available because
1_i16
and
1.0_f32
are conceptually
the same real number (despite having very different bit patterns technically).
String: From<char>
is available because they’re both
text
, but
String: From<u32>
is
not
available, since
1
(a number) and
"1"
(text) are too different.  (Converting values to text is instead covered
by the
Display
trait.)
The conversion is
obvious
: it’s the only reasonable conversion between
the two types.  Otherwise it’s better to have it be a named method or
constructor, like how
str::as_bytes
is a method and how integers have
methods like
u32::from_ne_bytes
,
u32::from_le_bytes
, and
u32::from_be_bytes
, none of which are
From
implementations.  Whereas
there’s only one reasonable way to wrap an
Ipv6Addr
into an
IpAddr
, thus
IpAddr: From<Ipv6Addr>
exists.
§
Examples
String
implements
From<&str>
:
An explicit conversion from a
&str
to a String is done as follows:
let
string =
"hello"
.to_string();
let
other_string = String::from(
"hello"
);
assert_eq!
(string, other_string);
While performing error handling it is often useful to implement
From
for your own error type.
By converting underlying error types to our own custom error type that encapsulates the
underlying error type, we can return a single error type without losing information on the
underlying cause. The ‘?’ operator automatically converts the underlying error type to our
custom error type with
From::from
.
use
std::fs;
use
std::io;
use
std::num;
enum
CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}
impl
From<io::Error>
for
CliError {
fn
from(error: io::Error) ->
Self
{
        CliError::IoError(error)
    }
}
impl
From<num::ParseIntError>
for
CliError {
fn
from(error: num::ParseIntError) ->
Self
{
        CliError::ParseError(error)
    }
}
fn
open_and_parse_file(file_name:
&
str) ->
Result
<i32, CliError> {
let
mut
contents = fs::read_to_string(
&
file_name)
?
;
let
num: i32 = contents.trim().parse()
?
;
Ok
(num)
}
Required Methods
§
1.0.0
·
Source
fn
from
(value: T) -> Self
Converts to this type from the input type.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.17.0
·
Source
§
impl
From
<&
str
> for
Box
<
str
>
1.21.0
·
Source
§
impl
From
<&
str
> for
Rc
<
str
>
1.0.0
·
Source
§
impl
From
<&
str
> for
String
1.21.0
·
Source
§
impl
From
<&
str
> for
Arc
<
str
>
1.0.0
·
Source
§
impl
From
<&
str
> for
Vec
<
u8
>
1.17.0
·
Source
§
impl
From
<&
CStr
> for
Box
<
CStr
>
1.7.0
·
Source
§
impl
From
<&
CStr
> for
CString
1.24.0
·
Source
§
impl
From
<&
CStr
> for
Rc
<
CStr
>
1.24.0
·
Source
§
impl
From
<&
CStr
> for
Arc
<
CStr
>
1.17.0
·
Source
§
impl
From
<&
OsStr
> for
Box
<
OsStr
>
1.24.0
·
Source
§
impl
From
<&
OsStr
> for
Rc
<
OsStr
>
1.24.0
·
Source
§
impl
From
<&
OsStr
> for
Arc
<
OsStr
>
1.17.0
·
Source
§
impl
From
<&
Path
> for
Box
<
Path
>
1.24.0
·
Source
§
impl
From
<&
Path
> for
Rc
<
Path
>
1.24.0
·
Source
§
impl
From
<&
Path
> for
Arc
<
Path
>
1.35.0
·
Source
§
impl
From
<&
String
> for
String
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Box
<
str
>
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Rc
<
str
>
1.44.0
·
Source
§
impl
From
<&mut
str
> for
String
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Arc
<
str
>
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Box
<
CStr
>
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Rc
<
CStr
>
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Arc
<
CStr
>
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Box
<
OsStr
>
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Rc
<
OsStr
>
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Arc
<
OsStr
>
1.84.0
·
Source
§
impl
From
<&mut
Path
> for
Box
<
Path
>
1.84.0
·
Source
§
impl
From
<&mut
Path
> for
Rc
<
Path
>
1.84.0
·
Source
§
impl
From
<&mut
Path
> for
Arc
<
Path
>
Source
§
impl
From
<
AsciiChar
> for
char
Source
§
impl
From
<
AsciiChar
> for
u8
Source
§
impl
From
<
AsciiChar
> for
u16
Source
§
impl
From
<
AsciiChar
> for
u32
Source
§
impl
From
<
AsciiChar
> for
u64
Source
§
impl
From
<
AsciiChar
> for
u128
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
str
>> for
Box
<
str
>
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
CStr
>> for
Box
<
CStr
>
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
OsStr
>> for
Box
<
OsStr
>
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
Path
>> for
Box
<
Path
>
Source
§
impl
From
<
TryReserveErrorKind
> for
TryReserveError
1.14.0
·
Source
§
impl
From
<
ErrorKind
> for
Error
Intended for use for errors not exposed to the user, where allocating onto
the heap (for normal construction via Error::new) is too costly.
1.36.0
·
Source
§
impl
From
<
Infallible
> for
TryFromSliceError
1.34.0
·
Source
§
impl
From
<
Infallible
> for
TryFromIntError
1.68.0
·
Source
§
impl
From
<
bool
> for
f16
1.68.0
·
Source
§
impl
From
<
bool
> for
f32
1.68.0
·
Source
§
impl
From
<
bool
> for
f64
1.68.0
·
Source
§
impl
From
<
bool
> for
f128
1.28.0
·
Source
§
impl
From
<
bool
> for
i8
1.28.0
·
Source
§
impl
From
<
bool
> for
i16
1.28.0
·
Source
§
impl
From
<
bool
> for
i32
1.28.0
·
Source
§
impl
From
<
bool
> for
i64
1.28.0
·
Source
§
impl
From
<
bool
> for
i128
1.28.0
·
Source
§
impl
From
<
bool
> for
isize
1.28.0
·
Source
§
impl
From
<
bool
> for
u8
1.28.0
·
Source
§
impl
From
<
bool
> for
u16
1.28.0
·
Source
§
impl
From
<
bool
> for
u32
1.28.0
·
Source
§
impl
From
<
bool
> for
u64
1.28.0
·
Source
§
impl
From
<
bool
> for
u128
1.28.0
·
Source
§
impl
From
<
bool
> for
usize
1.24.0
·
Source
§
impl
From
<
bool
> for
AtomicBool
1.13.0
·
Source
§
impl
From
<
char
> for
u32
1.51.0
·
Source
§
impl
From
<
char
> for
u64
1.51.0
·
Source
§
impl
From
<
char
> for
u128
1.46.0
·
Source
§
impl
From
<
char
> for
String
1.6.0
·
Source
§
impl
From
<
f16
> for
f64
1.6.0
·
Source
§
impl
From
<
f16
> for
f128
1.6.0
·
Source
§
impl
From
<
f32
> for
f64
1.6.0
·
Source
§
impl
From
<
f32
> for
f128
1.6.0
·
Source
§
impl
From
<
f64
> for
f128
1.6.0
·
Source
§
impl
From
<
i8
> for
f16
1.6.0
·
Source
§
impl
From
<
i8
> for
f32
1.6.0
·
Source
§
impl
From
<
i8
> for
f64
1.6.0
·
Source
§
impl
From
<
i8
> for
f128
1.5.0
·
Source
§
impl
From
<
i8
> for
i16
1.5.0
·
Source
§
impl
From
<
i8
> for
i32
1.5.0
·
Source
§
impl
From
<
i8
> for
i64
1.26.0
·
Source
§
impl
From
<
i8
> for
i128
1.5.0
·
Source
§
impl
From
<
i8
> for
isize
1.34.0
·
Source
§
impl
From
<
i8
> for
AtomicI8
1.6.0
·
Source
§
impl
From
<
i16
> for
f32
1.6.0
·
Source
§
impl
From
<
i16
> for
f64
1.6.0
·
Source
§
impl
From
<
i16
> for
f128
1.5.0
·
Source
§
impl
From
<
i16
> for
i32
1.5.0
·
Source
§
impl
From
<
i16
> for
i64
1.26.0
·
Source
§
impl
From
<
i16
> for
i128
1.26.0
·
Source
§
impl
From
<
i16
> for
isize
1.34.0
·
Source
§
impl
From
<
i16
> for
AtomicI16
1.6.0
·
Source
§
impl
From
<
i32
> for
f64
1.6.0
·
Source
§
impl
From
<
i32
> for
f128
1.5.0
·
Source
§
impl
From
<
i32
> for
i64
1.26.0
·
Source
§
impl
From
<
i32
> for
i128
1.34.0
·
Source
§
impl
From
<
i32
> for
AtomicI32
1.26.0
·
Source
§
impl
From
<
i64
> for
i128
1.34.0
·
Source
§
impl
From
<
i64
> for
AtomicI64
Source
§
impl
From
<
i128
> for
AtomicI128
1.23.0
·
Source
§
impl
From
<
isize
> for
AtomicIsize
1.34.0
·
Source
§
impl
From
<
!
> for
Infallible
Source
§
impl
From
<
!
> for
TryFromIntError
1.13.0
·
Source
§
impl
From
<
u8
> for
char
Maps a byte in 0x00..=0xFF to a
char
whose code point has the same value, in U+0000..=U+00FF.
Unicode is designed such that this effectively decodes bytes
with the character encoding that IANA calls ISO-8859-1.
This encoding is compatible with ASCII.
Note that this is different from ISO/IEC 8859-1 a.k.a. ISO 8859-1 (with one less hyphen),
which leaves some “blanks”, byte values that are not assigned to any character.
ISO-8859-1 (the IANA one) assigns them to the C0 and C1 control codes.
Note that this is
also
different from Windows-1252 a.k.a. code page 1252,
which is a superset ISO/IEC 8859-1 that assigns some (not all!) blanks
to punctuation and various Latin characters.
To confuse things further,
on the Web
ascii
,
iso-8859-1
, and
windows-1252
are all aliases
for a superset of Windows-1252 that fills the remaining blanks with corresponding
C0 and C1 control codes.
1.6.0
·
Source
§
impl
From
<
u8
> for
f16
1.6.0
·
Source
§
impl
From
<
u8
> for
f32
1.6.0
·
Source
§
impl
From
<
u8
> for
f64
1.6.0
·
Source
§
impl
From
<
u8
> for
f128
1.5.0
·
Source
§
impl
From
<
u8
> for
i16
1.5.0
·
Source
§
impl
From
<
u8
> for
i32
1.5.0
·
Source
§
impl
From
<
u8
> for
i64
1.26.0
·
Source
§
impl
From
<
u8
> for
i128
1.26.0
·
Source
§
impl
From
<
u8
> for
isize
1.5.0
·
Source
§
impl
From
<
u8
> for
u16
1.5.0
·
Source
§
impl
From
<
u8
> for
u32
1.5.0
·
Source
§
impl
From
<
u8
> for
u64
1.26.0
·
Source
§
impl
From
<
u8
> for
u128
1.5.0
·
Source
§
impl
From
<
u8
> for
usize
1.61.0
·
Source
§
impl
From
<
u8
> for
ExitCode
1.34.0
·
Source
§
impl
From
<
u8
> for
AtomicU8
1.6.0
·
Source
§
impl
From
<
u16
> for
f16
1.6.0
·
Source
§
impl
From
<
u16
> for
f32
1.6.0
·
Source
§
impl
From
<
u16
> for
f64
1.6.0
·
Source
§
impl
From
<
u16
> for
f128
1.5.0
·
Source
§
impl
From
<
u16
> for
i32
1.5.0
·
Source
§
impl
From
<
u16
> for
i64
1.26.0
·
Source
§
impl
From
<
u16
> for
i128
1.5.0
·
Source
§
impl
From
<
u16
> for
u32
1.5.0
·
Source
§
impl
From
<
u16
> for
u64
1.26.0
·
Source
§
impl
From
<
u16
> for
u128
1.26.0
·
Source
§
impl
From
<
u16
> for
usize
1.34.0
·
Source
§
impl
From
<
u16
> for
AtomicU16
1.6.0
·
Source
§
impl
From
<
u32
> for
f64
1.6.0
·
Source
§
impl
From
<
u32
> for
f128
1.5.0
·
Source
§
impl
From
<
u32
> for
i64
1.26.0
·
Source
§
impl
From
<
u32
> for
i128
1.5.0
·
Source
§
impl
From
<
u32
> for
u64
1.26.0
·
Source
§
impl
From
<
u32
> for
u128
1.1.0
·
Source
§
impl
From
<
u32
> for
Ipv4Addr
1.34.0
·
Source
§
impl
From
<
u32
> for
AtomicU32
1.26.0
·
Source
§
impl
From
<
u64
> for
i128
1.26.0
·
Source
§
impl
From
<
u64
> for
u128
1.34.0
·
Source
§
impl
From
<
u64
> for
AtomicU64
1.26.0
·
Source
§
impl
From
<
u128
> for
Ipv6Addr
Source
§
impl
From
<
u128
> for
AtomicU128
1.23.0
·
Source
§
impl
From
<
usize
> for
AtomicUsize
Source
§
impl
From
<
float64x1_t
> for
Simd
<
f64
, 1>
Source
§
impl
From
<
float64x2_t
> for
Simd
<
f64
, 2>
Source
§
impl
From
<
float32x2_t
> for
Simd
<
f32
, 2>
Source
§
impl
From
<
float32x4_t
> for
Simd
<
f32
, 4>
Source
§
impl
From
<
int8x8_t
> for
Simd
<
i8
, 8>
Source
§
impl
From
<
int8x16_t
> for
Simd
<
i8
, 16>
Source
§
impl
From
<
int16x4_t
> for
Simd
<
i16
, 4>
Source
§
impl
From
<
int16x8_t
> for
Simd
<
i16
, 8>
Source
§
impl
From
<
int32x2_t
> for
Simd
<
i32
, 2>
Source
§
impl
From
<
int32x4_t
> for
Simd
<
i32
, 4>
Source
§
impl
From
<
int64x1_t
> for
Simd
<
i64
, 1>
Source
§
impl
From
<
int64x2_t
> for
Simd
<
i64
, 2>
Source
§
impl
From
<
poly8x8_t
> for
Simd
<
u8
, 8>
Source
§
impl
From
<
poly8x16_t
> for
Simd
<
u8
, 16>
Source
§
impl
From
<
poly16x4_t
> for
Simd
<
u16
, 4>
Source
§
impl
From
<
poly16x8_t
> for
Simd
<
u16
, 8>
Source
§
impl
From
<
poly64x1_t
> for
Simd
<
u64
, 1>
Source
§
impl
From
<
poly64x2_t
> for
Simd
<
u64
, 2>
Source
§
impl
From
<
uint8x8_t
> for
Simd
<
u8
, 8>
Source
§
impl
From
<
uint8x16_t
> for
Simd
<
u8
, 16>
Source
§
impl
From
<
uint16x4_t
> for
Simd
<
u16
, 4>
Source
§
impl
From
<
uint16x8_t
> for
Simd
<
u16
, 8>
Source
§
impl
From
<
uint32x2_t
> for
Simd
<
u32
, 2>
Source
§
impl
From
<
uint32x4_t
> for
Simd
<
u32
, 4>
Source
§
impl
From
<
uint64x1_t
> for
Simd
<
u64
, 1>
Source
§
impl
From
<
uint64x2_t
> for
Simd
<
u64
, 2>
Source
§
impl
From
<
LayoutError
> for
TryReserveErrorKind
1.18.0
·
Source
§
impl
From
<
Box
<
str
>> for
String
Source
§
impl
From
<
Box
<
ByteStr
>> for
Box
<[
u8
]>
1.18.0
·
Source
§
impl
From
<
Box
<
CStr
>> for
CString
1.18.0
·
Source
§
impl
From
<
Box
<
OsStr
>> for
OsString
1.18.0
·
Source
§
impl
From
<
Box
<
Path
>> for
PathBuf
Source
§
impl
From
<
Box
<[
u8
]>> for
Box
<
ByteStr
>
Source
§
impl
From
<
ByteString
> for
Vec
<
u8
>
1.78.0
·
Source
§
impl
From
<
TryReserveError
> for
Error
1.20.0
·
Source
§
impl
From
<
CString
> for
Box
<
CStr
>
1.24.0
·
Source
§
impl
From
<
CString
> for
Rc
<
CStr
>
1.24.0
·
Source
§
impl
From
<
CString
> for
Arc
<
CStr
>
1.7.0
·
Source
§
impl
From
<
CString
> for
Vec
<
u8
>
1.0.0
·
Source
§
impl
From
<
NulError
> for
Error
1.20.0
·
Source
§
impl
From
<
OsString
> for
Box
<
OsStr
>
1.0.0
·
Source
§
impl
From
<
OsString
> for
PathBuf
1.24.0
·
Source
§
impl
From
<
OsString
> for
Rc
<
OsStr
>
1.24.0
·
Source
§
impl
From
<
OsString
> for
Arc
<
OsStr
>
1.63.0
·
Source
§
impl
From
<
File
> for
OwnedFd
1.63.0
·
Source
§
impl
From
<
File
> for
OwnedHandle
Available on
Windows
only.
1.20.0
·
Source
§
impl
From
<
File
> for
Stdio
1.87.0
·
Source
§
impl
From
<
PipeReader
> for
OwnedFd
1.87.0
·
Source
§
impl
From
<
PipeReader
> for
OwnedHandle
Available on
Windows
only.
1.87.0
·
Source
§
impl
From
<
PipeReader
> for
Stdio
1.87.0
·
Source
§
impl
From
<
PipeWriter
> for
OwnedFd
1.87.0
·
Source
§
impl
From
<
PipeWriter
> for
OwnedHandle
Available on
Windows
only.
1.87.0
·
Source
§
impl
From
<
PipeWriter
> for
Stdio
1.74.0
·
Source
§
impl
From
<
Stderr
> for
Stdio
1.74.0
·
Source
§
impl
From
<
Stdout
> for
Stdio
1.16.0
·
Source
§
impl
From
<
Ipv4Addr
> for
IpAddr
1.1.0
·
Source
§
impl
From
<
Ipv4Addr
> for
u32
1.16.0
·
Source
§
impl
From
<
Ipv6Addr
> for
IpAddr
1.26.0
·
Source
§
impl
From
<
Ipv6Addr
> for
u128
1.16.0
·
Source
§
impl
From
<
SocketAddrV4
> for
SocketAddr
1.16.0
·
Source
§
impl
From
<
SocketAddrV6
> for
SocketAddr
1.63.0
·
Source
§
impl
From
<
TcpListener
> for
OwnedFd
1.63.0
·
Source
§
impl
From
<
TcpListener
> for
OwnedSocket
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
TcpStream
> for
OwnedFd
1.63.0
·
Source
§
impl
From
<
TcpStream
> for
OwnedSocket
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
UdpSocket
> for
OwnedFd
1.63.0
·
Source
§
impl
From
<
UdpSocket
> for
OwnedSocket
Available on
Windows
only.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i16
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
isize
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
isize
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i32
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i32
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
i64
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i16
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
isize
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u16
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
usize
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u32
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
usize
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
i64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
u64
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
u128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u64
>> for
NonZero
<
i128
>
1.41.0
·
Source
§
impl
From
<
NonZero
<
u64
>> for
NonZero
<
u128
>
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
File
1.87.0
·
Source
§
impl
From
<
OwnedFd
> for
PipeReader
1.87.0
·
Source
§
impl
From
<
OwnedFd
> for
PipeWriter
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
TcpListener
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
TcpStream
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
UdpSocket
Source
§
impl
From
<
OwnedFd
> for
PidFd
Available on
Linux
only.
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
UnixDatagram
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
UnixListener
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
UnixStream
Available on
Unix
only.
1.74.0
·
Source
§
impl
From
<
OwnedFd
> for
ChildStderr
Available on
Unix
only.
Creates a
ChildStderr
from the provided
OwnedFd
.
The provided file descriptor must point to a pipe
with the
CLOEXEC
flag set.
1.74.0
·
Source
§
impl
From
<
OwnedFd
> for
ChildStdin
Available on
Unix
only.
Creates a
ChildStdin
from the provided
OwnedFd
.
The provided file descriptor must point to a pipe
with the
CLOEXEC
flag set.
1.74.0
·
Source
§
impl
From
<
OwnedFd
> for
ChildStdout
Available on
Unix
only.
Creates a
ChildStdout
from the provided
OwnedFd
.
The provided file descriptor must point to a pipe
with the
CLOEXEC
flag set.
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
Stdio
Available on
Unix
only.
Source
§
impl
From
<
PidFd
> for
OwnedFd
Available on
Linux
only.
1.63.0
·
Source
§
impl
From
<
UnixDatagram
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
UnixListener
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
UnixStream
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
OwnedHandle
> for
File
Available on
Windows
only.
1.87.0
·
Source
§
impl
From
<
OwnedHandle
> for
PipeReader
Available on
Windows
only.
1.87.0
·
Source
§
impl
From
<
OwnedHandle
> for
PipeWriter
Available on
Windows
only.
1.74.0
·
Source
§
impl
From
<
OwnedHandle
> for
ChildStderr
Available on
Windows
only.
Creates a
ChildStderr
from the provided
OwnedHandle
.
The provided handle must be asynchronous, as reading and
writing from and to it is implemented using asynchronous APIs.
1.74.0
·
Source
§
impl
From
<
OwnedHandle
> for
ChildStdin
Available on
Windows
only.
Creates a
ChildStdin
from the provided
OwnedHandle
.
The provided handle must be asynchronous, as reading and
writing from and to it is implemented using asynchronous APIs.
1.74.0
·
Source
§
impl
From
<
OwnedHandle
> for
ChildStdout
Available on
Windows
only.
Creates a
ChildStdout
from the provided
OwnedHandle
.
The provided handle must be asynchronous, as reading and
writing from and to it is implemented using asynchronous APIs.
1.63.0
·
Source
§
impl
From
<
OwnedHandle
> for
Stdio
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
OwnedSocket
> for
TcpListener
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
OwnedSocket
> for
TcpStream
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
OwnedSocket
> for
UdpSocket
Available on
Windows
only.
1.20.0
·
Source
§
impl
From
<
PathBuf
> for
Box
<
Path
>
1.14.0
·
Source
§
impl
From
<
PathBuf
> for
OsString
1.24.0
·
Source
§
impl
From
<
PathBuf
> for
Rc
<
Path
>
1.24.0
·
Source
§
impl
From
<
PathBuf
> for
Arc
<
Path
>
1.63.0
·
Source
§
impl
From
<
Child
> for
OwnedHandle
Available on
Windows
only.
1.63.0
·
Source
§
impl
From
<
ChildStderr
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
ChildStderr
> for
OwnedHandle
Available on
Windows
only.
1.20.0
·
Source
§
impl
From
<
ChildStderr
> for
Stdio
1.63.0
·
Source
§
impl
From
<
ChildStdin
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
ChildStdin
> for
OwnedHandle
Available on
Windows
only.
1.20.0
·
Source
§
impl
From
<
ChildStdin
> for
Stdio
1.63.0
·
Source
§
impl
From
<
ChildStdout
> for
OwnedFd
Available on
Unix
only.
1.63.0
·
Source
§
impl
From
<
ChildStdout
> for
OwnedHandle
Available on
Windows
only.
1.20.0
·
Source
§
impl
From
<
ChildStdout
> for
Stdio
Source
§
impl
From
<
ExitStatusError
> for
ExitStatus
Source
§
impl
From
<
Alignment
> for
usize
Source
§
impl
From
<
Alignment
> for
NonZero
<
usize
>
1.62.0
·
Source
§
impl
From
<
Rc
<
str
>> for
Rc
<[
u8
]>
Source
§
impl
From
<
Rc
<
ByteStr
>> for
Rc
<[
u8
]>
Source
§
impl
From
<
Rc
<[
u8
]>> for
Rc
<
ByteStr
>
Source
§
impl
From
<
Simd
<
f32
, 2>> for
float32x2_t
Source
§
impl
From
<
Simd
<
f32
, 4>> for
float32x4_t
Source
§
impl
From
<
Simd
<
f64
, 1>> for
float64x1_t
Source
§
impl
From
<
Simd
<
f64
, 2>> for
float64x2_t
Source
§
impl
From
<
Simd
<
i8
, 8>> for
int8x8_t
Source
§
impl
From
<
Simd
<
i8
, 16>> for
int8x16_t
Source
§
impl
From
<
Simd
<
i16
, 4>> for
int16x4_t
Source
§
impl
From
<
Simd
<
i16
, 8>> for
int16x8_t
Source
§
impl
From
<
Simd
<
i32
, 2>> for
int32x2_t
Source
§
impl
From
<
Simd
<
i32
, 4>> for
int32x4_t
Source
§
impl
From
<
Simd
<
i64
, 1>> for
int64x1_t
Source
§
impl
From
<
Simd
<
i64
, 2>> for
int64x2_t
Source
§
impl
From
<
Simd
<
u8
, 8>> for
poly8x8_t
Source
§
impl
From
<
Simd
<
u8
, 8>> for
uint8x8_t
Source
§
impl
From
<
Simd
<
u8
, 16>> for
poly8x16_t
Source
§
impl
From
<
Simd
<
u8
, 16>> for
uint8x16_t
Source
§
impl
From
<
Simd
<
u16
, 4>> for
poly16x4_t
Source
§
impl
From
<
Simd
<
u16
, 4>> for
uint16x4_t
Source
§
impl
From
<
Simd
<
u16
, 8>> for
poly16x8_t
Source
§
impl
From
<
Simd
<
u16
, 8>> for
uint16x8_t
Source
§
impl
From
<
Simd
<
u32
, 2>> for
uint32x2_t
Source
§
impl
From
<
Simd
<
u32
, 4>> for
uint32x4_t
Source
§
impl
From
<
Simd
<
u64
, 1>> for
poly64x1_t
Source
§
impl
From
<
Simd
<
u64
, 1>> for
uint64x1_t
Source
§
impl
From
<
Simd
<
u64
, 2>> for
poly64x2_t
Source
§
impl
From
<
Simd
<
u64
, 2>> for
uint64x2_t
1.20.0
·
Source
§
impl
From
<
String
> for
Box
<
str
>
1.0.0
·
Source
§
impl
From
<
String
> for
OsString
1.0.0
·
Source
§
impl
From
<
String
> for
PathBuf
1.21.0
·
Source
§
impl
From
<
String
> for
Rc
<
str
>
1.21.0
·
Source
§
impl
From
<
String
> for
Arc
<
str
>
1.14.0
·
Source
§
impl
From
<
String
> for
Vec
<
u8
>
1.24.0
·
Source
§
impl
From
<
RecvError
> for
RecvTimeoutError
1.24.0
·
Source
§
impl
From
<
RecvError
> for
TryRecvError
1.62.0
·
Source
§
impl
From
<
Arc
<
str
>> for
Arc
<[
u8
]>
Source
§
impl
From
<
Arc
<
ByteStr
>> for
Arc
<[
u8
]>
Source
§
impl
From
<
Arc
<[
u8
]>> for
Arc
<
ByteStr
>
1.43.0
·
Source
§
impl
From
<
Vec
<
NonZero
<
u8
>>> for
CString
1.17.0
·
Source
§
impl
From
<[
u8
;
4
]> for
IpAddr
1.9.0
·
Source
§
impl
From
<[
u8
;
4
]> for
Ipv4Addr
1.17.0
·
Source
§
impl
From
<[
u8
;
16
]> for
IpAddr
1.9.0
·
Source
§
impl
From
<[
u8
;
16
]> for
Ipv6Addr
1.17.0
·
Source
§
impl
From
<[
u16
;
8
]> for
IpAddr
1.16.0
·
Source
§
impl
From
<[
u16
;
8
]> for
Ipv6Addr
1.0.0
·
Source
§
impl<'a>
From
<&'a
str
> for
Cow
<'a,
str
>
Source
§
impl<'a>
From
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
From
<&'a
ByteStr
> for
ByteString
Source
§
impl<'a>
From
<&'a
ByteString
> for
Cow
<'a,
ByteStr
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
CStr
> for
Cow
<'a,
CStr
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
CString
> for
Cow
<'a,
CStr
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsStr
> for
Cow
<'a,
OsStr
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsString
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
From
<&'a
Path
> for
Cow
<'a,
Path
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
PathBuf
> for
Cow
<'a,
Path
>
1.28.0
·
Source
§
impl<'a>
From
<&'a
String
> for
Cow
<'a,
str
>
1.6.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+ 'a>
1.0.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
1.14.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
str
>> for
String
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
CStr
>> for
CString
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
OsStr
>> for
OsString
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
impl<'a>
From
<
ByteString
> for
Cow
<'a,
ByteStr
>
1.28.0
·
Source
§
impl<'a>
From
<
CString
> for
Cow
<'a,
CStr
>
1.28.0
·
Source
§
impl<'a>
From
<
OsString
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
From
<
PathBuf
> for
Cow
<'a,
Path
>
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Cow
<'a,
str
>
1.6.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+ 'a>
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+ 'a>
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
1.45.0
·
Source
§
impl<'a, B>
From
<
Cow
<'a, B>> for
Rc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Rc
<B>:
From
<
&'a B
> +
From
<<B as
ToOwned
>::
Owned
>,
1.45.0
·
Source
§
impl<'a, B>
From
<
Cow
<'a, B>> for
Arc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Arc
<B>:
From
<
&'a B
> +
From
<<B as
ToOwned
>::
Owned
>,
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+ 'a>
where
    E:
Error
+ 'a,
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
where
    E:
Error
+
Send
+
Sync
+ 'a,
1.30.0
·
Source
§
impl<'a, T>
From
<&'a
Option
<T>> for
Option
<
&'a T
>
1.8.0
·
Source
§
impl<'a, T>
From
<&'a
[T]
> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
1.28.0
·
Source
§
impl<'a, T>
From
<&'a
Vec
<T>> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
1.30.0
·
Source
§
impl<'a, T>
From
<&'a mut
Option
<T>> for
Option
<
&'a mut T
>
1.14.0
·
Source
§
impl<'a, T>
From
<
Cow
<'a,
[T]
>> for
Vec
<T>
where
[T]
:
ToOwned
<Owned =
Vec
<T>>,
1.8.0
·
Source
§
impl<'a, T>
From
<
Vec
<T>> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
1.77.0
·
Source
§
impl<'a, T, const N:
usize
>
From
<&'a
[T; N]
> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
impl<'data>
From
<&'data mut [
u8
]> for
BorrowedBuf
<'data>
Creates a new
BorrowedBuf
from a fully initialized slice.
Source
§
impl<'data>
From
<&'data mut [
MaybeUninit
<
u8
>]> for
BorrowedBuf
<'data>
Creates a new
BorrowedBuf
from an uninitialized buffer.
Use
set_init
if part of the buffer is known to be already initialized.
1.19.0
·
Source
§
impl<A>
From
<
Box
<
str
, A>> for
Box
<[
u8
], A>
where
    A:
Allocator
,
Source
§
impl<E>
From
<E> for
Report
<E>
where
    E:
Error
,
1.17.0
·
Source
§
impl<I>
From
<(I,
u16
)> for
SocketAddr
where
    I:
Into
<
IpAddr
>,
1.56.0
·
Source
§
impl<K, V, const N:
usize
>
From
<[
(K, V)
;
N
]> for
BTreeMap
<K, V>
where
    K:
Ord
,
1.56.0
·
Source
§
impl<K, V, const N:
usize
>
From
<[
(K, V)
;
N
]> for
HashMap
<K, V,
RandomState
>
where
    K:
Eq
+
Hash
,
1.17.0
·
Source
§
impl<T>
From
<&
[T]
> for
Box
<
[T]
>
where
    T:
Clone
,
1.21.0
·
Source
§
impl<T>
From
<&
[T]
> for
Rc
<
[T]
>
where
    T:
Clone
,
1.21.0
·
Source
§
impl<T>
From
<&
[T]
> for
Arc
<
[T]
>
where
    T:
Clone
,
1.0.0
·
Source
§
impl<T>
From
<&
[T]
> for
Vec
<T>
where
    T:
Clone
,
1.84.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Box
<
[T]
>
where
    T:
Clone
,
1.84.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Rc
<
[T]
>
where
    T:
Clone
,
1.84.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Arc
<
[T]
>
where
    T:
Clone
,
1.19.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Vec
<T>
where
    T:
Clone
,
1.45.0
·
Source
§
impl<T>
From
<
Cow
<'_,
[T]
>> for
Box
<
[T]
>
where
    T:
Clone
,
1.71.0
·
Source
§
impl<T>
From
<
[T; N]
> for
(T₁, T₂, …, Tₙ)
This trait is implemented for tuples up to twelve items long.
1.34.0
·
Source
§
impl<T>
From
<
!
> for T
Stability note:
This impl does not yet exist, but we are
“reserving space” to add it in the future. See
rust-lang/rust#64715
for details.
1.23.0
·
Source
§
impl<T>
From
<
*mut T
> for
AtomicPtr
<T>
1.25.0
·
Source
§
impl<T>
From
<
&T
> for
NonNull
<T>
where
    T: ?
Sized
,
1.25.0
·
Source
§
impl<T>
From
<
&mut T
> for
NonNull
<T>
where
    T: ?
Sized
,
1.71.0
·
Source
§
impl<T>
From
<
(T₁, T₂, …, Tₙ)
> for
[T; N]
This trait is implemented for tuples up to twelve items long.
1.31.0
·
Source
§
impl<T>
From
<
NonZero
<T>> for T
where
    T:
ZeroablePrimitive
,
Source
§
impl<T>
From
<
Range
<T>> for std::range::
Range
<T>
Source
§
impl<T>
From
<
RangeFrom
<T>> for std::range::
RangeFrom
<T>
Source
§
impl<T>
From
<
RangeInclusive
<T>> for std::range::
RangeInclusive
<T>
Source
§
impl<T>
From
<
Range
<T>> for std::ops::
Range
<T>
Source
§
impl<T>
From
<
RangeFrom
<T>> for std::ops::
RangeFrom
<T>
Source
§
impl<T>
From
<
RangeInclusive
<T>> for std::ops::
RangeInclusive
<T>
Source
§
impl<T>
From
<
SendError
<T>> for
SendTimeoutError
<T>
1.24.0
·
Source
§
impl<T>
From
<
SendError
<T>> for
TrySendError
<T>
1.0.0
·
Source
§
impl<T>
From
<
PoisonError
<T>> for
TryLockError
<T>
1.63.0
·
Source
§
impl<T>
From
<
JoinHandle
<T>> for
OwnedHandle
Available on
Windows
only.
1.12.0
·
Source
§
impl<T>
From
<T> for
Option
<T>
1.36.0
·
Source
§
impl<T>
From
<T> for
Poll
<T>
1.6.0
·
Source
§
impl<T>
From
<T> for
Box
<T>
1.12.0
·
Source
§
impl<T>
From
<T> for
Cell
<T>
1.70.0
·
Source
§
impl<T>
From
<T> for
OnceCell
<T>
1.12.0
·
Source
§
impl<T>
From
<T> for
RefCell
<T>
Source
§
impl<T>
From
<T> for
SyncUnsafeCell
<T>
1.12.0
·
Source
§
impl<T>
From
<T> for
UnsafeCell
<T>
1.6.0
·
Source
§
impl<T>
From
<T> for
Rc
<T>
1.6.0
·
Source
§
impl<T>
From
<T> for
Arc
<T>
Source
§
impl<T>
From
<T> for
Exclusive
<T>
1.24.0
·
Source
§
impl<T>
From
<T> for
Mutex
<T>
1.70.0
·
Source
§
impl<T>
From
<T> for
OnceLock
<T>
Source
§
impl<T>
From
<T> for
ReentrantLock
<T>
1.24.0
·
Source
§
impl<T>
From
<T> for
RwLock
<T>
1.0.0
·
Source
§
impl<T>
From
<T> for T
1.18.0
·
Source
§
impl<T, A>
From
<
Box
<
[T]
, A>> for
Vec
<T, A>
where
    A:
Allocator
,
1.33.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Pin
<
Box
<T, A>>
where
    A:
Allocator
+ 'static,
    T: ?
Sized
,
1.21.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Rc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.21.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Arc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.5.0
·
Source
§
impl<T, A>
From
<
BinaryHeap
<T, A>> for
Vec
<T, A>
where
    A:
Allocator
,
1.10.0
·
Source
§
impl<T, A>
From
<
VecDeque
<T, A>> for
Vec
<T, A>
where
    A:
Allocator
,
1.20.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Box
<
[T]
, A>
where
    A:
Allocator
,
1.5.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
BinaryHeap
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
1.10.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
VecDeque
<T, A>
where
    A:
Allocator
,
1.21.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Rc
<
[T]
, A>
where
    A:
Allocator
,
1.21.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Arc
<
[T]
, A>
where
    A:
Allocator
+
Clone
,
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<&
[T; N]
> for
Vec
<T>
where
    T:
Clone
,
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<&mut
[T; N]
> for
Vec
<T>
where
    T:
Clone
,
1.45.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Box
<
[T]
>
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
BTreeSet
<T>
where
    T:
Ord
,
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
BinaryHeap
<T>
where
    T:
Ord
,
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
HashSet
<T,
RandomState
>
where
    T:
Eq
+
Hash
,
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
LinkedList
<T>
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
VecDeque
<T>
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Rc
<
[T]
>
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Arc
<
[T]
>
1.44.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Vec
<T>
Source
§
impl<T, const N:
usize
>
From
<
Mask
<T, N>> for [
bool
;
N
]
where
    T:
MaskElement
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
From
<
Simd
<T, N>> for
[T; N]
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
impl<T, const N:
usize
>
From
<Mask<T, N>> for
Simd
<T, N>
where
    T:
MaskElement
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
From
<[
bool
;
N
]> for
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
1.0.0
·
Source
§
impl<T: ?
Sized
+
AsRef
<
OsStr
>>
From
<
&T
> for
OsString
1.0.0
·
Source
§
impl<T: ?
Sized
+
AsRef
<
OsStr
>>
From
<
&T
> for
PathBuf
1.0.0
·
Source
§
impl<W>
From
<
IntoInnerError
<W>> for
Error
Source
§
impl<W>
From
<
Rc
<W>> for
LocalWaker
where
    W:
LocalWake
+ 'static,
Source
§
impl<W>
From
<
Rc
<W>> for
RawWaker
where
    W:
LocalWake
+ 'static,
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
RawWaker
where
    W:
Wake
+
Send
+
Sync
+ 'static,
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
Waker
where
    W:
Wake
+
Send
+
Sync
+ 'static,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,