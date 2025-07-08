pattern_type in std::pat - Rust
std
::
pat
Macro
pattern_type
Copy item path
Source
macro_rules! pattern_type {
    ($($arg:tt)*) => { ... };
}
🔬
This is a nightly-only experimental API. (
pattern_type_macro
#123646
)
Expand description
Creates a pattern type.
ⓘ
type
Positive =
std::pat::pattern_type!
(i32 is
1
..);