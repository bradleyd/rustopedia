concat_idents in std - Rust
std
Macro
concat_idents
Copy item path
Source
macro_rules! concat_idents {
    ($($e:ident),+ $(,)?) => { ... };
}
🔬
This is a nightly-only experimental API. (
concat_idents
#29599
)
Expand description
Concatenates identifiers into one identifier.
This macro takes any number of comma-separated identifiers, and
concatenates them all into one, yielding an expression which is a new
identifier. Note that hygiene makes it such that this macro cannot
capture local variables. Also, as a general rule, macros are only
allowed in item, statement or expression position. That means while
you may use this macro for referring to existing variables, functions or
modules etc, you cannot define a new one with it.
§
Examples
#![feature(concat_idents)]
fn
foobar() -> u32 {
23
}
let
f =
concat_idents!
(foo, bar);
println!
(
"{}"
, f());
// fn concat_idents!(new, fun, name) { } // not usable in this way!