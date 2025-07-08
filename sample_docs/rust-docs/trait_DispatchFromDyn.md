DispatchFromDyn in std::ops - Rust
std
::
ops
Trait
DispatchFromDyn
Copy item path
Source
pub trait DispatchFromDyn<T> { }
🔬
This is a nightly-only experimental API. (
dispatch_from_dyn
)
Expand description
DispatchFromDyn
is used in the implementation of dyn-compatibility
1
checks (specifically
allowing arbitrary self types), to guarantee that a method’s receiver type can be dispatched on.
Note:
DispatchFromDyn
was briefly named
CoerceSized
(and had a slightly different
interpretation).
Imagine we have a trait object
t
with type
&dyn Tr
, where
Tr
is some trait with a method
m
defined as
fn m(&self);
. When calling
t.m()
, the receiver
t
is a wide pointer, but an
implementation of
m
will expect a narrow pointer as
&self
(a reference to the concrete
type). The compiler must generate an implicit conversion from the trait object/wide pointer to
the concrete reference/narrow pointer. Implementing
DispatchFromDyn
indicates that that
conversion is allowed and thus that the type implementing
DispatchFromDyn
is safe to use as
the self type in an dyn-compatible method. (in the above example, the compiler will require
DispatchFromDyn
is implemented for
&'a U
).
DispatchFromDyn
does not specify the conversion from wide pointer to narrow pointer; the
conversion is hard-wired into the compiler. For the conversion to work, the following
properties must hold (i.e., it is only safe to implement
DispatchFromDyn
for types which have
these properties, these are also checked by the compiler):
EITHER
Self
and
T
are either both references or both raw pointers; in either case, with
the same mutability.
OR, all of the following hold
Self
and
T
must have the same type constructor, and only vary in a single type parameter
formal (the
coerced type
, e.g.,
impl DispatchFromDyn<Rc<T>> for Rc<U>
is ok and the
single type parameter (instantiated with
T
or
U
) is the coerced type,
impl DispatchFromDyn<Arc<T>> for Rc<U>
is not ok).
The definition for
Self
must be a struct.
The definition for
Self
must not be
#[repr(packed)]
or
#[repr(C)]
.
Other than one-aligned, zero-sized fields, the definition for
Self
must have exactly one
field and that field’s type must be the coerced type. Furthermore,
Self
’s field type must
implement
DispatchFromDyn<F>
where
F
is the type of
T
’s field type.
An example implementation of the trait:
impl
<T:
?
Sized, U:
?
Sized> DispatchFromDyn<Rc<U>>
for
Rc<T>
where
T: Unsize<U>,
{}
Formerly known as
object safety
.
↩
Implementors
§
Source
§
impl<'a, T, U>
DispatchFromDyn
<
&'a U
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
DispatchFromDyn
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
1.33.0
·
Source
§
impl<Ptr, U>
DispatchFromDyn
<
Pin
<U>> for
Pin
<Ptr>
where
    Ptr:
DispatchFromDyn
<U> +
PinCoerceUnsized
,
    U:
PinCoerceUnsized
,
Source
§
impl<T, U>
DispatchFromDyn
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
DispatchFromDyn
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
DispatchFromDyn
<
Box
<U>> for
Box
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
impl<T, U>
DispatchFromDyn
<
Cell
<U>> for
Cell
<T>
where
    T:
DispatchFromDyn
<U>,
Source
§
impl<T, U>
DispatchFromDyn
<
SyncUnsafeCell
<U>> for
SyncUnsafeCell
<T>
where
    T:
DispatchFromDyn
<U>,
Source
§
impl<T, U>
DispatchFromDyn
<
UnsafeCell
<U>> for
UnsafeCell
<T>
where
    T:
DispatchFromDyn
<U>,
Source
§
impl<T, U>
DispatchFromDyn
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
impl<T, U>
DispatchFromDyn
<
Rc
<U>> for
Rc
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
impl<T, U>
DispatchFromDyn
<
UniqueRc
<U>> for
UniqueRc
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
impl<T, U>
DispatchFromDyn
<
Weak
<U>> for std::rc::
Weak
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
impl<T, U>
DispatchFromDyn
<
Arc
<U>> for
Arc
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
impl<T, U>
DispatchFromDyn
<
Weak
<U>> for std::sync::
Weak
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