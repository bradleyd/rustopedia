ref - Rust
Keyword
ref
Copy item path
Source
Expand description
Bind by reference during pattern matching.
ref
annotates pattern bindings to make them borrow rather than move.
It is
not
a part of the pattern as far as matching is concerned: it does
not affect
whether
a value is matched, only
how
it is matched.
By default,
match
statements consume all they can, which can sometimes
be a problem, when you don’t really need the value to be moved and owned:
ⓘ
let
maybe_name =
Some
(String::from(
"Alice"
));
// The variable 'maybe_name' is consumed here ...
match
maybe_name {
Some
(n) =>
println!
(
"Hello, {n}"
),
_
=>
println!
(
"Hello, world"
),
}
// ... and is now unavailable.
println!
(
"Hello again, {}"
, maybe_name.unwrap_or(
"world"
.into()));
Using the
ref
keyword, the value is only borrowed, not moved, making it
available for use after the
match
statement:
let
maybe_name =
Some
(String::from(
"Alice"
));
// Using `ref`, the value is borrowed, not moved ...
match
maybe_name {
Some
(
ref
n) =>
println!
(
"Hello, {n}"
),
_
=>
println!
(
"Hello, world"
),
}
// ... so it's available here!
println!
(
"Hello again, {}"
, maybe_name.unwrap_or(
"world"
.into()));
§
&
vs
ref
&
denotes that your pattern expects a reference to an object. Hence
&
is a part of said pattern:
&Foo
matches different objects than
Foo
does.
ref
indicates that you want a reference to an unpacked value. It is not
matched against:
Foo(ref foo)
matches the same objects as
Foo(foo)
.
See also the
Reference
for more information.