format_args_nl in std - Rust
std
Macro
format_args_nl
Copy item path
Source
macro_rules! format_args_nl {
    ($fmt:expr) => { ... };
    ($fmt:expr, $($args:tt)*) => { ... };
}
🔬
This is a nightly-only experimental API. (
format_args_nl
)
Expand description
Same as
format_args
, but adds a newline in the end.