DerefPure in std::ops - Rust
std
::
ops
Trait
DerefPure
Copy item path
Source
pub unsafe trait DerefPure { }
🔬
This is a nightly-only experimental API. (
deref_pure_trait
#87121
)
Expand description
Perma-unstable marker trait. Indicates that the type has a well-behaved
Deref
(and, if applicable,
DerefMut
) implementation. This is relied on for soundness
of deref patterns.
FIXME(deref_patterns): The precise semantics are undecided; the rough idea is that
successive calls to
deref
/
deref_mut
without intermediate mutation should be
idempotent, in the sense that they return the same value as far as pattern-matching
is concerned. Calls to
deref
/
deref_mut
must leave the pointer itself likewise
unchanged.
Implementors
§
Source
§
impl
DerefPure
for
Cow
<'_,
str
>
Source
§
impl
DerefPure
for
ByteStr
Source
§
impl
DerefPure
for
ByteString
Source
§
impl
DerefPure
for
String
Source
§
impl<Ptr>
DerefPure
for
Pin
<Ptr>
where
    Ptr:
DerefPure
,
Source
§
impl<T>
DerefPure
for
Cow
<'_,
[T]
>
where
    T:
Clone
,
Source
§
impl<T>
DerefPure
for
Cow
<'_, T>
where
    T:
Clone
,
Source
§
impl<T>
DerefPure
for
&T
where
    T: ?
Sized
,
Source
§
impl<T>
DerefPure
for
&mut T
where
    T: ?
Sized
,
Source
§
impl<T>
DerefPure
for
Ref
<'_, T>
where
    T: ?
Sized
,
Source
§
impl<T>
DerefPure
for
RefMut
<'_, T>
where
    T: ?
Sized
,
Source
§
impl<T>
DerefPure
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
Source
§
impl<T, A>
DerefPure
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
Source
§
impl<T, A>
DerefPure
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
DerefPure
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
Source
§
impl<T, A>
DerefPure
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
Source
§
impl<T, A>
DerefPure
for
Vec
<T, A>
where
    A:
Allocator
,