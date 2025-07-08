simd_bitmask in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_bitmask
Copy item path
Source
pub unsafe fn simd_bitmask<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Truncates an integer vector to a bitmask.
T
must be an integer vector.
U
must be either the smallest unsigned integer with at least as many bits as the length
of
T
, or the smallest array of
u8
with at least as many bits as the length of
T
.
Each element is truncated to a single bit and packed into the result.
No matter whether the output is an array or an unsigned integer, it is treated as a single
contiguous list of bits. The bitmask is always packed on the least-significant side of the
output, and padded with 0s in the most-significant bits. The order of the bits depends on
endianness:
On little endian, the least significant bit corresponds to the first vector element.
On big endian, the least significant bit corresponds to the last vector element.
For example,
[!0, 0, !0, !0]
packs to
0b1101u8
or
[0b1101]
on little endian, and
0b1011u8
or
[0b1011]
on big endian.
To consider a larger example,
[!0, 0, 0, 0, 0, 0, 0, 0, !0, !0, 0, 0, 0, 0, !0, 0]
packs to
0b0100001100000001u16
or
[0b00000001, 0b01000011]
on little endian, and
0b1000000011000010u16
or
[0b10000000, 0b11000010]
on big endian.
And finally, a non-power-of-2 example with multiple bytes:
[!0, !0, 0, !0, 0, 0, !0, 0, !0, 0]
packs to
0b0101001011u16
or
[0b01001011, 0b01]
on little endian, and
0b1101001010u16
or
[0b11, 0b01001010]
on big endian.
§
Safety
x
must contain only
0
and
!0
.