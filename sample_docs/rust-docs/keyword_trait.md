trait - Rust
Keyword
trait
Copy item path
Source
Expand description
A common interface for a group of types.
A
trait
is like an interface that data types can implement. When a type
implements a trait it can be treated abstractly as that trait using generics
or trait objects.
Traits can be made up of three varieties of associated items:
functions and methods
types
constants
Traits may also contain additional type parameters. Those type parameters
or the trait itself can be constrained by other traits.
Traits can serve as markers or carry other logical semantics that
aren’t expressed through their items. When a type implements that
trait it is promising to uphold its contract.
Send
and
Sync
are two
such marker traits present in the standard library.
See the
Reference
for a lot more information on traits.
§
Examples
Traits are declared using the
trait
keyword. Types can implement them
using
impl
Trait
for
Type
:
trait
Zero {
const
ZERO:
Self
;
fn
is_zero(
&
self
) -> bool;
}
impl
Zero
for
i32 {
const
ZERO:
Self
=
0
;
fn
is_zero(
&
self
) -> bool {
*
self
==
Self
::ZERO
    }
}
assert_eq!
(i32::ZERO,
0
);
assert!
(i32::ZERO.is_zero());
assert!
(!
4
.is_zero());
With an associated type:
trait
Builder {
type
Built;
fn
build(
&
self
) ->
Self
::Built;
}
Traits can be generic, with constraints or without:
trait
MaybeFrom<T> {
fn
maybe_from(value: T) ->
Option
<
Self
>
where
Self
: Sized;
}
Traits can build upon the requirements of other traits. In the example
below
Iterator
is a
supertrait
and
ThreeIterator
is a
subtrait
:
trait
ThreeIterator: Iterator {
fn
next_three(
&mut
self
) ->
Option
<[
Self
::Item;
3
]>;
}
Traits can be used in functions, as parameters:
fn
debug_iter<I: Iterator>(it: I)
where
I::Item: std::fmt::Debug {
for
elem
in
it {
println!
(
"{elem:#?}"
);
    }
}
// u8_len_1, u8_len_2 and u8_len_3 are equivalent
fn
u8_len_1(val:
impl
Into<Vec<u8>>) -> usize {
    val.into().len()
}
fn
u8_len_2<T: Into<Vec<u8>>>(val: T) -> usize {
    val.into().len()
}
fn
u8_len_3<T>(val: T) -> usize
where
T: Into<Vec<u8>>,
{
    val.into().len()
}
Or as return types:
fn
from_zero_to(v: u8) ->
impl
Iterator<Item = u8> {
    (
0
..v).into_iter()
}
The use of the
impl
keyword in this position allows the function writer
to hide the concrete type as an implementation detail which can change
without breaking user’s code.
§
Trait objects
A
trait object
is an opaque value of another type that implements a set of
traits. A trait object implements all specified traits as well as their
supertraits (if any).
The syntax is the following:
dyn BaseTrait + AutoTrait1 + ... AutoTraitN
.
Only one
BaseTrait
can be used so this will not compile:
ⓘ
trait
A {}
trait
B {}
let _
: Box<
dyn
A + B>;
Neither will this, which is a syntax error:
ⓘ
trait
A {}
trait
B {}
let _
: Box<
dyn
A +
dyn
B>;
On the other hand, this is correct:
trait
A {}
let _
: Box<
dyn
A + Send + Sync>;
The
Reference
has more information about trait objects,
their limitations and the differences between editions.
§
Unsafe traits
Some traits may be unsafe to implement. Using the
unsafe
keyword in
front of the trait’s declaration is used to mark this:
unsafe trait
UnsafeTrait {}
unsafe impl
UnsafeTrait
for
i32 {}
§
Differences between the 2015 and 2018 editions
In the 2015 edition the parameters pattern was not needed for traits:
ⓘ
trait
Tr {
fn
f(i32);
}
This behavior is no longer valid in edition 2018.