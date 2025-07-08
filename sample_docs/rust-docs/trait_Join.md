Join in std::slice - Rust
std
::
slice
Trait
Join
Copy item path
Source
pub trait Join<Separator> {
    type
Output
;

    // Required method
    fn
join
(slice: &Self, sep: Separator) -> Self::
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
[T]::join
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
join
(slice: &Self, sep: Separator) -> Self::
Output
🔬
This is a nightly-only experimental API. (
slice_concat_trait
#27747
)
Implementation of
[T]::join
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
Join
<&
str
> for
[S]
where
    S:
Borrow
<
str
>,
Source
§
type
Output
=
String
Source
§
impl<S:
Borrow
<
OsStr
>>
Join
<&
OsStr
> for
[S]
Source
§
type
Output
=
OsString
Source
§
impl<T, V>
Join
<&
[T]
> for
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
Source
§
impl<T, V>
Join
<
&T
> for
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