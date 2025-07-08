Freeze in std::marker - Rust
std
::
marker
Trait
Freeze
Copy item path
Source
pub unsafe auto trait Freeze { }
🔬
This is a nightly-only experimental API. (
freeze
#121675
)
Expand description
Used to determine whether a type contains
any
UnsafeCell
internally, but not through an indirection.
This affects, for example, whether a
static
of that type is
placed in read-only static memory or writable static memory.
This can be used to declare that a constant with a generic type
will not contain interior mutability, and subsequently allow
placing the constant behind references.
§
Safety
This trait is a core part of the language, it is just expressed as a trait in libcore for
convenience. Do
not
implement it for other types.
Implementors
§
Source
§
impl<T> !
Freeze
for
UnsafeCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
*const T
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
*mut T
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
&T
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
&mut T
where
    T: ?
Sized
,
1.28.0
·
Source
§
impl<T>
Freeze
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Freeze
,
Source
§
impl<T>
Freeze
for
PhantomData
<T>
where
    T: ?
Sized
,
Auto implementors
§
§
impl !
Freeze
for
Backtrace
§
impl !
Freeze
for
AtomicBool
§
impl !
Freeze
for
AtomicI8
§
impl !
Freeze
for
AtomicI16
§
impl !
Freeze
for
AtomicI32
§
impl !
Freeze
for
AtomicI64
§
impl !
Freeze
for
AtomicI128
§
impl !
Freeze
for
AtomicIsize
§
impl !
Freeze
for
AtomicU8
§
impl !
Freeze
for
AtomicU16
§
impl !
Freeze
for
AtomicU32
§
impl !
Freeze
for
AtomicU64
§
impl !
Freeze
for
AtomicU128
§
impl !
Freeze
for
AtomicUsize
§
impl !
Freeze
for
Barrier
§
impl !
Freeze
for
Condvar
§
impl !
Freeze
for std::sync::
Once
§
impl !
Freeze
for
OnceState
§
impl
Freeze
for
AsciiChar
§
impl
Freeze
for
BacktraceStatus
§
impl
Freeze
for std::cmp::
Ordering
§
impl
Freeze
for
TryReserveErrorKind
§
impl
Freeze
for
Infallible
§
impl
Freeze
for
VarError
§
impl
Freeze
for
FromBytesWithNulError
§
impl
Freeze
for
c_void
§
impl
Freeze
for std::fmt::
Alignment
§
impl
Freeze
for
DebugAsHex
§
impl
Freeze
for
Sign
§
impl
Freeze
for
BasicBlock
§
impl
Freeze
for
UnwindTerminateReason
§
impl
Freeze
for
ErrorKind
§
impl
Freeze
for
SeekFrom
§
impl
Freeze
for
IpAddr
§
impl
Freeze
for
Ipv6MulticastScope
§
impl
Freeze
for
Shutdown
§
impl
Freeze
for std::net::
SocketAddr
§
impl
Freeze
for
FpCategory
§
impl
Freeze
for
IntErrorKind
§
impl
Freeze
for
OneSidedRangeBound
§
impl
Freeze
for
AncillaryError
§
impl
Freeze
for
BacktraceStyle
§
impl
Freeze
for
GetDisjointMutError
§
impl
Freeze
for
SearchStep
§
impl
Freeze
for std::sync::atomic::
Ordering
§
impl
Freeze
for
RecvTimeoutError
§
impl
Freeze
for
TryRecvError
§
impl
Freeze
for
bool
§
impl
Freeze
for
char
§
impl
Freeze
for
f16
§
impl
Freeze
for
f32
§
impl
Freeze
for
f64
§
impl
Freeze
for
f128
§
impl
Freeze
for
i8
§
impl
Freeze
for
i16
§
impl
Freeze
for
i32
§
impl
Freeze
for
i64
§
impl
Freeze
for
i128
§
impl
Freeze
for
isize
§
impl
Freeze
for
str
§
impl
Freeze
for
u8
§
impl
Freeze
for
u16
§
impl
Freeze
for
u32
§
impl
Freeze
for
u64
§
impl
Freeze
for
u128
§
impl
Freeze
for
()
§
impl
Freeze
for
usize
§
impl
Freeze
for
AllocError
§
impl
Freeze
for
Global
§
impl
Freeze
for
Layout
§
impl
Freeze
for
LayoutError
§
impl
Freeze
for
System
§
impl
Freeze
for
TypeId
§
impl
Freeze
for
TryFromSliceError
§
impl
Freeze
for std::ascii::
EscapeDefault
§
impl
Freeze
for
BacktraceFrame
§
impl
Freeze
for
ByteStr
§
impl
Freeze
for
ByteString
§
impl
Freeze
for
BorrowError
§
impl
Freeze
for
BorrowMutError
§
impl
Freeze
for
CharTryFromError
§
impl
Freeze
for
DecodeUtf16Error
§
impl
Freeze
for std::char::
EscapeDebug
§
impl
Freeze
for std::char::
EscapeDefault
§
impl
Freeze
for std::char::
EscapeUnicode
§
impl
Freeze
for
ParseCharError
§
impl
Freeze
for
ToLowercase
§
impl
Freeze
for
ToUppercase
§
impl
Freeze
for
TryFromCharError
§
impl
Freeze
for
UnorderedKeyError
§
impl
Freeze
for
TryReserveError
§
impl
Freeze
for
Args
§
impl
Freeze
for
ArgsOs
§
impl
Freeze
for
JoinPathsError
§
impl
Freeze
for
Vars
§
impl
Freeze
for
VarsOs
§
impl
Freeze
for
CStr
§
impl
Freeze
for
CString
§
impl
Freeze
for
FromBytesUntilNulError
§
impl
Freeze
for
FromVecWithNulError
§
impl
Freeze
for
IntoStringError
§
impl
Freeze
for
NulError
§
impl
Freeze
for
OsStr
§
impl
Freeze
for
OsString
§
impl
Freeze
for std::fmt::
Error
§
impl
Freeze
for
FormattingOptions
§
impl
Freeze
for
DirBuilder
§
impl
Freeze
for
DirEntry
§
impl
Freeze
for
File
§
impl
Freeze
for
FileTimes
§
impl
Freeze
for
FileType
§
impl
Freeze
for
Metadata
§
impl
Freeze
for
OpenOptions
§
impl
Freeze
for
Permissions
§
impl
Freeze
for
ReadDir
§
impl
Freeze
for
DefaultHasher
§
impl
Freeze
for
RandomState
§
impl
Freeze
for
SipHasher
§
impl
Freeze
for
ReturnToArg
§
impl
Freeze
for
UnwindActionArg
§
impl
Freeze
for std::io::
Empty
§
impl
Freeze
for std::io::
Error
§
impl
Freeze
for
PipeReader
§
impl
Freeze
for
PipeWriter
§
impl
Freeze
for std::io::
Repeat
§
impl
Freeze
for
Sink
§
impl
Freeze
for
Stderr
§
impl
Freeze
for
Stdin
§
impl
Freeze
for
Stdout
§
impl
Freeze
for
WriterPanicked
§
impl
Freeze
for
Assume
§
impl
Freeze
for
AddrParseError
§
impl
Freeze
for
IntoIncoming
§
impl
Freeze
for
Ipv4Addr
§
impl
Freeze
for
Ipv6Addr
§
impl
Freeze
for
SocketAddrV4
§
impl
Freeze
for
SocketAddrV6
§
impl
Freeze
for
TcpListener
§
impl
Freeze
for
TcpStream
§
impl
Freeze
for
UdpSocket
§
impl
Freeze
for
ParseFloatError
§
impl
Freeze
for
ParseIntError
§
impl
Freeze
for
TryFromIntError
§
impl
Freeze
for
RangeFull
§
impl
Freeze
for
OwnedFd
§
impl
Freeze
for
PidFd
§
impl
Freeze
for std::os::linux::raw::
stat
§
impl
Freeze
for std::os::macos::raw::
stat
§
impl
Freeze
for std::os::unix::net::
SocketAddr
§
impl
Freeze
for
SocketCred
§
impl
Freeze
for
UCred
§
impl
Freeze
for
UnixDatagram
§
impl
Freeze
for
UnixListener
§
impl
Freeze
for
UnixStream
§
impl
Freeze
for
HandleOrInvalid
§
impl
Freeze
for
HandleOrNull
§
impl
Freeze
for
InvalidHandleError
§
impl
Freeze
for
NullHandleError
§
impl
Freeze
for
OwnedHandle
§
impl
Freeze
for
OwnedSocket
§
impl
Freeze
for
Path
§
impl
Freeze
for
PathBuf
§
impl
Freeze
for
StripPrefixError
§
impl
Freeze
for
Child
§
impl
Freeze
for
ChildStderr
§
impl
Freeze
for
ChildStdin
§
impl
Freeze
for
ChildStdout
§
impl
Freeze
for
Command
§
impl
Freeze
for
ExitCode
§
impl
Freeze
for
ExitStatus
§
impl
Freeze
for
ExitStatusError
§
impl
Freeze
for
Output
§
impl
Freeze
for
Stdio
§
impl
Freeze
for std::ptr::
Alignment
§
impl
Freeze
for
DefaultRandomSource
§
impl
Freeze
for
ParseBoolError
§
impl
Freeze
for
Utf8Error
§
impl
Freeze
for
FromUtf8Error
§
impl
Freeze
for
FromUtf16Error
§
impl
Freeze
for
IntoChars
§
impl
Freeze
for
String
§
impl
Freeze
for
RecvError
§
impl
Freeze
for
BarrierWaitResult
§
impl
Freeze
for
WaitTimeoutResult
§
impl
Freeze
for
LocalWaker
§
impl
Freeze
for
RawWaker
§
impl
Freeze
for
RawWakerVTable
§
impl
Freeze
for
Waker
§
impl
Freeze
for
AccessError
§
impl
Freeze
for
Builder
§
impl
Freeze
for
Thread
§
impl
Freeze
for
ThreadId
§
impl
Freeze
for
Duration
§
impl
Freeze
for
Instant
§
impl
Freeze
for
SystemTime
§
impl
Freeze
for
SystemTimeError
§
impl
Freeze
for
TryFromFloatSecsError
§
impl
Freeze
for
PhantomPinned
§
impl<'a> !
Freeze
for
Request
<'a>
§
impl<'a>
Freeze
for
AncillaryData
<'a>
§
impl<'a>
Freeze
for
Component
<'a>
§
impl<'a>
Freeze
for
Prefix
<'a>
§
impl<'a>
Freeze
for
Utf8Pattern
<'a>
§
impl<'a>
Freeze
for
SplitPaths
<'a>
§
impl<'a>
Freeze
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
Freeze
for
Arguments
<'a>
§
impl<'a>
Freeze
for
Formatter
<'a>
§
impl<'a>
Freeze
for
BorrowedCursor
<'a>
§
impl<'a>
Freeze
for
IoSlice
<'a>
§
impl<'a>
Freeze
for
IoSliceMut
<'a>
§
impl<'a>
Freeze
for
StderrLock
<'a>
§
impl<'a>
Freeze
for
StdinLock
<'a>
§
impl<'a>
Freeze
for
StdoutLock
<'a>
§
impl<'a>
Freeze
for std::net::
Incoming
<'a>
§
impl<'a>
Freeze
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
Freeze
for
Messages
<'a>
§
impl<'a>
Freeze
for
ScmCredentials
<'a>
§
impl<'a>
Freeze
for
ScmRights
<'a>
§
impl<'a>
Freeze
for
SocketAncillary
<'a>
§
impl<'a>
Freeze
for
EncodeWide
<'a>
§
impl<'a>
Freeze
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Freeze
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
Freeze
for
Location
<'a>
§
impl<'a>
Freeze
for
PanicHookInfo
<'a>
§
impl<'a>
Freeze
for
Ancestors
<'a>
§
impl<'a>
Freeze
for
Components
<'a>
§
impl<'a>
Freeze
for std::path::
Display
<'a>
§
impl<'a>
Freeze
for std::path::
Iter
<'a>
§
impl<'a>
Freeze
for
PrefixComponent
<'a>
§
impl<'a>
Freeze
for
CommandArgs
<'a>
§
impl<'a>
Freeze
for
CommandEnvs
<'a>
§
impl<'a>
Freeze
for
EscapeAscii
<'a>
§
impl<'a>
Freeze
for
CharSearcher
<'a>
§
impl<'a>
Freeze
for std::str::
Bytes
<'a>
§
impl<'a>
Freeze
for
CharIndices
<'a>
§
impl<'a>
Freeze
for
Chars
<'a>
§
impl<'a>
Freeze
for
EncodeUtf16
<'a>
§
impl<'a>
Freeze
for std::str::
EscapeDebug
<'a>
§
impl<'a>
Freeze
for std::str::
EscapeDefault
<'a>
§
impl<'a>
Freeze
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
Freeze
for std::str::
Lines
<'a>
§
impl<'a>
Freeze
for
LinesAny
<'a>
§
impl<'a>
Freeze
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
Freeze
for
SplitWhitespace
<'a>
§
impl<'a>
Freeze
for
Utf8Chunk
<'a>
§
impl<'a>
Freeze
for
Utf8Chunks
<'a>
§
impl<'a>
Freeze
for std::string::
Drain
<'a>
§
impl<'a>
Freeze
for
Context
<'a>
§
impl<'a>
Freeze
for
ContextBuilder
<'a>
§
impl<'a>
Freeze
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
Freeze
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Freeze
for
PhantomInvariantLifetime
<'a>
§
impl<'a, 'b>
Freeze
for
DebugList
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
DebugMap
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
DebugSet
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
Freeze
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
Freeze
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f>
Freeze
for
VaList
<'a, 'f>
§
impl<'a, A>
Freeze
for std::option::
Iter
<'a, A>
§
impl<'a, A>
Freeze
for std::option::
IterMut
<'a, A>
§
impl<'a, B>
Freeze
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Freeze
,
    B: ?
Sized
,
§
impl<'a, F>
Freeze
for
CharPredicateSearcher
<'a, F>
where
    F:
Freeze
,
§
impl<'a, I>
Freeze
for
ByRefSized
<'a, I>
§
impl<'a, I, A>
Freeze
for
Splice
<'a, I, A>
where
    I:
Freeze
,
§
impl<'a, K>
Freeze
for std::collections::btree_set::
Cursor
<'a, K>
§
impl<'a, K>
Freeze
for std::collections::hash_set::
Drain
<'a, K>
§
impl<'a, K>
Freeze
for std::collections::hash_set::
Iter
<'a, K>
§
impl<'a, K, A>
Freeze
for std::collections::btree_set::
CursorMut
<'a, K, A>
§
impl<'a, K, A>
Freeze
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
§
impl<'a, K, F>
Freeze
for std::collections::hash_set::
ExtractIf
<'a, K, F>
where
    F:
Freeze
,
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
Entry
<'a, K, V>
where
    K:
Freeze
,
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
Cursor
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
Iter
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
IterMut
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
Keys
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
Range
<'a, K, V>
§
impl<'a, K, V>
Freeze
for
RangeMut
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
Values
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::btree_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
Drain
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
Iter
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
IterMut
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
Keys
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
OccupiedEntry
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
OccupiedError
<'a, K, V>
where
    V:
Freeze
,
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
VacantEntry
<'a, K, V>
where
    K:
Freeze
,
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
Values
<'a, K, V>
§
impl<'a, K, V>
Freeze
for std::collections::hash_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
Entry
<'a, K, V, A>
where
    K:
Freeze
,
    A:
Freeze
,
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
where
    A:
Freeze
,
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
where
    V:
Freeze
,
    A:
Freeze
,
§
impl<'a, K, V, A>
Freeze
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
where
    K:
Freeze
,
    A:
Freeze
,
§
impl<'a, K, V, F>
Freeze
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
where
    F:
Freeze
,
§
impl<'a, K, V, F, A>
Freeze
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
where
    F:
Freeze
,
    A:
Freeze
,
§
impl<'a, P>
Freeze
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, P>
Freeze
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Freeze
,
§
impl<'a, T>
Freeze
for std::collections::binary_heap::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::btree_set::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::btree_set::
Range
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::btree_set::
SymmetricDifference
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::btree_set::
Union
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::linked_list::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::linked_list::
IterMut
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::vec_deque::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::collections::vec_deque::
IterMut
<'a, T>
§
impl<'a, T>
Freeze
for std::result::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::result::
IterMut
<'a, T>
§
impl<'a, T>
Freeze
for
Chunks
<'a, T>
§
impl<'a, T>
Freeze
for
ChunksExact
<'a, T>
§
impl<'a, T>
Freeze
for
ChunksExactMut
<'a, T>
§
impl<'a, T>
Freeze
for
ChunksMut
<'a, T>
§
impl<'a, T>
Freeze
for std::slice::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::slice::
IterMut
<'a, T>
§
impl<'a, T>
Freeze
for
RChunks
<'a, T>
§
impl<'a, T>
Freeze
for
RChunksExact
<'a, T>
§
impl<'a, T>
Freeze
for
RChunksExactMut
<'a, T>
§
impl<'a, T>
Freeze
for
RChunksMut
<'a, T>
§
impl<'a, T>
Freeze
for
Windows
<'a, T>
§
impl<'a, T>
Freeze
for std::sync::mpmc::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::sync::mpmc::
TryIter
<'a, T>
§
impl<'a, T>
Freeze
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T>
Freeze
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
Freeze
for
MappedMutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
MappedRwLockReadGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
MappedRwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
MutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
ReentrantLockGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
RwLockReadGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
Freeze
for
RwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T, A>
Freeze
for std::collections::btree_set::
Entry
<'a, T, A>
where
    A:
Freeze
,
    T:
Freeze
,
§
impl<'a, T, A>
Freeze
for std::collections::binary_heap::
Drain
<'a, T, A>
§
impl<'a, T, A>
Freeze
for
DrainSorted
<'a, T, A>
§
impl<'a, T, A>
Freeze
for
PeekMut
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::collections::btree_set::
Difference
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::collections::btree_set::
Intersection
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
where
    A:
Freeze
,
§
impl<'a, T, A>
Freeze
for std::collections::btree_set::
VacantEntry
<'a, T, A>
where
    T:
Freeze
,
    A:
Freeze
,
§
impl<'a, T, A>
Freeze
for std::collections::linked_list::
Cursor
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::collections::linked_list::
CursorMut
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::collections::vec_deque::
Drain
<'a, T, A>
§
impl<'a, T, A>
Freeze
for std::vec::
Drain
<'a, T, A>
§
impl<'a, T, F, A>
Freeze
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
where
    F:
Freeze
,
    A:
Freeze
,
§
impl<'a, T, F, A>
Freeze
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
where
    F:
Freeze
,
§
impl<'a, T, F, A>
Freeze
for std::vec::
ExtractIf
<'a, T, F, A>
where
    F:
Freeze
,
§
impl<'a, T, P>
Freeze
for
ChunkBy
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
ChunkByMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for std::slice::
RSplit
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
RSplitMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for std::slice::
RSplitN
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
RSplitNMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for std::slice::
Split
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
SplitInclusiveMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
SplitMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for std::slice::
SplitN
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, P>
Freeze
for
SplitNMut
<'a, T, P>
where
    P:
Freeze
,
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
Entry
<'a, T, S>
where
    T:
Freeze
,
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
Difference
<'a, T, S>
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
Intersection
<'a, T, S>
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
SymmetricDifference
<'a, T, S>
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
Union
<'a, T, S>
§
impl<'a, T, S>
Freeze
for std::collections::hash_set::
VacantEntry
<'a, T, S>
where
    T:
Freeze
,
§
impl<'a, T, const N:
usize
>
Freeze
for std::slice::
ArrayChunks
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Freeze
for
ArrayChunksMut
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Freeze
for
ArrayWindows
<'a, T, N>
§
impl<'a, const N:
usize
>
Freeze
for
CharArraySearcher
<'a, N>
§
impl<'b, T>
Freeze
for
Ref
<'b, T>
where
    T: ?
Sized
,
§
impl<'b, T>
Freeze
for
RefMut
<'b, T>
where
    T: ?
Sized
,
§
impl<'data>
Freeze
for
BorrowedBuf
<'data>
§
impl<'f>
Freeze
for
VaListImpl
<'f>
§
impl<'fd>
Freeze
for
BorrowedFd
<'fd>
§
impl<'handle>
Freeze
for
BorrowedHandle
<'handle>
§
impl<'scope, 'env>
Freeze
for
Scope
<'scope, 'env>
§
impl<'scope, T>
Freeze
for
ScopedJoinHandle
<'scope, T>
§
impl<'socket>
Freeze
for
BorrowedSocket
<'socket>
§
impl<A>
Freeze
for std::iter::
Repeat
<A>
where
    A:
Freeze
,
§
impl<A>
Freeze
for
RepeatN
<A>
where
    A:
Freeze
,
§
impl<A>
Freeze
for std::option::
IntoIter
<A>
where
    A:
Freeze
,
§
impl<A>
Freeze
for
IterRange
<A>
where
    A:
Freeze
,
§
impl<A>
Freeze
for
IterRangeFrom
<A>
where
    A:
Freeze
,
§
impl<A>
Freeze
for
IterRangeInclusive
<A>
where
    A:
Freeze
,
§
impl<A, B>
Freeze
for std::iter::
Chain
<A, B>
where
    A:
Freeze
,
    B:
Freeze
,
§
impl<A, B>
Freeze
for
Zip
<A, B>
where
    A:
Freeze
,
    B:
Freeze
,
§
impl<B>
Freeze
for std::io::
Lines
<B>
where
    B:
Freeze
,
§
impl<B>
Freeze
for std::io::
Split
<B>
where
    B:
Freeze
,
§
impl<B, C>
Freeze
for
ControlFlow
<B, C>
where
    C:
Freeze
,
    B:
Freeze
,
§
impl<Dyn>
Freeze
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
§
impl<E>
Freeze
for
Report
<E>
where
    E:
Freeze
,
§
impl<F>
Freeze
for std::fmt::
FromFn
<F>
where
    F:
Freeze
,
§
impl<F>
Freeze
for
PollFn
<F>
where
    F:
Freeze
,
§
impl<F>
Freeze
for std::iter::
FromFn
<F>
where
    F:
Freeze
,
§
impl<F>
Freeze
for
OnceWith
<F>
where
    F:
Freeze
,
§
impl<F>
Freeze
for
RepeatWith
<F>
where
    F:
Freeze
,
§
impl<G>
Freeze
for
FromCoroutine
<G>
where
    G:
Freeze
,
§
impl<H>
Freeze
for
BuildHasherDefault
<H>
§
impl<I>
Freeze
for
FromIter
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
DecodeUtf16
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
Cloned
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
Copied
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
Cycle
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
Enumerate
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
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
Freeze
,
    I:
Freeze
,
§
impl<I>
Freeze
for
Fuse
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
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
Freeze
,
    I:
Freeze
,
§
impl<I>
Freeze
for
Peekable
<I>
where
    I:
Freeze
,
    <I as
Iterator
>::
Item
:
Freeze
,
§
impl<I>
Freeze
for
Skip
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for
StepBy
<I>
where
    I:
Freeze
,
§
impl<I>
Freeze
for std::iter::
Take
<I>
where
    I:
Freeze
,
§
impl<I, F>
Freeze
for
FilterMap
<I, F>
where
    I:
Freeze
,
    F:
Freeze
,
§
impl<I, F>
Freeze
for
Inspect
<I, F>
where
    I:
Freeze
,
    F:
Freeze
,
§
impl<I, F>
Freeze
for
Map
<I, F>
where
    I:
Freeze
,
    F:
Freeze
,
§
impl<I, F, const N:
usize
>
Freeze
for
MapWindows
<I, F, N>
where
    F:
Freeze
,
    I:
Freeze
,
    <I as
Iterator
>::
Item
:
Freeze
,
§
impl<I, G>
Freeze
for
IntersperseWith
<I, G>
where
    G:
Freeze
,
    <I as
Iterator
>::
Item
:
Freeze
,
    I:
Freeze
,
§
impl<I, P>
Freeze
for
Filter
<I, P>
where
    I:
Freeze
,
    P:
Freeze
,
§
impl<I, P>
Freeze
for
MapWhile
<I, P>
where
    I:
Freeze
,
    P:
Freeze
,
§
impl<I, P>
Freeze
for
SkipWhile
<I, P>
where
    I:
Freeze
,
    P:
Freeze
,
§
impl<I, P>
Freeze
for
TakeWhile
<I, P>
where
    I:
Freeze
,
    P:
Freeze
,
§
impl<I, St, F>
Freeze
for
Scan
<I, St, F>
where
    I:
Freeze
,
    F:
Freeze
,
    St:
Freeze
,
§
impl<I, U, F>
Freeze
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
Freeze
,
    I:
Freeze
,
    F:
Freeze
,
§
impl<I, const N:
usize
>
Freeze
for std::iter::
ArrayChunks
<I, N>
where
    I:
Freeze
,
    <I as
Iterator
>::
Item
:
Freeze
,
§
impl<Idx>
Freeze
for std::ops::
Range
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for std::ops::
RangeFrom
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for
RangeTo
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for
RangeToInclusive
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for std::range::
Range
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for std::range::
RangeFrom
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
Freeze
for std::range::
RangeInclusive
<Idx>
where
    Idx:
Freeze
,
§
impl<K>
Freeze
for std::collections::hash_set::
IntoIter
<K>
§
impl<K, V>
Freeze
for std::collections::hash_map::
IntoIter
<K, V>
§
impl<K, V>
Freeze
for std::collections::hash_map::
IntoKeys
<K, V>
§
impl<K, V>
Freeze
for std::collections::hash_map::
IntoValues
<K, V>
§
impl<K, V, A>
Freeze
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Freeze
,
§
impl<K, V, A>
Freeze
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
Freeze
,
§
impl<K, V, A>
Freeze
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Freeze
,
§
impl<K, V, A>
Freeze
for
BTreeMap
<K, V, A>
where
    A:
Freeze
,
§
impl<K, V, S>
Freeze
for
HashMap
<K, V, S>
where
    S:
Freeze
,
§
impl<Ptr>
Freeze
for
Pin
<Ptr>
where
    Ptr:
Freeze
,
§
impl<R>
Freeze
for
BufReader
<R>
where
    R:
Freeze
+ ?
Sized
,
§
impl<R>
Freeze
for std::io::
Bytes
<R>
where
    R:
Freeze
,
§
impl<Ret, T>
Freeze
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T> !
Freeze
for
Cell
<T>
§
impl<T> !
Freeze
for
OnceCell
<T>
§
impl<T> !
Freeze
for
RefCell
<T>
§
impl<T> !
Freeze
for
SyncUnsafeCell
<T>
§
impl<T> !
Freeze
for
AtomicPtr
<T>
§
impl<T> !
Freeze
for
Mutex
<T>
§
impl<T> !
Freeze
for
OnceLock
<T>
§
impl<T> !
Freeze
for
ReentrantLock
<T>
§
impl<T> !
Freeze
for
RwLock
<T>
§
impl<T>
Freeze
for
Bound
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Option
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
TryLockError
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
SendTimeoutError
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
TrySendError
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Poll
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
[T]
where
    T:
Freeze
,
§
impl<T>
Freeze
for
(T₁, T₂, …, Tₙ)
where
    T:
Freeze
,
§
impl<T>
Freeze
for
ThinBox
<T>
where
    T: ?
Sized
,
§
impl<T>
Freeze
for
Reverse
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Freeze
,
    T: ?
Sized
,
§
impl<T>
Freeze
for
Pending
<T>
§
impl<T>
Freeze
for
Ready
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for std::io::
Cursor
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for std::io::
Take
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for std::iter::
Empty
<T>
§
impl<T>
Freeze
for std::iter::
Once
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Rev
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
Freeze
,
§
impl<T>
Freeze
for
ManuallyDrop
<T>
where
    T:
Freeze
+ ?
Sized
,
§
impl<T>
Freeze
for
Saturating
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Wrapping
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
Yeet
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
AssertUnwindSafe
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
NonNull
<T>
where
    T: ?
Sized
,
§
impl<T>
Freeze
for std::result::
IntoIter
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for std::sync::mpmc::
IntoIter
<T>
§
impl<T>
Freeze
for std::sync::mpmc::
Receiver
<T>
§
impl<T>
Freeze
for std::sync::mpmc::
Sender
<T>
§
impl<T>
Freeze
for std::sync::mpsc::
IntoIter
<T>
§
impl<T>
Freeze
for std::sync::mpsc::
Receiver
<T>
§
impl<T>
Freeze
for
SendError
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for std::sync::mpsc::
Sender
<T>
§
impl<T>
Freeze
for
SyncSender
<T>
§
impl<T>
Freeze
for
Exclusive
<T>
where
    T:
Freeze
+ ?
Sized
,
§
impl<T>
Freeze
for
PoisonError
<T>
where
    T:
Freeze
,
§
impl<T>
Freeze
for
JoinHandle
<T>
§
impl<T>
Freeze
for
LocalKey
<T>
§
impl<T>
Freeze
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Freeze
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Freeze
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Freeze
for
MaybeUninit
<T>
where
    T:
Freeze
,
§
impl<T, A>
Freeze
for
Box
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for std::collections::binary_heap::
IntoIter
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
IntoIterSorted
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
BTreeSet
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
BinaryHeap
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
LinkedList
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
VecDeque
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
Rc
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for
UniqueRc
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for std::rc::
Weak
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for
Arc
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for std::sync::
Weak
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
Freeze
for std::vec::
IntoIter
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
Freeze
for
Vec
<T, A>
where
    A:
Freeze
,
§
impl<T, E>
Freeze
for
Result
<T, E>
where
    T:
Freeze
,
    E:
Freeze
,
§
impl<T, F =
fn
() -> T> !
Freeze
for
LazyCell
<T, F>
§
impl<T, F =
fn
() -> T> !
Freeze
for
LazyLock
<T, F>
§
impl<T, F>
Freeze
for
Successors
<T, F>
where
    F:
Freeze
,
    T:
Freeze
,
§
impl<T, S>
Freeze
for
HashSet
<T, S>
where
    S:
Freeze
,
§
impl<T, U>
Freeze
for std::io::
Chain
<T, U>
where
    T:
Freeze
,
    U:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for
[T; N]
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for std::array::
IntoIter
<T, N>
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for
Mask
<T, N>
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for
Simd
<T, N>
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for [
Option
<T>;
N
]
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
Freeze
for [
MaybeUninit
<T>;
N
]
where
    T:
Freeze
,
§
impl<W>
Freeze
for
BufWriter
<W>
where
    W:
Freeze
+ ?
Sized
,
§
impl<W>
Freeze
for
IntoInnerError
<W>
where
    W:
Freeze
,
§
impl<W>
Freeze
for
LineWriter
<W>
where
    W:
Freeze
+ ?
Sized
,
§
impl<Y, R>
Freeze
for
CoroutineState
<Y, R>
where
    Y:
Freeze
,
    R:
Freeze
,
§
impl<const N:
usize
>
Freeze
for
LaneCount
<N>
§
impl<const N:
usize
>
Freeze
for [
u8
;
N
]