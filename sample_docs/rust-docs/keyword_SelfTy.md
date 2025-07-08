SelfTy - Rust
Keyword
SelfTy
Copy item path
Source
Expand description
The implementing type within a
trait
or
impl
block, or the current type within a type
definition.
Within a type definition:
struct
Node {
    elem: i32,
// `Self` is a `Node` here.
next:
Option
<Box<
Self
>>,
}
In an
impl
block:
struct
Foo(i32);
impl
Foo {
fn
new() ->
Self
{
Self
(
0
)
    }
}
assert_eq!
(Foo::new().
0
, Foo(
0
).
0
);
Generic parameters are implicit with
Self
:
struct
Wrap<T> {
    elem: T,
}
impl
<T> Wrap<T> {
fn
new(elem: T) ->
Self
{
Self
{ elem }
    }
}
In a
trait
definition and related
impl
block:
trait
Example {
fn
example() ->
Self
;
}
struct
Foo(i32);
impl
Example
for
Foo {
fn
example() ->
Self
{
Self
(
42
)
    }
}
assert_eq!
(Foo::example().
0
, Foo(
42
).
0
);