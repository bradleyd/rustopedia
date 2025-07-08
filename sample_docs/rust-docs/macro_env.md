env in std - Rust
std
Macro
env
Copy item path
1.38.0
·
Source
macro_rules! env {
    ($name:expr $(,)?) => { ... };
    ($name:expr, $error_msg:expr $(,)?) => { ... };
}
Expand description
Inspects an environment variable at compile time.
This macro will expand to the value of the named environment variable at
compile time, yielding an expression of type
&'static str
. Use
std::env::var
instead if you want to read the value at runtime.
If the environment variable is not defined, then a compilation error
will be emitted. To not emit a compile error, use the
option_env!
macro instead. A compilation error will also be emitted if the
environment variable is not a valid Unicode string.
§
Examples
let
path:
&
'static
str =
env!
(
"PATH"
);
println!
(
"the $PATH variable at the time of compiling was: {path}"
);
You can customize the error message by passing a string as the second
parameter:
ⓘ
let
doc:
&
'static
str =
env!
(
"documentation"
,
"what's that?!"
);
If the
documentation
environment variable is not defined, you’ll get
the following error:
error: what's that?!