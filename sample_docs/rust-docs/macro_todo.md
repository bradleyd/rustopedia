todo in std - Rust
std
Macro
todo
Copy item path
1.0.0
·
Source
macro_rules! todo {
    () => { ... };
    ($($arg:tt)+) => { ... };
}
Expand description
Indicates unfinished code.
This can be useful if you are prototyping and just
want a placeholder to let your code pass type analysis.
The difference between
unimplemented!
and
todo!
is that while
todo!
conveys
an intent of implementing the functionality later and the message is “not yet
implemented”,
unimplemented!
makes no such claims. Its message is “not implemented”.
Also, some IDEs will mark
todo!
s.
§
Panics
This will always
panic!
because
todo!
is just a shorthand for
panic!
with a
fixed, specific message.
Like
panic!
, this macro has a second form for displaying custom values.
§
Examples
Here’s an example of some in-progress code. We have a trait
Foo
:
trait
Foo {
fn
bar(
&
self
) -> u8;
fn
baz(
&
self
);
fn
qux(
&
self
) ->
Result
<u64, ()>;
}
We want to implement
Foo
on one of our types, but we also want to work on
just
bar()
first. In order for our code to compile, we need to implement
baz()
and
qux()
, so we can use
todo!
:
struct
MyStruct;
impl
Foo
for
MyStruct {
fn
bar(
&
self
) -> u8 {
1
+
1
}
fn
baz(
&
self
) {
// Let's not worry about implementing baz() for now
todo!
();
    }
fn
qux(
&
self
) ->
Result
<u64, ()> {
// We can add a message to todo! to display our omission.
        // This will display:
        // "thread 'main' panicked at 'not yet implemented: MyStruct is not yet quxable'".
todo!
(
"MyStruct is not yet quxable"
);
    }
}
fn
main() {
let
s = MyStruct;
    s.bar();
// We aren't even using baz() or qux(), so this is fine.
}