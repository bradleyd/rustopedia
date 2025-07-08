compile_error in std - Rust
std
Macro
compile_error
Copy item path
1.38.0
·
Source
macro_rules! compile_error {
    ($msg:expr $(,)?) => { ... };
}
Expand description
Causes compilation to fail with the given error message when encountered.
This macro should be used when a crate uses a conditional compilation strategy to provide
better error messages for erroneous conditions. It’s the compiler-level form of
panic!
,
but emits an error during
compilation
rather than at
runtime
.
§
Examples
Two such examples are macros and
#[cfg]
environments.
Emit a better compiler error if a macro is passed invalid values. Without the final branch,
the compiler would still emit an error, but the error’s message would not mention the two
valid values.
ⓘ
macro_rules!
give_me_foo_or_bar {
    (foo) => {};
    (bar) => {};
    (
$x
:ident) => {
compile_error!
(
"This macro only accepts `foo` or `bar`"
);
    }
}
give_me_foo_or_bar!
(neither);
// ^ will fail at compile time with message "This macro only accepts `foo` or `bar`"
Emit a compiler error if one of a number of features isn’t available.
ⓘ
#[cfg(not(any(feature =
"foo"
, feature =
"bar"
)))]
compile_error!
(
"Either feature \"foo\" or \"bar\" must be enabled for this crate."
);