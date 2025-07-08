dbg in std - Rust
std
Macro
dbg
Copy item path
1.32.0
·
Source
macro_rules! dbg {
    () => { ... };
    ($val:expr $(,)?) => { ... };
    ($($val:expr),+ $(,)?) => { ... };
}
Expand description
Prints and returns the value of a given expression for quick and dirty
debugging.
An example:
let
a =
2
;
let
b =
dbg!
(a *
2
) +
1
;
//      ^-- prints: [src/main.rs:2:9] a * 2 = 4
assert_eq!
(b,
5
);
The macro works by using the
Debug
implementation of the type of
the given expression to print the value to
stderr
along with the
source location of the macro invocation as well as the source code
of the expression.
Invoking the macro on an expression moves and takes ownership of it
before returning the evaluated expression unchanged. If the type
of the expression does not implement
Copy
and you don’t want
to give up ownership, you can instead borrow with
dbg!(&expr)
for some expression
expr
.
The
dbg!
macro works exactly the same in release builds.
This is useful when debugging issues that only occur in release
builds or when debugging in release mode is significantly faster.
Note that the macro is intended as a debugging tool and therefore you
should avoid having uses of it in version control for long periods
(other than in tests and similar).
Debug output from production code is better done with other facilities
such as the
debug!
macro from the
log
crate.
§
Stability
The exact output printed by this macro should not be relied upon
and is subject to future changes.
§
Panics
Panics if writing to
io::stderr
fails.
§
Further examples
With a method call:
fn
foo(n: usize) {
if let
Some
(
_
) =
dbg!
(n.checked_sub(
4
)) {
// ...
}
}

foo(
3
)
This prints to
stderr
:
[src/main.rs:2:22] n.checked_sub(4) = None
Naive factorial implementation:
fn
factorial(n: u32) -> u32 {
if
dbg!
(n <=
1
) {
dbg!
(
1
)
    }
else
{
dbg!
(n * factorial(n -
1
))
    }
}
dbg!
(factorial(
4
));
This prints to
stderr
:
[src/main.rs:2:8] n <= 1 = false
[src/main.rs:2:8] n <= 1 = false
[src/main.rs:2:8] n <= 1 = false
[src/main.rs:2:8] n <= 1 = true
[src/main.rs:3:9] 1 = 1
[src/main.rs:7:9] n * factorial(n - 1) = 2
[src/main.rs:7:9] n * factorial(n - 1) = 6
[src/main.rs:7:9] n * factorial(n - 1) = 24
[src/main.rs:9:1] factorial(4) = 24
The
dbg!(..)
macro moves the input:
ⓘ
/// A wrapper around `usize` which importantly is not Copyable.
#[derive(Debug)]
struct
NoCopy(usize);
let
a = NoCopy(
42
);
let _
=
dbg!
(a);
// <-- `a` is moved here.
let _
=
dbg!
(a);
// <-- `a` is moved again; error!
You can also use
dbg!()
without a value to just print the
file and line whenever it’s reached.
Finally, if you want to
dbg!(..)
multiple values, it will treat them as
a tuple (and return it, too):
assert_eq!
(
dbg!
(
1usize
,
2u32
), (
1
,
2
));
However, a single argument with a trailing comma will still not be treated
as a tuple, following the convention of ignoring trailing commas in macro
invocations. You can use a 1-tuple directly if you need one:
assert_eq!
(
1
,
dbg!
(
1u32
,));
// trailing comma ignored
assert_eq!
((
1
,),
dbg!
((
1u32
,)));
// 1-tuple