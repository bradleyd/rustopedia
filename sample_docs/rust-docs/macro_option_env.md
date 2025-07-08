option_env in std - Rust
std
Macro
option_env
Copy item path
1.38.0
·
Source
macro_rules! option_env {
    ($name:expr $(,)?) => { ... };
}
Expand description
Optionally inspects an environment variable at compile time.
If the named environment variable is present at compile time, this will
expand into an expression of type
Option<&'static str>
whose value is
Some
of the value of the environment variable (a compilation error
will be emitted if the environment variable is not a valid Unicode
string). If the environment variable is not present, then this will
expand to
None
. See
Option<T>
for more information on this
type.  Use
std::env::var
instead if you want to read the value at
runtime.
A compile time error is only emitted when using this macro if the
environment variable exists and is not a valid Unicode string. To also
emit a compile error if the environment variable is not present, use the
env!
macro instead.
§
Examples
let
key:
Option
<
&
'static
str> =
option_env!
(
"SECRET_KEY"
);
println!
(
"the secret key might be: {key:?}"
);