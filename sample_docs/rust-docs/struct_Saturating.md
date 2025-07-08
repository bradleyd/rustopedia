Saturating in std::num - Rust
std
::
num
Struct
Saturating
Copy item path
1.74.0
·
Source
#[repr(transparent)]
pub struct Saturating<T>(pub T);
Expand description
Provides intentionally-saturating arithmetic on
T
.
Operations like
+
on
u32
values are intended to never overflow,
and in some debug configurations overflow is detected and results
in a panic. While most arithmetic falls into this category, some
code explicitly expects and relies upon saturating arithmetic.
Saturating arithmetic can be achieved either through methods like
saturating_add
, or through the
Saturating<T>
type, which says that
all standard arithmetic operations on the underlying value are
intended to have saturating semantics.
The underlying value can be retrieved through the
.0
index of the
Saturating
tuple.
§
Examples
use
std::num::Saturating;
let
max = Saturating(u32::MAX);
let
one = Saturating(
1u32
);
assert_eq!
(u32::MAX, (max + one).
0
);
Tuple Fields
§
§
0: T
Implementations
§
Source
§
impl
Saturating
<
usize
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
usize
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<usize>>::MIN, Saturating(usize::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
usize
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<usize>>::MAX, Saturating(usize::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 64u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<usize>>::BITS, usize::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100usize
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0usize
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000usize
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
usize
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
usize
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
usize
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
usize
>) ->
Saturating
<
usize
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ausize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<usize>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<usize>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
usize
>) ->
Saturating
<
usize
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ausize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<usize>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<usize>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
usize
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
usize
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
usize
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3usize
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
u8
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
u8
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u8>>::MIN, Saturating(u8::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
u8
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u8>>::MAX, Saturating(u8::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 8u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u8>>::BITS, u8::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100u8
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0u8
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000u8
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
u8
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
u8
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
u8
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
u8
>) ->
Saturating
<
u8
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<u8>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<u8>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
u8
>) ->
Saturating
<
u8
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<u8>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<u8>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
u8
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
u8
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
u8
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3u8
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
u16
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
u16
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u16>>::MIN, Saturating(u16::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
u16
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u16>>::MAX, Saturating(u16::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 16u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u16>>::BITS, u16::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100u16
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0u16
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000u16
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
u16
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
u16
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
u16
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
u16
>) ->
Saturating
<
u16
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<u16>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<u16>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
u16
>) ->
Saturating
<
u16
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<u16>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<u16>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
u16
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
u16
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
u16
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3u16
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
u32
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
u32
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u32>>::MIN, Saturating(u32::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
u32
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u32>>::MAX, Saturating(u32::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 32u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u32>>::BITS, u32::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100u32
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0u32
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000u32
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
u32
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
u32
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
u32
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
u32
>) ->
Saturating
<
u32
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<u32>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<u32>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
u32
>) ->
Saturating
<
u32
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<u32>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<u32>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
u32
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
u32
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
u32
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3u32
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
u64
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
u64
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u64>>::MIN, Saturating(u64::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
u64
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u64>>::MAX, Saturating(u64::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 64u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u64>>::BITS, u64::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100u64
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0u64
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000u64
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
u64
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
u64
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
u64
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
u64
>) ->
Saturating
<
u64
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<u64>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<u64>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
u64
>) ->
Saturating
<
u64
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<u64>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<u64>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
u64
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
u64
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
u64
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3u64
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
u128
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
u128
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u128>>::MIN, Saturating(u128::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
u128
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u128>>::MAX, Saturating(u128::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 128u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<u128>>::BITS, u128::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100u128
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0u128
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000u128
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
u128
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
u128
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
u128
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
u128
>) ->
Saturating
<
u128
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<u128>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<u128>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
u128
>) ->
Saturating
<
u128
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Au128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<u128>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<u128>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
u128
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
u128
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
u128
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3u128
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
isize
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
isize
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<isize>>::MIN, Saturating(isize::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
isize
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<isize>>::MAX, Saturating(isize::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 64u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<isize>>::BITS, isize::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100isize
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0isize
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000isize
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
isize
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
isize
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
isize
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
isize
>) ->
Saturating
<
isize
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Aisize
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<isize>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<isize>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
isize
>) ->
Saturating
<
isize
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Aisize
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<isize>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<isize>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
isize
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
isize
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
isize
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3isize
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
i8
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
i8
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i8>>::MIN, Saturating(i8::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
i8
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i8>>::MAX, Saturating(i8::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 8u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i8>>::BITS, i8::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100i8
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0i8
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000i8
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
i8
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
i8
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
i8
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
i8
>) ->
Saturating
<
i8
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai8
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<i8>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<i8>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
i8
>) ->
Saturating
<
i8
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai8
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<i8>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<i8>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
i8
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
i8
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
i8
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
i16
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
i16
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i16>>::MIN, Saturating(i16::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
i16
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i16>>::MAX, Saturating(i16::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 16u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i16>>::BITS, i16::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100i16
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0i16
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000i16
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
i16
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
i16
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
i16
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
i16
>) ->
Saturating
<
i16
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai16
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<i16>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<i16>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
i16
>) ->
Saturating
<
i16
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai16
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<i16>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<i16>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
i16
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
i16
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
i16
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i16
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
i32
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
i32
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i32>>::MIN, Saturating(i32::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
i32
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i32>>::MAX, Saturating(i32::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 32u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i32>>::BITS, i32::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100i32
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0i32
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000i32
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
i32
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
i32
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
i32
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
i32
>) ->
Saturating
<
i32
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai32
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<i32>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<i32>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
i32
>) ->
Saturating
<
i32
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai32
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<i32>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<i32>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
i32
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
i32
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
i32
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i32
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
i64
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
i64
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i64>>::MIN, Saturating(i64::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
i64
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i64>>::MAX, Saturating(i64::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 64u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i64>>::BITS, i64::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100i64
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0i64
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000i64
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
i64
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
i64
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
i64
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
i64
>) ->
Saturating
<
i64
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai64
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<i64>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<i64>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
i64
>) ->
Saturating
<
i64
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai64
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<i64>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<i64>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
i64
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
i64
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
i64
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i64
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
i128
>
1.74.0
·
Source
pub const
MIN
:
Saturating
<
i128
>
Returns the smallest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i128>>::MIN, Saturating(i128::MIN));
1.74.0
·
Source
pub const
MAX
:
Saturating
<
i128
>
Returns the largest value that can be represented by this integer type.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i128>>::MAX, Saturating(i128::MAX));
1.74.0
·
Source
pub const
BITS
:
u32
= 128u32
Returns the size of this integer type in bits.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(<Saturating<i128>>::BITS, i128::BITS);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_ones
(self) ->
u32
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b01001100i128
);
assert_eq!
(n.count_ones(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
count_zeros
(self) ->
u32
Returns the number of zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(!
0i128
).count_zeros(),
0
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0b0101000i128
);
assert_eq!
(n.trailing_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
Saturating
<
i128
>
Shifts the bits to the left by a specified amount,
n
,
saturating the truncated bits to the end of the resulting
integer.
Please note this isn’t the same operation as the
<<
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0x76543210FEDCBA99
);
assert_eq!
(n.rotate_left(
32
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
Saturating
<
i128
>
Shifts the bits to the right by a specified amount,
n
,
saturating the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting
operator!
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i64> = Saturating(
0x0123456789ABCDEF
);
let
m: Saturating<i64> = Saturating(-
0xFEDCBA987654322
);
assert_eq!
(n.rotate_right(
4
), m);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
swap_bytes
(self) ->
Saturating
<
i128
>
Reverses the byte order of the integer.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n: Saturating<i16> = Saturating(
0b0000000_01010101
);
assert_eq!
(n, Saturating(
85
));
let
m = n.swap_bytes();
assert_eq!
(m, Saturating(
0b01010101_00000000
));
assert_eq!
(m, Saturating(
21760
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
reverse_bits
(self) ->
Saturating
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
std::num::Saturating;
let
n = Saturating(
0b0000000_01010101i16
);
assert_eq!
(n, Saturating(
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
(m, Saturating(-
22016
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_be
(x:
Saturating
<
i128
>) ->
Saturating
<
i128
>
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai128
);
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(<Saturating<i128>>::from_be(n), n)
}
else
{
assert_eq!
(<Saturating<i128>>::from_be(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
from_le
(x:
Saturating
<
i128
>) ->
Saturating
<
i128
>
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
0x1Ai128
);
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(<Saturating<i128>>::from_le(n), n)
}
else
{
assert_eq!
(<Saturating<i128>>::from_le(n), n.swap_bytes())
}
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_be
(self) ->
Saturating
<
i128
>
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
to_le
(self) ->
Saturating
<
i128
>
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(
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
1.74.0 (const: 1.74.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
Saturating
<
i128
>
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i128
).pow(
4
), Saturating(
81
));
Results that are too large are saturated:
use
std::num::Saturating;
assert_eq!
(Saturating(
3i8
).pow(
5
), Saturating(
127
));
assert_eq!
(Saturating(
3i8
).pow(
6
), Saturating(
127
));
Source
§
impl
Saturating
<
isize
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(isize::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
isize
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100isize
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100isize
).abs(), Saturating(
100
));
assert_eq!
(Saturating(isize::MIN).abs(), Saturating((isize::MIN +
1
).abs()));
assert_eq!
(Saturating(isize::MIN).abs(), Saturating(isize::MIN.saturating_abs()));
assert_eq!
(Saturating(isize::MIN).abs(), Saturating(isize::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
isize
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10isize
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0isize
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10isize
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10isize
).is_positive());
assert!
(!Saturating(-
10isize
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10isize
).is_negative());
assert!
(!Saturating(
10isize
).is_negative());
Source
§
impl
Saturating
<
i8
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(i8::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
i8
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100i8
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100i8
).abs(), Saturating(
100
));
assert_eq!
(Saturating(i8::MIN).abs(), Saturating((i8::MIN +
1
).abs()));
assert_eq!
(Saturating(i8::MIN).abs(), Saturating(i8::MIN.saturating_abs()));
assert_eq!
(Saturating(i8::MIN).abs(), Saturating(i8::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
i8
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10i8
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0i8
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10i8
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10i8
).is_positive());
assert!
(!Saturating(-
10i8
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10i8
).is_negative());
assert!
(!Saturating(
10i8
).is_negative());
Source
§
impl
Saturating
<
i16
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(i16::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
i16
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100i16
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100i16
).abs(), Saturating(
100
));
assert_eq!
(Saturating(i16::MIN).abs(), Saturating((i16::MIN +
1
).abs()));
assert_eq!
(Saturating(i16::MIN).abs(), Saturating(i16::MIN.saturating_abs()));
assert_eq!
(Saturating(i16::MIN).abs(), Saturating(i16::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
i16
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10i16
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0i16
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10i16
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10i16
).is_positive());
assert!
(!Saturating(-
10i16
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10i16
).is_negative());
assert!
(!Saturating(
10i16
).is_negative());
Source
§
impl
Saturating
<
i32
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(i32::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
i32
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100i32
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100i32
).abs(), Saturating(
100
));
assert_eq!
(Saturating(i32::MIN).abs(), Saturating((i32::MIN +
1
).abs()));
assert_eq!
(Saturating(i32::MIN).abs(), Saturating(i32::MIN.saturating_abs()));
assert_eq!
(Saturating(i32::MIN).abs(), Saturating(i32::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
i32
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10i32
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0i32
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10i32
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10i32
).is_positive());
assert!
(!Saturating(-
10i32
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10i32
).is_negative());
assert!
(!Saturating(
10i32
).is_negative());
Source
§
impl
Saturating
<
i64
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(i64::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
i64
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100i64
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100i64
).abs(), Saturating(
100
));
assert_eq!
(Saturating(i64::MIN).abs(), Saturating((i64::MIN +
1
).abs()));
assert_eq!
(Saturating(i64::MIN).abs(), Saturating(i64::MIN.saturating_abs()));
assert_eq!
(Saturating(i64::MIN).abs(), Saturating(i64::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
i64
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10i64
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0i64
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10i64
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10i64
).is_positive());
assert!
(!Saturating(-
10i64
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10i64
).is_negative());
assert!
(!Saturating(
10i64
).is_negative());
Source
§
impl
Saturating
<
i128
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(i128::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
3
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
abs
(self) ->
Saturating
<
i128
>
Saturating absolute value. Computes
self.abs()
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
100i128
).abs(), Saturating(
100
));
assert_eq!
(Saturating(-
100i128
).abs(), Saturating(
100
));
assert_eq!
(Saturating(i128::MIN).abs(), Saturating((i128::MIN +
1
).abs()));
assert_eq!
(Saturating(i128::MIN).abs(), Saturating(i128::MIN.saturating_abs()));
assert_eq!
(Saturating(i128::MIN).abs(), Saturating(i128::MAX));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
signum
(self) ->
Saturating
<
i128
>
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
use
std::num::Saturating;
assert_eq!
(Saturating(
10i128
).signum(), Saturating(
1
));
assert_eq!
(Saturating(
0i128
).signum(), Saturating(
0
));
assert_eq!
(Saturating(-
10i128
).signum(), Saturating(-
1
));
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_positive
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
10i128
).is_positive());
assert!
(!Saturating(-
10i128
).is_positive());
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_negative
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(-
10i128
).is_negative());
assert!
(!Saturating(
10i128
).is_negative());
Source
§
impl
Saturating
<
usize
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(usize::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16usize
).is_power_of_two());
assert!
(!Saturating(
10usize
).is_power_of_two());
Source
§
impl
Saturating
<
u8
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(u8::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16u8
).is_power_of_two());
assert!
(!Saturating(
10u8
).is_power_of_two());
Source
§
impl
Saturating
<
u16
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(u16::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16u16
).is_power_of_two());
assert!
(!Saturating(
10u16
).is_power_of_two());
Source
§
impl
Saturating
<
u32
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(u32::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16u32
).is_power_of_two());
assert!
(!Saturating(
10u32
).is_power_of_two());
Source
§
impl
Saturating
<
u64
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(u64::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16u64
).is_power_of_two());
assert!
(!Saturating(
10u64
).is_power_of_two());
Source
§
impl
Saturating
<
u128
>
1.74.0 (const: 1.74.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
§
Examples
Basic usage:
use
std::num::Saturating;
let
n = Saturating(u128::MAX >>
2
);
assert_eq!
(n.leading_zeros(),
2
);
1.74.0 (const: 1.74.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
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
use
std::num::Saturating;
assert!
(Saturating(
16u128
).is_power_of_two());
assert!
(!Saturating(
10u128
).is_power_of_two());
Trait Implementations
§
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Add
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Add
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
+
operation.
Read more
1.74.0
·
Source
§
impl
Add
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
add_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
i128
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
i16
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
i32
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
i64
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
i8
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
isize
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
u128
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
u16
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
u32
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
u64
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
u8
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl
AddAssign
for
Saturating
<
usize
>
Source
§
fn
add_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
+=
operation.
Read more
1.74.0
·
Source
§
impl<T>
Binary
for
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
i128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
i128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
u128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
u128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other:
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitAnd
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other:
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
&
operation.
Read more
1.74.0
·
Source
§
impl
BitAnd
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
i128
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
i16
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
i32
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
i64
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
i8
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
isize
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
u128
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
u16
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
u32
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
u64
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
u8
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitAndAssign
for
Saturating
<
usize
>
Source
§
fn
bitand_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
&=
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitOr
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
|
operation.
Read more
1.74.0
·
Source
§
impl
BitOr
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
i128
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
i16
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
i32
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
i64
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
i8
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
isize
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
u128
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
u16
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
u32
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
u64
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
u8
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitOrAssign
for
Saturating
<
usize
>
Source
§
fn
bitor_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
|=
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
i128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
i128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
u128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
u128
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other: &
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other:
Saturating
<
isize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
BitXor
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
(
    self,
    other:
Saturating
<
usize
>,
) -> <
Saturating
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
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
^
operation.
Read more
1.74.0
·
Source
§
impl
BitXor
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
i128
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
i16
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
i32
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
i64
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
i8
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
isize
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
u128
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
u16
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
u32
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
u64
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
u8
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl
BitXorAssign
for
Saturating
<
usize
>
Source
§
fn
bitxor_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
^=
operation.
Read more
1.74.0
·
Source
§
impl<T>
Clone
for
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl<T>
Debug
for
Saturating
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
1.74.0
·
Source
§
impl<T>
Default
for
Saturating
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
Saturating
<T>
Returns the “default value” for a type.
Read more
1.74.0
·
Source
§
impl<T>
Display
for
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Div
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i128
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i128
), Saturating(
5i128
) / Saturating(
2
));
assert_eq!
(Saturating(i128::MAX), Saturating(i128::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i128::MIN), Saturating(i128::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i128
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i16
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i16
), Saturating(
5i16
) / Saturating(
2
));
assert_eq!
(Saturating(i16::MAX), Saturating(i16::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i16::MIN), Saturating(i16::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i16
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i32
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i32
), Saturating(
5i32
) / Saturating(
2
));
assert_eq!
(Saturating(i32::MAX), Saturating(i32::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i32::MIN), Saturating(i32::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i32
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i64
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i64
), Saturating(
5i64
) / Saturating(
2
));
assert_eq!
(Saturating(i64::MAX), Saturating(i64::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i64::MIN), Saturating(i64::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i64
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
i8
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2i8
), Saturating(
5i8
) / Saturating(
2
));
assert_eq!
(Saturating(i8::MAX), Saturating(i8::MAX) / Saturating(
1
));
assert_eq!
(Saturating(i8::MIN), Saturating(i8::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0i8
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
isize
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2isize
), Saturating(
5isize
) / Saturating(
2
));
assert_eq!
(Saturating(isize::MAX), Saturating(isize::MAX) / Saturating(
1
));
assert_eq!
(Saturating(isize::MIN), Saturating(isize::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0isize
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u128
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u128
), Saturating(
5u128
) / Saturating(
2
));
assert_eq!
(Saturating(u128::MAX), Saturating(u128::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u128::MIN), Saturating(u128::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u128
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u16
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u16
), Saturating(
5u16
) / Saturating(
2
));
assert_eq!
(Saturating(u16::MAX), Saturating(u16::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u16::MIN), Saturating(u16::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u16
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u32
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u32
), Saturating(
5u32
) / Saturating(
2
));
assert_eq!
(Saturating(u32::MAX), Saturating(u32::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u32::MIN), Saturating(u32::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u32
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u64
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u64
), Saturating(
5u64
) / Saturating(
2
));
assert_eq!
(Saturating(u64::MAX), Saturating(u64::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u64::MIN), Saturating(u64::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u64
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
u8
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2u8
), Saturating(
5u8
) / Saturating(
2
));
assert_eq!
(Saturating(u8::MAX), Saturating(u8::MAX) / Saturating(
1
));
assert_eq!
(Saturating(u8::MIN), Saturating(u8::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0u8
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
/
operation.
Read more
1.74.0
·
Source
§
impl
Div
for
Saturating
<
usize
>
§
Examples
Basic usage:
use
std::num::Saturating;
assert_eq!
(Saturating(
2usize
), Saturating(
5usize
) / Saturating(
2
));
assert_eq!
(Saturating(usize::MAX), Saturating(usize::MAX) / Saturating(
1
));
assert_eq!
(Saturating(usize::MIN), Saturating(usize::MIN) / Saturating(
1
));
ⓘ
use
std::num::Saturating;
let _
= Saturating(
0usize
) / Saturating(
0
);
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
div_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
i128
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
i16
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
i32
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
i64
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
i8
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
isize
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
u128
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
u16
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
u32
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
u64
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
u8
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl
DivAssign
for
Saturating
<
usize
>
Source
§
fn
div_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
/=
operation.
Read more
1.74.0
·
Source
§
impl<T>
Hash
for
Saturating
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
1.74.0
·
Source
§
impl<T>
LowerHex
for
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Mul
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
*
operation.
Read more
1.74.0
·
Source
§
impl
Mul
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
mul_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
i128
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
i16
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
i32
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
i64
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
i8
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
isize
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
u128
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
u16
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
u32
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
u64
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
u8
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
MulAssign
for
Saturating
<
usize
>
Source
§
fn
mul_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
*=
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Neg
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>
Performs the unary
-
operation.
Read more
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Not
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl
Not
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>
Performs the unary
!
operation.
Read more
1.74.0
·
Source
§
impl<T>
Octal
for
Saturating
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
1.74.0
·
Source
§
impl<T>
Ord
for
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl<T>
PartialEq
for
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl<T>
PartialOrd
for
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Rem
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
%
operation.
Read more
1.74.0
·
Source
§
impl
Rem
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
rem_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
i128
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
i16
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
i32
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
i64
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
i8
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
isize
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
u128
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
u16
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
u32
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
u64
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
u8
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
RemAssign
for
Saturating
<
usize
>
Source
§
fn
rem_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
%=
operation.
Read more
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i128
>> for &
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i16
>> for &
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i32
>> for &
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i64
>> for &
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i8
>> for &
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
isize
>> for &
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u128
>> for &
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u16
>> for &
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u32
>> for &
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u64
>> for &
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u8
>> for &
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
usize
>> for &
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
<&
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
i128
>> for &'a
Saturating
<
i128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
i16
>> for &'a
Saturating
<
i16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
i32
>> for &'a
Saturating
<
i32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
i64
>> for &'a
Saturating
<
i64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
i8
>> for &'a
Saturating
<
i8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
i8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
isize
>> for &'a
Saturating
<
isize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
isize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
u128
>> for &'a
Saturating
<
u128
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u128
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
u16
>> for &'a
Saturating
<
u16
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u16
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
u32
>> for &'a
Saturating
<
u32
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u32
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
u64
>> for &'a
Saturating
<
u64
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u64
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
u8
>> for &'a
Saturating
<
u8
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
u8
>) -> <
Saturating
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
1.74.0
·
Source
§
impl<'a>
Sub
<
Saturating
<
usize
>> for &'a
Saturating
<
usize
>
Source
§
type
Output
= <
Saturating
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
Saturating
<
usize
>) -> <
Saturating
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
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
i128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i128
>) ->
Saturating
<
i128
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
i16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i16
>) ->
Saturating
<
i16
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
i32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i32
>) ->
Saturating
<
i32
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
i64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i64
>) ->
Saturating
<
i64
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
i8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
i8
>) ->
Saturating
<
i8
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
isize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
isize
>) ->
Saturating
<
isize
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
u128
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u128
>) ->
Saturating
<
u128
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
u16
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u16
>) ->
Saturating
<
u16
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
u32
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u32
>) ->
Saturating
<
u32
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
u64
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u64
>) ->
Saturating
<
u64
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
u8
>
Source
§
type
Output
=
Saturating
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
Saturating
<
u8
>) ->
Saturating
<
u8
>
Performs the
-
operation.
Read more
1.74.0
·
Source
§
impl
Sub
for
Saturating
<
usize
>
Source
§
type
Output
=
Saturating
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
Saturating
<
usize
>) ->
Saturating
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
Saturating
<
i128
>> for
Saturating
<
i128
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
i16
>> for
Saturating
<
i16
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
i32
>> for
Saturating
<
i32
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
i64
>> for
Saturating
<
i64
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
i8
>> for
Saturating
<
i8
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
isize
>> for
Saturating
<
isize
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
u128
>> for
Saturating
<
u128
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
u16
>> for
Saturating
<
u16
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
u32
>> for
Saturating
<
u32
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
u64
>> for
Saturating
<
u64
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
u8
>> for
Saturating
<
u8
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
<
usize
>> for
Saturating
<
usize
>
Source
§
fn
sub_assign
(&mut self, other: &
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
i128
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
i16
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
i32
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
i64
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
i8
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
isize
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
u128
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
u16
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
u32
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
u64
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
u8
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
<
usize
> for
Saturating
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
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
i128
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
i128
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
i16
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
i16
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
i32
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
i32
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
i64
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
i64
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
i8
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
i8
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
isize
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
isize
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
u128
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
u128
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
u16
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
u16
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
u32
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
u32
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
u64
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
u64
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
u8
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
u8
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl
SubAssign
for
Saturating
<
usize
>
Source
§
fn
sub_assign
(&mut self, other:
Saturating
<
usize
>)
Performs the
-=
operation.
Read more
1.74.0
·
Source
§
impl<T>
UpperHex
for
Saturating
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
1.74.0
·
Source
§
impl<T>
Copy
for
Saturating
<T>
where
    T:
Copy
,
1.74.0
·
Source
§
impl<T>
Eq
for
Saturating
<T>
where
    T:
Eq
,
1.74.0
·
Source
§
impl<T>
StructuralPartialEq
for
Saturating
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Saturating
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Saturating
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Saturating
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Saturating
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Saturating
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Saturating
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