Wrapping in std::num - Rust
std
::
num
Struct
Wrapping
Copy item path
1.0.0
·
Source
#[repr(transparent)]
pub struct Wrapping<T>(pub T);
Expand description
Provides intentionally-wrapped arithmetic on
T
.
Operations like
+
on
u32
values are intended to never overflow,
and in some debug configurations overflow is detected and results
in a panic. While most arithmetic falls into this category, some
code explicitly expects and relies upon modular arithmetic (e.g.,
hashing).
Wrapping arithmetic can be achieved either through methods like
wrapping_add
, or through the
Wrapping<T>
type, which says that
all standard arithmetic operations on the underlying value are
intended to have wrapping semantics.
The underlying value can be retrieved through the
.0
index of the
Wrapping
tuple.
§
Examples
use
std::num::Wrapping;
let
zero = Wrapping(
0u32
);
let
one = Wrapping(
1u32
);
assert_eq!
(u32::MAX, (zero - one).
0
);
§
Layout
Wrapping<T>
is guaranteed to have the same layout and ABI as
T
.
Tuple Fields
§
§
0: T
Implementations
§
Source
§
impl
Wrapping
<
usize
>
Source
pub const
MIN
:
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<usize>>::MIN, Wrapping(usize::MIN));
Source
pub const
MAX
:
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<usize>>::MAX, Wrapping(usize::MAX));
Source
pub const
BITS
:
u32
= 64u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<usize>>::BITS, usize::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100usize
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0usize
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000usize
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
usize
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ausize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<usize>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<usize>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ausize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<usize>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<usize>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ausize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ausize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3usize
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
u8
>
Source
pub const
MIN
:
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u8>>::MIN, Wrapping(u8::MIN));
Source
pub const
MAX
:
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u8>>::MAX, Wrapping(u8::MAX));
Source
pub const
BITS
:
u32
= 8u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u8>>::BITS, u8::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100u8
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0u8
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000u8
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
u8
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<u8>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<u8>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<u8>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<u8>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3u8
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
u16
>
Source
pub const
MIN
:
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u16>>::MIN, Wrapping(u16::MIN));
Source
pub const
MAX
:
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u16>>::MAX, Wrapping(u16::MAX));
Source
pub const
BITS
:
u32
= 16u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u16>>::BITS, u16::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100u16
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0u16
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000u16
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
u16
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<u16>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<u16>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<u16>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<u16>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3u16
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
u32
>
Source
pub const
MIN
:
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u32>>::MIN, Wrapping(u32::MIN));
Source
pub const
MAX
:
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u32>>::MAX, Wrapping(u32::MAX));
Source
pub const
BITS
:
u32
= 32u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u32>>::BITS, u32::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100u32
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0u32
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000u32
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
u32
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<u32>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<u32>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<u32>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<u32>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3u32
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
u64
>
Source
pub const
MIN
:
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u64>>::MIN, Wrapping(u64::MIN));
Source
pub const
MAX
:
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u64>>::MAX, Wrapping(u64::MAX));
Source
pub const
BITS
:
u32
= 64u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u64>>::BITS, u64::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100u64
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0u64
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000u64
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
u64
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<u64>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<u64>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<u64>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<u64>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3u64
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
u128
>
Source
pub const
MIN
:
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u128>>::MIN, Wrapping(u128::MIN));
Source
pub const
MAX
:
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u128>>::MAX, Wrapping(u128::MAX));
Source
pub const
BITS
:
u32
= 128u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<u128>>::BITS, u128::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100u128
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0u128
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000u128
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
u128
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<u128>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<u128>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<u128>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<u128>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Au128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3u128
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
isize
>
Source
pub const
MIN
:
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<isize>>::MIN, Wrapping(isize::MIN));
Source
pub const
MAX
:
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<isize>>::MAX, Wrapping(isize::MAX));
Source
pub const
BITS
:
u32
= 64u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<isize>>::BITS, isize::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100isize
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0isize
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000isize
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
isize
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Aisize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<isize>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<isize>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Aisize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<isize>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<isize>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Aisize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Aisize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3isize
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
i8
>
Source
pub const
MIN
:
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i8>>::MIN, Wrapping(i8::MIN));
Source
pub const
MAX
:
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i8>>::MAX, Wrapping(i8::MAX));
Source
pub const
BITS
:
u32
= 8u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i8>>::BITS, i8::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100i8
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0i8
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000i8
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
i8
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<i8>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<i8>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<i8>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<i8>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
i16
>
Source
pub const
MIN
:
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i16>>::MIN, Wrapping(i16::MIN));
Source
pub const
MAX
:
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i16>>::MAX, Wrapping(i16::MAX));
Source
pub const
BITS
:
u32
= 16u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i16>>::BITS, i16::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100i16
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0i16
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000i16
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
i16
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<i16>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<i16>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<i16>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<i16>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i16
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
i32
>
Source
pub const
MIN
:
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i32>>::MIN, Wrapping(i32::MIN));
Source
pub const
MAX
:
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i32>>::MAX, Wrapping(i32::MAX));
Source
pub const
BITS
:
u32
= 32u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i32>>::BITS, i32::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100i32
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0i32
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000i32
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
i32
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<i32>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<i32>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<i32>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<i32>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i32
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
i64
>
Source
pub const
MIN
:
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i64>>::MIN, Wrapping(i64::MIN));
Source
pub const
MAX
:
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i64>>::MAX, Wrapping(i64::MAX));
Source
pub const
BITS
:
u32
= 64u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i64>>::BITS, i64::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100i64
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0i64
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000i64
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
i64
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<i64>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<i64>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<i64>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<i64>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i64
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
i128
>
Source
pub const
MIN
:
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i128>>::MIN, Wrapping(i128::MIN));
Source
pub const
MAX
:
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i128>>::MAX, Wrapping(i128::MAX));
Source
pub const
BITS
:
u32
= 128u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the size of this integer type in bits.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(<Wrapping<i128>>::BITS, i128::BITS);
Source
pub const fn
count_ones
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b01001100i128
);
assert_eq!
(n.count_ones(),
3
);
Source
pub const fn
count_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(!
0i128
).count_zeros(),
0
);
Source
pub const fn
trailing_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0b0101000i128
);
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i64> = Wrapping(
0x0123456789ABCDEF
);
let
m: Wrapping<i64> = Wrapping(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n: Wrapping<i16> = Wrapping(
0b0000000_01010101
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Wrapping(
0b01010101_00000000
));
assert_eq!
(m, Wrapping(
21760
));
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
Wrapping
<
i128
>
Reverses the bit pattern of the integer.
§
Examples
Please note that this example is shared between integer types.
Which explains why
i16
is used here.
Basic usage:
use
std::num::Wrapping;
let
n = Wrapping(
0b0000000_01010101i16
);
assert_eq!
(n, Wrapping(
85
));
let
m = n.reverse_bits();
assert_eq!
(m.
0
as
u16,
0b10101010_00000000
);
assert_eq!
(m, Wrapping(-
22016
));
Source
pub const fn
from_be
(x:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Wrapping<i128>>::from_be(n), n)
}
else
{
assert_eq!
(<Wrapping<i128>>::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Wrapping<i128>>::from_le(n), n)
}
else
{
assert_eq!
(<Wrapping<i128>>::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(n.to_be(), n)
}
else
{
assert_eq!
(n.to_be(), n.swap_bytes())
}
Source
pub const fn
to_le
(self) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(
0x1Ai128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(n.to_le(), n)
}
else
{
assert_eq!
(n.to_le(), n.swap_bytes())
}
Source
pub fn
pow
(self, exp:
u32
) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i128
).pow(
4
), Wrapping(
81
));
Results that are too large are wrapped:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
3i8
).pow(
5
), Wrapping(-
13
));
assert_eq!
(Wrapping(
3i8
).pow(
6
), Wrapping(-
39
));
Source
§
impl
Wrapping
<
isize
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(isize::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100isize
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100isize
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(isize::MIN).abs(), Wrapping(isize::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
isize
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10isize
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0isize
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10isize
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10isize
).is_positive());
assert!
(!Wrapping(-
10isize
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10isize
).is_negative());
assert!
(!Wrapping(
10isize
).is_negative());
Source
§
impl
Wrapping
<
i8
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(i8::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100i8
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100i8
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(i8::MIN).abs(), Wrapping(i8::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
i8
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10i8
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0i8
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10i8
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10i8
).is_positive());
assert!
(!Wrapping(-
10i8
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10i8
).is_negative());
assert!
(!Wrapping(
10i8
).is_negative());
Source
§
impl
Wrapping
<
i16
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(i16::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100i16
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100i16
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(i16::MIN).abs(), Wrapping(i16::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
i16
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10i16
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0i16
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10i16
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10i16
).is_positive());
assert!
(!Wrapping(-
10i16
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10i16
).is_negative());
assert!
(!Wrapping(
10i16
).is_negative());
Source
§
impl
Wrapping
<
i32
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(i32::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100i32
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100i32
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(i32::MIN).abs(), Wrapping(i32::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
i32
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10i32
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0i32
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10i32
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10i32
).is_positive());
assert!
(!Wrapping(-
10i32
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10i32
).is_negative());
assert!
(!Wrapping(
10i32
).is_negative());
Source
§
impl
Wrapping
<
i64
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(i64::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100i64
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100i64
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(i64::MIN).abs(), Wrapping(i64::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
i64
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10i64
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0i64
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10i64
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10i64
).is_positive());
assert!
(!Wrapping(-
10i64
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10i64
).is_negative());
assert!
(!Wrapping(
10i64
).is_negative());
Source
§
impl
Wrapping
<
i128
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(i128::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
3
);
Source
pub fn
abs
(self) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Computes the absolute value of
self
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
100i128
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(-
100i128
).abs(), Wrapping(
100
));
assert_eq!
(Wrapping(i128::MIN).abs(), Wrapping(i128::MIN));
assert_eq!
(Wrapping(-
128i8
).abs().
0
as
u8,
128u8
);
Source
pub fn
signum
(self) ->
Wrapping
<
i128
>
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns a number representing sign of
self
.
0
if the number is zero
1
if the number is positive
-1
if the number is negative
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
10i128
).signum(), Wrapping(
1
));
assert_eq!
(Wrapping(
0i128
).signum(), Wrapping(
0
));
assert_eq!
(Wrapping(-
10i128
).signum(), Wrapping(-
1
));
Source
pub const fn
is_positive
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is positive and
false
if the number is zero or
negative.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
10i128
).is_positive());
assert!
(!Wrapping(-
10i128
).is_positive());
Source
pub const fn
is_negative
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if
self
is negative and
false
if the number is zero or
positive.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(-
10i128
).is_negative());
assert!
(!Wrapping(
10i128
).is_negative());
Source
§
impl
Wrapping
<
usize
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(usize::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16usize
).is_power_of_two());
assert!
(!Wrapping(
10usize
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
usize
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2usize
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3usize
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Source
§
impl
Wrapping
<
u8
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(u8::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16u8
).is_power_of_two());
assert!
(!Wrapping(
10u8
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
u8
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2u8
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3u8
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Source
§
impl
Wrapping
<
u16
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(u16::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16u16
).is_power_of_two());
assert!
(!Wrapping(
10u16
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
u16
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2u16
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3u16
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Source
§
impl
Wrapping
<
u32
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(u32::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16u32
).is_power_of_two());
assert!
(!Wrapping(
10u32
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
u32
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2u32
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3u32
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Source
§
impl
Wrapping
<
u64
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(u64::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16u64
).is_power_of_two());
assert!
(!Wrapping(
10u64
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
u64
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2u64
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3u64
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Source
§
impl
Wrapping
<
u128
>
Source
pub const fn
leading_zeros
(self) ->
u32
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
let
n = Wrapping(u128::MAX) >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
Source
pub fn
is_power_of_two
(self) ->
bool
🔬
This is a nightly-only experimental API. (
wrapping_int_impl
#32463
)
Returns
true
if and only if
self == 2^k
for some
k
.
§
Examples
Basic usage:
#![feature(wrapping_int_impl)]
use
std::num::Wrapping;
assert!
(Wrapping(
16u128
).is_power_of_two());
assert!
(!Wrapping(
10u128
).is_power_of_two());
Source
pub fn
next_power_of_two
(self) ->
Wrapping
<
u128
>
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), overflows to
2^N = 0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
use
std::num::Wrapping;
assert_eq!
(Wrapping(
2u128
).next_power_of_two(), Wrapping(
2
));
assert_eq!
(Wrapping(
3u128
).next_power_of_two(), Wrapping(
4
));
assert_eq!
(Wrapping(
200_u8
).next_power_of_two(), Wrapping(
0
));
Trait Implementations
§
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl
Add
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Add
>::
Output
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
+
operation.
Read more
1.0.0
·
Source
§
impl
Add
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
+
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
add_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
add_assign
(&mut self, other: &
i128
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
add_assign
(&mut self, other: &
i16
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
add_assign
(&mut self, other: &
i32
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
add_assign
(&mut self, other: &
i64
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
add_assign
(&mut self, other: &
i8
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
add_assign
(&mut self, other: &
isize
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
add_assign
(&mut self, other: &
u128
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
add_assign
(&mut self, other: &
u16
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
add_assign
(&mut self, other: &
u32
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
add_assign
(&mut self, other: &
u64
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
add_assign
(&mut self, other: &
u8
)
Performs the
+=
operation.
Read more
1.22.0
·
Source
§
impl
AddAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
add_assign
(&mut self, other: &
usize
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
add_assign
(&mut self, other:
i128
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
add_assign
(&mut self, other:
i16
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
add_assign
(&mut self, other:
i32
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
add_assign
(&mut self, other:
i64
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
add_assign
(&mut self, other:
i8
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
add_assign
(&mut self, other:
isize
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
add_assign
(&mut self, other:
u128
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
add_assign
(&mut self, other:
u16
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
add_assign
(&mut self, other:
u32
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
add_assign
(&mut self, other:
u64
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
add_assign
(&mut self, other:
u8
)
Performs the
+=
operation.
Read more
1.60.0
·
Source
§
impl
AddAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
add_assign
(&mut self, other:
usize
)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
i128
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
i16
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
i32
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
i64
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
i8
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
isize
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
u128
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
u16
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
u32
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
u64
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
u8
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AddAssign
for
Wrapping
<
usize
>
Source
§
fn
add_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
+=
operation.
Read more
1.11.0
·
Source
§
impl<T>
Binary
for
Wrapping
<T>
where
    T:
Binary
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl
BitAnd
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitAnd
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitAnd
>::
Output
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
&
operation.
Read more
1.0.0
·
Source
§
impl
BitAnd
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
&
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other: &
i128
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other: &
i16
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other: &
i32
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other: &
i64
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other: &
i8
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other: &
isize
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other: &
u128
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other: &
u16
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other: &
u32
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other: &
u64
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other: &
u8
)
Performs the
&=
operation.
Read more
1.22.0
·
Source
§
impl
BitAndAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other: &
usize
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other:
i128
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other:
i16
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other:
i32
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other:
i64
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other:
i8
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other:
isize
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other:
u128
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other:
u16
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other:
u32
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other:
u64
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other:
u8
)
Performs the
&=
operation.
Read more
1.60.0
·
Source
§
impl
BitAndAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other:
usize
)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
&=
operation.
Read more
1.8.0
·
Source
§
impl
BitAndAssign
for
Wrapping
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
&=
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl
BitOr
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitOr
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitOr
>::
Output
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
|
operation.
Read more
1.0.0
·
Source
§
impl
BitOr
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
|
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other: &
i128
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other: &
i16
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other: &
i32
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other: &
i64
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other: &
i8
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other: &
isize
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other: &
u128
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other: &
u16
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other: &
u32
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other: &
u64
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other: &
u8
)
Performs the
|=
operation.
Read more
1.22.0
·
Source
§
impl
BitOrAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other: &
usize
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other:
i128
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other:
i16
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other:
i32
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other:
i64
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other:
i8
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other:
isize
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other:
u128
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other:
u16
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other:
u32
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other:
u64
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other:
u8
)
Performs the
|=
operation.
Read more
1.60.0
·
Source
§
impl
BitOrAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other:
usize
)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
|=
operation.
Read more
1.8.0
·
Source
§
impl
BitOrAssign
for
Wrapping
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
|=
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl
BitXor
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.14.0
·
Source
§
impl<'a>
BitXor
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
BitXor
>::
Output
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
BitXor
>::
Output
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
^
operation.
Read more
1.0.0
·
Source
§
impl
BitXor
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
^
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
i128
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
i16
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
i32
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
i64
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
i8
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
isize
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
u128
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
u16
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
u32
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
u64
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
u8
)
Performs the
^=
operation.
Read more
1.22.0
·
Source
§
impl
BitXorAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
usize
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other:
i128
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other:
i16
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other:
i32
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other:
i64
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other:
i8
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other:
isize
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other:
u128
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other:
u16
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other:
u32
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other:
u64
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other:
u8
)
Performs the
^=
operation.
Read more
1.60.0
·
Source
§
impl
BitXorAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other:
usize
)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
^=
operation.
Read more
1.8.0
·
Source
§
impl
BitXorAssign
for
Wrapping
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
^=
operation.
Read more
1.0.0
·
Source
§
impl<T>
Clone
for
Wrapping
<T>
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
Wrapping
<T>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<T>
Debug
for
Wrapping
<T>
where
    T:
Debug
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl<T>
Default
for
Wrapping
<T>
where
    T:
Default
,
Source
§
fn
default
() ->
Wrapping
<T>
Returns the “default value” for a type.
Read more
1.10.0
·
Source
§
impl<T>
Display
for
Wrapping
<T>
where
    T:
Display
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl
Div
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Div
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Div
>::
Output
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Div
>::
Output
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
/
operation.
Read more
1.3.0
·
Source
§
impl
Div
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
/
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
div_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
div_assign
(&mut self, other: &
i128
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
div_assign
(&mut self, other: &
i16
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
div_assign
(&mut self, other: &
i32
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
div_assign
(&mut self, other: &
i64
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
div_assign
(&mut self, other: &
i8
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
div_assign
(&mut self, other: &
isize
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
div_assign
(&mut self, other: &
u128
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
div_assign
(&mut self, other: &
u16
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
div_assign
(&mut self, other: &
u32
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
div_assign
(&mut self, other: &
u64
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
div_assign
(&mut self, other: &
u8
)
Performs the
/=
operation.
Read more
1.22.0
·
Source
§
impl
DivAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
div_assign
(&mut self, other: &
usize
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
div_assign
(&mut self, other:
i128
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
div_assign
(&mut self, other:
i16
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
div_assign
(&mut self, other:
i32
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
div_assign
(&mut self, other:
i64
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
div_assign
(&mut self, other:
i8
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
div_assign
(&mut self, other:
isize
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
div_assign
(&mut self, other:
u128
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
div_assign
(&mut self, other:
u16
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
div_assign
(&mut self, other:
u32
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
div_assign
(&mut self, other:
u64
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
div_assign
(&mut self, other:
u8
)
Performs the
/=
operation.
Read more
1.60.0
·
Source
§
impl
DivAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
div_assign
(&mut self, other:
usize
)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
i128
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
i16
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
i32
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
i64
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
i8
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
isize
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
u128
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
u16
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
u32
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
u64
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
u8
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
/=
operation.
Read more
1.8.0
·
Source
§
impl
DivAssign
for
Wrapping
<
usize
>
Source
§
fn
div_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
/=
operation.
Read more
1.0.0
·
Source
§
impl<T>
Hash
for
Wrapping
<T>
where
    T:
Hash
,
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.11.0
·
Source
§
impl<T>
LowerHex
for
Wrapping
<T>
where
    T:
LowerHex
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl
Mul
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Mul
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Mul
>::
Output
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Mul
>::
Output
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
*
operation.
Read more
1.0.0
·
Source
§
impl
Mul
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
*
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
mul_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
mul_assign
(&mut self, other: &
i128
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
mul_assign
(&mut self, other: &
i16
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
mul_assign
(&mut self, other: &
i32
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
mul_assign
(&mut self, other: &
i64
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
mul_assign
(&mut self, other: &
i8
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
mul_assign
(&mut self, other: &
isize
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
mul_assign
(&mut self, other: &
u128
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
mul_assign
(&mut self, other: &
u16
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
mul_assign
(&mut self, other: &
u32
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
mul_assign
(&mut self, other: &
u64
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
mul_assign
(&mut self, other: &
u8
)
Performs the
*=
operation.
Read more
1.22.0
·
Source
§
impl
MulAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
mul_assign
(&mut self, other: &
usize
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
mul_assign
(&mut self, other:
i128
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
mul_assign
(&mut self, other:
i16
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
mul_assign
(&mut self, other:
i32
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
mul_assign
(&mut self, other:
i64
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
mul_assign
(&mut self, other:
i8
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
mul_assign
(&mut self, other:
isize
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
mul_assign
(&mut self, other:
u128
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
mul_assign
(&mut self, other:
u16
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
mul_assign
(&mut self, other:
u32
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
mul_assign
(&mut self, other:
u64
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
mul_assign
(&mut self, other:
u8
)
Performs the
*=
operation.
Read more
1.60.0
·
Source
§
impl
MulAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
mul_assign
(&mut self, other:
usize
)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
i128
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
i16
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
i32
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
i64
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
i8
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
isize
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
u128
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
u16
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
u32
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
u64
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
u8
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
*=
operation.
Read more
1.8.0
·
Source
§
impl
MulAssign
for
Wrapping
<
usize
>
Source
§
fn
mul_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
*=
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
i128
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
i16
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
i32
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
i64
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
i8
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
isize
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
u128
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
u16
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
u32
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
u64
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
u8
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Neg
for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Neg
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Wrapping
<
usize
> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
i128
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
i16
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
i32
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
i64
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
i8
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
isize
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
u128
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
u16
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
u32
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
u64
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
u8
>
Performs the unary
-
operation.
Read more
1.10.0
·
Source
§
impl
Neg
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
Wrapping
<
usize
>
Performs the unary
-
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
i128
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
i16
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
i32
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
i64
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
i8
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
isize
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
u128
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
u16
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
u32
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
u64
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
u8
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.14.0
·
Source
§
impl
Not
for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Not
>::
Output
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Wrapping
<
usize
> as
Not
>::
Output
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
i128
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
i16
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
i32
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
i64
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
i8
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
isize
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
u128
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
u16
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
u32
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
u64
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
u8
>
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Not
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
Wrapping
<
usize
>
Performs the unary
!
operation.
Read more
1.11.0
·
Source
§
impl<T>
Octal
for
Wrapping
<T>
where
    T:
Octal
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl<T>
Ord
for
Wrapping
<T>
where
    T:
Ord
,
Source
§
fn
cmp
(&self, other: &
Wrapping
<T>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.0.0
·
Source
§
impl<T>
PartialEq
for
Wrapping
<T>
where
    T:
PartialEq
,
Source
§
fn
eq
(&self, other: &
Wrapping
<T>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T>
PartialOrd
for
Wrapping
<T>
where
    T:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
Wrapping
<T>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i128
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i128
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i16
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i16
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i32
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i32
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i64
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i64
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i8
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i8
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
isize
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
isize
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u128
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u128
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u16
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u16
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u32
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u32
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u64
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u64
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u8
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u8
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl<'a>
Product
<&'a
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
usize
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
usize
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
i128
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i128
>
where
    I:
Iterator
<Item =
Wrapping
<
i128
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
i16
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i16
>
where
    I:
Iterator
<Item =
Wrapping
<
i16
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
i32
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i32
>
where
    I:
Iterator
<Item =
Wrapping
<
i32
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
i64
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i64
>
where
    I:
Iterator
<Item =
Wrapping
<
i64
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
i8
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
i8
>
where
    I:
Iterator
<Item =
Wrapping
<
i8
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
isize
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
isize
>
where
    I:
Iterator
<Item =
Wrapping
<
isize
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
u128
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u128
>
where
    I:
Iterator
<Item =
Wrapping
<
u128
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
u16
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u16
>
where
    I:
Iterator
<Item =
Wrapping
<
u16
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
u32
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u32
>
where
    I:
Iterator
<Item =
Wrapping
<
u32
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
u64
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u64
>
where
    I:
Iterator
<Item =
Wrapping
<
u64
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
u8
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
u8
>
where
    I:
Iterator
<Item =
Wrapping
<
u8
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Product
for
Wrapping
<
usize
>
Source
§
fn
product
<I>(iter: I) ->
Wrapping
<
usize
>
where
    I:
Iterator
<Item =
Wrapping
<
usize
>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl
Rem
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Rem
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Rem
>::
Output
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Rem
>::
Output
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
%
operation.
Read more
1.7.0
·
Source
§
impl
Rem
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
%
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
rem_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
rem_assign
(&mut self, other: &
i128
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
rem_assign
(&mut self, other: &
i16
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
rem_assign
(&mut self, other: &
i32
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
rem_assign
(&mut self, other: &
i64
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
rem_assign
(&mut self, other: &
i8
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
rem_assign
(&mut self, other: &
isize
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
rem_assign
(&mut self, other: &
u128
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
rem_assign
(&mut self, other: &
u16
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
rem_assign
(&mut self, other: &
u32
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
rem_assign
(&mut self, other: &
u64
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
rem_assign
(&mut self, other: &
u8
)
Performs the
%=
operation.
Read more
1.22.0
·
Source
§
impl
RemAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
rem_assign
(&mut self, other: &
usize
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
rem_assign
(&mut self, other:
i128
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
rem_assign
(&mut self, other:
i16
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
rem_assign
(&mut self, other:
i32
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
rem_assign
(&mut self, other:
i64
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
rem_assign
(&mut self, other:
i8
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
rem_assign
(&mut self, other:
isize
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
rem_assign
(&mut self, other:
u128
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
rem_assign
(&mut self, other:
u16
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
rem_assign
(&mut self, other:
u32
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
rem_assign
(&mut self, other:
u64
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
rem_assign
(&mut self, other:
u8
)
Performs the
%=
operation.
Read more
1.60.0
·
Source
§
impl
RemAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
rem_assign
(&mut self, other:
usize
)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
i128
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
i16
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
i32
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
i64
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
i8
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
isize
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
u128
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
u16
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
u32
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
u64
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
u8
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
%=
operation.
Read more
1.8.0
·
Source
§
impl
RemAssign
for
Wrapping
<
usize
>
Source
§
fn
rem_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
%=
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl
Shl
<&
usize
> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
usize
) -> <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
i128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
i16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
i32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
i64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
i8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
isize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
u128
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
u16
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
u32
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
u64
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
u8
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) -> <
Wrapping
<
usize
> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
i128
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
i16
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
i32
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
i64
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
i8
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
isize
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
u128
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
u16
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
u32
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
u64
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
u8
>
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl
Shl
<
usize
> for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
usize
) ->
Wrapping
<
usize
>
Performs the
<<
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
i128
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
i16
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
i32
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
i64
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
i8
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
isize
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
u128
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
u16
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
u32
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
u64
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
u8
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
shl_assign
(&mut self, other: &
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
i128
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
i16
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
i32
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
i64
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
i8
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
isize
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
u128
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
u16
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
u32
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
u64
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
u8
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.8.0
·
Source
§
impl
ShlAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
shl_assign
(&mut self, other:
usize
)
Performs the
<<=
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl
Shr
<&
usize
> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
usize
) -> <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
i128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
i16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
i32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
i64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
i8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
isize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
u128
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
u16
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
u32
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
u64
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
u8
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.39.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) -> <
Wrapping
<
usize
> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
i128
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
i16
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
i32
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
i64
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
i8
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
isize
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
u128
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
u16
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
u32
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
u64
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
u8
>
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<
usize
> for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
usize
) ->
Wrapping
<
usize
>
Performs the
>>
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i128
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i16
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i32
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i64
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i8
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
isize
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u128
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u16
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u32
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u64
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u8
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
shr_assign
(&mut self, other: &
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i128
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i16
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i32
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i64
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i8
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
isize
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u128
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u16
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u32
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u64
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u8
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
shr_assign
(&mut self, other:
usize
)
Performs the
>>=
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i128
>> for &
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i16
>> for &
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i32
>> for &
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i64
>> for &
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i8
>> for &
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
isize
>> for &
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u128
>> for &
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u16
>> for &
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u32
>> for &
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u64
>> for &
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u8
>> for &
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
usize
>> for &
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl
Sub
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other: &
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
i128
>> for &'a
Wrapping
<
i128
>
Source
§
type
Output
= <
Wrapping
<
i128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i128
>) -> <
Wrapping
<
i128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
i16
>> for &'a
Wrapping
<
i16
>
Source
§
type
Output
= <
Wrapping
<
i16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i16
>) -> <
Wrapping
<
i16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
i32
>> for &'a
Wrapping
<
i32
>
Source
§
type
Output
= <
Wrapping
<
i32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i32
>) -> <
Wrapping
<
i32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
i64
>> for &'a
Wrapping
<
i64
>
Source
§
type
Output
= <
Wrapping
<
i64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i64
>) -> <
Wrapping
<
i64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
i8
>> for &'a
Wrapping
<
i8
>
Source
§
type
Output
= <
Wrapping
<
i8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i8
>) -> <
Wrapping
<
i8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
isize
>> for &'a
Wrapping
<
isize
>
Source
§
type
Output
= <
Wrapping
<
isize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
isize
>) -> <
Wrapping
<
isize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
u128
>> for &'a
Wrapping
<
u128
>
Source
§
type
Output
= <
Wrapping
<
u128
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u128
>) -> <
Wrapping
<
u128
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
u16
>> for &'a
Wrapping
<
u16
>
Source
§
type
Output
= <
Wrapping
<
u16
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u16
>) -> <
Wrapping
<
u16
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
u32
>> for &'a
Wrapping
<
u32
>
Source
§
type
Output
= <
Wrapping
<
u32
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u32
>) -> <
Wrapping
<
u32
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
u64
>> for &'a
Wrapping
<
u64
>
Source
§
type
Output
= <
Wrapping
<
u64
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u64
>) -> <
Wrapping
<
u64
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
u8
>> for &'a
Wrapping
<
u8
>
Source
§
type
Output
= <
Wrapping
<
u8
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u8
>) -> <
Wrapping
<
u8
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sub
<
Wrapping
<
usize
>> for &'a
Wrapping
<
usize
>
Source
§
type
Output
= <
Wrapping
<
usize
> as
Sub
>::
Output
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
usize
>) -> <
Wrapping
<
usize
> as
Sub
>::
Output
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
i128
>
Source
§
type
Output
=
Wrapping
<
i128
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i128
>) ->
Wrapping
<
i128
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
i16
>
Source
§
type
Output
=
Wrapping
<
i16
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i16
>) ->
Wrapping
<
i16
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
i32
>
Source
§
type
Output
=
Wrapping
<
i32
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i32
>) ->
Wrapping
<
i32
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
i64
>
Source
§
type
Output
=
Wrapping
<
i64
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i64
>) ->
Wrapping
<
i64
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
i8
>
Source
§
type
Output
=
Wrapping
<
i8
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
i8
>) ->
Wrapping
<
i8
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
isize
>
Source
§
type
Output
=
Wrapping
<
isize
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
isize
>) ->
Wrapping
<
isize
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
u128
>
Source
§
type
Output
=
Wrapping
<
u128
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u128
>) ->
Wrapping
<
u128
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
u16
>
Source
§
type
Output
=
Wrapping
<
u16
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u16
>) ->
Wrapping
<
u16
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
u32
>
Source
§
type
Output
=
Wrapping
<
u32
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u32
>) ->
Wrapping
<
u32
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
u64
>
Source
§
type
Output
=
Wrapping
<
u64
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u64
>) ->
Wrapping
<
u64
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
u8
>
Source
§
type
Output
=
Wrapping
<
u8
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
u8
>) ->
Wrapping
<
u8
>
Performs the
-
operation.
Read more
1.0.0
·
Source
§
impl
Sub
for
Wrapping
<
usize
>
Source
§
type
Output
=
Wrapping
<
usize
>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Wrapping
<
usize
>) ->
Wrapping
<
usize
>
Performs the
-
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
i128
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
i16
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
i32
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
i64
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
i8
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
isize
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
u128
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
u16
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
u32
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
u64
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
u8
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
sub_assign
(&mut self, other: &
Wrapping
<
usize
>)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
i128
> for
Wrapping
<
i128
>
Source
§
fn
sub_assign
(&mut self, other: &
i128
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
i16
> for
Wrapping
<
i16
>
Source
§
fn
sub_assign
(&mut self, other: &
i16
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
i32
> for
Wrapping
<
i32
>
Source
§
fn
sub_assign
(&mut self, other: &
i32
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
i64
> for
Wrapping
<
i64
>
Source
§
fn
sub_assign
(&mut self, other: &
i64
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
i8
> for
Wrapping
<
i8
>
Source
§
fn
sub_assign
(&mut self, other: &
i8
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
isize
> for
Wrapping
<
isize
>
Source
§
fn
sub_assign
(&mut self, other: &
isize
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
u128
> for
Wrapping
<
u128
>
Source
§
fn
sub_assign
(&mut self, other: &
u128
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
u16
> for
Wrapping
<
u16
>
Source
§
fn
sub_assign
(&mut self, other: &
u16
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
u32
> for
Wrapping
<
u32
>
Source
§
fn
sub_assign
(&mut self, other: &
u32
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
u64
> for
Wrapping
<
u64
>
Source
§
fn
sub_assign
(&mut self, other: &
u64
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
u8
> for
Wrapping
<
u8
>
Source
§
fn
sub_assign
(&mut self, other: &
u8
)
Performs the
-=
operation.
Read more
1.22.0
·
Source
§
impl
SubAssign
<&
usize
> for
Wrapping
<
usize
>
Source
§
fn
sub_assign
(&mut self, other: &
usize
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
i128
> for
Wrapping
<
i128
>
Source
§
fn
sub_assign
(&mut self, other:
i128
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
i16
> for
Wrapping
<
i16
>
Source
§
fn
sub_assign
(&mut self, other:
i16
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
i32
> for
Wrapping
<
i32
>
Source
§
fn
sub_assign
(&mut self, other:
i32
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
i64
> for
Wrapping
<
i64
>
Source
§
fn
sub_assign
(&mut self, other:
i64
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
i8
> for
Wrapping
<
i8
>
Source
§
fn
sub_assign
(&mut self, other:
i8
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
isize
> for
Wrapping
<
isize
>
Source
§
fn
sub_assign
(&mut self, other:
isize
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
u128
> for
Wrapping
<
u128
>
Source
§
fn
sub_assign
(&mut self, other:
u128
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
u16
> for
Wrapping
<
u16
>
Source
§
fn
sub_assign
(&mut self, other:
u16
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
u32
> for
Wrapping
<
u32
>
Source
§
fn
sub_assign
(&mut self, other:
u32
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
u64
> for
Wrapping
<
u64
>
Source
§
fn
sub_assign
(&mut self, other:
u64
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
u8
> for
Wrapping
<
u8
>
Source
§
fn
sub_assign
(&mut self, other:
u8
)
Performs the
-=
operation.
Read more
1.60.0
·
Source
§
impl
SubAssign
<
usize
> for
Wrapping
<
usize
>
Source
§
fn
sub_assign
(&mut self, other:
usize
)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
i128
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
i128
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
i16
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
i16
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
i32
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
i32
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
i64
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
i64
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
i8
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
i8
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
isize
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
isize
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
u128
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
u128
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
u16
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
u16
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
u32
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
u32
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
u64
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
u64
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
u8
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
u8
>)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
SubAssign
for
Wrapping
<
usize
>
Source
§
fn
sub_assign
(&mut self, other:
Wrapping
<
usize
>)
Performs the
-=
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
i128
>> for
Wrapping
<
i128
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i128
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i128
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
i16
>> for
Wrapping
<
i16
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i16
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i16
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
i32
>> for
Wrapping
<
i32
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i32
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i32
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
i64
>> for
Wrapping
<
i64
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i64
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i64
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
i8
>> for
Wrapping
<
i8
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i8
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
i8
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
isize
>> for
Wrapping
<
isize
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
isize
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
isize
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
u128
>> for
Wrapping
<
u128
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u128
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u128
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
u16
>> for
Wrapping
<
u16
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u16
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u16
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
u32
>> for
Wrapping
<
u32
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u32
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u32
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
u64
>> for
Wrapping
<
u64
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u64
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u64
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
u8
>> for
Wrapping
<
u8
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u8
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
u8
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl<'a>
Sum
<&'a
Wrapping
<
usize
>> for
Wrapping
<
usize
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
usize
>
where
    I:
Iterator
<Item = &'a
Wrapping
<
usize
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
i128
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i128
>
where
    I:
Iterator
<Item =
Wrapping
<
i128
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
i16
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i16
>
where
    I:
Iterator
<Item =
Wrapping
<
i16
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
i32
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i32
>
where
    I:
Iterator
<Item =
Wrapping
<
i32
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
i64
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i64
>
where
    I:
Iterator
<Item =
Wrapping
<
i64
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
i8
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
i8
>
where
    I:
Iterator
<Item =
Wrapping
<
i8
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
isize
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
isize
>
where
    I:
Iterator
<Item =
Wrapping
<
isize
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
u128
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u128
>
where
    I:
Iterator
<Item =
Wrapping
<
u128
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
u16
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u16
>
where
    I:
Iterator
<Item =
Wrapping
<
u16
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
u32
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u32
>
where
    I:
Iterator
<Item =
Wrapping
<
u32
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
u64
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u64
>
where
    I:
Iterator
<Item =
Wrapping
<
u64
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
u8
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
u8
>
where
    I:
Iterator
<Item =
Wrapping
<
u8
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.14.0
·
Source
§
impl
Sum
for
Wrapping
<
usize
>
Source
§
fn
sum
<I>(iter: I) ->
Wrapping
<
usize
>
where
    I:
Iterator
<Item =
Wrapping
<
usize
>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.11.0
·
Source
§
impl<T>
UpperHex
for
Wrapping
<T>
where
    T:
UpperHex
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl<T>
Copy
for
Wrapping
<T>
where
    T:
Copy
,
1.0.0
·
Source
§
impl<T>
Eq
for
Wrapping
<T>
where
    T:
Eq
,
1.0.0
·
Source
§
impl<T>
StructuralPartialEq
for
Wrapping
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Wrapping
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Wrapping
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Wrapping
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Wrapping
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Wrapping
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Wrapping
<T>
where
    T:
UnwindSafe
,
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.