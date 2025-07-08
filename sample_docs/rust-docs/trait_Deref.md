Deref in std::ops - Rust
std
::
ops
Trait
Deref
Copy item path
1.0.0 (const:
unstable
)
·
Source
pub trait Deref {
    type
Target
: ?
Sized
;

    // Required method
    const fn
deref
(&self) -> &Self::
Target
;
}
Expand description
Used for immutable dereferencing operations, like
*v
.
In addition to being used for explicit dereferencing operations with the
(unary)
*
operator in immutable contexts,
Deref
is also used implicitly
by the compiler in many circumstances. This mechanism is called
“
Deref
coercion”
. In mutable contexts,
DerefMut
is used and
mutable deref coercion similarly occurs.
Warning:
Deref coercion is a powerful language feature which has
far-reaching implications for every type that implements
Deref
. The
compiler will silently insert calls to
Deref::deref
. For this reason, one
should be careful about implementing
Deref
and only do so when deref
coercion is desirable. See
below
for advice on when this is
typically desirable or undesirable.
Types that implement
Deref
or
DerefMut
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
Deref coercion
If
T
implements
Deref<Target = U>
, and
v
is a value of type
T
, then:
In immutable contexts,
*v
(where
T
is neither a reference nor a raw
pointer) is equivalent to
*Deref::deref(&v)
.
Values of type
&T
are coerced to values of type
&U
T
implicitly implements all the methods of the type
U
which take the
&self
receiver.
For more details, visit
the chapter in
The Rust Programming Language
as well as the reference sections on
the dereference operator
,
method resolution
, and
type coercions
.
§
When to implement
Deref
or
DerefMut
The same advice applies to both deref traits. In general, deref traits
should
be implemented if:
a value of the type transparently behaves like a value of the target
type;
the implementation of the deref function is cheap; and
users of the type will not be surprised by any deref coercion behavior.
In general, deref traits
should not
be implemented if:
the deref implementations could fail unexpectedly; or
the type has methods that are likely to collide with methods on the
target type; or
committing to deref coercion as part of the public API is not desirable.
Note that there’s a large difference between implementing deref traits
generically over many target types, and doing so only for specific target
types.
Generic implementations, such as for
Box<T>
(which is generic over
every type and dereferences to
T
) should be careful to provide few or no
methods, since the target type is unknown and therefore every method could
collide with one on the target type, causing confusion for users.
impl<T> Box<T>
has no methods (though several associated functions),
partly for this reason.
Specific implementations, such as for
String
(whose
Deref
implementation has
Target = str
) can have many methods, since avoiding
collision is much easier.
String
and
str
both have many methods, and
String
additionally behaves as if it has every method of
str
because of
deref coercion. The implementing type may also be generic while the
implementation is still specific in this sense; for example,
Vec<T>
dereferences to
[T]
, so methods of
T
are not applicable.
Consider also that deref coercion means that deref traits are a much larger
part of a type’s public API than any other trait as it is implicitly called
by the compiler. Therefore, it is advisable to consider whether this is
something you are comfortable supporting as a public API.
The
AsRef
and
Borrow
traits have very similar
signatures to
Deref
. It may be desirable to implement either or both of
these, whether in addition to or rather than deref traits. See their
documentation for details.
§
Fallibility
This trait’s method should never unexpectedly fail
. Deref coercion means
the compiler will often insert calls to
Deref::deref
implicitly. Failure
during dereferencing can be extremely confusing when
Deref
is invoked
implicitly. In the majority of uses it should be infallible, though it may
be acceptable to panic if the type is misused through programmer error, for
example.
However, infallibility is not enforced and therefore not guaranteed.
As such,
unsafe
code should not rely on infallibility in general for
soundness.
§
Examples
A struct with a single field which is accessible by dereferencing the
struct.
use
std::ops::Deref;
struct
DerefExample<T> {
    value: T
}
impl
<T> Deref
for
DerefExample<T> {
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
let
x = DerefExample { value:
'a'
};
assert_eq!
(
'a'
,
*
x);
Required Associated Types
§
1.0.0
·
Source
type
Target
: ?
Sized
The resulting type after dereferencing.
Required Methods
§
1.0.0
·
Source
const fn
deref
(&self) -> &Self::
Target
Dereferences the value.
Implementors
§
Source
§
impl
Deref
for
ByteStr
Source
§
type
Target
= [
u8
]
Source
§
impl
Deref
for
ByteString
Source
§
type
Target
=
Vec
<
u8
>
1.0.0
·
Source
§
impl
Deref
for
CString
Source
§
type
Target
=
CStr
1.0.0
·
Source
§
impl
Deref
for
OsString
Source
§
type
Target
=
OsStr
1.0.0
·
Source
§
impl
Deref
for
PathBuf
Source
§
type
Target
=
Path
1.0.0
·
Source
§
impl
Deref
for
String
Source
§
type
Target
=
str
1.36.0
·
Source
§
impl<'a>
Deref
for
IoSlice
<'a>
Source
§
type
Target
= [
u8
]
1.36.0
·
Source
§
impl<'a>
Deref
for
IoSliceMut
<'a>
Source
§
type
Target
= [
u8
]
Source
§
impl<'a, 'f>
Deref
for
VaList
<'a, 'f>
where
    'f: 'a,
Source
§
type
Target
=
VaListImpl
<'f>
1.0.0
·
Source
§
impl<B>
Deref
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
Borrow
<B>,
Source
§
type
Target
= B
1.33.0
·
Source
§
impl<Ptr>
Deref
for
Pin
<Ptr>
where
    Ptr:
Deref
,
Source
§
type
Target
= <Ptr as
Deref
>::
Target
1.0.0 (const:
unstable
)
·
Source
§
impl<T>
Deref
for
&T
where
    T: ?
Sized
,
Source
§
type
Target
= T
1.0.0 (const:
unstable
)
·
Source
§
impl<T>
Deref
for
&mut T
where
    T: ?
Sized
,
Source
§
type
Target
= T
Source
§
impl<T>
Deref
for
ThinBox
<T>
where
    T: ?
Sized
,
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T>
Deref
for
Ref
<'_, T>
where
    T: ?
Sized
,
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T>
Deref
for
RefMut
<'_, T>
where
    T: ?
Sized
,
Source
§
type
Target
= T
1.20.0
·
Source
§
impl<T>
Deref
for
ManuallyDrop
<T>
where
    T: ?
Sized
,
Source
§
type
Target
= T
1.9.0
·
Source
§
impl<T>
Deref
for
AssertUnwindSafe
<T>
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T, A>
Deref
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
type
Target
= T
1.12.0
·
Source
§
impl<T, A>
Deref
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
type
Target
= T
1.0.0
·
Source
§
impl<T, A>
Deref
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
type
Target
= T
Source
§
impl<T, A>
Deref
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
type
Target
= T
1.0.0
·
Source
§
impl<T, A>
Deref
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
type
Target
= T
1.0.0
·
Source
§
impl<T, A>
Deref
for
Vec
<T, A>
where
    A:
Allocator
,
Source
§
type
Target
=
[T]
1.80.0
·
Source
§
impl<T, F>
Deref
for
LazyCell
<T, F>
where
    F:
FnOnce
() -> T,
Source
§
type
Target
= T
1.80.0
·
Source
§
impl<T, F:
FnOnce
() -> T>
Deref
for
LazyLock
<T, F>
Source
§
type
Target
= T
Source
§
impl<T: ?
Sized
>
Deref
for
MappedMutexGuard
<'_, T>
Source
§
type
Target
= T
Source
§
impl<T: ?
Sized
>
Deref
for
MappedRwLockReadGuard
<'_, T>
Source
§
type
Target
= T
Source
§
impl<T: ?
Sized
>
Deref
for
MappedRwLockWriteGuard
<'_, T>
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T: ?
Sized
>
Deref
for
MutexGuard
<'_, T>
Source
§
type
Target
= T
Source
§
impl<T: ?
Sized
>
Deref
for
ReentrantLockGuard
<'_, T>
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T: ?
Sized
>
Deref
for
RwLockReadGuard
<'_, T>
Source
§
type
Target
= T
1.0.0
·
Source
§
impl<T: ?
Sized
>
Deref
for
RwLockWriteGuard
<'_, T>
Source
§
type
Target
= T