unimplemented in std - Rust
std
Macro
unimplemented
Copy item path
1.0.0
·
Source
macro_rules! unimplemented {
    () => { ... };
    ($($arg:tt)+) => { ... };
}
Expand description
Indicates unimplemented code by panicking with a message of “not implemented”.
This allows your code to type-check, which is useful if you are prototyping or
implementing a trait that requires multiple methods which you don’t plan to use all of.
The difference between
unimplemented!
and
todo!
is that while
todo!
conveys an intent of implementing the functionality later and the message is “not yet
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
unimplemented!
is just a shorthand for
panic!
with a
fixed, specific message.
Like
panic!
, this macro has a second form for displaying custom values.
§
Examples
Say we have a trait
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
for ‘MyStruct’, but for some reason it only makes sense
to implement the
bar()
function.
baz()
and
qux()
will still need to be defined
in our implementation of
Foo
, but we can use
unimplemented!
in their definitions
to allow our code to compile.
We still want to have our program stop running if the unimplemented methods are
reached.
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
// It makes no sense to `baz` a `MyStruct`, so we have no logic here
        // at all.
        // This will display "thread 'main' panicked at 'not implemented'".
unimplemented!
();
    }
fn
qux(
&
self
) ->
Result
<u64, ()> {
// We have some logic here,
        // We can add a message to unimplemented! to display our omission.
        // This will display:
        // "thread 'main' panicked at 'not implemented: MyStruct isn't quxable'".
unimplemented!
(
"MyStruct isn't quxable"
);
    }
}
fn
main() {
let
s = MyStruct;
    s.bar();
}