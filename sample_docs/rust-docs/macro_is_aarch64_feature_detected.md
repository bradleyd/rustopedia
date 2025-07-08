is_aarch64_feature_detected in std::arch - Rust
std
::
arch
Macro
is_aarch64_feature_detected
Copy item path
1.60.0
·
Source
macro_rules! is_aarch64_feature_detected {
    ("neon") => { ... };
    ("pmull") => { ... };
    ("fp") => { ... };
    ("aes") => { ... };
    ("bf16") => { ... };
    ("bti") => { ... };
    ("crc") => { ... };
    ("cssc") => { ... };
    ("dit") => { ... };
    ("dpb") => { ... };
    ("dpb2") => { ... };
    ("dotprod") => { ... };
    ("ecv") => { ... };
    ("f32mm") => { ... };
    ("f64mm") => { ... };
    ("faminmax") => { ... };
    ("fcma") => { ... };
    ("fhm") => { ... };
    ("flagm") => { ... };
    ("flagm2") => { ... };
    ("fp16") => { ... };
    ("fp8") => { ... };
    ("fp8dot2") => { ... };
    ("fp8dot4") => { ... };
    ("fp8fma") => { ... };
    ("fpmr") => { ... };
    ("frintts") => { ... };
    ("hbc") => { ... };
    ("i8mm") => { ... };
    ("jsconv") => { ... };
    ("lse") => { ... };
    ("lse128") => { ... };
    ("lse2") => { ... };
    ("lut") => { ... };
    ("mops") => { ... };
    ("mte") => { ... };
    ("paca") => { ... };
    ("pacg") => { ... };
    ("pauth-lr") => { ... };
    ("rand") => { ... };
    ("rcpc") => { ... };
    ("rcpc2") => { ... };
    ("rcpc3") => { ... };
    ("rdm") => { ... };
    ("sb") => { ... };
    ("sha2") => { ... };
    ("sha3") => { ... };
    ("sm4") => { ... };
    ("sme") => { ... };
    ("sme2") => { ... };
    ("sme2p1") => { ... };
    ("sme-b16b16") => { ... };
    ("sme-f16f16") => { ... };
    ("sme-f64f64") => { ... };
    ("sme-f8f16") => { ... };
    ("sme-f8f32") => { ... };
    ("sme-fa64") => { ... };
    ("sme-i16i64") => { ... };
    ("sme-lutv2") => { ... };
    ("ssbs") => { ... };
    ("ssve-fp8dot2") => { ... };
    ("ssve-fp8dot4") => { ... };
    ("ssve-fp8fma") => { ... };
    ("sve") => { ... };
    ("sve2") => { ... };
    ("sve2p1") => { ... };
    ("sve2-aes") => { ... };
    ("sve-b16b16") => { ... };
    ("sve2-bitperm") => { ... };
    ("sve2-sha3") => { ... };
    ("sve2-sm4") => { ... };
    ("tme") => { ... };
    ("wfxt") => { ... };
    ("asimd") => { ... };
    ("ras") => { ... };
    ("v8.1a") => { ... };
    ("v8.2a") => { ... };
    ("v8.3a") => { ... };
    ("v8.4a") => { ... };
    ("v8.5a") => { ... };
    ("v8.6a") => { ... };
    ("v8.7a") => { ... };
    ("v8.8a") => { ... };
    ("v8.9a") => { ... };
    ("v9.1a") => { ... };
    ("v9.2a") => { ... };
    ("v9.3a") => { ... };
    ("v9.4a") => { ... };
    ("v9.5a") => { ... };
    ("v9a") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
Expand description
This macro tests, at runtime, whether an
aarch64
feature is enabled on aarch64 platforms.
Currently most features are only supported on linux-based platforms.
This macro takes one argument which is a string literal of the feature being tested for.
The feature names are mostly taken from their FEAT_* definitions in the
ARM Architecture
Reference Manual
.
§
Supported arguments
"aes"
- FEAT_AES & FEAT_PMULL
"asimd"
or “neon” - FEAT_AdvSIMD
"bf16"
- FEAT_BF16
"bti"
- FEAT_BTI
"crc"
- FEAT_CRC
"cssc"
- FEAT_CSSC
"dit"
- FEAT_DIT
"dotprod"
- FEAT_DotProd
"dpb"
- FEAT_DPB
"dpb2"
- FEAT_DPB2
"ecv"
- FEAT_ECV
"f32mm"
- FEAT_F32MM
"f64mm"
- FEAT_F64MM
"faminmax"
- FEAT_FAMINMAX
"fcma"
- FEAT_FCMA
"fhm"
- FEAT_FHM
"flagm"
- FEAT_FLAGM
"flagm2"
- FEAT_FLAGM2
"fp"
- FEAT_FP
"fp16"
- FEAT_FP16
"fp8"
- FEAT_FP8
"fp8dot2"
- FEAT_FP8DOT2
"fp8dot4"
- FEAT_FP8DOT4
"fp8fma"
- FEAT_FP8FMA
"fpmr"
- FEAT_FPMR
"frintts"
- FEAT_FRINTTS
"hbc"
- FEAT_HBC
"i8mm"
- FEAT_I8MM
"jsconv"
- FEAT_JSCVT
"lse"
- FEAT_LSE
"lse128"
- FEAT_LSE128
"lse2"
- FEAT_LSE2
"lut"
- FEAT_LUT
"mops"
- FEAT_MOPS
"mte"
- FEAT_MTE & FEAT_MTE2
"paca"
- FEAT_PAuth (address authentication)
"pacg"
- FEAT_Pauth (generic authentication)
"pauth-lr"
- FEAT_PAuth_LR
"pmull"
- FEAT_PMULL
"rand"
- FEAT_RNG
"rcpc"
- FEAT_LRCPC
"rcpc2"
- FEAT_LRCPC2
"rcpc3"
- FEAT_LRCPC3
"rdm"
- FEAT_RDM
"sb"
- FEAT_SB
"sha2"
- FEAT_SHA1 & FEAT_SHA256
"sha3"
- FEAT_SHA512 & FEAT_SHA3
"sm4"
- FEAT_SM3 & FEAT_SM4
"sme"
- FEAT_SME
"sme-b16b16"
- FEAT_SME_B16B16
"sme-f16f16"
- FEAT_SME_F16F16
"sme-f64f64"
- FEAT_SME_F64F64
"sme-f8f16"
- FEAT_SME_F8F16
"sme-f8f32"
- FEAT_SME_F8F32
"sme-fa64"
- FEAT_SME_FA64
"sme-i16i64"
- FEAT_SME_I16I64
"sme-lutv2"
- FEAT_SME_LUTv2
"sme2"
- FEAT_SME2
"sme2p1"
- FEAT_SME2p1
"ssbs"
- FEAT_SSBS & FEAT_SSBS2
"ssve-fp8dot2"
- FEAT_SSVE_FP8DOT2
"ssve-fp8dot4"
- FEAT_SSVE_FP8DOT4
"ssve-fp8fma"
- FEAT_SSVE_FP8FMA
"sve"
- FEAT_SVE
"sve-b16b16"
- FEAT_SVE_B16B16 (SVE or SME Z-targeting instructions)
"sve2"
- FEAT_SVE2
"sve2-aes"
- FEAT_SVE_AES & FEAT_SVE_PMULL128 (SVE2 AES crypto)
"sve2-bitperm"
- FEAT_SVE2_BitPerm
"sve2-sha3"
- FEAT_SVE2_SHA3
"sve2-sm4"
- FEAT_SVE2_SM4
"sve2p1"
- FEAT_SVE2p1
"tme"
- FEAT_TME
"wfxt"
- FEAT_WFxT