is_loongarch_feature_detected in std::arch - Rust
std
::
arch
Macro
is_loongarch_feature_detected
Copy item path
Source
macro_rules! is_loongarch_feature_detected {
    ("f") => { ... };
    ("d") => { ... };
    ("frecipe") => { ... };
    ("lsx") => { ... };
    ("lasx") => { ... };
    ("lbt") => { ... };
    ("lvz") => { ... };
    ("ual") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
🔬
This is a nightly-only experimental API. (
stdarch_loongarch_feature_detection
#117425
)
Expand description
Checks if
loongarch
feature is enabled.
Supported arguments are:
"f"
"d"
"frecipe"
"lsx"
"lasx"
"lbt"
"lvz"
"ual"