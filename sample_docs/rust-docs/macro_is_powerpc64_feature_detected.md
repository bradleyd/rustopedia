is_powerpc64_feature_detected in std::arch - Rust
std
::
arch
Macro
is_powerpc64_feature_detected
Copy item path
Source
macro_rules! is_powerpc64_feature_detected {
    ("altivec") => { ... };
    ("vsx") => { ... };
    ("power8") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
🔬
This is a nightly-only experimental API. (
stdarch_powerpc_feature_detection
#111191
)
Expand description
Checks if
powerpc
feature is enabled.