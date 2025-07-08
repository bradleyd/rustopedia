Sized in std::marker - Rust
std
::
marker
Trait
Sized
Copy item path
1.0.0
·
Source
pub trait Sized { }
Expand description
Types with a constant size known at compile time.
All type parameters have an implicit bound of
Sized
. The special syntax
?Sized
can be used to remove this bound if it’s not appropriate.
struct
Foo<T>(T);
struct
Bar<T:
?
Sized>(T);
// struct FooUse(Foo<[i32]>); // error: Sized is not implemented for [i32]
struct
BarUse(Bar<[i32]>);
// OK
The one exception is the implicit
Self
type of a trait. A trait does not
have an implicit
Sized
bound as this is incompatible with
trait object
s
where, by definition, the trait needs to work with all possible implementors,
and thus could be any size.
Although Rust will let you bind
Sized
to a trait, you won’t
be able to use it to form a trait object later:
trait
Foo { }
trait
Bar: Sized { }
struct
Impl;
impl
Foo
for
Impl { }
impl
Bar
for
Impl { }
let
x:
&
dyn
Foo =
&
Impl;
// OK
// let y: &dyn Bar = &Impl; // error: the trait `Bar` cannot
                            // be made into an object
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§