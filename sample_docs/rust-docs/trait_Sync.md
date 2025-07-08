Sync in std::marker - Rust
std
::
marker
Trait
Sync
Copy item path
1.0.0
·
Source
pub unsafe auto trait Sync { }
Expand description
Types for which it is safe to share references between threads.
This trait is automatically implemented when the compiler determines
it’s appropriate.
The precise definition is: a type
T
is
Sync
if and only if
&T
is
Send
. In other words, if there is no possibility of
undefined behavior
(including data races) when passing
&T
references between threads.
As one would expect, primitive types like
u8
and
f64
are all
Sync
, and so are simple aggregate types containing them,
like tuples, structs and enums. More examples of basic
Sync
types include “immutable” types like
&T
, and those with simple
inherited mutability, such as
Box<T>
,
Vec<T>
and
most other collection types. (Generic parameters need to be
Sync
for their container to be
Sync
.)
A somewhat surprising consequence of the definition is that
&mut T
is
Sync
(if
T
is
Sync
) even though it seems like that might
provide unsynchronized mutation. The trick is that a mutable
reference behind a shared reference (that is,
& &mut T
)
becomes read-only, as if it were a
& &T
. Hence there is no risk
of a data race.
A shorter overview of how
Sync
and
Send
relate to referencing:
&T
is
Send
if and only if
T
is
Sync
&mut T
is
Send
if and only if
T
is
Send
&T
and
&mut T
are
Sync
if and only if
T
is
Sync
Types that are not
Sync
are those that have “interior
mutability” in a non-thread-safe form, such as
Cell
and
RefCell
. These types allow for mutation of
their contents even through an immutable, shared reference. For
example the
set
method on
Cell<T>
takes
&self
, so it requires
only a shared reference
&Cell<T>
. The method performs no
synchronization, thus
Cell
cannot be
Sync
.
Another example of a non-
Sync
type is the reference-counting
pointer
Rc
. Given any reference
&Rc<T>
, you can clone
a new
Rc<T>
, modifying the reference counts in a non-atomic way.
For cases when one does need thread-safe interior mutability,
Rust provides
atomic data types
, as well as explicit locking via
sync::Mutex
and
sync::RwLock
. These types
ensure that any mutation cannot cause data races, hence the types
are
Sync
. Likewise,
sync::Arc
provides a thread-safe
analogue of
Rc
.
Any types with interior mutability must also use the
cell::UnsafeCell
wrapper around the value(s) which
can be mutated through a shared reference. Failing to doing this is
undefined behavior
. For example,
transmute
-ing
from
&T
to
&mut T
is invalid.
See
the Nomicon
for more details about
Sync
.
Implementors
§
1.26.0
·
Source
§
impl !
Sync
for
Args
1.26.0
·
Source
§
impl !
Sync
for
ArgsOs
1.0.0
·
Source
§
impl !
Sync
for
Arguments
<'_>
Source
§
impl !
Sync
for
LocalWaker
Source
§
impl
Sync
for core::ffi::c_str::
Bytes
<'_>
1.63.0
·
Source
§
impl
Sync
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
Sync
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
Sync
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
Sync
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
Sync
for std::string::
Drain
<'_>
1.0.0
·
Source
§
impl
Sync
for
AtomicBool
1.34.0
·
Source
§
impl
Sync
for
AtomicI8
1.34.0
·
Source
§
impl
Sync
for
AtomicI16
1.34.0
·
Source
§
impl
Sync
for
AtomicI32
1.34.0
·
Source
§
impl
Sync
for
AtomicI64
Source
§
impl
Sync
for
AtomicI128
1.0.0
·
Source
§
impl
Sync
for
AtomicIsize
1.34.0
·
Source
§
impl
Sync
for
AtomicU8
1.34.0
·
Source
§
impl
Sync
for
AtomicU16
1.34.0
·
Source
§
impl
Sync
for
AtomicU32
1.34.0
·
Source
§
impl
Sync
for
AtomicU64
Source
§
impl
Sync
for
AtomicU128
1.0.0
·
Source
§
impl
Sync
for
AtomicUsize
1.36.0
·
Source
§
impl
Sync
for
Waker
1.44.0
·
Source
§
impl<'a>
Sync
for
IoSlice
<'a>
1.44.0
·
Source
§
impl<'a>
Sync
for
IoSliceMut
<'a>
Source
§
impl<Dyn>
Sync
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
Sync
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
Sync
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
impl<T> !
Sync
for
Cell
<T>
where
    T: ?
Sized
,
1.70.0
·
Source
§
impl<T> !
Sync
for
OnceCell
<T>
1.0.0
·
Source
§
impl<T> !
Sync
for
RefCell
<T>
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Sync
for
UnsafeCell
<T>
where
    T: ?
Sized
,
1.25.0
·
Source
§
impl<T> !
Sync
for
NonNull
<T>
where
    T: ?
Sized
,
NonNull
pointers are not
Sync
because the data they reference may be aliased.
1.0.0
·
Source
§
impl<T> !
Sync
for std::sync::mpsc::
Receiver
<T>
Source
§
impl<T>
Sync
for
ThinBox
<T>
where
    T:
Sync
+ ?
Sized
,
ThinBox<T>
is
Sync
if
T
is
Sync
because the data is owned.
Source
§
impl<T>
Sync
for
SyncUnsafeCell
<T>
where
    T:
Sync
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Sync
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
Sync
for std::collections::linked_list::
IterMut
<'_, T>
where
    T:
Sync
,
1.28.0
·
Source
§
impl<T>
Sync
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Sync
,
1.31.0
·
Source
§
impl<T>
Sync
for
ChunksExactMut
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
Sync
for
ChunksMut
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
Sync
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
Sync
for std::slice::
IterMut
<'_, T>
where
    T:
Sync
,
1.31.0
·
Source
§
impl<T>
Sync
for
RChunksExactMut
<'_, T>
where
    T:
Sync
,
1.31.0
·
Source
§
impl<T>
Sync
for
RChunksMut
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
Sync
for
AtomicPtr
<T>
Source
§
impl<T>
Sync
for
Exclusive
<T>
where
    T: ?
Sized
,
1.29.0
·
Source
§
impl<T>
Sync
for
JoinHandle
<T>
1.0.0
·
Source
§
impl<T, A> !
Sync
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
Sync
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
Sync
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
Sync
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
Sync
for std::collections::linked_list::
CursorMut
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
1.0.0
·
Source
§
impl<T, A>
Sync
for
LinkedList
<T, A>
where
    T:
Sync
,
    A:
Allocator
+
Sync
,
1.6.0
·
Source
§
impl<T, A>
Sync
for std::collections::vec_deque::
Drain
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
1.0.0
·
Source
§
impl<T, A>
Sync
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
Sync
,
1.4.0
·
Source
§
impl<T, A>
Sync
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
Sync
,
1.6.0
·
Source
§
impl<T, A>
Sync
for std::vec::
Drain
<'_, T, A>
where
    T:
Sync
,
    A:
Sync
+
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Sync
for std::vec::
IntoIter
<T, A>
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
impl<T:
Send
+ ?
Sized
>
Sync
for
ReentrantLock
<T>
Source
§
impl<T:
Send
>
Sync
for std::sync::mpmc::
Receiver
<T>
Source
§
impl<T:
Send
>
Sync
for std::sync::mpmc::
Sender
<T>
1.72.0
·
Source
§
impl<T:
Send
>
Sync
for std::sync::mpsc::
Sender
<T>
1.70.0
·
Source
§
impl<T:
Sync
+
Send
>
Sync
for
OnceLock
<T>
1.80.0
·
Source
§
impl<T:
Sync
+
Send
, F:
Send
>
Sync
for
LazyLock
<T, F>
1.0.0
·
Source
§
impl<T: ?
Sized
+
Send
+
Sync
>
Sync
for
RwLock
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
Sync
for
Mutex
<T>
T
must be
Send
for
Mutex
to be
Sync
.
This ensures that the protected data can be accessed safely from multiple threads
without causing data races or other unsafe behavior.
Mutex<T>
provides mutable access to
T
to one thread at a time. However, it’s essential
for
T
to be
Send
because it’s not safe for non-
Send
structures to be accessed in
this manner. For instance, consider
Rc
, a non-atomic reference counted smart pointer,
which is not
Send
. With
Rc
, we can have multiple copies pointing to the same heap
allocation with a non-atomic reference count. If we were to use
Mutex<Rc<_>>
, it would
only protect one instance of
Rc
from shared access, leaving other copies vulnerable
to potential data races.
Also note that it is not necessary for
T
to be
Sync
as
&T
is only made available
to one thread at a time if
T
is not
Sync
.
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
MappedMutexGuard
<'_, T>
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
MappedRwLockReadGuard
<'_, T>
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
MappedRwLockWriteGuard
<'_, T>
1.19.0
·
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
MutexGuard
<'_, T>
T
must be
Sync
for a
MutexGuard<T>
to be
Sync
because it is possible to get a
&T
from
&MutexGuard
(via
Deref
).
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
ReentrantLockGuard
<'_, T>
1.23.0
·
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
RwLockReadGuard
<'_, T>
1.23.0
·
Source
§
impl<T: ?
Sized
+
Sync
>
Sync
for
RwLockWriteGuard
<'_, T>
Auto implementors
§
§
impl !
Sync
for
Vars
§
impl !
Sync
for
VarsOs
§
impl !
Sync
for
OnceState
§
impl !
Sync
for
RawWaker
§
impl
Sync
for
AsciiChar
§
impl
Sync
for
BacktraceStatus
§
impl
Sync
for std::cmp::
Ordering
§
impl
Sync
for
TryReserveErrorKind
§
impl
Sync
for
Infallible
§
impl
Sync
for
VarError
§
impl
Sync
for
FromBytesWithNulError
§
impl
Sync
for
c_void
§
impl
Sync
for std::fmt::
Alignment
§
impl
Sync
for
DebugAsHex
§
impl
Sync
for
Sign
§
impl
Sync
for
BasicBlock
§
impl
Sync
for
UnwindTerminateReason
§
impl
Sync
for
ErrorKind
§
impl
Sync
for
SeekFrom
§
impl
Sync
for
IpAddr
§
impl
Sync
for
Ipv6MulticastScope
§
impl
Sync
for
Shutdown
§
impl
Sync
for std::net::
SocketAddr
§
impl
Sync
for
FpCategory
§
impl
Sync
for
IntErrorKind
§
impl
Sync
for
OneSidedRangeBound
§
impl
Sync
for
AncillaryError
§
impl
Sync
for
BacktraceStyle
§
impl
Sync
for
GetDisjointMutError
§
impl
Sync
for
SearchStep
§
impl
Sync
for std::sync::atomic::
Ordering
§
impl
Sync
for
RecvTimeoutError
§
impl
Sync
for
TryRecvError
§
impl
Sync
for
bool
§
impl
Sync
for
char
§
impl
Sync
for
f16
§
impl
Sync
for
f32
§
impl
Sync
for
f64
§
impl
Sync
for
f128
§
impl
Sync
for
i8
§
impl
Sync
for
i16
§
impl
Sync
for
i32
§
impl
Sync
for
i64
§
impl
Sync
for
i128
§
impl
Sync
for
isize
§
impl
Sync
for
str
§
impl
Sync
for
u8
§
impl
Sync
for
u16
§
impl
Sync
for
u32
§
impl
Sync
for
u64
§
impl
Sync
for
u128
§
impl
Sync
for
()
§
impl
Sync
for
usize
§
impl
Sync
for
AllocError
§
impl
Sync
for
Global
§
impl
Sync
for
Layout
§
impl
Sync
for
LayoutError
§
impl
Sync
for
System
§
impl
Sync
for
TypeId
§
impl
Sync
for
TryFromSliceError
§
impl
Sync
for std::ascii::
EscapeDefault
§
impl
Sync
for
Backtrace
§
impl
Sync
for
BacktraceFrame
§
impl
Sync
for
ByteStr
§
impl
Sync
for
ByteString
§
impl
Sync
for
BorrowError
§
impl
Sync
for
BorrowMutError
§
impl
Sync
for
CharTryFromError
§
impl
Sync
for
DecodeUtf16Error
§
impl
Sync
for std::char::
EscapeDebug
§
impl
Sync
for std::char::
EscapeDefault
§
impl
Sync
for std::char::
EscapeUnicode
§
impl
Sync
for
ParseCharError
§
impl
Sync
for
ToLowercase
§
impl
Sync
for
ToUppercase
§
impl
Sync
for
TryFromCharError
§
impl
Sync
for
UnorderedKeyError
§
impl
Sync
for
TryReserveError
§
impl
Sync
for
JoinPathsError
§
impl
Sync
for
CStr
§
impl
Sync
for
CString
§
impl
Sync
for
FromBytesUntilNulError
§
impl
Sync
for
FromVecWithNulError
§
impl
Sync
for
IntoStringError
§
impl
Sync
for
NulError
§
impl
Sync
for
OsStr
§
impl
Sync
for
OsString
§
impl
Sync
for std::fmt::
Error
§
impl
Sync
for
FormattingOptions
§
impl
Sync
for
DirBuilder
§
impl
Sync
for
DirEntry
§
impl
Sync
for
File
§
impl
Sync
for
FileTimes
§
impl
Sync
for
FileType
§
impl
Sync
for
Metadata
§
impl
Sync
for
OpenOptions
§
impl
Sync
for
Permissions
§
impl
Sync
for
ReadDir
§
impl
Sync
for
DefaultHasher
§
impl
Sync
for
RandomState
§
impl
Sync
for
SipHasher
§
impl
Sync
for
ReturnToArg
§
impl
Sync
for
UnwindActionArg
§
impl
Sync
for std::io::
Empty
§
impl
Sync
for std::io::
Error
§
impl
Sync
for
PipeReader
§
impl
Sync
for
PipeWriter
§
impl
Sync
for std::io::
Repeat
§
impl
Sync
for
Sink
§
impl
Sync
for
Stderr
§
impl
Sync
for
Stdin
§
impl
Sync
for
Stdout
§
impl
Sync
for
WriterPanicked
§
impl
Sync
for
Assume
§
impl
Sync
for
AddrParseError
§
impl
Sync
for
IntoIncoming
§
impl
Sync
for
Ipv4Addr
§
impl
Sync
for
Ipv6Addr
§
impl
Sync
for
SocketAddrV4
§
impl
Sync
for
SocketAddrV6
§
impl
Sync
for
TcpListener
§
impl
Sync
for
TcpStream
§
impl
Sync
for
UdpSocket
§
impl
Sync
for
ParseFloatError
§
impl
Sync
for
ParseIntError
§
impl
Sync
for
TryFromIntError
§
impl
Sync
for
RangeFull
§
impl
Sync
for
OwnedFd
§
impl
Sync
for
PidFd
§
impl
Sync
for std::os::linux::raw::
stat
§
impl
Sync
for std::os::macos::raw::
stat
§
impl
Sync
for std::os::unix::net::
SocketAddr
§
impl
Sync
for
SocketCred
§
impl
Sync
for
UCred
§
impl
Sync
for
UnixDatagram
§
impl
Sync
for
UnixListener
§
impl
Sync
for
UnixStream
§
impl
Sync
for
InvalidHandleError
§
impl
Sync
for
NullHandleError
§
impl
Sync
for
OwnedSocket
§
impl
Sync
for
Path
§
impl
Sync
for
PathBuf
§
impl
Sync
for
StripPrefixError
§
impl
Sync
for
Child
§
impl
Sync
for
ChildStderr
§
impl
Sync
for
ChildStdin
§
impl
Sync
for
ChildStdout
§
impl
Sync
for
Command
§
impl
Sync
for
ExitCode
§
impl
Sync
for
ExitStatus
§
impl
Sync
for
ExitStatusError
§
impl
Sync
for
Output
§
impl
Sync
for
Stdio
§
impl
Sync
for std::ptr::
Alignment
§
impl
Sync
for
DefaultRandomSource
§
impl
Sync
for
ParseBoolError
§
impl
Sync
for
Utf8Error
§
impl
Sync
for
FromUtf8Error
§
impl
Sync
for
FromUtf16Error
§
impl
Sync
for
IntoChars
§
impl
Sync
for
String
§
impl
Sync
for
RecvError
§
impl
Sync
for
Barrier
§
impl
Sync
for
BarrierWaitResult
§
impl
Sync
for
Condvar
§
impl
Sync
for std::sync::
Once
§
impl
Sync
for
WaitTimeoutResult
§
impl
Sync
for
RawWakerVTable
§
impl
Sync
for
AccessError
§
impl
Sync
for
Builder
§
impl
Sync
for
Thread
§
impl
Sync
for
ThreadId
§
impl
Sync
for
Duration
§
impl
Sync
for
Instant
§
impl
Sync
for
SystemTime
§
impl
Sync
for
SystemTimeError
§
impl
Sync
for
TryFromFloatSecsError
§
impl
Sync
for
PhantomPinned
§
impl<'a> !
Sync
for
Request
<'a>
§
impl<'a> !
Sync
for
Formatter
<'a>
§
impl<'a> !
Sync
for
StderrLock
<'a>
§
impl<'a> !
Sync
for
StdoutLock
<'a>
§
impl<'a> !
Sync
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a> !
Sync
for
PanicHookInfo
<'a>
§
impl<'a> !
Sync
for
Context
<'a>
§
impl<'a> !
Sync
for
ContextBuilder
<'a>
§
impl<'a>
Sync
for
AncillaryData
<'a>
§
impl<'a>
Sync
for
Component
<'a>
§
impl<'a>
Sync
for
Prefix
<'a>
§
impl<'a>
Sync
for
Utf8Pattern
<'a>
§
impl<'a>
Sync
for
SplitPaths
<'a>
§
impl<'a>
Sync
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
Sync
for
BorrowedCursor
<'a>
§
impl<'a>
Sync
for
StdinLock
<'a>
§
impl<'a>
Sync
for std::net::
Incoming
<'a>
§
impl<'a>
Sync
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
Sync
for
Messages
<'a>
§
impl<'a>
Sync
for
ScmCredentials
<'a>
§
impl<'a>
Sync
for
ScmRights
<'a>
§
impl<'a>
Sync
for
SocketAncillary
<'a>
§
impl<'a>
Sync
for
EncodeWide
<'a>
§
impl<'a>
Sync
for
ProcThreadAttributeList
<'a>
§
impl<'a>
Sync
for
Location
<'a>
§
impl<'a>
Sync
for
Ancestors
<'a>
§
impl<'a>
Sync
for
Components
<'a>
§
impl<'a>
Sync
for std::path::
Display
<'a>
§
impl<'a>
Sync
for std::path::
Iter
<'a>
§
impl<'a>
Sync
for
PrefixComponent
<'a>
§
impl<'a>
Sync
for
CommandArgs
<'a>
§
impl<'a>
Sync
for
CommandEnvs
<'a>
§
impl<'a>
Sync
for
EscapeAscii
<'a>
§
impl<'a>
Sync
for
CharSearcher
<'a>
§
impl<'a>
Sync
for std::str::
Bytes
<'a>
§
impl<'a>
Sync
for
CharIndices
<'a>
§
impl<'a>
Sync
for
Chars
<'a>
§
impl<'a>
Sync
for
EncodeUtf16
<'a>
§
impl<'a>
Sync
for std::str::
EscapeDebug
<'a>
§
impl<'a>
Sync
for std::str::
EscapeDefault
<'a>
§
impl<'a>
Sync
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
Sync
for std::str::
Lines
<'a>
§
impl<'a>
Sync
for
LinesAny
<'a>
§
impl<'a>
Sync
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
Sync
for
SplitWhitespace
<'a>
§
impl<'a>
Sync
for
Utf8Chunk
<'a>
§
impl<'a>
Sync
for
Utf8Chunks
<'a>
§
impl<'a>
Sync
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
Sync
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Sync
for
PhantomInvariantLifetime
<'a>
§
impl<'a, 'b> !
Sync
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugSet
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
Sync
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
Sync
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
Sync
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
Sync
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f> !
Sync
for
VaList
<'a, 'f>
§
impl<'a, A>
Sync
for std::option::
Iter
<'a, A>
where
    A:
Sync
,
§
impl<'a, A>
Sync
for std::option::
IterMut
<'a, A>
where
    A:
Sync
,
§
impl<'a, B>
Sync
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Sync
,
    B:
Sync
+ ?
Sized
,
§
impl<'a, F>
Sync
for
CharPredicateSearcher
<'a, F>
where
    F:
Sync
,
§
impl<'a, I>
Sync
for
ByRefSized
<'a, I>
where
    I:
Sync
,
§
impl<'a, I, A>
Sync
for
Splice
<'a, I, A>
where
    I:
Sync
,
    <I as
Iterator
>::
Item
:
Sync
,
    A:
Sync
,
§
impl<'a, K>
Sync
for std::collections::btree_set::
Cursor
<'a, K>
where
    K:
Sync
,
§
impl<'a, K>
Sync
for std::collections::hash_set::
Drain
<'a, K>
where
    K:
Sync
,
§
impl<'a, K>
Sync
for std::collections::hash_set::
Iter
<'a, K>
where
    K:
Sync
,
§
impl<'a, K, A>
Sync
for std::collections::btree_set::
CursorMut
<'a, K, A>
where
    A:
Sync
,
    K:
Sync
,
§
impl<'a, K, A>
Sync
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
where
    A:
Sync
,
    K:
Sync
,
§
impl<'a, K, F>
Sync
for std::collections::hash_set::
ExtractIf
<'a, K, F>
where
    F:
Sync
,
    K:
Sync
,
§
impl<'a, K, V>
Sync
for std::collections::hash_map::
Entry
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
Sync
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
Sync
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
Sync
for std::collections::btree_map::
IterMut
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
Sync
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
Sync
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
Sync
for
RangeMut
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
Sync
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
Sync
for std::collections::btree_map::
ValuesMut
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
Sync
for std::collections::hash_map::
Drain
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
Sync
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
Sync
for std::collections::hash_map::
IterMut
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
Sync
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
Sync
for std::collections::hash_map::
OccupiedEntry
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
Sync
for std::collections::hash_map::
OccupiedError
<'a, K, V>
where
    V:
Sync
,
    K:
Sync
,
§
impl<'a, K, V>
Sync
for std::collections::hash_map::
VacantEntry
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
Sync
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
Sync
for std::collections::hash_map::
ValuesMut
<'a, K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
Entry
<'a, K, V, A>
where
    K:
Sync
,
    A:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
where
    V:
Sync
,
    A:
Sync
,
    K:
Sync
,
§
impl<'a, K, V, A>
Sync
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
where
    K:
Sync
,
    A:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, F>
Sync
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
where
    F:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<'a, K, V, F, A>
Sync
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
where
    F:
Sync
,
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<'a, P>
Sync
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, P>
Sync
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
Sync
,
§
impl<'a, T> !
Sync
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T> !
Sync
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
Sync
for std::collections::binary_heap::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::btree_set::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::btree_set::
Range
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::btree_set::
SymmetricDifference
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::btree_set::
Union
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::vec_deque::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::collections::vec_deque::
IterMut
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::result::
Iter
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::result::
IterMut
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for
Chunks
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for
ChunksExact
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for
RChunks
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for
RChunksExact
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for
Windows
<'a, T>
where
    T:
Sync
,
§
impl<'a, T>
Sync
for std::sync::mpmc::
Iter
<'a, T>
where
    T:
Send
,
§
impl<'a, T>
Sync
for std::sync::mpmc::
TryIter
<'a, T>
where
    T:
Send
,
§
impl<'a, T, A>
Sync
for std::collections::btree_set::
Entry
<'a, T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, A>
Sync
for std::collections::binary_heap::
Drain
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
Sync
for
DrainSorted
<'a, T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, A>
Sync
for
PeekMut
<'a, T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, A>
Sync
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
Sync
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
Sync
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, A>
Sync
for std::collections::btree_set::
VacantEntry
<'a, T, A>
where
    T:
Sync
,
    A:
Sync
,
§
impl<'a, T, F, A =
Global
> !
Sync
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
§
impl<'a, T, F, A>
Sync
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
where
    F:
Sync
,
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, F, A>
Sync
for std::vec::
ExtractIf
<'a, T, F, A>
where
    F:
Sync
,
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
ChunkBy
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
ChunkByMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for std::slice::
RSplit
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
RSplitMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for std::slice::
RSplitN
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
RSplitNMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for std::slice::
Split
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
SplitInclusiveMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
SplitMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for std::slice::
SplitN
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, P>
Sync
for
SplitNMut
<'a, T, P>
where
    P:
Sync
,
    T:
Sync
,
§
impl<'a, T, S>
Sync
for std::collections::hash_set::
Entry
<'a, T, S>
where
    T:
Sync
,
    S:
Sync
,
§
impl<'a, T, S>
Sync
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
Sync
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
Sync
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
where
    T:
Sync
,
    S:
Sync
,
§
impl<'a, T, S>
Sync
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
Sync
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
Sync
for std::collections::hash_set::
VacantEntry
<'a, T, S>
where
    T:
Sync
,
    S:
Sync
,
§
impl<'a, T, const N:
usize
> !
Sync
for
ArrayWindows
<'a, T, N>
§
impl<'a, T, const N:
usize
>
Sync
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
Sync
for
ArrayChunksMut
<'a, T, N>
where
    T:
Sync
,
§
impl<'a, const N:
usize
>
Sync
for
CharArraySearcher
<'a, N>
§
impl<'b, T> !
Sync
for
Ref
<'b, T>
§
impl<'b, T> !
Sync
for
RefMut
<'b, T>
§
impl<'data>
Sync
for
BorrowedBuf
<'data>
§
impl<'f> !
Sync
for
VaListImpl
<'f>
§
impl<'fd>
Sync
for
BorrowedFd
<'fd>
§
impl<'scope, 'env>
Sync
for
Scope
<'scope, 'env>
§
impl<'scope, T>
Sync
for
ScopedJoinHandle
<'scope, T>
where
    T:
Send
,
§
impl<'socket>
Sync
for
BorrowedSocket
<'socket>
§
impl<A>
Sync
for std::iter::
Repeat
<A>
where
    A:
Sync
,
§
impl<A>
Sync
for
RepeatN
<A>
where
    A:
Sync
,
§
impl<A>
Sync
for std::option::
IntoIter
<A>
where
    A:
Sync
,
§
impl<A>
Sync
for
IterRange
<A>
where
    A:
Sync
,
§
impl<A>
Sync
for
IterRangeFrom
<A>
where
    A:
Sync
,
§
impl<A>
Sync
for
IterRangeInclusive
<A>
where
    A:
Sync
,
§
impl<A, B>
Sync
for std::iter::
Chain
<A, B>
where
    A:
Sync
,
    B:
Sync
,
§
impl<A, B>
Sync
for
Zip
<A, B>
where
    A:
Sync
,
    B:
Sync
,
§
impl<B>
Sync
for std::io::
Lines
<B>
where
    B:
Sync
,
§
impl<B>
Sync
for std::io::
Split
<B>
where
    B:
Sync
,
§
impl<B, C>
Sync
for
ControlFlow
<B, C>
where
    C:
Sync
,
    B:
Sync
,
§
impl<E>
Sync
for
Report
<E>
where
    E:
Sync
,
§
impl<F>
Sync
for std::fmt::
FromFn
<F>
where
    F:
Sync
,
§
impl<F>
Sync
for
PollFn
<F>
where
    F:
Sync
,
§
impl<F>
Sync
for std::iter::
FromFn
<F>
where
    F:
Sync
,
§
impl<F>
Sync
for
OnceWith
<F>
where
    F:
Sync
,
§
impl<F>
Sync
for
RepeatWith
<F>
where
    F:
Sync
,
§
impl<G>
Sync
for
FromCoroutine
<G>
where
    G:
Sync
,
§
impl<H>
Sync
for
BuildHasherDefault
<H>
§
impl<I>
Sync
for
FromIter
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
DecodeUtf16
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
Cloned
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
Copied
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
Cycle
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
Enumerate
<I>
where
    I:
Sync
,
§
impl<I>
Sync
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
Sync
,
    I:
Sync
,
§
impl<I>
Sync
for
Fuse
<I>
where
    I:
Sync
,
§
impl<I>
Sync
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
Sync
,
    I:
Sync
,
§
impl<I>
Sync
for
Peekable
<I>
where
    I:
Sync
,
    <I as
Iterator
>::
Item
:
Sync
,
§
impl<I>
Sync
for
Skip
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for
StepBy
<I>
where
    I:
Sync
,
§
impl<I>
Sync
for std::iter::
Take
<I>
where
    I:
Sync
,
§
impl<I, F>
Sync
for
FilterMap
<I, F>
where
    I:
Sync
,
    F:
Sync
,
§
impl<I, F>
Sync
for
Inspect
<I, F>
where
    I:
Sync
,
    F:
Sync
,
§
impl<I, F>
Sync
for
Map
<I, F>
where
    I:
Sync
,
    F:
Sync
,
§
impl<I, F, const N:
usize
>
Sync
for
MapWindows
<I, F, N>
where
    F:
Sync
,
    I:
Sync
,
    <I as
Iterator
>::
Item
:
Sync
,
§
impl<I, G>
Sync
for
IntersperseWith
<I, G>
where
    G:
Sync
,
    <I as
Iterator
>::
Item
:
Sync
,
    I:
Sync
,
§
impl<I, P>
Sync
for
Filter
<I, P>
where
    I:
Sync
,
    P:
Sync
,
§
impl<I, P>
Sync
for
MapWhile
<I, P>
where
    I:
Sync
,
    P:
Sync
,
§
impl<I, P>
Sync
for
SkipWhile
<I, P>
where
    I:
Sync
,
    P:
Sync
,
§
impl<I, P>
Sync
for
TakeWhile
<I, P>
where
    I:
Sync
,
    P:
Sync
,
§
impl<I, St, F>
Sync
for
Scan
<I, St, F>
where
    I:
Sync
,
    F:
Sync
,
    St:
Sync
,
§
impl<I, U, F>
Sync
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
Sync
,
    I:
Sync
,
    F:
Sync
,
§
impl<I, const N:
usize
>
Sync
for std::iter::
ArrayChunks
<I, N>
where
    I:
Sync
,
    <I as
Iterator
>::
Item
:
Sync
,
§
impl<Idx>
Sync
for std::ops::
Range
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for std::ops::
RangeFrom
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for
RangeTo
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for
RangeToInclusive
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for std::range::
Range
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for std::range::
RangeFrom
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Sync
for std::range::
RangeInclusive
<Idx>
where
    Idx:
Sync
,
§
impl<K>
Sync
for std::collections::hash_set::
IntoIter
<K>
where
    K:
Sync
,
§
impl<K, V>
Sync
for std::collections::hash_map::
IntoIter
<K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<K, V>
Sync
for std::collections::hash_map::
IntoKeys
<K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<K, V>
Sync
for std::collections::hash_map::
IntoValues
<K, V>
where
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, A>
Sync
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, A>
Sync
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, A>
Sync
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, A>
Sync
for
BTreeMap
<K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, S>
Sync
for
HashMap
<K, V, S>
where
    S:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<Ptr>
Sync
for
Pin
<Ptr>
where
    Ptr:
Sync
,
§
impl<R>
Sync
for
BufReader
<R>
where
    R:
Sync
+ ?
Sized
,
§
impl<R>
Sync
for std::io::
Bytes
<R>
where
    R:
Sync
,
§
impl<Ret, T>
Sync
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T> !
Sync
for std::sync::mpsc::
IntoIter
<T>
§
impl<T>
Sync
for
Bound
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Option
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
TryLockError
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
SendTimeoutError
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
TrySendError
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Poll
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
[T]
where
    T:
Sync
,
§
impl<T>
Sync
for
(T₁, T₂, …, Tₙ)
where
    T:
Sync
,
§
impl<T>
Sync
for
Reverse
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Sync
,
    T: ?
Sized
,
§
impl<T>
Sync
for
Pending
<T>
§
impl<T>
Sync
for
Ready
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for std::io::
Cursor
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for std::io::
Take
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for std::iter::
Empty
<T>
§
impl<T>
Sync
for std::iter::
Once
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Rev
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Discriminant
<T>
§
impl<T>
Sync
for
ManuallyDrop
<T>
where
    T:
Sync
+ ?
Sized
,
§
impl<T>
Sync
for
Saturating
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Wrapping
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
Yeet
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
AssertUnwindSafe
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for std::result::
IntoIter
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for std::sync::mpmc::
IntoIter
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
SendError
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
SyncSender
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
PoisonError
<T>
where
    T:
Sync
,
§
impl<T>
Sync
for
LocalKey
<T>
§
impl<T>
Sync
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Sync
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Sync
for
PhantomData
<T>
where
    T:
Sync
+ ?
Sized
,
§
impl<T>
Sync
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
Sync
for
MaybeUninit
<T>
where
    T:
Sync
,
§
impl<T, A>
Sync
for
Box
<T, A>
where
    A:
Sync
,
    T:
Sync
+ ?
Sized
,
§
impl<T, A>
Sync
for std::collections::binary_heap::
IntoIter
<T, A>
where
    T:
Sync
,
    A:
Sync
,
§
impl<T, A>
Sync
for
IntoIterSorted
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for std::collections::linked_list::
IntoIter
<T, A>
where
    T:
Sync
,
    A:
Sync
,
§
impl<T, A>
Sync
for
BTreeSet
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for
BinaryHeap
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for
VecDeque
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Sync
for
Vec
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, E>
Sync
for
Result
<T, E>
where
    T:
Sync
,
    E:
Sync
,
§
impl<T, F =
fn
() -> T> !
Sync
for
LazyCell
<T, F>
§
impl<T, F>
Sync
for
Successors
<T, F>
where
    F:
Sync
,
    T:
Sync
,
§
impl<T, S>
Sync
for
HashSet
<T, S>
where
    S:
Sync
,
    T:
Sync
,
§
impl<T, U>
Sync
for std::io::
Chain
<T, U>
where
    T:
Sync
,
    U:
Sync
,
§
impl<T, const N:
usize
>
Sync
for
[T; N]
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Sync
for std::array::
IntoIter
<T, N>
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Sync
for
Mask
<T, N>
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Sync
for
Simd
<T, N>
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Sync
for [
Option
<T>;
N
]
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Sync
for [
MaybeUninit
<T>;
N
]
where
    T:
Sync
,
§
impl<W>
Sync
for
BufWriter
<W>
where
    W:
Sync
+ ?
Sized
,
§
impl<W>
Sync
for
IntoInnerError
<W>
where
    W:
Sync
,
§
impl<W>
Sync
for
LineWriter
<W>
where
    W:
Sync
+ ?
Sized
,
§
impl<Y, R>
Sync
for
CoroutineState
<Y, R>
where
    Y:
Sync
,
    R:
Sync
,
§
impl<const N:
usize
>
Sync
for
LaneCount
<N>
§
impl<const N:
usize
>
Sync
for [
u8
;
N
]