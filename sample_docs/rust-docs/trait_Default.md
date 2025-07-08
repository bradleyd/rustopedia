Default in std::default - Rust
std
::
default
Trait
Default
Copy item path
1.0.0
·
Source
pub trait Default:
Sized
{
    // Required method
    fn
default
() -> Self;
}
Expand description
A trait for giving a type a useful default value.
Sometimes, you want to fall back to some kind of default value, and
don’t particularly care what it is. This comes up often with
struct
s
that define a set of options:
struct
SomeOptions {
    foo: i32,
    bar: f32,
}
How can we define some default values? You can use
Default
:
#[derive(Default)]
struct
SomeOptions {
    foo: i32,
    bar: f32,
}
fn
main() {
let
options: SomeOptions = Default::default();
}
Now, you get all of the default values. Rust implements
Default
for various primitives types.
If you want to override a particular option, but still retain the other defaults:
fn
main() {
let
options = SomeOptions { foo:
42
, ..Default::default() };
}
§
Derivable
This trait can be used with
#[derive]
if all of the type’s fields implement
Default
. When
derive
d, it will use the default value for each field’s type.
§
enum
s
When using
#[derive(Default)]
on an
enum
, you need to choose which unit variant will be
default. You do this by placing the
#[default]
attribute on the variant.
#[derive(Default)]
enum
Kind {
#[default]
A,
    B,
    C,
}
You cannot use the
#[default]
attribute on non-unit or non-exhaustive variants.
The
#[default]
attribute was stabilized in Rust 1.62.0.
§
How can I implement
Default
?
Provide an implementation for the
default()
method that returns the value of
your type that should be the default:
enum
Kind {
    A,
    B,
    C,
}
impl
Default
for
Kind {
fn
default() ->
Self
{ Kind::A }
}
§
Examples
#[derive(Default)]
struct
SomeOptions {
    foo: i32,
    bar: f32,
}
Required Methods
§
1.0.0
·
Source
fn
default
() -> Self
Returns the “default value” for a type.
Default values are often some kind of initial value, identity value, or anything else that
may make sense as a default.
§
Examples
Using built-in default values:
let
i: i8 = Default::default();
let
(x, y): (
Option
<String>, f64) = Default::default();
let
(a, b, (c, d)): (i32, u32, (bool, bool)) = Default::default();
Making your own:
enum
Kind {
    A,
    B,
    C,
}
impl
Default
for
Kind {
fn
default() ->
Self
{ Kind::A }
}
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.0.0
·
Source
§
impl
Default
for &
str
1.10.0
·
Source
§
impl
Default
for &
CStr
1.9.0
·
Source
§
impl
Default
for &
OsStr
1.28.0
·
Source
§
impl
Default
for &mut
str
1.0.0
·
Source
§
impl
Default
for
AsciiChar
1.0.0
·
Source
§
impl
Default
for
bool
1.0.0
·
Source
§
impl
Default
for
char
1.0.0
·
Source
§
impl
Default
for
f16
1.0.0
·
Source
§
impl
Default
for
f32
1.0.0
·
Source
§
impl
Default
for
f64
1.0.0
·
Source
§
impl
Default
for
f128
1.0.0
·
Source
§
impl
Default
for
i8
1.0.0
·
Source
§
impl
Default
for
i16
1.0.0
·
Source
§
impl
Default
for
i32
1.0.0
·
Source
§
impl
Default
for
i64
1.0.0
·
Source
§
impl
Default
for
i128
1.0.0
·
Source
§
impl
Default
for
isize
1.0.0
·
Source
§
impl
Default
for
u8
1.0.0
·
Source
§
impl
Default
for
u16
1.0.0
·
Source
§
impl
Default
for
u32
1.0.0
·
Source
§
impl
Default
for
u64
1.0.0
·
Source
§
impl
Default
for
u128
1.0.0
·
Source
§
impl
Default
for
()
1.0.0
·
Source
§
impl
Default
for
usize
Source
§
impl
Default
for
Global
1.28.0
·
Source
§
impl
Default
for
System
1.17.0
·
Source
§
impl
Default
for
Box
<
str
>
1.17.0
·
Source
§
impl
Default
for
Box
<
CStr
>
1.17.0
·
Source
§
impl
Default
for
Box
<
OsStr
>
Source
§
impl
Default
for
ByteString
1.10.0
·
Source
§
impl
Default
for
CString
1.9.0
·
Source
§
impl
Default
for
OsString
1.0.0
·
Source
§
impl
Default
for
Error
Source
§
impl
Default
for
FormattingOptions
1.75.0
·
Source
§
impl
Default
for
FileTimes
1.13.0
·
Source
§
impl
Default
for
DefaultHasher
1.7.0
·
Source
§
impl
Default
for
RandomState
1.0.0
·
Source
§
impl
Default
for
SipHasher
1.0.0
·
Source
§
impl
Default
for std::io::
Empty
1.0.0
·
Source
§
impl
Default
for
Sink
1.33.0
·
Source
§
impl
Default
for
PhantomPinned
1.0.0
·
Source
§
impl
Default
for
RangeFull
1.17.0
·
Source
§
impl
Default
for
PathBuf
1.75.0
·
Source
§
impl
Default
for
ExitCode
The default value is
ExitCode::SUCCESS
1.73.0
·
Source
§
impl
Default
for
ExitStatus
The default value is one which indicates successful completion.
Source
§
impl
Default
for
Alignment
Returns
Alignment::MIN
, which is valid for any type.
Source
§
impl
Default
for
DefaultRandomSource
1.80.0
·
Source
§
impl
Default
for
Rc
<
str
>
1.80.0
·
Source
§
impl
Default
for
Rc
<
CStr
>
1.0.0
·
Source
§
impl
Default
for
String
1.0.0
·
Source
§
impl
Default
for
AtomicBool
1.34.0
·
Source
§
impl
Default
for
AtomicI8
1.34.0
·
Source
§
impl
Default
for
AtomicI16
1.34.0
·
Source
§
impl
Default
for
AtomicI32
1.34.0
·
Source
§
impl
Default
for
AtomicI64
Source
§
impl
Default
for
AtomicI128
1.0.0
·
Source
§
impl
Default
for
AtomicIsize
1.34.0
·
Source
§
impl
Default
for
AtomicU8
1.34.0
·
Source
§
impl
Default
for
AtomicU16
1.34.0
·
Source
§
impl
Default
for
AtomicU32
1.34.0
·
Source
§
impl
Default
for
AtomicU64
Source
§
impl
Default
for
AtomicU128
1.0.0
·
Source
§
impl
Default
for
AtomicUsize
1.80.0
·
Source
§
impl
Default
for
Arc
<
str
>
1.80.0
·
Source
§
impl
Default
for
Arc
<
CStr
>
1.10.0
·
Source
§
impl
Default
for
Condvar
1.3.0
·
Source
§
impl
Default
for
Duration
Source
§
impl<'a>
Default
for &'a
ByteStr
Source
§
impl<'a>
Default
for &'a mut
ByteStr
Source
§
impl<'a>
Default
for
PhantomContravariantLifetime
<'a>
Source
§
impl<'a>
Default
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
Default
for
PhantomInvariantLifetime
<'a>
1.70.0
·
Source
§
impl<'a, K, V>
Default
for std::collections::btree_map::
Iter
<'a, K, V>
where
    K: 'a,
    V: 'a,
1.70.0
·
Source
§
impl<'a, K, V>
Default
for std::collections::btree_map::
IterMut
<'a, K, V>
where
    K: 'a,
    V: 'a,
1.70.0
·
Source
§
impl<A, B>
Default
for
Chain
<A, B>
where
    A:
Default
,
    B:
Default
,
1.11.0
·
Source
§
impl<B>
Default
for
Cow
<'_, B>
where
    B:
ToOwned
+ ?
Sized
,
    <B as
ToOwned
>::
Owned
:
Default
,
1.7.0
·
Source
§
impl<H>
Default
for
BuildHasherDefault
<H>
1.70.0
·
Source
§
impl<I>
Default
for
Cloned
<I>
where
    I:
Default
,
1.70.0
·
Source
§
impl<I>
Default
for
Copied
<I>
where
    I:
Default
,
1.70.0
·
Source
§
impl<I>
Default
for
Enumerate
<I>
where
    I:
Default
,
1.70.0
·
Source
§
impl<I>
Default
for
Flatten
<I>
where
    I:
Default
+
Iterator
,
    <I as
Iterator
>::
Item
:
IntoIterator
,
1.70.0
·
Source
§
impl<I>
Default
for
Fuse
<I>
where
    I:
Default
,
1.70.0
·
Source
§
impl<I>
Default
for
Rev
<I>
where
    I:
Default
,
1.0.0
·
Source
§
impl<Idx>
Default
for std::ops::
Range
<Idx>
where
    Idx:
Default
,
Source
§
impl<Idx>
Default
for std::range::
Range
<Idx>
where
    Idx:
Default
,
1.83.0
·
Source
§
impl<K>
Default
for std::collections::hash_set::
IntoIter
<K>
1.83.0
·
Source
§
impl<K>
Default
for std::collections::hash_set::
Iter
<'_, K>
1.70.0
·
Source
§
impl<K, V>
Default
for std::collections::btree_map::
Keys
<'_, K, V>
1.70.0
·
Source
§
impl<K, V>
Default
for std::collections::btree_map::
Range
<'_, K, V>
1.82.0
·
Source
§
impl<K, V>
Default
for
RangeMut
<'_, K, V>
1.70.0
·
Source
§
impl<K, V>
Default
for std::collections::btree_map::
Values
<'_, K, V>
1.82.0
·
Source
§
impl<K, V>
Default
for std::collections::btree_map::
ValuesMut
<'_, K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
IntoIter
<K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
IntoKeys
<K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
IntoValues
<K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
Iter
<'_, K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
IterMut
<'_, K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
Keys
<'_, K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
Values
<'_, K, V>
1.83.0
·
Source
§
impl<K, V>
Default
for std::collections::hash_map::
ValuesMut
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
Default
for
BTreeMap
<K, V>
1.70.0
·
Source
§
impl<K, V, A>
Default
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Allocator
+
Default
+
Clone
,
1.70.0
·
Source
§
impl<K, V, A>
Default
for std::collections::btree_map::
IntoKeys
<K, V, A>
where
    A:
Allocator
+
Default
+
Clone
,
1.70.0
·
Source
§
impl<K, V, A>
Default
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Allocator
+
Default
+
Clone
,
1.0.0
·
Source
§
impl<K, V, S>
Default
for
HashMap
<K, V, S>
where
    S:
Default
,
1.0.0
·
Source
§
impl<T>
Default
for &
[T]
1.5.0
·
Source
§
impl<T>
Default
for &mut
[T]
1.0.0
·
Source
§
impl<T>
Default
for
Option
<T>
1.4.0
·
Source
§
impl<T>
Default
for
[T; 0]
1.4.0
·
Source
§
impl<T>
Default
for
[T; 1]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 2]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 3]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 4]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 5]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 6]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 7]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 8]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 9]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 10]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 11]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 12]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 13]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 14]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 15]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 16]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 17]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 18]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 19]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 20]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 21]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 22]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 23]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 24]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 25]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 26]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 27]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 28]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 29]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 30]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 31]
where
    T:
Default
,
1.4.0
·
Source
§
impl<T>
Default
for
[T; 32]
where
    T:
Default
,
1.0.0
·
Source
§
impl<T>
Default
for
(T₁, T₂, …, Tₙ)
where
    T:
Default
,
This trait is implemented for tuples up to twelve items long.
1.0.0
·
Source
§
impl<T>
Default
for
Box
<
[T]
>
1.0.0
·
Source
§
impl<T>
Default
for
Box
<T>
where
    T:
Default
,
1.0.0
·
Source
§
impl<T>
Default
for
Cell
<T>
where
    T:
Default
,
1.80.0
·
Source
§
impl<T>
Default
for
LazyCell
<T>
where
    T:
Default
,
1.70.0
·
Source
§
impl<T>
Default
for
OnceCell
<T>
1.0.0
·
Source
§
impl<T>
Default
for
RefCell
<T>
where
    T:
Default
,
Source
§
impl<T>
Default
for
SyncUnsafeCell
<T>
where
    T:
Default
,
1.10.0
·
Source
§
impl<T>
Default
for
UnsafeCell
<T>
where
    T:
Default
,
1.19.0
·
Source
§
impl<T>
Default
for
Reverse
<T>
where
    T:
Default
,
1.70.0
·
Source
§
impl<T>
Default
for std::collections::binary_heap::
IntoIter
<T>
1.82.0
·
Source
§
impl<T>
Default
for std::collections::binary_heap::
Iter
<'_, T>
1.70.0
·
Source
§
impl<T>
Default
for std::collections::btree_set::
Iter
<'_, T>
1.70.0
·
Source
§
impl<T>
Default
for std::collections::btree_set::
Range
<'_, T>
1.70.0
·
Source
§
impl<T>
Default
for std::collections::linked_list::
IntoIter
<T>
1.70.0
·
Source
§
impl<T>
Default
for std::collections::linked_list::
Iter
<'_, T>
1.70.0
·
Source
§
impl<T>
Default
for std::collections::linked_list::
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
Default
for
BTreeSet
<T>
1.0.0
·
Source
§
impl<T>
Default
for
BinaryHeap
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
Default
for
LinkedList
<T>
1.0.0
·
Source
§
impl<T>
Default
for
VecDeque
<T>
1.82.0
·
Source
§
impl<T>
Default
for std::collections::vec_deque::
Iter
<'_, T>
1.82.0
·
Source
§
impl<T>
Default
for std::collections::vec_deque::
IterMut
<'_, T>
1.2.0
·
Source
§
impl<T>
Default
for std::iter::
Empty
<T>
Source
§
impl<T>
Default
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
Default
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
Default
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
Default
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
Default
for
ManuallyDrop
<T>
where
    T:
Default
+ ?
Sized
,
1.74.0
·
Source
§
impl<T>
Default
for
Saturating
<T>
where
    T:
Default
,
1.0.0
·
Source
§
impl<T>
Default
for
Wrapping
<T>
where
    T:
Default
,
1.62.0
·
Source
§
impl<T>
Default
for
AssertUnwindSafe
<T>
where
    T:
Default
,
1.80.0
·
Source
§
impl<T>
Default
for
Rc
<
[T]
>
1.0.0
·
Source
§
impl<T>
Default
for
Rc
<T>
where
    T:
Default
,
1.10.0
·
Source
§
impl<T>
Default
for std::rc::
Weak
<T>
1.70.0
·
Source
§
impl<T>
Default
for std::slice::
Iter
<'_, T>
1.70.0
·
Source
§
impl<T>
Default
for std::slice::
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
Default
for
AtomicPtr
<T>
1.80.0
·
Source
§
impl<T>
Default
for
Arc
<
[T]
>
1.0.0
·
Source
§
impl<T>
Default
for
Arc
<T>
where
    T:
Default
,
Source
§
impl<T>
Default
for
Exclusive
<T>
where
    T:
Default
+ ?
Sized
,
1.70.0
·
Source
§
impl<T>
Default
for
OnceLock
<T>
1.10.0
·
Source
§
impl<T>
Default
for std::sync::
Weak
<T>
1.0.0
·
Source
§
impl<T>
Default
for
Vec
<T>
1.70.0
·
Source
§
impl<T, A>
Default
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Allocator
+
Default
+
Clone
,
1.70.0
·
Source
§
impl<T, A>
Default
for std::vec::
IntoIter
<T, A>
where
    A:
Allocator
+
Default
,
1.0.0
·
Source
§
impl<T, S>
Default
for
HashSet
<T, S>
where
    S:
Default
,
Source
§
impl<T, const N:
usize
>
Default
for
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
Source
§
impl<T, const N:
usize
>
Default
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
Default
,
1.0.0
·
Source
§
impl<T:
Default
>
Default
for
Cursor
<T>
1.80.0
·
Source
§
impl<T:
Default
>
Default
for
LazyLock
<T>
Source
§
impl<T:
Default
>
Default
for
ReentrantLock
<T>
1.10.0
·
Source
§
impl<T:
Default
>
Default
for
RwLock
<T>
1.10.0
·
Source
§
impl<T: ?
Sized
+
Default
>
Default
for
Mutex
<T>