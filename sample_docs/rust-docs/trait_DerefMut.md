DerefMut in std::ops - Rust
std
::
ops
Trait
DerefMut
Copy item path
1.0.0 (const:
unstable
)
·
Source
pub trait DerefMut:
Deref
{
    // Required method
    const fn
deref_mut
(&mut self) -> &mut Self::
Target
;
}
Expand description
Used for mutable dereferencing operations, like in
*v = 1;
.
In addition to being used for explicit dereferencing operations with the
(unary)
*
operator in mutable contexts,
DerefMut
is also used implicitly
by the compiler in many circumstances. This mechanism is called
“mutable deref coercion”
. In immutable contexts,
Deref
is used.
Warning:
Deref coercion is a powerful language feature which has
far-reaching implications for every type that implements
DerefMut
. The
compiler will silently insert calls to
DerefMut::deref_mut
. For this
reason, one should be careful about implementing
DerefMut
and only do so
when mutable deref coercion is desirable. See
the
Deref
docs
for advice on when this is typically desirable or undesirable.
Types that implement
DerefMut
or
Deref
are often called “smart
pointers” and the mechanism of deref coercion has been specifically designed
to facilitate the pointer-like behavior that name suggests. Often, the
purpose of a “smart pointer” type is to change the ownership semantics
of a contained value (for example,
Rc
or
Cow
) or the
storage semantics of a contained value (for example,
Box
).
§
Mutable deref coercion
If
T
implements
DerefMut<Target = U>
, and
v
is a value of type
T
,
then:
In mutable contexts,
*v
(where
T
is neither a reference nor a raw pointer)
is equivalent to
*DerefMut::deref_mut(&mut v)
.
Values of type
&mut T
are coerced to values of type
&mut U
T
implicitly implements all the (mutable) methods of the type
U
.
For more details, visit
the chapter in
The Rust Programming Language
as well as the reference sections on
the dereference operator
,
method resolution
and
type coercions
.
§
Fallibility
This trait’s method should never unexpectedly fail
. Deref coercion means
the compiler will often insert calls to
DerefMut::deref_mut
implicitly.
Failure during dereferencing can be extremely confusing when
DerefMut
is
invoked implicitly. In the majority of uses it should be infallible, though
it may be acceptable to panic if the type is misused through programmer
error, for example.
However, infallibility is not enforced and therefore not guaranteed.
As such,
unsafe
code should not rely on infallibility in general for
soundness.
§
Examples
A struct with a single field which is modifiable by dereferencing the
struct.
use
std::ops::{Deref, DerefMut};
struct
DerefMutExample<T> {
    value: T
}
impl
<T> Deref
for
DerefMutExample<T> {
type
Target = T;
fn
deref(
&
self
) ->
&
Self
::Target {
&
self
.value
    }
}
impl
<T> DerefMut
for
DerefMutExample<T> {
fn
deref_mut(
&mut
self
) ->
&mut
Self
::Target {
&mut
self
.value
    }
}
let
mut
x = DerefMutExample { value:
'a'
};
*
x =
'b'
;
assert_eq!
(
'b'
, x.value);
Required Methods
§
1.0.0
·
Source
const fn
deref_mut
(&mut self) -> &mut Self::
Target
Mutably dereferences the value.
Implementors
§
Source
§
impl
DerefMut
for
ByteStr
Source
§
impl
DerefMut
for
ByteString
1.44.0
·
Source
§
impl
DerefMut
for
OsString
1.68.0
·
Source
§
impl
DerefMut
for
PathBuf
1.3.0
·
Source
§
impl
DerefMut
for
String
1.36.0
·
Source
§
impl<'a>
DerefMut
for
IoSliceMut
<'a>
Source
§
impl<'a, 'f>
DerefMut
for
VaList
<'a, 'f>
where
    'f: 'a,
1.33.0
·
Source
§
impl<Ptr>
DerefMut
for
Pin
<Ptr>
where
    Ptr:
DerefMut
,
    <Ptr as
Deref
>::
Target
:
Unpin
,
1.0.0
·
Source
§
impl<T> !
DerefMut
for
&T
where
    T: ?
Sized
,
1.0.0 (const:
unstable
)
·
Source
§
impl<T>
DerefMut
for
&mut T
where
    T: ?
Sized
,
Source
§
impl<T>
DerefMut
for
ThinBox
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
DerefMut
for
RefMut
<'_, T>
where
    T: ?
Sized
,
1.20.0
·
Source
§
impl<T>
DerefMut
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
1.9.0
·
Source
§
impl<T>
DerefMut
for
AssertUnwindSafe
<T>
1.0.0
·
Source
§
impl<T, A>
DerefMut
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
1.12.0
·
Source
§
impl<T, A>
DerefMut
for
PeekMut
<'_, T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
§
impl<T, A>
DerefMut
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
1.0.0
·
Source
§
impl<T, A>
DerefMut
for
Vec
<T, A>
where
    A:
Allocator
,
Source
§
impl<T: ?
Sized
>
DerefMut
for
MappedMutexGuard
<'_, T>
Source
§
impl<T: ?
Sized
>
DerefMut
for
MappedRwLockWriteGuard
<'_, T>
1.0.0
·
Source
§
impl<T: ?
Sized
>
DerefMut
for
MutexGuard
<'_, T>
1.0.0
·
Source
§
impl<T: ?
Sized
>
DerefMut
for
RwLockWriteGuard
<'_, T>