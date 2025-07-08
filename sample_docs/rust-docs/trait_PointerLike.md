PointerLike in std::marker - Rust
std
::
marker
Trait
PointerLike
Copy item path
Source
pub trait PointerLike { }
🔬
This is a nightly-only experimental API. (
pointer_like_trait
)
Expand description
A marker for pointer-like types.
This trait can only be implemented for types that are certain to have
the same size and alignment as a
usize
or
*const ()
.
To ensure this, there are special requirements on implementations
of
PointerLike
(other than the already-provided implementations
for built-in types):
The type must have
#[repr(transparent)]
.
The type’s sole non-zero-sized field must itself implement
PointerLike
.
Implementors
§
Source
§
impl
PointerLike
for
isize
Source
§
impl
PointerLike
for
usize
Source
§
impl<T>
PointerLike
for
*const T
Source
§
impl<T>
PointerLike
for
*mut T
Source
§
impl<T>
PointerLike
for
&T
Source
§
impl<T>
PointerLike
for
&mut T
Source
§
impl<T>
PointerLike
for
Box
<T>
Source
§
impl<T>
PointerLike
for
Cell
<T>
where
    T:
PointerLike
,
Source
§
impl<T>
PointerLike
for
SyncUnsafeCell
<T>
where
    T:
PointerLike
,
Source
§
impl<T>
PointerLike
for
UnsafeCell
<T>
where
    T:
PointerLike
,
Source
§
impl<T>
PointerLike
for
Pin
<T>
where
    T:
PointerLike
,
Source
§
impl<T>
PointerLike
for
NonNull
<T>