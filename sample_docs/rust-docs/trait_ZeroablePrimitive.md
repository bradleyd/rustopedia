ZeroablePrimitive in std::num - Rust
std
::
num
Trait
ZeroablePrimitive
Copy item path
Source
pub unsafe trait ZeroablePrimitive:
Sized
+
Copy
+ Sealed { }
🔬
This is a nightly-only experimental API. (
nonzero_internals
)
Expand description
A marker trait for primitive types which can be zero.
This is an implementation detail for
NonZero
<T>
which may disappear or be replaced at any time.
§
Safety
Types implementing this trait must be primitives that are valid when zeroed.
The associated
Self::NonZeroInner
type must have the same size+align as
Self
,
but with a niche and bit validity making it so the following
transmutes
are sound:
Self::NonZeroInner
to
Option<Self::NonZeroInner>
Option<Self::NonZeroInner>
to
Self
(And, consequently,
Self::NonZeroInner
to
Self
.)
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
ZeroablePrimitive
for
i8
Source
§
impl
ZeroablePrimitive
for
i16
Source
§
impl
ZeroablePrimitive
for
i32
Source
§
impl
ZeroablePrimitive
for
i64
Source
§
impl
ZeroablePrimitive
for
i128
Source
§
impl
ZeroablePrimitive
for
isize
Source
§
impl
ZeroablePrimitive
for
u8
Source
§
impl
ZeroablePrimitive
for
u16
Source
§
impl
ZeroablePrimitive
for
u32
Source
§
impl
ZeroablePrimitive
for
u64
Source
§
impl
ZeroablePrimitive
for
u128
Source
§
impl
ZeroablePrimitive
for
usize