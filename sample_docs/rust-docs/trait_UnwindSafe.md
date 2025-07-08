UnwindSafe in std::panic - Rust
std
::
panic
Trait
UnwindSafe
Copy item path
1.9.0
·
Source
pub auto trait UnwindSafe { }
Expand description
A marker trait which represents “panic safe” types in Rust.
This trait is implemented by default for many types and behaves similarly in
terms of inference of implementation to the
Send
and
Sync
traits. The
purpose of this trait is to encode what types are safe to cross a
catch_unwind
boundary with no fear of unwind safety.
§
What is unwind safety?
In Rust a function can “return” early if it either panics or calls a
function which transitively panics. This sort of control flow is not always
anticipated, and has the possibility of causing subtle bugs through a
combination of two critical components:
A data structure is in a temporarily invalid state when the thread
panics.
This broken invariant is then later observed.
Typically in Rust, it is difficult to perform step (2) because catching a
panic involves either spawning a thread (which in turn makes it difficult
to later witness broken invariants) or using the
catch_unwind
function in this
module. Additionally, even if an invariant is witnessed, it typically isn’t a
problem in Rust because there are no uninitialized values (like in C or C++).
It is possible, however, for
logical
invariants to be broken in Rust,
which can end up causing behavioral bugs. Another key aspect of unwind safety
in Rust is that, in the absence of
unsafe
code, a panic cannot lead to
memory unsafety.
That was a bit of a whirlwind tour of unwind safety, but for more information
about unwind safety and how it applies to Rust, see an
associated RFC
.
§
What is
UnwindSafe
?
Now that we’ve got an idea of what unwind safety is in Rust, it’s also
important to understand what this trait represents. As mentioned above, one
way to witness broken invariants is through the
catch_unwind
function in this
module as it allows catching a panic and then re-using the environment of
the closure.
Simply put, a type
T
implements
UnwindSafe
if it cannot easily allow
witnessing a broken invariant through the use of
catch_unwind
(catching a
panic). This trait is an auto trait, so it is automatically implemented for
many types, and it is also structurally composed (e.g., a struct is unwind
safe if all of its components are unwind safe).
Note, however, that this is not an unsafe trait, so there is not a succinct
contract that this trait is providing. Instead it is intended as more of a
“speed bump” to alert users of
catch_unwind
that broken invariants may be
witnessed and may need to be accounted for.
§
Who implements
UnwindSafe
?
Types such as
&mut T
and
&RefCell<T>
are examples which are
not
unwind safe. The general idea is that any mutable state which can be shared
across
catch_unwind
is not unwind safe by default. This is because it is very
easy to witness a broken invariant outside of
catch_unwind
as the data is
simply accessed as usual.
Types like
&Mutex<T>
, however, are unwind safe because they implement
poisoning by default. They still allow witnessing a broken invariant, but
they already provide their own “speed bumps” to do so.
§
When should
UnwindSafe
be used?
It is not intended that most types or functions need to worry about this trait.
It is only used as a bound on the
catch_unwind
function and as mentioned
above, the lack of
unsafe
means it is mostly an advisory. The
AssertUnwindSafe
wrapper struct can be used to force this trait to be
implemented for any closed over variables passed to
catch_unwind
.
Implementors
§
1.9.0
·
Source
§
impl
UnwindSafe
for
Stderr
1.9.0
·
Source
§
impl
UnwindSafe
for
StderrLock
<'_>
1.9.0
·
Source
§
impl
UnwindSafe
for
Stdout
1.9.0
·
Source
§
impl
UnwindSafe
for
StdoutLock
<'_>
1.9.0
·
Source
§
impl
UnwindSafe
for
Condvar
1.59.0
·
Source
§
impl
UnwindSafe
for std::sync::
Once
1.64.0
·
Source
§
impl<K, V, A>
UnwindSafe
for
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
+
UnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
1.36.0
·
Source
§
impl<K, V, S>
UnwindSafe
for
HashMap
<K, V, S>
where
    K:
UnwindSafe
,
    V:
UnwindSafe
,
    S:
UnwindSafe
,
1.9.0
·
Source
§
impl<T> !
UnwindSafe
for
&mut T
where
    T: ?
Sized
,
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
*const T
where
    T:
RefUnwindSafe
+ ?
Sized
,
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
*mut T
where
    T:
RefUnwindSafe
+ ?
Sized
,
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
&T
where
    T:
RefUnwindSafe
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
UnwindSafe
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
UnwindSafe
,
1.25.0
·
Source
§
impl<T>
UnwindSafe
for
NonNull
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
Source
§
impl<T>
UnwindSafe
for std::sync::mpmc::
Receiver
<T>
Source
§
impl<T>
UnwindSafe
for std::sync::mpmc::
Sender
<T>
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
AssertUnwindSafe
<T>
1.9.0
·
Source
§
impl<T, A>
UnwindSafe
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
1.9.0
·
Source
§
impl<T, A>
UnwindSafe
for
Arc
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
Source
§
impl<T:
UnwindSafe
+ ?
Sized
>
UnwindSafe
for
ReentrantLock
<T>
1.70.0
·
Source
§
impl<T:
UnwindSafe
>
UnwindSafe
for
OnceLock
<T>
1.80.0
·
Source
§
impl<T:
UnwindSafe
, F:
UnwindSafe
>
UnwindSafe
for
LazyLock
<T, F>
1.9.0
·
Source
§
impl<T: ?
Sized
>
UnwindSafe
for
Mutex
<T>
1.9.0
·
Source
§
impl<T: ?
Sized
>
UnwindSafe
for
RwLock
<T>
Auto implementors
§
§
impl !
UnwindSafe
for std::io::
Error
§
impl !
UnwindSafe
for
Command
§
impl
UnwindSafe
for
AsciiChar
§
impl
UnwindSafe
for
BacktraceStatus
§
impl
UnwindSafe
for std::cmp::
Ordering
§
impl
UnwindSafe
for
TryReserveErrorKind
§
impl
UnwindSafe
for
Infallible
§
impl
UnwindSafe
for
VarError
§
impl
UnwindSafe
for
FromBytesWithNulError
§
impl
UnwindSafe
for
c_void
§
impl
UnwindSafe
for std::fmt::
Alignment
§
impl
UnwindSafe
for
DebugAsHex
§
impl
UnwindSafe
for
Sign
§
impl
UnwindSafe
for
BasicBlock
§
impl
UnwindSafe
for
UnwindTerminateReason
§
impl
UnwindSafe
for
ErrorKind
§
impl
UnwindSafe
for
SeekFrom
§
impl
UnwindSafe
for
IpAddr
§
impl
UnwindSafe
for
Ipv6MulticastScope
§
impl
UnwindSafe
for
Shutdown
§
impl
UnwindSafe
for std::net::
SocketAddr
§
impl
UnwindSafe
for
FpCategory
§
impl
UnwindSafe
for
IntErrorKind
§
impl
UnwindSafe
for
OneSidedRangeBound
§
impl
UnwindSafe
for
AncillaryError
§
impl
UnwindSafe
for
GetDisjointMutError
§
impl
UnwindSafe
for
SearchStep
§
impl
UnwindSafe
for std::sync::atomic::
Ordering
§
impl
UnwindSafe
for
RecvTimeoutError
§
impl
UnwindSafe
for
TryRecvError
§
impl
UnwindSafe
for
BacktraceStyle
§
impl
UnwindSafe
for
bool
§
impl
UnwindSafe
for
char
§
impl
UnwindSafe
for
f16
§
impl
UnwindSafe
for
f32
§
impl
UnwindSafe
for
f64
§
impl
UnwindSafe
for
f128
§
impl
UnwindSafe
for
i8
§
impl
UnwindSafe
for
i16
§
impl
UnwindSafe
for
i32
§
impl
UnwindSafe
for
i64
§
impl
UnwindSafe
for
i128
§
impl
UnwindSafe
for
isize
§
impl
UnwindSafe
for
str
§
impl
UnwindSafe
for
u8
§
impl
UnwindSafe
for
u16
§
impl
UnwindSafe
for
u32
§
impl
UnwindSafe
for
u64
§
impl
UnwindSafe
for
u128
§
impl
UnwindSafe
for
()
§
impl
UnwindSafe
for
usize
§
impl
UnwindSafe
for
AllocError
§
impl
UnwindSafe
for
Global
§
impl
UnwindSafe
for
Layout
§
impl
UnwindSafe
for
LayoutError
§
impl
UnwindSafe
for
System
§
impl
UnwindSafe
for
TypeId
§
impl
UnwindSafe
for
TryFromSliceError
§
impl
UnwindSafe
for std::ascii::
EscapeDefault
§
impl
UnwindSafe
for
Backtrace
§
impl
UnwindSafe
for
BacktraceFrame
§
impl
UnwindSafe
for
ByteStr
§
impl
UnwindSafe
for
ByteString
§
impl
UnwindSafe
for
BorrowError
§
impl
UnwindSafe
for
BorrowMutError
§
impl
UnwindSafe
for
CharTryFromError
§
impl
UnwindSafe
for
DecodeUtf16Error
§
impl
UnwindSafe
for std::char::
EscapeDebug
§
impl
UnwindSafe
for std::char::
EscapeDefault
§
impl
UnwindSafe
for std::char::
EscapeUnicode
§
impl
UnwindSafe
for
ParseCharError
§
impl
UnwindSafe
for
ToLowercase
§
impl
UnwindSafe
for
ToUppercase
§
impl
UnwindSafe
for
TryFromCharError
§
impl
UnwindSafe
for
UnorderedKeyError
§
impl
UnwindSafe
for
TryReserveError
§
impl
UnwindSafe
for
Args
§
impl
UnwindSafe
for
ArgsOs
§
impl
UnwindSafe
for
JoinPathsError
§
impl
UnwindSafe
for
Vars
§
impl
UnwindSafe
for
VarsOs
§
impl
UnwindSafe
for
CStr
§
impl
UnwindSafe
for
CString
§
impl
UnwindSafe
for
FromBytesUntilNulError
§
impl
UnwindSafe
for
FromVecWithNulError
§
impl
UnwindSafe
for
IntoStringError
§
impl
UnwindSafe
for
NulError
§
impl
UnwindSafe
for
OsStr
§
impl
UnwindSafe
for
OsString
§
impl
UnwindSafe
for std::fmt::
Error
§
impl
UnwindSafe
for
FormattingOptions
§
impl
UnwindSafe
for
DirBuilder
§
impl
UnwindSafe
for
DirEntry
§
impl
UnwindSafe
for
File
§
impl
UnwindSafe
for
FileTimes
§
impl
UnwindSafe
for
FileType
§
impl
UnwindSafe
for
Metadata
§
impl
UnwindSafe
for
OpenOptions
§
impl
UnwindSafe
for
Permissions
§
impl
UnwindSafe
for
ReadDir
§
impl
UnwindSafe
for
DefaultHasher
§
impl
UnwindSafe
for
RandomState
§
impl
UnwindSafe
for
SipHasher
§
impl
UnwindSafe
for
ReturnToArg
§
impl
UnwindSafe
for
UnwindActionArg
§
impl
UnwindSafe
for std::io::
Empty
§
impl
UnwindSafe
for
PipeReader
§
impl
UnwindSafe
for
PipeWriter
§
impl
UnwindSafe
for std::io::
Repeat
§
impl
UnwindSafe
for
Sink
§
impl
UnwindSafe
for
Stdin
§
impl
UnwindSafe
for
WriterPanicked
§
impl
UnwindSafe
for
PhantomPinned
§
impl
UnwindSafe
for
Assume
§
impl
UnwindSafe
for
AddrParseError
§
impl
UnwindSafe
for
IntoIncoming
§
impl
UnwindSafe
for
Ipv4Addr
§
impl
UnwindSafe
for
Ipv6Addr
§
impl
UnwindSafe
for
SocketAddrV4
§
impl
UnwindSafe
for
SocketAddrV6
§
impl
UnwindSafe
for
TcpListener
§
impl
UnwindSafe
for
TcpStream
§
impl
UnwindSafe
for
UdpSocket
§
impl
UnwindSafe
for
ParseFloatError
§
impl
UnwindSafe
for
ParseIntError
§
impl
UnwindSafe
for
TryFromIntError
§
impl
UnwindSafe
for
RangeFull
§
impl
UnwindSafe
for
OwnedFd
§
impl
UnwindSafe
for
PidFd
§
impl
UnwindSafe
for std::os::linux::raw::
stat
§
impl
UnwindSafe
for std::os::macos::raw::
stat
§
impl
UnwindSafe
for std::os::unix::net::
SocketAddr
§
impl
UnwindSafe
for
SocketCred
§
impl
UnwindSafe
for
UCred
§
impl
UnwindSafe
for
UnixDatagram
§
impl
UnwindSafe
for
UnixListener
§
impl
UnwindSafe
for
UnixStream
§
impl
UnwindSafe
for
HandleOrInvalid
§
impl
UnwindSafe
for
HandleOrNull
§
impl
UnwindSafe
for
InvalidHandleError
§
impl
UnwindSafe
for
NullHandleError
§
impl
UnwindSafe
for
OwnedHandle
§
impl
UnwindSafe
for
OwnedSocket
§
impl
UnwindSafe
for
Path
§
impl
UnwindSafe
for
PathBuf
§
impl
UnwindSafe
for
StripPrefixError
§
impl
UnwindSafe
for
Child
§
impl
UnwindSafe
for
ChildStderr
§
impl
UnwindSafe
for
ChildStdin
§
impl
UnwindSafe
for
ChildStdout
§
impl
UnwindSafe
for
ExitCode
§
impl
UnwindSafe
for
ExitStatus
§
impl
UnwindSafe
for
ExitStatusError
§
impl
UnwindSafe
for
Output
§
impl
UnwindSafe
for
Stdio
§
impl
UnwindSafe
for std::ptr::
Alignment
§
impl
UnwindSafe
for
DefaultRandomSource
§
impl
UnwindSafe
for
ParseBoolError
§
impl
UnwindSafe
for
Utf8Error
§
impl
UnwindSafe
for
FromUtf8Error
§
impl
UnwindSafe
for
FromUtf16Error
§
impl
UnwindSafe
for
IntoChars
§
impl
UnwindSafe
for
String
§
impl
UnwindSafe
for
AtomicBool
§
impl
UnwindSafe
for
AtomicI8
§
impl
UnwindSafe
for
AtomicI16
§
impl
UnwindSafe
for
AtomicI32
§
impl
UnwindSafe
for
AtomicI64
§
impl
UnwindSafe
for
AtomicI128
§
impl
UnwindSafe
for
AtomicIsize
§
impl
UnwindSafe
for
AtomicU8
§
impl
UnwindSafe
for
AtomicU16
§
impl
UnwindSafe
for
AtomicU32
§
impl
UnwindSafe
for
AtomicU64
§
impl
UnwindSafe
for
AtomicU128
§
impl
UnwindSafe
for
AtomicUsize
§
impl
UnwindSafe
for
RecvError
§
impl
UnwindSafe
for
Barrier
§
impl
UnwindSafe
for
BarrierWaitResult
§
impl
UnwindSafe
for
OnceState
§
impl
UnwindSafe
for
WaitTimeoutResult
§
impl
UnwindSafe
for
LocalWaker
§
impl
UnwindSafe
for
RawWaker
§
impl
UnwindSafe
for
RawWakerVTable
§
impl
UnwindSafe
for
Waker
§
impl
UnwindSafe
for
AccessError
§
impl
UnwindSafe
for
Builder
§
impl
UnwindSafe
for
Thread
§
impl
UnwindSafe
for
ThreadId
§
impl
UnwindSafe
for
Duration
§
impl
UnwindSafe
for
Instant
§
impl
UnwindSafe
for
SystemTime
§
impl
UnwindSafe
for
SystemTimeError
§
impl
UnwindSafe
for
TryFromFloatSecsError
§
impl<'a> !
UnwindSafe
for
Request
<'a>
§
impl<'a> !
UnwindSafe
for
Formatter
<'a>
§
impl<'a> !
UnwindSafe
for
BorrowedCursor
<'a>
§
impl<'a> !
UnwindSafe
for
IoSliceMut
<'a>
§
impl<'a> !
UnwindSafe
for
SocketAncillary
<'a>
§
impl<'a> !
UnwindSafe
for
ContextBuilder
<'a>
§
impl<'a> !
UnwindSafe
for
PanicHookInfo
<'a>
§
impl<'a>
UnwindSafe
for
AncillaryData
<'a>
§
impl<'a>
UnwindSafe
for
Component
<'a>
§
impl<'a>
UnwindSafe
for
Prefix
<'a>
§
impl<'a>
UnwindSafe
for
Utf8Pattern
<'a>
§
impl<'a>
UnwindSafe
for
SplitPaths
<'a>
§
impl<'a>
UnwindSafe
for std::ffi::os_str::
Display
<'a>
§
impl<'a>
UnwindSafe
for
Arguments
<'a>
§
impl<'a>
UnwindSafe
for
IoSlice
<'a>
§
impl<'a>
UnwindSafe
for
StdinLock
<'a>
§
impl<'a>
UnwindSafe
for
PhantomContravariantLifetime
<'a>
§
impl<'a>
UnwindSafe
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
UnwindSafe
for
PhantomInvariantLifetime
<'a>
§
impl<'a>
UnwindSafe
for std::net::
Incoming
<'a>
§
impl<'a>
UnwindSafe
for std::os::unix::net::
Incoming
<'a>
§
impl<'a>
UnwindSafe
for
Messages
<'a>
§
impl<'a>
UnwindSafe
for
ScmCredentials
<'a>
§
impl<'a>
UnwindSafe
for
ScmRights
<'a>
§
impl<'a>
UnwindSafe
for
EncodeWide
<'a>
§
impl<'a>
UnwindSafe
for
ProcThreadAttributeList
<'a>
§
impl<'a>
UnwindSafe
for
ProcThreadAttributeListBuilder
<'a>
§
impl<'a>
UnwindSafe
for
Ancestors
<'a>
§
impl<'a>
UnwindSafe
for
Components
<'a>
§
impl<'a>
UnwindSafe
for std::path::
Display
<'a>
§
impl<'a>
UnwindSafe
for std::path::
Iter
<'a>
§
impl<'a>
UnwindSafe
for
PrefixComponent
<'a>
§
impl<'a>
UnwindSafe
for
CommandArgs
<'a>
§
impl<'a>
UnwindSafe
for
CommandEnvs
<'a>
§
impl<'a>
UnwindSafe
for
EscapeAscii
<'a>
§
impl<'a>
UnwindSafe
for
CharSearcher
<'a>
§
impl<'a>
UnwindSafe
for std::str::
Bytes
<'a>
§
impl<'a>
UnwindSafe
for
CharIndices
<'a>
§
impl<'a>
UnwindSafe
for
Chars
<'a>
§
impl<'a>
UnwindSafe
for
EncodeUtf16
<'a>
§
impl<'a>
UnwindSafe
for std::str::
EscapeDebug
<'a>
§
impl<'a>
UnwindSafe
for std::str::
EscapeDefault
<'a>
§
impl<'a>
UnwindSafe
for std::str::
EscapeUnicode
<'a>
§
impl<'a>
UnwindSafe
for std::str::
Lines
<'a>
§
impl<'a>
UnwindSafe
for
LinesAny
<'a>
§
impl<'a>
UnwindSafe
for
SplitAsciiWhitespace
<'a>
§
impl<'a>
UnwindSafe
for
SplitWhitespace
<'a>
§
impl<'a>
UnwindSafe
for
Utf8Chunk
<'a>
§
impl<'a>
UnwindSafe
for
Utf8Chunks
<'a>
§
impl<'a>
UnwindSafe
for std::string::
Drain
<'a>
§
impl<'a>
UnwindSafe
for
Context
<'a>
§
impl<'a>
UnwindSafe
for
Location
<'a>
§
impl<'a, 'b> !
UnwindSafe
for
DebugList
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugMap
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugSet
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugStruct
<'a, 'b>
§
impl<'a, 'b> !
UnwindSafe
for
DebugTuple
<'a, 'b>
§
impl<'a, 'b>
UnwindSafe
for
CharSliceSearcher
<'a, 'b>
§
impl<'a, 'b>
UnwindSafe
for
StrSearcher
<'a, 'b>
§
impl<'a, 'b, const N:
usize
>
UnwindSafe
for
CharArrayRefSearcher
<'a, 'b, N>
§
impl<'a, 'f> !
UnwindSafe
for
VaList
<'a, 'f>
§
impl<'a, A> !
UnwindSafe
for std::option::
IterMut
<'a, A>
§
impl<'a, A>
UnwindSafe
for std::option::
Iter
<'a, A>
where
    A:
RefUnwindSafe
,
§
impl<'a, B>
UnwindSafe
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
UnwindSafe
,
    B:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, F>
UnwindSafe
for
CharPredicateSearcher
<'a, F>
where
    F:
UnwindSafe
,
§
impl<'a, I> !
UnwindSafe
for
ByRefSized
<'a, I>
§
impl<'a, I, A>
UnwindSafe
for
Splice
<'a, I, A>
where
    I:
UnwindSafe
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
UnwindSafe
for std::collections::btree_set::
Cursor
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K>
UnwindSafe
for std::collections::hash_set::
Drain
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K>
UnwindSafe
for std::collections::hash_set::
Iter
<'a, K>
where
    K:
RefUnwindSafe
,
§
impl<'a, K, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
CursorMut
<'a, K, A>
§
impl<'a, K, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
CursorMutKey
<'a, K, A>
§
impl<'a, K, F> !
UnwindSafe
for std::collections::hash_set::
ExtractIf
<'a, K, F>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
Entry
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::btree_map::
IterMut
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for
RangeMut
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::btree_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
IterMut
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
OccupiedEntry
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
OccupiedError
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
VacantEntry
<'a, K, V>
§
impl<'a, K, V> !
UnwindSafe
for std::collections::hash_map::
ValuesMut
<'a, K, V>
§
impl<'a, K, V>
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
Entry
<'a, K, V, A>
§
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
CursorMut
<'a, K, V, A>
§
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
CursorMutKey
<'a, K, V, A>
§
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
OccupiedEntry
<'a, K, V, A>
§
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
OccupiedError
<'a, K, V, A>
§
impl<'a, K, V, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
VacantEntry
<'a, K, V, A>
§
impl<'a, K, V, F> !
UnwindSafe
for std::collections::hash_map::
ExtractIf
<'a, K, V, F>
§
impl<'a, K, V, F, A =
Global
> !
UnwindSafe
for std::collections::btree_map::
ExtractIf
<'a, K, V, F, A>
§
impl<'a, P>
UnwindSafe
for
MatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for
Matches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for
RMatchIndices
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for
RMatches
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for std::str::
RSplit
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for std::str::
RSplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for
RSplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for std::str::
Split
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for std::str::
SplitInclusive
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for std::str::
SplitN
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, P>
UnwindSafe
for
SplitTerminator
<'a, P>
where
    <P as
Pattern
>::
Searcher
<'a>:
UnwindSafe
,
§
impl<'a, T> !
UnwindSafe
for std::collections::linked_list::
IterMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for std::collections::vec_deque::
IterMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for std::result::
IterMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
ChunksExactMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
ChunksMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for std::slice::
IterMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
RChunksExactMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
RChunksMut
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
MappedMutexGuard
<'a, T>
§
impl<'a, T> !
UnwindSafe
for
MappedRwLockWriteGuard
<'a, T>
§
impl<'a, T>
UnwindSafe
for std::collections::binary_heap::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::btree_set::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::btree_set::
Range
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::btree_set::
SymmetricDifference
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::btree_set::
Union
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::linked_list::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::collections::vec_deque::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::result::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for
Chunks
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for
ChunksExact
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::slice::
Iter
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for
RChunks
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for
RChunksExact
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for
Windows
<'a, T>
where
    T:
RefUnwindSafe
,
§
impl<'a, T>
UnwindSafe
for std::sync::mpmc::
Iter
<'a, T>
§
impl<'a, T>
UnwindSafe
for std::sync::mpmc::
TryIter
<'a, T>
§
impl<'a, T>
UnwindSafe
for std::sync::mpsc::
Iter
<'a, T>
§
impl<'a, T>
UnwindSafe
for std::sync::mpsc::
TryIter
<'a, T>
§
impl<'a, T>
UnwindSafe
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
UnwindSafe
for
MutexGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T>
UnwindSafe
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
UnwindSafe
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
UnwindSafe
for
RwLockWriteGuard
<'a, T>
where
    T: ?
Sized
,
§
impl<'a, T, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
Entry
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for
DrainSorted
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for
PeekMut
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
OccupiedEntry
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
VacantEntry
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for std::collections::linked_list::
CursorMut
<'a, T, A>
§
impl<'a, T, A>
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
impl<'a, T, F, A =
Global
> !
UnwindSafe
for std::collections::btree_set::
ExtractIf
<'a, T, F, A>
§
impl<'a, T, F, A =
Global
> !
UnwindSafe
for std::collections::linked_list::
ExtractIf
<'a, T, F, A>
§
impl<'a, T, F, A =
Global
> !
UnwindSafe
for std::vec::
ExtractIf
<'a, T, F, A>
§
impl<'a, T, P> !
UnwindSafe
for
ChunkByMut
<'a, T, P>
§
impl<'a, T, P> !
UnwindSafe
for
RSplitMut
<'a, T, P>
§
impl<'a, T, P> !
UnwindSafe
for
RSplitNMut
<'a, T, P>
§
impl<'a, T, P> !
UnwindSafe
for
SplitInclusiveMut
<'a, T, P>
§
impl<'a, T, P> !
UnwindSafe
for
SplitMut
<'a, T, P>
§
impl<'a, T, P> !
UnwindSafe
for
SplitNMut
<'a, T, P>
§
impl<'a, T, P>
UnwindSafe
for
ChunkBy
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
UnwindSafe
for std::slice::
RSplit
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
UnwindSafe
for std::slice::
RSplitN
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
UnwindSafe
for std::slice::
Split
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
UnwindSafe
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, P>
UnwindSafe
for std::slice::
SplitN
<'a, T, P>
where
    P:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, S> !
UnwindSafe
for std::collections::hash_set::
Entry
<'a, T, S>
§
impl<'a, T, S> !
UnwindSafe
for std::collections::hash_set::
OccupiedEntry
<'a, T, S>
§
impl<'a, T, S> !
UnwindSafe
for std::collections::hash_set::
VacantEntry
<'a, T, S>
§
impl<'a, T, S>
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
UnwindSafe
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
impl<'a, T, const N:
usize
> !
UnwindSafe
for
ArrayChunksMut
<'a, T, N>
§
impl<'a, T, const N:
usize
>
UnwindSafe
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
UnwindSafe
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
UnwindSafe
for
CharArraySearcher
<'a, N>
§
impl<'b, T> !
UnwindSafe
for
Ref
<'b, T>
§
impl<'b, T> !
UnwindSafe
for
RefMut
<'b, T>
§
impl<'data> !
UnwindSafe
for
BorrowedBuf
<'data>
§
impl<'f> !
UnwindSafe
for
VaListImpl
<'f>
§
impl<'fd>
UnwindSafe
for
BorrowedFd
<'fd>
§
impl<'handle>
UnwindSafe
for
BorrowedHandle
<'handle>
§
impl<'scope, 'env> !
UnwindSafe
for
Scope
<'scope, 'env>
§
impl<'scope, T> !
UnwindSafe
for
ScopedJoinHandle
<'scope, T>
§
impl<'socket>
UnwindSafe
for
BorrowedSocket
<'socket>
§
impl<A>
UnwindSafe
for std::iter::
Repeat
<A>
where
    A:
UnwindSafe
,
§
impl<A>
UnwindSafe
for
RepeatN
<A>
where
    A:
UnwindSafe
,
§
impl<A>
UnwindSafe
for std::option::
IntoIter
<A>
where
    A:
UnwindSafe
,
§
impl<A>
UnwindSafe
for
IterRange
<A>
where
    A:
UnwindSafe
,
§
impl<A>
UnwindSafe
for
IterRangeFrom
<A>
where
    A:
UnwindSafe
,
§
impl<A>
UnwindSafe
for
IterRangeInclusive
<A>
where
    A:
UnwindSafe
,
§
impl<A, B>
UnwindSafe
for std::iter::
Chain
<A, B>
where
    A:
UnwindSafe
,
    B:
UnwindSafe
,
§
impl<A, B>
UnwindSafe
for
Zip
<A, B>
where
    A:
UnwindSafe
,
    B:
UnwindSafe
,
§
impl<B>
UnwindSafe
for std::io::
Lines
<B>
where
    B:
UnwindSafe
,
§
impl<B>
UnwindSafe
for std::io::
Split
<B>
where
    B:
UnwindSafe
,
§
impl<B, C>
UnwindSafe
for
ControlFlow
<B, C>
where
    C:
UnwindSafe
,
    B:
UnwindSafe
,
§
impl<Dyn> !
UnwindSafe
for
DynMetadata
<Dyn>
§
impl<E>
UnwindSafe
for
Report
<E>
where
    E:
UnwindSafe
,
§
impl<F>
UnwindSafe
for std::fmt::
FromFn
<F>
where
    F:
UnwindSafe
,
§
impl<F>
UnwindSafe
for
PollFn
<F>
where
    F:
UnwindSafe
,
§
impl<F>
UnwindSafe
for std::iter::
FromFn
<F>
where
    F:
UnwindSafe
,
§
impl<F>
UnwindSafe
for
OnceWith
<F>
where
    F:
UnwindSafe
,
§
impl<F>
UnwindSafe
for
RepeatWith
<F>
where
    F:
UnwindSafe
,
§
impl<G>
UnwindSafe
for
FromCoroutine
<G>
where
    G:
UnwindSafe
,
§
impl<H>
UnwindSafe
for
BuildHasherDefault
<H>
§
impl<I>
UnwindSafe
for
FromIter
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
DecodeUtf16
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Cloned
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Copied
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Cycle
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Enumerate
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
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
UnwindSafe
,
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Fuse
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
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
UnwindSafe
,
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Peekable
<I>
where
    I:
UnwindSafe
,
    <I as
Iterator
>::
Item
:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
Skip
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for
StepBy
<I>
where
    I:
UnwindSafe
,
§
impl<I>
UnwindSafe
for std::iter::
Take
<I>
where
    I:
UnwindSafe
,
§
impl<I, F>
UnwindSafe
for
FilterMap
<I, F>
where
    I:
UnwindSafe
,
    F:
UnwindSafe
,
§
impl<I, F>
UnwindSafe
for
Inspect
<I, F>
where
    I:
UnwindSafe
,
    F:
UnwindSafe
,
§
impl<I, F>
UnwindSafe
for
Map
<I, F>
where
    I:
UnwindSafe
,
    F:
UnwindSafe
,
§
impl<I, F, const N:
usize
>
UnwindSafe
for
MapWindows
<I, F, N>
where
    F:
UnwindSafe
,
    I:
UnwindSafe
,
    <I as
Iterator
>::
Item
:
UnwindSafe
,
§
impl<I, G>
UnwindSafe
for
IntersperseWith
<I, G>
where
    G:
UnwindSafe
,
    <I as
Iterator
>::
Item
:
UnwindSafe
,
    I:
UnwindSafe
,
§
impl<I, P>
UnwindSafe
for
Filter
<I, P>
where
    I:
UnwindSafe
,
    P:
UnwindSafe
,
§
impl<I, P>
UnwindSafe
for
MapWhile
<I, P>
where
    I:
UnwindSafe
,
    P:
UnwindSafe
,
§
impl<I, P>
UnwindSafe
for
SkipWhile
<I, P>
where
    I:
UnwindSafe
,
    P:
UnwindSafe
,
§
impl<I, P>
UnwindSafe
for
TakeWhile
<I, P>
where
    I:
UnwindSafe
,
    P:
UnwindSafe
,
§
impl<I, St, F>
UnwindSafe
for
Scan
<I, St, F>
where
    I:
UnwindSafe
,
    F:
UnwindSafe
,
    St:
UnwindSafe
,
§
impl<I, U, F>
UnwindSafe
for
FlatMap
<I, U, F>
where
    <U as
IntoIterator
>::
IntoIter
:
UnwindSafe
,
    I:
UnwindSafe
,
    F:
UnwindSafe
,
§
impl<I, const N:
usize
>
UnwindSafe
for std::iter::
ArrayChunks
<I, N>
where
    I:
UnwindSafe
,
    <I as
Iterator
>::
Item
:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::ops::
Range
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::ops::
RangeFrom
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::ops::
RangeInclusive
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for
RangeTo
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for
RangeToInclusive
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::range::
Range
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::range::
RangeFrom
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<Idx>
UnwindSafe
for std::range::
RangeInclusive
<Idx>
where
    Idx:
UnwindSafe
,
§
impl<K>
UnwindSafe
for std::collections::hash_set::
IntoIter
<K>
where
    K:
UnwindSafe
+
RefUnwindSafe
,
§
impl<K, V>
UnwindSafe
for std::collections::hash_map::
IntoIter
<K, V>
where
    K:
UnwindSafe
+
RefUnwindSafe
,
    V:
UnwindSafe
+
RefUnwindSafe
,
§
impl<K, V>
UnwindSafe
for std::collections::hash_map::
IntoKeys
<K, V>
where
    K:
UnwindSafe
+
RefUnwindSafe
,
    V:
UnwindSafe
+
RefUnwindSafe
,
§
impl<K, V>
UnwindSafe
for std::collections::hash_map::
IntoValues
<K, V>
where
    K:
UnwindSafe
+
RefUnwindSafe
,
    V:
UnwindSafe
+
RefUnwindSafe
,
§
impl<K, V, A>
UnwindSafe
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
UnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
UnwindSafe
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
UnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
UnwindSafe
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
UnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<Ptr>
UnwindSafe
for
Pin
<Ptr>
where
    Ptr:
UnwindSafe
,
§
impl<R>
UnwindSafe
for
BufReader
<R>
where
    R:
UnwindSafe
+ ?
Sized
,
§
impl<R>
UnwindSafe
for std::io::
Bytes
<R>
where
    R:
UnwindSafe
,
§
impl<Ret, T>
UnwindSafe
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<T> !
UnwindSafe
for
JoinHandle
<T>
§
impl<T>
UnwindSafe
for
Bound
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
Option
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
TryLockError
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
SendTimeoutError
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
TrySendError
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
Poll
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
[T]
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
(T₁, T₂, …, Tₙ)
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
ThinBox
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
Cell
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
OnceCell
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
RefCell
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
SyncUnsafeCell
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
UnsafeCell
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
Reverse
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
UnwindSafe
,
    T: ?
Sized
,
§
impl<T>
UnwindSafe
for
Pending
<T>
§
impl<T>
UnwindSafe
for
Ready
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for std::io::
Cursor
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for std::io::
Take
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for std::iter::
Empty
<T>
§
impl<T>
UnwindSafe
for std::iter::
Once
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
Rev
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
§
impl<T>
UnwindSafe
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
§
impl<T>
UnwindSafe
for
PhantomData
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
PhantomInvariant
<T>
where
    T: ?
Sized
,
§
impl<T>
UnwindSafe
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
ManuallyDrop
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
Saturating
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
Wrapping
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
Yeet
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for std::result::
IntoIter
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
AtomicPtr
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
UnwindSafe
for std::sync::mpmc::
IntoIter
<T>
§
impl<T>
UnwindSafe
for std::sync::mpsc::
IntoIter
<T>
§
impl<T>
UnwindSafe
for std::sync::mpsc::
Receiver
<T>
§
impl<T>
UnwindSafe
for
SendError
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for std::sync::mpsc::
Sender
<T>
§
impl<T>
UnwindSafe
for
SyncSender
<T>
§
impl<T>
UnwindSafe
for
Exclusive
<T>
where
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
PoisonError
<T>
where
    T:
UnwindSafe
,
§
impl<T>
UnwindSafe
for
LocalKey
<T>
§
impl<T>
UnwindSafe
for
MaybeUninit
<T>
where
    T:
UnwindSafe
,
§
impl<T, A =
Global
> !
UnwindSafe
for
UniqueRc
<T, A>
§
impl<T, A =
Global
> !
UnwindSafe
for std::rc::
Weak
<T, A>
§
impl<T, A>
UnwindSafe
for
Box
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
+ ?
Sized
,
§
impl<T, A>
UnwindSafe
for std::collections::binary_heap::
IntoIter
<T, A>
where
    T:
RefUnwindSafe
+
UnwindSafe
,
    A:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for
IntoIterSorted
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
UnwindSafe
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
+
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for
BTreeSet
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
UnwindSafe
for
BinaryHeap
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for
LinkedList
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
+
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for
VecDeque
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for std::sync::
Weak
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T, A>
UnwindSafe
for std::vec::
IntoIter
<T, A>
where
    T:
RefUnwindSafe
+
UnwindSafe
,
    A:
UnwindSafe
,
§
impl<T, A>
UnwindSafe
for
Vec
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, E>
UnwindSafe
for
Result
<T, E>
where
    T:
UnwindSafe
,
    E:
UnwindSafe
,
§
impl<T, F>
UnwindSafe
for
LazyCell
<T, F>
where
    F:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, F>
UnwindSafe
for
Successors
<T, F>
where
    F:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, S>
UnwindSafe
for
HashSet
<T, S>
where
    S:
UnwindSafe
,
    T:
UnwindSafe
,
§
impl<T, U>
UnwindSafe
for std::io::
Chain
<T, U>
where
    T:
UnwindSafe
,
    U:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for
[T; N]
where
    T:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for std::array::
IntoIter
<T, N>
where
    T:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for
Mask
<T, N>
where
    T:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for
Simd
<T, N>
where
    T:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for [
Option
<T>;
N
]
where
    T:
UnwindSafe
,
§
impl<T, const N:
usize
>
UnwindSafe
for [
MaybeUninit
<T>;
N
]
where
    T:
UnwindSafe
,
§
impl<W> !
UnwindSafe
for
IntoInnerError
<W>
§
impl<W>
UnwindSafe
for
BufWriter
<W>
where
    W:
UnwindSafe
+ ?
Sized
,
§
impl<W>
UnwindSafe
for
LineWriter
<W>
where
    W:
UnwindSafe
+ ?
Sized
,
§
impl<Y, R>
UnwindSafe
for
CoroutineState
<Y, R>
where
    Y:
UnwindSafe
,
    R:
UnwindSafe
,
§
impl<const N:
usize
>
UnwindSafe
for
LaneCount
<N>
§
impl<const N:
usize
>
UnwindSafe
for [
u8
;
N
]