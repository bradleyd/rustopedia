RefUnwindSafe in std::panic - Rust
std
::
panic
Trait
RefUnwindSafe
Copy item path
1.9.0
·
Source
pub auto trait RefUnwindSafe { }
Expand description
A marker trait representing types where a shared reference is considered
unwind safe.
This trait is namely not implemented by
UnsafeCell
, the root of all
interior mutability.
This is a “helper marker trait” used to provide impl blocks for the
UnwindSafe
trait, for more information see that documentation.
Implementors
§
1.9.0
·
Source
§
impl
RefUnwindSafe
for
Stderr
1.9.0
·
Source
§
impl
RefUnwindSafe
for
StderrLock
<'_>
1.9.0
·
Source
§
impl
RefUnwindSafe
for
Stdout
1.9.0
·
Source
§
impl
RefUnwindSafe
for
StdoutLock
<'_>
1.14.0
·
Source
§
impl
RefUnwindSafe
for
AtomicBool
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI8
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI16
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI32
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI64
Source
§
impl
RefUnwindSafe
for
AtomicI128
1.14.0
·
Source
§
impl
RefUnwindSafe
for
AtomicIsize
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicU8
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicU16
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicU32
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicU64
Source
§
impl
RefUnwindSafe
for
AtomicU128
1.14.0
·
Source
§
impl
RefUnwindSafe
for
AtomicUsize
1.12.0
·
Source
§
impl
RefUnwindSafe
for
Condvar
1.59.0
·
Source
§
impl
RefUnwindSafe
for std::sync::
Once
1.9.0
·
Source
§
impl<T> !
RefUnwindSafe
for
UnsafeCell
<T>
where
    T: ?
Sized
,
1.28.0
·
Source
§
impl<T>
RefUnwindSafe
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
RefUnwindSafe
,
1.14.0
·
Source
§
impl<T>
RefUnwindSafe
for
AtomicPtr
<T>
Source
§
impl<T>
RefUnwindSafe
for std::sync::mpmc::
Receiver
<T>
Source
§
impl<T>
RefUnwindSafe
for std::sync::mpmc::
Sender
<T>
1.9.0
·
Source
§
impl<T>
RefUnwindSafe
for
AssertUnwindSafe
<T>
1.58.0
·
Source
§
impl<T, A>
RefUnwindSafe
for
Rc
<T, A>
where
    T:
RefUnwindSafe
+ ?
Sized
,
    A:
Allocator
+
UnwindSafe
,
1.70.0
·
Source
§
impl<T:
RefUnwindSafe
+
UnwindSafe
>
RefUnwindSafe
for
OnceLock
<T>
1.80.0
·
Source
§
impl<T:
RefUnwindSafe
+
UnwindSafe
, F:
UnwindSafe
>
RefUnwindSafe
for
LazyLock
<T, F>
Source
§
impl<T:
RefUnwindSafe
+ ?
Sized
>
RefUnwindSafe
for
ReentrantLock
<T>
1.12.0
·
Source
§
impl<T: ?
Sized
>
RefUnwindSafe
for
Mutex
<T>
1.12.0
·
Source
§
impl<T: ?
Sized
>
RefUnwindSafe
for
RwLock
<T>
Auto implementors
§
§
impl !
RefUnwindSafe
for std::io::
Error
§
impl !
RefUnwindSafe
for
Command
§
impl !
RefUnwindSafe
for
OnceState
§
impl
RefUnwindSafe
for
AsciiChar
§
impl
RefUnwindSafe
for
BacktraceStatus
§
impl
RefUnwindSafe
for std::cmp::
Ordering
§
impl
RefUnwindSafe
for
TryReserveErrorKind
§
impl
RefUnwindSafe
for
Infallible
§
impl
RefUnwindSafe
for
VarError
§
impl
RefUnwindSafe
for
FromBytesWithNulError
§
impl
RefUnwindSafe
for
c_void
§
impl
RefUnwindSafe
for std::fmt::
Alignment
§
impl
RefUnwindSafe
for
DebugAsHex
§
impl
RefUnwindSafe
for
Sign
§
impl
RefUnwindSafe
for
BasicBlock
§
impl
RefUnwindSafe
for
UnwindTerminateReason
§
impl
RefUnwindSafe
for
ErrorKind
§
impl
RefUnwindSafe
for
SeekFrom
§
impl
RefUnwindSafe
for
IpAddr
§
impl
RefUnwindSafe
for
Ipv6MulticastScope
§
impl
RefUnwindSafe
for
Shutdown
§
impl
RefUnwindSafe
for std::net::
SocketAddr
§
impl
RefUnwindSafe
for
FpCategory
§
impl
RefUnwindSafe
for
IntErrorKind
§
impl
RefUnwindSafe
for
OneSidedRangeBound
§
impl
RefUnwindSafe
for
AncillaryError
§
impl
RefUnwindSafe
for
GetDisjointMutError
§
impl
RefUnwindSafe
for
SearchStep
§
impl
RefUnwindSafe
for std::sync::atomic::
Ordering
§
impl
RefUnwindSafe
for
RecvTimeoutError
§
impl
RefUnwindSafe
for
TryRecvError
§
impl
RefUnwindSafe
for
BacktraceStyle
§
impl
RefUnwindSafe
for
bool
§
impl
RefUnwindSafe
for
char
§
impl
RefUnwindSafe
for
f16
§
impl
RefUnwindSafe
for
f32
§
impl
RefUnwindSafe
for
f64
§
impl
RefUnwindSafe
for
f128
§
impl
RefUnwindSafe
for
i8
§
impl
RefUnwindSafe
for
i16
§
impl
RefUnwindSafe
for
i32
§
impl
RefUnwindSafe
for
i64
§
impl
RefUnwindSafe
for
i128
§
impl
RefUnwindSafe
for
isize
§
impl
RefUnwindSafe
for
str
§
impl
RefUnwindSafe
for
u8
§
impl
RefUnwindSafe
for
u16
§
impl
RefUnwindSafe
for
u32
§
impl
RefUnwindSafe
for
u64
§
impl
RefUnwindSafe
for
u128
§
impl
RefUnwindSafe
for
()
§
impl
RefUnwindSafe
for
usize
§
impl
RefUnwindSafe
for
AllocError
§
impl
RefUnwindSafe
for
Global
§
impl
RefUnwindSafe
for
Layout
§
impl
RefUnwindSafe
for
LayoutError
§
impl
RefUnwindSafe
for
System
§
impl
RefUnwindSafe
for
TypeId
§
impl
RefUnwindSafe
for
TryFromSliceError
§
impl
RefUnwindSafe
for std::ascii::
EscapeDefault
§
impl
RefUnwindSafe
for
Backtrace
§
impl
RefUnwindSafe
for
BacktraceFrame
§
impl
RefUnwindSafe
for
ByteStr
§
impl
RefUnwindSafe
for
ByteString
§
impl
RefUnwindSafe
for
BorrowError
§
impl
RefUnwindSafe
for
BorrowMutError
§
impl
RefUnwindSafe
for
CharTryFromError
§
impl
RefUnwindSafe
for
DecodeUtf16Error
§
impl
RefUnwindSafe
for std::char::
EscapeDebug
§
impl
RefUnwindSafe
for std::char::
EscapeDefault
§
impl
RefUnwindSafe
for std::char::
EscapeUnicode
§
impl
RefUnwindSafe
for
ParseCharError
§
impl
RefUnwindSafe
for
ToLowercase
§
impl
RefUnwindSafe
for
ToUppercase
§
impl
RefUnwindSafe
for
TryFromCharError
§
impl
RefUnwindSafe
for
UnorderedKeyError
§
impl
RefUnwindSafe
for
TryReserveError
§
impl
RefUnwindSafe
for
Args
§
impl
RefUnwindSafe
for
ArgsOs
§
impl
RefUnwindSafe
for
JoinPathsError
§
impl
RefUnwindSafe
for
Vars
§
impl
RefUnwindSafe
for
VarsOs
§
impl
RefUnwindSafe
for
CStr
§
impl
RefUnwindSafe
for
CString
§
impl
RefUnwindSafe
for
FromBytesUntilNulError
§
impl
RefUnwindSafe
for
FromVecWithNulError
§
impl
RefUnwindSafe
for
IntoStringError
§
impl
RefUnwindSafe
for
NulError
§
impl
RefUnwindSafe
for
OsStr
§
impl
RefUnwindSafe
for
OsString
§
impl
RefUnwindSafe
for std::fmt::
Error
§
impl
RefUnwindSafe
for
FormattingOptions
§
impl
RefUnwindSafe
for
DirBuilder
§
impl
RefUnwindSafe
for
DirEntry
§
impl
RefUnwindSafe
for
File
§
impl
RefUnwindSafe
for
FileTimes
§
impl
RefUnwindSafe
for
FileType
§
impl
RefUnwindSafe
for
Metadata
§
impl
RefUnwindSafe
for
OpenOptions
§
impl
RefUnwindSafe
for
Permissions
§
impl
RefUnwindSafe
for
ReadDir
§
impl
RefUnwindSafe
for
DefaultHasher
§
impl
RefUnwindSafe
for
RandomState
§
impl
RefUnwindSafe
for
SipHasher
§
impl
RefUnwindSafe
for
ReturnToArg
§
impl
RefUnwindSafe
for
UnwindActionArg
§
impl
RefUnwindSafe
for std::io::
Empty
§
impl
RefUnwindSafe
for
PipeReader
§
impl
RefUnwindSafe
for
PipeWriter
§
impl
RefUnwindSafe
for std::io::
Repeat
§
impl
RefUnwindSafe
for
Sink
§
impl
RefUnwindSafe
for
Stdin
§
impl
RefUnwindSafe
for
WriterPanicked
§
impl
RefUnwindSafe
for
PhantomPinned
§
impl
RefUnwindSafe
for
Assume
§
impl
RefUnwindSafe
for
AddrParseError
§
impl
RefUnwindSafe
for
IntoIncoming
§
impl
RefUnwindSafe
for
Ipv4Addr
§
impl
RefUnwindSafe
for
Ipv6Addr
§
impl
RefUnwindSafe
for
SocketAddrV4
§
impl
RefUnwindSafe
for
SocketAddrV6
§
impl
RefUnwindSafe
for
TcpListener
§
impl
RefUnwindSafe
for
TcpStream
§
impl
RefUnwindSafe
for
UdpSocket
§
impl
RefUnwindSafe
for
ParseFloatError
§
impl
RefUnwindSafe
for
ParseIntError
§
impl
RefUnwindSafe
for
TryFromIntError
§
impl
RefUnwindSafe
for
RangeFull
§
impl
RefUnwindSafe
for
OwnedFd
§
impl
RefUnwindSafe
for
PidFd
§
impl
RefUnwindSafe
for std::os::linux::raw::
stat
§
impl
RefUnwindSafe
for std::os::macos::raw::
stat
§
impl
RefUnwindSafe
for std::os::unix::net::
SocketAddr
§
impl
RefUnwindSafe
for
SocketCred
§
impl
RefUnwindSafe
for
UCred
§
impl
RefUnwindSafe
for
UnixDatagram
§
impl
RefUnwindSafe
for
UnixListener
§
impl
RefUnwindSafe
for
UnixStream
§
impl
RefUnwindSafe
for
HandleOrInvalid
§
impl
RefUnwindSafe
for
HandleOrNull
§
impl
RefUnwindSafe
for
InvalidHandleError
§
impl
RefUnwindSafe
for
NullHandleError
§
impl
RefUnwindSafe
for
OwnedHandle
§
impl
RefUnwindSafe
for
OwnedSocket
§
impl
RefUnwindSafe
for
Path
§
impl
RefUnwindSafe
for
PathBuf
§
impl
RefUnwindSafe
for
StripPrefixError
§
impl
RefUnwindSafe
for
Child
§
impl
RefUnwindSafe
for
ChildStderr
§
impl
RefUnwindSafe
for
ChildStdin
§
impl
RefUnwindSafe
for
ChildStdout
§
impl
RefUnwindSafe
for
ExitCode
§
impl
RefUnwindSafe
for
ExitStatus
§
impl
RefUnwindSafe
for
ExitStatusError
§
impl
RefUnwindSafe
for
Output
§
impl
RefUnwindSafe
for
Stdio
§
impl
RefUnwindSafe
for std::ptr::
Alignment
§
impl
RefUnwindSafe
for
DefaultRandomSource
§
impl
RefUnwindSafe
for
ParseBoolError
§
impl
RefUnwindSafe
for
Utf8Error
§
impl
RefUnwindSafe
for
FromUtf8Error
§
impl
RefUnwindSafe
for
FromUtf16Error
§
impl
RefUnwindSafe
for
IntoChars
§
impl
RefUnwindSafe
for
String
§
impl
RefUnwindSafe
for
RecvError
§
impl
RefUnwindSafe
for
Barrier
§
impl
RefUnwindSafe
for
BarrierWaitResult
§
impl
RefUnwindSafe
for
WaitTimeoutResult
§
impl
RefUnwindSafe
for
LocalWaker
§
impl
RefUnwindSafe
for
RawWaker
§
impl
RefUnwindSafe
for
RawWakerVTable
§
impl
RefUnwindSafe
for
Waker
§
impl
RefUnwindSafe
for
AccessError
§
impl
RefUnwindSafe
for
Builder
§
impl
RefUnwindSafe
for
Thread
§
impl
RefUnwindSafe
for
ThreadId
§
impl
RefUnwindSafe
for
Duration
§
impl
RefUnwindSafe
for
Instant
§
impl
RefUnwindSafe
for
SystemTime
§
impl
RefUnwindSafe
for
SystemTimeError
§
impl
RefUnwindSafe
for
TryFromFloatSecsError
§
impl<'a> !
RefUnwindSafe
for
Request
<'a>
§
impl<'a> !
RefUnwindSafe
for
Formatter
<'a>
§
impl<'a> !
RefUnwindSafe
for
ContextBuilder
<'a>
§
impl<'a> !
RefUnwindSafe
for
PanicHookInfo
<'a>
§
impl<'a>
RefUnwindSafe
for
AncillaryData
<'a>
§
impl<'a>
RefUnwindSafe
for
Component
<'a>
§
impl<'a>
RefUnwindSafe
for
Prefix
<'a>
§
impl<'a>
RefUnwindSafe
for
Utf8Pattern
<'a>
§
impl<'a>
RefUnwindSafe
for
SplitPaths
<'a>
§
impl<'a>
RefUnwindSafe
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
RefUnwindSafe
for
Arguments
<'a>
§
impl<'a>
RefUnwindSafe
for
BorrowedCursor
<'a>
§
impl<'a>
RefUnwindSafe
for
IoSlice
<'a>
§
impl<'a>
RefUnwindSafe
for
IoSliceMut
<'a>
§
impl<'a>
RefUnwindSafe
for
StdinLock
<'a>
§
impl<'a>
RefUnwindSafe
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
RefUnwindSafe
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
RefUnwindSafe
for
PhantomInvariantLifetime
<'a>
§
impl<'a>
RefUnwindSafe
for std::net::
Incoming
<'a>
§
impl<'a>
RefUnwindSafe
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
RefUnwindSafe
for
Messages
<'a>
§
impl<'a>
RefUnwindSafe
for
ScmCredentials
<'a>
§
impl<'a>
RefUnwindSafe
for
ScmRights
<'a>
§
impl<'a>
RefUnwindSafe
for
SocketAncillary
<'a>
§
impl<'a>
RefUnwindSafe
for
EncodeWide
<'a>
§
impl<'a>
RefUnwindSafe
for
ProcThreadAttributeList
<'a>
§
impl<'a>
RefUnwindSafe
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
RefUnwindSafe
for
Ancestors
<'a>
§
impl<'a>
RefUnwindSafe
for
Components
<'a>
§
impl<'a>
RefUnwindSafe
for std::path::
Display
<'a>
§
impl<'a>
RefUnwindSafe
for std::path::
Iter
<'a>
§
impl<'a>
RefUnwindSafe
for
PrefixComponent
<'a>
§
impl<'a>
RefUnwindSafe
for
CommandArgs
<'a>
§
impl<'a>
RefUnwindSafe
for
CommandEnvs
<'a>
§
impl<'a>
RefUnwindSafe
for
EscapeAscii
<'a>
§
impl<'a>
RefUnwindSafe
for
CharSearcher
<'a>
§
impl<'a>
RefUnwindSafe
for std::str::
Bytes
<'a>
§
impl<'a>
RefUnwindSafe
for
CharIndices
<'a>
§
impl<'a>
RefUnwindSafe
for
Chars
<'a>
§
impl<'a>
RefUnwindSafe
for
EncodeUtf16
<'a>
§
impl<'a>
RefUnwindSafe
for std::str::
EscapeDebug
<'a>
§
impl<'a>
RefUnwindSafe
for std::str::
EscapeDefault
<'a>
§
impl<'a>
RefUnwindSafe
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
RefUnwindSafe
for std::str::
Lines
<'a>
§
impl<'a>
RefUnwindSafe
for
LinesAny
<'a>
§
impl<'a>
RefUnwindSafe
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
RefUnwindSafe
for
SplitWhitespace
<'a>
§
impl<'a>
RefUnwindSafe
for
Utf8Chunk
<'a>
§
impl<'a>
RefUnwindSafe
for
Utf8Chunks
<'a>
§
impl<'a>
RefUnwindSafe
for std::string::
Drain
<'a>
§
impl<'a>
RefUnwindSafe
for
Context
<'a>
§
impl<'a>
RefUnwindSafe
for
Location
<'a>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugSet
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
RefUnwindSafe
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
RefUnwindSafe
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
RefUnwindSafe
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
RefUnwindSafe
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f>
RefUnwindSafe
for
VaList
<'a, 'f>
§
impl<'a, A>
RefUnwindSafe
for std::option::
Iter
<'a, A>
where
    A:
RefUnwindSafe
,
§
impl<'a, A>
RefUnwindSafe
for std::option::
IterMut
<'a, A>
where
    A:
RefUnwindSafe
,
§
impl<'a, B>
RefUnwindSafe
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
RefUnwindSafe
,
    B:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, F>
RefUnwindSafe
for
CharPredicateSearcher
<'a, F>
where
    F:
RefUnwindSafe
,
§
impl<'a, I>
RefUnwindSafe
for
ByRefSized
<'a, I>
where
    I:
RefUnwindSafe
,
§
impl<'a, I, A>
RefUnwindSafe
for
Splice
<'a, I, A>
where
    I:
RefUnwindSafe
,
    <I as
Iterator
>::
Item
:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, K>
RefUnwindSafe
for std::collections::btree_set::
Cursor
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K>
RefUnwindSafe
for std::collections::hash_set::
Drain
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K>
RefUnwindSafe
for std::collections::hash_set::
Iter
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K, A>
RefUnwindSafe
for std::collections::btree_set::
CursorMut
<'a, K, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, A>
RefUnwindSafe
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, F>
RefUnwindSafe
for std::collections::hash_set::
ExtractIf
<'a, K, F>
where
    F:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
Entry
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
Cursor
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
Iter
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
IterMut
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
Keys
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
Range
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for
RangeMut
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
Values
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::btree_map::
ValuesMut
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
Drain
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
Iter
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
IterMut
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
Keys
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
OccupiedEntry
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
OccupiedError
<'a, K, V>
where
    V:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
VacantEntry
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
Values
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V>
RefUnwindSafe
for std::collections::hash_map::
ValuesMut
<'a, K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
Entry
<'a, K, V, A>
where
    K:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
where
    V:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, V, A>
RefUnwindSafe
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
where
    K:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, F>
RefUnwindSafe
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
where
    F:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, K, V, F, A>
RefUnwindSafe
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
where
    F:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, P>
RefUnwindSafe
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::binary_heap::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::btree_set::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::btree_set::
Range
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::btree_set::
SymmetricDifference
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::btree_set::
Union
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::linked_list::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::linked_list::
IterMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::vec_deque::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::collections::vec_deque::
IterMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::result::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::result::
IterMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
Chunks
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
ChunksExact
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
ChunksExactMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
ChunksMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::slice::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::slice::
IterMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
RChunks
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
RChunksExact
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
RChunksExactMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
RChunksMut
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for
Windows
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
RefUnwindSafe
for std::sync::mpmc::
Iter
<'a, T>
§
impl<'a, T>
RefUnwindSafe
for std::sync::mpmc::
TryIter
<'a, T>
§
impl<'a, T>
RefUnwindSafe
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T>
RefUnwindSafe
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
RefUnwindSafe
for
MappedMutexGuard
<'a, T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
MappedRwLockReadGuard
<'a, T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
MappedRwLockWriteGuard
<'a, T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
MutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
ReentrantLockGuard
<'a, T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
RwLockReadGuard
<'a, T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, T>
RefUnwindSafe
for
RwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::btree_set::
Entry
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::binary_heap::
Drain
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for
DrainSorted
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for
PeekMut
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::btree_set::
Difference
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::btree_set::
Intersection
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::btree_set::
VacantEntry
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::linked_list::
Cursor
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::linked_list::
CursorMut
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::collections::vec_deque::
Drain
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, A>
RefUnwindSafe
for std::vec::
Drain
<'a, T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<'a, T, F, A>
RefUnwindSafe
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
where
    F:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, F, A>
RefUnwindSafe
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
where
    F:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, F, A>
RefUnwindSafe
for std::vec::
ExtractIf
<'a, T, F, A>
where
    F:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
ChunkBy
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
ChunkByMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for std::slice::
RSplit
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
RSplitMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for std::slice::
RSplitN
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
RSplitNMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for std::slice::
Split
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
SplitInclusiveMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
SplitMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for std::slice::
SplitN
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
RefUnwindSafe
for
SplitNMut
<'a, T, P>
where
    P:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
Entry
<'a, T, S>
where
    T:
RefUnwindSafe
,
    S:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
Difference
<'a, T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
Intersection
<'a, T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
SymmetricDifference
<'a, T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
Union
<'a, T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S>
RefUnwindSafe
for std::collections::hash_set::
VacantEntry
<'a, T, S>
where
    T:
RefUnwindSafe
,
    S:
RefUnwindSafe
,
§
impl<'a, T, const N:
usize
>
RefUnwindSafe
for std::slice::
ArrayChunks
<'a, T, N>
where
    T:
RefUnwindSafe
,
§
impl<'a, T, const N:
usize
>
RefUnwindSafe
for
ArrayChunksMut
<'a, T, N>
where
    T:
RefUnwindSafe
,
§
impl<'a, T, const N:
usize
>
RefUnwindSafe
for
ArrayWindows
<'a, T, N>
where
    T:
RefUnwindSafe
,
§
impl<'a, const N:
usize
>
RefUnwindSafe
for
CharArraySearcher
<'a, N>
§
impl<'b, T> !
RefUnwindSafe
for
Ref
<'b, T>
§
impl<'b, T> !
RefUnwindSafe
for
RefMut
<'b, T>
§
impl<'data>
RefUnwindSafe
for
BorrowedBuf
<'data>
§
impl<'f>
RefUnwindSafe
for
VaListImpl
<'f>
§
impl<'fd>
RefUnwindSafe
for
BorrowedFd
<'fd>
§
impl<'handle>
RefUnwindSafe
for
BorrowedHandle
<'handle>
§
impl<'scope, 'env>
RefUnwindSafe
for
Scope
<'scope, 'env>
§
impl<'scope, T> !
RefUnwindSafe
for
ScopedJoinHandle
<'scope, T>
§
impl<'socket>
RefUnwindSafe
for
BorrowedSocket
<'socket>
§
impl<A>
RefUnwindSafe
for std::iter::
Repeat
<A>
where
    A:
RefUnwindSafe
,
§
impl<A>
RefUnwindSafe
for
RepeatN
<A>
where
    A:
RefUnwindSafe
,
§
impl<A>
RefUnwindSafe
for std::option::
IntoIter
<A>
where
    A:
RefUnwindSafe
,
§
impl<A>
RefUnwindSafe
for
IterRange
<A>
where
    A:
RefUnwindSafe
,
§
impl<A>
RefUnwindSafe
for
IterRangeFrom
<A>
where
    A:
RefUnwindSafe
,
§
impl<A>
RefUnwindSafe
for
IterRangeInclusive
<A>
where
    A:
RefUnwindSafe
,
§
impl<A, B>
RefUnwindSafe
for std::iter::
Chain
<A, B>
where
    A:
RefUnwindSafe
,
    B:
RefUnwindSafe
,
§
impl<A, B>
RefUnwindSafe
for
Zip
<A, B>
where
    A:
RefUnwindSafe
,
    B:
RefUnwindSafe
,
§
impl<B>
RefUnwindSafe
for std::io::
Lines
<B>
where
    B:
RefUnwindSafe
,
§
impl<B>
RefUnwindSafe
for std::io::
Split
<B>
where
    B:
RefUnwindSafe
,
§
impl<B, C>
RefUnwindSafe
for
ControlFlow
<B, C>
where
    C:
RefUnwindSafe
,
    B:
RefUnwindSafe
,
§
impl<Dyn> !
RefUnwindSafe
for
DynMetadata
<Dyn>
§
impl<E>
RefUnwindSafe
for
Report
<E>
where
    E:
RefUnwindSafe
,
§
impl<F>
RefUnwindSafe
for std::fmt::
FromFn
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
RefUnwindSafe
for
PollFn
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
RefUnwindSafe
for std::iter::
FromFn
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
RefUnwindSafe
for
OnceWith
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
RefUnwindSafe
for
RepeatWith
<F>
where
    F:
RefUnwindSafe
,
§
impl<G>
RefUnwindSafe
for
FromCoroutine
<G>
where
    G:
RefUnwindSafe
,
§
impl<H>
RefUnwindSafe
for
BuildHasherDefault
<H>
§
impl<I>
RefUnwindSafe
for
FromIter
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
DecodeUtf16
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Cloned
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Copied
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Cycle
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Enumerate
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
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
RefUnwindSafe
,
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Fuse
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
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
RefUnwindSafe
,
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Peekable
<I>
where
    I:
RefUnwindSafe
,
    <I as
Iterator
>::
Item
:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
Skip
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for
StepBy
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
RefUnwindSafe
for std::iter::
Take
<I>
where
    I:
RefUnwindSafe
,
§
impl<I, F>
RefUnwindSafe
for
FilterMap
<I, F>
where
    I:
RefUnwindSafe
,
    F:
RefUnwindSafe
,
§
impl<I, F>
RefUnwindSafe
for
Inspect
<I, F>
where
    I:
RefUnwindSafe
,
    F:
RefUnwindSafe
,
§
impl<I, F>
RefUnwindSafe
for
Map
<I, F>
where
    I:
RefUnwindSafe
,
    F:
RefUnwindSafe
,
§
impl<I, F, const N:
usize
>
RefUnwindSafe
for
MapWindows
<I, F, N>
where
    F:
RefUnwindSafe
,
    I:
RefUnwindSafe
,
    <I as
Iterator
>::
Item
:
RefUnwindSafe
,
§
impl<I, G>
RefUnwindSafe
for
IntersperseWith
<I, G>
where
    G:
RefUnwindSafe
,
    <I as
Iterator
>::
Item
:
RefUnwindSafe
,
    I:
RefUnwindSafe
,
§
impl<I, P>
RefUnwindSafe
for
Filter
<I, P>
where
    I:
RefUnwindSafe
,
    P:
RefUnwindSafe
,
§
impl<I, P>
RefUnwindSafe
for
MapWhile
<I, P>
where
    I:
RefUnwindSafe
,
    P:
RefUnwindSafe
,
§
impl<I, P>
RefUnwindSafe
for
SkipWhile
<I, P>
where
    I:
RefUnwindSafe
,
    P:
RefUnwindSafe
,
§
impl<I, P>
RefUnwindSafe
for
TakeWhile
<I, P>
where
    I:
RefUnwindSafe
,
    P:
RefUnwindSafe
,
§
impl<I, St, F>
RefUnwindSafe
for
Scan
<I, St, F>
where
    I:
RefUnwindSafe
,
    F:
RefUnwindSafe
,
    St:
RefUnwindSafe
,
§
impl<I, U, F>
RefUnwindSafe
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
RefUnwindSafe
,
    I:
RefUnwindSafe
,
    F:
RefUnwindSafe
,
§
impl<I, const N:
usize
>
RefUnwindSafe
for std::iter::
ArrayChunks
<I, N>
where
    I:
RefUnwindSafe
,
    <I as
Iterator
>::
Item
:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::ops::
Range
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::ops::
RangeFrom
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for
RangeTo
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for
RangeToInclusive
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::range::
Range
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::range::
RangeFrom
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
RefUnwindSafe
for std::range::
RangeInclusive
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<K>
RefUnwindSafe
for std::collections::hash_set::
IntoIter
<K>
where
    K:
RefUnwindSafe
,
§
impl<K, V>
RefUnwindSafe
for std::collections::hash_map::
IntoIter
<K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V>
RefUnwindSafe
for std::collections::hash_map::
IntoKeys
<K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V>
RefUnwindSafe
for std::collections::hash_map::
IntoValues
<K, V>
where
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
RefUnwindSafe
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
RefUnwindSafe
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
RefUnwindSafe
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
RefUnwindSafe
for
BTreeMap
<K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, S>
RefUnwindSafe
for
HashMap
<K, V, S>
where
    S:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<Ptr>
RefUnwindSafe
for
Pin
<Ptr>
where
    Ptr:
RefUnwindSafe
,
§
impl<R>
RefUnwindSafe
for
BufReader
<R>
where
    R:
RefUnwindSafe
+ ?
Sized
,
§
impl<R>
RefUnwindSafe
for std::io::
Bytes
<R>
where
    R:
RefUnwindSafe
,
§
impl<Ret, T>
RefUnwindSafe
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T> !
RefUnwindSafe
for
Cell
<T>
§
impl<T> !
RefUnwindSafe
for
OnceCell
<T>
§
impl<T> !
RefUnwindSafe
for
RefCell
<T>
§
impl<T> !
RefUnwindSafe
for
SyncUnsafeCell
<T>
§
impl<T> !
RefUnwindSafe
for
JoinHandle
<T>
§
impl<T>
RefUnwindSafe
for
Bound
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
Option
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
TryLockError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
SendTimeoutError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
TrySendError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
Poll
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
*const T
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
*mut T
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
[T]
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
(T₁, T₂, …, Tₙ)
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
ThinBox
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
Reverse
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
RefUnwindSafe
,
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
Pending
<T>
§
impl<T>
RefUnwindSafe
for
Ready
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for std::io::
Cursor
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for std::io::
Take
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for std::iter::
Empty
<T>
§
impl<T>
RefUnwindSafe
for std::iter::
Once
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
Rev
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
PhantomData
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
ManuallyDrop
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
Saturating
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
Wrapping
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
Yeet
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
NonNull
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for std::result::
IntoIter
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for std::sync::mpmc::
IntoIter
<T>
§
impl<T>
RefUnwindSafe
for std::sync::mpsc::
IntoIter
<T>
§
impl<T>
RefUnwindSafe
for std::sync::mpsc::
Receiver
<T>
§
impl<T>
RefUnwindSafe
for
SendError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for std::sync::mpsc::
Sender
<T>
§
impl<T>
RefUnwindSafe
for
SyncSender
<T>
§
impl<T>
RefUnwindSafe
for
Exclusive
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
RefUnwindSafe
for
PoisonError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
RefUnwindSafe
for
LocalKey
<T>
§
impl<T>
RefUnwindSafe
for
MaybeUninit
<T>
where
    T:
RefUnwindSafe
,
§
impl<T, A =
Global
> !
RefUnwindSafe
for
UniqueRc
<T, A>
§
impl<T, A =
Global
> !
RefUnwindSafe
for std::rc::
Weak
<T, A>
§
impl<T, A>
RefUnwindSafe
for
Box
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T, A>
RefUnwindSafe
for std::collections::binary_heap::
IntoIter
<T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
IntoIterSorted
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
BTreeSet
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
BinaryHeap
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
LinkedList
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
VecDeque
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
Arc
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T, A>
RefUnwindSafe
for std::sync::
Weak
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T, A>
RefUnwindSafe
for std::vec::
IntoIter
<T, A>
where
    T:
RefUnwindSafe
,
    A:
RefUnwindSafe
,
§
impl<T, A>
RefUnwindSafe
for
Vec
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, E>
RefUnwindSafe
for
Result
<T, E>
where
    T:
RefUnwindSafe
,
    E:
RefUnwindSafe
,
§
impl<T, F =
fn
() -> T> !
RefUnwindSafe
for
LazyCell
<T, F>
§
impl<T, F>
RefUnwindSafe
for
Successors
<T, F>
where
    F:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, S>
RefUnwindSafe
for
HashSet
<T, S>
where
    S:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, U>
RefUnwindSafe
for std::io::
Chain
<T, U>
where
    T:
RefUnwindSafe
,
    U:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for
[T; N]
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for std::array::
IntoIter
<T, N>
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for
Mask
<T, N>
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for
Simd
<T, N>
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for [
Option
<T>;
N
]
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for [
MaybeUninit
<T>;
N
]
where
    T:
RefUnwindSafe
,
§
impl<W> !
RefUnwindSafe
for
IntoInnerError
<W>
§
impl<W>
RefUnwindSafe
for
BufWriter
<W>
where
    W:
RefUnwindSafe
+ ?
Sized
,
§
impl<W>
RefUnwindSafe
for
LineWriter
<W>
where
    W:
RefUnwindSafe
+ ?
Sized
,
§
impl<Y, R>
RefUnwindSafe
for
CoroutineState
<Y, R>
where
    Y:
RefUnwindSafe
,
    R:
RefUnwindSafe
,
§
impl<const N:
usize
>
RefUnwindSafe
for
LaneCount
<N>
§
impl<const N:
usize
>
RefUnwindSafe
for [
u8
;
N
]