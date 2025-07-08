is_s390x_feature_detected in std::arch - Rust
std
::
arch
Macro
is_s390x_feature_detected
Copy item path
Source
macro_rules! is_s390x_feature_detected {
    ("deflate-conversion") => { ... };
    ("enhanced-sort") => { ... };
    ("guarded-storage") => { ... };
    ("high-word") => { ... };
    ("nnp-assist") => { ... };
    ("transactional-execution") => { ... };
    ("vector") => { ... };
    ("vector-enhancements-1") => { ... };
    ("vector-enhancements-2") => { ... };
    ("vector-packed-decimal") => { ... };
    ("vector-packed-decimal-enhancement") => { ... };
    ("vector-packed-decimal-enhancement-2") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
🔬
This is a nightly-only experimental API. (
stdarch_s390x_feature_detection
#135413
)
Expand description
Checks if
s390x
feature is enabled.