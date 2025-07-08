UseCloned in std::clone - Rust
std
::
clone
Trait
UseCloned
Copy item path
Source
pub trait UseCloned:
Clone
{ }
🔬
This is a nightly-only experimental API. (
ergonomic_clones
#132290
)
Expand description
Trait for objects whose
Clone
impl is lightweight (e.g. reference-counted)
Cloning an object implementing this trait should in general:
be O(1) (constant) time regardless of the amount of data managed by the object,
not require a memory allocation,
not require copying more than roughly 64 bytes (a typical cache line size),
not block the current thread,
not have any semantic side effects (e.g. allocating a file descriptor), and
not have overhead larger than a couple of atomic operations.
The
UseCloned
trait does not provide a method; instead, it indicates that
Clone::clone
is lightweight, and allows the use of the
.use
syntax.
§
.use postfix syntax
Values can be
.use
d by adding
.use
postfix to the value you want to use.
ⓘ
fn
foo(f: Foo) {
// if `Foo` implements `Copy` f would be copied into x.
    // if `Foo` implements `UseCloned` f would be cloned into x.
    // otherwise f would be moved into x.
let
x = f.
use
;
// ...
}
§
use closures
Use closures allow captured values to be automatically used.
This is similar to have a closure that you would call
.use
over each captured value.
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
UseCloned
for
bool
Source
§
impl
UseCloned
for
char
Source
§
impl
UseCloned
for
f16
Source
§
impl
UseCloned
for
f32
Source
§
impl
UseCloned
for
f64
Source
§
impl
UseCloned
for
f128
Source
§
impl
UseCloned
for
i8
Source
§
impl
UseCloned
for
i16
Source
§
impl
UseCloned
for
i32
Source
§
impl
UseCloned
for
i64
Source
§
impl
UseCloned
for
i128
Source
§
impl
UseCloned
for
isize
Source
§
impl
UseCloned
for
u8
Source
§
impl
UseCloned
for
u16
Source
§
impl
UseCloned
for
u32
Source
§
impl
UseCloned
for
u64
Source
§
impl
UseCloned
for
u128
Source
§
impl
UseCloned
for
usize
Source
§
impl<T>
UseCloned
for
Option
<T>
where
    T:
UseCloned
,
Source
§
impl<T>
UseCloned
for
NonZero
<T>
where
    T:
ZeroablePrimitive
,
Source
§
impl<T, A>
UseCloned
for
Rc
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
§
impl<T, A>
UseCloned
for std::rc::
Weak
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
§
impl<T, A>
UseCloned
for
Arc
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
§
impl<T, A>
UseCloned
for std::sync::
Weak
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
§
impl<T, E>
UseCloned
for
Result
<T, E>
where
    T:
UseCloned
,
    E:
UseCloned
,