ConstParamTy_ in std::marker - Rust
std
::
marker
Trait
ConstParamTy_
Copy item path
Source
pub trait ConstParamTy_:
UnsizedConstParamTy
+
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
These types must have a proper equivalence relation (
Eq
) and it must be automatically
derived (
StructuralPartialEq
). There’s a hard-coded check in the compiler ensuring
that all fields are also
ConstParamTy
, which implies that recursively, all fields
are
StructuralPartialEq
.
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
ConstParamTy_
for
bool
Source
§
impl
ConstParamTy_
for
char
Source
§
impl
ConstParamTy_
for
i8
Source
§
impl
ConstParamTy_
for
i16
Source
§
impl
ConstParamTy_
for
i32
Source
§
impl
ConstParamTy_
for
i64
Source
§
impl
ConstParamTy_
for
i128
Source
§
impl
ConstParamTy_
for
isize
Source
§
impl
ConstParamTy_
for
u8
Source
§
impl
ConstParamTy_
for
u16
Source
§
impl
ConstParamTy_
for
u32
Source
§
impl
ConstParamTy_
for
u64
Source
§
impl
ConstParamTy_
for
u128
Source
§
impl
ConstParamTy_
for
()
Source
§
impl
ConstParamTy_
for
usize
Source
§
impl
ConstParamTy_
for
Assume
Source
§
impl<T>
ConstParamTy_
for
(T₁, T₂, …, Tₙ)
where
    T:
ConstParamTy_
,
This trait is implemented for tuples up to twelve items long.
Source
§
impl<T, const N:
usize
>
ConstParamTy_
for
[T; N]
where
    T:
ConstParamTy_
,