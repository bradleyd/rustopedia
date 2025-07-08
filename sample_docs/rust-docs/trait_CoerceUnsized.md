CoerceUnsized in std::ops - Rust
std
::
ops
Trait
CoerceUnsized
Copy item path
Source
pub trait CoerceUnsized<T>
where
    T: ?
Sized
,
{ }
🔬
This is a nightly-only experimental API. (
coerce_unsized
#18598
)
Expand description
Trait that indicates that this is a pointer or a wrapper for one,
where unsizing can be performed on the pointee.
See the
DST coercion RFC
and
the nomicon entry on coercion
for more details.
For builtin pointer types, pointers to
T
will coerce to pointers to
U
if
T: Unsize<U>
by converting from a thin pointer to a fat pointer.
For custom types, the coercion here works by coercing
Foo<T>
to
Foo<U>
provided an impl of
CoerceUnsized<Foo<U>> for Foo<T>
exists.
Such an impl can only be written if
Foo<T>
has only a single non-phantomdata
field involving
T
. If the type of that field is
Bar<T>
, an implementation
of
CoerceUnsized<Bar<U>> for Bar<T>
must exist. The coercion will work by
coercing the
Bar<T>
field into
Bar<U>
and filling in the rest of the fields
from
Foo<T>
to create a
Foo<U>
. This will effectively drill down to a pointer
field and coerce that.
Generally, for smart pointers you will implement
CoerceUnsized<Ptr<U>> for Ptr<T> where T: Unsize<U>, U: ?Sized
, with an
optional
?Sized
bound on
T
itself. For wrapper types that directly embed
T
like
Cell<T>
and
RefCell<T>
, you
can directly implement
CoerceUnsized<Wrap<U>> for Wrap<T> where T: CoerceUnsized<U>
.
This will let coercions of types like
Cell<Box<T>>
work.
Unsize
is used to mark types which can be coerced to DSTs if behind
pointers. It is implemented automatically by the compiler.
Implementors
§
Source
§
impl<'a, 'b, T, U>
CoerceUnsized
<
&'a U
> for
&'b T
where
    'b: 'a,
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, 'b, T, U>
CoerceUnsized
<
&'a U
> for
&'b mut T
where
    'b: 'a,
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
*const U
> for
&'a T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
*const U
> for
&'a mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
*mut U
> for
&'a mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
&'a mut U
> for
&'a mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'b, T, U>
CoerceUnsized
<
Ref
<'b, U>> for
Ref
<'b, T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'b, T, U>
CoerceUnsized
<
RefMut
<'b, U>> for
RefMut
<'b, T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
1.33.0
·
Source
§
impl<Ptr, U>
CoerceUnsized
<
Pin
<U>> for
Pin
<Ptr>
where
    Ptr:
CoerceUnsized
<U> +
PinCoerceUnsized
,
    U:
PinCoerceUnsized
,
Source
§
impl<T, U>
CoerceUnsized
<
*const U
> for
*const T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
*const U
> for
*mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
*mut U
> for
*mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
Cell
<U>> for
Cell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
CoerceUnsized
<
RefCell
<U>> for
RefCell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
CoerceUnsized
<
SyncUnsafeCell
<U>> for
SyncUnsafeCell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
CoerceUnsized
<
UnsafeCell
<U>> for
UnsafeCell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
CoerceUnsized
<
NonNull
<U>> for
NonNull
<T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
Box
<U, A>> for
Box
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
Rc
<U, A>> for
Rc
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
UniqueRc
<U, A>> for
UniqueRc
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
Weak
<U, A>> for std::rc::
Weak
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
Arc
<U, A>> for
Arc
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U, A>
CoerceUnsized
<
Weak
<U, A>> for std::sync::
Weak
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,