let - Rust
Keyword
let
Copy item path
Source
Expand description
Bind a value to a variable.
The primary use for the
let
keyword is in
let
statements, which are used to introduce a new
set of variables into the current scope, as given by a pattern.
let
thing1: i32 =
100
;
let
thing2 =
200
+ thing1;
let
mut
changing_thing =
true
;
changing_thing =
false
;
let
(part1, part2) = (
"first"
,
"second"
);
struct
Example {
    a: bool,
    b: u64,
}
let
Example { a, b:
_
} = Example {
    a:
true
,
    b:
10004
,
};
assert!
(a);
The pattern is most commonly a single variable, which means no pattern matching is done and
the expression given is bound to the variable. Apart from that, patterns used in
let
bindings
can be as complicated as needed, given that the pattern is exhaustive. See the
Rust
book
for more information on pattern matching. The type of the pattern is optionally
given afterwards, but if left blank is automatically inferred by the compiler if possible.
Variables in Rust are immutable by default, and require the
mut
keyword to be made mutable.
Multiple variables can be defined with the same name, known as shadowing. This doesn’t affect
the original variable in any way beyond being unable to directly access it beyond the point of
shadowing. It continues to remain in scope, getting dropped only when it falls out of scope.
Shadowed variables don’t need to have the same type as the variables shadowing them.
let
shadowing_example =
true
;
let
shadowing_example =
123.4
;
let
shadowing_example = shadowing_example
as
u32;
let
mut
shadowing_example =
format!
(
"cool! {shadowing_example}"
);
shadowing_example +=
" something else!"
;
// not shadowing
Other places the
let
keyword is used include along with
if
, in the form of
if let
expressions. They’re useful if the pattern being matched isn’t exhaustive, such as with
enumerations.
while let
also exists, which runs a loop with a pattern matched value until
that pattern can’t be matched.
For more information on the
let
keyword, see the
Rust book
or the
Reference