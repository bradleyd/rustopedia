PanicInfo in std::panic - Rust
std
::
panic
Type Alias
PanicInfo
Copy item path
1.10.0
·
Source
pub type PanicInfo<'a> =
PanicHookInfo
<'a>;
👎
Deprecated since 1.82.0: use
PanicHookInfo
instead
Expand description
A struct providing information about a panic.
PanicInfo
has been renamed to
PanicHookInfo
to avoid confusion with
core::panic::PanicInfo
.
Aliased Type
§
struct PanicInfo<'a> {
/* private fields */
}