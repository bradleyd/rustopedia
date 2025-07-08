AsMut in std::convert - Rust
std
::
convert
Trait
AsMut
Copy item path
1.0.0
·
Source
pub trait AsMut<T>
where
    T: ?
Sized
,
{
    // Required method
    fn
as_mut
(&mut self) ->
&mut T
;
}
Expand description
Used to do a cheap mutable-to-mutable reference conversion.
This trait is similar to
AsRef
but used for converting between mutable
references. If you need to do a costly conversion it is better to
implement
From
with type
&mut T
or write a custom function.
Note: This trait must not fail
. If the conversion can fail, use a
dedicated method which returns an
Option<T>
or a
Result<T, E>
.
§
Generic Implementations
AsMut
auto-dereferences if the inner type is a mutable reference
(e.g.:
foo.as_mut()
will work the same if
foo
has type
&mut Foo
or
&mut &mut Foo
).
Note that due to historic reasons, the above currently does not hold generally for all
mutably dereferenceable types
, e.g.
foo.as_mut()
will
not
work the same as
Box::new(foo).as_mut()
. Instead, many smart pointers provide an
as_mut
implementation which
simply returns a reference to the
pointed-to value
(but do not perform a cheap
reference-to-reference conversion for that value). However,
AsMut::as_mut
should not be
used for the sole purpose of mutable dereferencing; instead
‘
Deref
coercion’
can be used:
let
mut
x = Box::new(
5i32
);
// Avoid this:
// let y: &mut i32 = x.as_mut();
// Better just write:
let
y:
&mut
i32 =
&mut
x;
Types which implement
DerefMut
should consider to add an implementation of
AsMut<T>
as
follows:
impl
<T> AsMut<T>
for
SomeType
where
<SomeType
as
Deref>::Target: AsMut<T>,
{
fn
as_mut(
&mut
self
) ->
&mut
T {
self
.deref_mut().as_mut()
    }
}
§
Reflexivity
Ideally,
AsMut
would be reflexive, i.e. there would be an
impl<T: ?Sized> AsMut<T> for T
with
as_mut
simply returning its argument unchanged.
Such a blanket implementation is currently
not
provided due to technical restrictions of
Rust’s type system (it would be overlapping with another existing blanket implementation for
&mut T where T: AsMut<U>
which allows
AsMut
to auto-dereference, see “Generic
Implementations” above).
A trivial implementation of
AsMut<T> for T
must be added explicitly for a particular type
T
where needed or desired. Note, however, that not all types from
std
contain such an
implementation, and those cannot be added by external code due to orphan rules.
§
Examples
Using
AsMut
as trait bound for a generic function, we can accept all mutable references that
can be converted to type
&mut T
. Unlike
dereference
, which has a single
target type
,
there can be multiple implementations of
AsMut
for a type. In particular,
Vec<T>
implements
both
AsMut<Vec<T>>
and
AsMut<[T]>
.
In the following, the example functions
caesar
and
null_terminate
provide a generic
interface which work with any type that can be converted by cheap mutable-to-mutable conversion
into a byte slice (
[u8]
) or byte vector (
Vec<u8>
), respectively.
struct
Document {
    info: String,
    content: Vec<u8>,
}
impl
<T:
?
Sized> AsMut<T>
for
Document
where
Vec<u8>: AsMut<T>,
{
fn
as_mut(
&mut
self
) ->
&mut
T {
self
.content.as_mut()
    }
}
fn
caesar<T: AsMut<[u8]>>(data:
&mut
T, key: u8) {
for
byte
in
data.as_mut() {
*
byte = byte.wrapping_add(key);
    }
}
fn
null_terminate<T: AsMut<Vec<u8>>>(data:
&mut
T) {
// Using a non-generic inner function, which contains most of the
    // functionality, helps to minimize monomorphization overhead.
fn
doit(data:
&mut
Vec<u8>) {
let
len = data.len();
if
len ==
0
|| data[len-
1
] !=
0
{
            data.push(
0
);
        }
    }
    doit(data.as_mut());
}
fn
main() {
let
mut
v: Vec<u8> =
vec!
[
1
,
2
,
3
];
    caesar(
&mut
v,
5
);
assert_eq!
(v, [
6
,
7
,
8
]);
    null_terminate(
&mut
v);
assert_eq!
(v, [
6
,
7
,
8
,
0
]);
let
mut
doc = Document {
        info: String::from(
"Example"
),
        content:
vec!
[
17
,
19
,
8
],
    };
    caesar(
&mut
doc,
1
);
assert_eq!
(doc.content, [
18
,
20
,
9
]);
    null_terminate(
&mut
doc);
assert_eq!
(doc.content, [
18
,
20
,
9
,
0
]);
}
Note, however, that APIs don’t need to be generic. In many cases taking a
&mut [u8]
or
&mut Vec<u8>
, for example, is the better choice (callers need to pass the correct type then).
Required Methods
§
1.0.0
·
Source
fn
as_mut
(&mut self) ->
&mut T
Converts this type into a mutable reference of the (usually inferred) input type.
Implementors
§
1.51.0
·
Source
§
impl
AsMut
<
str
> for
str
1.43.0
·
Source
§
impl
AsMut
<
str
> for
String
Source
§
impl
AsMut
<
ByteStr
> for
ByteString
Source
§
impl
AsMut
<[
u8
]> for
ByteStr
Source
§
impl
AsMut
<[
u8
]> for
ByteString
1.0.0
·
Source
§
impl<T>
AsMut
<
[T]
> for
[T]
1.5.0
·
Source
§
impl<T, A>
AsMut
<
[T]
> for
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
AsMut
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
AsMut
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
Source
§
impl<T, A>
AsMut
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
1.0.0
·
Source
§
impl<T, U>
AsMut
<U> for
&mut T
where
    T:
AsMut
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
AsMut
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
AsMut
<
[T]
> for
[T; N]
Source
§
impl<T, const N:
usize
>
AsMut
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