self - Rust
Keyword
self
Copy item path
Source
Expand description
The receiver of a method, or the current module.
self
is used in two situations: referencing the current module and marking
the receiver of a method.
In paths,
self
can be used to refer to the current module, either in a
use
statement or in a path to access an element:
use
std::io::{
self
, Read};
Is functionally the same as:
use
std::io;
use
std::io::Read;
Using
self
to access an element in the current module:
fn
foo() {}
fn
bar() {
self
::foo()
}
self
as the current receiver for a method allows to omit the parameter
type most of the time. With the exception of this particularity,
self
is
used much like any other parameter:
struct
Foo(i32);
impl
Foo {
// No `self`.
fn
new() ->
Self
{
Self
(
0
)
    }
// Consuming `self`.
fn
consume(
self
) ->
Self
{
Self
(
self
.
0
+
1
)
    }
// Borrowing `self`.
fn
borrow(
&
self
) ->
&
i32 {
&
self
.
0
}
// Borrowing `self` mutably.
fn
borrow_mut(
&mut
self
) ->
&mut
i32 {
&mut
self
.
0
}
}
// This method must be called with a `Type::` prefix.
let
foo = Foo::new();
assert_eq!
(foo.
0
,
0
);
// Those two calls produces the same result.
let
foo = Foo::consume(foo);
assert_eq!
(foo.
0
,
1
);
let
foo = foo.consume();
assert_eq!
(foo.
0
,
2
);
// Borrowing is handled automatically with the second syntax.
let
borrow_1 = Foo::borrow(
&
foo);
let
borrow_2 = foo.borrow();
assert_eq!
(borrow_1, borrow_2);
// Borrowing mutably is handled automatically too with the second syntax.
let
mut
foo = Foo::new();
*
Foo::borrow_mut(
&mut
foo) +=
1
;
assert_eq!
(foo.
0
,
1
);
*
foo.borrow_mut() +=
1
;
assert_eq!
(foo.
0
,
2
);
Note that this automatic conversion when calling
foo.method()
is not
limited to the examples above. See the
Reference
for more information.