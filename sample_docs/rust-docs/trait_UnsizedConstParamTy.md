UnsizedConstParamTy in std::marker - Rust
std
::
marker
Trait
UnsizedConstParamTy
Copy item path
Source
pub trait UnsizedConstParamTy:
StructuralPartialEq
+
Eq
{ }
🔬
This is a nightly-only experimental API. (
unsized_const_params
#95174
)
Expand description
A marker for types which can be used as types of
const
generic parameters.
Equivalent to
ConstParamTy_
except that this is used by
the
unsized_const_params
to allow for fake unstable impls.
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
impl
UnsizedConstParamTy
for
bool
Source
§
impl
UnsizedConstParamTy
for
char
Source
§
impl
UnsizedConstParamTy
for
i8
Source
§
impl
UnsizedConstParamTy
for
i16
Source
§
impl
UnsizedConstParamTy
for
i32
Source
§
impl
UnsizedConstParamTy
for
i64
Source
§
impl
UnsizedConstParamTy
for
i128
Source
§
impl
UnsizedConstParamTy
for
isize
Source
§
impl
UnsizedConstParamTy
for
str
Source
§
impl
UnsizedConstParamTy
for
u8
Source
§
impl
UnsizedConstParamTy
for
u16
Source
§
impl
UnsizedConstParamTy
for
u32
Source
§
impl
UnsizedConstParamTy
for
u64
Source
§
impl
UnsizedConstParamTy
for
u128
Source
§
impl
UnsizedConstParamTy
for
()
Source
§
impl
UnsizedConstParamTy
for
usize
Source
§
impl
UnsizedConstParamTy
for
Assume
Source
§
impl<T>
UnsizedConstParamTy
for
&T
where
    T:
UnsizedConstParamTy
+ ?
Sized
,
Source
§
impl<T>
UnsizedConstParamTy
for
[T]
where
    T:
UnsizedConstParamTy
,
Source
§
impl<T>
UnsizedConstParamTy
for
(T₁, T₂, …, Tₙ)
where
    T:
UnsizedConstParamTy
,
This trait is implemented for tuples up to twelve items long.
Source
§
impl<T, const N:
usize
>
UnsizedConstParamTy
for
[T; N]
where
    T:
UnsizedConstParamTy
,