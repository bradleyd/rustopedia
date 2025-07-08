Send in std::marker - Rust
std
::
marker
Trait
Send
Copy item path
1.0.0
·
Source
pub unsafe auto trait Send { }
Expand description
Types that can be transferred across thread boundaries.
This trait is automatically implemented when the compiler determines it’s
appropriate.
An example of a non-
Send
type is the reference-counting pointer
rc::Rc
. If two threads attempt to clone
Rc
s that point to the same
reference-counted value, they might try to update the reference count at the
same time, which is
undefined behavior
because
Rc
doesn’t use atomic
operations. Its cousin
sync::Arc
does use atomic operations (incurring
some overhead) and thus is
Send
.
See
the Nomicon
and the
Sync
trait for more details.
Implementors
§
1.26.0
·
Source
§
impl !
Send
for
Args
1.26.0
·
Source
§
impl !
Send
for
ArgsOs
1.0.0
·
Source
§
impl !
Send
for
Arguments
<'_>
Source
§
impl !
Send
for
LocalWaker
Source
§
impl
Send
for core::ffi::c_str::
Bytes
<'_>
1.63.0
·
Source
§
impl
Send
for
BorrowedHandle
<'_>
Available on
Windows
only.
1.63.0
·
Source
§
impl
Send
for
HandleOrInvalid
Available on
Windows
only.
1.63.0
·
Source
§
impl
Send
for
HandleOrNull
Available on
Windows
only.
1.63.0
·
Source
§
impl
Send
for
OwnedHandle
Available on
Windows
only.
1.6.0
·
Source
§
impl
Send
for std::string::
Drain
<'_>
1.36.0
·
Source
§
impl
Send
for
Waker
1.44.0
·
Source
§
impl<'a>
Send
for
IoSlice
<'a>
1.44.0
·
Source
§
impl<'a>
Send
for
IoSliceMut
<'a>
Source
§
impl<Dyn>
Send
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Send
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
impl<T> !
Send
for
*mut T
where
    T: ?
Sized
,
1.25.0
·
Source
§
impl<T> !
Send
for
NonNull
<T>
where
    T: ?
Sized
,
NonNull
pointers are not
Send
because the data they reference may be aliased.
1.0.0
·
Source
§
impl<T>
Send
for
&T
where
    T:
Sync
+ ?
Sized
,
Source
§
impl<T>
Send
for
ThinBox
<T>
where
    T:
Send
+ ?
Sized
,
ThinBox<T>
is
Send
if
T
is
Send
because the data is owned.
1.0.0
·
Source
§
impl<T>
Send
for
Cell
<T>
where
    T:
Send
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Send
for
RefCell
<T>
where
    T:
Send
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Send
for std::collections::linked_list::
Iter
<'_, T>
where
    T:
Sync
,
1.0.0
·
Source
§
impl<T>
Send
for std::collections::linked_list::
IterMut
<'_, T>
where
    T:
Send
,
1.28.0
·
Source
§
impl<T>
Send
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Send
,
1.31.0
·
Source
§
impl<T>
Send
for
ChunksExactMut
<'_, T>
where
    T:
Send
,
1.0.0
·
Source
§
impl<T>
Send
for
ChunksMut
<'_, T>
where
    T:
Send
,
1.0.0
·
Source
§
impl<T>
Send
for std::slice::
Iter
<'_, T>
where
    T:
Sync
,
1.0.0
·
Source
§
impl<T>
Send
for std::slice::
IterMut
<'_, T>
where
    T:
Send
,
1.31.0
·
Source
§
impl<T>
Send
for
RChunksExactMut
<'_, T>
where
    T:
Send
,
1.31.0
·
Source
§
impl<T>
Send
for
RChunksMut
<'_, T>
where
    T:
Send
,
1.0.0
·
Source
§
impl<T>
Send
for
AtomicPtr
<T>
1.29.0
·
Source
§
impl<T>
Send
for
JoinHandle
<T>
1.0.0
·
Source
§
impl<T, A> !
Send
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
impl<T, A> !
Send
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
1.4.0
·
Source
§
impl<T, A> !
Send
for std::rc::
Weak
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
Send
for std::collections::linked_list::
Cursor
<'_, T, A>
where
    T:
Sync
,
    A:
Allocator
+
Sync
,
Source
§
impl<T, A>
Send
for std::collections::linked_list::
CursorMut
<'_, T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
1.0.0
·
Source
§
impl<T, A>
Send
for
LinkedList
<T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
1.6.0
·
Source
§
impl<T, A>
Send
for std::collections::vec_deque::
Drain
<'_, T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
1.0.0
·
Source
§
impl<T, A>
Send
for
Arc
<T, A>
where
    T:
Sync
+
Send
+ ?
Sized
,
    A:
Allocator
+
Send
,
1.4.0
·
Source
§
impl<T, A>
Send
for std::sync::
Weak
<T, A>
where
    T:
Sync
+
Send
+ ?
Sized
,
    A:
Allocator
+
Send
,
1.6.0
·
Source
§
impl<T, A>
Send
for std::vec::
Drain
<'_, T, A>
where
    T:
Send
,
    A:
Send
+
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Send
for std::vec::
IntoIter
<T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
Source
§
impl<T:
Send
+ ?
Sized
>
Send
for
ReentrantLock
<T>
Source
§
impl<T:
Send
>
Send
for std::sync::mpmc::
Receiver
<T>
Source
§
impl<T:
Send
>
Send
for std::sync::mpmc::
Sender
<T>
1.0.0
·
Source
§
impl<T:
Send
>
Send
for std::sync::mpsc::
Receiver
<T>
1.0.0
·
Source
§
impl<T:
Send
>
Send
for std::sync::mpsc::
Sender
<T>
1.0.0
·
Source
§
impl<T:
Send
>
Send
for
SyncSender
<T>
1.70.0
·
Source
§
impl<T:
Send
>
Send
for
OnceLock
<T>
1.0.0
·
Source
§
impl<T: ?
Sized
+
Send
>
Send
for
Mutex
<T>
T
must be
Send
for a
Mutex
to be
Send
because it is possible to acquire
the owned
T
from the
Mutex
via
into_inner
.
1.0.0
·
Source
§
impl<T: ?
Sized
+
Send
>
Send
for
RwLock
<T>
Source
§
impl<T: ?
Sized
> !
Send
for
MappedMutexGuard
<'_, T>
Source
§
impl<T: ?
Sized
> !
Send
for
MappedRwLockReadGuard
<'_, T>
Source
§
impl<T: ?
Sized
> !
Send
for
MappedRwLockWriteGuard
<'_, T>
1.0.0
·
Source
§
impl<T: ?
Sized
> !
Send
for
MutexGuard
<'_, T>
A
MutexGuard
is not
Send
to maximize platform portablity.
On platforms that use POSIX threads (commonly referred to as pthreads) there is a requirement to
release mutex locks on the same thread they were acquired.
For this reason,
MutexGuard
must not implement
Send
to prevent it being dropped from
another thread.
Source
§
impl<T: ?
Sized
> !
Send
for
ReentrantLockGuard
<'_, T>
1.0.0
·
Source
§
impl<T: ?
Sized
> !
Send
for
RwLockReadGuard
<'_, T>
1.0.0
·
Source
§
impl<T: ?
Sized
> !
Send
for
RwLockWriteGuard
<'_, T>
Auto implementors
§
§
impl !
Send
for
Vars
§
impl !
Send
for
VarsOs
§
impl !
Send
for
OnceState
§
impl !
Send
for
RawWaker
§
impl
Send
for
AsciiChar
§
impl
Send
for
BacktraceStatus
§
impl
Send
for std::cmp::
Ordering
§
impl
Send
for
TryReserveErrorKind
§
impl
Send
for
Infallible
§
impl
Send
for
VarError
§
impl
Send
for
FromBytesWithNulError
§
impl
Send
for
c_void
§
impl
Send
for std::fmt::
Alignment
§
impl
Send
for
DebugAsHex
§
impl
Send
for
Sign
§
impl
Send
for
BasicBlock
§
impl
Send
for
UnwindTerminateReason
§
impl
Send
for
ErrorKind
§
impl
Send
for
SeekFrom
§
impl
Send
for
IpAddr
§
impl
Send
for
Ipv6MulticastScope
§
impl
Send
for
Shutdown
§
impl
Send
for std::net::
SocketAddr
§
impl
Send
for
FpCategory
§
impl
Send
for
IntErrorKind
§
impl
Send
for
OneSidedRangeBound
§
impl
Send
for
AncillaryError
§
impl
Send
for
BacktraceStyle
§
impl
Send
for
GetDisjointMutError
§
impl
Send
for
SearchStep
§
impl
Send
for std::sync::atomic::
Ordering
§
impl
Send
for
RecvTimeoutError
§
impl
Send
for
TryRecvError
§
impl
Send
for
bool
§
impl
Send
for
char
§
impl
Send
for
f16
§
impl
Send
for
f32
§
impl
Send
for
f64
§
impl
Send
for
f128
§
impl
Send
for
i8
§
impl
Send
for
i16
§
impl
Send
for
i32
§
impl
Send
for
i64
§
impl
Send
for
i128
§
impl
Send
for
isize
§
impl
Send
for
str
§
impl
Send
for
u8
§
impl
Send
for
u16
§
impl
Send
for
u32
§
impl
Send
for
u64
§
impl
Send
for
u128
§
impl
Send
for
()
§
impl
Send
for
usize
§
impl
Send
for
AllocError
§
impl
Send
for
Global
§
impl
Send
for
Layout
§
impl
Send
for
LayoutError
§
impl
Send
for
System
§
impl
Send
for
TypeId
§
impl
Send
for
TryFromSliceError
§
impl
Send
for std::ascii::
EscapeDefault
§
impl
Send
for
Backtrace
§
impl
Send
for
BacktraceFrame
§
impl
Send
for
ByteStr
§
impl
Send
for
ByteString
§
impl
Send
for
BorrowError
§
impl
Send
for
BorrowMutError
§
impl
Send
for
CharTryFromError
§
impl
Send
for
DecodeUtf16Error
§
impl
Send
for std::char::
EscapeDebug
§
impl
Send
for std::char::
EscapeDefault
§
impl
Send
for std::char::
EscapeUnicode
§
impl
Send
for
ParseCharError
§
impl
Send
for
ToLowercase
§
impl
Send
for
ToUppercase
§
impl
Send
for
TryFromCharError
§
impl
Send
for
UnorderedKeyError
§
impl
Send
for
TryReserveError
§
impl
Send
for
JoinPathsError
§
impl
Send
for
CStr
§
impl
Send
for
CString
§
impl
Send
for
FromBytesUntilNulError
§
impl
Send
for
FromVecWithNulError
§
impl
Send
for
IntoStringError
§
impl
Send
for
NulError
§
impl
Send
for
OsStr
§
impl
Send
for
OsString
§
impl
Send
for std::fmt::
Error
§
impl
Send
for
FormattingOptions
§
impl
Send
for
DirBuilder
§
impl
Send
for
DirEntry
§
impl
Send
for
File
§
impl
Send
for
FileTimes
§
impl
Send
for
FileType
§
impl
Send
for
Metadata
§
impl
Send
for
OpenOptions
§
impl
Send
for
Permissions
§
impl
Send
for
ReadDir
§
impl
Send
for
DefaultHasher
§
impl
Send
for
RandomState
§
impl
Send
for
SipHasher
§
impl
Send
for
ReturnToArg
§
impl
Send
for
UnwindActionArg
§
impl
Send
for std::io::
Empty
§
impl
Send
for std::io::
Error
§
impl
Send
for
PipeReader
§
impl
Send
for
PipeWriter
§
impl
Send
for std::io::
Repeat
§
impl
Send
for
Sink
§
impl
Send
for
Stderr
§
impl
Send
for
Stdin
§
impl
Send
for
Stdout
§
impl
Send
for
WriterPanicked
§
impl
Send
for
Assume
§
impl
Send
for
AddrParseError
§
impl
Send
for
IntoIncoming
§
impl
Send
for
Ipv4Addr
§
impl
Send
for
Ipv6Addr
§
impl
Send
for
SocketAddrV4
§
impl
Send
for
SocketAddrV6
§
impl
Send
for
TcpListener
§
impl
Send
for
TcpStream
§
impl
Send
for
UdpSocket
§
impl
Send
for
ParseFloatError
§
impl
Send
for
ParseIntError
§
impl
Send
for
TryFromIntError
§
impl
Send
for
RangeFull
§
impl
Send
for
OwnedFd
§
impl
Send
for
PidFd
§
impl
Send
for std::os::linux::raw::
stat
§
impl
Send
for std::os::macos::raw::
stat
§
impl
Send
for std::os::unix::net::
SocketAddr
§
impl
Send
for
SocketCred
§
impl
Send
for
UCred
§
impl
Send
for
UnixDatagram
§
impl
Send
for
UnixListener
§
impl
Send
for
UnixStream
§
impl
Send
for
InvalidHandleError
§
impl
Send
for
NullHandleError
§
impl
Send
for
OwnedSocket
§
impl
Send
for
Path
§
impl
Send
for
PathBuf
§
impl
Send
for
StripPrefixError
§
impl
Send
for
Child
§
impl
Send
for
ChildStderr
§
impl
Send
for
ChildStdin
§
impl
Send
for
ChildStdout
§
impl
Send
for
Command
§
impl
Send
for
ExitCode
§
impl
Send
for
ExitStatus
§
impl
Send
for
ExitStatusError
§
impl
Send
for
Output
§
impl
Send
for
Stdio
§
impl
Send
for std::ptr::
Alignment
§
impl
Send
for
DefaultRandomSource
§
impl
Send
for
ParseBoolError
§
impl
Send
for
Utf8Error
§
impl
Send
for
FromUtf8Error
§
impl
Send
for
FromUtf16Error
§
impl
Send
for
IntoChars
§
impl
Send
for
String
§
impl
Send
for
AtomicBool
§
impl
Send
for
AtomicI8
§
impl
Send
for
AtomicI16
§
impl
Send
for
AtomicI32
§
impl
Send
for
AtomicI64
§
impl
Send
for
AtomicI128
§
impl
Send
for
AtomicIsize
§
impl
Send
for
AtomicU8
§
impl
Send
for
AtomicU16
§
impl
Send
for
AtomicU32
§
impl
Send
for
AtomicU64
§
impl
Send
for
AtomicU128
§
impl
Send
for
AtomicUsize
§
impl
Send
for
RecvError
§
impl
Send
for
Barrier
§
impl
Send
for
BarrierWaitResult
§
impl
Send
for
Condvar
§
impl
Send
for std::sync::
Once
§
impl
Send
for
WaitTimeoutResult
§
impl
Send
for
RawWakerVTable
§
impl
Send
for
AccessError
§
impl
Send
for
Builder
§
impl
Send
for
Thread
§
impl
Send
for
ThreadId
§
impl
Send
for
Duration
§
impl
Send
for
Instant
§
impl
Send
for
SystemTime
§
impl
Send
for
SystemTimeError
§
impl
Send
for
TryFromFloatSecsError
§
impl
Send
for
PhantomPinned
§
impl<'a> !
Send
for
Request
<'a>
§
impl<'a> !
Send
for
Formatter
<'a>
§
impl<'a> !
Send
for
StderrLock
<'a>
§
impl<'a> !
Send
for
StdinLock
<'a>
§
impl<'a> !
Send
for
StdoutLock
<'a>
§
impl<'a> !
Send
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a> !
Send
for
PanicHookInfo
<'a>
§
impl<'a> !
Send
for
Context
<'a>
§
impl<'a> !
Send
for
ContextBuilder
<'a>
§
impl<'a>
Send
for
AncillaryData
<'a>
§
impl<'a>
Send
for
Component
<'a>
§
impl<'a>
Send
for
Prefix
<'a>
§
impl<'a>
Send
for
Utf8Pattern
<'a>
§
impl<'a>
Send
for
SplitPaths
<'a>
§
impl<'a>
Send
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
Send
for
BorrowedCursor
<'a>
§
impl<'a>
Send
for std::net::
Incoming
<'a>
§
impl<'a>
Send
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
Send
for
Messages
<'a>
§
impl<'a>
Send
for
ScmCredentials
<'a>
§
impl<'a>
Send
for
ScmRights
<'a>
§
impl<'a>
Send
for
SocketAncillary
<'a>
§
impl<'a>
Send
for
EncodeWide
<'a>
§
impl<'a>
Send
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Send
for
Location
<'a>
§
impl<'a>
Send
for
Ancestors
<'a>
§
impl<'a>
Send
for
Components
<'a>
§
impl<'a>
Send
for std::path::
Display
<'a>
§
impl<'a>
Send
for std::path::
Iter
<'a>
§
impl<'a>
Send
for
PrefixComponent
<'a>
§
impl<'a>
Send
for
CommandArgs
<'a>
§
impl<'a>
Send
for
CommandEnvs
<'a>
§
impl<'a>
Send
for
EscapeAscii
<'a>
§
impl<'a>
Send
for
CharSearcher
<'a>
§
impl<'a>
Send
for std::str::
Bytes
<'a>
§
impl<'a>
Send
for
CharIndices
<'a>
§
impl<'a>
Send
for
Chars
<'a>
§
impl<'a>
Send
for
EncodeUtf16
<'a>
§
impl<'a>
Send
for std::str::
EscapeDebug
<'a>
§
impl<'a>
Send
for std::str::
EscapeDefault
<'a>
§
impl<'a>
Send
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
Send
for std::str::
Lines
<'a>
§
impl<'a>
Send
for
LinesAny
<'a>
§
impl<'a>
Send
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
Send
for
SplitWhitespace
<'a>
§
impl<'a>
Send
for
Utf8Chunk
<'a>
§
impl<'a>
Send
for
Utf8Chunks
<'a>
§
impl<'a>
Send
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
Send
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Send
for
PhantomInvariantLifetime
<'a>
§
impl<'a, 'b> !
Send
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugSet
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
Send
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
Send
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
Send
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
Send
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f> !
Send
for
VaList
<'a, 'f>
§
impl<'a, A>
Send
for std::option::
Iter
<'a, A>
where
    A:
Sync
,
§
impl<'a, A>
Send
for std::option::
IterMut
<'a, A>
where
    A:
Send
,
§
impl<'a, B>
Send
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Send
,
    B:
Sync
+ ?
Sized
,
§
impl<'a, F>
Send
for
CharPredicateSearcher
<'a, F>
where
    F:
Send
,
§
impl<'a, I>
Send
for
ByRefSized
<'a, I>
where
    I:
Send
,
§
impl<'a, I, A>
Send
for
Splice
<'a, I, A>
where
    I:
Send
,
    <I as
Iterator
>::
Item
:
Send
,
    A:
Send
,
§
impl<'a, K>
Send
for std::collections::btree_set::
Cursor
<'a, K>
where
    K:
Sync
,
§
impl<'a, K>
Send
for std::collections::hash_set::
Drain
<'a, K>
where
    K:
Send
,
§
impl<'a, K>
Send
for std::collections::hash_set::
Iter
<'a, K>
where
    K:
Sync
,
§
impl<'a, K, A>
Send
for std::collections::btree_set::
CursorMut
<'a, K, A>
where
    A:
Send
,
    K:
Send
,
§
impl<'a, K, A>
Send
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
where
    A:
Send
,
    K:
Send
,
§
impl<'a, K, F>
Send
for std::collections::hash_set::
ExtractIf
<'a, K, F>
where
    F:
Send
,
    K:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
Entry
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
Cursor
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
Iter
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
IterMut
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
Keys
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
Range
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for
RangeMut
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
Values
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::btree_map::
ValuesMut
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
Drain
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
Iter
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
IterMut
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
Keys
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
OccupiedEntry
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
OccupiedError
<'a, K, V>
where
    V:
Send
,
    K:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
VacantEntry
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
Values
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V>
Send
for std::collections::hash_map::
ValuesMut
<'a, K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
Entry
<'a, K, V, A>
where
    K:
Send
,
    A:
Send
,
    V:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
where
    V:
Send
,
    A:
Send
,
    K:
Send
,
§
impl<'a, K, V, A>
Send
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
where
    K:
Send
,
    A:
Send
,
    V:
Send
,
§
impl<'a, K, V, F>
Send
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
where
    F:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<'a, K, V, F, A>
Send
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
where
    F:
Send
,
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<'a, P>
Send
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, P>
Send
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Send
,
§
impl<'a, T> !
Send
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T> !
Send
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
Send
for std::collections::binary_heap::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::btree_set::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::btree_set::
Range
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::btree_set::
SymmetricDifference
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::btree_set::
Union
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::vec_deque::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::collections::vec_deque::
IterMut
<'a, T>
where
    T:
Send
,
§
impl<'a, T>
Send
for std::result::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::result::
IterMut
<'a, T>
where
    T:
Send
,
§
impl<'a, T>
Send
for
Chunks
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for
ChunksExact
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for
RChunks
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for
RChunksExact
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for
Windows
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Send
for std::sync::mpmc::
Iter
<'a, T>
where
    T:
Send
,
§
impl<'a, T>
Send
for std::sync::mpmc::
TryIter
<'a, T>
where
    T:
Send
,
§
impl<'a, T, A>
Send
for std::collections::btree_set::
Entry
<'a, T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<'a, T, A>
Send
for std::collections::binary_heap::
Drain
<'a, T, A>
where
    T:
Send
,
    A:
Send
,
§
impl<'a, T, A>
Send
for
DrainSorted
<'a, T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<'a, T, A>
Send
for
PeekMut
<'a, T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<'a, T, A>
Send
for std::collections::btree_set::
Difference
<'a, T, A>
where
    T:
Sync
,
    A:
Sync
,
§
impl<'a, T, A>
Send
for std::collections::btree_set::
Intersection
<'a, T, A>
where
    T:
Sync
,
    A:
Sync
,
§
impl<'a, T, A>
Send
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<'a, T, A>
Send
for std::collections::btree_set::
VacantEntry
<'a, T, A>
where
    T:
Send
,
    A:
Send
,
§
impl<'a, T, F, A =
Global
> !
Send
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
§
impl<'a, T, F, A>
Send
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
where
    F:
Send
,
    A:
Send
,
    T:
Send
,
§
impl<'a, T, F, A>
Send
for std::vec::
ExtractIf
<'a, T, F, A>
where
    F:
Send
,
    A:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for
ChunkBy
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for
ChunkByMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for std::slice::
RSplit
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for
RSplitMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for std::slice::
RSplitN
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for
RSplitNMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for std::slice::
Split
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for
SplitInclusiveMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for
SplitMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, P>
Send
for std::slice::
SplitN
<'a, T, P>
where
    P:
Send
,
    T:
Sync
,
§
impl<'a, T, P>
Send
for
SplitNMut
<'a, T, P>
where
    P:
Send
,
    T:
Send
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
Entry
<'a, T, S>
where
    T:
Send
,
    S:
Send
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
Difference
<'a, T, S>
where
    S:
Sync
,
    T:
Sync
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
Intersection
<'a, T, S>
where
    S:
Sync
,
    T:
Sync
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
where
    T:
Send
,
    S:
Send
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
SymmetricDifference
<'a, T, S>
where
    S:
Sync
,
    T:
Sync
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
Union
<'a, T, S>
where
    S:
Sync
,
    T:
Sync
,
§
impl<'a, T, S>
Send
for std::collections::hash_set::
VacantEntry
<'a, T, S>
where
    T:
Send
,
    S:
Send
,
§
impl<'a, T, const N:
usize
> !
Send
for
ArrayWindows
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Send
for std::slice::
ArrayChunks
<'a, T, N>
where
    T:
Sync
,
§
impl<'a, T, const N:
usize
>
Send
for
ArrayChunksMut
<'a, T, N>
where
    T:
Send
,
§
impl<'a, const N:
usize
>
Send
for
CharArraySearcher
<'a, N>
§
impl<'b, T> !
Send
for
Ref
<'b, T>
§
impl<'b, T> !
Send
for
RefMut
<'b, T>
§
impl<'data>
Send
for
BorrowedBuf
<'data>
§
impl<'f> !
Send
for
VaListImpl
<'f>
§
impl<'fd>
Send
for
BorrowedFd
<'fd>
§
impl<'scope, 'env>
Send
for
Scope
<'scope, 'env>
§
impl<'scope, T>
Send
for
ScopedJoinHandle
<'scope, T>
where
    T:
Send
,
§
impl<'socket>
Send
for
BorrowedSocket
<'socket>
§
impl<A>
Send
for std::iter::
Repeat
<A>
where
    A:
Send
,
§
impl<A>
Send
for
RepeatN
<A>
where
    A:
Send
,
§
impl<A>
Send
for std::option::
IntoIter
<A>
where
    A:
Send
,
§
impl<A>
Send
for
IterRange
<A>
where
    A:
Send
,
§
impl<A>
Send
for
IterRangeFrom
<A>
where
    A:
Send
,
§
impl<A>
Send
for
IterRangeInclusive
<A>
where
    A:
Send
,
§
impl<A, B>
Send
for std::iter::
Chain
<A, B>
where
    A:
Send
,
    B:
Send
,
§
impl<A, B>
Send
for
Zip
<A, B>
where
    A:
Send
,
    B:
Send
,
§
impl<B>
Send
for std::io::
Lines
<B>
where
    B:
Send
,
§
impl<B>
Send
for std::io::
Split
<B>
where
    B:
Send
,
§
impl<B, C>
Send
for
ControlFlow
<B, C>
where
    C:
Send
,
    B:
Send
,
§
impl<E>
Send
for
Report
<E>
where
    E:
Send
,
§
impl<F>
Send
for std::fmt::
FromFn
<F>
where
    F:
Send
,
§
impl<F>
Send
for
PollFn
<F>
where
    F:
Send
,
§
impl<F>
Send
for std::iter::
FromFn
<F>
where
    F:
Send
,
§
impl<F>
Send
for
OnceWith
<F>
where
    F:
Send
,
§
impl<F>
Send
for
RepeatWith
<F>
where
    F:
Send
,
§
impl<G>
Send
for
FromCoroutine
<G>
where
    G:
Send
,
§
impl<H>
Send
for
BuildHasherDefault
<H>
§
impl<I>
Send
for
FromIter
<I>
where
    I:
Send
,
§
impl<I>
Send
for
DecodeUtf16
<I>
where
    I:
Send
,
§
impl<I>
Send
for
Cloned
<I>
where
    I:
Send
,
§
impl<I>
Send
for
Copied
<I>
where
    I:
Send
,
§
impl<I>
Send
for
Cycle
<I>
where
    I:
Send
,
§
impl<I>
Send
for
Enumerate
<I>
where
    I:
Send
,
§
impl<I>
Send
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
Send
,
    I:
Send
,
§
impl<I>
Send
for
Fuse
<I>
where
    I:
Send
,
§
impl<I>
Send
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
Send
,
    I:
Send
,
§
impl<I>
Send
for
Peekable
<I>
where
    I:
Send
,
    <I as
Iterator
>::
Item
:
Send
,
§
impl<I>
Send
for
Skip
<I>
where
    I:
Send
,
§
impl<I>
Send
for
StepBy
<I>
where
    I:
Send
,
§
impl<I>
Send
for std::iter::
Take
<I>
where
    I:
Send
,
§
impl<I, F>
Send
for
FilterMap
<I, F>
where
    I:
Send
,
    F:
Send
,
§
impl<I, F>
Send
for
Inspect
<I, F>
where
    I:
Send
,
    F:
Send
,
§
impl<I, F>
Send
for
Map
<I, F>
where
    I:
Send
,
    F:
Send
,
§
impl<I, F, const N:
usize
>
Send
for
MapWindows
<I, F, N>
where
    F:
Send
,
    I:
Send
,
    <I as
Iterator
>::
Item
:
Send
,
§
impl<I, G>
Send
for
IntersperseWith
<I, G>
where
    G:
Send
,
    <I as
Iterator
>::
Item
:
Send
,
    I:
Send
,
§
impl<I, P>
Send
for
Filter
<I, P>
where
    I:
Send
,
    P:
Send
,
§
impl<I, P>
Send
for
MapWhile
<I, P>
where
    I:
Send
,
    P:
Send
,
§
impl<I, P>
Send
for
SkipWhile
<I, P>
where
    I:
Send
,
    P:
Send
,
§
impl<I, P>
Send
for
TakeWhile
<I, P>
where
    I:
Send
,
    P:
Send
,
§
impl<I, St, F>
Send
for
Scan
<I, St, F>
where
    I:
Send
,
    F:
Send
,
    St:
Send
,
§
impl<I, U, F>
Send
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
Send
,
    I:
Send
,
    F:
Send
,
§
impl<I, const N:
usize
>
Send
for std::iter::
ArrayChunks
<I, N>
where
    I:
Send
,
    <I as
Iterator
>::
Item
:
Send
,
§
impl<Idx>
Send
for std::ops::
Range
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for std::ops::
RangeFrom
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for
RangeTo
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for
RangeToInclusive
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for std::range::
Range
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for std::range::
RangeFrom
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Send
for std::range::
RangeInclusive
<Idx>
where
    Idx:
Send
,
§
impl<K>
Send
for std::collections::hash_set::
IntoIter
<K>
where
    K:
Send
,
§
impl<K, V>
Send
for std::collections::hash_map::
IntoIter
<K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<K, V>
Send
for std::collections::hash_map::
IntoKeys
<K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<K, V>
Send
for std::collections::hash_map::
IntoValues
<K, V>
where
    K:
Send
,
    V:
Send
,
§
impl<K, V, A>
Send
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<K, V, A>
Send
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<K, V, A>
Send
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<K, V, A>
Send
for
BTreeMap
<K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<K, V, S>
Send
for
HashMap
<K, V, S>
where
    S:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<Ptr>
Send
for
Pin
<Ptr>
where
    Ptr:
Send
,
§
impl<R>
Send
for
BufReader
<R>
where
    R:
Send
+ ?
Sized
,
§
impl<R>
Send
for std::io::
Bytes
<R>
where
    R:
Send
,
§
impl<Ret, T>
Send
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T>
Send
for
Bound
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Option
<T>
where
    T:
Send
,
§
impl<T>
Send
for
TryLockError
<T>
where
    T:
Send
,
§
impl<T>
Send
for
SendTimeoutError
<T>
where
    T:
Send
,
§
impl<T>
Send
for
TrySendError
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Poll
<T>
where
    T:
Send
,
§
impl<T>
Send
for
[T]
where
    T:
Send
,
§
impl<T>
Send
for
(T₁, T₂, …, Tₙ)
where
    T:
Send
,
§
impl<T>
Send
for
OnceCell
<T>
where
    T:
Send
,
§
impl<T>
Send
for
SyncUnsafeCell
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Send
for
UnsafeCell
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Send
for
Reverse
<T>
where
    T:
Send
,
§
impl<T>
Send
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Send
,
    T: ?
Sized
,
§
impl<T>
Send
for
Pending
<T>
§
impl<T>
Send
for
Ready
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::io::
Cursor
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::io::
Take
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::iter::
Empty
<T>
§
impl<T>
Send
for std::iter::
Once
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Rev
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Discriminant
<T>
§
impl<T>
Send
for
ManuallyDrop
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Send
for
Saturating
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Wrapping
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Yeet
<T>
where
    T:
Send
,
§
impl<T>
Send
for
AssertUnwindSafe
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::result::
IntoIter
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::sync::mpmc::
IntoIter
<T>
where
    T:
Send
,
§
impl<T>
Send
for std::sync::mpsc::
IntoIter
<T>
where
    T:
Send
,
§
impl<T>
Send
for
SendError
<T>
where
    T:
Send
,
§
impl<T>
Send
for
Exclusive
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Send
for
PoisonError
<T>
where
    T:
Send
,
§
impl<T>
Send
for
LocalKey
<T>
§
impl<T>
Send
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Send
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Send
for
PhantomData
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Send
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Send
for
MaybeUninit
<T>
where
    T:
Send
,
§
impl<T, A>
Send
for
Box
<T, A>
where
    A:
Send
,
    T:
Send
+ ?
Sized
,
§
impl<T, A>
Send
for std::collections::binary_heap::
IntoIter
<T, A>
where
    T:
Send
,
    A:
Send
,
§
impl<T, A>
Send
for
IntoIterSorted
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for std::collections::linked_list::
IntoIter
<T, A>
where
    T:
Send
,
    A:
Send
,
§
impl<T, A>
Send
for
BTreeSet
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for
BinaryHeap
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for
VecDeque
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Send
for
Vec
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, E>
Send
for
Result
<T, E>
where
    T:
Send
,
    E:
Send
,
§
impl<T, F>
Send
for
LazyCell
<T, F>
where
    F:
Send
,
    T:
Send
,
§
impl<T, F>
Send
for
Successors
<T, F>
where
    F:
Send
,
    T:
Send
,
§
impl<T, F>
Send
for
LazyLock
<T, F>
where
    T:
Send
,
    F:
Send
,
§
impl<T, S>
Send
for
HashSet
<T, S>
where
    S:
Send
,
    T:
Send
,
§
impl<T, U>
Send
for std::io::
Chain
<T, U>
where
    T:
Send
,
    U:
Send
,
§
impl<T, const N:
usize
>
Send
for
[T; N]
where
    T:
Send
,
§
impl<T, const N:
usize
>
Send
for std::array::
IntoIter
<T, N>
where
    T:
Send
,
§
impl<T, const N:
usize
>
Send
for
Mask
<T, N>
where
    T:
Send
,
§
impl<T, const N:
usize
>
Send
for
Simd
<T, N>
where
    T:
Send
,
§
impl<T, const N:
usize
>
Send
for [
Option
<T>;
N
]
where
    T:
Send
,
§
impl<T, const N:
usize
>
Send
for [
MaybeUninit
<T>;
N
]
where
    T:
Send
,
§
impl<W>
Send
for
BufWriter
<W>
where
    W:
Send
+ ?
Sized
,
§
impl<W>
Send
for
IntoInnerError
<W>
where
    W:
Send
,
§
impl<W>
Send
for
LineWriter
<W>
where
    W:
Send
+ ?
Sized
,
§
impl<Y, R>
Send
for
CoroutineState
<Y, R>
where
    Y:
Send
,
    R:
Send
,
§
impl<const N:
usize
>
Send
for
LaneCount
<N>
§
impl<const N:
usize
>
Send
for [
u8
;
N
]