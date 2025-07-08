is_arm_feature_detected in std::arch - Rust
std
::
arch
Macro
is_arm_feature_detected
Copy item path
Source
macro_rules! is_arm_feature_detected {
    ("neon") => { ... };
    ("pmull") => { ... };
    ("crc") => { ... };
    ("aes") => { ... };
    ("sha2") => { ... };
    ("i8mm") => { ... };
    ("dotprod") => { ... };
    ("v7") => { ... };
    ("vfp2") => { ... };
    ("vfp3") => { ... };
    ("vfp4") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
🔬
This is a nightly-only experimental API. (
stdarch_arm_feature_detection
#111190
)
Expand description
Checks if
arm
feature is enabled.