must_use in std::hint - Rust
std
::
hint
Function
must_use
Copy item path
Source
pub const fn must_use<T>(value: T) -> T
🔬
This is a nightly-only experimental API. (
hint_must_use
#94745
)
Expand description
An identity function that causes an
unused_must_use
warning to be
triggered if the given value is not used (returned, stored in a variable,
etc) by the caller.
This is primarily intended for use in macro-generated code, in which a
#[must_use]
attribute
either on a type or a function would not
be convenient.
§
Example
#![feature(hint_must_use)]
use
core::fmt;
pub struct
Error(
/* ... */
);
#[macro_export]
macro_rules!
make_error {
    ($(
$args
:expr),
*
) => {
        core::hint::must_use({
let
error =
$crate::make_error
(
core::format_args!
($(
$args
),
*
));
            error
        })
    };
}
// Implementation detail of make_error! macro.
#[doc(hidden)]
pub fn
make_error(args: fmt::Arguments<
'_
>) -> Error {
    Error(
/* ... */
)
}
fn
demo() ->
Option
<Error> {
if
true
{
// Oops, meant to write `return Some(make_error!("..."));`
Some
(
make_error!
(
"..."
));
    }
None
}
In the above example, we’d like an
unused_must_use
lint to apply to the
value created by
make_error!
. However, neither
#[must_use]
on a struct
nor
#[must_use]
on a function is appropriate here, so the macro expands
using
core::hint::must_use
instead.
We wouldn’t want
#[must_use]
on the
struct Error
because that would
make the following unproblematic code trigger a warning:
fn
f(arg:
&
str) ->
Result
<(), Error>
#[test]
fn
t() {
// Assert that `f` returns error if passed an empty string.
    // A value of type `Error` is unused here but that's not a problem.
f(
""
).unwrap_err();
}
Using
#[must_use]
on
fn make_error
can’t help because the return value
is
used, as the right-hand side of a
let
statement. The
let
statement looks useless but is in fact necessary for ensuring that
temporaries within the
format_args
expansion are not kept alive past the
creation of the
Error
, as keeping them alive past that point can cause
autotrait issues in async code:
async fn
f() {
// Using `let` inside the make_error expansion causes temporaries like
    // `unsync()` to drop at the semicolon of that `let` statement, which
    // is prior to the await point. They would otherwise stay around until
    // the semicolon on *this* statement, which is after the await point,
    // and the enclosing Future would not implement Send.
log(
make_error!
(
"look: {:p}"
, unsync())).
await
;
}
async fn
log(error: Error) {
/* ... */
}
// Returns something without a Sync impl.
fn
unsync() ->
*const
() {
0
as
*const
()
}