Concat in std::slice - Rust
std
::
slice
Trait
Concat
Copy item path
Source
pub trait Concat<Item>
where
    Item: ?
Sized
,
{
    type
Output
;

    // Required method
    fn
concat
(slice: &Self) -> Self::
Output
;
}
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
Expand description
Helper trait for
[T]::concat
.
Note: the
Item
type parameter is not used in this trait,
but it allows impls to be more generic.
Without it, we get this error:
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predica
   --> library/alloc/src/slice.rs:608:6
    |
608 | impl<T: Clone, V: Borrow<[T]>> Concat for [V] {
    |      ^ unconstrained type parameter
This is because there could exist
V
types with multiple
Borrow<[_]>
impls,
such that multiple
T
types would apply:
pub struct
Foo(Vec<u32>, Vec<String>);
impl
std::borrow::Borrow<[u32]>
for
Foo {
fn
borrow(
&
self
) ->
&
[u32] {
&
self
.
0
}
}
impl
std::borrow::Borrow<[String]>
for
Foo {
fn
borrow(
&
self
) ->
&
[String] {
&
self
.
1
}
}
Required Associated Types
§
Source
type
Output
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
The resulting type after concatenation
Required Methods
§
Source
fn
concat
(slice: &Self) -> Self::
Output
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
Implementation of
[T]::concat
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
impl<S>
Concat
<
str
> for
[S]
where
    S:
Borrow
<
str
>,
Note:
str
in
Concat<str>
is not meaningful here.
This type parameter of the trait only exists to enable another impl.
Source
§
type
Output
=
String
Source
§
impl<T, V>
Concat
<T> for
[V]
where
    T:
Clone
,
    V:
Borrow
<
[T]
>,
Source
§
type
Output
=
Vec
<T>