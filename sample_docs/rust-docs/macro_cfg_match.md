cfg_match in std - Rust
std
Macro
cfg_match
Copy item path
Source
pub macro cfg_match {
    ({ $($tt:tt)* }) => { ... },
    (_ => { $($output:tt)* }) => { ... },
    ($cfg:meta => $output:tt $($($rest:tt)+)?) => { ... },
}
🔬
This is a nightly-only experimental API. (
cfg_match
#115585
)
Expand description
A macro for defining
#[cfg]
match-like statements.
It is similar to the
if/elif
C preprocessor macro by allowing definition of a cascade of
#[cfg]
cases, emitting the implementation which matches first.
This allows you to conveniently provide a long list
#[cfg]
’d blocks of code
without having to rewrite each clause multiple times.
Trailing
_
wildcard match arms are
optional
and they indicate a fallback branch when
all previous declarations do not evaluate to true.
§
Example
#![feature(cfg_match)]
cfg_match!
{
    unix => {
fn
foo() {
/* unix specific functionality */
}
    }
    target_pointer_width =
"32"
=> {
fn
foo() {
/* non-unix, 32-bit functionality */
}
    }
_
=> {
fn
foo() {
/* fallback implementation */
}
    }
}
If desired, it is possible to return expressions through the use of surrounding braces:
#![feature(cfg_match)]
let
_some_string =
cfg_match!
{{
    unix => {
"With great power comes great electricity bills"
}
_
=> {
"Behind every successful diet is an unwatched pizza"
}
}};