PinCoerceUnsized in std::pin - Rust
std
::
pin
Trait
PinCoerceUnsized
Copy item path
Source
pub unsafe trait PinCoerceUnsized { }
🔬
This is a nightly-only experimental API. (
pin_coerce_unsized_trait
#123430
)
Expand description
Trait that indicates that this is a pointer or a wrapper for one, where
unsizing can be performed on the pointee when it is pinned.
§
Safety
If this type implements
Deref
, then the concrete type returned by
deref
and
deref_mut
must not change without a modification. The following
operations are not considered modifications:
Moving the pointer.
Performing unsizing coercions on the pointer.
Performing dynamic dispatch with the pointer.
Calling
deref
or
deref_mut
on the pointer.
The concrete type of a trait object is the type that the vtable corresponds
to. The concrete type of a slice is an array of the same element type and
the length specified in the metadata. The concrete type of a sized type
is the type itself.
Implementors
§
1.33.0
·
Source
§
impl<'a, T>
PinCoerceUnsized
for
&'a T
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<'a, T>
PinCoerceUnsized
for
&'a mut T
where
    T: ?
Sized
,
Source
§
impl<'b, T>
PinCoerceUnsized
for
Ref
<'b, T>
where
    T: ?
Sized
,
Source
§
impl<'b, T>
PinCoerceUnsized
for
RefMut
<'b, T>
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
*const T
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
*mut T
where
    T: ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
Cell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
RefCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
SyncUnsafeCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PinCoerceUnsized
for
UnsafeCell
<T>
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
NonNull
<T>
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
Pin
<T>
where
    T:
PinCoerceUnsized
,
Source
§
impl<T, A>
PinCoerceUnsized
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
PinCoerceUnsized
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
PinCoerceUnsized
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
PinCoerceUnsized
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
PinCoerceUnsized
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
PinCoerceUnsized
for std::sync::
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,