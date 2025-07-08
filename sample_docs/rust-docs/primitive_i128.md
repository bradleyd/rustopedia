i128 - Rust
Primitive Type
i128
Copy item path
1.26.0
Expand description
The 128-bit signed integer type.
Implementations
§
Source
§
impl
i128
1.43.0
·
Source
pub const
MIN
:
i128
= -170_141_183_460_469_231_731_687_303_715_884_105_728i128
The smallest value that can be represented by this integer type
(−2
127
).
§
Examples
Basic usage:
assert_eq!
(i128::MIN, -
170141183460469231731687303715884105728
);
1.43.0
·
Source
pub const
MAX
:
i128
= 170_141_183_460_469_231_731_687_303_715_884_105_727i128
The largest value that can be represented by this integer type
(2
127
− 1).
§
Examples
Basic usage:
assert_eq!
(i128::MAX,
170141183460469231731687303715884105727
);
1.53.0
·
Source
pub const
BITS
:
u32
= 128u32
The size of this integer type in bits.
§
Examples
assert_eq!
(i128::BITS,
128
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
0b100_0000i128
;
assert_eq!
(n.count_ones(),
1
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
assert_eq!
(i128::MAX.count_zeros(),
1
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
n = -
1i128
;
assert_eq!
(n.leading_zeros(),
0
);
1.0.0 (const: 1.32.0)
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
let
n = -
4i128
;
assert_eq!
(n.trailing_zeros(),
2
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
n = -
1i128
;
assert_eq!
(n.leading_ones(),
128
);
1.46.0 (const: 1.46.0)
·
Source
pub const fn
trailing_ones
(self) ->
u32
Returns the number of trailing ones in the binary representation of
self
.
§
Examples
Basic usage:
let
n =
3i128
;
assert_eq!
(n.trailing_ones(),
2
);
Source
pub const fn
isolate_most_significant_one
(self) ->
i128
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
n: i128 =
0b_01100100
;
assert_eq!
(n.isolate_most_significant_one(),
0b_01000000
);
assert_eq!
(
0_i128
.isolate_most_significant_one(),
0
);
Source
pub const fn
isolate_least_significant_one
(self) ->
i128
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
n: i128 =
0b_01100100
;
assert_eq!
(n.isolate_least_significant_one(),
0b_00000100
);
assert_eq!
(
0_i128
.isolate_least_significant_one(),
0
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
u128
Returns the bit pattern of
self
reinterpreted as an unsigned integer of the same size.
This produces the same result as an
as
cast, but ensures that the bit-width remains
the same.
§
Examples
Basic usage:
let
n = -
1i128
;
assert_eq!
(n.cast_unsigned(), u128::MAX);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
i128
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
0x13f40000000000000000000000004f76i128
;
let
m =
0x4f7613f4
;
assert_eq!
(n.rotate_left(
16
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
i128
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
0x4f7613f4i128
;
let
m =
0x13f40000000000000000000000004f76
;
assert_eq!
(n.rotate_right(
16
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
swap_bytes
(self) ->
i128
Reverses the byte order of the integer.
§
Examples
Basic usage:
let
n =
0x12345678901234567890123456789012i128
;
let
m = n.swap_bytes();
assert_eq!
(m,
0x12907856341290785634129078563412
);
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
i128
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
let
n =
0x12345678901234567890123456789012i128
;
let
m = n.reverse_bits();
assert_eq!
(m,
0x48091e6a2c48091e6a2c48091e6a2c48
);
assert_eq!
(
0
,
0i128
.reverse_bits());
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_be
(x:
i128
) ->
i128
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Ai128
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(i128::from_be(n), n)
}
else
{
assert_eq!
(i128::from_be(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_le
(x:
i128
) ->
i128
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Ai128
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(i128::from_le(n), n)
}
else
{
assert_eq!
(i128::from_le(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_be
(self) ->
i128
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Ai128
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
i128
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Ai128
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
i128
) ->
Option
<
i128
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
((i128::MAX -
2
).checked_add(
1
),
Some
(i128::MAX -
1
));
assert_eq!
((i128::MAX -
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
i128
) ->
i128
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
((i128::MAX -
2
).strict_add(
1
), i128::MAX -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (i128::MAX -
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
i128
) ->
i128
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
self + rhs > i128::MAX
or
self + rhs < i128::MIN
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
checked_add_unsigned
(self, rhs:
u128
) ->
Option
<
i128
>
Checked addition with an unsigned integer. Computes
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
1i128
.checked_add_unsigned(
2
),
Some
(
3
));
assert_eq!
((i128::MAX -
2
).checked_add_unsigned(
3
),
None
);
Source
pub const fn
strict_add_unsigned
(self, rhs:
u128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict addition with an unsigned integer. Computes
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
1i128
.strict_add_unsigned(
2
),
3
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (i128::MAX -
2
).strict_add_unsigned(
3
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
checked_sub
(self, rhs:
i128
) ->
Option
<
i128
>
Checked integer subtraction. Computes
self - rhs
, returning
None
if
overflow occurred.
§
Examples
Basic usage:
assert_eq!
((i128::MIN +
2
).checked_sub(
1
),
Some
(i128::MIN +
1
));
assert_eq!
((i128::MIN +
2
).checked_sub(
3
),
None
);
Source
pub const fn
strict_sub
(self, rhs:
i128
) ->
i128
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
((i128::MIN +
2
).strict_sub(
1
), i128::MIN +
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (i128::MIN +
2
).strict_sub(
3
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_sub
(self, rhs:
i128
) ->
i128
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
§
Safety
This results in undefined behavior when
self - rhs > i128::MAX
or
self - rhs < i128::MIN
,
i.e. when
checked_sub
would return
None
.
1.66.0 (const: 1.66.0)
·
Source
pub const fn
checked_sub_unsigned
(self, rhs:
u128
) ->
Option
<
i128
>
Checked subtraction with an unsigned integer. Computes
self - rhs
,
returning
None
if overflow occurred.
§
Examples
Basic usage:
assert_eq!
(
1i128
.checked_sub_unsigned(
2
),
Some
(-
1
));
assert_eq!
((i128::MIN +
2
).checked_sub_unsigned(
3
),
None
);
Source
pub const fn
strict_sub_unsigned
(self, rhs:
u128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict subtraction with an unsigned integer. Computes
self - rhs
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
1i128
.strict_sub_unsigned(
2
), -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (i128::MIN +
2
).strict_sub_unsigned(
3
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
checked_mul
(self, rhs:
i128
) ->
Option
<
i128
>
Checked integer multiplication. Computes
self * rhs
, returning
None
if
overflow occurred.
§
Examples
Basic usage:
assert_eq!
(i128::MAX.checked_mul(
1
),
Some
(i128::MAX));
assert_eq!
(i128::MAX.checked_mul(
2
),
None
);
Source
pub const fn
strict_mul
(self, rhs:
i128
) ->
i128
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
(i128::MAX.strict_mul(
1
), i128::MAX);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MAX.strict_mul(
2
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_mul
(self, rhs:
i128
) ->
i128
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
self * rhs > i128::MAX
or
self * rhs < i128::MIN
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
i128
) ->
Option
<
i128
>
Checked integer division. Computes
self / rhs
, returning
None
if
rhs == 0
or the division results in overflow.
§
Examples
Basic usage:
assert_eq!
((i128::MIN +
1
).checked_div(-
1
),
Some
(
170141183460469231731687303715884105727
));
assert_eq!
(i128::MIN.checked_div(-
1
),
None
);
assert_eq!
((
1i128
).checked_div(
0
),
None
);
Source
pub const fn
strict_div
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer division. Computes
self / rhs
, panicking
if overflow occurred.
§
Panics
This function will panic if
rhs
is zero.
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
The only case where such an overflow can occur is when one divides
MIN / -1
on a signed type (where
MIN
is the negative minimal value for the type); this is equivalent to
-MIN
, a positive value
that is too large to represent in the type.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
((i128::MIN +
1
).strict_div(-
1
),
170141183460469231731687303715884105727
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_div(-
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1i128
).strict_div(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_div_euclid
(self, rhs:
i128
) ->
Option
<
i128
>
Checked Euclidean division. Computes
self.div_euclid(rhs)
,
returning
None
if
rhs == 0
or the division results in overflow.
§
Examples
Basic usage:
assert_eq!
((i128::MIN +
1
).checked_div_euclid(-
1
),
Some
(
170141183460469231731687303715884105727
));
assert_eq!
(i128::MIN.checked_div_euclid(-
1
),
None
);
assert_eq!
((
1i128
).checked_div_euclid(
0
),
None
);
Source
pub const fn
strict_div_euclid
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict Euclidean division. Computes
self.div_euclid(rhs)
, panicking
if overflow occurred.
§
Panics
This function will panic if
rhs
is zero.
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
The only case where such an overflow can occur is when one divides
MIN / -1
on a signed type (where
MIN
is the negative minimal value for the type); this is equivalent to
-MIN
, a positive value
that is too large to represent in the type.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
((i128::MIN +
1
).strict_div_euclid(-
1
),
170141183460469231731687303715884105727
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_div_euclid(-
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1i128
).strict_div_euclid(
0
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem
(self, rhs:
i128
) ->
Option
<
i128
>
Checked integer remainder. Computes
self % rhs
, returning
None
if
rhs == 0
or the division results in overflow.
§
Examples
Basic usage:
assert_eq!
(
5i128
.checked_rem(
2
),
Some
(
1
));
assert_eq!
(
5i128
.checked_rem(
0
),
None
);
assert_eq!
(i128::MIN.checked_rem(-
1
),
None
);
Source
pub const fn
strict_rem
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict integer remainder. Computes
self % rhs
, panicking if
the division results in overflow.
§
Panics
This function will panic if
rhs
is zero.
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
The only case where such an overflow can occur is
x % y
for
MIN / -1
on a
signed type (where
MIN
is the negative minimal value), which is invalid due to implementation artifacts.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
5i128
.strict_rem(
2
),
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
5i128
.strict_rem(
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_rem(-
1
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem_euclid
(self, rhs:
i128
) ->
Option
<
i128
>
Checked Euclidean remainder. Computes
self.rem_euclid(rhs)
, returning
None
if
rhs == 0
or the division results in overflow.
§
Examples
Basic usage:
assert_eq!
(
5i128
.checked_rem_euclid(
2
),
Some
(
1
));
assert_eq!
(
5i128
.checked_rem_euclid(
0
),
None
);
assert_eq!
(i128::MIN.checked_rem_euclid(-
1
),
None
);
Source
pub const fn
strict_rem_euclid
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict Euclidean remainder. Computes
self.rem_euclid(rhs)
, panicking if
the division results in overflow.
§
Panics
This function will panic if
rhs
is zero.
§
Overflow behavior
This function will always panic on overflow, regardless of whether overflow checks are enabled.
The only case where such an overflow can occur is
x % y
for
MIN / -1
on a
signed type (where
MIN
is the negative minimal value), which is invalid due to implementation artifacts.
§
Examples
Basic usage:
#![feature(strict_overflow_ops)]
assert_eq!
(
5i128
.strict_rem_euclid(
2
),
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
5i128
.strict_rem_euclid(
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_rem_euclid(-
1
);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
checked_neg
(self) ->
Option
<
i128
>
Checked negation. Computes
-self
, returning
None
if
self == MIN
.
§
Examples
Basic usage:
assert_eq!
(
5i128
.checked_neg(),
Some
(-
5
));
assert_eq!
(i128::MIN.checked_neg(),
None
);
Source
pub const unsafe fn
unchecked_neg
(self) ->
i128
🔬
This is a nightly-only experimental API. (
unchecked_neg
#85122
)
Unchecked negation. Computes
-self
, assuming overflow cannot occur.
§
Safety
This results in undefined behavior when
self == i128::MIN
,
i.e. when
checked_neg
would return
None
.
Source
pub const fn
strict_neg
(self) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict negation. Computes
-self
, panicking if
self == MIN
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
5i128
.strict_neg(), -
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_neg();
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
i128
>
Checked shift left. Computes
self << rhs
, returning
None
if
rhs
is larger
than or equal to the number of bits in
self
.
§
Examples
Basic usage:
assert_eq!
(
0x1i128
.checked_shl(
4
),
Some
(
0x10
));
assert_eq!
(
0x1i128
.checked_shl(
129
),
None
);
assert_eq!
(
0x10i128
.checked_shl(
127
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
i128
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
0x1i128
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
0x1i128
.strict_shl(
129
);
Source
pub const unsafe fn
unchecked_shl
(self, rhs:
u32
) ->
i128
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
i128
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
0x1i128
.unbounded_shl(
4
),
0x10
);
assert_eq!
(
0x1i128
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
i128
>
Checked shift right. Computes
self >> rhs
, returning
None
if
rhs
is
larger than or equal to the number of bits in
self
.
§
Examples
Basic usage:
assert_eq!
(
0x10i128
.checked_shr(
4
),
Some
(
0x1
));
assert_eq!
(
0x10i128
.checked_shr(
128
),
None
);
Source
pub const fn
strict_shr
(self, rhs:
u32
) ->
i128
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
0x10i128
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
0x10i128
.strict_shr(
128
);
Source
pub const unsafe fn
unchecked_shr
(self, rhs:
u32
) ->
i128
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
i128
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
the entire value is shifted out, which yields
0
for a positive number,
and
-1
for a negative number.
§
Examples
Basic usage:
assert_eq!
(
0x10i128
.unbounded_shr(
4
),
0x1
);
assert_eq!
(
0x10i128
.unbounded_shr(
129
),
0
);
assert_eq!
(i128::MIN.unbounded_shr(
129
), -
1
);
1.13.0 (const: 1.47.0)
·
Source
pub const fn
checked_abs
(self) ->
Option
<
i128
>
Checked absolute value. Computes
self.abs()
, returning
None
if
self == MIN
.
§
Examples
Basic usage:
assert_eq!
((-
5i128
).checked_abs(),
Some
(
5
));
assert_eq!
(i128::MIN.checked_abs(),
None
);
Source
pub const fn
strict_abs
(self) ->
i128
🔬
This is a nightly-only experimental API. (
strict_overflow_ops
#118260
)
Strict absolute value. Computes
self.abs()
, panicking if
self == MIN
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
((-
5i128
).strict_abs(),
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MIN.strict_abs();
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
i128
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
8i128
.checked_pow(
2
),
Some
(
64
));
assert_eq!
(i128::MAX.checked_pow(
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
i128
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
8i128
.strict_pow(
2
),
64
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= i128::MAX.strict_pow(
2
);
1.84.0 (const: 1.84.0)
·
Source
pub const fn
checked_isqrt
(self) ->
Option
<
i128
>
Returns the square root of the number, rounded down.
Returns
None
if
self
is negative.
§
Examples
Basic usage:
assert_eq!
(
10i128
.checked_isqrt(),
Some
(
3
));
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_add
(self, rhs:
i128
) ->
i128
Saturating integer addition. Computes
self + rhs
, saturating at the numeric
bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100i128
.saturating_add(
1
),
101
);
assert_eq!
(i128::MAX.saturating_add(
100
), i128::MAX);
assert_eq!
(i128::MIN.saturating_add(-
1
), i128::MIN);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_add_unsigned
(self, rhs:
u128
) ->
i128
Saturating addition with an unsigned integer. Computes
self + rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
1i128
.saturating_add_unsigned(
2
),
3
);
assert_eq!
(i128::MAX.saturating_add_unsigned(
100
), i128::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
i128
) ->
i128
Saturating integer subtraction. Computes
self - rhs
, saturating at the
numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100i128
.saturating_sub(
127
), -
27
);
assert_eq!
(i128::MIN.saturating_sub(
100
), i128::MIN);
assert_eq!
(i128::MAX.saturating_sub(-
1
), i128::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_sub_unsigned
(self, rhs:
u128
) ->
i128
Saturating subtraction with an unsigned integer. Computes
self - rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100i128
.saturating_sub_unsigned(
127
), -
27
);
assert_eq!
(i128::MIN.saturating_sub_unsigned(
100
), i128::MIN);
1.45.0 (const: 1.47.0)
·
Source
pub const fn
saturating_neg
(self) ->
i128
Saturating integer negation. Computes
-self
, returning
MAX
if
self == MIN
instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100i128
.saturating_neg(), -
100
);
assert_eq!
((-
100i128
).saturating_neg(),
100
);
assert_eq!
(i128::MIN.saturating_neg(), i128::MAX);
assert_eq!
(i128::MAX.saturating_neg(), i128::MIN +
1
);
1.45.0 (const: 1.47.0)
·
Source
pub const fn
saturating_abs
(self) ->
i128
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
assert_eq!
(
100i128
.saturating_abs(),
100
);
assert_eq!
((-
100i128
).saturating_abs(),
100
);
assert_eq!
(i128::MIN.saturating_abs(), i128::MAX);
assert_eq!
((i128::MIN +
1
).saturating_abs(), i128::MAX);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
i128
) ->
i128
Saturating integer multiplication. Computes
self * rhs
, saturating at the
numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
10i128
.saturating_mul(
12
),
120
);
assert_eq!
(i128::MAX.saturating_mul(
10
), i128::MAX);
assert_eq!
(i128::MIN.saturating_mul(
10
), i128::MIN);
1.58.0 (const: 1.58.0)
·
Source
pub const fn
saturating_div
(self, rhs:
i128
) ->
i128
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
5i128
.saturating_div(
2
),
2
);
assert_eq!
(i128::MAX.saturating_div(-
1
), i128::MIN +
1
);
assert_eq!
(i128::MIN.saturating_div(-
1
), i128::MAX);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
saturating_pow
(self, exp:
u32
) ->
i128
Saturating integer exponentiation. Computes
self.pow(exp)
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
((-
4i128
).saturating_pow(
3
), -
64
);
assert_eq!
(i128::MIN.saturating_pow(
2
), i128::MAX);
assert_eq!
(i128::MIN.saturating_pow(
3
), i128::MIN);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_add
(self, rhs:
i128
) ->
i128
Wrapping (modular) addition. Computes
self + rhs
, wrapping around at the
boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100i128
.wrapping_add(
27
),
127
);
assert_eq!
(i128::MAX.wrapping_add(
2
), i128::MIN +
1
);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_add_unsigned
(self, rhs:
u128
) ->
i128
Wrapping (modular) addition with an unsigned integer. Computes
self + rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100i128
.wrapping_add_unsigned(
27
),
127
);
assert_eq!
(i128::MAX.wrapping_add_unsigned(
2
), i128::MIN +
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_sub
(self, rhs:
i128
) ->
i128
Wrapping (modular) subtraction. Computes
self - rhs
, wrapping around at the
boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
0i128
.wrapping_sub(
127
), -
127
);
assert_eq!
((-
2i128
).wrapping_sub(i128::MAX), i128::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_sub_unsigned
(self, rhs:
u128
) ->
i128
Wrapping (modular) subtraction with an unsigned integer. Computes
self - rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
0i128
.wrapping_sub_unsigned(
127
), -
127
);
assert_eq!
((-
2i128
).wrapping_sub_unsigned(u128::MAX), -
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_mul
(self, rhs:
i128
) ->
i128
Wrapping (modular) multiplication. Computes
self * rhs
, wrapping around at
the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
10i128
.wrapping_mul(
12
),
120
);
assert_eq!
(
11i8
.wrapping_mul(
12
), -
124
);
1.2.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_div
(self, rhs:
i128
) ->
i128
Wrapping (modular) division. Computes
self / rhs
, wrapping around at the
boundary of the type.
The only case where such wrapping can occur is when one divides
MIN / -1
on a signed type (where
MIN
is the negative minimal value for the type); this is equivalent to
-MIN
, a positive value
that is too large to represent in the type. In such a case, this function returns
MIN
itself.
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
100i128
.wrapping_div(
10
),
10
);
assert_eq!
((-
128i8
).wrapping_div(-
1
), -
128
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_div_euclid
(self, rhs:
i128
) ->
i128
Wrapping Euclidean division. Computes
self.div_euclid(rhs)
,
wrapping around at the boundary of the type.
Wrapping will only occur in
MIN / -1
on a signed type (where
MIN
is the negative minimal value
for the type). This is equivalent to
-MIN
, a positive value that is too large to represent in the
type. In this case, this method returns
MIN
itself.
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
100i128
.wrapping_div_euclid(
10
),
10
);
assert_eq!
((-
128i8
).wrapping_div_euclid(-
1
), -
128
);
1.2.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_rem
(self, rhs:
i128
) ->
i128
Wrapping (modular) remainder. Computes
self % rhs
, wrapping around at the
boundary of the type.
Such wrap-around never actually occurs mathematically; implementation artifacts make
x % y
invalid for
MIN / -1
on a signed type (where
MIN
is the negative minimal value). In such a case,
this function returns
0
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
100i128
.wrapping_rem(
10
),
0
);
assert_eq!
((-
128i8
).wrapping_rem(-
1
),
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
wrapping_rem_euclid
(self, rhs:
i128
) ->
i128
Wrapping Euclidean remainder. Computes
self.rem_euclid(rhs)
, wrapping around
at the boundary of the type.
Wrapping will only occur in
MIN % -1
on a signed type (where
MIN
is the negative minimal value
for the type). In this case, this method returns 0.
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
100i128
.wrapping_rem_euclid(
10
),
0
);
assert_eq!
((-
128i8
).wrapping_rem_euclid(-
1
),
0
);
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_neg
(self) ->
i128
Wrapping (modular) negation. Computes
-self
, wrapping around at the boundary
of the type.
The only case where such wrapping can occur is when one negates
MIN
on a signed type (where
MIN
is the negative minimal value for the type); this is a positive value that is too large to represent
in the type. In such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
assert_eq!
(
100i128
.wrapping_neg(), -
100
);
assert_eq!
((-
100i128
).wrapping_neg(),
100
);
assert_eq!
(i128::MIN.wrapping_neg(), i128::MIN);
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_shl
(self, rhs:
u32
) ->
i128
Panic-free bitwise shift-left; yields
self << mask(rhs)
, where
mask
removes
any high-order bits of
rhs
that would cause the shift to exceed the bitwidth of the type.
Note that this is
not
the same as a rotate-left; the RHS of a wrapping shift-left is restricted to
the range of the type, rather than the bits shifted out of the LHS being returned to the other end.
The primitive integer types all implement a
rotate_left
function,
which may be what you want instead.
§
Examples
Basic usage:
assert_eq!
((-
1i128
).wrapping_shl(
7
), -
128
);
assert_eq!
((-
1i128
).wrapping_shl(
128
), -
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
i128
Panic-free bitwise shift-right; yields
self >> mask(rhs)
, where
mask
removes any high-order bits of
rhs
that would cause the shift to exceed the bitwidth of the type.
Note that this is
not
the same as a rotate-right; the RHS of a wrapping shift-right is restricted
to the range of the type, rather than the bits shifted out of the LHS being returned to the other
end. The primitive integer types all implement a
rotate_right
function,
which may be what you want instead.
§
Examples
Basic usage:
assert_eq!
((-
128i128
).wrapping_shr(
7
), -
1
);
assert_eq!
((-
128i16
).wrapping_shr(
64
), -
128
);
1.13.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_abs
(self) ->
i128
Wrapping (modular) absolute value. Computes
self.abs()
, wrapping around at
the boundary of the type.
The only case where such wrapping can occur is when one takes the absolute value of the negative
minimal value for the type; this is a positive value that is too large to represent in the type. In
such a case, this function returns
MIN
itself.
§
Examples
Basic usage:
assert_eq!
(
100i128
.wrapping_abs(),
100
);
assert_eq!
((-
100i128
).wrapping_abs(),
100
);
assert_eq!
(i128::MIN.wrapping_abs(), i128::MIN);
assert_eq!
((-
128i8
).wrapping_abs()
as
u8,
128
);
1.51.0 (const: 1.51.0)
·
Source
pub const fn
unsigned_abs
(self) ->
u128
Computes the absolute value of
self
without any wrapping
or panicking.
§
Examples
Basic usage:
assert_eq!
(
100i128
.unsigned_abs(),
100u128
);
assert_eq!
((-
100i128
).unsigned_abs(),
100u128
);
assert_eq!
((-
128i8
).unsigned_abs(),
128u8
);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
wrapping_pow
(self, exp:
u32
) ->
i128
Wrapping (modular) exponentiation. Computes
self.pow(exp)
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
3i128
.wrapping_pow(
4
),
81
);
assert_eq!
(
3i8
.wrapping_pow(
5
), -
13
);
assert_eq!
(
3i8
.wrapping_pow(
6
), -
39
);
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_add
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates
self
+
rhs
.
Returns a tuple of the addition along with a boolean indicating
whether an arithmetic overflow would occur. If an overflow would have
occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
5i128
.overflowing_add(
2
), (
7
,
false
));
assert_eq!
(i128::MAX.overflowing_add(
1
), (i128::MIN,
true
));
Source
pub const fn
carrying_add
(self, rhs:
i128
, carry:
bool
) -> (
i128
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
and checks for overflow.
Performs “ternary addition” of two integer operands and a carry-in
bit, and returns a tuple of the sum along with a boolean indicating
whether an arithmetic overflow would occur. On overflow, the wrapped
value is returned.
This allows chaining together multiple additions to create a wider
addition, and can be useful for bignum addition. This method should
only be used for the most significant word; for the less significant
words the unsigned method
u128::carrying_add
should be used.
The output boolean returned by this method is
not
a carry flag,
and should
not
be added to a more significant word.
If the input carry is false, this method is equivalent to
overflowing_add
.
§
Examples
#![feature(bigint_helper_methods)]
// Only the most significant word is signed.
//
//   10  MAX    (a = 10 × 2^128 + 2^128 - 1)
// + -5    9    (b = -5 × 2^128 + 9)
// ---------
//    6    8    (sum = 6 × 2^128 + 8)
let
(a1, a0): (i128, u128) = (
10
, u128::MAX);
let
(b1, b0): (i128, u128) = (-
5
,
9
);
let
carry0 =
false
;
// u128::carrying_add for the less significant words
let
(sum0, carry1) = a0.carrying_add(b0, carry0);
assert_eq!
(carry1,
true
);
// i128::carrying_add for the most significant word
let
(sum1, overflow) = a1.carrying_add(b1, carry1);
assert_eq!
(overflow,
false
);
assert_eq!
((sum1, sum0), (
6
,
8
));
1.66.0 (const: 1.66.0)
·
Source
pub const fn
overflowing_add_unsigned
(self, rhs:
u128
) -> (
i128
,
bool
)
Calculates
self
+
rhs
with an unsigned
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
1i128
.overflowing_add_unsigned(
2
), (
3
,
false
));
assert_eq!
((i128::MIN).overflowing_add_unsigned(u128::MAX), (i128::MAX,
false
));
assert_eq!
((i128::MAX -
2
).overflowing_add_unsigned(
3
), (i128::MIN,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_sub
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates
self
-
rhs
.
Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow
would occur. If an overflow would have occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
5i128
.overflowing_sub(
2
), (
3
,
false
));
assert_eq!
(i128::MIN.overflowing_sub(
1
), (i128::MAX,
true
));
Source
pub const fn
borrowing_sub
(self, rhs:
i128
, borrow:
bool
) -> (
i128
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
and checks for
overflow.
Performs “ternary subtraction” by subtracting both an integer
operand and a borrow-in bit from
self
, and returns a tuple of the
difference along with a boolean indicating whether an arithmetic
overflow would occur. On overflow, the wrapped value is returned.
This allows chaining together multiple subtractions to create a
wider subtraction, and can be useful for bignum subtraction. This
method should only be used for the most significant word; for the
less significant words the unsigned method
u128::borrowing_sub
should be used.
The output boolean returned by this method is
not
a borrow flag,
and should
not
be subtracted from a more significant word.
If the input borrow is false, this method is equivalent to
overflowing_sub
.
§
Examples
#![feature(bigint_helper_methods)]
// Only the most significant word is signed.
//
//    6    8    (a = 6 × 2^128 + 8)
// - -5    9    (b = -5 × 2^128 + 9)
// ---------
//   10  MAX    (diff = 10 × 2^128 + 2^128 - 1)
let
(a1, a0): (i128, u128) = (
6
,
8
);
let
(b1, b0): (i128, u128) = (-
5
,
9
);
let
borrow0 =
false
;
// u128::borrowing_sub for the less significant words
let
(diff0, borrow1) = a0.borrowing_sub(b0, borrow0);
assert_eq!
(borrow1,
true
);
// i128::borrowing_sub for the most significant word
let
(diff1, overflow) = a1.borrowing_sub(b1, borrow1);
assert_eq!
(overflow,
false
);
assert_eq!
((diff1, diff0), (
10
, u128::MAX));
1.66.0 (const: 1.66.0)
·
Source
pub const fn
overflowing_sub_unsigned
(self, rhs:
u128
) -> (
i128
,
bool
)
Calculates
self
-
rhs
with an unsigned
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
1i128
.overflowing_sub_unsigned(
2
), (-
1
,
false
));
assert_eq!
((i128::MAX).overflowing_sub_unsigned(u128::MAX), (i128::MIN,
false
));
assert_eq!
((i128::MIN +
2
).overflowing_sub_unsigned(
3
), (i128::MAX,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_mul
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates the multiplication of
self
and
rhs
.
Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow
would occur. If an overflow would have occurred then the wrapped value is returned.
§
Examples
Basic usage:
assert_eq!
(
5i128
.overflowing_mul(
2
), (
10
,
false
));
assert_eq!
(
1_000_000_000i32
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
i128
) -> (
u128
,
i128
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
i32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5i32
.widening_mul(-
2
), (
4294967286
, -
1
));
assert_eq!
(
1_000_000_000i32
.widening_mul(-
10
), (
2884901888
, -
3
));
Source
pub const fn
carrying_mul
(self, rhs:
i128
, carry:
i128
) -> (
u128
,
i128
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
i32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5i32
.carrying_mul(-
2
,
0
), (
4294967286
, -
1
));
assert_eq!
(
5i32
.carrying_mul(-
2
,
10
), (
0
,
0
));
assert_eq!
(
1_000_000_000i32
.carrying_mul(-
10
,
0
), (
2884901888
, -
3
));
assert_eq!
(
1_000_000_000i32
.carrying_mul(-
10
,
10
), (
2884901898
, -
3
));
assert_eq!
(i128::MAX.carrying_mul(i128::MAX, i128::MAX), (i128::MAX.unsigned_abs() +
1
, i128::MAX /
2
));
Source
pub const fn
carrying_mul_add
(
    self,
    rhs:
i128
,
    carry:
i128
,
    add:
i128
,
) -> (
u128
,
i128
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
Please note that this example is shared between integer types.
Which explains why
i32
is used here.
#![feature(bigint_helper_methods)]
assert_eq!
(
5i32
.carrying_mul_add(-
2
,
0
,
0
), (
4294967286
, -
1
));
assert_eq!
(
5i32
.carrying_mul_add(-
2
,
10
,
10
), (
10
,
0
));
assert_eq!
(
1_000_000_000i32
.carrying_mul_add(-
10
,
0
,
0
), (
2884901888
, -
3
));
assert_eq!
(
1_000_000_000i32
.carrying_mul_add(-
10
,
10
,
10
), (
2884901908
, -
3
));
assert_eq!
(i128::MAX.carrying_mul_add(i128::MAX, i128::MAX, i128::MAX), (u128::MAX, i128::MAX /
2
));
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates the divisor when
self
is divided by
rhs
.
Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
occur. If an overflow would occur then self is returned.
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
5i128
.overflowing_div(
2
), (
2
,
false
));
assert_eq!
(i128::MIN.overflowing_div(-
1
), (i128::MIN,
true
));
1.38.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div_euclid
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates the quotient of Euclidean division
self.div_euclid(rhs)
.
Returns a tuple of the divisor along with a boolean indicating whether an arithmetic overflow would
occur. If an overflow would occur then
self
is returned.
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
5i128
.overflowing_div_euclid(
2
), (
2
,
false
));
assert_eq!
(i128::MIN.overflowing_div_euclid(-
1
), (i128::MIN,
true
));
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_rem
(self, rhs:
i128
) -> (
i128
,
bool
)
Calculates the remainder when
self
is divided by
rhs
.
Returns a tuple of the remainder after dividing along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then 0 is returned.
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
5i128
.overflowing_rem(
2
), (
1
,
false
));
assert_eq!
(i128::MIN.overflowing_rem(-
1
), (
0
,
true
));
1.38.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_rem_euclid
(self, rhs:
i128
) -> (
i128
,
bool
)
Overflowing Euclidean remainder. Calculates
self.rem_euclid(rhs)
.
Returns a tuple of the remainder after dividing along with a boolean indicating whether an
arithmetic overflow would occur. If an overflow would occur then 0 is returned.
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
5i128
.overflowing_rem_euclid(
2
), (
1
,
false
));
assert_eq!
(i128::MIN.overflowing_rem_euclid(-
1
), (
0
,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_neg
(self) -> (
i128
,
bool
)
Negates self, overflowing if this is equal to the minimum value.
Returns a tuple of the negated version of self along with a boolean indicating whether an overflow
happened. If
self
is the minimum value (e.g.,
i32::MIN
for values of type
i32
), then the
minimum value will be returned again and
true
will be returned for an overflow happening.
§
Examples
Basic usage:
assert_eq!
(
2i128
.overflowing_neg(), (-
2
,
false
));
assert_eq!
(i128::MIN.overflowing_neg(), (i128::MIN,
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
i128
,
bool
)
Shifts self left by
rhs
bits.
Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
value was larger than or equal to the number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
§
Examples
Basic usage:
assert_eq!
(
0x1i128
.overflowing_shl(
4
), (
0x10
,
false
));
assert_eq!
(
0x1i32
.overflowing_shl(
36
), (
0x10
,
true
));
assert_eq!
(
0x10i128
.overflowing_shl(
127
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
i128
,
bool
)
Shifts self right by
rhs
bits.
Returns a tuple of the shifted version of self along with a boolean indicating whether the shift
value was larger than or equal to the number of bits. If the shift value is too large, then value is
masked (N-1) where N is the number of bits, and this value is then used to perform the shift.
§
Examples
Basic usage:
assert_eq!
(
0x10i128
.overflowing_shr(
4
), (
0x1
,
false
));
assert_eq!
(
0x10i32
.overflowing_shr(
36
), (
0x1
,
true
));
1.13.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_abs
(self) -> (
i128
,
bool
)
Computes the absolute value of
self
.
Returns a tuple of the absolute version of self along with a boolean indicating whether an overflow
happened. If self is the minimum value
(e.g., i128::MIN for values of type i128),
then the minimum value will be returned again and true will be returned
for an overflow happening.
§
Examples
Basic usage:
assert_eq!
(
10i128
.overflowing_abs(), (
10
,
false
));
assert_eq!
((-
10i128
).overflowing_abs(), (
10
,
false
));
assert_eq!
((i128::MIN).overflowing_abs(), (i128::MIN,
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
i128
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
3i128
.overflowing_pow(
4
), (
81
,
false
));
assert_eq!
(
3i8
.overflowing_pow(
5
), (-
13
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
i128
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
let
x: i128 =
2
;
// or any other integer type
assert_eq!
(x.pow(
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
i128
Returns the square root of the number, rounded down.
§
Panics
This function will panic if
self
is negative.
§
Examples
Basic usage:
assert_eq!
(
10i128
.isqrt(),
3
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
div_euclid
(self, rhs:
i128
) ->
i128
Calculates the quotient of Euclidean division of
self
by
rhs
.
This computes the integer
q
such that
self = q * rhs + r
, with
r = self.rem_euclid(rhs)
and
0 <= r < abs(rhs)
.
In other words, the result is
self / rhs
rounded to the integer
q
such that
self >= q * rhs
.
If
self > 0
, this is equal to rounding towards zero (the default in Rust);
if
self < 0
, this is equal to rounding away from zero (towards +/- infinity).
If
rhs > 0
, this is equal to rounding towards -infinity;
if
rhs < 0
, this is equal to rounding towards +infinity.
§
Panics
This function will panic if
rhs
is zero or if
self
is
Self::MIN
and
rhs
is -1. This behavior is not affected by the
overflow-checks
flag.
§
Examples
Basic usage:
let
a: i128 =
7
;
// or any other integer type
let
b =
4
;
assert_eq!
(a.div_euclid(b),
1
);
// 7 >= 4 * 1
assert_eq!
(a.div_euclid(-b), -
1
);
// 7 >= -4 * -1
assert_eq!
((-a).div_euclid(b), -
2
);
// -7 >= 4 * -2
assert_eq!
((-a).div_euclid(-b),
2
);
// -7 >= -4 * 2
1.38.0 (const: 1.52.0)
·
Source
pub const fn
rem_euclid
(self, rhs:
i128
) ->
i128
Calculates the least nonnegative remainder of
self (mod rhs)
.
This is done as if by the Euclidean division algorithm – given
r = self.rem_euclid(rhs)
, the result satisfies
self = rhs * self.div_euclid(rhs) + r
and
0 <= r < abs(rhs)
.
§
Panics
This function will panic if
rhs
is zero or if
self
is
Self::MIN
and
rhs
is -1. This behavior is not affected by the
overflow-checks
flag.
§
Examples
Basic usage:
let
a: i128 =
7
;
// or any other integer type
let
b =
4
;
assert_eq!
(a.rem_euclid(b),
3
);
assert_eq!
((-a).rem_euclid(b),
1
);
assert_eq!
(a.rem_euclid(-b),
3
);
assert_eq!
((-a).rem_euclid(-b),
1
);
This will panic:
ⓘ
let _
= i128::MIN.rem_euclid(-
1
);
Source
pub const fn
div_floor
(self, rhs:
i128
) ->
i128
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
§
Panics
This function will panic if
rhs
is zero or if
self
is
Self::MIN
and
rhs
is -1. This behavior is not affected by the
overflow-checks
flag.
§
Examples
Basic usage:
#![feature(int_roundings)]
let
a: i128 =
8
;
let
b =
3
;
assert_eq!
(a.div_floor(b),
2
);
assert_eq!
(a.div_floor(-b), -
3
);
assert_eq!
((-a).div_floor(b), -
3
);
assert_eq!
((-a).div_floor(-b),
2
);
Source
pub const fn
div_ceil
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
int_roundings
#88581
)
Calculates the quotient of
self
and
rhs
, rounding the result towards positive infinity.
§
Panics
This function will panic if
rhs
is zero or if
self
is
Self::MIN
and
rhs
is -1. This behavior is not affected by the
overflow-checks
flag.
§
Examples
Basic usage:
#![feature(int_roundings)]
let
a: i128 =
8
;
let
b =
3
;
assert_eq!
(a.div_ceil(b),
3
);
assert_eq!
(a.div_ceil(-b), -
2
);
assert_eq!
((-a).div_ceil(b), -
2
);
assert_eq!
((-a).div_ceil(-b),
3
);
Source
pub const fn
next_multiple_of
(self, rhs:
i128
) ->
i128
🔬
This is a nightly-only experimental API. (
int_roundings
#88581
)
If
rhs
is positive, calculates the smallest value greater than or
equal to
self
that is a multiple of
rhs
. If
rhs
is negative,
calculates the largest value less than or equal to
self
that is a
multiple of
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
#![feature(int_roundings)]
assert_eq!
(
16_i128
.next_multiple_of(
8
),
16
);
assert_eq!
(
23_i128
.next_multiple_of(
8
),
24
);
assert_eq!
(
16_i128
.next_multiple_of(-
8
),
16
);
assert_eq!
(
23_i128
.next_multiple_of(-
8
),
16
);
assert_eq!
((-
16_i128
).next_multiple_of(
8
), -
16
);
assert_eq!
((-
23_i128
).next_multiple_of(
8
), -
16
);
assert_eq!
((-
16_i128
).next_multiple_of(-
8
), -
16
);
assert_eq!
((-
23_i128
).next_multiple_of(-
8
), -
24
);
Source
pub const fn
checked_next_multiple_of
(self, rhs:
i128
) ->
Option
<
i128
>
🔬
This is a nightly-only experimental API. (
int_roundings
#88581
)
If
rhs
is positive, calculates the smallest value greater than or
equal to
self
that is a multiple of
rhs
. If
rhs
is negative,
calculates the largest value less than or equal to
self
that is a
multiple of
rhs
. Returns
None
if
rhs
is zero or the operation
would result in overflow.
§
Examples
Basic usage:
#![feature(int_roundings)]
assert_eq!
(
16_i128
.checked_next_multiple_of(
8
),
Some
(
16
));
assert_eq!
(
23_i128
.checked_next_multiple_of(
8
),
Some
(
24
));
assert_eq!
(
16_i128
.checked_next_multiple_of(-
8
),
Some
(
16
));
assert_eq!
(
23_i128
.checked_next_multiple_of(-
8
),
Some
(
16
));
assert_eq!
((-
16_i128
).checked_next_multiple_of(
8
),
Some
(-
16
));
assert_eq!
((-
23_i128
).checked_next_multiple_of(
8
),
Some
(-
16
));
assert_eq!
((-
16_i128
).checked_next_multiple_of(-
8
),
Some
(-
16
));
assert_eq!
((-
23_i128
).checked_next_multiple_of(-
8
),
Some
(-
24
));
assert_eq!
(
1_i128
.checked_next_multiple_of(
0
),
None
);
assert_eq!
(i128::MAX.checked_next_multiple_of(
2
),
None
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
ilog
(self, base:
i128
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
is less than or equal to zero,
or if
base
is less than 2.
§
Examples
assert_eq!
(
5i128
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
is less than or equal to zero.
§
Examples
assert_eq!
(
2i128
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
is less than or equal to zero.
§
Example
assert_eq!
(
10i128
.ilog10(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog
(self, base:
i128
) ->
Option
<
u32
>
Returns the logarithm of the number with respect to an arbitrary base,
rounded down.
Returns
None
if the number is negative or zero, or if the base is not at least 2.
This method might not be optimized owing to implementation details;
checked_ilog2
can produce results more efficiently for base 2, and
checked_ilog10
can produce results more efficiently for base 10.
§
Examples
assert_eq!
(
5i128
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
if the number is negative or zero.
§
Examples
assert_eq!
(
2i128
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
if the number is negative or zero.
§
Example
assert_eq!
(
10i128
.checked_ilog10(),
Some
(
1
));
1.0.0 (const: 1.32.0)
·
Source
pub const fn
abs
(self) ->
i128
Computes the absolute value of
self
.
§
Overflow behavior
The absolute value of
i128::MIN
cannot be represented as an
i128
,
and attempting to calculate it will cause an overflow. This means
that code in debug mode will trigger a panic on this case and
optimized code will return
i128::MIN
without a panic. If you do not want this behavior, consider
using
unsigned_abs
instead.
§
Examples
Basic usage:
assert_eq!
(
10i128
.abs(),
10
);
assert_eq!
((-
10i128
).abs(),
10
);
1.60.0 (const: 1.60.0)
·
Source
pub const fn
abs_diff
(self, other:
i128
) ->
u128
Computes the absolute difference between
self
and
other
.
This function always returns the correct answer without overflow or
panics by returning an unsigned integer.
§
Examples
Basic usage:
assert_eq!
(
100i128
.abs_diff(
80
),
20u128
);
assert_eq!
(
100i128
.abs_diff(
110
),
10u128
);
assert_eq!
((-
100i128
).abs_diff(
80
),
180u128
);
assert_eq!
((-
100i128
).abs_diff(-
120
),
20u128
);
assert_eq!
(i128::MIN.abs_diff(i128::MAX), u128::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
signum
(self) ->
i128
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
assert_eq!
(
10i128
.signum(),
1
);
assert_eq!
(
0i128
.signum(),
0
);
assert_eq!
((-
10i128
).signum(), -
1
);
1.0.0 (const: 1.32.0)
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
assert!
(
10i128
.is_positive());
assert!
(!(-
10i128
).is_positive());
1.0.0 (const: 1.32.0)
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
assert!
((-
10i128
).is_negative());
assert!
(!
10i128
.is_negative());
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_be_bytes
(self) -> [
u8
;
16
]
Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.
§
Examples
let
bytes =
0x12345678901234567890123456789012i128
.to_be_bytes();
assert_eq!
(bytes, [
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
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
16
]
Returns the memory representation of this integer as a byte array in
little-endian byte order.
§
Examples
let
bytes =
0x12345678901234567890123456789012i128
.to_le_bytes();
assert_eq!
(bytes, [
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
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
16
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
§
Examples
let
bytes =
0x12345678901234567890123456789012i128
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
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
]
    }
else
{
        [
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
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
16
]) ->
i128
Creates an integer value from its representation as a byte array in
big endian.
§
Examples
let
value = i128::from_be_bytes([
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
]);
assert_eq!
(value,
0x12345678901234567890123456789012
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_be_i128(input:
&mut &
[u8]) -> i128 {
let
(int_bytes, rest) = input.split_at(size_of::<i128>());
*
input = rest;
    i128::from_be_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_le_bytes
(bytes: [
u8
;
16
]) ->
i128
Creates an integer value from its representation as a byte array in
little endian.
§
Examples
let
value = i128::from_le_bytes([
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
]);
assert_eq!
(value,
0x12345678901234567890123456789012
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_le_i128(input:
&mut &
[u8]) -> i128 {
let
(int_bytes, rest) = input.split_at(size_of::<i128>());
*
input = rest;
    i128::from_le_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_ne_bytes
(bytes: [
u8
;
16
]) ->
i128
Creates an integer value from its memory representation as a byte
array in native endianness.
As the target platform’s native endianness is used, portable code
likely wants to use
from_be_bytes
or
from_le_bytes
, as
appropriate instead.
§
Examples
let
value = i128::from_ne_bytes(
if
cfg!
(target_endian =
"big"
) {
    [
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
,
0x34
,
0x56
,
0x78
,
0x90
,
0x12
]
}
else
{
    [
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
,
0x90
,
0x78
,
0x56
,
0x34
,
0x12
]
});
assert_eq!
(value,
0x12345678901234567890123456789012
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_ne_i128(input:
&mut &
[u8]) -> i128 {
let
(int_bytes, rest) = input.split_at(size_of::<i128>());
*
input = rest;
    i128::from_ne_bytes(int_bytes.try_into().unwrap())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
min_value
() ->
i128
👎
Deprecating in a future version: replaced by the
MIN
associated constant on this type
New code should prefer to use
i128::MIN
instead.
Returns the smallest value that can be represented by this integer type.
1.0.0 (const: 1.32.0)
·
Source
pub const fn
max_value
() ->
i128
👎
Deprecating in a future version: replaced by the
MAX
associated constant on this type
New code should prefer to use
i128::MAX
instead.
Returns the largest value that can be represented by this integer type.
1.87.0 (const: 1.87.0)
·
Source
pub const fn
midpoint
(self, rhs:
i128
) ->
i128
Calculates the middle point of
self
and
rhs
.
midpoint(a, b)
is
(a + b) / 2
as if it were performed in a
sufficiently-large signed integral type. This implies that the result is
always rounded towards zero and that no overflow will ever occur.
§
Examples
assert_eq!
(
0i128
.midpoint(
4
),
2
);
assert_eq!
((-
1i128
).midpoint(
2
),
0
);
assert_eq!
((-
7i128
).midpoint(
0
), -
3
);
assert_eq!
(
0i128
.midpoint(-
7
), -
3
);
assert_eq!
(
0i128
.midpoint(
7
),
3
);
Source
§
impl
i128
1.0.0 (const: 1.82.0)
·
Source
pub const fn
from_str_radix
(
    src: &
str
,
    radix:
u32
,
) ->
Result
<
i128
,
ParseIntError
>
Parses an integer from a string slice with digits in a given base.
The string is expected to be an optional
+
or
-
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
(i128::from_str_radix(
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
(i128::from_str_radix(
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
i128
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
or
-
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
§
Examples
Basic usage:
#![feature(int_from_ascii)]
assert_eq!
(i128::from_ascii(
b"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(i128::from_ascii(
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
i128
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
or
-
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
(i128::from_ascii_radix(
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
(i128::from_ascii_radix(
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
AddAssign
for
i128
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
1.0.0
·
Source
§
impl
Binary
for
i128
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
BitAndAssign
for
i128
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
1.0.0
·
Source
§
impl
BitOr
<&
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
BitOrAssign
for
i128
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
1.0.0
·
Source
§
impl
BitXor
<&
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
BitXorAssign
for
i128
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
Source
§
impl
CarryingMulAdd
for
i128
Source
§
type
Unsigned
=
u128
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Source
§
const fn
carrying_mul_add
(self, b:
i128
, c:
i128
, d:
i128
) -> (
u128
,
i128
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
i128
Source
§
fn
clone
(&self) ->
i128
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
i128
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
i128
Source
§
fn
default
() ->
i128
Returns the default value of
0
Source
§
impl
DisjointBitOr
for
i128
Source
§
const unsafe fn
disjoint_bitor
(self, other:
i128
) ->
i128
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
i128
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
impl<'a>
Div
<
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
This operation rounds towards zero, truncating any
fractional part of the exact result.
§
Panics
This operation will panic if
other == 0
or the division results in overflow.
Source
§
type
Output
=
i128
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
DivAssign
for
i128
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
1.28.0
·
Source
§
impl
From
<
bool
> for
i128
Source
§
fn
from
(small:
bool
) ->
i128
Converts a
bool
to
i128
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
(i128::from(
true
),
1
);
assert_eq!
(i128::from(
false
),
0
);
Source
§
impl
From
<
i128
> for
AtomicI128
Source
§
fn
from
(v:
i128
) ->
AtomicI128
Converts an
i128
into an
AtomicI128
.
1.26.0
·
Source
§
impl
From
<
i16
> for
i128
Source
§
fn
from
(small:
i16
) ->
i128
Converts
i16
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
i32
> for
i128
Source
§
fn
from
(small:
i32
) ->
i128
Converts
i32
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
i64
> for
i128
Source
§
fn
from
(small:
i64
) ->
i128
Converts
i64
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
i8
> for
i128
Source
§
fn
from
(small:
i8
) ->
i128
Converts
i8
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
u16
> for
i128
Source
§
fn
from
(small:
u16
) ->
i128
Converts
u16
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
u32
> for
i128
Source
§
fn
from
(small:
u32
) ->
i128
Converts
u32
to
i128
losslessly.
1.26.0
·
Source
§
impl
From
<
u64
> for
i128
Source
§
fn
from
(small:
u64
) ->
i128
Converts
u64
to
i128
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
1.0.0
·
Source
§
impl
FromStr
for
i128
Source
§
fn
from_str
(src: &
str
) ->
Result
<
i128
,
ParseIntError
>
Parses an integer from a string slice with decimal digits.
The characters are expected to be an optional
+
or
-
sign followed by only digits. Leading and trailing non-digit characters (including
whitespace) represent an error. Underscores (which are accepted in Rust literals)
also represent an error.
§
Examples
Basic usage:
use
std::str::FromStr;
assert_eq!
(i128::from_str(
"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(i128::from_str(
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
i128
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
i128
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
i128
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
i128
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
MulAssign
for
i128
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
1.0.0
·
Source
§
impl
Neg
for &
i128
Source
§
type
Output
= <
i128
as
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
i128
as
Neg
>::
Output
Performs the unary
-
operation.
Read more
1.0.0
·
Source
§
impl
Neg
for
i128
Source
§
type
Output
=
i128
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
i128
Performs the unary
-
operation.
Read more
1.0.0
·
Source
§
impl
Not
for &
i128
Source
§
type
Output
= <
i128
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
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
i128
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
i128
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
i128
Source
§
fn
cmp
(&self, other: &
i128
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
i128
Source
§
fn
eq
(&self, other: &
i128
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
i128
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
i128
Source
§
fn
partial_cmp
(&self, other: &
i128
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
i128
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
i128
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
i128
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
i128
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
i128
> for
i128
Source
§
fn
product
<I>(iter: I) ->
i128
where
    I:
Iterator
<Item = &'a
i128
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
i128
Source
§
fn
product
<I>(iter: I) ->
i128
where
    I:
Iterator
<Item =
i128
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
i128
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
i128
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
i128
Source
§
const
MIN
:
i128
= -170_141_183_460_469_231_731_687_303_715_884_105_728i128
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
i128
= 170_141_183_460_469_231_731_687_303_715_884_105_727i128
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
i128
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
impl<'a>
Rem
<
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
This operation satisfies
n % d == n - (n / d) * d
. The
result has the same sign as the left operand.
§
Panics
This operation will panic if
other == 0
or if
self / other
results in overflow.
Source
§
type
Output
=
i128
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
RemAssign
for
i128
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
1.0.0
·
Source
§
impl
Shl
<&
i128
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
i128
) -> <
i128
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
i128
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
i16
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
i32
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
i64
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
i8
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
isize
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
u128
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
u16
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
u32
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
u64
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
usize
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
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
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
i16
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
i32
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
i64
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
i8
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
isize
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
u128
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
u16
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
u32
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
u64
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
i128
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
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
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
i128
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
i128
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
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
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
i16
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
impl<'a>
Shl
<
i128
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
i32
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
impl<'a>
Shl
<
i128
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
i64
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
impl<'a>
Shl
<
i128
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
i8
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
impl<'a>
Shl
<
i128
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
isize
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
impl<'a>
Shl
<
i128
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
u128
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
impl<'a>
Shl
<
i128
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
u16
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
impl<'a>
Shl
<
i128
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
u32
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
impl<'a>
Shl
<
i128
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
u64
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
impl<'a>
Shl
<
i128
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
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
impl
Shl
<
i128
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
i128
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
i16
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
i128
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
i16
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
impl<'a>
Shl
<
i32
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
i128
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
i32
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
impl<'a>
Shl
<
i64
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
i128
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
i64
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
impl<'a>
Shl
<
i8
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
i128
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
i8
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
impl<'a>
Shl
<
isize
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
i128
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
isize
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
impl<'a>
Shl
<
u128
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
i128
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
u128
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
impl<'a>
Shl
<
u16
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
i128
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
u16
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
impl<'a>
Shl
<
u32
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
i128
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
u32
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
impl<'a>
Shl
<
u64
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
i128
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
u64
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
impl<'a>
Shl
<
usize
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
i128
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
usize
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
for
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
i128
) ->
i128
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
i128
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
i128
> for
i16
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
i128
> for
i32
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
i128
> for
i64
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
i128
> for
i8
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
i128
> for
isize
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
i128
> for
u128
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
i128
> for
u16
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
i128
> for
u32
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
i128
> for
u64
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
i128
> for
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
> for
i128
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
i16
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
i128
> for
i32
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
i128
> for
i64
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
i128
> for
i8
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
i128
> for
isize
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
i128
> for
u128
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
i128
> for
u16
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
i128
> for
u32
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
i128
> for
u64
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
i128
> for
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
> for
i128
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
i128
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
1.0.0
·
Source
§
impl
Shr
<&
i128
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
i128
) -> <
i128
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
i128
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
i16
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
i32
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
i64
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
i8
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
isize
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
u128
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
u16
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
u32
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
u64
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
usize
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
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
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
i16
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
i32
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
i64
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
i8
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
isize
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
u128
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
u16
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
u32
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
u64
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
i128
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
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
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
i128
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
i128
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
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
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
i16
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
impl<'a>
Shr
<
i128
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
i32
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
impl<'a>
Shr
<
i128
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
i64
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
impl<'a>
Shr
<
i128
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
i8
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
impl<'a>
Shr
<
i128
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
isize
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
impl<'a>
Shr
<
i128
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
u128
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
impl<'a>
Shr
<
i128
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
u16
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
impl<'a>
Shr
<
i128
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
u32
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
impl<'a>
Shr
<
i128
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
u64
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
impl<'a>
Shr
<
i128
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
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
impl
Shr
<
i128
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
i128
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
i16
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
i128
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
i16
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
impl<'a>
Shr
<
i32
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
i128
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
i32
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
impl<'a>
Shr
<
i64
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
i128
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
i64
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
impl<'a>
Shr
<
i8
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
i128
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
i8
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
impl<'a>
Shr
<
isize
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
i128
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
isize
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
impl<'a>
Shr
<
u128
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
i128
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
u128
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
impl<'a>
Shr
<
u16
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
i128
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
u16
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
impl<'a>
Shr
<
u32
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
i128
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
u32
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
impl<'a>
Shr
<
u64
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
i128
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
u64
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
impl<'a>
Shr
<
usize
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
i128
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
usize
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
for
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
i128
) ->
i128
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
i128
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
i128
> for
i16
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
i128
> for
i32
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
i128
> for
i64
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
i128
> for
i8
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
i128
> for
isize
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
i128
> for
u128
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
i128
> for
u16
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
i128
> for
u32
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
i128
> for
u64
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
i128
> for
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
> for
i128
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
i16
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
i128
> for
i32
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
i128
> for
i64
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
i128
> for
i8
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
i128
> for
isize
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
i128
> for
u128
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
i128
> for
u16
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
i128
> for
u32
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
i128
> for
u64
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
i128
> for
usize
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
i128
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
usize
> for
i128
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
i128
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
Source
§
impl
Step
for
i128
Source
§
fn
forward
(start:
i128
, n:
usize
) ->
i128
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
i128
, n:
usize
) ->
i128
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
i128
, n:
usize
) ->
i128
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
i128
, n:
usize
) ->
i128
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
i128
, end: &
i128
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
i128
, n:
usize
) ->
Option
<
i128
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
i128
, n:
usize
) ->
Option
<
i128
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
i128
> for &
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
> for &'a
i128
Source
§
type
Output
= <
i128
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
i128
) -> <
i128
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
i128
Source
§
type
Output
=
i128
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
i128
) ->
i128
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
i128
> for
i128
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
1.8.0
·
Source
§
impl
SubAssign
for
i128
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
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
i128
> for
i128
Source
§
fn
sum
<I>(iter: I) ->
i128
where
    I:
Iterator
<Item = &'a
i128
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
i128
Source
§
fn
sum
<I>(iter: I) ->
i128
where
    I:
Iterator
<Item =
i128
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
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
1.34.0
·
Source
§
impl
TryFrom
<
i128
> for
i16
Source
§
fn
try_from
(u:
i128
) ->
Result
<
i16
, <
i16
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
i128
> for
i32
Source
§
fn
try_from
(u:
i128
) ->
Result
<
i32
, <
i32
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
i128
> for
i64
Source
§
fn
try_from
(u:
i128
) ->
Result
<
i64
, <
i64
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
i128
> for
i8
Source
§
fn
try_from
(u:
i128
) ->
Result
<
i8
, <
i8
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
i128
> for
isize
Source
§
fn
try_from
(u:
i128
) ->
Result
<
isize
, <
isize
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
i128
> for
u128
Source
§
fn
try_from
(u:
i128
) ->
Result
<
u128
, <
u128
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
i128
> for
u16
Source
§
fn
try_from
(u:
i128
) ->
Result
<
u16
, <
u16
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
i128
> for
u32
Source
§
fn
try_from
(u:
i128
) ->
Result
<
u32
, <
u32
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
i128
> for
u64
Source
§
fn
try_from
(u:
i128
) ->
Result
<
u64
, <
u64
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
i128
> for
usize
Source
§
fn
try_from
(u:
i128
) ->
Result
<
usize
, <
usize
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
isize
> for
i128
Source
§
fn
try_from
(value:
isize
) ->
Result
<
i128
, <
i128
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
i128
Source
§
fn
try_from
(u:
u128
) ->
Result
<
i128
, <
i128
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
usize
> for
i128
Source
§
fn
try_from
(value:
usize
) ->
Result
<
i128
, <
i128
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
i128
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
i128
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
i128
1.0.0
·
Source
§
impl
Copy
for
i128
1.0.0
·
Source
§
impl
Eq
for
i128
Source
§
impl
FloatToInt
<
i128
> for
f128
Source
§
impl
FloatToInt
<
i128
> for
f16
Source
§
impl
FloatToInt
<
i128
> for
f32
Source
§
impl
FloatToInt
<
i128
> for
f64
Source
§
impl
StructuralPartialEq
for
i128
Source
§
impl
TrustedStep
for
i128
Source
§
impl
UnsizedConstParamTy
for
i128
Source
§
impl
UseCloned
for
i128
Source
§
impl
ZeroablePrimitive
for
i128
Auto Trait Implementations
§
§
impl
Freeze
for
i128
§
impl
RefUnwindSafe
for
i128
§
impl
Send
for
i128
§
impl
Sync
for
i128
§
impl
Unpin
for
i128
§
impl
UnwindSafe
for
i128
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