Unpin in std::marker - Rust
std
::
marker
Trait
Unpin
Copy item path
1.33.0
·
Source
pub auto trait Unpin { }
Expand description
Types that do not require any pinning guarantees.
For information on what “pinning” is, see the
pin
module
documentation.
Implementing the
Unpin
trait for
T
expresses the fact that
T
is pinning-agnostic:
it shall not expose nor rely on any pinning guarantees. This, in turn, means that a
Pin
-wrapped pointer to such a type can feature a
fully unrestricted
API.
In other words, if
T: Unpin
, a value of type
T
will
not
be bound by the invariants
which pinning otherwise offers, even when “pinned” by a
Pin<Ptr>
pointing at it.
When a value of type
T
is pointed at by a
Pin<Ptr>
,
Pin
will not restrict access
to the pointee value like it normally would, thus allowing the user to do anything that they
normally could with a non-
Pin
-wrapped
Ptr
to that value.
The idea of this trait is to alleviate the reduced ergonomics of APIs that require the use
of
Pin
for soundness for some types, but which also want to be used by other types that
don’t care about pinning. The prime example of such an API is
Future::poll
. There are many
Future
types that don’t care about pinning. These futures can implement
Unpin
and
therefore get around the pinning related restrictions in the API, while still allowing the
subset of
Future
s which
do
require pinning to be implemented soundly.
For more discussion on the consequences of
Unpin
within the wider scope of the pinning
system, see the
section about
Unpin
in the
pin
module
.
Unpin
has no consequence at all for non-pinned data. In particular,
mem::replace
happily
moves
!Unpin
data, which would be immovable when pinned (
mem::replace
works for any
&mut T
, not just when
T: Unpin
).
However
, you cannot use
mem::replace
on
!Unpin
data which is
pinned
by being wrapped
inside a
Pin<Ptr>
pointing at it. This is because you cannot (safely) use a
Pin<Ptr>
to get a
&mut T
to its pointee value, which you would need to call
mem::replace
, and
that
is what makes this system work.
So this, for example, can only be done on types implementing
Unpin
:
use
std::mem;
use
std::pin::Pin;
let
mut
string =
"this"
.to_string();
let
mut
pinned_string = Pin::new(
&mut
string);
// We need a mutable reference to call `mem::replace`.
// We can obtain such a reference by (implicitly) invoking `Pin::deref_mut`,
// but that is only possible because `String` implements `Unpin`.
mem::replace(
&mut *
pinned_string,
"other"
.to_string());
This trait is automatically implemented for almost every type. The compiler is free
to take the conservative stance of marking types as
Unpin
so long as all of the types that
compose its fields are also
Unpin
. This is because if a type implements
Unpin
, then it
is unsound for that type’s implementation to rely on pinning-related guarantees for soundness,
even
when viewed through a “pinning” pointer! It is the responsibility of the implementor of
a type that relies upon pinning for soundness to ensure that type is
not
marked as
Unpin
by adding
PhantomPinned
field. For more details, see the
pin
module
docs.
Implementors
§
1.33.0
·
Source
§
impl !
Unpin
for
PhantomPinned
Source
§
impl
Unpin
for
LocalWaker
1.36.0
·
Source
§
impl
Unpin
for
Waker
Source
§
impl<Dyn>
Unpin
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
1.64.0
·
Source
§
impl<F>
Unpin
for
PollFn
<F>
where
    F:
Unpin
,
Source
§
impl<I>
Unpin
for
FromIter
<I>
1.38.0
·
Source
§
impl<T>
Unpin
for
*const T
where
    T: ?
Sized
,
1.38.0
·
Source
§
impl<T>
Unpin
for
*mut T
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
Unpin
for
&T
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
Unpin
for
&mut T
where
    T: ?
Sized
,
1.48.0
·
Source
§
impl<T>
Unpin
for
Ready
<T>
1.28.0
·
Source
§
impl<T>
Unpin
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Unpin
,
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Box
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Rc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A>
Unpin
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Arc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Auto implementors
§
§
impl
Unpin
for
AsciiChar
§
impl
Unpin
for
BacktraceStatus
§
impl
Unpin
for std::cmp::
Ordering
§
impl
Unpin
for
TryReserveErrorKind
§
impl
Unpin
for
Infallible
§
impl
Unpin
for
VarError
§
impl
Unpin
for
FromBytesWithNulError
§
impl
Unpin
for
c_void
§
impl
Unpin
for std::fmt::
Alignment
§
impl
Unpin
for
DebugAsHex
§
impl
Unpin
for
Sign
§
impl
Unpin
for
BasicBlock
§
impl
Unpin
for
UnwindTerminateReason
§
impl
Unpin
for
ErrorKind
§
impl
Unpin
for
SeekFrom
§
impl
Unpin
for
IpAddr
§
impl
Unpin
for
Ipv6MulticastScope
§
impl
Unpin
for
Shutdown
§
impl
Unpin
for std::net::
SocketAddr
§
impl
Unpin
for
FpCategory
§
impl
Unpin
for
IntErrorKind
§
impl
Unpin
for
OneSidedRangeBound
§
impl
Unpin
for
AncillaryError
§
impl
Unpin
for
BacktraceStyle
§
impl
Unpin
for
GetDisjointMutError
§
impl
Unpin
for
SearchStep
§
impl
Unpin
for std::sync::atomic::
Ordering
§
impl
Unpin
for
RecvTimeoutError
§
impl
Unpin
for
TryRecvError
§
impl
Unpin
for
bool
§
impl
Unpin
for
char
§
impl
Unpin
for
f16
§
impl
Unpin
for
f32
§
impl
Unpin
for
f64
§
impl
Unpin
for
f128
§
impl
Unpin
for
i8
§
impl
Unpin
for
i16
§
impl
Unpin
for
i32
§
impl
Unpin
for
i64
§
impl
Unpin
for
i128
§
impl
Unpin
for
isize
§
impl
Unpin
for
str
§
impl
Unpin
for
u8
§
impl
Unpin
for
u16
§
impl
Unpin
for
u32
§
impl
Unpin
for
u64
§
impl
Unpin
for
u128
§
impl
Unpin
for
()
§
impl
Unpin
for
usize
§
impl
Unpin
for
AllocError
§
impl
Unpin
for
Global
§
impl
Unpin
for
Layout
§
impl
Unpin
for
LayoutError
§
impl
Unpin
for
System
§
impl
Unpin
for
TypeId
§
impl
Unpin
for
TryFromSliceError
§
impl
Unpin
for std::ascii::
EscapeDefault
§
impl
Unpin
for
Backtrace
§
impl
Unpin
for
BacktraceFrame
§
impl
Unpin
for
ByteStr
§
impl
Unpin
for
ByteString
§
impl
Unpin
for
BorrowError
§
impl
Unpin
for
BorrowMutError
§
impl
Unpin
for
CharTryFromError
§
impl
Unpin
for
DecodeUtf16Error
§
impl
Unpin
for std::char::
EscapeDebug
§
impl
Unpin
for std::char::
EscapeDefault
§
impl
Unpin
for std::char::
EscapeUnicode
§
impl
Unpin
for
ParseCharError
§
impl
Unpin
for
ToLowercase
§
impl
Unpin
for
ToUppercase
§
impl
Unpin
for
TryFromCharError
§
impl
Unpin
for
UnorderedKeyError
§
impl
Unpin
for
TryReserveError
§
impl
Unpin
for
Args
§
impl
Unpin
for
ArgsOs
§
impl
Unpin
for
JoinPathsError
§
impl
Unpin
for
Vars
§
impl
Unpin
for
VarsOs
§
impl
Unpin
for
CStr
§
impl
Unpin
for
CString
§
impl
Unpin
for
FromBytesUntilNulError
§
impl
Unpin
for
FromVecWithNulError
§
impl
Unpin
for
IntoStringError
§
impl
Unpin
for
NulError
§
impl
Unpin
for
OsStr
§
impl
Unpin
for
OsString
§
impl
Unpin
for std::fmt::
Error
§
impl
Unpin
for
FormattingOptions
§
impl
Unpin
for
DirBuilder
§
impl
Unpin
for
DirEntry
§
impl
Unpin
for
File
§
impl
Unpin
for
FileTimes
§
impl
Unpin
for
FileType
§
impl
Unpin
for
Metadata
§
impl
Unpin
for
OpenOptions
§
impl
Unpin
for
Permissions
§
impl
Unpin
for
ReadDir
§
impl
Unpin
for
DefaultHasher
§
impl
Unpin
for
RandomState
§
impl
Unpin
for
SipHasher
§
impl
Unpin
for
ReturnToArg
§
impl
Unpin
for
UnwindActionArg
§
impl
Unpin
for std::io::
Empty
§
impl
Unpin
for std::io::
Error
§
impl
Unpin
for
PipeReader
§
impl
Unpin
for
PipeWriter
§
impl
Unpin
for std::io::
Repeat
§
impl
Unpin
for
Sink
§
impl
Unpin
for
Stderr
§
impl
Unpin
for
Stdin
§
impl
Unpin
for
Stdout
§
impl
Unpin
for
WriterPanicked
§
impl
Unpin
for
Assume
§
impl
Unpin
for
AddrParseError
§
impl
Unpin
for
IntoIncoming
§
impl
Unpin
for
Ipv4Addr
§
impl
Unpin
for
Ipv6Addr
§
impl
Unpin
for
SocketAddrV4
§
impl
Unpin
for
SocketAddrV6
§
impl
Unpin
for
TcpListener
§
impl
Unpin
for
TcpStream
§
impl
Unpin
for
UdpSocket
§
impl
Unpin
for
ParseFloatError
§
impl
Unpin
for
ParseIntError
§
impl
Unpin
for
TryFromIntError
§
impl
Unpin
for
RangeFull
§
impl
Unpin
for
OwnedFd
§
impl
Unpin
for
PidFd
§
impl
Unpin
for std::os::linux::raw::
stat
§
impl
Unpin
for std::os::macos::raw::
stat
§
impl
Unpin
for std::os::unix::net::
SocketAddr
§
impl
Unpin
for
SocketCred
§
impl
Unpin
for
UCred
§
impl
Unpin
for
UnixDatagram
§
impl
Unpin
for
UnixListener
§
impl
Unpin
for
UnixStream
§
impl
Unpin
for
HandleOrInvalid
§
impl
Unpin
for
HandleOrNull
§
impl
Unpin
for
InvalidHandleError
§
impl
Unpin
for
NullHandleError
§
impl
Unpin
for
OwnedHandle
§
impl
Unpin
for
OwnedSocket
§
impl
Unpin
for
Path
§
impl
Unpin
for
PathBuf
§
impl
Unpin
for
StripPrefixError
§
impl
Unpin
for
Child
§
impl
Unpin
for
ChildStderr
§
impl
Unpin
for
ChildStdin
§
impl
Unpin
for
ChildStdout
§
impl
Unpin
for
Command
§
impl
Unpin
for
ExitCode
§
impl
Unpin
for
ExitStatus
§
impl
Unpin
for
ExitStatusError
§
impl
Unpin
for
Output
§
impl
Unpin
for
Stdio
§
impl
Unpin
for std::ptr::
Alignment
§
impl
Unpin
for
DefaultRandomSource
§
impl
Unpin
for
ParseBoolError
§
impl
Unpin
for
Utf8Error
§
impl
Unpin
for
FromUtf8Error
§
impl
Unpin
for
FromUtf16Error
§
impl
Unpin
for
IntoChars
§
impl
Unpin
for
String
§
impl
Unpin
for
AtomicBool
§
impl
Unpin
for
AtomicI8
§
impl
Unpin
for
AtomicI16
§
impl
Unpin
for
AtomicI32
§
impl
Unpin
for
AtomicI64
§
impl
Unpin
for
AtomicI128
§
impl
Unpin
for
AtomicIsize
§
impl
Unpin
for
AtomicU8
§
impl
Unpin
for
AtomicU16
§
impl
Unpin
for
AtomicU32
§
impl
Unpin
for
AtomicU64
§
impl
Unpin
for
AtomicU128
§
impl
Unpin
for
AtomicUsize
§
impl
Unpin
for
RecvError
§
impl
Unpin
for
Barrier
§
impl
Unpin
for
BarrierWaitResult
§
impl
Unpin
for
Condvar
§
impl
Unpin
for std::sync::
Once
§
impl
Unpin
for
OnceState
§
impl
Unpin
for
WaitTimeoutResult
§
impl
Unpin
for
RawWaker
§
impl
Unpin
for
RawWakerVTable
§
impl
Unpin
for
AccessError
§
impl
Unpin
for
Builder
§
impl
Unpin
for
Thread
§
impl
Unpin
for
ThreadId
§
impl
Unpin
for
Duration
§
impl
Unpin
for
Instant
§
impl
Unpin
for
SystemTime
§
impl
Unpin
for
SystemTimeError
§
impl
Unpin
for
TryFromFloatSecsError
§
impl<'a> !
Unpin
for
Request
<'a>
§
impl<'a>
Unpin
for
AncillaryData
<'a>
§
impl<'a>
Unpin
for
Component
<'a>
§
impl<'a>
Unpin
for
Prefix
<'a>
§
impl<'a>
Unpin
for
Utf8Pattern
<'a>
§
impl<'a>
Unpin
for
SplitPaths
<'a>
§
impl<'a>
Unpin
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
Unpin
for
Arguments
<'a>
§
impl<'a>
Unpin
for
Formatter
<'a>
§
impl<'a>
Unpin
for
BorrowedCursor
<'a>
§
impl<'a>
Unpin
for
IoSlice
<'a>
§
impl<'a>
Unpin
for
IoSliceMut
<'a>
§
impl<'a>
Unpin
for
StderrLock
<'a>
§
impl<'a>
Unpin
for
StdinLock
<'a>
§
impl<'a>
Unpin
for
StdoutLock
<'a>
§
impl<'a>
Unpin
for std::net::
Incoming
<'a>
§
impl<'a>
Unpin
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
Unpin
for
Messages
<'a>
§
impl<'a>
Unpin
for
ScmCredentials
<'a>
§
impl<'a>
Unpin
for
ScmRights
<'a>
§
impl<'a>
Unpin
for
SocketAncillary
<'a>
§
impl<'a>
Unpin
for
EncodeWide
<'a>
§
impl<'a>
Unpin
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Unpin
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
Unpin
for
Location
<'a>
§
impl<'a>
Unpin
for
PanicHookInfo
<'a>
§
impl<'a>
Unpin
for
Ancestors
<'a>
§
impl<'a>
Unpin
for
Components
<'a>
§
impl<'a>
Unpin
for std::path::
Display
<'a>
§
impl<'a>
Unpin
for std::path::
Iter
<'a>
§
impl<'a>
Unpin
for
PrefixComponent
<'a>
§
impl<'a>
Unpin
for
CommandArgs
<'a>
§
impl<'a>
Unpin
for
CommandEnvs
<'a>
§
impl<'a>
Unpin
for
EscapeAscii
<'a>
§
impl<'a>
Unpin
for
CharSearcher
<'a>
§
impl<'a>
Unpin
for std::str::
Bytes
<'a>
§
impl<'a>
Unpin
for
CharIndices
<'a>
§
impl<'a>
Unpin
for
Chars
<'a>
§
impl<'a>
Unpin
for
EncodeUtf16
<'a>
§
impl<'a>
Unpin
for std::str::
EscapeDebug
<'a>
§
impl<'a>
Unpin
for std::str::
EscapeDefault
<'a>
§
impl<'a>
Unpin
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
Unpin
for std::str::
Lines
<'a>
§
impl<'a>
Unpin
for
LinesAny
<'a>
§
impl<'a>
Unpin
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
Unpin
for
SplitWhitespace
<'a>
§
impl<'a>
Unpin
for
Utf8Chunk
<'a>
§
impl<'a>
Unpin
for
Utf8Chunks
<'a>
§
impl<'a>
Unpin
for std::string::
Drain
<'a>
§
impl<'a>
Unpin
for
Context
<'a>
§
impl<'a>
Unpin
for
ContextBuilder
<'a>
§
impl<'a>
Unpin
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
Unpin
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Unpin
for
PhantomInvariantLifetime
<'a>
§
impl<'a, 'b>
Unpin
for
DebugList
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugMap
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugSet
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
Unpin
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
Unpin
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f>
Unpin
for
VaList
<'a, 'f>
§
impl<'a, A>
Unpin
for std::option::
Iter
<'a, A>
§
impl<'a, A>
Unpin
for std::option::
IterMut
<'a, A>
§
impl<'a, B>
Unpin
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Unpin
,
    B: ?
Sized
,
§
impl<'a, F>
Unpin
for
CharPredicateSearcher
<'a, F>
where
    F:
Unpin
,
§
impl<'a, I>
Unpin
for
ByRefSized
<'a, I>
§
impl<'a, I, A>
Unpin
for
Splice
<'a, I, A>
where
    I:
Unpin
,
§
impl<'a, K>
Unpin
for std::collections::btree_set::
Cursor
<'a, K>
§
impl<'a, K>
Unpin
for std::collections::hash_set::
Drain
<'a, K>
§
impl<'a, K>
Unpin
for std::collections::hash_set::
Iter
<'a, K>
§
impl<'a, K, A>
Unpin
for std::collections::btree_set::
CursorMut
<'a, K, A>
§
impl<'a, K, A>
Unpin
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
§
impl<'a, K, F>
Unpin
for std::collections::hash_set::
ExtractIf
<'a, K, F>
where
    F:
Unpin
,
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
Entry
<'a, K, V>
where
    K:
Unpin
,
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
Cursor
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
Iter
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
IterMut
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
Keys
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
Range
<'a, K, V>
§
impl<'a, K, V>
Unpin
for
RangeMut
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
Values
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::btree_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
Drain
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
Iter
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
IterMut
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
Keys
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
OccupiedEntry
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
OccupiedError
<'a, K, V>
where
    V:
Unpin
,
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
VacantEntry
<'a, K, V>
where
    K:
Unpin
,
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
Values
<'a, K, V>
§
impl<'a, K, V>
Unpin
for std::collections::hash_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
Entry
<'a, K, V, A>
where
    K:
Unpin
,
    A:
Unpin
,
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
where
    A:
Unpin
,
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
where
    V:
Unpin
,
    A:
Unpin
,
§
impl<'a, K, V, A>
Unpin
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
where
    K:
Unpin
,
    A:
Unpin
,
§
impl<'a, K, V, F>
Unpin
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
where
    F:
Unpin
,
§
impl<'a, K, V, F, A>
Unpin
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
where
    F:
Unpin
,
    A:
Unpin
,
§
impl<'a, P>
Unpin
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, P>
Unpin
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Unpin
,
§
impl<'a, T>
Unpin
for std::collections::binary_heap::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::btree_set::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::btree_set::
Range
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::btree_set::
SymmetricDifference
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::btree_set::
Union
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::linked_list::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::linked_list::
IterMut
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::vec_deque::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::collections::vec_deque::
IterMut
<'a, T>
§
impl<'a, T>
Unpin
for std::result::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::result::
IterMut
<'a, T>
§
impl<'a, T>
Unpin
for
Chunks
<'a, T>
§
impl<'a, T>
Unpin
for
ChunksExact
<'a, T>
§
impl<'a, T>
Unpin
for
ChunksExactMut
<'a, T>
§
impl<'a, T>
Unpin
for
ChunksMut
<'a, T>
§
impl<'a, T>
Unpin
for std::slice::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::slice::
IterMut
<'a, T>
§
impl<'a, T>
Unpin
for
RChunks
<'a, T>
§
impl<'a, T>
Unpin
for
RChunksExact
<'a, T>
§
impl<'a, T>
Unpin
for
RChunksExactMut
<'a, T>
§
impl<'a, T>
Unpin
for
RChunksMut
<'a, T>
§
impl<'a, T>
Unpin
for
Windows
<'a, T>
§
impl<'a, T>
Unpin
for std::sync::mpmc::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::sync::mpmc::
TryIter
<'a, T>
§
impl<'a, T>
Unpin
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T>
Unpin
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
Unpin
for
MappedMutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
MappedRwLockReadGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
MappedRwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
MutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
ReentrantLockGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
RwLockReadGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Unpin
for
RwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T, A>
Unpin
for std::collections::btree_set::
Entry
<'a, T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<'a, T, A>
Unpin
for std::collections::binary_heap::
Drain
<'a, T, A>
§
impl<'a, T, A>
Unpin
for
DrainSorted
<'a, T, A>
§
impl<'a, T, A>
Unpin
for
PeekMut
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::collections::btree_set::
Difference
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::collections::btree_set::
Intersection
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
where
    A:
Unpin
,
§
impl<'a, T, A>
Unpin
for std::collections::btree_set::
VacantEntry
<'a, T, A>
where
    T:
Unpin
,
    A:
Unpin
,
§
impl<'a, T, A>
Unpin
for std::collections::linked_list::
Cursor
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::collections::linked_list::
CursorMut
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::collections::vec_deque::
Drain
<'a, T, A>
§
impl<'a, T, A>
Unpin
for std::vec::
Drain
<'a, T, A>
§
impl<'a, T, F, A>
Unpin
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
where
    F:
Unpin
,
    A:
Unpin
,
§
impl<'a, T, F, A>
Unpin
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
where
    F:
Unpin
,
§
impl<'a, T, F, A>
Unpin
for std::vec::
ExtractIf
<'a, T, F, A>
where
    F:
Unpin
,
§
impl<'a, T, P>
Unpin
for
ChunkBy
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
ChunkByMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for std::slice::
RSplit
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
RSplitMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for std::slice::
RSplitN
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
RSplitNMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for std::slice::
Split
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
SplitInclusiveMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
SplitMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for std::slice::
SplitN
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, P>
Unpin
for
SplitNMut
<'a, T, P>
where
    P:
Unpin
,
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
Entry
<'a, T, S>
where
    T:
Unpin
,
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
Difference
<'a, T, S>
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
Intersection
<'a, T, S>
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
SymmetricDifference
<'a, T, S>
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
Union
<'a, T, S>
§
impl<'a, T, S>
Unpin
for std::collections::hash_set::
VacantEntry
<'a, T, S>
where
    T:
Unpin
,
§
impl<'a, T, const N:
usize
>
Unpin
for std::slice::
ArrayChunks
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Unpin
for
ArrayChunksMut
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Unpin
for
ArrayWindows
<'a, T, N>
§
impl<'a, const N:
usize
>
Unpin
for
CharArraySearcher
<'a, N>
§
impl<'b, T>
Unpin
for
Ref
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T>
Unpin
for
RefMut
<'b, T>
where
    T: ?
Sized
,
§
impl<'data>
Unpin
for
BorrowedBuf
<'data>
§
impl<'f>
Unpin
for
VaListImpl
<'f>
§
impl<'fd>
Unpin
for
BorrowedFd
<'fd>
§
impl<'handle>
Unpin
for
BorrowedHandle
<'handle>
§
impl<'scope, 'env>
Unpin
for
Scope
<'scope, 'env>
§
impl<'scope, T>
Unpin
for
ScopedJoinHandle
<'scope, T>
§
impl<'socket>
Unpin
for
BorrowedSocket
<'socket>
§
impl<A>
Unpin
for std::iter::
Repeat
<A>
where
    A:
Unpin
,
§
impl<A>
Unpin
for
RepeatN
<A>
where
    A:
Unpin
,
§
impl<A>
Unpin
for std::option::
IntoIter
<A>
where
    A:
Unpin
,
§
impl<A>
Unpin
for
IterRange
<A>
where
    A:
Unpin
,
§
impl<A>
Unpin
for
IterRangeFrom
<A>
where
    A:
Unpin
,
§
impl<A>
Unpin
for
IterRangeInclusive
<A>
where
    A:
Unpin
,
§
impl<A, B>
Unpin
for std::iter::
Chain
<A, B>
where
    A:
Unpin
,
    B:
Unpin
,
§
impl<A, B>
Unpin
for
Zip
<A, B>
where
    A:
Unpin
,
    B:
Unpin
,
§
impl<B>
Unpin
for std::io::
Lines
<B>
where
    B:
Unpin
,
§
impl<B>
Unpin
for std::io::
Split
<B>
where
    B:
Unpin
,
§
impl<B, C>
Unpin
for
ControlFlow
<B, C>
where
    C:
Unpin
,
    B:
Unpin
,
§
impl<E>
Unpin
for
Report
<E>
where
    E:
Unpin
,
§
impl<F>
Unpin
for std::fmt::
FromFn
<F>
where
    F:
Unpin
,
§
impl<F>
Unpin
for std::iter::
FromFn
<F>
where
    F:
Unpin
,
§
impl<F>
Unpin
for
OnceWith
<F>
where
    F:
Unpin
,
§
impl<F>
Unpin
for
RepeatWith
<F>
where
    F:
Unpin
,
§
impl<G>
Unpin
for
FromCoroutine
<G>
where
    G:
Unpin
,
§
impl<H>
Unpin
for
BuildHasherDefault
<H>
§
impl<I>
Unpin
for
DecodeUtf16
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Cloned
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Copied
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Cycle
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Enumerate
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Flatten
<I>
where
    <<I as
Iterator
>::
Item
as
IntoIterator
>::
IntoIter
:
Unpin
,
    I:
Unpin
,
§
impl<I>
Unpin
for
Fuse
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
Intersperse
<I>
where
    <I as
Iterator
>::
Item
:
Sized
+
Unpin
,
    I:
Unpin
,
§
impl<I>
Unpin
for
Peekable
<I>
where
    I:
Unpin
,
    <I as
Iterator
>::
Item
:
Unpin
,
§
impl<I>
Unpin
for
Skip
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for
StepBy
<I>
where
    I:
Unpin
,
§
impl<I>
Unpin
for std::iter::
Take
<I>
where
    I:
Unpin
,
§
impl<I, F>
Unpin
for
FilterMap
<I, F>
where
    I:
Unpin
,
    F:
Unpin
,
§
impl<I, F>
Unpin
for
Inspect
<I, F>
where
    I:
Unpin
,
    F:
Unpin
,
§
impl<I, F>
Unpin
for
Map
<I, F>
where
    I:
Unpin
,
    F:
Unpin
,
§
impl<I, F, const N:
usize
>
Unpin
for
MapWindows
<I, F, N>
where
    F:
Unpin
,
    I:
Unpin
,
    <I as
Iterator
>::
Item
:
Unpin
,
§
impl<I, G>
Unpin
for
IntersperseWith
<I, G>
where
    G:
Unpin
,
    <I as
Iterator
>::
Item
:
Unpin
,
    I:
Unpin
,
§
impl<I, P>
Unpin
for
Filter
<I, P>
where
    I:
Unpin
,
    P:
Unpin
,
§
impl<I, P>
Unpin
for
MapWhile
<I, P>
where
    I:
Unpin
,
    P:
Unpin
,
§
impl<I, P>
Unpin
for
SkipWhile
<I, P>
where
    I:
Unpin
,
    P:
Unpin
,
§
impl<I, P>
Unpin
for
TakeWhile
<I, P>
where
    I:
Unpin
,
    P:
Unpin
,
§
impl<I, St, F>
Unpin
for
Scan
<I, St, F>
where
    I:
Unpin
,
    F:
Unpin
,
    St:
Unpin
,
§
impl<I, U, F>
Unpin
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
Unpin
,
    I:
Unpin
,
    F:
Unpin
,
§
impl<I, const N:
usize
>
Unpin
for std::iter::
ArrayChunks
<I, N>
where
    I:
Unpin
,
    <I as
Iterator
>::
Item
:
Unpin
,
§
impl<Idx>
Unpin
for std::ops::
Range
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for std::ops::
RangeFrom
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for
RangeTo
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for
RangeToInclusive
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for std::range::
Range
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for std::range::
RangeFrom
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
Unpin
for std::range::
RangeInclusive
<Idx>
where
    Idx:
Unpin
,
§
impl<K>
Unpin
for std::collections::hash_set::
IntoIter
<K>
where
    K:
Unpin
,
§
impl<K, V>
Unpin
for std::collections::hash_map::
IntoIter
<K, V>
where
    K:
Unpin
,
    V:
Unpin
,
§
impl<K, V>
Unpin
for std::collections::hash_map::
IntoKeys
<K, V>
where
    K:
Unpin
,
    V:
Unpin
,
§
impl<K, V>
Unpin
for std::collections::hash_map::
IntoValues
<K, V>
where
    K:
Unpin
,
    V:
Unpin
,
§
impl<K, V, A>
Unpin
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Unpin
,
§
impl<K, V, A>
Unpin
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
Unpin
,
§
impl<K, V, A>
Unpin
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Unpin
,
§
impl<K, V, A>
Unpin
for
BTreeMap
<K, V, A>
where
    A:
Unpin
,
§
impl<K, V, S>
Unpin
for
HashMap
<K, V, S>
where
    S:
Unpin
,
    K:
Unpin
,
    V:
Unpin
,
§
impl<Ptr>
Unpin
for
Pin
<Ptr>
where
    Ptr:
Unpin
,
§
impl<R>
Unpin
for
BufReader
<R>
where
    R:
Unpin
+ ?
Sized
,
§
impl<R>
Unpin
for std::io::
Bytes
<R>
where
    R:
Unpin
,
§
impl<Ret, T>
Unpin
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T>
Unpin
for
Bound
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Option
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
TryLockError
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
SendTimeoutError
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
TrySendError
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Poll
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
[T]
where
    T:
Unpin
,
§
impl<T>
Unpin
for
(T₁, T₂, …, Tₙ)
where
    T:
Unpin
,
§
impl<T>
Unpin
for
ThinBox
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
Cell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
OnceCell
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
RefCell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
SyncUnsafeCell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
UnsafeCell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
Reverse
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Unpin
,
    T: ?
Sized
,
§
impl<T>
Unpin
for
Pending
<T>
§
impl<T>
Unpin
for std::io::
Cursor
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for std::io::
Take
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for std::iter::
Empty
<T>
§
impl<T>
Unpin
for std::iter::
Once
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Rev
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Discriminant
<T>
§
impl<T>
Unpin
for
ManuallyDrop
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
Saturating
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Wrapping
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
Yeet
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
AssertUnwindSafe
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
NonNull
<T>
where
    T: ?
Sized
,
§
impl<T>
Unpin
for std::result::
IntoIter
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
AtomicPtr
<T>
§
impl<T>
Unpin
for std::sync::mpmc::
IntoIter
<T>
§
impl<T>
Unpin
for std::sync::mpmc::
Receiver
<T>
§
impl<T>
Unpin
for std::sync::mpmc::
Sender
<T>
§
impl<T>
Unpin
for std::sync::mpsc::
IntoIter
<T>
§
impl<T>
Unpin
for std::sync::mpsc::
Receiver
<T>
§
impl<T>
Unpin
for
SendError
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for std::sync::mpsc::
Sender
<T>
§
impl<T>
Unpin
for
SyncSender
<T>
§
impl<T>
Unpin
for
Exclusive
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
Mutex
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
OnceLock
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
PoisonError
<T>
where
    T:
Unpin
,
§
impl<T>
Unpin
for
ReentrantLock
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
RwLock
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
JoinHandle
<T>
§
impl<T>
Unpin
for
LocalKey
<T>
§
impl<T>
Unpin
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Unpin
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Unpin
for
PhantomData
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
Unpin
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Unpin
for
MaybeUninit
<T>
where
    T:
Unpin
,
§
impl<T, A>
Unpin
for std::collections::binary_heap::
IntoIter
<T, A>
where
    T:
Unpin
,
    A:
Unpin
,
§
impl<T, A>
Unpin
for
IntoIterSorted
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, A>
Unpin
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
Unpin
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
Unpin
for
BTreeSet
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
Unpin
for
BinaryHeap
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, A>
Unpin
for
LinkedList
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
Unpin
for
VecDeque
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, A>
Unpin
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, A>
Unpin
for std::rc::
Weak
<T, A>
where
    A:
Unpin
,
    T: ?
Sized
,
§
impl<T, A>
Unpin
for std::sync::
Weak
<T, A>
where
    A:
Unpin
,
    T: ?
Sized
,
§
impl<T, A>
Unpin
for std::vec::
IntoIter
<T, A>
where
    T:
Unpin
,
    A:
Unpin
,
§
impl<T, A>
Unpin
for
Vec
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, E>
Unpin
for
Result
<T, E>
where
    T:
Unpin
,
    E:
Unpin
,
§
impl<T, F>
Unpin
for
LazyCell
<T, F>
where
    F:
Unpin
,
    T:
Unpin
,
§
impl<T, F>
Unpin
for
Successors
<T, F>
where
    F:
Unpin
,
    T:
Unpin
,
§
impl<T, F>
Unpin
for
LazyLock
<T, F>
where
    T:
Unpin
,
    F:
Unpin
,
§
impl<T, S>
Unpin
for
HashSet
<T, S>
where
    S:
Unpin
,
    T:
Unpin
,
§
impl<T, U>
Unpin
for std::io::
Chain
<T, U>
where
    T:
Unpin
,
    U:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for
[T; N]
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for std::array::
IntoIter
<T, N>
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for
Mask
<T, N>
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for
Simd
<T, N>
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for [
Option
<T>;
N
]
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
Unpin
for [
MaybeUninit
<T>;
N
]
where
    T:
Unpin
,
§
impl<W>
Unpin
for
BufWriter
<W>
where
    W:
Unpin
+ ?
Sized
,
§
impl<W>
Unpin
for
IntoInnerError
<W>
where
    W:
Unpin
,
§
impl<W>
Unpin
for
LineWriter
<W>
where
    W:
Unpin
+ ?
Sized
,
§
impl<Y, R>
Unpin
for
CoroutineState
<Y, R>
where
    Y:
Unpin
,
    R:
Unpin
,
§
impl<const N:
usize
>
Unpin
for
LaneCount
<N>
§
impl<const N:
usize
>
Unpin
for [
u8
;
N
]