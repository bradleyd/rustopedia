u8 - Rust
Primitive Type
u8
Copy item path
1.0.0
Expand description
The 8-bit unsigned integer type.
Implementations
§
Source
§
impl
u8
1.43.0
·
Source
pub const
MIN
:
u8
= 0u8
The smallest value that can be represented by this integer type.
§
Examples
Basic usage:
assert_eq!
(u8::MIN,
0
);
1.43.0
·
Source
pub const
MAX
:
u8
= 255u8
The largest value that can be represented by this integer type
(2
8
− 1).
§
Examples
Basic usage:
assert_eq!
(u8::MAX,
255
);
1.53.0
·
Source
pub const
BITS
:
u32
= 8u32
The size of this integer type in bits.
§
Examples
assert_eq!
(u8::BITS,
8
);
1.0.0 (const: 1.32.0)
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
let
n =
0b01001100u8
;
assert_eq!
(n.count_ones(),
3
);
let
max = u8::MAX;
assert_eq!
(max.count_ones(),
8
);
let
zero =
0u8
;
assert_eq!
(zero.count_ones(),
0
);
1.0.0 (const: 1.32.0)
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
let
zero =
0u8
;
assert_eq!
(zero.count_zeros(),
8
);
let
max = u8::MAX;
assert_eq!
(max.count_zeros(),
0
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
leading_zeros
(self) ->
u32
Returns the number of leading zeros in the binary representation of
self
.
Depending on what you’re doing with the value, you might also be interested in the
ilog2
function which returns a consistent number, even if the type widens.
§
Examples
Basic usage:
let
n = u8::MAX >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
let
zero =
0u8
;
assert_eq!
(zero.leading_zeros(),
8
);
let
max = u8::MAX;
assert_eq!
(max.leading_zeros(),
0
);
1.0.0 (const: 1.32.0)
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
§
Examples
Basic usage:
let
n =
0b0101000u8
;
assert_eq!
(n.trailing_zeros(),
3
);
let
zero =
0u8
;
assert_eq!
(zero.trailing_zeros(),
8
);
let
max = u8::MAX;
assert_eq!
(max.trailing_zeros(),
0
);
1.46.0 (const: 1.46.0)
·
Source
pub const fn
leading_ones
(self) ->
u32
Returns the number of leading ones in the binary representation of
self
.
§
Examples
Basic usage:
let
n = !(u8::MAX >>
2
);
assert_eq!
(n.leading_ones(),
2
);
let
zero =
0u8
;
assert_eq!
(zero.leading_ones(),
0
);
let
max = u8::MAX;
assert_eq!
(max.leading_ones(),
8
);
1.46.0 (const: 1.46.0)
·
Source
pub const fn
trailing_ones
(self) ->
u32
Returns the number of trailing ones in the binary representation
of
self
.
§
Examples
Basic usage:
let
n =
0b1010111u8
;
assert_eq!
(n.trailing_ones(),
3
);
let
zero =
0u8
;
assert_eq!
(zero.trailing_ones(),
0
);
let
max = u8::MAX;
assert_eq!
(max.trailing_ones(),
8
);
Source
pub const fn
isolate_most_significant_one
(self) ->
u8
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the most significant bit set, or
0
if
the input is
0
.
§
Examples
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
n: u8 =
0b_01100100
;
assert_eq!
(n.isolate_most_significant_one(),
0b_01000000
);
assert_eq!
(
0_u8
.isolate_most_significant_one(),
0
);
Source
pub const fn
isolate_least_significant_one
(self) ->
u8
🔬
This is a nightly-only experimental API. (
isolate_most_least_significant_one
#136909
)
Returns
self
with only the least significant bit set, or
0
if
the input is
0
.
§
Examples
Basic usage:
#![feature(isolate_most_least_significant_one)]
let
n: u8 =
0b_01100100
;
assert_eq!
(n.isolate_least_significant_one(),
0b_00000100
);
assert_eq!
(
0_u8
.isolate_least_significant_one(),
0
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
i8
Returns the bit pattern of
self
reinterpreted as a signed integer of the same size.
This produces the same result as an
as
cast, but ensures that the bit-width remains
the same.
§
Examples
Basic usage:
let
n = u8::MAX;
assert_eq!
(n.cast_signed(), -
1i8
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
u8
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
let
n =
0x82u8
;
let
m =
0xa
;
assert_eq!
(n.rotate_left(
2
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
u8
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
let
n =
0xau8
;
let
m =
0x82
;
assert_eq!
(n.rotate_right(
2
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
swap_bytes
(self) ->
u8
Reverses the byte order of the integer.
§
Examples
Basic usage:
let
n =
0x12u8
;
let
m = n.swap_bytes();
assert_eq!
(m,
0x12
);
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
u8
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
let
n =
0x12u8
;
let
m = n.reverse_bits();
assert_eq!
(m,
0x48
);
assert_eq!
(
0
,
0u8
.reverse_bits());
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_be
(x:
u8
) ->
u8
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au8
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(u8::from_be(n), n)
}
else
{
assert_eq!
(u8::from_be(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_le
(x:
u8
) ->
u8
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au8
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(u8::from_le(n), n)
}
else
{
assert_eq!
(u8::from_le(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_be
(self) ->
u8
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au8
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
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_le
(self) ->
u8
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au8
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
1.0.0 (const: 1.47.0)
·
Source
pub const fn
checked_add
(self, rhs:
u8
) ->
Option
<
u8
>
Checked integer addition. Computes
self + rhs
, returning
None
if overflow occurred.
§
Examples
Basic usage:
assert_eq!
((u8::MAX -
2
).checked_add(
1
),
Some
(u8::MAX -
1
));
assert_eq!
((u8::MAX -
2
).checked_add(
3
),
None
);
Source
pub const fn
strict_add
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer addition. Computes
self + rhs
, panicking
if overflow occurred.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
((u8::MAX -
2
).strict_add(
1
), u8::MAX -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (u8::MAX -
2
).strict_add(
3
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_add
(self, rhs:
u8
) ->
u8
Unchecked integer addition. Computes
self + rhs
, assuming overflow
cannot occur.
Calling
x.unchecked_add(y)
is semantically equivalent to calling
x.
checked_add
(y).
unwrap_unchecked
()
.
If you’re just trying to avoid the panic in debug mode, then
do not
use this.  Instead, you’re looking for
wrapping_add
.
§
Safety
This results in undefined behavior when
self + rhs > u8::MAX
or
self + rhs < u8::MIN
,
i.e. when
checked_add
would return
None
.
1.66.0 (const: 1.66.0)
·
Source
pub const fn
checked_add_signed
(self, rhs:
i8
) ->
Option
<
u8
>
Checked addition with a signed integer. Computes
self + rhs
,
returning
None
if overflow occurred.
§
Examples
Basic usage:
assert_eq!
(
1u8
.checked_add_signed(
2
),
Some
(
3
));
assert_eq!
(
1u8
.checked_add_signed(-
2
),
None
);
assert_eq!
((u8::MAX -
2
).checked_add_signed(
3
),
None
);
Source
pub const fn
strict_add_signed
(self, rhs:
i8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict addition with a signed integer. Computes
self + rhs
,
panicking if overflow occurred.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
1u8
.strict_add_signed(
2
),
3
);
The following panic because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
1u8
.strict_add_signed(-
2
);
ⓘ
#![feature(strict_overflow_ops)]
let _
= (u8::MAX -
2
).strict_add_signed(
3
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
checked_sub
(self, rhs:
u8
) ->
Option
<
u8
>
Checked integer subtraction. Computes
self - rhs
, returning
None
if overflow occurred.
§
Examples
Basic usage:
assert_eq!
(
1u8
.checked_sub(
1
),
Some
(
0
));
assert_eq!
(
0u8
.checked_sub(
1
),
None
);
Source
pub const fn
strict_sub
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer subtraction. Computes
self - rhs
, panicking if
overflow occurred.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
1u8
.strict_sub(
1
),
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
0u8
.strict_sub(
1
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_sub
(self, rhs:
u8
) ->
u8
Unchecked integer subtraction. Computes
self - rhs
, assuming overflow
cannot occur.
Calling
x.unchecked_sub(y)
is semantically equivalent to calling
x.
checked_sub
(y).
unwrap_unchecked
()
.
If you’re just trying to avoid the panic in debug mode, then
do not
use this.  Instead, you’re looking for
wrapping_sub
.
If you find yourself writing code like this:
if
foo >= bar {
// SAFETY: just checked it will not overflow
let
diff =
unsafe
{ foo.unchecked_sub(bar) };
// ... use diff ...
}
Consider changing it to
if let
Some
(diff) = foo.checked_sub(bar) {
// ... use diff ...
}
As that does exactly the same thing – including telling the optimizer
that the subtraction cannot overflow – but avoids needing
unsafe
.
§
Safety
This results in undefined behavior when
self - rhs > u8::MAX
or
self - rhs < u8::MIN
,
i.e. when
checked_sub
would return
None
.
Source
pub const fn
checked_sub_signed
(self, rhs:
i8
) ->
Option
<
u8
>
🔬
This is a nightly-only experimental API. (
mixed_integer_ops_unsigned_sub
#126043
)
Checked subtraction with a signed integer. Computes
self - rhs
,
returning
None
if overflow occurred.
§
Examples
Basic usage:
#![feature(mixed_integer_ops_unsigned_sub)]
assert_eq!
(
1u8
.checked_sub_signed(
2
),
None
);
assert_eq!
(
1u8
.checked_sub_signed(-
2
),
Some
(
3
));
assert_eq!
((u8::MAX -
2
).checked_sub_signed(-
4
),
None
);
Source
pub const fn
checked_signed_diff
(self, rhs:
u8
) ->
Option
<
i8
>
🔬
This is a nightly-only experimental API. (
unsigned_signed_diff
#126041
)
Checked integer subtraction. Computes
self - rhs
and checks if the result fits into an
i8
, returning
None
if overflow occurred.
§
Examples
Basic usage:
#![feature(unsigned_signed_diff)]
assert_eq!
(
10u8
.checked_signed_diff(
2
),
Some
(
8
));
assert_eq!
(
2u8
.checked_signed_diff(
10
),
Some
(-
8
));
assert_eq!
(u8::MAX.checked_signed_diff(i8::MAX
as
u8),
None
);
assert_eq!
((i8::MAX
as
u8).checked_signed_diff(u8::MAX),
Some
(i8::MIN));
assert_eq!
((i8::MAX
as
u8 +
1
).checked_signed_diff(
0
),
None
);
assert_eq!
(u8::MAX.checked_signed_diff(u8::MAX),
Some
(
0
));
1.0.0 (const: 1.47.0)
·
Source
pub const fn
checked_mul
(self, rhs:
u8
) ->
Option
<
u8
>
Checked integer multiplication. Computes
self * rhs
, returning
None
if overflow occurred.
§
Examples
Basic usage:
assert_eq!
(
5u8
.checked_mul(
1
),
Some
(
5
));
assert_eq!
(u8::MAX.checked_mul(
2
),
None
);
Source
pub const fn
strict_mul
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer multiplication. Computes
self * rhs
, panicking if
overflow occurred.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
5u8
.strict_mul(
1
),
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= u8::MAX.strict_mul(
2
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_mul
(self, rhs:
u8
) ->
u8
Unchecked integer multiplication. Computes
self * rhs
, assuming overflow
cannot occur.
Calling
x.unchecked_mul(y)
is semantically equivalent to calling
x.
checked_mul
(y).
unwrap_unchecked
()
.
If you’re just trying to avoid the panic in debug mode, then
do not
use this.  Instead, you’re looking for
wrapping_mul
.
§
Safety
This results in undefined behavior when
self * rhs > u8::MAX
or
self * rhs < u8::MIN
,
i.e. when
checked_mul
would return
None
.
1.0.0 (const: 1.52.0)
·
Source
pub const fn
checked_div
(self, rhs:
u8
) ->
Option
<
u8
>
Checked integer division. Computes
self / rhs
, returning
None
if
rhs == 0
.
§
Examples
Basic usage:
assert_eq!
(
128u8
.checked_div(
2
),
Some
(
64
));
assert_eq!
(
1u8
.checked_div(
0
),
None
);
Source
pub const fn
strict_div
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer division. Computes
self / rhs
.
Strict division on unsigned types is just normal division. There’s no
way overflow could ever happen. This function exists so that all
operations are accounted for in the strict operations.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
100u8
.strict_div(
10
),
10
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1u8
).strict_div(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_div_euclid
(self, rhs:
u8
) ->
Option
<
u8
>
Checked Euclidean division. Computes
self.div_euclid(rhs)
, returning
None
if
rhs == 0
.
§
Examples
Basic usage:
assert_eq!
(
128u8
.checked_div_euclid(
2
),
Some
(
64
));
assert_eq!
(
1u8
.checked_div_euclid(
0
),
None
);
Source
pub const fn
strict_div_euclid
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict Euclidean division. Computes
self.div_euclid(rhs)
.
Strict division on unsigned types is just normal division. There’s no
way overflow could ever happen. This function exists so that all
operations are accounted for in the strict operations. Since, for the
positive integers, all common definitions of division are equal, this
is exactly equal to
self.strict_div(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
100u8
.strict_div_euclid(
10
),
10
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1u8
).strict_div_euclid(
0
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem
(self, rhs:
u8
) ->
Option
<
u8
>
Checked integer remainder. Computes
self % rhs
, returning
None
if
rhs == 0
.
§
Examples
Basic usage:
assert_eq!
(
5u8
.checked_rem(
2
),
Some
(
1
));
assert_eq!
(
5u8
.checked_rem(
0
),
None
);
Source
pub const fn
strict_rem
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer remainder. Computes
self % rhs
.
Strict remainder calculation on unsigned types is just the regular
remainder calculation. There’s no way overflow could ever happen.
This function exists so that all operations are accounted for in the
strict operations.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
100u8
.strict_rem(
10
),
0
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
5u8
.strict_rem(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem_euclid
(self, rhs:
u8
) ->
Option
<
u8
>
Checked Euclidean modulo. Computes
self.rem_euclid(rhs)
, returning
None
if
rhs == 0
.
§
Examples
Basic usage:
assert_eq!
(
5u8
.checked_rem_euclid(
2
),
Some
(
1
));
assert_eq!
(
5u8
.checked_rem_euclid(
0
),
None
);
Source
pub const fn
strict_rem_euclid
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict Euclidean modulo. Computes
self.rem_euclid(rhs)
.
Strict modulo calculation on unsigned types is just the regular
remainder calculation. There’s no way overflow could ever happen.
This function exists so that all operations are accounted for in the
strict operations. Since, for the positive integers, all common
definitions of division are equal, this is exactly equal to
self.strict_rem(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
100u8
.strict_rem_euclid(
10
),
0
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
5u8
.strict_rem_euclid(
0
);
Source
pub const unsafe fn
unchecked_disjoint_bitor
(self, other:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
disjoint_bitor
#135758
)
Same value as
self | other
, but UB if any bit position is set in both inputs.
This is a situational micro-optimization for places where you’d rather
use addition on some platforms and bitwise or on other platforms, based
on exactly which instructions combine better with whatever else you’re
doing.  Note that there’s no reason to bother using this for places
where it’s clear from the operations involved that they can’t overlap.
For example, if you’re combining
u16
s into a
u32
with
((a as u32) << 16) | (b as u32)
, that’s fine, as the backend will
know those sides of the
|
are disjoint without needing help.
§
Examples
#![feature(disjoint_bitor)]
// SAFETY: `1` and `4` have no bits in common.
unsafe
{
assert_eq!
(
1_u8
.unchecked_disjoint_bitor(
4
),
5
);
}
§
Safety
Requires that
(self & other) == 0
, otherwise it’s immediate UB.
Equivalently, requires that
(self | other) == (self + other)
.
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog
(self, base:
u8
) ->
u32
Returns the logarithm of the number with respect to an arbitrary base,
rounded down.
This method might not be optimized owing to implementation details;
ilog2
can produce results more efficiently for base 2, and
ilog10
can produce results more efficiently for base 10.
§
Panics
This function will panic if
self
is zero, or if
base
is less than 2.
§
Examples
assert_eq!
(
5u8
.ilog(
5
),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog2
(self) ->
u32
Returns the base 2 logarithm of the number, rounded down.
§
Panics
This function will panic if
self
is zero.
§
Examples
assert_eq!
(
2u8
.ilog2(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog10
(self) ->
u32
Returns the base 10 logarithm of the number, rounded down.
§
Panics
This function will panic if
self
is zero.
§
Example
assert_eq!
(
10u8
.ilog10(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog
(self, base:
u8
) ->
Option
<
u32
>
Returns the logarithm of the number with respect to an arbitrary base,
rounded down.
Returns
None
if the number is zero, or if the base is not at least 2.
This method might not be optimized owing to implementation details;
checked_ilog2
can produce results more efficiently for base 2, and
checked_ilog10
can produce results more efficiently for base 10.
§
Examples
assert_eq!
(
5u8
.checked_ilog(
5
),
Some
(
1
));
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog2
(self) ->
Option
<
u32
>
Returns the base 2 logarithm of the number, rounded down.
Returns
None
if the number is zero.
§
Examples
assert_eq!
(
2u8
.checked_ilog2(),
Some
(
1
));
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog10
(self) ->
Option
<
u32
>
Returns the base 10 logarithm of the number, rounded down.
Returns
None
if the number is zero.
§
Examples
assert_eq!
(
10u8
.checked_ilog10(),
Some
(
1
));
1.7.0 (const: 1.47.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
u8
>
Checked negation. Computes
-self
, returning
None
unless
self == 0
.
Note that negating any positive integer will overflow.
§
Examples
Basic usage:
assert_eq!
(
0u8
.checked_neg(),
Some
(
0
));
assert_eq!
(
1u8
.checked_neg(),
None
);
Source
pub const fn
strict_neg
(self) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict negation. Computes
-self
, panicking unless
self == 0
.
Note that negating any positive integer will overflow.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
0u8
.strict_neg(),
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
1u8
.strict_neg();
1.7.0 (const: 1.47.0)
·
Source
pub const fn
checked_shl
(self, rhs:
u32
) ->
Option
<
u8
>
Checked shift left. Computes
self << rhs
, returning
None
if
rhs
is larger than or equal to the number of bits in
self
.
§
Examples
Basic usage:
assert_eq!
(
0x1u8
.checked_shl(
4
),
Some
(
0x10
));
assert_eq!
(
0x10u8
.checked_shl(
129
),
None
);
assert_eq!
(
0x10u8
.checked_shl(
7
),
Some
(
0
));
Source
pub const fn
strict_shl
(self, rhs:
u32
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict shift left. Computes
self << rhs
, panicking if
rhs
is larger
than or equal to the number of bits in
self
.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
0x1u8
.strict_shl(
4
),
0x10
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
0x10u8
.strict_shl(
129
);
Source
pub const unsafe fn
unchecked_shl
(self, rhs:
u32
) ->
u8
🔬
This is a nightly-only experimental API. (
unchecked_shifts
#85122
)
Unchecked shift left. Computes
self << rhs
, assuming that
rhs
is less than the number of bits in
self
.
§
Safety
This results in undefined behavior if
rhs
is larger than
or equal to the number of bits in
self
,
i.e. when
checked_shl
would return
None
.
1.87.0 (const: 1.87.0)
·
Source
pub const fn
unbounded_shl
(self, rhs:
u32
) ->
u8
Unbounded shift left. Computes
self << rhs
, without bounding the value of
rhs
.
If
rhs
is larger or equal to the number of bits in
self
,
the entire value is shifted out, and
0
is returned.
§
Examples
Basic usage:
assert_eq!
(
0x1u8
.unbounded_shl(
4
),
0x10
);
assert_eq!
(
0x1u8
.unbounded_shl(
129
),
0
);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
checked_shr
(self, rhs:
u32
) ->
Option
<
u8
>
Checked shift right. Computes
self >> rhs
, returning
None
if
rhs
is larger than or equal to the number of bits in
self
.
§
Examples
Basic usage:
assert_eq!
(
0x10u8
.checked_shr(
4
),
Some
(
0x1
));
assert_eq!
(
0x10u8
.checked_shr(
129
),
None
);
Source
pub const fn
strict_shr
(self, rhs:
u32
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict shift right. Computes
self >> rhs
, panicking
rhs
is
larger than or equal to the number of bits in
self
.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
0x10u8
.strict_shr(
4
),
0x1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
0x10u8
.strict_shr(
129
);
Source
pub const unsafe fn
unchecked_shr
(self, rhs:
u32
) ->
u8
🔬
This is a nightly-only experimental API. (
unchecked_shifts
#85122
)
Unchecked shift right. Computes
self >> rhs
, assuming that
rhs
is less than the number of bits in
self
.
§
Safety
This results in undefined behavior if
rhs
is larger than
or equal to the number of bits in
self
,
i.e. when
checked_shr
would return
None
.
1.87.0 (const: 1.87.0)
·
Source
pub const fn
unbounded_shr
(self, rhs:
u32
) ->
u8
Unbounded shift right. Computes
self >> rhs
, without bounding the value of
rhs
.
If
rhs
is larger or equal to the number of bits in
self
,
the entire value is shifted out, and
0
is returned.
§
Examples
Basic usage:
assert_eq!
(
0x10u8
.unbounded_shr(
4
),
0x1
);
assert_eq!
(
0x10u8
.unbounded_shr(
129
),
0
);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
checked_pow
(self, exp:
u32
) ->
Option
<
u8
>
Checked exponentiation. Computes
self.pow(exp)
, returning
None
if
overflow occurred.
§
Examples
Basic usage:
assert_eq!
(
2u8
.checked_pow(
5
),
Some
(
32
));
assert_eq!
(u8::MAX.checked_pow(
2
),
None
);
Source
pub const fn
strict_pow
(self, exp:
u32
) ->
u8
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict exponentiation. Computes
self.pow(exp)
, panicking if
overflow occurred.
§
Panics
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
2u8
.strict_pow(
5
),
32
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= u8::MAX.strict_pow(
2
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_add
(self, rhs:
u8
) ->
u8
Saturating integer addition. Computes
self + rhs
, saturating at
the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100u8
.saturating_add(
1
),
101
);
assert_eq!
(u8::MAX.saturating_add(
127
), u8::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_add_signed
(self, rhs:
i8
) ->
u8
Saturating addition with a signed integer. Computes
self + rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
1u8
.saturating_add_signed(
2
),
3
);
assert_eq!
(
1u8
.saturating_add_signed(-
2
),
0
);
assert_eq!
((u8::MAX -
2
).saturating_add_signed(
4
), u8::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
u8
) ->
u8
Saturating integer subtraction. Computes
self - rhs
, saturating
at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100u8
.saturating_sub(
27
),
73
);
assert_eq!
(
13u8
.saturating_sub(
127
),
0
);
Source
pub const fn
saturating_sub_signed
(self, rhs:
i8
) ->
u8
🔬
This is a nightly-only experimental API. (
mixed_integer_ops_unsigned_sub
#126043
)
Saturating integer subtraction. Computes
self
-
rhs
, saturating at
the numeric bounds instead of overflowing.
§
Examples
Basic usage:
#![feature(mixed_integer_ops_unsigned_sub)]
assert_eq!
(
1u8
.saturating_sub_signed(
2
),
0
);
assert_eq!
(
1u8
.saturating_sub_signed(-
2
),
3
);
assert_eq!
((u8::MAX -
2
).saturating_sub_signed(-
4
), u8::MAX);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
u8
) ->
u8
Saturating integer multiplication. Computes
self * rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
2u8
.saturating_mul(
10
),
20
);
assert_eq!
((u8::MAX).saturating_mul(
10
), u8::MAX);
1.58.0 (const: 1.58.0)
·
Source
pub const fn
saturating_div
(self, rhs:
u8
) ->
u8
Saturating integer division. Computes
self / rhs
, saturating at the
numeric bounds instead of overflowing.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
5u8
.saturating_div(
2
),
2
);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
saturating_pow
(self, exp:
u32
) ->
u8
Saturating integer exponentiation. Computes
self.pow(exp)
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
4u8
.saturating_pow(
3
),
64
);
assert_eq!
(u8::MAX.saturating_pow(
2
), u8::MAX);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_add
(self, rhs:
u8
) ->
u8
Wrapping (modular) addition. Computes
self + rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
200u8
.wrapping_add(
55
),
255
);
assert_eq!
(
200u8
.wrapping_add(u8::MAX),
199
);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_add_signed
(self, rhs:
i8
) ->
u8
Wrapping (modular) addition with a signed integer. Computes
self + rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
1u8
.wrapping_add_signed(
2
),
3
);
assert_eq!
(
1u8
.wrapping_add_signed(-
2
), u8::MAX);
assert_eq!
((u8::MAX -
2
).wrapping_add_signed(
4
),
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_sub
(self, rhs:
u8
) ->
u8
Wrapping (modular) subtraction. Computes
self - rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100u8
.wrapping_sub(
100
),
0
);
assert_eq!
(
100u8
.wrapping_sub(u8::MAX),
101
);
Source
pub const fn
wrapping_sub_signed
(self, rhs:
i8
) ->
u8
🔬
This is a nightly-only experimental API. (
mixed_integer_ops_unsigned_sub
#126043
)
Wrapping (modular) subtraction with a signed integer. Computes
self - rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
#![feature(mixed_integer_ops_unsigned_sub)]
assert_eq!
(
1u8
.wrapping_sub_signed(
2
), u8::MAX);
assert_eq!
(
1u8
.wrapping_sub_signed(-
2
),
3
);
assert_eq!
((u8::MAX -
2
).wrapping_sub_signed(-
4
),
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_mul
(self, rhs:
u8
) ->
u8
Wrapping (modular) multiplication. Computes
self * rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
Please note that this example is shared between integer types.
Which explains why
u8
is used here.
assert_eq!
(
10u8
.wrapping_mul(
12
),
120
);
assert_eq!
(
25u8
.wrapping_mul(
12
),
44
);
1.2.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_div
(self, rhs:
u8
) ->
u8
Wrapping (modular) division. Computes
self / rhs
.
Wrapped division on unsigned types is just normal division. There’s
no way wrapping could ever happen. This function exists so that all
operations are accounted for in the wrapping operations.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
100u8
.wrapping_div(
10
),
10
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_div_euclid
(self, rhs:
u8
) ->
u8
Wrapping Euclidean division. Computes
self.div_euclid(rhs)
.
Wrapped division on unsigned types is just normal division. There’s
no way wrapping could ever happen. This function exists so that all
operations are accounted for in the wrapping operations. Since, for
the positive integers, all common definitions of division are equal,
this is exactly equal to
self.wrapping_div(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
100u8
.wrapping_div_euclid(
10
),
10
);
1.2.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_rem
(self, rhs:
u8
) ->
u8
Wrapping (modular) remainder. Computes
self % rhs
.
Wrapped remainder calculation on unsigned types is just the regular
remainder calculation. There’s no way wrapping could ever happen.
This function exists so that all operations are accounted for in the
wrapping operations.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
100u8
.wrapping_rem(
10
),
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_rem_euclid
(self, rhs:
u8
) ->
u8
Wrapping Euclidean modulo. Computes
self.rem_euclid(rhs)
.
Wrapped modulo calculation on unsigned types is just the regular
remainder calculation. There’s no way wrapping could ever happen.
This function exists so that all operations are accounted for in the
wrapping operations. Since, for the positive integers, all common
definitions of division are equal, this is exactly equal to
self.wrapping_rem(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
100u8
.wrapping_rem_euclid(
10
),
0
);
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_neg
(self) ->
u8
Wrapping (modular) negation. Computes
-self
,
wrapping around at the boundary of the type.
Since unsigned types do not have negative equivalents
all applications of this function will wrap (except for
-0
).
For values smaller than the corresponding signed type’s maximum
the result is the same as casting the corresponding signed value.
Any larger values are equivalent to
MAX + 1 - (val - MAX - 1)
where
MAX
is the corresponding signed type’s maximum.
§
Examples
Basic usage:
assert_eq!
(
0_u8
.wrapping_neg(),
0
);
assert_eq!
(u8::MAX.wrapping_neg(),
1
);
assert_eq!
(
13_u8
.wrapping_neg(), (!
13
) +
1
);
assert_eq!
(
42_u8
.wrapping_neg(), !(
42
-
1
));
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_shl
(self, rhs:
u32
) ->
u8
Panic-free bitwise shift-left; yields
self << mask(rhs)
,
where
mask
removes any high-order bits of
rhs
that
would cause the shift to exceed the bitwidth of the type.
Note that this is
not
the same as a rotate-left; the
RHS of a wrapping shift-left is restricted to the range
of the type, rather than the bits shifted out of the LHS
being returned to the other end. The primitive integer
types all implement a
rotate_left
function,
which may be what you want instead.
§
Examples
Basic usage:
assert_eq!
(
1u8
.wrapping_shl(
7
),
128
);
assert_eq!
(
1u8
.wrapping_shl(
128
),
1
);
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_shr
(self, rhs:
u32
) ->
u8
Panic-free bitwise shift-right; yields
self >> mask(rhs)
,
where
mask
removes any high-order bits of
rhs
that
would cause the shift to exceed the bitwidth of the type.
Note that this is
not
the same as a rotate-right; the
RHS of a wrapping shift-right is restricted to the range
of the type, rather than the bits shifted out of the LHS
being returned to the other end. The primitive integer
types all implement a
rotate_right
function,
which may be what you want instead.
§
Examples
Basic usage:
assert_eq!
(
128u8
.wrapping_shr(
7
),
1
);
assert_eq!
(
128u8
.wrapping_shr(
128
),
128
);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
wrapping_pow
(self, exp:
u32
) ->
u8
Wrapping (modular) exponentiation. Computes
self.pow(exp)
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
3u8
.wrapping_pow(
5
),
243
);
assert_eq!
(
3u8
.wrapping_pow(
6
),
217
);
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_add
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates
self
+
rhs
.
Returns a tuple of the addition along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_add(
2
), (
7
,
false
));
assert_eq!
(u8::MAX.overflowing_add(
1
), (
0
,
true
));
Source
pub const fn
carrying_add
(self, rhs:
u8
, carry:
bool
) -> (
u8
,
bool
)
🔬
This is a nightly-only experimental API. (
bigint_helper_methods
#85532
)
Calculates
self
+
rhs
+
carry
and returns a tuple containing
the sum and the output carry.
Performs “ternary addition” of two integer operands and a carry-in
bit, and returns an output integer and a carry-out bit. This allows
chaining together multiple additions to create a wider addition, and
can be useful for bignum addition.
This can be thought of as a 8-bit “full adder”, in the electronics sense.
If the input carry is false, this method is equivalent to
overflowing_add
, and the output carry is
equal to the overflow flag. Note that although carry and overflow
flags are similar for unsigned integers, they are different for
signed integers.
§
Examples
#![feature(bigint_helper_methods)]
//    3  MAX    (a = 3 × 2^8 + 2^8 - 1)
// +  5    7    (b = 5 × 2^8 + 7)
// ---------
//    9    6    (sum = 9 × 2^8 + 6)
let
(a1, a0): (u8, u8) = (
3
, u8::MAX);
let
(b1, b0): (u8, u8) = (
5
,
7
);
let
carry0 =
false
;
let
(sum0, carry1) = a0.carrying_add(b0, carry0);
assert_eq!
(carry1,
true
);
let
(sum1, carry2) = a1.carrying_add(b1, carry1);
assert_eq!
(carry2,
false
);
assert_eq!
((sum1, sum0), (
9
,
6
));
1.66.0 (const: 1.66.0)
·
Source
pub const fn
overflowing_add_signed
(self, rhs:
i8
) -> (
u8
,
bool
)
Calculates
self
+
rhs
with a signed
rhs
.
Returns a tuple of the addition along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
1u8
.overflowing_add_signed(
2
), (
3
,
false
));
assert_eq!
(
1u8
.overflowing_add_signed(-
2
), (u8::MAX,
true
));
assert_eq!
((u8::MAX -
2
).overflowing_add_signed(
4
), (
1
,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_sub
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates
self
-
rhs
.
Returns a tuple of the subtraction along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_sub(
2
), (
3
,
false
));
assert_eq!
(
0u8
.overflowing_sub(
1
), (u8::MAX,
true
));
Source
pub const fn
borrowing_sub
(self, rhs:
u8
, borrow:
bool
) -> (
u8
,
bool
)
🔬
This is a nightly-only experimental API. (
bigint_helper_methods
#85532
)
Calculates
self
−
rhs
−
borrow
and returns a tuple
containing the difference and the output borrow.
Performs “ternary subtraction” by subtracting both an integer
operand and a borrow-in bit from
self
, and returns an output
integer and a borrow-out bit. This allows chaining together multiple
subtractions to create a wider subtraction, and can be useful for
bignum subtraction.
§
Examples
#![feature(bigint_helper_methods)]
//    9    6    (a = 9 × 2^8 + 6)
// -  5    7    (b = 5 × 2^8 + 7)
// ---------
//    3  MAX    (diff = 3 × 2^8 + 2^8 - 1)
let
(a1, a0): (u8, u8) = (
9
,
6
);
let
(b1, b0): (u8, u8) = (
5
,
7
);
let
borrow0 =
false
;
let
(diff0, borrow1) = a0.borrowing_sub(b0, borrow0);
assert_eq!
(borrow1,
true
);
let
(diff1, borrow2) = a1.borrowing_sub(b1, borrow1);
assert_eq!
(borrow2,
false
);
assert_eq!
((diff1, diff0), (
3
, u8::MAX));
Source
pub const fn
overflowing_sub_signed
(self, rhs:
i8
) -> (
u8
,
bool
)
🔬
This is a nightly-only experimental API. (
mixed_integer_ops_unsigned_sub
#126043
)
Calculates
self
-
rhs
with a signed
rhs
Returns a tuple of the subtraction along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would
have occurred then the wrapped value is returned.
§
Examples
Basic usage:
#![feature(mixed_integer_ops_unsigned_sub)]
assert_eq!
(
1u8
.overflowing_sub_signed(
2
), (u8::MAX,
true
));
assert_eq!
(
1u8
.overflowing_sub_signed(-
2
), (
3
,
false
));
assert_eq!
((u8::MAX -
2
).overflowing_sub_signed(-
4
), (
1
,
true
));
1.60.0 (const: 1.60.0)
·
Source
pub const fn
abs_diff
(self, other:
u8
) ->
u8
Computes the absolute difference between
self
and
other
.
§
Examples
Basic usage:
assert_eq!
(
100u8
.abs_diff(
80
),
20u8
);
assert_eq!
(
100u8
.abs_diff(
110
),
10u8
);
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_mul
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates the multiplication of
self
and
rhs
.
Returns a tuple of the multiplication along with a boolean
indicating whether an arithmetic overflow would occur. If an
overflow would have occurred then the wrapped value is returned.
§
Examples
Basic usage:
Please note that this example is shared between integer types.
Which explains why
u32
is used here.
assert_eq!
(
5u32
.overflowing_mul(
2
), (
10
,
false
));
assert_eq!
(
1_000_000_000u32
.overflowing_mul(
10
), (
1410065408
,
true
));
Source
pub const fn
widening_mul
(self, rhs:
u8
) -> (
u8
,
u8
)
🔬
This is a nightly-only experimental API. (
bigint_helper_methods
#85532
)
Calculates the complete product
self * rhs
without the possibility to overflow.
This returns the low-order (wrapping) bits and the high-order (overflow) bits
of the result as two separate values, in that order.
If you also need to add a carry to the wide result, then you want
Self::carrying_mul
instead.
§
Examples
Basic usage:
Please note that this example is shared between integer types.
Which explains why
u32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5u32
.widening_mul(
2
), (
10
,
0
));
assert_eq!
(
1_000_000_000u32
.widening_mul(
10
), (
1410065408
,
2
));
Source
pub const fn
carrying_mul
(self, rhs:
u8
, carry:
u8
) -> (
u8
,
u8
)
🔬
This is a nightly-only experimental API. (
bigint_helper_methods
#85532
)
Calculates the “full multiplication”
self * rhs + carry
without the possibility to overflow.
This returns the low-order (wrapping) bits and the high-order (overflow) bits
of the result as two separate values, in that order.
Performs “long multiplication” which takes in an extra amount to add, and may return an
additional amount of overflow. This allows for chaining together multiple
multiplications to create “big integers” which represent larger values.
If you don’t need the
carry
, then you can use
Self::widening_mul
instead.
§
Examples
Basic usage:
Please note that this example is shared between integer types.
Which explains why
u32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5u32
.carrying_mul(
2
,
0
), (
10
,
0
));
assert_eq!
(
5u32
.carrying_mul(
2
,
10
), (
20
,
0
));
assert_eq!
(
1_000_000_000u32
.carrying_mul(
10
,
0
), (
1410065408
,
2
));
assert_eq!
(
1_000_000_000u32
.carrying_mul(
10
,
10
), (
1410065418
,
2
));
assert_eq!
(u8::MAX.carrying_mul(u8::MAX, u8::MAX), (
0
, u8::MAX));
This is the core operation needed for scalar multiplication when
implementing it for wider-than-native types.
#![feature(bigint_helper_methods)]
fn
scalar_mul_eq(little_endian_digits:
&mut
Vec<u16>, multiplicand: u16) {
let
mut
carry =
0
;
for
d
in
little_endian_digits.iter_mut() {
        (
*
d, carry) = d.carrying_mul(multiplicand, carry);
    }
if
carry !=
0
{
        little_endian_digits.push(carry);
    }
}
let
mut
v =
vec!
[
10
,
20
];
scalar_mul_eq(
&mut
v,
3
);
assert_eq!
(v, [
30
,
60
]);
assert_eq!
(
0x87654321_u64
*
0xFEED
,
0x86D3D159E38D
);
let
mut
v =
vec!
[
0x4321
,
0x8765
];
scalar_mul_eq(
&mut
v,
0xFEED
);
assert_eq!
(v, [
0xE38D
,
0xD159
,
0x86D3
]);
If
carry
is zero, this is similar to
overflowing_mul
,
except that it gives the value of the overflow instead of just whether one happened:
#![feature(bigint_helper_methods)]
let
r = u8::carrying_mul(
7
,
13
,
0
);
assert_eq!
((r.
0
, r.
1
!=
0
), u8::overflowing_mul(
7
,
13
));
let
r = u8::carrying_mul(
13
,
42
,
0
);
assert_eq!
((r.
0
, r.
1
!=
0
), u8::overflowing_mul(
13
,
42
));
The value of the first field in the returned tuple matches what you’d get
by combining the
wrapping_mul
and
wrapping_add
methods:
#![feature(bigint_helper_methods)]
assert_eq!
(
789_u16
.carrying_mul(
456
,
123
).
0
,
789_u16
.wrapping_mul(
456
).wrapping_add(
123
),
);
Source
pub const fn
carrying_mul_add
(self, rhs:
u8
, carry:
u8
, add:
u8
) -> (
u8
,
u8
)
🔬
This is a nightly-only experimental API. (
bigint_helper_methods
#85532
)
Calculates the “full multiplication”
self * rhs + carry1 + carry2
without the possibility to overflow.
This returns the low-order (wrapping) bits and the high-order (overflow) bits
of the result as two separate values, in that order.
Performs “long multiplication” which takes in an extra amount to add, and may return an
additional amount of overflow. This allows for chaining together multiple
multiplications to create “big integers” which represent larger values.
If you don’t need either
carry
, then you can use
Self::widening_mul
instead,
and if you only need one
carry
, then you can use
Self::carrying_mul
instead.
§
Examples
Basic usage:
Please note that this example is shared between integer types,
which explains why
u32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5u32
.carrying_mul_add(
2
,
0
,
0
), (
10
,
0
));
assert_eq!
(
5u32
.carrying_mul_add(
2
,
10
,
10
), (
30
,
0
));
assert_eq!
(
1_000_000_000u32
.carrying_mul_add(
10
,
0
,
0
), (
1410065408
,
2
));
assert_eq!
(
1_000_000_000u32
.carrying_mul_add(
10
,
10
,
10
), (
1410065428
,
2
));
assert_eq!
(u8::MAX.carrying_mul_add(u8::MAX, u8::MAX, u8::MAX), (u8::MAX, u8::MAX));
This is the core per-digit operation for “grade school” O(n²) multiplication.
Please note that this example is shared between integer types,
using
u8
for simplicity of the demonstration.
#![feature(bigint_helper_methods)]
fn
quadratic_mul<
const
N: usize>(a: [u8; N], b: [u8; N]) -> [u8; N] {
let
mut
out = [
0
; N];
for
j
in
0
..N {
let
mut
carry =
0
;
for
i
in
0
..(N - j) {
            (out[j + i], carry) = u8::carrying_mul_add(a[i], b[j], out[j + i], carry);
        }
    }
    out
}
// -1 * -1 == 1
assert_eq!
(quadratic_mul([
0xFF
;
3
], [
0xFF
;
3
]), [
1
,
0
,
0
]);
assert_eq!
(u32::wrapping_mul(
0x9e3779b9
,
0x7f4a7c15
),
0xCFFC982D
);
assert_eq!
(
    quadratic_mul(u32::to_le_bytes(
0x9e3779b9
), u32::to_le_bytes(
0x7f4a7c15
)),
    u32::to_le_bytes(
0xCFFC982D
)
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates the divisor when
self
is divided by
rhs
.
Returns a tuple of the divisor along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned
integers overflow never occurs, so the second value is always
false
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_div(
2
), (
2
,
false
));
1.38.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div_euclid
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates the quotient of Euclidean division
self.div_euclid(rhs)
.
Returns a tuple of the divisor along with a boolean indicating
whether an arithmetic overflow would occur. Note that for unsigned
integers overflow never occurs, so the second value is always
false
.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to
self.overflowing_div(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_div_euclid(
2
), (
2
,
false
));
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_rem
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates the remainder when
self
is divided by
rhs
.
Returns a tuple of the remainder after dividing along with a boolean
indicating whether an arithmetic overflow would occur. Note that for
unsigned integers overflow never occurs, so the second value is
always
false
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_rem(
2
), (
1
,
false
));
1.38.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_rem_euclid
(self, rhs:
u8
) -> (
u8
,
bool
)
Calculates the remainder
self.rem_euclid(rhs)
as if by Euclidean division.
Returns a tuple of the modulo after dividing along with a boolean
indicating whether an arithmetic overflow would occur. Note that for
unsigned integers overflow never occurs, so the second value is
always
false
.
Since, for the positive integers, all common
definitions of division are equal, this operation
is exactly equal to
self.overflowing_rem(rhs)
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
5u8
.overflowing_rem_euclid(
2
), (
1
,
false
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
u8
,
bool
)
Negates self in an overflowing fashion.
Returns
!self + 1
using wrapping operations to return the value
that represents the negation of this unsigned value. Note that for
positive unsigned values overflow always occurs, but negating 0 does
not overflow.
§
Examples
Basic usage:
assert_eq!
(
0u8
.overflowing_neg(), (
0
,
false
));
assert_eq!
(
2u8
.overflowing_neg(), (-
2i32
as
u8,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_shl
(self, rhs:
u32
) -> (
u8
,
bool
)
Shifts self left by
rhs
bits.
Returns a tuple of the shifted version of self along with a boolean
indicating whether the shift value was larger than or equal to the
number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then
used to perform the shift.
§
Examples
Basic usage:
assert_eq!
(
0x1u8
.overflowing_shl(
4
), (
0x10
,
false
));
assert_eq!
(
0x1u8
.overflowing_shl(
132
), (
0x10
,
true
));
assert_eq!
(
0x10u8
.overflowing_shl(
7
), (
0
,
false
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_shr
(self, rhs:
u32
) -> (
u8
,
bool
)
Shifts self right by
rhs
bits.
Returns a tuple of the shifted version of self along with a boolean
indicating whether the shift value was larger than or equal to the
number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then
used to perform the shift.
§
Examples
Basic usage:
assert_eq!
(
0x10u8
.overflowing_shr(
4
), (
0x1
,
false
));
assert_eq!
(
0x10u8
.overflowing_shr(
132
), (
0x1
,
true
));
1.34.0 (const: 1.50.0)
·
Source
pub const fn
overflowing_pow
(self, exp:
u32
) -> (
u8
,
bool
)
Raises self to the power of
exp
, using exponentiation by squaring.
Returns a tuple of the exponentiation along with a bool indicating
whether an overflow happened.
§
Examples
Basic usage:
assert_eq!
(
3u8
.overflowing_pow(
5
), (
243
,
false
));
assert_eq!
(
3u8
.overflowing_pow(
6
), (
217
,
true
));
1.0.0 (const: 1.50.0)
·
Source
pub const fn
pow
(self, exp:
u32
) ->
u8
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
assert_eq!
(
2u8
.pow(
5
),
32
);
1.84.0 (const: 1.84.0)
·
Source
pub const fn
isqrt
(self) ->
u8
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
assert_eq!
(
10u8
.isqrt(),
3
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
div_euclid
(self, rhs:
u8
) ->
u8
Performs Euclidean division.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to
self / rhs
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
7u8
.div_euclid(
4
),
1
);
// or any other integer type
1.38.0 (const: 1.52.0)
·
Source
pub const fn
rem_euclid
(self, rhs:
u8
) ->
u8
Calculates the least remainder of
self (mod rhs)
.
Since, for the positive integers, all common
definitions of division are equal, this
is exactly equal to
self % rhs
.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
7u8
.rem_euclid(
4
),
3
);
// or any other integer type
Source
pub const fn
div_floor
(self, rhs:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
int_roundings
#88581
)
Calculates the quotient of
self
and
rhs
, rounding the result towards negative infinity.
This is the same as performing
self / rhs
for all unsigned integers.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
#![feature(int_roundings)]
assert_eq!
(
7_u8
.div_floor(
4
),
1
);
1.73.0 (const: 1.73.0)
·
Source
pub const fn
div_ceil
(self, rhs:
u8
) ->
u8
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
§
Panics
This function will panic if
rhs
is zero.
§
Examples
Basic usage:
assert_eq!
(
7_u8
.div_ceil(
4
),
2
);
1.73.0 (const: 1.73.0)
·
Source
pub const fn
next_multiple_of
(self, rhs:
u8
) ->
u8
Calculates the smallest value greater than or equal to
self
that
is a multiple of
rhs
.
§
Panics
This function will panic if
rhs
is zero.
§
Overflow behavior
On overflow, this function will panic if overflow checks are enabled (default in debug
mode) and wrap if overflow checks are disabled (default in release mode).
§
Examples
Basic usage:
assert_eq!
(
16_u8
.next_multiple_of(
8
),
16
);
assert_eq!
(
23_u8
.next_multiple_of(
8
),
24
);
1.73.0 (const: 1.73.0)
·
Source
pub const fn
checked_next_multiple_of
(self, rhs:
u8
) ->
Option
<
u8
>
Calculates the smallest value greater than or equal to
self
that
is a multiple of
rhs
. Returns
None
if
rhs
is zero or the
operation would result in overflow.
§
Examples
Basic usage:
assert_eq!
(
16_u8
.checked_next_multiple_of(
8
),
Some
(
16
));
assert_eq!
(
23_u8
.checked_next_multiple_of(
8
),
Some
(
24
));
assert_eq!
(
1_u8
.checked_next_multiple_of(
0
),
None
);
assert_eq!
(u8::MAX.checked_next_multiple_of(
2
),
None
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
is_multiple_of
(self, rhs:
u8
) ->
bool
Returns
true
if
self
is an integer multiple of
rhs
, and false otherwise.
This function is equivalent to
self % rhs == 0
, except that it will not panic
for
rhs == 0
. Instead,
0.is_multiple_of(0) == true
, and for any non-zero
n
,
n.is_multiple_of(0) == false
.
§
Examples
Basic usage:
assert!
(
6_u8
.is_multiple_of(
2
));
assert!
(!
5_u8
.is_multiple_of(
2
));
assert!
(
0_u8
.is_multiple_of(
0
));
assert!
(!
6_u8
.is_multiple_of(
0
));
1.0.0 (const: 1.32.0)
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
assert!
(
16u8
.is_power_of_two());
assert!
(!
10u8
.is_power_of_two());
1.0.0 (const: 1.50.0)
·
Source
pub const fn
next_power_of_two
(self) ->
u8
Returns the smallest power of two greater than or equal to
self
.
When return value overflows (i.e.,
self > (1 << (N-1))
for type
uN
), it panics in debug mode and the return value is wrapped to 0 in
release mode (the only situation in which this method can return 0).
§
Examples
Basic usage:
assert_eq!
(
2u8
.next_power_of_two(),
2
);
assert_eq!
(
3u8
.next_power_of_two(),
4
);
assert_eq!
(
0u8
.next_power_of_two(),
1
);
1.0.0 (const: 1.50.0)
·
Source
pub const fn
checked_next_power_of_two
(self) ->
Option
<
u8
>
Returns the smallest power of two greater than or equal to
self
. If
the next power of two is greater than the type’s maximum value,
None
is returned, otherwise the power of two is wrapped in
Some
.
§
Examples
Basic usage:
assert_eq!
(
2u8
.checked_next_power_of_two(),
Some
(
2
));
assert_eq!
(
3u8
.checked_next_power_of_two(),
Some
(
4
));
assert_eq!
(u8::MAX.checked_next_power_of_two(),
None
);
Source
pub const fn
wrapping_next_power_of_two
(self) ->
u8
🔬
This is a nightly-only experimental API. (
wrapping_next_power_of_two
#32463
)
Returns the smallest power of two greater than or equal to
n
. If
the next power of two is greater than the type’s maximum value,
the return value is wrapped to
0
.
§
Examples
Basic usage:
#![feature(wrapping_next_power_of_two)]
assert_eq!
(
2u8
.wrapping_next_power_of_two(),
2
);
assert_eq!
(
3u8
.wrapping_next_power_of_two(),
4
);
assert_eq!
(u8::MAX.wrapping_next_power_of_two(),
0
);
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_be_bytes
(self) -> [
u8
;
1
]
Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
bytes =
0x12u8
.to_be_bytes();
assert_eq!
(bytes, [
0x12
]);
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_le_bytes
(self) -> [
u8
;
1
]
Returns the memory representation of this integer as a byte array in
little-endian byte order.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
bytes =
0x12u8
.to_le_bytes();
assert_eq!
(bytes, [
0x12
]);
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_ne_bytes
(self) -> [
u8
;
1
]
Returns the memory representation of this integer as a byte array in
native byte order.
As the target platform’s native endianness is used, portable code
should use
to_be_bytes
or
to_le_bytes
, as appropriate,
instead.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
bytes =
0x12u8
.to_ne_bytes();
assert_eq!
(
    bytes,
if
cfg!
(target_endian =
"big"
) {
        [
0x12
]
    }
else
{
        [
0x12
]
    }
);
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_be_bytes
(bytes: [
u8
;
1
]) ->
u8
Creates a native endian integer value from its representation
as a byte array in big endian.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
value = u8::from_be_bytes([
0x12
]);
assert_eq!
(value,
0x12
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_be_u8(input:
&mut &
[u8]) -> u8 {
let
(int_bytes, rest) = input.split_at(size_of::<u8>());
*
input = rest;
    u8::from_be_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_le_bytes
(bytes: [
u8
;
1
]) ->
u8
Creates a native endian integer value from its representation
as a byte array in little endian.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
value = u8::from_le_bytes([
0x12
]);
assert_eq!
(value,
0x12
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_le_u8(input:
&mut &
[u8]) -> u8 {
let
(int_bytes, rest) = input.split_at(size_of::<u8>());
*
input = rest;
    u8::from_le_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_ne_bytes
(bytes: [
u8
;
1
]) ->
u8
Creates a native endian integer value from its memory representation
as a byte array in native endianness.
As the target platform’s native endianness is used, portable code
likely wants to use
from_be_bytes
or
from_le_bytes
, as
appropriate instead.
Note
: This function is meaningless on
u8
. Byte order does not exist as a
concept for byte-sized integers. This function is only provided in symmetry
with larger integer types.
§
Examples
let
value = u8::from_ne_bytes(
if
cfg!
(target_endian =
"big"
) {
    [
0x12
]
}
else
{
    [
0x12
]
});
assert_eq!
(value,
0x12
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_ne_u8(input:
&mut &
[u8]) -> u8 {
let
(int_bytes, rest) = input.split_at(size_of::<u8>());
*
input = rest;
    u8::from_ne_bytes(int_bytes.try_into().unwrap())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
min_value
() ->
u8
👎
Deprecating in a future version: replaced by the
MIN
associated constant on this type
New code should prefer to use
u8::MIN
instead.
Returns the smallest value that can be represented by this integer type.
1.0.0 (const: 1.32.0)
·
Source
pub const fn
max_value
() ->
u8
👎
Deprecating in a future version: replaced by the
MAX
associated constant on this type
New code should prefer to use
u8::MAX
instead.
Returns the largest value that can be represented by this integer type.
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
u8
) ->
u8
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) / 2
as if it were performed in a
sufficiently-large unsigned integral type. This implies that the result is
always rounded towards zero and that no overflow will ever occur.
§
Examples
assert_eq!
(
0u8
.midpoint(
4
),
2
);
assert_eq!
(
1u8
.midpoint(
4
),
2
);
1.23.0 (const: 1.43.0)
·
Source
pub const fn
is_ascii
(&self) ->
bool
Checks if the value is within the ASCII range.
§
Examples
let
ascii =
97u8
;
let
non_ascii =
150u8
;
assert!
(ascii.is_ascii());
assert!
(!non_ascii.is_ascii());
Source
pub const fn
as_ascii
(&self) ->
Option
<
AsciiChar
>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
If the value of this byte is within the ASCII range, returns it as an
ASCII character
.  Otherwise, returns
None
.
1.23.0 (const: 1.52.0)
·
Source
pub const fn
to_ascii_uppercase
(&self) ->
u8
Makes a copy of the value in its ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
make_ascii_uppercase
.
§
Examples
let
lowercase_a =
97u8
;
assert_eq!
(
65
, lowercase_a.to_ascii_uppercase());
1.23.0 (const: 1.52.0)
·
Source
pub const fn
to_ascii_lowercase
(&self) ->
u8
Makes a copy of the value in its ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
make_ascii_lowercase
.
§
Examples
let
uppercase_a =
65u8
;
assert_eq!
(
97
, uppercase_a.to_ascii_lowercase());
1.23.0 (const: 1.52.0)
·
Source
pub const fn
eq_ignore_ascii_case
(&self, other: &
u8
) ->
bool
Checks that two values are an ASCII case-insensitive match.
This is equivalent to
to_ascii_lowercase(a) == to_ascii_lowercase(b)
.
§
Examples
let
lowercase_a =
97u8
;
let
uppercase_a =
65u8
;
assert!
(lowercase_a.eq_ignore_ascii_case(
&
uppercase_a));
1.23.0 (const: 1.84.0)
·
Source
pub const fn
make_ascii_uppercase
(&mut self)
Converts this value to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
to_ascii_uppercase
.
§
Examples
let
mut
byte =
b'a'
;

byte.make_ascii_uppercase();
assert_eq!
(
b'A'
, byte);
1.23.0 (const: 1.84.0)
·
Source
pub const fn
make_ascii_lowercase
(&mut self)
Converts this value to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
to_ascii_lowercase
.
§
Examples
let
mut
byte =
b'A'
;

byte.make_ascii_lowercase();
assert_eq!
(
b'a'
, byte);
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_alphabetic
(&self) ->
bool
Checks if the value is an ASCII alphabetic character:
U+0041 ‘A’ ..= U+005A ‘Z’, or
U+0061 ‘a’ ..= U+007A ‘z’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(uppercase_a.is_ascii_alphabetic());
assert!
(uppercase_g.is_ascii_alphabetic());
assert!
(a.is_ascii_alphabetic());
assert!
(g.is_ascii_alphabetic());
assert!
(!zero.is_ascii_alphabetic());
assert!
(!percent.is_ascii_alphabetic());
assert!
(!space.is_ascii_alphabetic());
assert!
(!lf.is_ascii_alphabetic());
assert!
(!esc.is_ascii_alphabetic());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_uppercase
(&self) ->
bool
Checks if the value is an ASCII uppercase character:
U+0041 ‘A’ ..= U+005A ‘Z’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(uppercase_a.is_ascii_uppercase());
assert!
(uppercase_g.is_ascii_uppercase());
assert!
(!a.is_ascii_uppercase());
assert!
(!g.is_ascii_uppercase());
assert!
(!zero.is_ascii_uppercase());
assert!
(!percent.is_ascii_uppercase());
assert!
(!space.is_ascii_uppercase());
assert!
(!lf.is_ascii_uppercase());
assert!
(!esc.is_ascii_uppercase());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_lowercase
(&self) ->
bool
Checks if the value is an ASCII lowercase character:
U+0061 ‘a’ ..= U+007A ‘z’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(!uppercase_a.is_ascii_lowercase());
assert!
(!uppercase_g.is_ascii_lowercase());
assert!
(a.is_ascii_lowercase());
assert!
(g.is_ascii_lowercase());
assert!
(!zero.is_ascii_lowercase());
assert!
(!percent.is_ascii_lowercase());
assert!
(!space.is_ascii_lowercase());
assert!
(!lf.is_ascii_lowercase());
assert!
(!esc.is_ascii_lowercase());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_alphanumeric
(&self) ->
bool
Checks if the value is an ASCII alphanumeric character:
U+0041 ‘A’ ..= U+005A ‘Z’, or
U+0061 ‘a’ ..= U+007A ‘z’, or
U+0030 ‘0’ ..= U+0039 ‘9’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(uppercase_a.is_ascii_alphanumeric());
assert!
(uppercase_g.is_ascii_alphanumeric());
assert!
(a.is_ascii_alphanumeric());
assert!
(g.is_ascii_alphanumeric());
assert!
(zero.is_ascii_alphanumeric());
assert!
(!percent.is_ascii_alphanumeric());
assert!
(!space.is_ascii_alphanumeric());
assert!
(!lf.is_ascii_alphanumeric());
assert!
(!esc.is_ascii_alphanumeric());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_digit
(&self) ->
bool
Checks if the value is an ASCII decimal digit:
U+0030 ‘0’ ..= U+0039 ‘9’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(!uppercase_a.is_ascii_digit());
assert!
(!uppercase_g.is_ascii_digit());
assert!
(!a.is_ascii_digit());
assert!
(!g.is_ascii_digit());
assert!
(zero.is_ascii_digit());
assert!
(!percent.is_ascii_digit());
assert!
(!space.is_ascii_digit());
assert!
(!lf.is_ascii_digit());
assert!
(!esc.is_ascii_digit());
Source
pub const fn
is_ascii_octdigit
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
is_ascii_octdigit
#101288
)
Checks if the value is an ASCII octal digit:
U+0030 ‘0’ ..= U+0037 ‘7’.
§
Examples
#![feature(is_ascii_octdigit)]
let
uppercase_a =
b'A'
;
let
a =
b'a'
;
let
zero =
b'0'
;
let
seven =
b'7'
;
let
nine =
b'9'
;
let
percent =
b'%'
;
let
lf =
b'\n'
;
assert!
(!uppercase_a.is_ascii_octdigit());
assert!
(!a.is_ascii_octdigit());
assert!
(zero.is_ascii_octdigit());
assert!
(seven.is_ascii_octdigit());
assert!
(!nine.is_ascii_octdigit());
assert!
(!percent.is_ascii_octdigit());
assert!
(!lf.is_ascii_octdigit());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_hexdigit
(&self) ->
bool
Checks if the value is an ASCII hexadecimal digit:
U+0030 ‘0’ ..= U+0039 ‘9’, or
U+0041 ‘A’ ..= U+0046 ‘F’, or
U+0061 ‘a’ ..= U+0066 ‘f’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(uppercase_a.is_ascii_hexdigit());
assert!
(!uppercase_g.is_ascii_hexdigit());
assert!
(a.is_ascii_hexdigit());
assert!
(!g.is_ascii_hexdigit());
assert!
(zero.is_ascii_hexdigit());
assert!
(!percent.is_ascii_hexdigit());
assert!
(!space.is_ascii_hexdigit());
assert!
(!lf.is_ascii_hexdigit());
assert!
(!esc.is_ascii_hexdigit());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_punctuation
(&self) ->
bool
Checks if the value is an ASCII punctuation character:
U+0021 ..= U+002F
! " # $ % & ' ( ) * + , - . /
, or
U+003A ..= U+0040
: ; < = > ? @
, or
U+005B ..= U+0060
[ \ ] ^ _ `
, or
U+007B ..= U+007E
{ | } ~
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(!uppercase_a.is_ascii_punctuation());
assert!
(!uppercase_g.is_ascii_punctuation());
assert!
(!a.is_ascii_punctuation());
assert!
(!g.is_ascii_punctuation());
assert!
(!zero.is_ascii_punctuation());
assert!
(percent.is_ascii_punctuation());
assert!
(!space.is_ascii_punctuation());
assert!
(!lf.is_ascii_punctuation());
assert!
(!esc.is_ascii_punctuation());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_graphic
(&self) ->
bool
Checks if the value is an ASCII graphic character:
U+0021 ‘!’ ..= U+007E ‘~’.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(uppercase_a.is_ascii_graphic());
assert!
(uppercase_g.is_ascii_graphic());
assert!
(a.is_ascii_graphic());
assert!
(g.is_ascii_graphic());
assert!
(zero.is_ascii_graphic());
assert!
(percent.is_ascii_graphic());
assert!
(!space.is_ascii_graphic());
assert!
(!lf.is_ascii_graphic());
assert!
(!esc.is_ascii_graphic());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_whitespace
(&self) ->
bool
Checks if the value is an ASCII whitespace character:
U+0020 SPACE, U+0009 HORIZONTAL TAB, U+000A LINE FEED,
U+000C FORM FEED, or U+000D CARRIAGE RETURN.
Rust uses the WhatWG Infra Standard’s
definition of ASCII
whitespace
. There are several other definitions in
wide use. For instance,
the POSIX locale
includes
U+000B VERTICAL TAB as well as all the above characters,
but—from the very same specification—
the default rule for
“field splitting” in the Bourne shell
considers
only
SPACE, HORIZONTAL TAB, and LINE FEED as whitespace.
If you are writing a program that will process an existing
file format, check what that format’s definition of whitespace is
before using this function.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(!uppercase_a.is_ascii_whitespace());
assert!
(!uppercase_g.is_ascii_whitespace());
assert!
(!a.is_ascii_whitespace());
assert!
(!g.is_ascii_whitespace());
assert!
(!zero.is_ascii_whitespace());
assert!
(!percent.is_ascii_whitespace());
assert!
(space.is_ascii_whitespace());
assert!
(lf.is_ascii_whitespace());
assert!
(!esc.is_ascii_whitespace());
1.24.0 (const: 1.47.0)
·
Source
pub const fn
is_ascii_control
(&self) ->
bool
Checks if the value is an ASCII control character:
U+0000 NUL ..= U+001F UNIT SEPARATOR, or U+007F DELETE.
Note that most ASCII whitespace characters are control
characters, but SPACE is not.
§
Examples
let
uppercase_a =
b'A'
;
let
uppercase_g =
b'G'
;
let
a =
b'a'
;
let
g =
b'g'
;
let
zero =
b'0'
;
let
percent =
b'%'
;
let
space =
b' '
;
let
lf =
b'\n'
;
let
esc =
b'\x1b'
;
assert!
(!uppercase_a.is_ascii_control());
assert!
(!uppercase_g.is_ascii_control());
assert!
(!a.is_ascii_control());
assert!
(!g.is_ascii_control());
assert!
(!zero.is_ascii_control());
assert!
(!percent.is_ascii_control());
assert!
(!space.is_ascii_control());
assert!
(lf.is_ascii_control());
assert!
(esc.is_ascii_control());
1.60.0
·
Source
pub fn
escape_ascii
(self) ->
EscapeDefault
ⓘ
Returns an iterator that produces an escaped version of a
u8
,
treating it as an ASCII character.
The behavior is identical to
ascii::escape_default
.
§
Examples
assert_eq!
(
"0"
,
b'0'
.escape_ascii().to_string());
assert_eq!
(
"\\t"
,
b'\t'
.escape_ascii().to_string());
assert_eq!
(
"\\r"
,
b'\r'
.escape_ascii().to_string());
assert_eq!
(
"\\n"
,
b'\n'
.escape_ascii().to_string());
assert_eq!
(
"\\'"
,
b'\''
.escape_ascii().to_string());
assert_eq!
(
"\\\""
,
b'"'
.escape_ascii().to_string());
assert_eq!
(
"\\\\"
,
b'\\'
.escape_ascii().to_string());
assert_eq!
(
"\\x9d"
,
b'\x9d'
.escape_ascii().to_string());
Source
§
impl
u8
1.0.0 (const: 1.82.0)
·
Source
pub const fn
from_str_radix
(src: &
str
, radix:
u32
) ->
Result
<
u8
,
ParseIntError
>
Parses an integer from a string slice with digits in a given base.
The string is expected to be an optional
+
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
Digits are a subset of these characters, depending on
radix
:
0-9
a-z
A-Z
§
Panics
This function panics if
radix
is not in the range from 2 to 36.
§
Examples
Basic usage:
assert_eq!
(u8::from_str_radix(
"A"
,
16
),
Ok
(
10
));
Trailing space returns error:
assert!
(u8::from_str_radix(
"1 "
,
10
).is_err());
Source
pub const fn
from_ascii
(src: &[
u8
]) ->
Result
<
u8
,
ParseIntError
>
🔬
This is a nightly-only experimental API. (
int_from_ascii
#134821
)
Parses an integer from an ASCII-byte slice with decimal digits.
The characters are expected to be an optional
+
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
§
Examples
Basic usage:
#![feature(int_from_ascii)]
assert_eq!
(u8::from_ascii(
b"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(u8::from_ascii(
b"1 "
).is_err());
Source
pub const fn
from_ascii_radix
(
    src: &[
u8
],
    radix:
u32
,
) ->
Result
<
u8
,
ParseIntError
>
🔬
This is a nightly-only experimental API. (
int_from_ascii
#134821
)
Parses an integer from an ASCII-byte slice with digits in a given base.
The characters are expected to be an optional
+
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
Digits are a subset of these characters, depending on
radix
:
0-9
a-z
A-Z
§
Panics
This function panics if
radix
is not in the range from 2 to 36.
§
Examples
Basic usage:
#![feature(int_from_ascii)]
assert_eq!
(u8::from_ascii_radix(
b"A"
,
16
),
Ok
(
10
));
Trailing space returns error:
assert!
(u8::from_ascii_radix(
b"1 "
,
10
).is_err());
Trait Implementations
§
1.0.0
·
Source
§
impl
Add
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
Add
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Add
>::
Output
Performs the
+
operation.
Read more
1.0.0 (const:
unstable
)
·
Source
§
impl
Add
for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
AddAssign
for
u8
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
1.0.0
·
Source
§
impl
AsciiExt
for
u8
Source
§
type
Owned
=
u8
👎
Deprecated since 1.26.0: use inherent methods instead
Container type for copied ASCII characters.
Source
§
fn
is_ascii
(&self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks if the value is within the ASCII range.
Read more
Source
§
fn
to_ascii_uppercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII upper case equivalent.
Read more
Source
§
fn
to_ascii_lowercase
(&self) -> Self::
Owned
👎
Deprecated since 1.26.0: use inherent methods instead
Makes a copy of the value in its ASCII lower case equivalent.
Read more
Source
§
fn
eq_ignore_ascii_case
(&self, o: &Self) ->
bool
👎
Deprecated since 1.26.0: use inherent methods instead
Checks that two values are an ASCII case-insensitive match.
Read more
Source
§
fn
make_ascii_uppercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII upper case equivalent in-place.
Read more
Source
§
fn
make_ascii_lowercase
(&mut self)
👎
Deprecated since 1.26.0: use inherent methods instead
Converts this type to its ASCII lower case equivalent in-place.
Read more
1.0.0
·
Source
§
impl
Binary
for
u8
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
impl
BitAnd
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
BitAnd
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
BitAndAssign
for
u8
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
1.0.0
·
Source
§
impl
BitOr
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
BitOr
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
BitOrAssign
for
u8
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
1.0.0
·
Source
§
impl
BitXor
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
BitXor
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
BitXorAssign
for
u8
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
Source
§
impl
CarryingMulAdd
for
u8
Source
§
type
Unsigned
=
u8
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Source
§
const fn
carrying_mul_add
(self, a:
u8
, b:
u8
, c:
u8
) -> (
u8
,
u8
)
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
1.0.0
·
Source
§
impl
Clone
for
u8
Source
§
fn
clone
(&self) ->
u8
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
impl
Debug
for
u8
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
impl
Default
for
u8
Source
§
fn
default
() ->
u8
Returns the default value of
0
Source
§
impl
DisjointBitOr
for
u8
Source
§
const unsafe fn
disjoint_bitor
(self, other:
u8
) ->
u8
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
See
super::disjoint_bitor
; we just need the trait indirection to handle
different types since calling intrinsics with generics doesn’t work.
1.0.0
·
Source
§
impl
Display
for
u8
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
impl
Div
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Div
>::
Output
Performs the
/
operation.
Read more
1.0.0
·
Source
§
impl
Div
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Div
>::
Output
Performs the
/
operation.
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
1.0.0
·
Source
§
impl<'a>
Div
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Div
>::
Output
Performs the
/
operation.
Read more
1.0.0
·
Source
§
impl
Div
for
u8
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u8
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
DivAssign
for
u8
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
Source
§
impl
From
<
AsciiChar
> for
u8
Source
§
fn
from
(chr:
AsciiChar
) ->
u8
Converts to this type from the input type.
1.28.0
·
Source
§
impl
From
<
bool
> for
u8
Source
§
fn
from
(small:
bool
) ->
u8
Converts a
bool
to
u8
losslessly.
The resulting value is
0
for
false
and
1
for
true
values.
§
Examples
assert_eq!
(u8::from(
true
),
1
);
assert_eq!
(u8::from(
false
),
0
);
1.34.0
·
Source
§
impl
From
<
u8
> for
AtomicU8
Source
§
fn
from
(v:
u8
) ->
AtomicU8
Converts an
u8
into an
AtomicU8
.
1.61.0
·
Source
§
impl
From
<
u8
> for
ExitCode
Source
§
fn
from
(code:
u8
) -> Self
Constructs an
ExitCode
from an arbitrary u8 value.
1.13.0
·
Source
§
impl
From
<
u8
> for
char
Maps a byte in 0x00..=0xFF to a
char
whose code point has the same value, in U+0000..=U+00FF.
Unicode is designed such that this effectively decodes bytes
with the character encoding that IANA calls ISO-8859-1.
This encoding is compatible with ASCII.
Note that this is different from ISO/IEC 8859-1 a.k.a. ISO 8859-1 (with one less hyphen),
which leaves some “blanks”, byte values that are not assigned to any character.
ISO-8859-1 (the IANA one) assigns them to the C0 and C1 control codes.
Note that this is
also
different from Windows-1252 a.k.a. code page 1252,
which is a superset ISO/IEC 8859-1 that assigns some (not all!) blanks
to punctuation and various Latin characters.
To confuse things further,
on the Web
ascii
,
iso-8859-1
, and
windows-1252
are all aliases
for a superset of Windows-1252 that fills the remaining blanks with corresponding
C0 and C1 control codes.
Source
§
fn
from
(i:
u8
) ->
char
Converts a
u8
into a
char
.
§
Examples
let
u =
32
as
u8;
let
c = char::from(u);
assert!
(
4
== size_of_val(
&
c))
1.6.0
·
Source
§
impl
From
<
u8
> for
f128
Source
§
fn
from
(small:
u8
) ->
f128
Converts
u8
to
f128
losslessly.
1.6.0
·
Source
§
impl
From
<
u8
> for
f16
Source
§
fn
from
(small:
u8
) ->
f16
Converts
u8
to
f16
losslessly.
1.6.0
·
Source
§
impl
From
<
u8
> for
f32
Source
§
fn
from
(small:
u8
) ->
f32
Converts
u8
to
f32
losslessly.
1.6.0
·
Source
§
impl
From
<
u8
> for
f64
Source
§
fn
from
(small:
u8
) ->
f64
Converts
u8
to
f64
losslessly.
1.26.0
·
Source
§
impl
From
<
u8
> for
i128
Source
§
fn
from
(small:
u8
) ->
i128
Converts
u8
to
i128
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
i16
Source
§
fn
from
(small:
u8
) ->
i16
Converts
u8
to
i16
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
i32
Source
§
fn
from
(small:
u8
) ->
i32
Converts
u8
to
i32
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
i64
Source
§
fn
from
(small:
u8
) ->
i64
Converts
u8
to
i64
losslessly.
1.26.0
·
Source
§
impl
From
<
u8
> for
isize
Source
§
fn
from
(small:
u8
) ->
isize
Converts
u8
to
isize
losslessly.
1.26.0
·
Source
§
impl
From
<
u8
> for
u128
Source
§
fn
from
(small:
u8
) ->
u128
Converts
u8
to
u128
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
u16
Source
§
fn
from
(small:
u8
) ->
u16
Converts
u8
to
u16
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
u32
Source
§
fn
from
(small:
u8
) ->
u32
Converts
u8
to
u32
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
u64
Source
§
fn
from
(small:
u8
) ->
u64
Converts
u8
to
u64
losslessly.
1.5.0
·
Source
§
impl
From
<
u8
> for
usize
Source
§
fn
from
(small:
u8
) ->
usize
Converts
u8
to
usize
losslessly.
Source
§
impl
FromIterator
<
u8
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item =
u8
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl
FromStr
for
u8
Source
§
fn
from_str
(src: &
str
) ->
Result
<
u8
,
ParseIntError
>
Parses an integer from a string slice with decimal digits.
The characters are expected to be an optional
+
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
§
Examples
Basic usage:
use
std::str::FromStr;
assert_eq!
(u8::from_str(
"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(u8::from_str(
"1 "
).is_err());
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
1.0.0
·
Source
§
impl
Hash
for
u8
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
Source
§
fn
hash_slice
<H>(data: &[
u8
], state:
&mut H
)
where
    H:
Hasher
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.42.0
·
Source
§
impl
LowerExp
for
u8
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
impl
LowerHex
for
u8
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
impl
Mul
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
Mul
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
MulAssign
for
u8
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
1.0.0
·
Source
§
impl
Not
for &
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
u8
Performs the unary
!
operation.
Read more
1.0.0
·
Source
§
impl
Octal
for
u8
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
impl
Ord
for
u8
Source
§
fn
cmp
(&self, other: &
u8
) ->
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
impl
PartialEq
for
u8
Source
§
fn
eq
(&self, other: &
u8
) ->
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
u8
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
impl
PartialOrd
for
u8
Source
§
fn
partial_cmp
(&self, other: &
u8
) ->
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
u8
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
Source
§
fn
le
(&self, other: &
u8
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
Source
§
fn
gt
(&self, other: &
u8
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
Source
§
fn
ge
(&self, other: &
u8
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
1.12.0
·
Source
§
impl<'a>
Product
<&'a
u8
> for
u8
Source
§
fn
product
<I>(iter: I) ->
u8
where
    I:
Iterator
<Item = &'a
u8
>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
1.12.0
·
Source
§
impl
Product
for
u8
Source
§
fn
product
<I>(iter: I) ->
u8
where
    I:
Iterator
<Item =
u8
>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl
Random
for
u8
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
u8
🔬
This is a nightly-only experimental API. (
random
#130703
)
Generates a random value.
Warning:
Be careful when manipulating the resulting value! This
method samples according to a uniform distribution, so a value of 1 is
just as likely as
MAX
. By using modulo operations, some
values can become more likely than others. Use audited crates when in
doubt.
Source
§
impl
RangePattern
for
u8
Source
§
const
MIN
:
u8
= 0u8
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
Trait version of the inherent
MIN
assoc const.
Source
§
const
MAX
:
u8
= 255u8
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
Trait version of the inherent
MIN
assoc const.
Source
§
const fn
sub_one
(self) ->
u8
🔬
This is a nightly-only experimental API. (
pattern_type_range_trait
#123646
)
A compile-time helper to subtract 1 for exclusive ranges.
1.0.0
·
Source
§
impl
Rem
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Rem
>::
Output
Performs the
%
operation.
Read more
1.0.0
·
Source
§
impl
Rem
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Rem
>::
Output
Performs the
%
operation.
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
1.0.0
·
Source
§
impl<'a>
Rem
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
Rem
>::
Output
Performs the
%
operation.
Read more
1.0.0
·
Source
§
impl
Rem
for
u8
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
.
Source
§
type
Output
=
u8
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
RemAssign
for
u8
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
1.0.0
·
Source
§
impl
Shl
<&
i128
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
i128
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
i128
) -> <
u8
as
Shl
<
i128
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
<&
i128
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
i128
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
i128
) -> <
u8
as
Shl
<
i128
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
<&
i16
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
i16
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
i16
) -> <
u8
as
Shl
<
i16
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
<&
i16
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
i16
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
i16
) -> <
u8
as
Shl
<
i16
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
<&
i32
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
i32
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
i32
) -> <
u8
as
Shl
<
i32
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
<&
i32
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
i32
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
i32
) -> <
u8
as
Shl
<
i32
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
<&
i64
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
i64
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
i64
) -> <
u8
as
Shl
<
i64
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
<&
i64
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
i64
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
i64
) -> <
u8
as
Shl
<
i64
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
<&
i8
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
i8
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
i8
) -> <
u8
as
Shl
<
i8
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
<&
i8
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
i8
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
i8
) -> <
u8
as
Shl
<
i8
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
<&
isize
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
isize
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
isize
) -> <
u8
as
Shl
<
isize
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
<&
isize
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
isize
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
isize
) -> <
u8
as
Shl
<
isize
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
<&
u128
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
u128
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
u128
) -> <
u8
as
Shl
<
u128
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
<&
u128
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
u128
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
u128
) -> <
u8
as
Shl
<
u128
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
<&
u16
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
u16
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
u16
) -> <
u8
as
Shl
<
u16
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
<&
u16
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
u16
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
u16
) -> <
u8
as
Shl
<
u16
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
<&
u32
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
u32
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
u32
) -> <
u8
as
Shl
<
u32
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
<&
u32
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
u32
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
u32
) -> <
u8
as
Shl
<
u32
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
<&
u64
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
<
u64
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
u64
) -> <
u8
as
Shl
<
u64
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
<&
u64
> for
u8
Source
§
type
Output
= <
u8
as
Shl
<
u64
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
u64
) -> <
u8
as
Shl
<
u64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shl
<&
u8
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
<&
u8
> for &
i128
Source
§
type
Output
= <
i128
as
Shl
<
u8
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
u8
) -> <
i128
as
Shl
<
u8
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
<&
u8
> for &
i16
Source
§
type
Output
= <
i16
as
Shl
<
u8
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
u8
) -> <
i16
as
Shl
<
u8
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
<&
u8
> for &
i32
Source
§
type
Output
= <
i32
as
Shl
<
u8
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
u8
) -> <
i32
as
Shl
<
u8
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
<&
u8
> for &
i64
Source
§
type
Output
= <
i64
as
Shl
<
u8
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
u8
) -> <
i64
as
Shl
<
u8
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
<&
u8
> for &
i8
Source
§
type
Output
= <
i8
as
Shl
<
u8
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
u8
) -> <
i8
as
Shl
<
u8
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
<&
u8
> for &
isize
Source
§
type
Output
= <
isize
as
Shl
<
u8
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
u8
) -> <
isize
as
Shl
<
u8
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
<&
u8
> for &
u128
Source
§
type
Output
= <
u128
as
Shl
<
u8
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
u8
) -> <
u128
as
Shl
<
u8
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
<&
u8
> for &
u16
Source
§
type
Output
= <
u16
as
Shl
<
u8
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
u8
) -> <
u16
as
Shl
<
u8
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
<&
u8
> for &
u32
Source
§
type
Output
= <
u32
as
Shl
<
u8
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
u8
) -> <
u32
as
Shl
<
u8
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
<&
u8
> for &
u64
Source
§
type
Output
= <
u64
as
Shl
<
u8
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
u8
) -> <
u64
as
Shl
<
u8
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
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
Shl
>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
u8
) -> <
u8
as
Shl
>::
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
<&
u8
> for &
usize
Source
§
type
Output
= <
usize
as
Shl
<
u8
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
u8
) -> <
usize
as
Shl
<
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u8
) -> <
Simd
<
u8
, N> as
Shl
<&
u8
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
<&
u8
> for
i128
Source
§
type
Output
= <
i128
as
Shl
<
u8
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
u8
) -> <
i128
as
Shl
<
u8
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
<&
u8
> for
i16
Source
§
type
Output
= <
i16
as
Shl
<
u8
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
u8
) -> <
i16
as
Shl
<
u8
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
<&
u8
> for
i32
Source
§
type
Output
= <
i32
as
Shl
<
u8
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
u8
) -> <
i32
as
Shl
<
u8
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
<&
u8
> for
i64
Source
§
type
Output
= <
i64
as
Shl
<
u8
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
u8
) -> <
i64
as
Shl
<
u8
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
<&
u8
> for
i8
Source
§
type
Output
= <
i8
as
Shl
<
u8
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
u8
) -> <
i8
as
Shl
<
u8
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
<&
u8
> for
isize
Source
§
type
Output
= <
isize
as
Shl
<
u8
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
u8
) -> <
isize
as
Shl
<
u8
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
<&
u8
> for
u128
Source
§
type
Output
= <
u128
as
Shl
<
u8
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
u8
) -> <
u128
as
Shl
<
u8
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
<&
u8
> for
u16
Source
§
type
Output
= <
u16
as
Shl
<
u8
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
u8
) -> <
u16
as
Shl
<
u8
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
<&
u8
> for
u32
Source
§
type
Output
= <
u32
as
Shl
<
u8
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
u8
) -> <
u32
as
Shl
<
u8
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
<&
u8
> for
u64
Source
§
type
Output
= <
u64
as
Shl
<
u8
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
u8
) -> <
u64
as
Shl
<
u8
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
Shl
>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other: &
u8
) -> <
u8
as
Shl
>::
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
<&
u8
> for
usize
Source
§
type
Output
= <
usize
as
Shl
<
u8
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
u8
) -> <
usize
as
Shl
<
u8
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
<&
usize
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
<&
usize
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
impl<'a>
Shl
<
i128
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
i128
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
i128
) -> <
u8
as
Shl
<
i128
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
i128
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
i128
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
i16
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
i16
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
i16
) -> <
u8
as
Shl
<
i16
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
i16
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
i16
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
i32
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
i32
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
i32
) -> <
u8
as
Shl
<
i32
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
i32
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
i32
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
i64
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
i64
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
i64
) -> <
u8
as
Shl
<
i64
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
i64
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
i64
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
i8
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
i8
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
i8
) -> <
u8
as
Shl
<
i8
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
i8
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
i8
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
isize
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
isize
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
isize
) -> <
u8
as
Shl
<
isize
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
isize
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
isize
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
u128
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
u128
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
u128
) -> <
u8
as
Shl
<
u128
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
u128
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u128
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
u16
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
u16
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
u16
) -> <
u8
as
Shl
<
u16
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
u16
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u16
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
u32
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
u32
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
u32
) -> <
u8
as
Shl
<
u32
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
u32
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u32
) ->
u8
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
u64
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
<
u64
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
u64
) -> <
u8
as
Shl
<
u64
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
u64
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u64
) ->
u8
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
i128
Source
§
type
Output
= <
i128
as
Shl
<
u8
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
u8
) -> <
i128
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
i16
Source
§
type
Output
= <
i16
as
Shl
<
u8
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
u8
) -> <
i16
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
i32
Source
§
type
Output
= <
i32
as
Shl
<
u8
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
u8
) -> <
i32
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
i64
Source
§
type
Output
= <
i64
as
Shl
<
u8
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
u8
) -> <
i64
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
i8
Source
§
type
Output
= <
i8
as
Shl
<
u8
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
u8
) -> <
i8
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
isize
Source
§
type
Output
= <
isize
as
Shl
<
u8
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
u8
) -> <
isize
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
u128
Source
§
type
Output
= <
u128
as
Shl
<
u8
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
u8
) -> <
u128
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
u16
Source
§
type
Output
= <
u16
as
Shl
<
u8
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
u8
) -> <
u16
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
u32
Source
§
type
Output
= <
u32
as
Shl
<
u8
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
u8
) -> <
u32
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
u64
Source
§
type
Output
= <
u64
as
Shl
<
u8
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
u8
) -> <
u64
as
Shl
<
u8
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
impl<'a>
Shl
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shl
>::
Output
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) -> <
u8
as
Shl
>::
Output
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
u8
> for &'a
usize
Source
§
type
Output
= <
usize
as
Shl
<
u8
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
u8
) -> <
usize
as
Shl
<
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u8
) -> <
Simd
<
u8
, N> as
Shl
<
u8
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
u8
> for
i128
Source
§
type
Output
=
i128
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
i128
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
u8
> for
i16
Source
§
type
Output
=
i16
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
i16
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
u8
> for
i32
Source
§
type
Output
=
i32
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
i32
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
u8
> for
i64
Source
§
type
Output
=
i64
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
i64
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
u8
> for
i8
Source
§
type
Output
=
i8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
i8
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
u8
> for
isize
Source
§
type
Output
=
isize
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
isize
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
u8
> for
u128
Source
§
type
Output
=
u128
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
u128
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
u8
> for
u16
Source
§
type
Output
=
u16
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
u16
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
u8
> for
u32
Source
§
type
Output
=
u32
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
u32
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
u8
> for
u64
Source
§
type
Output
=
u64
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
u64
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
u8
> for
usize
Source
§
type
Output
=
usize
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
usize
Performs the
<<
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shl
<
usize
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
u8
Source
§
type
Output
=
u8
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
u8
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
for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, other:
u8
) ->
u8
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
i128
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
i128
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
i16
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
i16
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
i32
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
i32
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
i64
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
i64
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
i8
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
i8
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
isize
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
isize
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
u128
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
u128
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
u16
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
u16
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
u32
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
u32
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
u64
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
u64
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
u8
> for
i128
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
i16
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
i32
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
i64
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
i8
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
isize
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
u128
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
u16
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
u32
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
u64
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
u8
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
> for
usize
Source
§
fn
shl_assign
(&mut self, other: &
u8
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
u8
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
i128
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
i128
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
i16
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
i16
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
i32
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
i32
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
i64
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
i64
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
i8
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
i8
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
isize
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
isize
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
u128
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
u128
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
u16
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
u16
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
u32
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
u32
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
u64
> for
u8
Source
§
fn
shl_assign
(&mut self, other:
u64
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
u8
> for
i128
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
i16
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
i32
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
i64
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
i8
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
isize
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
u128
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
u16
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
u32
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
u64
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
> for
usize
Source
§
fn
shl_assign
(&mut self, other:
u8
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
u8
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
for
u8
Source
§
fn
shl_assign
(&mut self, other:
u8
)
Performs the
<<=
operation.
Read more
1.0.0
·
Source
§
impl
Shr
<&
i128
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
i128
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
i128
) -> <
u8
as
Shr
<
i128
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
<&
i128
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
i128
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
i128
) -> <
u8
as
Shr
<
i128
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
<&
i16
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
i16
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
i16
) -> <
u8
as
Shr
<
i16
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
<&
i16
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
i16
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
i16
) -> <
u8
as
Shr
<
i16
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
<&
i32
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
i32
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
i32
) -> <
u8
as
Shr
<
i32
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
<&
i32
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
i32
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
i32
) -> <
u8
as
Shr
<
i32
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
<&
i64
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
i64
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
i64
) -> <
u8
as
Shr
<
i64
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
<&
i64
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
i64
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
i64
) -> <
u8
as
Shr
<
i64
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
<&
i8
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
i8
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
i8
) -> <
u8
as
Shr
<
i8
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
<&
i8
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
i8
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
i8
) -> <
u8
as
Shr
<
i8
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
<&
isize
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
isize
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
isize
) -> <
u8
as
Shr
<
isize
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
<&
isize
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
isize
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
isize
) -> <
u8
as
Shr
<
isize
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
<&
u128
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
u128
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
u128
) -> <
u8
as
Shr
<
u128
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
<&
u128
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
u128
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
u128
) -> <
u8
as
Shr
<
u128
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
<&
u16
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
u16
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
u16
) -> <
u8
as
Shr
<
u16
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
<&
u16
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
u16
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
u16
) -> <
u8
as
Shr
<
u16
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
<&
u32
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
u32
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
u32
) -> <
u8
as
Shr
<
u32
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
<&
u32
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
u32
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
u32
) -> <
u8
as
Shr
<
u32
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
<&
u64
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
<
u64
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
u64
) -> <
u8
as
Shr
<
u64
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
<&
u64
> for
u8
Source
§
type
Output
= <
u8
as
Shr
<
u64
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
u64
) -> <
u8
as
Shr
<
u64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shr
<&
u8
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
<&
u8
> for &
i128
Source
§
type
Output
= <
i128
as
Shr
<
u8
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
u8
) -> <
i128
as
Shr
<
u8
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
<&
u8
> for &
i16
Source
§
type
Output
= <
i16
as
Shr
<
u8
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
u8
) -> <
i16
as
Shr
<
u8
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
<&
u8
> for &
i32
Source
§
type
Output
= <
i32
as
Shr
<
u8
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
u8
) -> <
i32
as
Shr
<
u8
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
<&
u8
> for &
i64
Source
§
type
Output
= <
i64
as
Shr
<
u8
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
u8
) -> <
i64
as
Shr
<
u8
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
<&
u8
> for &
i8
Source
§
type
Output
= <
i8
as
Shr
<
u8
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
u8
) -> <
i8
as
Shr
<
u8
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
<&
u8
> for &
isize
Source
§
type
Output
= <
isize
as
Shr
<
u8
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
u8
) -> <
isize
as
Shr
<
u8
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
<&
u8
> for &
u128
Source
§
type
Output
= <
u128
as
Shr
<
u8
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
u8
) -> <
u128
as
Shr
<
u8
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
<&
u8
> for &
u16
Source
§
type
Output
= <
u16
as
Shr
<
u8
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
u8
) -> <
u16
as
Shr
<
u8
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
<&
u8
> for &
u32
Source
§
type
Output
= <
u32
as
Shr
<
u8
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
u8
) -> <
u32
as
Shr
<
u8
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
<&
u8
> for &
u64
Source
§
type
Output
= <
u64
as
Shr
<
u8
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
u8
) -> <
u64
as
Shr
<
u8
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
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
Shr
>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
u8
) -> <
u8
as
Shr
>::
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
<&
u8
> for &
usize
Source
§
type
Output
= <
usize
as
Shr
<
u8
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
u8
) -> <
usize
as
Shr
<
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u8
) -> <
Simd
<
u8
, N> as
Shr
<&
u8
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
<&
u8
> for
i128
Source
§
type
Output
= <
i128
as
Shr
<
u8
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
u8
) -> <
i128
as
Shr
<
u8
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
<&
u8
> for
i16
Source
§
type
Output
= <
i16
as
Shr
<
u8
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
u8
) -> <
i16
as
Shr
<
u8
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
<&
u8
> for
i32
Source
§
type
Output
= <
i32
as
Shr
<
u8
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
u8
) -> <
i32
as
Shr
<
u8
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
<&
u8
> for
i64
Source
§
type
Output
= <
i64
as
Shr
<
u8
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
u8
) -> <
i64
as
Shr
<
u8
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
<&
u8
> for
i8
Source
§
type
Output
= <
i8
as
Shr
<
u8
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
u8
) -> <
i8
as
Shr
<
u8
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
<&
u8
> for
isize
Source
§
type
Output
= <
isize
as
Shr
<
u8
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
u8
) -> <
isize
as
Shr
<
u8
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
<&
u8
> for
u128
Source
§
type
Output
= <
u128
as
Shr
<
u8
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
u8
) -> <
u128
as
Shr
<
u8
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
<&
u8
> for
u16
Source
§
type
Output
= <
u16
as
Shr
<
u8
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
u8
) -> <
u16
as
Shr
<
u8
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
<&
u8
> for
u32
Source
§
type
Output
= <
u32
as
Shr
<
u8
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
u8
) -> <
u32
as
Shr
<
u8
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
<&
u8
> for
u64
Source
§
type
Output
= <
u64
as
Shr
<
u8
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
u8
) -> <
u64
as
Shr
<
u8
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
Shr
>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other: &
u8
) -> <
u8
as
Shr
>::
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
<&
u8
> for
usize
Source
§
type
Output
= <
usize
as
Shr
<
u8
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
u8
) -> <
usize
as
Shr
<
u8
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
<&
usize
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
<&
usize
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
impl<'a>
Shr
<
i128
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
i128
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
i128
) -> <
u8
as
Shr
<
i128
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
i128
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
i128
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
i16
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
i16
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
i16
) -> <
u8
as
Shr
<
i16
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
i16
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
i16
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
i32
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
i32
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
i32
) -> <
u8
as
Shr
<
i32
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
i32
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
i32
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
i64
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
i64
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
i64
) -> <
u8
as
Shr
<
i64
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
i64
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
i64
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
i8
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
i8
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
i8
) -> <
u8
as
Shr
<
i8
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
i8
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
i8
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
isize
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
isize
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
isize
) -> <
u8
as
Shr
<
isize
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
isize
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
isize
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
u128
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
u128
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
u128
) -> <
u8
as
Shr
<
u128
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
u128
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u128
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
u16
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
u16
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
u16
) -> <
u8
as
Shr
<
u16
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
u16
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u16
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
u32
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
u32
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
u32
) -> <
u8
as
Shr
<
u32
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
u32
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u32
) ->
u8
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
u64
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
<
u64
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
u64
) -> <
u8
as
Shr
<
u64
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
u64
> for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u64
) ->
u8
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
i128
Source
§
type
Output
= <
i128
as
Shr
<
u8
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
u8
) -> <
i128
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
i16
Source
§
type
Output
= <
i16
as
Shr
<
u8
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
u8
) -> <
i16
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
i32
Source
§
type
Output
= <
i32
as
Shr
<
u8
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
u8
) -> <
i32
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
i64
Source
§
type
Output
= <
i64
as
Shr
<
u8
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
u8
) -> <
i64
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
i8
Source
§
type
Output
= <
i8
as
Shr
<
u8
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
u8
) -> <
i8
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
isize
Source
§
type
Output
= <
isize
as
Shr
<
u8
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
u8
) -> <
isize
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
u128
Source
§
type
Output
= <
u128
as
Shr
<
u8
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
u8
) -> <
u128
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
u16
Source
§
type
Output
= <
u16
as
Shr
<
u8
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
u8
) -> <
u16
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
u32
Source
§
type
Output
= <
u32
as
Shr
<
u8
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
u8
) -> <
u32
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
u64
Source
§
type
Output
= <
u64
as
Shr
<
u8
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
u8
) -> <
u64
as
Shr
<
u8
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
impl<'a>
Shr
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
Shr
>::
Output
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) -> <
u8
as
Shr
>::
Output
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
u8
> for &'a
usize
Source
§
type
Output
= <
usize
as
Shr
<
u8
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
u8
) -> <
usize
as
Shr
<
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u8
) -> <
Simd
<
u8
, N> as
Shr
<
u8
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
u8
> for
i128
Source
§
type
Output
=
i128
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
i128
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
u8
> for
i16
Source
§
type
Output
=
i16
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
i16
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
u8
> for
i32
Source
§
type
Output
=
i32
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
i32
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
u8
> for
i64
Source
§
type
Output
=
i64
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
i64
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
u8
> for
i8
Source
§
type
Output
=
i8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
i8
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
u8
> for
isize
Source
§
type
Output
=
isize
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
isize
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
u8
> for
u128
Source
§
type
Output
=
u128
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
u128
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
u8
> for
u16
Source
§
type
Output
=
u16
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
u16
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
u8
> for
u32
Source
§
type
Output
=
u32
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
u32
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
u8
> for
u64
Source
§
type
Output
=
u64
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
u64
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
u8
> for
usize
Source
§
type
Output
=
usize
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
usize
Performs the
>>
operation.
Read more
1.0.0
·
Source
§
impl<'a>
Shr
<
usize
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
as
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
u8
Source
§
type
Output
=
u8
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
u8
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
for
u8
Source
§
type
Output
=
u8
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, other:
u8
) ->
u8
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
i128
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
i128
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
i16
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
i16
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
i32
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
i32
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
i64
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
i64
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
i8
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
i8
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
isize
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
isize
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
u128
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
u128
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
u16
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
u16
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
u32
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
u32
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
u64
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
u64
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
u8
> for
i128
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
i16
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
i32
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
i64
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
i8
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
isize
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
u128
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
u16
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
u32
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
u64
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
u8
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
> for
usize
Source
§
fn
shr_assign
(&mut self, other: &
u8
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
u8
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
i128
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
i128
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
i16
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
i16
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
i32
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
i32
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
i64
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
i64
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
i8
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
i8
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
isize
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
isize
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
u128
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
u128
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
u16
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
u16
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
u32
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
u32
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
u64
> for
u8
Source
§
fn
shr_assign
(&mut self, other:
u64
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
u8
> for
i128
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
i16
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
i32
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
i64
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
i8
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
isize
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
u128
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
u16
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
u32
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
u64
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
> for
usize
Source
§
fn
shr_assign
(&mut self, other:
u8
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
u8
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
for
u8
Source
§
fn
shr_assign
(&mut self, other:
u8
)
Performs the
>>=
operation.
Read more
Source
§
impl
SimdElement
for
u8
Source
§
type
Mask
=
i8
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask element type corresponding to this element type.
Source
§
impl
Step
for
u8
Source
§
fn
forward
(start:
u8
, n:
usize
) ->
u8
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward
(start:
u8
, n:
usize
) ->
u8
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
unsafe fn
forward_unchecked
(start:
u8
, n:
usize
) ->
u8
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
unsafe fn
backward_unchecked
(start:
u8
, n:
usize
) ->
u8
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
Source
§
fn
steps_between
(start: &
u8
, end: &
u8
) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the bounds on the number of
successor
steps required to get from
start
to
end
like
Iterator::size_hint()
.
Read more
Source
§
fn
forward_checked
(start:
u8
, n:
usize
) ->
Option
<
u8
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
Read more
Source
§
fn
backward_checked
(start:
u8
, n:
usize
) ->
Option
<
u8
>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
Read more
1.0.0
·
Source
§
impl
Sub
<&
u8
> for &
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
<&
u8
> for
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
impl<'a>
Sub
<
u8
> for &'a
u8
Source
§
type
Output
= <
u8
as
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
u8
) -> <
u8
as
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
u8
Source
§
type
Output
=
u8
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
u8
) ->
u8
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
u8
> for
u8
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
1.8.0
·
Source
§
impl
SubAssign
for
u8
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
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
u8
> for
u8
Source
§
fn
sum
<I>(iter: I) ->
u8
where
    I:
Iterator
<Item = &'a
u8
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.12.0
·
Source
§
impl
Sum
for
u8
Source
§
fn
sum
<I>(iter: I) ->
u8
where
    I:
Iterator
<Item =
u8
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.59.0
·
Source
§
impl
TryFrom
<
char
> for
u8
Maps a
char
with code point in U+0000..=U+00FF to a byte in 0x00..=0xFF with same value,
failing if the code point is greater than U+00FF.
See
impl From<u8> for char
for details on the encoding.
Source
§
fn
try_from
(c:
char
) ->
Result
<
u8
, <
u8
as
TryFrom
<
char
>>::
Error
>
Tries to convert a
char
into a
u8
.
§
Examples
let
a =
'ÿ'
;
// U+00FF
let
b =
'Ā'
;
// U+0100
assert_eq!
(u8::try_from(a),
Ok
(
0xFF_u8
));
assert!
(u8::try_from(b).is_err());
Source
§
type
Error
=
TryFromCharError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
i128
> for
u8
Source
§
fn
try_from
(u:
i128
) ->
Result
<
u8
, <
u8
as
TryFrom
<
i128
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
i16
> for
u8
Source
§
fn
try_from
(u:
i16
) ->
Result
<
u8
, <
u8
as
TryFrom
<
i16
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
i32
> for
u8
Source
§
fn
try_from
(u:
i32
) ->
Result
<
u8
, <
u8
as
TryFrom
<
i32
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
i64
> for
u8
Source
§
fn
try_from
(u:
i64
) ->
Result
<
u8
, <
u8
as
TryFrom
<
i64
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
i8
> for
u8
Source
§
fn
try_from
(u:
i8
) ->
Result
<
u8
, <
u8
as
TryFrom
<
i8
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
isize
> for
u8
Source
§
fn
try_from
(u:
isize
) ->
Result
<
u8
, <
u8
as
TryFrom
<
isize
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
u128
> for
u8
Source
§
fn
try_from
(u:
u128
) ->
Result
<
u8
, <
u8
as
TryFrom
<
u128
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
u16
> for
u8
Source
§
fn
try_from
(u:
u16
) ->
Result
<
u8
, <
u8
as
TryFrom
<
u16
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
u32
> for
u8
Source
§
fn
try_from
(u:
u32
) ->
Result
<
u8
, <
u8
as
TryFrom
<
u32
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
u64
> for
u8
Source
§
fn
try_from
(u:
u64
) ->
Result
<
u8
, <
u8
as
TryFrom
<
u64
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
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
1.34.0
·
Source
§
impl
TryFrom
<
u8
> for
i8
Source
§
fn
try_from
(u:
u8
) ->
Result
<
i8
, <
i8
as
TryFrom
<
u8
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
u8
Source
§
fn
try_from
(u:
usize
) ->
Result
<
u8
, <
u8
as
TryFrom
<
usize
>>::
Error
>
Tries to create the target number type from a source
number type. This returns an error if the source value
is outside of the range of the target type.
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
1.42.0
·
Source
§
impl
UpperExp
for
u8
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
impl
UpperHex
for
u8
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
Source
§
impl
ConstParamTy_
for
u8
1.0.0
·
Source
§
impl
Copy
for
u8
1.0.0
·
Source
§
impl
Eq
for
u8
Source
§
impl
FloatToInt
<
u8
> for
f128
Source
§
impl
FloatToInt
<
u8
> for
f16
Source
§
impl
FloatToInt
<
u8
> for
f32
Source
§
impl
FloatToInt
<
u8
> for
f64
Source
§
impl
SimdCast
for
u8
Source
§
impl
StructuralPartialEq
for
u8
Source
§
impl
TrustedStep
for
u8
Source
§
impl
UnsizedConstParamTy
for
u8
Source
§
impl
UseCloned
for
u8
Source
§
impl
ZeroablePrimitive
for
u8
Auto Trait Implementations
§
§
impl
Freeze
for
u8
§
impl
RefUnwindSafe
for
u8
§
impl
Send
for
u8
§
impl
Sync
for
u8
§
impl
Unpin
for
u8
§
impl
UnwindSafe
for
u8
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