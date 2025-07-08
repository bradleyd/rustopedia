NonZero in std::num - Rust
std
::
num
Struct
NonZero
Copy item path
1.79.0
·
Source
pub struct NonZero<T>(
/* private fields */
)
where
    T:
ZeroablePrimitive
;
Expand description
A value that is known not to equal zero.
This enables some memory layout optimization.
For example,
Option<NonZero<u32>>
is the same size as
u32
:
use
core::{num::NonZero};
assert_eq!
(size_of::<
Option
<NonZero<u32>>>(), size_of::<u32>());
§
Layout
NonZero<T>
is guaranteed to have the same layout and bit validity as
T
with the exception that the all-zero bit pattern is invalid.
Option<NonZero<T>>
is guaranteed to be compatible with
T
, including in
FFI.
Thanks to the
null pointer optimization
,
NonZero<T>
and
Option<NonZero<T>>
are guaranteed to have the same size and alignment:
use
std::num::NonZero;
assert_eq!
(size_of::<NonZero<u32>>(), size_of::<
Option
<NonZero<u32>>>());
assert_eq!
(align_of::<NonZero<u32>>(), align_of::<
Option
<NonZero<u32>>>());
Implementations
§
Source
§
impl<T>
NonZero
<T>
where
    T:
ZeroablePrimitive
,
1.28.0 (const: 1.47.0)
·
Source
pub const fn
new
(n: T) ->
Option
<
NonZero
<T>>
Creates a non-zero if the given value is not zero.
1.28.0 (const: 1.28.0)
·
Source
pub const unsafe fn
new_unchecked
(n: T) ->
NonZero
<T>
Creates a non-zero without checking whether the value is non-zero.
This results in undefined behavior if the value is zero.
§
Safety
The value must not be zero.
Source
pub fn
from_mut
(n:
&mut T
) ->
Option
<&mut
NonZero
<T>>
🔬
This is a nightly-only experimental API. (
nonzero_from_mut
#106290
)
Converts a reference to a non-zero mutable reference
if the referenced value is not zero.
Source
pub unsafe fn
from_mut_unchecked
(n:
&mut T
) -> &mut
NonZero
<T>
🔬
This is a nightly-only experimental API. (
nonzero_from_mut
#106290
)
Converts a mutable reference to a non-zero mutable reference
without checking whether the referenced value is non-zero.
This results in undefined behavior if the referenced value is zero.
§
Safety
The referenced value must not be zero.
1.28.0 (const: 1.34.0)
·
Source
pub const fn
get
(self) -> T
Returns the contained value as a primitive type.
Source
§
impl
NonZero
<
u8
>
1.67.0
·
Source
pub const
BITS
:
u32
= 8u32
The size of this non-zero integer type in bits.
This value is equal to
u8::BITS
.
§
Examples
assert_eq!
(NonZero::<u8>::BITS, u8::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
u8
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<u8>::MIN.get(),
1u8
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
u8
>
The largest value that can be represented by this non-zero
integer type,
equal to
u8::MAX
.
§
Examples
assert_eq!
(NonZero::<u8>::MAX.get(), u8::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u8>::new(u8::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u8>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u8>::new(
0b_01100100
)
?
;
let
b = NonZero::<u8>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u8>::new(
0b_01100100
)
?
;
let
b = NonZero::<u8>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<u8>::new(
0b100_0000
)
?
;
let
b = NonZero::<u8>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x82u8
)
?
;
let
m = NonZero::new(
0xa
)
?
;
assert_eq!
(n.rotate_left(
2
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xau8
)
?
;
let
m = NonZero::new(
0x82
)
?
;
assert_eq!
(n.rotate_right(
2
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12u8
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x12
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12u8
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
u8
>) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU8;
let
n = NonZero::new(
0x1Au8
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroU8::from_be(n), n)
}
else
{
assert_eq!
(NonZeroU8::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
u8
>) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU8;
let
n = NonZero::new(
0x1Au8
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroU8::from_le(n), n)
}
else
{
assert_eq!
(NonZeroU8::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au8
)
?
;
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
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au8
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
u8
) ->
Option
<
NonZero
<
u8
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1u8
)
?
;
let
two = NonZero::new(
2u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
u8
) ->
NonZero
<
u8
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<u8>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1u8
)
?
;
let
two = NonZero::new(
2u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
u8
) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > u8::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1u8
)
?
;
let
two = NonZero::new(
2u8
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
u8
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u8
)
?
;
let
three = NonZero::new(
3u8
)
?
;
let
four = NonZero::new(
4u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
u8::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7u8
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8u8
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9u8
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
u8::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99u8
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100u8
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101u8
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
u8
>) ->
NonZero
<
u8
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1u8
)
?
;
let
two = NonZero::new(
2u8
)
?
;
let
four = NonZero::new(
4u8
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8u8
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10u8
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
u8
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10u8
)
?
;
let
three = NonZero::new(
3u8
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
i8
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<u8>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1i8
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
u8
>) ->
Option
<
NonZero
<
u8
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u8
)
?
;
let
four = NonZero::new(
4u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
u8
>) ->
NonZero
<
u8
>
Multiplies two non-zero integers together.
Return
NonZero::<u8>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2u8
)
?
;
let
four = NonZero::new(
4u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
u8
>) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > u8::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2u8
)
?
;
let
four = NonZero::new(
4u8
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
u8
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3u8
)
?
;
let
twenty_seven = NonZero::new(
27u8
)
?
;
let
half_max = NonZero::new(u8::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
u8
>
Raise non-zero value to an integer power.
Return
NonZero::<u8>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3u8
)
?
;
let
twenty_seven = NonZero::new(
27u8
)
?
;
let
max = NonZero::new(u8::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
u8
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
u8
>) ->
NonZero
<
u8
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1u8
).unwrap();
let
max = NonZero::new(u8::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2u8
).unwrap();
let
three = NonZero::new(
3u8
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
u16
>
1.67.0
·
Source
pub const
BITS
:
u32
= 16u32
The size of this non-zero integer type in bits.
This value is equal to
u16::BITS
.
§
Examples
assert_eq!
(NonZero::<u16>::BITS, u16::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
u16
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<u16>::MIN.get(),
1u16
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
u16
>
The largest value that can be represented by this non-zero
integer type,
equal to
u16::MAX
.
§
Examples
assert_eq!
(NonZero::<u16>::MAX.get(), u16::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u16>::new(u16::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u16>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u16>::new(
0b_01100100
)
?
;
let
b = NonZero::<u16>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u16>::new(
0b_01100100
)
?
;
let
b = NonZero::<u16>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<u16>::new(
0b100_0000
)
?
;
let
b = NonZero::<u16>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xa003u16
)
?
;
let
m = NonZero::new(
0x3a
)
?
;
assert_eq!
(n.rotate_left(
4
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x3au16
)
?
;
let
m = NonZero::new(
0xa003
)
?
;
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234u16
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x3412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234u16
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
u16
>) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU16;
let
n = NonZero::new(
0x1Au16
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroU16::from_be(n), n)
}
else
{
assert_eq!
(NonZeroU16::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
u16
>) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU16;
let
n = NonZero::new(
0x1Au16
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroU16::from_le(n), n)
}
else
{
assert_eq!
(NonZeroU16::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au16
)
?
;
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
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au16
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
u16
) ->
Option
<
NonZero
<
u16
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1u16
)
?
;
let
two = NonZero::new(
2u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
u16
) ->
NonZero
<
u16
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<u16>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1u16
)
?
;
let
two = NonZero::new(
2u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
u16
) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > u16::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1u16
)
?
;
let
two = NonZero::new(
2u16
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
u16
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u16
)
?
;
let
three = NonZero::new(
3u16
)
?
;
let
four = NonZero::new(
4u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
u16::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7u16
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8u16
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9u16
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
u16::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99u16
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100u16
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101u16
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
u16
>) ->
NonZero
<
u16
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1u16
)
?
;
let
two = NonZero::new(
2u16
)
?
;
let
four = NonZero::new(
4u16
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8u16
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10u16
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
u16
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10u16
)
?
;
let
three = NonZero::new(
3u16
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
i16
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<u16>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1i16
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
u16
>) ->
Option
<
NonZero
<
u16
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u16
)
?
;
let
four = NonZero::new(
4u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
u16
>) ->
NonZero
<
u16
>
Multiplies two non-zero integers together.
Return
NonZero::<u16>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2u16
)
?
;
let
four = NonZero::new(
4u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
u16
>) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > u16::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2u16
)
?
;
let
four = NonZero::new(
4u16
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
u16
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3u16
)
?
;
let
twenty_seven = NonZero::new(
27u16
)
?
;
let
half_max = NonZero::new(u16::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
u16
>
Raise non-zero value to an integer power.
Return
NonZero::<u16>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3u16
)
?
;
let
twenty_seven = NonZero::new(
27u16
)
?
;
let
max = NonZero::new(u16::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
u16
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
u16
>) ->
NonZero
<
u16
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1u16
).unwrap();
let
max = NonZero::new(u16::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2u16
).unwrap();
let
three = NonZero::new(
3u16
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
u32
>
1.67.0
·
Source
pub const
BITS
:
u32
= 32u32
The size of this non-zero integer type in bits.
This value is equal to
u32::BITS
.
§
Examples
assert_eq!
(NonZero::<u32>::BITS, u32::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
u32
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<u32>::MIN.get(),
1u32
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
u32
>
The largest value that can be represented by this non-zero
integer type,
equal to
u32::MAX
.
§
Examples
assert_eq!
(NonZero::<u32>::MAX.get(), u32::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u32>::new(u32::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u32>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u32>::new(
0b_01100100
)
?
;
let
b = NonZero::<u32>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u32>::new(
0b_01100100
)
?
;
let
b = NonZero::<u32>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<u32>::new(
0b100_0000
)
?
;
let
b = NonZero::<u32>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x10000b3u32
)
?
;
let
m = NonZero::new(
0xb301
)
?
;
assert_eq!
(n.rotate_left(
8
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xb301u32
)
?
;
let
m = NonZero::new(
0x10000b3
)
?
;
assert_eq!
(n.rotate_right(
8
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678u32
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x78563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678u32
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x1e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
u32
>) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU32;
let
n = NonZero::new(
0x1Au32
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroU32::from_be(n), n)
}
else
{
assert_eq!
(NonZeroU32::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
u32
>) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU32;
let
n = NonZero::new(
0x1Au32
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroU32::from_le(n), n)
}
else
{
assert_eq!
(NonZeroU32::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au32
)
?
;
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
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au32
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
u32
) ->
Option
<
NonZero
<
u32
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1u32
)
?
;
let
two = NonZero::new(
2u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
u32
) ->
NonZero
<
u32
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<u32>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1u32
)
?
;
let
two = NonZero::new(
2u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
u32
) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > u32::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1u32
)
?
;
let
two = NonZero::new(
2u32
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
u32
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u32
)
?
;
let
three = NonZero::new(
3u32
)
?
;
let
four = NonZero::new(
4u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
u32::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7u32
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8u32
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9u32
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
u32::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99u32
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100u32
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101u32
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
u32
>) ->
NonZero
<
u32
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1u32
)
?
;
let
two = NonZero::new(
2u32
)
?
;
let
four = NonZero::new(
4u32
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8u32
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10u32
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
u32
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10u32
)
?
;
let
three = NonZero::new(
3u32
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
i32
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<u32>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1i32
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
u32
>) ->
Option
<
NonZero
<
u32
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u32
)
?
;
let
four = NonZero::new(
4u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
u32
>) ->
NonZero
<
u32
>
Multiplies two non-zero integers together.
Return
NonZero::<u32>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2u32
)
?
;
let
four = NonZero::new(
4u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
u32
>) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > u32::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2u32
)
?
;
let
four = NonZero::new(
4u32
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
u32
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3u32
)
?
;
let
twenty_seven = NonZero::new(
27u32
)
?
;
let
half_max = NonZero::new(u32::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
u32
>
Raise non-zero value to an integer power.
Return
NonZero::<u32>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3u32
)
?
;
let
twenty_seven = NonZero::new(
27u32
)
?
;
let
max = NonZero::new(u32::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
u32
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
u32
>) ->
NonZero
<
u32
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1u32
).unwrap();
let
max = NonZero::new(u32::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2u32
).unwrap();
let
three = NonZero::new(
3u32
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
u64
>
1.67.0
·
Source
pub const
BITS
:
u32
= 64u32
The size of this non-zero integer type in bits.
This value is equal to
u64::BITS
.
§
Examples
assert_eq!
(NonZero::<u64>::BITS, u64::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
u64
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<u64>::MIN.get(),
1u64
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
u64
>
The largest value that can be represented by this non-zero
integer type,
equal to
u64::MAX
.
§
Examples
assert_eq!
(NonZero::<u64>::MAX.get(), u64::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u64>::new(u64::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u64>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u64>::new(
0b_01100100
)
?
;
let
b = NonZero::<u64>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u64>::new(
0b_01100100
)
?
;
let
b = NonZero::<u64>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<u64>::new(
0b100_0000
)
?
;
let
b = NonZero::<u64>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xaa00000000006e1u64
)
?
;
let
m = NonZero::new(
0x6e10aa
)
?
;
assert_eq!
(n.rotate_left(
12
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x6e10aau64
)
?
;
let
m = NonZero::new(
0xaa00000000006e1
)
?
;
assert_eq!
(n.rotate_right(
12
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456u64
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x5634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456u64
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
u64
>) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU64;
let
n = NonZero::new(
0x1Au64
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroU64::from_be(n), n)
}
else
{
assert_eq!
(NonZeroU64::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
u64
>) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU64;
let
n = NonZero::new(
0x1Au64
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroU64::from_le(n), n)
}
else
{
assert_eq!
(NonZeroU64::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au64
)
?
;
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
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au64
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
u64
) ->
Option
<
NonZero
<
u64
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1u64
)
?
;
let
two = NonZero::new(
2u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
u64
) ->
NonZero
<
u64
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<u64>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1u64
)
?
;
let
two = NonZero::new(
2u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
u64
) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > u64::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1u64
)
?
;
let
two = NonZero::new(
2u64
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
u64
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u64
)
?
;
let
three = NonZero::new(
3u64
)
?
;
let
four = NonZero::new(
4u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
u64::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7u64
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8u64
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9u64
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
u64::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99u64
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100u64
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101u64
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
u64
>) ->
NonZero
<
u64
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1u64
)
?
;
let
two = NonZero::new(
2u64
)
?
;
let
four = NonZero::new(
4u64
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8u64
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10u64
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
u64
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10u64
)
?
;
let
three = NonZero::new(
3u64
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
i64
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<u64>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1i64
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
u64
>) ->
Option
<
NonZero
<
u64
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u64
)
?
;
let
four = NonZero::new(
4u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
u64
>) ->
NonZero
<
u64
>
Multiplies two non-zero integers together.
Return
NonZero::<u64>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2u64
)
?
;
let
four = NonZero::new(
4u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
u64
>) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > u64::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2u64
)
?
;
let
four = NonZero::new(
4u64
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
u64
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3u64
)
?
;
let
twenty_seven = NonZero::new(
27u64
)
?
;
let
half_max = NonZero::new(u64::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
u64
>
Raise non-zero value to an integer power.
Return
NonZero::<u64>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3u64
)
?
;
let
twenty_seven = NonZero::new(
27u64
)
?
;
let
max = NonZero::new(u64::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
u64
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
u64
>) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1u64
).unwrap();
let
max = NonZero::new(u64::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2u64
).unwrap();
let
three = NonZero::new(
3u64
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
u128
>
1.67.0
·
Source
pub const
BITS
:
u32
= 128u32
The size of this non-zero integer type in bits.
This value is equal to
u128::BITS
.
§
Examples
assert_eq!
(NonZero::<u128>::BITS, u128::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
u128
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<u128>::MIN.get(),
1u128
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
u128
>
The largest value that can be represented by this non-zero
integer type,
equal to
u128::MAX
.
§
Examples
assert_eq!
(NonZero::<u128>::MAX.get(), u128::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u128>::new(u128::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<u128>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u128>::new(
0b_01100100
)
?
;
let
b = NonZero::<u128>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<u128>::new(
0b_01100100
)
?
;
let
b = NonZero::<u128>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<u128>::new(
0b100_0000
)
?
;
let
b = NonZero::<u128>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x13f40000000000000000000000004f76u128
)
?
;
let
m = NonZero::new(
0x4f7613f4
)
?
;
assert_eq!
(n.rotate_left(
16
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x4f7613f4u128
)
?
;
let
m = NonZero::new(
0x13f40000000000000000000000004f76
)
?
;
assert_eq!
(n.rotate_right(
16
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678901234567890123456789012u128
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x12907856341290785634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678901234567890123456789012u128
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x48091e6a2c48091e6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
u128
>) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU128;
let
n = NonZero::new(
0x1Au128
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroU128::from_be(n), n)
}
else
{
assert_eq!
(NonZeroU128::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
u128
>) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroU128;
let
n = NonZero::new(
0x1Au128
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroU128::from_le(n), n)
}
else
{
assert_eq!
(NonZeroU128::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au128
)
?
;
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
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Au128
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
u128
) ->
Option
<
NonZero
<
u128
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1u128
)
?
;
let
two = NonZero::new(
2u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
u128
) ->
NonZero
<
u128
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<u128>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1u128
)
?
;
let
two = NonZero::new(
2u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
u128
) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > u128::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1u128
)
?
;
let
two = NonZero::new(
2u128
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
u128
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u128
)
?
;
let
three = NonZero::new(
3u128
)
?
;
let
four = NonZero::new(
4u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
u128::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7u128
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8u128
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9u128
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
u128::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99u128
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100u128
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101u128
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
u128
>) ->
NonZero
<
u128
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1u128
)
?
;
let
two = NonZero::new(
2u128
)
?
;
let
four = NonZero::new(
4u128
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8u128
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10u128
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
u128
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10u128
)
?
;
let
three = NonZero::new(
3u128
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
i128
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<u128>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1i128
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
u128
>) ->
Option
<
NonZero
<
u128
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2u128
)
?
;
let
four = NonZero::new(
4u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
u128
>) ->
NonZero
<
u128
>
Multiplies two non-zero integers together.
Return
NonZero::<u128>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2u128
)
?
;
let
four = NonZero::new(
4u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
u128
>) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > u128::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2u128
)
?
;
let
four = NonZero::new(
4u128
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
u128
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3u128
)
?
;
let
twenty_seven = NonZero::new(
27u128
)
?
;
let
half_max = NonZero::new(u128::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
u128
>
Raise non-zero value to an integer power.
Return
NonZero::<u128>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3u128
)
?
;
let
twenty_seven = NonZero::new(
27u128
)
?
;
let
max = NonZero::new(u128::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
u128
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
u128
>) ->
NonZero
<
u128
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1u128
).unwrap();
let
max = NonZero::new(u128::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2u128
).unwrap();
let
three = NonZero::new(
3u128
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
usize
>
1.67.0
·
Source
pub const
BITS
:
u32
= 64u32
The size of this non-zero integer type in bits.
This value is equal to
usize::BITS
.
§
Examples
assert_eq!
(NonZero::<usize>::BITS, usize::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
usize
>
The smallest value that can be represented by this non-zero
integer type, 1.
§
Examples
assert_eq!
(NonZero::<usize>::MIN.get(),
1usize
);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
usize
>
The largest value that can be represented by this non-zero
integer type,
equal to
usize::MAX
.
§
Examples
assert_eq!
(NonZero::<usize>::MAX.get(), usize::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<usize>::new(usize::MAX)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<usize>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<usize>::new(
0b_01100100
)
?
;
let
b = NonZero::<usize>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<usize>::new(
0b_01100100
)
?
;
let
b = NonZero::<usize>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<usize>::new(
0b100_0000
)
?
;
let
b = NonZero::<usize>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xaa00000000006e1usize
)
?
;
let
m = NonZero::new(
0x6e10aa
)
?
;
assert_eq!
(n.rotate_left(
12
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x6e10aausize
)
?
;
let
m = NonZero::new(
0xaa00000000006e1
)
?
;
assert_eq!
(n.rotate_right(
12
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456usize
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x5634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456usize
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
usize
>) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroUsize;
let
n = NonZero::new(
0x1Ausize
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroUsize::from_be(n), n)
}
else
{
assert_eq!
(NonZeroUsize::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
usize
>) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroUsize;
let
n = NonZero::new(
0x1Ausize
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroUsize::from_le(n), n)
}
else
{
assert_eq!
(NonZeroUsize::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ausize
)
?
;
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
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ausize
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_add
(self, other:
usize
) ->
Option
<
NonZero
<
usize
>>
Adds an unsigned integer to a non-zero value.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
one = NonZero::new(
1usize
)
?
;
let
two = NonZero::new(
2usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(
Some
(two), one.checked_add(
1
));
assert_eq!
(
None
, max.checked_add(
1
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_add
(self, other:
usize
) ->
NonZero
<
usize
>
Adds an unsigned integer to a non-zero value.
Return
NonZero::<usize>::MAX
on overflow.
§
Examples
let
one = NonZero::new(
1usize
)
?
;
let
two = NonZero::new(
2usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(two, one.saturating_add(
1
));
assert_eq!
(max, max.saturating_add(
1
));
Source
pub const unsafe fn
unchecked_add
(self, other:
usize
) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Adds an unsigned integer to a non-zero value,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self + rhs > usize::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
one = NonZero::new(
1usize
)
?
;
let
two = NonZero::new(
2usize
)
?
;
assert_eq!
(two,
unsafe
{ one.unchecked_add(
1
) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
NonZero
<
usize
>>
Returns the smallest power of two greater than or equal to
self
.
Checks for overflow and returns
None
if the next power of two is greater than the type’s maximum value.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2usize
)
?
;
let
three = NonZero::new(
3usize
)
?
;
let
four = NonZero::new(
4usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(
Some
(two), two.checked_next_power_of_two() );
assert_eq!
(
Some
(four), three.checked_next_power_of_two() );
assert_eq!
(
None
, max.checked_next_power_of_two() );
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
This is the same operation as
usize::ilog2
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
7usize
)
?
.ilog2(),
2
);
assert_eq!
(NonZero::new(
8usize
)
?
.ilog2(),
3
);
assert_eq!
(NonZero::new(
9usize
)
?
.ilog2(),
3
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
This is the same operation as
usize::ilog10
,
except that it has no failure cases to worry about
since this value can never be zero.
§
Examples
assert_eq!
(NonZero::new(
99usize
)
?
.ilog10(),
1
);
assert_eq!
(NonZero::new(
100usize
)
?
.ilog10(),
2
);
assert_eq!
(NonZero::new(
101usize
)
?
.ilog10(),
2
);
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
NonZero
<
usize
>) ->
NonZero
<
usize
>
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) >> 1
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards negative infinity and that no overflow will ever occur.
§
Examples
let
one = NonZero::new(
1usize
)
?
;
let
two = NonZero::new(
2usize
)
?
;
let
four = NonZero::new(
4usize
)
?
;
assert_eq!
(one.midpoint(four), two);
assert_eq!
(four.midpoint(one), two);
1.59.0 (const: 1.59.0)
·
Source
pub const fn
is_power_of_two
(self) ->
bool
Returns
true
if and only if
self == (1 << k)
for some
k
.
On many architectures, this function can perform better than
is_power_of_two()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
eight = NonZero::new(
8usize
)
?
;
assert!
(eight.is_power_of_two());
let
ten = NonZero::new(
10usize
)
?
;
assert!
(!ten.is_power_of_two());
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
NonZero
<
usize
>
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
let
ten = NonZero::new(
10usize
)
?
;
let
three = NonZero::new(
3usize
)
?
;
assert_eq!
(ten.isqrt(), three);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
NonZero
<
isize
>
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::<usize>::MAX;
assert_eq!
(n.cast_signed(), NonZero::new(-
1isize
).unwrap());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
usize
>) ->
Option
<
NonZero
<
usize
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2usize
)
?
;
let
four = NonZero::new(
4usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
usize
>) ->
NonZero
<
usize
>
Multiplies two non-zero integers together.
Return
NonZero::<usize>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2usize
)
?
;
let
four = NonZero::new(
4usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
usize
>) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > usize::MAX
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2usize
)
?
;
let
four = NonZero::new(
4usize
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
usize
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3usize
)
?
;
let
twenty_seven = NonZero::new(
27usize
)
?
;
let
half_max = NonZero::new(usize::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
usize
>
Raise non-zero value to an integer power.
Return
NonZero::<usize>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3usize
)
?
;
let
twenty_seven = NonZero::new(
27usize
)
?
;
let
max = NonZero::new(usize::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
usize
>
Source
pub const fn
div_ceil
(self, rhs:
NonZero
<
usize
>) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
unsigned_nonzero_div_ceil
#132968
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
The result is guaranteed to be non-zero.
§
Examples
let
one = NonZero::new(
1usize
).unwrap();
let
max = NonZero::new(usize::MAX).unwrap();
assert_eq!
(one.div_ceil(max), one);
let
two = NonZero::new(
2usize
).unwrap();
let
three = NonZero::new(
3usize
).unwrap();
assert_eq!
(three.div_ceil(two), two);
Source
§
impl
NonZero
<
i8
>
1.67.0
·
Source
pub const
BITS
:
u32
= 8u32
The size of this non-zero integer type in bits.
This value is equal to
i8::BITS
.
§
Examples
assert_eq!
(NonZero::<i8>::BITS, i8::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
i8
>
The smallest value that can be represented by this non-zero
integer type,
equal to
i8::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i8>::MIN.get(), i8::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
i8
>
The largest value that can be represented by this non-zero
integer type,
equal to
i8::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i8>::MAX.get(), i8::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i8>::new(-
1i8
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i8>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i8>::new(
0b_01100100
)
?
;
let
b = NonZero::<i8>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i8>::new(
0b_01100100
)
?
;
let
b = NonZero::<i8>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<i8>::new(
0b100_0000
)
?
;
let
b = NonZero::<i8>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(-
0x7ei8
)
?
;
let
m = NonZero::new(
0xa
)
?
;
assert_eq!
(n.rotate_left(
2
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xai8
)
?
;
let
m = NonZero::new(-
0x7e
)
?
;
assert_eq!
(n.rotate_right(
2
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12i8
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x12
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12i8
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
i8
>) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI8;
let
n = NonZero::new(
0x1Ai8
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroI8::from_be(n), n)
}
else
{
assert_eq!
(NonZeroI8::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
i8
>) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI8;
let
n = NonZero::new(
0x1Ai8
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroI8::from_le(n), n)
}
else
{
assert_eq!
(NonZeroI8::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai8
)
?
;
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
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai8
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
i8
>
Computes the absolute value of self.
See
i8::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1i8
)
?
;
let
neg = NonZero::new(-
1i8
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
i8
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<i8>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1i8
)
?
;
let
neg = NonZero::new(-
1i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
i8
>,
bool
)
Computes the absolute value of self,
with overflow information, see
i8::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1i8
)
?
;
let
neg = NonZero::new(-
1i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
i8
>
Saturating absolute value, see
i8::saturating_abs
.
§
Example
let
pos = NonZero::new(
1i8
)
?
;
let
neg = NonZero::new(-
1i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
let
min_plus = NonZero::new(i8::MIN +
1
)
?
;
let
max = NonZero::new(i8::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
i8
>
Wrapping absolute value, see
i8::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1i8
)
?
;
let
neg = NonZero::new(-
1i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
u8
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1u8
)
?
;
let
i_pos = NonZero::new(
1i8
)
?
;
let
i_neg = NonZero::new(-
1i8
)
?
;
let
i_min = NonZero::new(i8::MIN)
?
;
let
u_max = NonZero::new(u8::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
i8
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<i8>::MIN
.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
i8
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
i8::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
i8
>
Saturating negation. Computes
-self
,
returning
NonZero::<i8>::MAX
if
self == NonZero::<i8>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
let
min_plus_one = NonZero::new(i8::MIN +
1
)
?
;
let
max = NonZero::new(i8::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
i8
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
i8::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i8
)
?
;
let
neg_five = NonZero::new(-
5i8
)
?
;
let
min = NonZero::new(i8::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
u8
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1i8
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<u8>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
i8
>) ->
Option
<
NonZero
<
i8
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2i8
)
?
;
let
four = NonZero::new(
4i8
)
?
;
let
max = NonZero::new(i8::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
i8
>) ->
NonZero
<
i8
>
Multiplies two non-zero integers together.
Return
NonZero::<i8>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2i8
)
?
;
let
four = NonZero::new(
4i8
)
?
;
let
max = NonZero::new(i8::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
i8
>) ->
NonZero
<
i8
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > i8::MAX
, or
self * rhs < i8::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2i8
)
?
;
let
four = NonZero::new(
4i8
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
i8
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3i8
)
?
;
let
twenty_seven = NonZero::new(
27i8
)
?
;
let
half_max = NonZero::new(i8::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
i8
>
Raise non-zero value to an integer power.
Return
NonZero::<i8>::MIN
or
NonZero::<i8>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3i8
)
?
;
let
twenty_seven = NonZero::new(
27i8
)
?
;
let
max = NonZero::new(i8::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
i16
>
1.67.0
·
Source
pub const
BITS
:
u32
= 16u32
The size of this non-zero integer type in bits.
This value is equal to
i16::BITS
.
§
Examples
assert_eq!
(NonZero::<i16>::BITS, i16::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
i16
>
The smallest value that can be represented by this non-zero
integer type,
equal to
i16::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i16>::MIN.get(), i16::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
i16
>
The largest value that can be represented by this non-zero
integer type,
equal to
i16::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i16>::MAX.get(), i16::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i16>::new(-
1i16
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i16>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i16>::new(
0b_01100100
)
?
;
let
b = NonZero::<i16>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i16>::new(
0b_01100100
)
?
;
let
b = NonZero::<i16>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<i16>::new(
0b100_0000
)
?
;
let
b = NonZero::<i16>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(-
0x5ffdi16
)
?
;
let
m = NonZero::new(
0x3a
)
?
;
assert_eq!
(n.rotate_left(
4
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x3ai16
)
?
;
let
m = NonZero::new(-
0x5ffd
)
?
;
assert_eq!
(n.rotate_right(
4
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234i16
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x3412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234i16
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
i16
>) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI16;
let
n = NonZero::new(
0x1Ai16
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroI16::from_be(n), n)
}
else
{
assert_eq!
(NonZeroI16::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
i16
>) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI16;
let
n = NonZero::new(
0x1Ai16
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroI16::from_le(n), n)
}
else
{
assert_eq!
(NonZeroI16::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai16
)
?
;
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
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai16
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
i16
>
Computes the absolute value of self.
See
i16::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1i16
)
?
;
let
neg = NonZero::new(-
1i16
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
i16
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<i16>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1i16
)
?
;
let
neg = NonZero::new(-
1i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
i16
>,
bool
)
Computes the absolute value of self,
with overflow information, see
i16::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1i16
)
?
;
let
neg = NonZero::new(-
1i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
i16
>
Saturating absolute value, see
i16::saturating_abs
.
§
Example
let
pos = NonZero::new(
1i16
)
?
;
let
neg = NonZero::new(-
1i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
let
min_plus = NonZero::new(i16::MIN +
1
)
?
;
let
max = NonZero::new(i16::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
i16
>
Wrapping absolute value, see
i16::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1i16
)
?
;
let
neg = NonZero::new(-
1i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
u16
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1u16
)
?
;
let
i_pos = NonZero::new(
1i16
)
?
;
let
i_neg = NonZero::new(-
1i16
)
?
;
let
i_min = NonZero::new(i16::MIN)
?
;
let
u_max = NonZero::new(u16::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
i16
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<i16>::MIN
.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
i16
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
i16::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
i16
>
Saturating negation. Computes
-self
,
returning
NonZero::<i16>::MAX
if
self == NonZero::<i16>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
let
min_plus_one = NonZero::new(i16::MIN +
1
)
?
;
let
max = NonZero::new(i16::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
i16
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
i16::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i16
)
?
;
let
neg_five = NonZero::new(-
5i16
)
?
;
let
min = NonZero::new(i16::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
u16
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1i16
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<u16>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
i16
>) ->
Option
<
NonZero
<
i16
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2i16
)
?
;
let
four = NonZero::new(
4i16
)
?
;
let
max = NonZero::new(i16::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
i16
>) ->
NonZero
<
i16
>
Multiplies two non-zero integers together.
Return
NonZero::<i16>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2i16
)
?
;
let
four = NonZero::new(
4i16
)
?
;
let
max = NonZero::new(i16::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
i16
>) ->
NonZero
<
i16
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > i16::MAX
, or
self * rhs < i16::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2i16
)
?
;
let
four = NonZero::new(
4i16
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
i16
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3i16
)
?
;
let
twenty_seven = NonZero::new(
27i16
)
?
;
let
half_max = NonZero::new(i16::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
i16
>
Raise non-zero value to an integer power.
Return
NonZero::<i16>::MIN
or
NonZero::<i16>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3i16
)
?
;
let
twenty_seven = NonZero::new(
27i16
)
?
;
let
max = NonZero::new(i16::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
i32
>
1.67.0
·
Source
pub const
BITS
:
u32
= 32u32
The size of this non-zero integer type in bits.
This value is equal to
i32::BITS
.
§
Examples
assert_eq!
(NonZero::<i32>::BITS, i32::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
i32
>
The smallest value that can be represented by this non-zero
integer type,
equal to
i32::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i32>::MIN.get(), i32::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
i32
>
The largest value that can be represented by this non-zero
integer type,
equal to
i32::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i32>::MAX.get(), i32::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i32>::new(-
1i32
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i32>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i32>::new(
0b_01100100
)
?
;
let
b = NonZero::<i32>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i32>::new(
0b_01100100
)
?
;
let
b = NonZero::<i32>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<i32>::new(
0b100_0000
)
?
;
let
b = NonZero::<i32>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x10000b3i32
)
?
;
let
m = NonZero::new(
0xb301
)
?
;
assert_eq!
(n.rotate_left(
8
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xb301i32
)
?
;
let
m = NonZero::new(
0x10000b3
)
?
;
assert_eq!
(n.rotate_right(
8
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678i32
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x78563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678i32
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x1e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
i32
>) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI32;
let
n = NonZero::new(
0x1Ai32
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroI32::from_be(n), n)
}
else
{
assert_eq!
(NonZeroI32::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
i32
>) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI32;
let
n = NonZero::new(
0x1Ai32
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroI32::from_le(n), n)
}
else
{
assert_eq!
(NonZeroI32::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai32
)
?
;
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
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai32
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
i32
>
Computes the absolute value of self.
See
i32::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1i32
)
?
;
let
neg = NonZero::new(-
1i32
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
i32
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<i32>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1i32
)
?
;
let
neg = NonZero::new(-
1i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
i32
>,
bool
)
Computes the absolute value of self,
with overflow information, see
i32::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1i32
)
?
;
let
neg = NonZero::new(-
1i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
i32
>
Saturating absolute value, see
i32::saturating_abs
.
§
Example
let
pos = NonZero::new(
1i32
)
?
;
let
neg = NonZero::new(-
1i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
let
min_plus = NonZero::new(i32::MIN +
1
)
?
;
let
max = NonZero::new(i32::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
i32
>
Wrapping absolute value, see
i32::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1i32
)
?
;
let
neg = NonZero::new(-
1i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
u32
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1u32
)
?
;
let
i_pos = NonZero::new(
1i32
)
?
;
let
i_neg = NonZero::new(-
1i32
)
?
;
let
i_min = NonZero::new(i32::MIN)
?
;
let
u_max = NonZero::new(u32::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
i32
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<i32>::MIN
.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
i32
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
i32::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
i32
>
Saturating negation. Computes
-self
,
returning
NonZero::<i32>::MAX
if
self == NonZero::<i32>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
let
min_plus_one = NonZero::new(i32::MIN +
1
)
?
;
let
max = NonZero::new(i32::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
i32
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
i32::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i32
)
?
;
let
neg_five = NonZero::new(-
5i32
)
?
;
let
min = NonZero::new(i32::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
u32
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1i32
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<u32>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
i32
>) ->
Option
<
NonZero
<
i32
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2i32
)
?
;
let
four = NonZero::new(
4i32
)
?
;
let
max = NonZero::new(i32::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
i32
>) ->
NonZero
<
i32
>
Multiplies two non-zero integers together.
Return
NonZero::<i32>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2i32
)
?
;
let
four = NonZero::new(
4i32
)
?
;
let
max = NonZero::new(i32::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
i32
>) ->
NonZero
<
i32
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > i32::MAX
, or
self * rhs < i32::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2i32
)
?
;
let
four = NonZero::new(
4i32
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
i32
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3i32
)
?
;
let
twenty_seven = NonZero::new(
27i32
)
?
;
let
half_max = NonZero::new(i32::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
i32
>
Raise non-zero value to an integer power.
Return
NonZero::<i32>::MIN
or
NonZero::<i32>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3i32
)
?
;
let
twenty_seven = NonZero::new(
27i32
)
?
;
let
max = NonZero::new(i32::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
i64
>
1.67.0
·
Source
pub const
BITS
:
u32
= 64u32
The size of this non-zero integer type in bits.
This value is equal to
i64::BITS
.
§
Examples
assert_eq!
(NonZero::<i64>::BITS, i64::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
i64
>
The smallest value that can be represented by this non-zero
integer type,
equal to
i64::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i64>::MIN.get(), i64::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
i64
>
The largest value that can be represented by this non-zero
integer type,
equal to
i64::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i64>::MAX.get(), i64::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i64>::new(-
1i64
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i64>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i64>::new(
0b_01100100
)
?
;
let
b = NonZero::<i64>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i64>::new(
0b_01100100
)
?
;
let
b = NonZero::<i64>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<i64>::new(
0b100_0000
)
?
;
let
b = NonZero::<i64>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xaa00000000006e1i64
)
?
;
let
m = NonZero::new(
0x6e10aa
)
?
;
assert_eq!
(n.rotate_left(
12
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x6e10aai64
)
?
;
let
m = NonZero::new(
0xaa00000000006e1
)
?
;
assert_eq!
(n.rotate_right(
12
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456i64
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x5634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456i64
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
i64
>) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI64;
let
n = NonZero::new(
0x1Ai64
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroI64::from_be(n), n)
}
else
{
assert_eq!
(NonZeroI64::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
i64
>) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI64;
let
n = NonZero::new(
0x1Ai64
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroI64::from_le(n), n)
}
else
{
assert_eq!
(NonZeroI64::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai64
)
?
;
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
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai64
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
i64
>
Computes the absolute value of self.
See
i64::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1i64
)
?
;
let
neg = NonZero::new(-
1i64
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
i64
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<i64>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1i64
)
?
;
let
neg = NonZero::new(-
1i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
i64
>,
bool
)
Computes the absolute value of self,
with overflow information, see
i64::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1i64
)
?
;
let
neg = NonZero::new(-
1i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
i64
>
Saturating absolute value, see
i64::saturating_abs
.
§
Example
let
pos = NonZero::new(
1i64
)
?
;
let
neg = NonZero::new(-
1i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
let
min_plus = NonZero::new(i64::MIN +
1
)
?
;
let
max = NonZero::new(i64::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
i64
>
Wrapping absolute value, see
i64::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1i64
)
?
;
let
neg = NonZero::new(-
1i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
u64
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1u64
)
?
;
let
i_pos = NonZero::new(
1i64
)
?
;
let
i_neg = NonZero::new(-
1i64
)
?
;
let
i_min = NonZero::new(i64::MIN)
?
;
let
u_max = NonZero::new(u64::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
i64
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<i64>::MIN
.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
i64
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
i64::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
i64
>
Saturating negation. Computes
-self
,
returning
NonZero::<i64>::MAX
if
self == NonZero::<i64>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
let
min_plus_one = NonZero::new(i64::MIN +
1
)
?
;
let
max = NonZero::new(i64::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
i64
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
i64::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i64
)
?
;
let
neg_five = NonZero::new(-
5i64
)
?
;
let
min = NonZero::new(i64::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
u64
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1i64
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<u64>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
i64
>) ->
Option
<
NonZero
<
i64
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2i64
)
?
;
let
four = NonZero::new(
4i64
)
?
;
let
max = NonZero::new(i64::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
i64
>) ->
NonZero
<
i64
>
Multiplies two non-zero integers together.
Return
NonZero::<i64>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2i64
)
?
;
let
four = NonZero::new(
4i64
)
?
;
let
max = NonZero::new(i64::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
i64
>) ->
NonZero
<
i64
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > i64::MAX
, or
self * rhs < i64::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2i64
)
?
;
let
four = NonZero::new(
4i64
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
i64
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3i64
)
?
;
let
twenty_seven = NonZero::new(
27i64
)
?
;
let
half_max = NonZero::new(i64::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
i64
>
Raise non-zero value to an integer power.
Return
NonZero::<i64>::MIN
or
NonZero::<i64>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3i64
)
?
;
let
twenty_seven = NonZero::new(
27i64
)
?
;
let
max = NonZero::new(i64::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
i128
>
1.67.0
·
Source
pub const
BITS
:
u32
= 128u32
The size of this non-zero integer type in bits.
This value is equal to
i128::BITS
.
§
Examples
assert_eq!
(NonZero::<i128>::BITS, i128::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
i128
>
The smallest value that can be represented by this non-zero
integer type,
equal to
i128::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i128>::MIN.get(), i128::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
i128
>
The largest value that can be represented by this non-zero
integer type,
equal to
i128::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<i128>::MAX.get(), i128::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i128>::new(-
1i128
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<i128>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i128>::new(
0b_01100100
)
?
;
let
b = NonZero::<i128>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<i128>::new(
0b_01100100
)
?
;
let
b = NonZero::<i128>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<i128>::new(
0b100_0000
)
?
;
let
b = NonZero::<i128>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x13f40000000000000000000000004f76i128
)
?
;
let
m = NonZero::new(
0x4f7613f4
)
?
;
assert_eq!
(n.rotate_left(
16
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x4f7613f4i128
)
?
;
let
m = NonZero::new(
0x13f40000000000000000000000004f76
)
?
;
assert_eq!
(n.rotate_right(
16
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678901234567890123456789012i128
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x12907856341290785634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x12345678901234567890123456789012i128
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x48091e6a2c48091e6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
i128
>) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI128;
let
n = NonZero::new(
0x1Ai128
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroI128::from_be(n), n)
}
else
{
assert_eq!
(NonZeroI128::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
i128
>) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroI128;
let
n = NonZero::new(
0x1Ai128
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroI128::from_le(n), n)
}
else
{
assert_eq!
(NonZeroI128::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai128
)
?
;
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
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Ai128
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
i128
>
Computes the absolute value of self.
See
i128::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1i128
)
?
;
let
neg = NonZero::new(-
1i128
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
i128
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<i128>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1i128
)
?
;
let
neg = NonZero::new(-
1i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
i128
>,
bool
)
Computes the absolute value of self,
with overflow information, see
i128::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1i128
)
?
;
let
neg = NonZero::new(-
1i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
i128
>
Saturating absolute value, see
i128::saturating_abs
.
§
Example
let
pos = NonZero::new(
1i128
)
?
;
let
neg = NonZero::new(-
1i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
let
min_plus = NonZero::new(i128::MIN +
1
)
?
;
let
max = NonZero::new(i128::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
i128
>
Wrapping absolute value, see
i128::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1i128
)
?
;
let
neg = NonZero::new(-
1i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
u128
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1u128
)
?
;
let
i_pos = NonZero::new(
1i128
)
?
;
let
i_neg = NonZero::new(-
1i128
)
?
;
let
i_min = NonZero::new(i128::MIN)
?
;
let
u_max = NonZero::new(u128::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
i128
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<i128>::MIN
.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
i128
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
i128::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
i128
>
Saturating negation. Computes
-self
,
returning
NonZero::<i128>::MAX
if
self == NonZero::<i128>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
let
min_plus_one = NonZero::new(i128::MIN +
1
)
?
;
let
max = NonZero::new(i128::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
i128
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
i128::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5i128
)
?
;
let
neg_five = NonZero::new(-
5i128
)
?
;
let
min = NonZero::new(i128::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
u128
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1i128
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<u128>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
i128
>) ->
Option
<
NonZero
<
i128
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2i128
)
?
;
let
four = NonZero::new(
4i128
)
?
;
let
max = NonZero::new(i128::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
i128
>) ->
NonZero
<
i128
>
Multiplies two non-zero integers together.
Return
NonZero::<i128>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2i128
)
?
;
let
four = NonZero::new(
4i128
)
?
;
let
max = NonZero::new(i128::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
i128
>) ->
NonZero
<
i128
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > i128::MAX
, or
self * rhs < i128::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2i128
)
?
;
let
four = NonZero::new(
4i128
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
i128
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3i128
)
?
;
let
twenty_seven = NonZero::new(
27i128
)
?
;
let
half_max = NonZero::new(i128::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
i128
>
Raise non-zero value to an integer power.
Return
NonZero::<i128>::MIN
or
NonZero::<i128>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3i128
)
?
;
let
twenty_seven = NonZero::new(
27i128
)
?
;
let
max = NonZero::new(i128::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Source
§
impl
NonZero
<
isize
>
1.67.0
·
Source
pub const
BITS
:
u32
= 64u32
The size of this non-zero integer type in bits.
This value is equal to
isize::BITS
.
§
Examples
assert_eq!
(NonZero::<isize>::BITS, isize::BITS);
1.70.0
·
Source
pub const
MIN
:
NonZero
<
isize
>
The smallest value that can be represented by this non-zero
integer type,
equal to
isize::MIN
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<isize>::MIN.get(), isize::MIN);
1.70.0
·
Source
pub const
MAX
:
NonZero
<
isize
>
The largest value that can be represented by this non-zero
integer type,
equal to
isize::MAX
.
Note: While most integer types are defined for every whole
number between
MIN
and
MAX
, signed non-zero integers are
a special case. They have a “gap” at 0.
§
Examples
assert_eq!
(NonZero::<isize>::MAX.get(), isize::MAX);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
On many architectures, this function can perform better than
leading_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<isize>::new(-
1isize
)
?
;
assert_eq!
(n.leading_zeros(),
0
);
1.53.0 (const: 1.53.0)
·
Source
pub const fn
trailing_zeros
(self) ->
u32
Returns the number of trailing zeros in the binary representation
of
self
.
On many architectures, this function can perform better than
trailing_zeros()
on the underlying integer type, as special handling of zero can be avoided.
§
Examples
Basic usage:
let
n = NonZero::<isize>::new(
0b0101000
)
?
;
assert_eq!
(n.trailing_zeros(),
3
);
Source
pub const fn
isolate_most_significant_one
(self) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<isize>::new(
0b_01100100
)
?
;
let
b = NonZero::<isize>::new(
0b_01000000
)
?
;
assert_eq!
(a.isolate_most_significant_one(), b);
Source
pub const fn
isolate_least_significant_one
(self) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set.
§
Example
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
a = NonZero::<isize>::new(
0b_01100100
)
?
;
let
b = NonZero::<isize>::new(
0b_00000100
)
?
;
assert_eq!
(a.isolate_least_significant_one(), b);
1.86.0 (const: 1.86.0)
·
Source
pub const fn
count_ones
(self) ->
NonZero
<
u32
>
Returns the number of ones in the binary representation of
self
.
§
Examples
Basic usage:
let
a = NonZero::<isize>::new(
0b100_0000
)
?
;
let
b = NonZero::<isize>::new(
0b100_0011
)
?
;
assert_eq!
(a.count_ones(), NonZero::new(
1
)
?
);
assert_eq!
(b.count_ones(), NonZero::new(
3
)
?
);
Source
pub const fn
rotate_left
(self, n:
u32
) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the left by a specified amount,
n
,
wrapping the truncated bits to the end of the resulting integer.
Please note this isn’t the same operation as the
<<
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0xaa00000000006e1isize
)
?
;
let
m = NonZero::new(
0x6e10aa
)
?
;
assert_eq!
(n.rotate_left(
12
), m);
Source
pub const fn
rotate_right
(self, n:
u32
) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Shifts the bits to the right by a specified amount,
n
,
wrapping the truncated bits to the beginning of the resulting
integer.
Please note this isn’t the same operation as the
>>
shifting operator!
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x6e10aaisize
)
?
;
let
m = NonZero::new(
0xaa00000000006e1
)
?
;
assert_eq!
(n.rotate_right(
12
), m);
Source
pub const fn
swap_bytes
(self) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the byte order of the integer.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456isize
)
?
;
let
m = n.swap_bytes();
assert_eq!
(m, NonZero::new(
0x5634129078563412
)
?
);
Source
pub const fn
reverse_bits
(self) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1234567890123456isize
)
?
;
let
m = n.reverse_bits();
assert_eq!
(m, NonZero::new(
0x6a2c48091e6a2c48
)
?
);
Source
pub const fn
from_be
(x:
NonZero
<
isize
>) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroIsize;
let
n = NonZero::new(
0x1Aisize
)
?
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(NonZeroIsize::from_be(n), n)
}
else
{
assert_eq!
(NonZeroIsize::from_be(n), n.swap_bytes())
}
Source
pub const fn
from_le
(x:
NonZero
<
isize
>) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
use
std::num::NonZeroIsize;
let
n = NonZero::new(
0x1Aisize
)
?
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(NonZeroIsize::from_le(n), n)
}
else
{
assert_eq!
(NonZeroIsize::from_le(n), n.swap_bytes())
}
Source
pub const fn
to_be
(self) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Aisize
)
?
;
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
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_bitwise
#128281
)
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
#![feature(nonzero_bitwise)]
let
n = NonZero::new(
0x1Aisize
)
?
;
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
1.64.0 (const: 1.64.0)
·
Source
pub const fn
abs
(self) ->
NonZero
<
isize
>
Computes the absolute value of self.
See
isize::abs
for documentation on overflow behavior.
§
Example
let
pos = NonZero::new(
1isize
)
?
;
let
neg = NonZero::new(-
1isize
)
?
;
assert_eq!
(pos, pos.abs());
assert_eq!
(pos, neg.abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
NonZero
<
isize
>>
Checked absolute value.
Checks for overflow and returns
None
if
self == NonZero::<isize>::MIN
.
The result cannot be zero.
§
Example
let
pos = NonZero::new(
1isize
)
?
;
let
neg = NonZero::new(-
1isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
(
Some
(pos), neg.checked_abs());
assert_eq!
(
None
, min.checked_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
NonZero
<
isize
>,
bool
)
Computes the absolute value of self,
with overflow information, see
isize::overflowing_abs
.
§
Example
let
pos = NonZero::new(
1isize
)
?
;
let
neg = NonZero::new(-
1isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
((pos,
false
), pos.overflowing_abs());
assert_eq!
((pos,
false
), neg.overflowing_abs());
assert_eq!
((min,
true
), min.overflowing_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_abs
(self) ->
NonZero
<
isize
>
Saturating absolute value, see
isize::saturating_abs
.
§
Example
let
pos = NonZero::new(
1isize
)
?
;
let
neg = NonZero::new(-
1isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
let
min_plus = NonZero::new(isize::MIN +
1
)
?
;
let
max = NonZero::new(isize::MAX)
?
;
assert_eq!
(pos, pos.saturating_abs());
assert_eq!
(pos, neg.saturating_abs());
assert_eq!
(max, min.saturating_abs());
assert_eq!
(max, min_plus.saturating_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
wrapping_abs
(self) ->
NonZero
<
isize
>
Wrapping absolute value, see
isize::wrapping_abs
.
§
Example
let
pos = NonZero::new(
1isize
)
?
;
let
neg = NonZero::new(-
1isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
(pos, pos.wrapping_abs());
assert_eq!
(pos, neg.wrapping_abs());
assert_eq!
(min, min.wrapping_abs());
assert_eq!
(max, (-max).wrapping_abs());
1.64.0 (const: 1.64.0)
·
Source
pub const fn
unsigned_abs
(self) ->
NonZero
<
usize
>
Computes the absolute value of self
without any wrapping or panicking.
§
Example
let
u_pos = NonZero::new(
1usize
)
?
;
let
i_pos = NonZero::new(
1isize
)
?
;
let
i_neg = NonZero::new(-
1isize
)
?
;
let
i_min = NonZero::new(isize::MIN)
?
;
let
u_max = NonZero::new(usize::MAX /
2
+
1
)
?
;
assert_eq!
(u_pos, i_pos.unsigned_abs());
assert_eq!
(u_pos, i_neg.unsigned_abs());
assert_eq!
(u_max, i_min.unsigned_abs());
1.71.0 (const: 1.71.0)
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
if the
number is negative.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
assert!
(pos_five.is_positive());
assert!
(!neg_five.is_positive());
1.71.0 (const: 1.71.0)
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
if the
number is positive.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
assert!
(neg_five.is_negative());
assert!
(!pos_five.is_negative());
1.71.0 (const: 1.71.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
NonZero
<
isize
>>
Checked negation. Computes
-self
,
returning
None
if
self == NonZero::<isize>::MIN
.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
(pos_five.checked_neg(),
Some
(neg_five));
assert_eq!
(min.checked_neg(),
None
);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
NonZero
<
isize
>,
bool
)
Negates self, overflowing if this is equal to the minimum value.
See
isize::overflowing_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
(pos_five.overflowing_neg(), (neg_five,
false
));
assert_eq!
(min.overflowing_neg(), (min,
true
));
1.71.0 (const: 1.71.0)
·
Source
pub const fn
saturating_neg
(self) ->
NonZero
<
isize
>
Saturating negation. Computes
-self
,
returning
NonZero::<isize>::MAX
if
self == NonZero::<isize>::MIN
instead of overflowing.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
let
min_plus_one = NonZero::new(isize::MIN +
1
)
?
;
let
max = NonZero::new(isize::MAX)
?
;
assert_eq!
(pos_five.saturating_neg(), neg_five);
assert_eq!
(min.saturating_neg(), max);
assert_eq!
(max.saturating_neg(), min_plus_one);
1.71.0 (const: 1.71.0)
·
Source
pub const fn
wrapping_neg
(self) ->
NonZero
<
isize
>
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
See
isize::wrapping_neg
for documentation on overflow behavior.
§
Example
let
pos_five = NonZero::new(
5isize
)
?
;
let
neg_five = NonZero::new(-
5isize
)
?
;
let
min = NonZero::new(isize::MIN)
?
;
assert_eq!
(pos_five.wrapping_neg(), neg_five);
assert_eq!
(min.wrapping_neg(), min);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
NonZero
<
usize
>
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
§
Examples
Basic usage:
let
n = NonZero::new(-
1isize
).unwrap();
assert_eq!
(n.cast_unsigned(), NonZero::<usize>::MAX);
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_mul
(self, other:
NonZero
<
isize
>) ->
Option
<
NonZero
<
isize
>>
Multiplies two non-zero integers together.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
two = NonZero::new(
2isize
)
?
;
let
four = NonZero::new(
4isize
)
?
;
let
max = NonZero::new(isize::MAX)
?
;
assert_eq!
(
Some
(four), two.checked_mul(two));
assert_eq!
(
None
, max.checked_mul(two));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_mul
(self, other:
NonZero
<
isize
>) ->
NonZero
<
isize
>
Multiplies two non-zero integers together.
Return
NonZero::<isize>::MAX
on overflow.
§
Examples
let
two = NonZero::new(
2isize
)
?
;
let
four = NonZero::new(
4isize
)
?
;
let
max = NonZero::new(isize::MAX)
?
;
assert_eq!
(four, two.saturating_mul(two));
assert_eq!
(max, four.saturating_mul(max));
Source
pub const unsafe fn
unchecked_mul
(self, other:
NonZero
<
isize
>) ->
NonZero
<
isize
>
🔬
This is a nightly-only experimental API. (
nonzero_ops
#84186
)
Multiplies two non-zero integers together,
assuming overflow cannot occur.
Overflow is unchecked, and it is undefined behavior to overflow
even if the result would wrap to a non-zero value
.
The behavior is undefined as soon as
self * rhs > isize::MAX
, or
self * rhs < isize::MIN
.
§
Examples
#![feature(nonzero_ops)]
let
two = NonZero::new(
2isize
)
?
;
let
four = NonZero::new(
4isize
)
?
;
assert_eq!
(four,
unsafe
{ two.unchecked_mul(two) });
1.64.0 (const: 1.64.0)
·
Source
pub const fn
checked_pow
(self, other:
u32
) ->
Option
<
NonZero
<
isize
>>
Raises non-zero value to an integer power.
Checks for overflow and returns
None
on overflow.
As a consequence, the result cannot wrap to zero.
§
Examples
let
three = NonZero::new(
3isize
)
?
;
let
twenty_seven = NonZero::new(
27isize
)
?
;
let
half_max = NonZero::new(isize::MAX /
2
)
?
;
assert_eq!
(
Some
(twenty_seven), three.checked_pow(
3
));
assert_eq!
(
None
, half_max.checked_pow(
3
));
1.64.0 (const: 1.64.0)
·
Source
pub const fn
saturating_pow
(self, other:
u32
) ->
NonZero
<
isize
>
Raise non-zero value to an integer power.
Return
NonZero::<isize>::MIN
or
NonZero::<isize>::MAX
on overflow.
§
Examples
let
three = NonZero::new(
3isize
)
?
;
let
twenty_seven = NonZero::new(
27isize
)
?
;
let
max = NonZero::new(isize::MAX)
?
;
assert_eq!
(twenty_seven, three.saturating_pow(
3
));
assert_eq!
(max, max.saturating_pow(
3
));
Trait Implementations
§
1.28.0
·
Source
§
impl<T>
Binary
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.45.0
·
Source
§
impl<T>
BitOr
<
NonZero
<T>> for T
where
    T:
ZeroablePrimitive
+
BitOr
<Output = T>,
Source
§
type
Output
=
NonZero
<T>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
NonZero
<T>) -> <T as
BitOr
<
NonZero
<T>>>::
Output
Performs the
|
operation.
Read more
1.45.0
·
Source
§
impl<T>
BitOr
<T> for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
BitOr
<Output = T>,
Source
§
type
Output
=
NonZero
<T>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs: T) -> <
NonZero
<T> as
BitOr
<T>>::
Output
Performs the
|
operation.
Read more
1.45.0
·
Source
§
impl<T>
BitOr
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
BitOr
<Output = T>,
Source
§
type
Output
=
NonZero
<T>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
NonZero
<T>) -> <
NonZero
<T> as
BitOr
>::
Output
Performs the
|
operation.
Read more
1.45.0
·
Source
§
impl<T>
BitOrAssign
<T> for
NonZero
<T>
where
    T:
ZeroablePrimitive
,
NonZero
<T>:
BitOr
<T, Output =
NonZero
<T>>,
Source
§
fn
bitor_assign
(&mut self, rhs: T)
Performs the
|=
operation.
Read more
1.45.0
·
Source
§
impl<T>
BitOrAssign
for
NonZero
<T>
where
    T:
ZeroablePrimitive
,
NonZero
<T>:
BitOr
<Output =
NonZero
<T>>,
Source
§
fn
bitor_assign
(&mut self, rhs:
NonZero
<T>)
Performs the
|=
operation.
Read more
1.28.0
·
Source
§
impl<T>
Clone
for
NonZero
<T>
where
    T:
ZeroablePrimitive
,
Source
§
fn
clone
(&self) ->
NonZero
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
1.28.0
·
Source
§
impl<T>
Debug
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.28.0
·
Source
§
impl<T>
Display
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u128
>> for
u128
Source
§
fn
div
(self, other:
NonZero
<
u128
>) ->
u128
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
u128
The resulting type after applying the
/
operator.
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u16
>> for
u16
Source
§
fn
div
(self, other:
NonZero
<
u16
>) ->
u16
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
u16
The resulting type after applying the
/
operator.
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u32
>> for
u32
Source
§
fn
div
(self, other:
NonZero
<
u32
>) ->
u32
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
u32
The resulting type after applying the
/
operator.
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u64
>> for
u64
Source
§
fn
div
(self, other:
NonZero
<
u64
>) ->
u64
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
u64
The resulting type after applying the
/
operator.
1.51.0
·
Source
§
impl
Div
<
NonZero
<
u8
>> for
u8
Source
§
fn
div
(self, other:
NonZero
<
u8
>) ->
u8
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
u8
The resulting type after applying the
/
operator.
1.51.0
·
Source
§
impl
Div
<
NonZero
<
usize
>> for
usize
Source
§
fn
div
(self, other:
NonZero
<
usize
>) ->
usize
Same as
self / other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
type
Output
=
usize
The resulting type after applying the
/
operator.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
u128
>> for
u128
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
u128
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
u16
>> for
u16
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
u16
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
u32
>> for
u32
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
u32
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
u64
>> for
u64
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
u64
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
u8
>> for
u8
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
u8
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
1.79.0
·
Source
§
impl
DivAssign
<
NonZero
<
usize
>> for
usize
Source
§
fn
div_assign
(&mut self, other:
NonZero
<
usize
>)
Same as
self /= other.get()
, but because
other
is a
NonZero<_>
,
there’s never a runtime check for division-by-zero.
This operation rounds towards zero, truncating any fractional
part of the exact result, and cannot panic.
Source
§
impl
From
<
Alignment
> for
NonZero
<
usize
>
Source
§
fn
from
(align:
Alignment
) ->
NonZero
<
usize
>
Converts to this type from the input type.
1.31.0
·
Source
§
impl<T>
From
<
NonZero
<T>> for T
where
    T:
ZeroablePrimitive
,
Source
§
fn
from
(nonzero:
NonZero
<T>) -> T
Converts to this type from the input type.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
i16
>) ->
NonZero
<
i128
>
Converts
NonZero
<
i16
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i32
>
Source
§
fn
from
(small:
NonZero
<
i16
>) ->
NonZero
<
i32
>
Converts
NonZero
<
i16
>
to
NonZero
<
i32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
i16
>) ->
NonZero
<
i64
>
Converts
NonZero
<
i16
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i16
>> for
NonZero
<
isize
>
Source
§
fn
from
(small:
NonZero
<
i16
>) ->
NonZero
<
isize
>
Converts
NonZero
<
i16
>
to
NonZero
<
isize
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i32
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
i32
>) ->
NonZero
<
i128
>
Converts
NonZero
<
i32
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i32
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
i32
>) ->
NonZero
<
i64
>
Converts
NonZero
<
i32
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i64
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
i64
>) ->
NonZero
<
i128
>
Converts
NonZero
<
i64
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
i8
>) ->
NonZero
<
i128
>
Converts
NonZero
<
i8
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i16
>
Source
§
fn
from
(small:
NonZero
<
i8
>) ->
NonZero
<
i16
>
Converts
NonZero
<
i8
>
to
NonZero
<
i16
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i32
>
Source
§
fn
from
(small:
NonZero
<
i8
>) ->
NonZero
<
i32
>
Converts
NonZero
<
i8
>
to
NonZero
<
i32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
i8
>) ->
NonZero
<
i64
>
Converts
NonZero
<
i8
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
i8
>> for
NonZero
<
isize
>
Source
§
fn
from
(small:
NonZero
<
i8
>) ->
NonZero
<
isize
>
Converts
NonZero
<
i8
>
to
NonZero
<
isize
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
i128
>
Converts
NonZero
<
u16
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i32
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
i32
>
Converts
NonZero
<
u16
>
to
NonZero
<
i32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
i64
>
Converts
NonZero
<
u16
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u128
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
u128
>
Converts
NonZero
<
u16
>
to
NonZero
<
u128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u32
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
u32
>
Converts
NonZero
<
u16
>
to
NonZero
<
u32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
u64
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
u64
>
Converts
NonZero
<
u16
>
to
NonZero
<
u64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u16
>> for
NonZero
<
usize
>
Source
§
fn
from
(small:
NonZero
<
u16
>) ->
NonZero
<
usize
>
Converts
NonZero
<
u16
>
to
NonZero
<
usize
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
u32
>) ->
NonZero
<
i128
>
Converts
NonZero
<
u32
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
u32
>) ->
NonZero
<
i64
>
Converts
NonZero
<
u32
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
u128
>
Source
§
fn
from
(small:
NonZero
<
u32
>) ->
NonZero
<
u128
>
Converts
NonZero
<
u32
>
to
NonZero
<
u128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u32
>> for
NonZero
<
u64
>
Source
§
fn
from
(small:
NonZero
<
u32
>) ->
NonZero
<
u64
>
Converts
NonZero
<
u32
>
to
NonZero
<
u64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u64
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
u64
>) ->
NonZero
<
i128
>
Converts
NonZero
<
u64
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u64
>> for
NonZero
<
u128
>
Source
§
fn
from
(small:
NonZero
<
u64
>) ->
NonZero
<
u128
>
Converts
NonZero
<
u64
>
to
NonZero
<
u128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i128
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
i128
>
Converts
NonZero
<
u8
>
to
NonZero
<
i128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i16
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
i16
>
Converts
NonZero
<
u8
>
to
NonZero
<
i16
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i32
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
i32
>
Converts
NonZero
<
u8
>
to
NonZero
<
i32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
i64
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
i64
>
Converts
NonZero
<
u8
>
to
NonZero
<
i64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
isize
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
isize
>
Converts
NonZero
<
u8
>
to
NonZero
<
isize
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u128
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
u128
>
Converts
NonZero
<
u8
>
to
NonZero
<
u128
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u16
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
u16
>
Converts
NonZero
<
u8
>
to
NonZero
<
u16
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u32
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
u32
>
Converts
NonZero
<
u8
>
to
NonZero
<
u32
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
u64
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
u64
>
Converts
NonZero
<
u8
>
to
NonZero
<
u64
>
losslessly.
1.41.0
·
Source
§
impl
From
<
NonZero
<
u8
>> for
NonZero
<
usize
>
Source
§
fn
from
(small:
NonZero
<
u8
>) ->
NonZero
<
usize
>
Converts
NonZero
<
u8
>
to
NonZero
<
usize
>
losslessly.
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
i128
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
i128
>, <
NonZero
<
i128
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
i16
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
i32
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
i64
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
i8
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
isize
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(
    src: &
str
,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
u128
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
u16
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
u32
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
u64
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
u8
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(src: &
str
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.35.0
·
Source
§
impl
FromStr
for
NonZero
<
usize
>
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
fn
from_str
(
    src: &
str
,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.28.0
·
Source
§
impl<T>
Hash
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Hash
,
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
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
1.84.0
·
Source
§
impl<T>
LowerExp
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
LowerExp
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
1.28.0
·
Source
§
impl<T>
LowerHex
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
i128
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
i16
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
i32
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
i64
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
i8
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for &
NonZero
<
isize
>
Source
§
type
Output
= <
NonZero
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
NonZero
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
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
i128
>
Source
§
type
Output
=
NonZero
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
NonZero
<
i128
>
Performs the unary
-
operation.
Read more
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
i16
>
Source
§
type
Output
=
NonZero
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
NonZero
<
i16
>
Performs the unary
-
operation.
Read more
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
i32
>
Source
§
type
Output
=
NonZero
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
NonZero
<
i32
>
Performs the unary
-
operation.
Read more
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
i64
>
Source
§
type
Output
=
NonZero
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
NonZero
<
i64
>
Performs the unary
-
operation.
Read more
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
i8
>
Source
§
type
Output
=
NonZero
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
NonZero
<
i8
>
Performs the unary
-
operation.
Read more
1.71.0
·
Source
§
impl
Neg
for
NonZero
<
isize
>
Source
§
type
Output
=
NonZero
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
NonZero
<
isize
>
Performs the unary
-
operation.
Read more
1.28.0
·
Source
§
impl<T>
Octal
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.28.0
·
Source
§
impl<T>
Ord
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Ord
,
Source
§
fn
cmp
(&self, other: &
NonZero
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
Source
§
fn
max
(self, other:
NonZero
<T>) ->
NonZero
<T>
Compares and returns the maximum of two values.
Read more
Source
§
fn
min
(self, other:
NonZero
<T>) ->
NonZero
<T>
Compares and returns the minimum of two values.
Read more
Source
§
fn
clamp
(self, min:
NonZero
<T>, max:
NonZero
<T>) ->
NonZero
<T>
Restrict a value to a certain interval.
Read more
1.28.0
·
Source
§
impl<T>
PartialEq
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
PartialEq
,
Source
§
fn
eq
(&self, other: &
NonZero
<T>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
NonZero
<T>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.28.0
·
Source
§
impl<T>
PartialOrd
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
NonZero
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
Source
§
fn
lt
(&self, other: &
NonZero
<T>) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
Source
§
fn
le
(&self, other: &
NonZero
<T>) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
Source
§
fn
gt
(&self, other: &
NonZero
<T>) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
Source
§
fn
ge
(&self, other: &
NonZero
<T>) ->
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
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
u128
>> for
u128
Source
§
fn
rem
(self, other:
NonZero
<
u128
>) ->
u128
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
u128
The resulting type after applying the
%
operator.
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
u16
>> for
u16
Source
§
fn
rem
(self, other:
NonZero
<
u16
>) ->
u16
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
u16
The resulting type after applying the
%
operator.
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
u32
>> for
u32
Source
§
fn
rem
(self, other:
NonZero
<
u32
>) ->
u32
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
u32
The resulting type after applying the
%
operator.
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
u64
>> for
u64
Source
§
fn
rem
(self, other:
NonZero
<
u64
>) ->
u64
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
u64
The resulting type after applying the
%
operator.
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
u8
>> for
u8
Source
§
fn
rem
(self, other:
NonZero
<
u8
>) ->
u8
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
u8
The resulting type after applying the
%
operator.
1.51.0
·
Source
§
impl
Rem
<
NonZero
<
usize
>> for
usize
Source
§
fn
rem
(self, other:
NonZero
<
usize
>) ->
usize
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
Source
§
type
Output
=
usize
The resulting type after applying the
%
operator.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
u128
>> for
u128
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
u128
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
u16
>> for
u16
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
u16
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
u32
>> for
u32
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
u32
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
u64
>> for
u64
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
u64
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
u8
>> for
u8
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
u8
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.79.0
·
Source
§
impl
RemAssign
<
NonZero
<
usize
>> for
usize
Source
§
fn
rem_assign
(&mut self, other:
NonZero
<
usize
>)
This operation satisfies
n % d == n - (n / d) * d
, and cannot panic.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i128
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i128
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
i128
>>>::
Error
>
Attempts to convert
NonZero
<
i128
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i16
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i16
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
i16
>>>::
Error
>
Attempts to convert
NonZero
<
i16
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i32
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i32
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
i32
>>>::
Error
>
Attempts to convert
NonZero
<
i32
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i64
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i64
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
i64
>>>::
Error
>
Attempts to convert
NonZero
<
i64
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
i8
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
i8
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
i8
>>>::
Error
>
Attempts to convert
NonZero
<
i8
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
i128
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
i128
>, <
NonZero
<
i128
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
i128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
isize
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
isize
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
isize
>>>::
Error
>
Attempts to convert
NonZero
<
isize
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
i128
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
i128
>, <
NonZero
<
i128
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
i128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u128
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u128
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
u128
>>>::
Error
>
Attempts to convert
NonZero
<
u128
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u16
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u16
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
u16
>>>::
Error
>
Attempts to convert
NonZero
<
u16
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u16
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u16
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
u16
>>>::
Error
>
Attempts to convert
NonZero
<
u16
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u16
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u16
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
u16
>>>::
Error
>
Attempts to convert
NonZero
<
u16
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u16
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u16
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
u16
>>>::
Error
>
Attempts to convert
NonZero
<
u16
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u32
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u32
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
u32
>>>::
Error
>
Attempts to convert
NonZero
<
u32
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u64
>> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
NonZero
<
u64
>,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
NonZero
<
u64
>>>::
Error
>
Attempts to convert
NonZero
<
u64
>
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
u8
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
u8
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
u8
>>>::
Error
>
Attempts to convert
NonZero
<
u8
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
Alignment
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    align:
NonZero
<
usize
>,
) ->
Result
<
Alignment
, <
Alignment
as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Performs the conversion.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
i128
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
i128
>, <
NonZero
<
i128
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
i128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.49.0
·
Source
§
impl
TryFrom
<
NonZero
<
usize
>> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
NonZero
<
usize
>,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Attempts to convert
NonZero
<
usize
>
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
i128
> for
NonZero
<
i128
>
Source
§
fn
try_from
(
    value:
i128
,
) ->
Result
<
NonZero
<
i128
>, <
NonZero
<
i128
> as
TryFrom
<
i128
>>::
Error
>
Attempts to convert
i128
to
NonZero
<
i128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
i16
> for
NonZero
<
i16
>
Source
§
fn
try_from
(
    value:
i16
,
) ->
Result
<
NonZero
<
i16
>, <
NonZero
<
i16
> as
TryFrom
<
i16
>>::
Error
>
Attempts to convert
i16
to
NonZero
<
i16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
i32
> for
NonZero
<
i32
>
Source
§
fn
try_from
(
    value:
i32
,
) ->
Result
<
NonZero
<
i32
>, <
NonZero
<
i32
> as
TryFrom
<
i32
>>::
Error
>
Attempts to convert
i32
to
NonZero
<
i32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
i64
> for
NonZero
<
i64
>
Source
§
fn
try_from
(
    value:
i64
,
) ->
Result
<
NonZero
<
i64
>, <
NonZero
<
i64
> as
TryFrom
<
i64
>>::
Error
>
Attempts to convert
i64
to
NonZero
<
i64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
i8
> for
NonZero
<
i8
>
Source
§
fn
try_from
(
    value:
i8
,
) ->
Result
<
NonZero
<
i8
>, <
NonZero
<
i8
> as
TryFrom
<
i8
>>::
Error
>
Attempts to convert
i8
to
NonZero
<
i8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
isize
> for
NonZero
<
isize
>
Source
§
fn
try_from
(
    value:
isize
,
) ->
Result
<
NonZero
<
isize
>, <
NonZero
<
isize
> as
TryFrom
<
isize
>>::
Error
>
Attempts to convert
isize
to
NonZero
<
isize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
u128
> for
NonZero
<
u128
>
Source
§
fn
try_from
(
    value:
u128
,
) ->
Result
<
NonZero
<
u128
>, <
NonZero
<
u128
> as
TryFrom
<
u128
>>::
Error
>
Attempts to convert
u128
to
NonZero
<
u128
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
u16
> for
NonZero
<
u16
>
Source
§
fn
try_from
(
    value:
u16
,
) ->
Result
<
NonZero
<
u16
>, <
NonZero
<
u16
> as
TryFrom
<
u16
>>::
Error
>
Attempts to convert
u16
to
NonZero
<
u16
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
u32
> for
NonZero
<
u32
>
Source
§
fn
try_from
(
    value:
u32
,
) ->
Result
<
NonZero
<
u32
>, <
NonZero
<
u32
> as
TryFrom
<
u32
>>::
Error
>
Attempts to convert
u32
to
NonZero
<
u32
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
u64
> for
NonZero
<
u64
>
Source
§
fn
try_from
(
    value:
u64
,
) ->
Result
<
NonZero
<
u64
>, <
NonZero
<
u64
> as
TryFrom
<
u64
>>::
Error
>
Attempts to convert
u64
to
NonZero
<
u64
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
u8
> for
NonZero
<
u8
>
Source
§
fn
try_from
(
    value:
u8
,
) ->
Result
<
NonZero
<
u8
>, <
NonZero
<
u8
> as
TryFrom
<
u8
>>::
Error
>
Attempts to convert
u8
to
NonZero
<
u8
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.46.0
·
Source
§
impl
TryFrom
<
usize
> for
NonZero
<
usize
>
Source
§
fn
try_from
(
    value:
usize
,
) ->
Result
<
NonZero
<
usize
>, <
NonZero
<
usize
> as
TryFrom
<
usize
>>::
Error
>
Attempts to convert
usize
to
NonZero
<
usize
>
.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.84.0
·
Source
§
impl<T>
UpperExp
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
UpperExp
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
1.28.0
·
Source
§
impl<T>
UpperHex
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
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
1.28.0
·
Source
§
impl<T>
Copy
for
NonZero
<T>
where
    T:
ZeroablePrimitive
,
1.28.0
·
Source
§
impl<T>
Eq
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Eq
,
1.28.0
·
Source
§
impl<T>
Freeze
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Freeze
,
1.28.0
·
Source
§
impl<T>
RefUnwindSafe
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
RefUnwindSafe
,
1.28.0
·
Source
§
impl<T>
Send
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Send
,
Source
§
impl<T>
StructuralPartialEq
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
StructuralPartialEq
,
1.28.0
·
Source
§
impl<T>
Sync
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Sync
,
1.28.0
·
Source
§
impl<T>
Unpin
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Unpin
,
1.28.0
·
Source
§
impl<T>
UnwindSafe
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
UnwindSafe
,
Source
§
impl<T>
UseCloned
for
NonZero
<T>
where
    T:
ZeroablePrimitive
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