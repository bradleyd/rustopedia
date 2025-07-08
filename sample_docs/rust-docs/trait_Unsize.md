Unsize in std::marker - Rust
std
::
marker
Trait
Unsize
Copy item path
Source
pub trait Unsize<T>
where
    T: ?
Sized
,
{ }
🔬
This is a nightly-only experimental API. (
unsize
#18598
)
Expand description
Types that can be “unsized” to a dynamically-sized type.
For example, the sized array type
[i8; 2]
implements
Unsize<[i8]>
and
Unsize<dyn fmt::Debug>
.
All implementations of
Unsize
are provided automatically by the compiler.
Those implementations are:
Arrays
[T; N]
implement
Unsize<[T]>
.
A type implements
Unsize<dyn Trait + 'a>
if all of these conditions are met:
The type implements
Trait
.
Trait
is dyn-compatible
1
.
The type is sized.
The type outlives
'a
.
Structs
Foo<..., T1, ..., Tn, ...>
implement
Unsize<Foo<..., U1, ..., Un, ...>>
where any number of (type and const) parameters may be changed if all of these conditions
are met:
Only the last field of
Foo
has a type involving the parameters
T1
, …,
Tn
.
All other parameters of the struct are equal.
Field<T1, ..., Tn>: Unsize<Field<U1, ..., Un>>
, where
Field<...>
stands for the actual
type of the struct’s last field.
Unsize
is used along with
ops::CoerceUnsized
to allow
“user-defined” containers such as
Rc
to contain dynamically-sized
types. See the
DST coercion RFC
and
the nomicon entry on coercion
for more details.
Formerly known as
object safe
.
↩
Implementors
§