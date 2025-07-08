is_riscv_feature_detected in std::arch - Rust
std
::
arch
Macro
is_riscv_feature_detected
Copy item path
1.76.0
·
Source
macro_rules! is_riscv_feature_detected {
    ("rv32i") => { ... };
    ("zifencei") => { ... };
    ("zihintpause") => { ... };
    ("rv64i") => { ... };
    ("m") => { ... };
    ("a") => { ... };
    ("zicsr") => { ... };
    ("zicntr") => { ... };
    ("zihpm") => { ... };
    ("f") => { ... };
    ("d") => { ... };
    ("q") => { ... };
    ("c") => { ... };
    ("zfinx") => { ... };
    ("zdinx") => { ... };
    ("zhinx") => { ... };
    ("zhinxmin") => { ... };
    ("ztso") => { ... };
    ("rv32e") => { ... };
    ("rv128i") => { ... };
    ("zfh") => { ... };
    ("zfhmin") => { ... };
    ("j") => { ... };
    ("p") => { ... };
    ("v") => { ... };
    ("zam") => { ... };
    ("s") => { ... };
    ("svnapot") => { ... };
    ("svpbmt") => { ... };
    ("svinval") => { ... };
    ("h") => { ... };
    ("zba") => { ... };
    ("zbb") => { ... };
    ("zbc") => { ... };
    ("zbs") => { ... };
    ("zbkb") => { ... };
    ("zbkc") => { ... };
    ("zbkx") => { ... };
    ("zknd") => { ... };
    ("zkne") => { ... };
    ("zknh") => { ... };
    ("zksed") => { ... };
    ("zksh") => { ... };
    ("zkr") => { ... };
    ("zkn") => { ... };
    ("zks") => { ... };
    ("zk") => { ... };
    ("zkt") => { ... };
    ($t:tt,) => { ... };
    ($t:tt) => { ... };
}
Expand description
A macro to test at
runtime
whether instruction sets are available on
RISC-V platforms.
RISC-V standard defined the base sets and the extension sets.
The base sets are RV32I, RV64I, RV32E or RV128I. Any RISC-V platform
must support one base set and/or multiple extension sets.
Any RISC-V standard instruction sets can be in state of either ratified,
frozen or draft. The version and status of current standard instruction
sets can be checked out from preface section of the
ISA manual
.
Platform may define and support their own custom instruction sets with
ISA prefix X. These sets are highly platform specific and should be
detected with their own platform support crates.
§
Unprivileged Specification
The supported ratified RISC-V instruction sets are as follows:
RV32E:
"rv32e"
RV32I:
"rv32i"
RV64I:
"rv64i"
A:
"a"
B:
"b"
Zba:
"zba"
Zbb:
"zbb"
Zbc:
"zbc"
Zbs:
"zbs"
C:
"c"
D:
"d"
F:
"f"
M:
"m"
Q:
"q"
V:
"v"
Zicntr:
"zicntr"
Zicsr:
"zicsr"
Zifencei:
"zifencei"
Zihintpause:
"zihintpause"
Zihpm:
"zihpm"
Zk:
"zk"
Zbkb:
"zbkb"
Zbkc:
"zbkc"
Zbkx:
"zbkx"
Zkn:
"zkn"
Zknd:
"zknd"
Zkne:
"zkne"
Zknh:
"zknh"
Zkr:
"zkr"
Zks:
"zks"
Zksed:
"zksed"
Zksh:
"zksh"
Zkt:
"zkt"
There’s also bases and extensions marked as standard instruction set,
but they are in frozen or draft state. These instruction sets are also
reserved by this macro and can be detected in the future platforms.
Frozen RISC-V instruction sets:
Zfh:
"zfh"
Zfhmin:
"zfhmin"
Zfinx:
"zfinx"
Zdinx:
"zdinx"
Zhinx:
"zhinx"
Zhinxmin:
"zhinxmin"
Ztso:
"ztso"
Draft RISC-V instruction sets:
RV128I:
"rv128i"
J:
"j"
P:
"p"
Zam:
"zam"
Defined by Privileged Specification:
Supervisor:
"s"
Svnapot:
"svnapot"
Svpbmt:
"svpbmt"
Svinval:
"svinval"
Hypervisor:
"h"