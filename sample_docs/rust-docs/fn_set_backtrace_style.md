set_backtrace_style in std::panic - Rust
std
::
panic
Function
set_backtrace_style
Copy item path
Source
pub fn set_backtrace_style(style:
BacktraceStyle
)
🔬
This is a nightly-only experimental API. (
panic_backtrace_config
#93346
)
Expand description
Configures whether the default panic hook will capture and display a
backtrace.
The default value for this setting may be set by the
RUST_BACKTRACE
environment variable; see the details in
get_backtrace_style
.