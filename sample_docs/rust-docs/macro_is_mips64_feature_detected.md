is_mips64_feature_detected in std::arch - Rust
std
::
arch
Macro
is_mips64_feature_detected
Copy item path
Source
macro_rules! is_mips64_feature_detected {
    ("msa") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
🔬
This is a nightly-only experimental API. (
stdarch_mips_feature_detection
#111188
)
Expand description
Checks if
mips64
feature is enabled.