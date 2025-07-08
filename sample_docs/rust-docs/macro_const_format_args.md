const_format_args in std - Rust
std
Macro
const_format_args
Copy item path
Source
macro_rules! const_format_args {
    ($fmt:expr) => { ... };
    ($fmt:expr, $($args:tt)*) => { ... };
}
🔬
This is a nightly-only experimental API. (
const_format_args
)
Expand description
Same as
format_args
, but can be used in some const contexts.
This macro is used by the panic macros for the
const_panic
feature.
This macro will be removed once
format_args
is allowed in const contexts.