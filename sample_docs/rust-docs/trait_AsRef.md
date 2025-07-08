AsRef in std::convert - Rust
std
::
convert
Trait
AsRef
Copy item path
1.0.0
·
Source
pub trait AsRef<T>
where
    T: ?
Sized
,
{
    // Required method
    fn
as_ref
(&self) ->
&T
;
}
Expand description
Used to do a cheap reference-to-reference conversion.
This trait is similar to
AsMut
which is used for converting between mutable references.
If you need to do a costly conversion it is better to implement
From
with type
&T
or write a custom function.
§
Relation to
Borrow
AsRef
has the same signature as
Borrow
, but
Borrow
is different in a few aspects:
Unlike
AsRef
,
Borrow
has a blanket impl for any
T
, and can be used to accept either
a reference or a value. (See also note on
AsRef
’s reflexibility below.)
Borrow
also requires that
Hash
,
Eq
and
Ord
for a borrowed value are
equivalent to those of the owned value. For this reason, if you want to
borrow only a single field of a struct you can implement
AsRef
, but not
Borrow
.
Note: This trait must not fail
. If the conversion can fail, use a
dedicated method which returns an
Option<T>
or a
Result<T, E>
.
§
Generic Implementations
AsRef
auto-dereferences if the inner type is a reference or a mutable reference
(e.g.:
foo.as_ref()
will work the same if
foo
has type
&mut Foo
or
&&mut Foo
).
Note that due to historic reasons, the above currently does not hold generally for all
dereferenceable types
, e.g.
foo.as_ref()
will
not
work the same as
Box::new(foo).as_ref()
. Instead, many smart pointers provide an
as_ref
implementation which
simply returns a reference to the
pointed-to value
(but do not perform a cheap
reference-to-reference conversion for that value). However,
AsRef::as_ref
should not be
used for the sole purpose of dereferencing; instead
‘
Deref
coercion’
can be used:
let
x = Box::new(
5i32
);
// Avoid this:
// let y: &i32 = x.as_ref();
// Better just write:
let
y:
&
i32 =
&
x;
Types which implement
Deref
should consider implementing
AsRef<T>
as follows:
impl
<T> AsRef<T>
for
SomeType
where
T:
?
Sized,
    <SomeType
as
Deref>::Target: AsRef<T>,
{
fn
as_ref(
&
self
) ->
&
T {
self
.deref().as_ref()
    }
}
§
Reflexivity
Ideally,
AsRef
would be reflexive, i.e. there would be an
impl<T: ?Sized> AsRef<T> for T
with
as_ref
simply returning its argument unchanged.
Such a blanket implementation is currently
not
provided due to technical restrictions of
Rust’s type system (it would be overlapping with another existing blanket implementation for
&T where T: AsRef<U>
which allows
AsRef
to auto-dereference, see “Generic Implementations”
above).
A trivial implementation of
AsRef<T> for T
must be added explicitly for a particular type
T
where needed or desired. Note, however, that not all types from
std
contain such an
implementation, and those cannot be added by external code due to orphan rules.
§
Examples
By using trait bounds we can accept arguments of different types as long as they can be
converted to the specified type
T
.
For example: By creating a generic function that takes an
AsRef<str>
we express that we
want to accept all references that can be converted to
&str
as an argument.
Since both
String
and
&str
implement
AsRef<str>
we can accept both as input argument.
fn
is_hello<T: AsRef<str>>(s: T) {
assert_eq!
(
"hello"
, s.as_ref());
}
let
s =
"hello"
;
is_hello(s);
let
s =
"hello"
.to_string();
is_hello(s);
Required Methods
§
1.0.0
·
Source
fn
as_ref
(&self) ->
&T
Converts this type into a shared reference of the (usually inferred) input type.
Implementors
§
1.0.0
·
Source
§
impl
AsRef
<
str
> for
str
1.0.0
·
Source
§
impl
AsRef
<
str
> for
String
Source
§
impl
AsRef
<
ByteStr
> for
str
Source
§
impl
AsRef
<
ByteStr
> for
ByteStr
Source
§
impl
AsRef
<
ByteStr
> for
ByteString
1.7.0
·
Source
§
impl
AsRef
<
CStr
> for
CStr
1.7.0
·
Source
§
impl
AsRef
<
CStr
> for
CString
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Component
<'_>
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
str
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
OsStr
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
OsString
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Components
<'_>
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for std::path::
Iter
<'_>
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
Path
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
PathBuf
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
String
1.8.0
·
Source
§
impl
AsRef
<
Path
> for
Cow
<'_,
OsStr
>
1.25.0
·
Source
§
impl
AsRef
<
Path
> for
Component
<'_>
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
str
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
OsStr
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
OsString
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
Components
<'_>
1.0.0
·
Source
§
impl
AsRef
<
Path
> for std::path::
Iter
<'_>
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
Path
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
PathBuf
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
String
Source
§
impl
AsRef
<
LocalWaker
> for
Waker
1.0.0
·
Source
§
impl
AsRef
<[
u8
]> for
str
Source
§
impl
AsRef
<[
u8
]> for
ByteStr
Source
§
impl
AsRef
<[
u8
]> for
ByteString
1.0.0
·
Source
§
impl
AsRef
<[
u8
]> for
String
1.55.0
·
Source
§
impl<'a>
AsRef
<
str
> for std::string::
Drain
<'a>
1.55.0
·
Source
§
impl<'a>
AsRef
<[
u8
]> for std::string::
Drain
<'a>
1.46.0
·
Source
§
impl<'a, T, A>
AsRef
<
[T]
> for std::vec::
Drain
<'a, T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T>
AsRef
<
[T]
> for
[T]
1.13.0
·
Source
§
impl<T>
AsRef
<
[T]
> for std::slice::
Iter
<'_, T>
1.53.0
·
Source
§
impl<T>
AsRef
<
[T]
> for
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
AsRef
<T> for
Cow
<'_, T>
where
    T:
ToOwned
+ ?
Sized
,
1.46.0
·
Source
§
impl<T, A>
AsRef
<
[T]
> for
IntoIter
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
AsRef
<
[T]
> for
Vec
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
AsRef
<
Vec
<T, A>> for
Vec
<T, A>
where
    A:
Allocator
,
1.5.0
·
Source
§
impl<T, A>
AsRef
<T> for
Box
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.5.0
·
Source
§
impl<T, A>
AsRef
<T> for
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
AsRef
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.5.0
·
Source
§
impl<T, A>
AsRef
<T> for
Arc
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
impl<T, U>
AsRef
<U> for
&T
where
    T:
AsRef
<U> + ?
Sized
,
    U: ?
Sized
,
1.0.0
·
Source
§
impl<T, U>
AsRef
<U> for
&mut T
where
    T:
AsRef
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, const N:
usize
>
AsRef
<
[T; N]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
1.0.0
·
Source
§
impl<T, const N:
usize
>
AsRef
<
[T]
> for
[T; N]
Source
§
impl<T, const N:
usize
>
AsRef
<
[T]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,