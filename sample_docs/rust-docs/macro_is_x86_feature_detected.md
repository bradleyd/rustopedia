is_x86_feature_detected in std::arch - Rust
std
::
arch
Macro
is_x86_feature_detected
Copy item path
1.27.0
·
Source
macro_rules! is_x86_feature_detected {
    ("aes") => { ... };
    ("pclmulqdq") => { ... };
    ("rdrand") => { ... };
    ("rdseed") => { ... };
    ("tsc") => { ... };
    ("mmx") => { ... };
    ("sse") => { ... };
    ("sse2") => { ... };
    ("sse3") => { ... };
    ("ssse3") => { ... };
    ("sse4.1") => { ... };
    ("sse4.2") => { ... };
    ("sse4a") => { ... };
    ("sha") => { ... };
    ("avx") => { ... };
    ("avx2") => { ... };
    ("sha512") => { ... };
    ("sm3") => { ... };
    ("sm4") => { ... };
    ("avx512f") => { ... };
    ("avx512cd") => { ... };
    ("avx512er") => { ... };
    ("avx512pf") => { ... };
    ("avx512bw") => { ... };
    ("avx512dq") => { ... };
    ("avx512vl") => { ... };
    ("avx512ifma") => { ... };
    ("avx512vbmi") => { ... };
    ("avx512vpopcntdq") => { ... };
    ("avx512vbmi2") => { ... };
    ("gfni") => { ... };
    ("vaes") => { ... };
    ("vpclmulqdq") => { ... };
    ("avx512vnni") => { ... };
    ("avx512bitalg") => { ... };
    ("avx512bf16") => { ... };
    ("avx512vp2intersect") => { ... };
    ("avx512fp16") => { ... };
    ("avxifma") => { ... };
    ("avxneconvert") => { ... };
    ("avxvnni") => { ... };
    ("avxvnniint16") => { ... };
    ("avxvnniint8") => { ... };
    ("amx-tile") => { ... };
    ("amx-int8") => { ... };
    ("amx-bf16") => { ... };
    ("amx-fp16") => { ... };
    ("amx-complex") => { ... };
    ("f16c") => { ... };
    ("fma") => { ... };
    ("bmi1") => { ... };
    ("bmi2") => { ... };
    ("lzcnt") => { ... };
    ("tbm") => { ... };
    ("popcnt") => { ... };
    ("fxsr") => { ... };
    ("xsave") => { ... };
    ("xsaveopt") => { ... };
    ("xsaves") => { ... };
    ("xsavec") => { ... };
    ("cmpxchg16b") => { ... };
    ("kl") => { ... };
    ("widekl") => { ... };
    ("adx") => { ... };
    ("rtm") => { ... };
    ("movbe") => { ... };
    ("ermsb") => { ... };
    ("xop") => { ... };
    ("abm") => { ... };
    ("avx512gfni") => { ... };
    ("avx512vaes") => { ... };
    ("avx512vpclmulqdq") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
Expand description
A macro to test at
runtime
whether a CPU feature is available on
x86/x86-64 platforms.
This macro is provided in the standard library and will detect at runtime
whether the specified CPU feature is detected. This does
not
resolve at
compile time unless the specified feature is already enabled for the entire
crate. Runtime detection currently relies mostly on the
cpuid
instruction.
This macro only takes one argument which is a string literal of the feature
being tested for. The feature names supported are the lowercase versions of
the ones defined by Intel in
their documentation
.
§
Supported arguments
This macro supports the same names that
#[target_feature]
supports. Unlike
#[target_feature]
, however, this macro does not support names separated
with a comma. Instead testing for multiple features must be done through
separate macro invocations for now.
Supported arguments are:
"aes"
"pclmulqdq"
"rdrand"
"rdseed"
"tsc"
"mmx"
"sse"
"sse2"
"sse3"
"ssse3"
"sse4.1"
"sse4.2"
"sse4a"
"sha"
"avx"
"avx2"
"sha512"
"sm3"
"sm4"
"avx512f"
"avx512cd"
"avx512er"
"avx512pf"
"avx512bw"
"avx512dq"
"avx512vl"
"avx512ifma"
"avx512vbmi"
"avx512vpopcntdq"
"avx512vbmi2"
"gfni"
"vaes"
"vpclmulqdq"
"avx512vnni"
"avx512bitalg"
"avx512bf16"
"avx512vp2intersect"
"avx512fp16"
"avxvnni"
"avxifma"
"avxneconvert"
"avxvnniint8"
"avxvnniint16"
"amx-tile"
"amx-int8"
"amx-bf16"
"amx-fp16"
"amx-complex"
"f16c"
"fma"
"bmi1"
"bmi2"
"abm"
"lzcnt"
"tbm"
"popcnt"
"fxsr"
"xsave"
"xsaveopt"
"xsaves"
"xsavec"
"cmpxchg16b"
"kl"
"widekl"
"adx"
"rtm"
"movbe"
"ermsb"