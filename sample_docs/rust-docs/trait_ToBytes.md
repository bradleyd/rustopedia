ToBytes in std::simd - Rust
std
::
simd
Trait
ToBytes
Copy item path
Source
pub trait ToBytes: Sealed {
    type
Bytes
:
Copy
+
Unpin
+
Send
+
Sync
+
AsRef
<[
u8
]> +
AsMut
<[
u8
]> +
SimdUint
<Scalar =
u8
> + 'static;

    // Required methods
    fn
to_ne_bytes
(self) -> Self::
Bytes
;
fn
to_be_bytes
(self) -> Self::
Bytes
;
fn
to_le_bytes
(self) -> Self::
Bytes
;
fn
from_ne_bytes
(bytes: Self::
Bytes
) -> Self;
fn
from_be_bytes
(bytes: Self::
Bytes
) -> Self;
fn
from_le_bytes
(bytes: Self::
Bytes
) -> Self;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Converts SIMD vectors to vectors of bytes
Required Associated Types
§
Source
type
Bytes
:
Copy
+
Unpin
+
Send
+
Sync
+
AsRef
<[
u8
]> +
AsMut
<[
u8
]> +
SimdUint
<Scalar =
u8
> + 'static
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Required Methods
§
Source
fn
to_ne_bytes
(self) -> Self::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
fn
to_be_bytes
(self) -> Self::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
fn
to_le_bytes
(self) -> Self::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
fn
from_ne_bytes
(bytes: Self::
Bytes
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
fn
from_be_bytes
(bytes: Self::
Bytes
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
fn
from_le_bytes
(bytes: Self::
Bytes
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
ToBytes
for
Simd
<
f32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#52}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#53}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#54}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#55}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#56}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#57}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#58}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#59}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
f64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#60}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#26}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#27}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#28}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#29}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#30}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#31}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i8
, 64>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#32}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#33}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#34}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#35}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#36}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#37}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i16
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#38}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#39}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#40}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#41}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#42}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#43}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#44}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#45}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#46}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
i64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#47}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
isize
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#48}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
isize
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#49}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
isize
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#50}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
isize
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#51}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#0}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#1}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#2}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#3}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#4}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#5}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u8
, 64>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#6}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#7}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#8}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#9}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#10}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#11}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u16
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#12}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#13}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#14}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#15}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#16}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#17}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#18}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#19}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#20}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
u64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#21}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
usize
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#22}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
usize
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#23}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
usize
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#24}::Bytes::{constant#0}>
Source
§
impl
ToBytes
for
Simd
<
usize
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#25}::Bytes::{constant#0}>