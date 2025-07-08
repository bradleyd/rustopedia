isize - Rust
Primitive Type
isize
Copy item path
1.0.0
Expand description
The pointer-sized signed integer type.
The size of this primitive is how many bytes it takes to reference any
location in memory. For example, on a 32 bit target, this is 4 bytes
and on a 64 bit target, this is 8 bytes.
Implementations
§
Source
§
impl
isize
1.43.0
·
Source
pub const
MIN
:
isize
= -9_223_372_036_854_775_808isize
The smallest value that can be represented by this integer type
(−2
63
on 64-bit targets).
§
Examples
Basic usage:
assert_eq!
(isize::MIN, -
9223372036854775808
);
1.43.0
·
Source
pub const
MAX
:
isize
= 9_223_372_036_854_775_807isize
The largest value that can be represented by this integer type
(2
63
− 1 on 64-bit targets).
§
Examples
Basic usage:
assert_eq!
(isize::MAX,
9223372036854775807
);
1.53.0
·
Source
pub const
BITS
:
u32
= 64u32
The size of this integer type in bits.
§
Examples
assert_eq!
(isize::BITS,
64
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
0b100_0000isize
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
(isize::MAX.count_zeros(),
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
1isize
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
4isize
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
1isize
;
assert_eq!
(n.leading_ones(),
64
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
3isize
;
assert_eq!
(n.trailing_ones(),
2
);
Source
pub const fn
isolate_most_significant_one
(self) ->
isize
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
n: isize =
0b_01100100
;
assert_eq!
(n.isolate_most_significant_one(),
0b_01000000
);
assert_eq!
(
0_isize
.isolate_most_significant_one(),
0
);
Source
pub const fn
isolate_least_significant_one
(self) ->
isize
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
n: isize =
0b_01100100
;
assert_eq!
(n.isolate_least_significant_one(),
0b_00000100
);
assert_eq!
(
0_isize
.isolate_least_significant_one(),
0
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_unsigned
(self) ->
usize
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
1isize
;
assert_eq!
(n.cast_unsigned(), usize::MAX);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
isize
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
0xaa00000000006e1isize
;
let
m =
0x6e10aa
;
assert_eq!
(n.rotate_left(
12
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_right
(self, n:
u32
) ->
isize
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
0x6e10aaisize
;
let
m =
0xaa00000000006e1
;
assert_eq!
(n.rotate_right(
12
), m);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
swap_bytes
(self) ->
isize
Reverses the byte order of the integer.
§
Examples
Basic usage:
let
n =
0x1234567890123456isize
;
let
m = n.swap_bytes();
assert_eq!
(m,
0x5634129078563412
);
1.37.0 (const: 1.37.0)
·
Source
pub const fn
reverse_bits
(self) ->
isize
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
let
n =
0x1234567890123456isize
;
let
m = n.reverse_bits();
assert_eq!
(m,
0x6a2c48091e6a2c48
);
assert_eq!
(
0
,
0isize
.reverse_bits());
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_be
(x:
isize
) ->
isize
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Aisize
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(isize::from_be(n), n)
}
else
{
assert_eq!
(isize::from_be(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_le
(x:
isize
) ->
isize
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Aisize
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(isize::from_le(n), n)
}
else
{
assert_eq!
(isize::from_le(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_be
(self) ->
isize
Converts
self
to big endian from the target’s endianness.
On big endian this is a no-op. On little endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Aisize
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
isize
Converts
self
to little endian from the target’s endianness.
On little endian this is a no-op. On big endian the bytes are swapped.
§
Examples
Basic usage:
let
n =
0x1Aisize
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
isize
) ->
Option
<
isize
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
((isize::MAX -
2
).checked_add(
1
),
Some
(isize::MAX -
1
));
assert_eq!
((isize::MAX -
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
isize
) ->
isize
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
((isize::MAX -
2
).strict_add(
1
), isize::MAX -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (isize::MAX -
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
isize
) ->
isize
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
self + rhs > isize::MAX
or
self + rhs < isize::MIN
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
usize
) ->
Option
<
isize
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
1isize
.checked_add_unsigned(
2
),
Some
(
3
));
assert_eq!
((isize::MAX -
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
usize
) ->
isize
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
1isize
.strict_add_unsigned(
2
),
3
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (isize::MAX -
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
isize
) ->
Option
<
isize
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
((isize::MIN +
2
).checked_sub(
1
),
Some
(isize::MIN +
1
));
assert_eq!
((isize::MIN +
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
isize
) ->
isize
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
((isize::MIN +
2
).strict_sub(
1
), isize::MIN +
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (isize::MIN +
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
isize
) ->
isize
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
self - rhs > isize::MAX
or
self - rhs < isize::MIN
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
usize
) ->
Option
<
isize
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
1isize
.checked_sub_unsigned(
2
),
Some
(-
1
));
assert_eq!
((isize::MIN +
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
usize
) ->
isize
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
1isize
.strict_sub_unsigned(
2
), -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (isize::MIN +
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
isize
) ->
Option
<
isize
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
(isize::MAX.checked_mul(
1
),
Some
(isize::MAX));
assert_eq!
(isize::MAX.checked_mul(
2
),
None
);
Source
pub const fn
strict_mul
(self, rhs:
isize
) ->
isize
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
(isize::MAX.strict_mul(
1
), isize::MAX);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MAX.strict_mul(
2
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_mul
(self, rhs:
isize
) ->
isize
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
self * rhs > isize::MAX
or
self * rhs < isize::MIN
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
isize
) ->
Option
<
isize
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
((isize::MIN +
1
).checked_div(-
1
),
Some
(
9223372036854775807
));
assert_eq!
(isize::MIN.checked_div(-
1
),
None
);
assert_eq!
((
1isize
).checked_div(
0
),
None
);
Source
pub const fn
strict_div
(self, rhs:
isize
) ->
isize
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
((isize::MIN +
1
).strict_div(-
1
),
9223372036854775807
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_div(-
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1isize
).strict_div(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_div_euclid
(self, rhs:
isize
) ->
Option
<
isize
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
((isize::MIN +
1
).checked_div_euclid(-
1
),
Some
(
9223372036854775807
));
assert_eq!
(isize::MIN.checked_div_euclid(-
1
),
None
);
assert_eq!
((
1isize
).checked_div_euclid(
0
),
None
);
Source
pub const fn
strict_div_euclid
(self, rhs:
isize
) ->
isize
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
((isize::MIN +
1
).strict_div_euclid(-
1
),
9223372036854775807
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_div_euclid(-
1
);
The following panics because of division by zero:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (
1isize
).strict_div_euclid(
0
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem
(self, rhs:
isize
) ->
Option
<
isize
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
5isize
.checked_rem(
2
),
Some
(
1
));
assert_eq!
(
5isize
.checked_rem(
0
),
None
);
assert_eq!
(isize::MIN.checked_rem(-
1
),
None
);
Source
pub const fn
strict_rem
(self, rhs:
isize
) ->
isize
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
5isize
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
5isize
.strict_rem(
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_rem(-
1
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem_euclid
(self, rhs:
isize
) ->
Option
<
isize
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
5isize
.checked_rem_euclid(
2
),
Some
(
1
));
assert_eq!
(
5isize
.checked_rem_euclid(
0
),
None
);
assert_eq!
(isize::MIN.checked_rem_euclid(-
1
),
None
);
Source
pub const fn
strict_rem_euclid
(self, rhs:
isize
) ->
isize
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
5isize
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
5isize
.strict_rem_euclid(
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_rem_euclid(-
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
isize
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
5isize
.checked_neg(),
Some
(-
5
));
assert_eq!
(isize::MIN.checked_neg(),
None
);
Source
pub const unsafe fn
unchecked_neg
(self) ->
isize
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
self == isize::MIN
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
isize
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
5isize
.strict_neg(), -
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_neg();
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
isize
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
0x1isize
.checked_shl(
4
),
Some
(
0x10
));
assert_eq!
(
0x1isize
.checked_shl(
129
),
None
);
assert_eq!
(
0x10isize
.checked_shl(
63
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
isize
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
0x1isize
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
0x1isize
.strict_shl(
129
);
Source
pub const unsafe fn
unchecked_shl
(self, rhs:
u32
) ->
isize
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
isize
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
0x1isize
.unbounded_shl(
4
),
0x10
);
assert_eq!
(
0x1isize
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
isize
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
0x10isize
.checked_shr(
4
),
Some
(
0x1
));
assert_eq!
(
0x10isize
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
isize
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
0x10isize
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
0x10isize
.strict_shr(
128
);
Source
pub const unsafe fn
unchecked_shr
(self, rhs:
u32
) ->
isize
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
isize
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
0x10isize
.unbounded_shr(
4
),
0x1
);
assert_eq!
(
0x10isize
.unbounded_shr(
129
),
0
);
assert_eq!
(isize::MIN.unbounded_shr(
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
isize
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
5isize
).checked_abs(),
Some
(
5
));
assert_eq!
(isize::MIN.checked_abs(),
None
);
Source
pub const fn
strict_abs
(self) ->
isize
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
5isize
).strict_abs(),
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MIN.strict_abs();
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
isize
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
8isize
.checked_pow(
2
),
Some
(
64
));
assert_eq!
(isize::MAX.checked_pow(
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
isize
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
8isize
.strict_pow(
2
),
64
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= isize::MAX.strict_pow(
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
isize
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
10isize
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
isize
) ->
isize
Saturating integer addition. Computes
self + rhs
, saturating at the numeric
bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100isize
.saturating_add(
1
),
101
);
assert_eq!
(isize::MAX.saturating_add(
100
), isize::MAX);
assert_eq!
(isize::MIN.saturating_add(-
1
), isize::MIN);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_add_unsigned
(self, rhs:
usize
) ->
isize
Saturating addition with an unsigned integer. Computes
self + rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
1isize
.saturating_add_unsigned(
2
),
3
);
assert_eq!
(isize::MAX.saturating_add_unsigned(
100
), isize::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
isize
) ->
isize
Saturating integer subtraction. Computes
self - rhs
, saturating at the
numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100isize
.saturating_sub(
127
), -
27
);
assert_eq!
(isize::MIN.saturating_sub(
100
), isize::MIN);
assert_eq!
(isize::MAX.saturating_sub(-
1
), isize::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_sub_unsigned
(self, rhs:
usize
) ->
isize
Saturating subtraction with an unsigned integer. Computes
self - rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100isize
.saturating_sub_unsigned(
127
), -
27
);
assert_eq!
(isize::MIN.saturating_sub_unsigned(
100
), isize::MIN);
1.45.0 (const: 1.47.0)
·
Source
pub const fn
saturating_neg
(self) ->
isize
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
100isize
.saturating_neg(), -
100
);
assert_eq!
((-
100isize
).saturating_neg(),
100
);
assert_eq!
(isize::MIN.saturating_neg(), isize::MAX);
assert_eq!
(isize::MAX.saturating_neg(), isize::MIN +
1
);
1.45.0 (const: 1.47.0)
·
Source
pub const fn
saturating_abs
(self) ->
isize
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
100isize
.saturating_abs(),
100
);
assert_eq!
((-
100isize
).saturating_abs(),
100
);
assert_eq!
(isize::MIN.saturating_abs(), isize::MAX);
assert_eq!
((isize::MIN +
1
).saturating_abs(), isize::MAX);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
isize
) ->
isize
Saturating integer multiplication. Computes
self * rhs
, saturating at the
numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
10isize
.saturating_mul(
12
),
120
);
assert_eq!
(isize::MAX.saturating_mul(
10
), isize::MAX);
assert_eq!
(isize::MIN.saturating_mul(
10
), isize::MIN);
1.58.0 (const: 1.58.0)
·
Source
pub const fn
saturating_div
(self, rhs:
isize
) ->
isize
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
5isize
.saturating_div(
2
),
2
);
assert_eq!
(isize::MAX.saturating_div(-
1
), isize::MIN +
1
);
assert_eq!
(isize::MIN.saturating_div(-
1
), isize::MAX);
1.34.0 (const: 1.50.0)
·
Source
pub const fn
saturating_pow
(self, exp:
u32
) ->
isize
Saturating integer exponentiation. Computes
self.pow(exp)
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
((-
4isize
).saturating_pow(
3
), -
64
);
assert_eq!
(isize::MIN.saturating_pow(
2
), isize::MAX);
assert_eq!
(isize::MIN.saturating_pow(
3
), isize::MIN);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_add
(self, rhs:
isize
) ->
isize
Wrapping (modular) addition. Computes
self + rhs
, wrapping around at the
boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100isize
.wrapping_add(
27
),
127
);
assert_eq!
(isize::MAX.wrapping_add(
2
), isize::MIN +
1
);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_add_unsigned
(self, rhs:
usize
) ->
isize
Wrapping (modular) addition with an unsigned integer. Computes
self + rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100isize
.wrapping_add_unsigned(
27
),
127
);
assert_eq!
(isize::MAX.wrapping_add_unsigned(
2
), isize::MIN +
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_sub
(self, rhs:
isize
) ->
isize
Wrapping (modular) subtraction. Computes
self - rhs
, wrapping around at the
boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
0isize
.wrapping_sub(
127
), -
127
);
assert_eq!
((-
2isize
).wrapping_sub(isize::MAX), isize::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_sub_unsigned
(self, rhs:
usize
) ->
isize
Wrapping (modular) subtraction with an unsigned integer. Computes
self - rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
0isize
.wrapping_sub_unsigned(
127
), -
127
);
assert_eq!
((-
2isize
).wrapping_sub_unsigned(usize::MAX), -
1
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_mul
(self, rhs:
isize
) ->
isize
Wrapping (modular) multiplication. Computes
self * rhs
, wrapping around at
the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
10isize
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
isize
) ->
isize
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
100isize
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
isize
) ->
isize
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
100isize
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
isize
) ->
isize
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
100isize
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
isize
) ->
isize
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
100isize
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
isize
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
100isize
.wrapping_neg(), -
100
);
assert_eq!
((-
100isize
).wrapping_neg(),
100
);
assert_eq!
(isize::MIN.wrapping_neg(), isize::MIN);
1.2.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_shl
(self, rhs:
u32
) ->
isize
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
1isize
).wrapping_shl(
7
), -
128
);
assert_eq!
((-
1isize
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
isize
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
128isize
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
isize
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
100isize
.wrapping_abs(),
100
);
assert_eq!
((-
100isize
).wrapping_abs(),
100
);
assert_eq!
(isize::MIN.wrapping_abs(), isize::MIN);
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
usize
Computes the absolute value of
self
without any wrapping
or panicking.
§
Examples
Basic usage:
assert_eq!
(
100isize
.unsigned_abs(),
100usize
);
assert_eq!
((-
100isize
).unsigned_abs(),
100usize
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
isize
Wrapping (modular) exponentiation. Computes
self.pow(exp)
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
3isize
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
isize
) -> (
isize
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
5isize
.overflowing_add(
2
), (
7
,
false
));
assert_eq!
(isize::MAX.overflowing_add(
1
), (isize::MIN,
true
));
Source
pub const fn
carrying_add
(self, rhs:
isize
, carry:
bool
) -> (
isize
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
usize::carrying_add
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
//   10  MAX    (a = 10 × 2^64 + 2^64 - 1)
// + -5    9    (b = -5 × 2^64 + 9)
// ---------
//    6    8    (sum = 6 × 2^64 + 8)
let
(a1, a0): (isize, usize) = (
10
, usize::MAX);
let
(b1, b0): (isize, usize) = (-
5
,
9
);
let
carry0 =
false
;
// usize::carrying_add for the less significant words
let
(sum0, carry1) = a0.carrying_add(b0, carry0);
assert_eq!
(carry1,
true
);
// isize::carrying_add for the most significant word
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
usize
) -> (
isize
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
1isize
.overflowing_add_unsigned(
2
), (
3
,
false
));
assert_eq!
((isize::MIN).overflowing_add_unsigned(usize::MAX), (isize::MAX,
false
));
assert_eq!
((isize::MAX -
2
).overflowing_add_unsigned(
3
), (isize::MIN,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_sub
(self, rhs:
isize
) -> (
isize
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
5isize
.overflowing_sub(
2
), (
3
,
false
));
assert_eq!
(isize::MIN.overflowing_sub(
1
), (isize::MAX,
true
));
Source
pub const fn
borrowing_sub
(self, rhs:
isize
, borrow:
bool
) -> (
isize
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
usize::borrowing_sub
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
//    6    8    (a = 6 × 2^64 + 8)
// - -5    9    (b = -5 × 2^64 + 9)
// ---------
//   10  MAX    (diff = 10 × 2^64 + 2^64 - 1)
let
(a1, a0): (isize, usize) = (
6
,
8
);
let
(b1, b0): (isize, usize) = (-
5
,
9
);
let
borrow0 =
false
;
// usize::borrowing_sub for the less significant words
let
(diff0, borrow1) = a0.borrowing_sub(b0, borrow0);
assert_eq!
(borrow1,
true
);
// isize::borrowing_sub for the most significant word
let
(diff1, overflow) = a1.borrowing_sub(b1, borrow1);
assert_eq!
(overflow,
false
);
assert_eq!
((diff1, diff0), (
10
, usize::MAX));
1.66.0 (const: 1.66.0)
·
Source
pub const fn
overflowing_sub_unsigned
(self, rhs:
usize
) -> (
isize
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
1isize
.overflowing_sub_unsigned(
2
), (-
1
,
false
));
assert_eq!
((isize::MAX).overflowing_sub_unsigned(usize::MAX), (isize::MIN,
false
));
assert_eq!
((isize::MIN +
2
).overflowing_sub_unsigned(
3
), (isize::MAX,
true
));
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_mul
(self, rhs:
isize
) -> (
isize
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
5isize
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
isize
) -> (
usize
,
isize
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
isize
, carry:
isize
) -> (
usize
,
isize
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
(isize::MAX.carrying_mul(isize::MAX, isize::MAX), (isize::MAX.unsigned_abs() +
1
, isize::MAX /
2
));
Source
pub const fn
carrying_mul_add
(
    self,
    rhs:
isize
,
    carry:
isize
,
    add:
isize
,
) -> (
usize
,
isize
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
(isize::MAX.carrying_mul_add(isize::MAX, isize::MAX, isize::MAX), (usize::MAX, isize::MAX /
2
));
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div
(self, rhs:
isize
) -> (
isize
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
5isize
.overflowing_div(
2
), (
2
,
false
));
assert_eq!
(isize::MIN.overflowing_div(-
1
), (isize::MIN,
true
));
1.38.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_div_euclid
(self, rhs:
isize
) -> (
isize
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
5isize
.overflowing_div_euclid(
2
), (
2
,
false
));
assert_eq!
(isize::MIN.overflowing_div_euclid(-
1
), (isize::MIN,
true
));
1.7.0 (const: 1.52.0)
·
Source
pub const fn
overflowing_rem
(self, rhs:
isize
) -> (
isize
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
5isize
.overflowing_rem(
2
), (
1
,
false
));
assert_eq!
(isize::MIN.overflowing_rem(-
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
isize
) -> (
isize
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
5isize
.overflowing_rem_euclid(
2
), (
1
,
false
));
assert_eq!
(isize::MIN.overflowing_rem_euclid(-
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
isize
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
2isize
.overflowing_neg(), (-
2
,
false
));
assert_eq!
(isize::MIN.overflowing_neg(), (isize::MIN,
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
isize
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
0x1isize
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
0x10isize
.overflowing_shl(
63
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
isize
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
0x10isize
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
isize
,
bool
)
Computes the absolute value of
self
.
Returns a tuple of the absolute version of self along with a boolean indicating whether an overflow
happened. If self is the minimum value
(e.g., isize::MIN for values of type isize),
then the minimum value will be returned again and true will be returned
for an overflow happening.
§
Examples
Basic usage:
assert_eq!
(
10isize
.overflowing_abs(), (
10
,
false
));
assert_eq!
((-
10isize
).overflowing_abs(), (
10
,
false
));
assert_eq!
((isize::MIN).overflowing_abs(), (isize::MIN,
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
isize
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
3isize
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
isize
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
let
x: isize =
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
isize
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
10isize
.isqrt(),
3
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
div_euclid
(self, rhs:
isize
) ->
isize
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
a: isize =
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
isize
) ->
isize
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
a: isize =
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
= isize::MIN.rem_euclid(-
1
);
Source
pub const fn
div_floor
(self, rhs:
isize
) ->
isize
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
a: isize =
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
isize
) ->
isize
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
a: isize =
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
isize
) ->
isize
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
16_isize
.next_multiple_of(
8
),
16
);
assert_eq!
(
23_isize
.next_multiple_of(
8
),
24
);
assert_eq!
(
16_isize
.next_multiple_of(-
8
),
16
);
assert_eq!
(
23_isize
.next_multiple_of(-
8
),
16
);
assert_eq!
((-
16_isize
).next_multiple_of(
8
), -
16
);
assert_eq!
((-
23_isize
).next_multiple_of(
8
), -
16
);
assert_eq!
((-
16_isize
).next_multiple_of(-
8
), -
16
);
assert_eq!
((-
23_isize
).next_multiple_of(-
8
), -
24
);
Source
pub const fn
checked_next_multiple_of
(self, rhs:
isize
) ->
Option
<
isize
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
16_isize
.checked_next_multiple_of(
8
),
Some
(
16
));
assert_eq!
(
23_isize
.checked_next_multiple_of(
8
),
Some
(
24
));
assert_eq!
(
16_isize
.checked_next_multiple_of(-
8
),
Some
(
16
));
assert_eq!
(
23_isize
.checked_next_multiple_of(-
8
),
Some
(
16
));
assert_eq!
((-
16_isize
).checked_next_multiple_of(
8
),
Some
(-
16
));
assert_eq!
((-
23_isize
).checked_next_multiple_of(
8
),
Some
(-
16
));
assert_eq!
((-
16_isize
).checked_next_multiple_of(-
8
),
Some
(-
16
));
assert_eq!
((-
23_isize
).checked_next_multiple_of(-
8
),
Some
(-
24
));
assert_eq!
(
1_isize
.checked_next_multiple_of(
0
),
None
);
assert_eq!
(isize::MAX.checked_next_multiple_of(
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
isize
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
5isize
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
2isize
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
10isize
.ilog10(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog
(self, base:
isize
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
5isize
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
2isize
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
10isize
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
isize
Computes the absolute value of
self
.
§
Overflow behavior
The absolute value of
isize::MIN
cannot be represented as an
isize
,
and attempting to calculate it will cause an overflow. This means
that code in debug mode will trigger a panic on this case and
optimized code will return
isize::MIN
without a panic. If you do not want this behavior, consider
using
unsigned_abs
instead.
§
Examples
Basic usage:
assert_eq!
(
10isize
.abs(),
10
);
assert_eq!
((-
10isize
).abs(),
10
);
1.60.0 (const: 1.60.0)
·
Source
pub const fn
abs_diff
(self, other:
isize
) ->
usize
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
100isize
.abs_diff(
80
),
20usize
);
assert_eq!
(
100isize
.abs_diff(
110
),
10usize
);
assert_eq!
((-
100isize
).abs_diff(
80
),
180usize
);
assert_eq!
((-
100isize
).abs_diff(-
120
),
20usize
);
assert_eq!
(isize::MIN.abs_diff(isize::MAX), usize::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
signum
(self) ->
isize
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
10isize
.signum(),
1
);
assert_eq!
(
0isize
.signum(),
0
);
assert_eq!
((-
10isize
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
10isize
.is_positive());
assert!
(!(-
10isize
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
10isize
).is_negative());
assert!
(!
10isize
.is_negative());
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_be_bytes
(self) -> [
u8
;
8
]
Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.
Note
: This function returns an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
bytes =
0x1234567890123456isize
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
]);
1.32.0 (const: 1.44.0)
·
Source
pub const fn
to_le_bytes
(self) -> [
u8
;
8
]
Returns the memory representation of this integer as a byte array in
little-endian byte order.
Note
: This function returns an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
bytes =
0x1234567890123456isize
.to_le_bytes();
assert_eq!
(bytes, [
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
8
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
: This function returns an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
bytes =
0x1234567890123456isize
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
]
    }
else
{
        [
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
8
]) ->
isize
Creates an integer value from its representation as a byte array in
big endian.
Note
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = isize::from_be_bytes([
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
]);
assert_eq!
(value,
0x1234567890123456
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_be_isize(input:
&mut &
[u8]) -> isize {
let
(int_bytes, rest) = input.split_at(size_of::<isize>());
*
input = rest;
    isize::from_be_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_le_bytes
(bytes: [
u8
;
8
]) ->
isize
Creates an integer value from its representation as a byte array in
little endian.
Note
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = isize::from_le_bytes([
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
0x1234567890123456
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_le_isize(input:
&mut &
[u8]) -> isize {
let
(int_bytes, rest) = input.split_at(size_of::<isize>());
*
input = rest;
    isize::from_le_bytes(int_bytes.try_into().unwrap())
}
1.32.0 (const: 1.44.0)
·
Source
pub const fn
from_ne_bytes
(bytes: [
u8
;
8
]) ->
isize
Creates an integer value from its memory representation as a byte
array in native endianness.
As the target platform’s native endianness is used, portable code
likely wants to use
from_be_bytes
or
from_le_bytes
, as
appropriate instead.
Note
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = isize::from_ne_bytes(
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
]
}
else
{
    [
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
0x1234567890123456
);
When starting from a slice rather than an array, fallible conversion APIs can be used:
fn
read_ne_isize(input:
&mut &
[u8]) -> isize {
let
(int_bytes, rest) = input.split_at(size_of::<isize>());
*
input = rest;
    isize::from_ne_bytes(int_bytes.try_into().unwrap())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
min_value
() ->
isize
👎
Deprecating in a future version: replaced by the
MIN
associated constant on this type
New code should prefer to use
isize::MIN
instead.
Returns the smallest value that can be represented by this integer type.
1.0.0 (const: 1.32.0)
·
Source
pub const fn
max_value
() ->
isize
👎
Deprecating in a future version: replaced by the
MAX
associated constant on this type
New code should prefer to use
isize::MAX
instead.
Returns the largest value that can be represented by this integer type.
1.87.0 (const: 1.87.0)
·
Source
pub const fn
midpoint
(self, rhs:
isize
) ->
isize
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
0isize
.midpoint(
4
),
2
);
assert_eq!
((-
1isize
).midpoint(
2
),
0
);
assert_eq!
((-
7isize
).midpoint(
0
), -
3
);
assert_eq!
(
0isize
.midpoint(-
7
), -
3
);
assert_eq!
(
0isize
.midpoint(
7
),
3
);
Source
§
impl
isize
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
isize
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
(isize::from_str_radix(
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
(isize::from_str_radix(
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
isize
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
(isize::from_ascii(
b"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(isize::from_ascii(
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
isize
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
(isize::from_ascii_radix(
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
(isize::from_ascii_radix(
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
AddAssign
for
isize
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
1.0.0
·
Source
§
impl
Binary
for
isize
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
BitAndAssign
for
isize
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
1.0.0
·
Source
§
impl
BitOr
<&
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
BitOrAssign
for
isize
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
1.0.0
·
Source
§
impl
BitXor
<&
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
BitXorAssign
for
isize
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
Source
§
impl
CarryingMulAdd
for
isize
Source
§
type
Unsigned
=
usize
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Source
§
const fn
carrying_mul_add
(self, a:
isize
, b:
isize
, c:
isize
) -> (
usize
,
isize
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
isize
Source
§
fn
clone
(&self) ->
isize
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
isize
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
isize
Source
§
fn
default
() ->
isize
Returns the default value of
0
Source
§
impl
DisjointBitOr
for
isize
Source
§
const unsafe fn
disjoint_bitor
(self, other:
isize
) ->
isize
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
isize
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
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
isize
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
DivAssign
for
isize
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
1.28.0
·
Source
§
impl
From
<
bool
> for
isize
Source
§
fn
from
(small:
bool
) ->
isize
Converts a
bool
to
isize
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
(isize::from(
true
),
1
);
assert_eq!
(isize::from(
false
),
0
);
1.26.0
·
Source
§
impl
From
<
i16
> for
isize
Source
§
fn
from
(small:
i16
) ->
isize
Converts
i16
to
isize
losslessly.
1.5.0
·
Source
§
impl
From
<
i8
> for
isize
Source
§
fn
from
(small:
i8
) ->
isize
Converts
i8
to
isize
losslessly.
1.23.0
·
Source
§
impl
From
<
isize
> for
AtomicIsize
Source
§
fn
from
(v:
isize
) ->
AtomicIsize
Converts an
isize
into an
AtomicIsize
.
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
1.0.0
·
Source
§
impl
FromStr
for
isize
Source
§
fn
from_str
(src: &
str
) ->
Result
<
isize
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
(isize::from_str(
"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(isize::from_str(
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
isize
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
isize
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
isize
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
isize
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
MulAssign
for
isize
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
1.0.0
·
Source
§
impl
Neg
for &
isize
Source
§
type
Output
= <
isize
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
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) ->
isize
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
isize
Source
§
type
Output
= <
isize
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
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
isize
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
isize
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
isize
Source
§
fn
cmp
(&self, other: &
isize
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
isize
Source
§
fn
eq
(&self, other: &
isize
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
isize
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
isize
Source
§
fn
partial_cmp
(&self, other: &
isize
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
isize
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
isize
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
isize
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
isize
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
isize
> for
isize
Source
§
fn
product
<I>(iter: I) ->
isize
where
    I:
Iterator
<Item = &'a
isize
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
isize
Source
§
fn
product
<I>(iter: I) ->
isize
where
    I:
Iterator
<Item =
isize
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
isize
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
isize
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
isize
Source
§
const
MIN
:
isize
= -9_223_372_036_854_775_808isize
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
isize
= 9_223_372_036_854_775_807isize
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
isize
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
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
isize
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
RemAssign
for
isize
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
i16
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
isize
> for &'lhs
Simd
<
isize
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
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shl
<&
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
i16
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
i32
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
i64
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
i8
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
isize
) -> <
isize
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
isize
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
u128
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
u16
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
u32
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
u64
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
usize
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
Source
§
impl<const N:
usize
>
Shl
<&
isize
> for
Simd
<
isize
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
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
isize
) -> <
Simd
<
isize
, N> as
Shl
<&
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
isize
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
i16
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
i32
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
i64
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
i8
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
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
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
u128
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
u16
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
u32
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
u64
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
isize
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
usize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
usize
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
isize
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
isize
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
impl<'a>
Shl
<
i16
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
isize
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
i16
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
impl<'a>
Shl
<
i32
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
isize
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
i32
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
impl<'a>
Shl
<
i64
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
isize
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
i64
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
impl<'a>
Shl
<
i8
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
isize
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
i8
) ->
isize
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
isize
> for &'lhs
Simd
<
isize
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
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
isize
) -> <&'lhs
Simd
<
isize
, N> as
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
impl<'a>
Shl
<
isize
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
i16
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
impl<'a>
Shl
<
isize
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
i32
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
impl<'a>
Shl
<
isize
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
i64
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
impl<'a>
Shl
<
isize
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
i8
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
impl<'a>
Shl
<
isize
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
isize
) -> <
isize
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
isize
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
u128
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
impl<'a>
Shl
<
isize
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
u16
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
impl<'a>
Shl
<
isize
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
u32
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
impl<'a>
Shl
<
isize
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
u64
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
impl<'a>
Shl
<
isize
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
usize
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
Source
§
impl<const N:
usize
>
Shl
<
isize
> for
Simd
<
isize
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
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
isize
) -> <
Simd
<
isize
, N> as
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
impl
Shl
<
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
impl
Shl
<
isize
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
isize
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
u128
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
isize
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
u128
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
impl<'a>
Shl
<
u16
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
isize
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
u16
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
impl<'a>
Shl
<
u32
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
isize
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
u32
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
impl<'a>
Shl
<
u64
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
isize
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
u64
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
impl<'a>
Shl
<
usize
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
isize
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
usize
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
for
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
isize
) ->
isize
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
i16
> for
isize
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
isize
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
isize
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
isize
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
isize
> for
i16
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
isize
> for
i32
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
isize
> for
i64
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
isize
> for
i8
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
isize
> for
isize
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
isize
> for
u128
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
isize
> for
u16
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
isize
> for
u32
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
isize
> for
u64
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
isize
> for
usize
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
isize
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
isize
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
isize
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
isize
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
usize
> for
isize
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
i16
> for
isize
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
isize
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
isize
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
isize
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
isize
> for
i16
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
isize
> for
i32
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
isize
> for
i64
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
isize
> for
i8
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
isize
> for
u128
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
isize
> for
u16
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
isize
> for
u32
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
isize
> for
u64
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
isize
> for
usize
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
isize
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
isize
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
isize
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
isize
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
usize
> for
isize
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
isize
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
i16
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
isize
> for &'lhs
Simd
<
isize
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
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shr
<&
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
i16
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
i32
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
i64
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
i8
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
isize
) -> <
isize
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
isize
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
u128
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
u16
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
u32
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
u64
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
usize
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
Source
§
impl<const N:
usize
>
Shr
<&
isize
> for
Simd
<
isize
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
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
isize
) -> <
Simd
<
isize
, N> as
Shr
<&
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
isize
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
i16
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
i32
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
i64
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
i8
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
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
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
u128
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
u16
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
u32
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
u64
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
isize
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
usize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
usize
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
isize
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
isize
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
impl<'a>
Shr
<
i16
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
isize
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
i16
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
impl<'a>
Shr
<
i32
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
isize
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
i32
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
impl<'a>
Shr
<
i64
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
isize
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
i64
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
impl<'a>
Shr
<
i8
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
isize
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
i8
) ->
isize
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
isize
> for &'lhs
Simd
<
isize
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
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
isize
) -> <&'lhs
Simd
<
isize
, N> as
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
impl<'a>
Shr
<
isize
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
i16
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
impl<'a>
Shr
<
isize
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
i32
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
impl<'a>
Shr
<
isize
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
i64
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
impl<'a>
Shr
<
isize
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
i8
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
impl<'a>
Shr
<
isize
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
isize
) -> <
isize
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
isize
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
u128
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
impl<'a>
Shr
<
isize
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
u16
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
impl<'a>
Shr
<
isize
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
u32
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
impl<'a>
Shr
<
isize
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
u64
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
impl<'a>
Shr
<
isize
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
usize
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
Source
§
impl<const N:
usize
>
Shr
<
isize
> for
Simd
<
isize
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
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
isize
) -> <
Simd
<
isize
, N> as
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
impl
Shr
<
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
isize
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
impl
Shr
<
isize
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
isize
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
u128
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
isize
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
u128
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
impl<'a>
Shr
<
u16
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
isize
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
u16
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
impl<'a>
Shr
<
u32
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
isize
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
u32
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
impl<'a>
Shr
<
u64
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
isize
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
u64
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
impl<'a>
Shr
<
usize
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
isize
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
usize
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
for
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
isize
) ->
isize
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
i16
> for
isize
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
isize
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
isize
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
isize
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
isize
> for
i16
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
isize
> for
i32
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
isize
> for
i64
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
isize
> for
i8
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
isize
> for
isize
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
isize
> for
u128
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
isize
> for
u16
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
isize
> for
u32
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
isize
> for
u64
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
isize
> for
usize
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
isize
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
isize
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
isize
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
isize
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
usize
> for
isize
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
i16
> for
isize
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
isize
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
isize
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
isize
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
isize
> for
i16
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
isize
> for
i32
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
isize
> for
i64
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
isize
> for
i8
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
isize
> for
u128
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
isize
> for
u16
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
isize
> for
u32
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
isize
> for
u64
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
isize
> for
usize
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
isize
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
isize
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
isize
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
isize
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
usize
> for
isize
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
isize
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
Source
§
impl
SimdElement
for
isize
Source
§
type
Mask
=
isize
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
isize
Source
§
fn
forward
(start:
isize
, n:
usize
) ->
isize
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
isize
, n:
usize
) ->
isize
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
isize
, n:
usize
) ->
isize
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
isize
, n:
usize
) ->
isize
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
isize
, end: &
isize
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
isize
, n:
usize
) ->
Option
<
isize
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
isize
, n:
usize
) ->
Option
<
isize
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
isize
> for &
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
> for &'a
isize
Source
§
type
Output
= <
isize
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
isize
) -> <
isize
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
isize
Source
§
type
Output
=
isize
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
isize
) ->
isize
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
isize
> for
isize
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
1.8.0
·
Source
§
impl
SubAssign
for
isize
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
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
isize
> for
isize
Source
§
fn
sum
<I>(iter: I) ->
isize
where
    I:
Iterator
<Item = &'a
isize
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
isize
Source
§
fn
sum
<I>(iter: I) ->
isize
where
    I:
Iterator
<Item =
isize
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
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
i32
> for
isize
Source
§
fn
try_from
(value:
i32
) ->
Result
<
isize
, <
isize
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
isize
Source
§
fn
try_from
(value:
i64
) ->
Result
<
isize
, <
isize
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
isize
> for
i16
Source
§
fn
try_from
(u:
isize
) ->
Result
<
i16
, <
i16
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
isize
> for
i32
Source
§
fn
try_from
(u:
isize
) ->
Result
<
i32
, <
i32
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
isize
> for
i64
Source
§
fn
try_from
(value:
isize
) ->
Result
<
i64
, <
i64
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
isize
> for
i8
Source
§
fn
try_from
(u:
isize
) ->
Result
<
i8
, <
i8
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
isize
> for
u128
Source
§
fn
try_from
(u:
isize
) ->
Result
<
u128
, <
u128
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
isize
> for
u16
Source
§
fn
try_from
(u:
isize
) ->
Result
<
u16
, <
u16
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
isize
> for
u32
Source
§
fn
try_from
(u:
isize
) ->
Result
<
u32
, <
u32
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
isize
> for
u64
Source
§
fn
try_from
(u:
isize
) ->
Result
<
u64
, <
u64
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
isize
> for
usize
Source
§
fn
try_from
(u:
isize
) ->
Result
<
usize
, <
usize
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
isize
Source
§
fn
try_from
(u:
u128
) ->
Result
<
isize
, <
isize
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
isize
Source
§
fn
try_from
(value:
u16
) ->
Result
<
isize
, <
isize
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
isize
Source
§
fn
try_from
(value:
u32
) ->
Result
<
isize
, <
isize
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
isize
Source
§
fn
try_from
(u:
u64
) ->
Result
<
isize
, <
isize
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
isize
Source
§
fn
try_from
(u:
usize
) ->
Result
<
isize
, <
isize
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
isize
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
isize
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
isize
1.0.0
·
Source
§
impl
Copy
for
isize
1.0.0
·
Source
§
impl
Eq
for
isize
Source
§
impl
FloatToInt
<
isize
> for
f128
Source
§
impl
FloatToInt
<
isize
> for
f16
Source
§
impl
FloatToInt
<
isize
> for
f32
Source
§
impl
FloatToInt
<
isize
> for
f64
Source
§
impl
MaskElement
for
isize
Source
§
impl
PointerLike
for
isize
Source
§
impl
SimdCast
for
isize
Source
§
impl
StructuralPartialEq
for
isize
Source
§
impl
TrustedStep
for
isize
Source
§
impl
UnsizedConstParamTy
for
isize
Source
§
impl
UseCloned
for
isize
Source
§
impl
ZeroablePrimitive
for
isize
Auto Trait Implementations
§
§
impl
Freeze
for
isize
§
impl
RefUnwindSafe
for
isize
§
impl
Send
for
isize
§
impl
Sync
for
isize
§
impl
Unpin
for
isize
§
impl
UnwindSafe
for
isize
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