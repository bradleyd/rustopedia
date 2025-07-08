u64 - Rust
Primitive Type
u64
Copy item path
1.0.0
Expand description
The 64-bit unsigned integer type.
Implementations
§
Source
§
impl
u64
1.43.0
·
Source
pub const
MIN
:
u64
= 0u64
The smallest value that can be represented by this integer type.
§
Examples
Basic usage:
assert_eq!
(u64::MIN,
0
);
1.43.0
·
Source
pub const
MAX
:
u64
= 18_446_744_073_709_551_615u64
The largest value that can be represented by this integer type
(2
64
− 1).
§
Examples
Basic usage:
assert_eq!
(u64::MAX,
18446744073709551615
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
(u64::BITS,
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
0b01001100u64
;
assert_eq!
(n.count_ones(),
3
);
let
max = u64::MAX;
assert_eq!
(max.count_ones(),
64
);
let
zero =
0u64
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
0u64
;
assert_eq!
(zero.count_zeros(),
64
);
let
max = u64::MAX;
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
n = u64::MAX >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
let
zero =
0u64
;
assert_eq!
(zero.leading_zeros(),
64
);
let
max = u64::MAX;
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
0b0101000u64
;
assert_eq!
(n.trailing_zeros(),
3
);
let
zero =
0u64
;
assert_eq!
(zero.trailing_zeros(),
64
);
let
max = u64::MAX;
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
n = !(u64::MAX >>
2
);
assert_eq!
(n.leading_ones(),
2
);
let
zero =
0u64
;
assert_eq!
(zero.leading_ones(),
0
);
let
max = u64::MAX;
assert_eq!
(max.leading_ones(),
64
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
0b1010111u64
;
assert_eq!
(n.trailing_ones(),
3
);
let
zero =
0u64
;
assert_eq!
(zero.trailing_ones(),
0
);
let
max = u64::MAX;
assert_eq!
(max.trailing_ones(),
64
);
Source
pub const fn
isolate_most_significant_one
(self) ->
u64
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
n: u64 =
0b_01100100
;
assert_eq!
(n.isolate_most_significant_one(),
0b_01000000
);
assert_eq!
(
0_u64
.isolate_most_significant_one(),
0
);
Source
pub const fn
isolate_least_significant_one
(self) ->
u64
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
n: u64 =
0b_01100100
;
assert_eq!
(n.isolate_least_significant_one(),
0b_00000100
);
assert_eq!
(
0_u64
.isolate_least_significant_one(),
0
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
i64
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
n = u64::MAX;
assert_eq!
(n.cast_signed(), -
1i64
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
u64
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
0xaa00000000006e1u64
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
u64
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
0x6e10aau64
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
u64
Reverses the byte order of the integer.
§
Examples
Basic usage:
let
n =
0x1234567890123456u64
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
u64
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
let
n =
0x1234567890123456u64
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
0u64
.reverse_bits());
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_be
(x:
u64
) ->
u64
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au64
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(u64::from_be(n), n)
}
else
{
assert_eq!
(u64::from_be(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_le
(x:
u64
) ->
u64
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Au64
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(u64::from_le(n), n)
}
else
{
assert_eq!
(u64::from_le(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_be
(self) ->
u64
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
0x1Au64
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
u64
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
0x1Au64
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
u64
) ->
Option
<
u64
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
((u64::MAX -
2
).checked_add(
1
),
Some
(u64::MAX -
1
));
assert_eq!
((u64::MAX -
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
u64
) ->
u64
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
((u64::MAX -
2
).strict_add(
1
), u64::MAX -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (u64::MAX -
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
u64
) ->
u64
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
self + rhs > u64::MAX
or
self + rhs < u64::MIN
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
i64
) ->
Option
<
u64
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
1u64
.checked_add_signed(
2
),
Some
(
3
));
assert_eq!
(
1u64
.checked_add_signed(-
2
),
None
);
assert_eq!
((u64::MAX -
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
i64
) ->
u64
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
1u64
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
1u64
.strict_add_signed(-
2
);
ⓘ
#![feature(strict_overflow_ops)]
let _
= (u64::MAX -
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
u64
) ->
Option
<
u64
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
1u64
.checked_sub(
1
),
Some
(
0
));
assert_eq!
(
0u64
.checked_sub(
1
),
None
);
Source
pub const fn
strict_sub
(self, rhs:
u64
) ->
u64
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
1u64
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
0u64
.strict_sub(
1
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_sub
(self, rhs:
u64
) ->
u64
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
self - rhs > u64::MAX
or
self - rhs < u64::MIN
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
i64
) ->
Option
<
u64
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
1u64
.checked_sub_signed(
2
),
None
);
assert_eq!
(
1u64
.checked_sub_signed(-
2
),
Some
(
3
));
assert_eq!
((u64::MAX -
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
u64
) ->
Option
<
i64
>
🔬
This is a nightly-only experimental API. (
unsigned_signed_diff
#126041
)
Checked integer subtraction. Computes
self - rhs
and checks if the result fits into an
i64
, returning
None
if overflow occurred.
§
Examples
Basic usage:
#![feature(unsigned_signed_diff)]
assert_eq!
(
10u64
.checked_signed_diff(
2
),
Some
(
8
));
assert_eq!
(
2u64
.checked_signed_diff(
10
),
Some
(-
8
));
assert_eq!
(u64::MAX.checked_signed_diff(i64::MAX
as
u64),
None
);
assert_eq!
((i64::MAX
as
u64).checked_signed_diff(u64::MAX),
Some
(i64::MIN));
assert_eq!
((i64::MAX
as
u64 +
1
).checked_signed_diff(
0
),
None
);
assert_eq!
(u64::MAX.checked_signed_diff(u64::MAX),
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
u64
) ->
Option
<
u64
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
5u64
.checked_mul(
1
),
Some
(
5
));
assert_eq!
(u64::MAX.checked_mul(
2
),
None
);
Source
pub const fn
strict_mul
(self, rhs:
u64
) ->
u64
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
5u64
.strict_mul(
1
),
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= u64::MAX.strict_mul(
2
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_mul
(self, rhs:
u64
) ->
u64
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
self * rhs > u64::MAX
or
self * rhs < u64::MIN
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
u64
) ->
Option
<
u64
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
128u64
.checked_div(
2
),
Some
(
64
));
assert_eq!
(
1u64
.checked_div(
0
),
None
);
Source
pub const fn
strict_div
(self, rhs:
u64
) ->
u64
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
100u64
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
1u64
).strict_div(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_div_euclid
(self, rhs:
u64
) ->
Option
<
u64
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
128u64
.checked_div_euclid(
2
),
Some
(
64
));
assert_eq!
(
1u64
.checked_div_euclid(
0
),
None
);
Source
pub const fn
strict_div_euclid
(self, rhs:
u64
) ->
u64
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
100u64
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
1u64
).strict_div_euclid(
0
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem
(self, rhs:
u64
) ->
Option
<
u64
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
5u64
.checked_rem(
2
),
Some
(
1
));
assert_eq!
(
5u64
.checked_rem(
0
),
None
);
Source
pub const fn
strict_rem
(self, rhs:
u64
) ->
u64
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
100u64
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
5u64
.strict_rem(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem_euclid
(self, rhs:
u64
) ->
Option
<
u64
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
5u64
.checked_rem_euclid(
2
),
Some
(
1
));
assert_eq!
(
5u64
.checked_rem_euclid(
0
),
None
);
Source
pub const fn
strict_rem_euclid
(self, rhs:
u64
) ->
u64
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
100u64
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
5u64
.strict_rem_euclid(
0
);
Source
pub const unsafe fn
unchecked_disjoint_bitor
(self, other:
u64
) ->
u64
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
1_u64
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
u64
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
5u64
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
2u64
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
10u64
.ilog10(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog
(self, base:
u64
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
5u64
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
2u64
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
10u64
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
u64
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
0u64
.checked_neg(),
Some
(
0
));
assert_eq!
(
1u64
.checked_neg(),
None
);
Source
pub const fn
strict_neg
(self) ->
u64
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
0u64
.strict_neg(),
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
1u64
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
u64
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
0x1u64
.checked_shl(
4
),
Some
(
0x10
));
assert_eq!
(
0x10u64
.checked_shl(
129
),
None
);
assert_eq!
(
0x10u64
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
u64
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
0x1u64
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
0x10u64
.strict_shl(
129
);
Source
pub const unsafe fn
unchecked_shl
(self, rhs:
u32
) ->
u64
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
u64
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
0x1u64
.unbounded_shl(
4
),
0x10
);
assert_eq!
(
0x1u64
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
u64
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
0x10u64
.checked_shr(
4
),
Some
(
0x1
));
assert_eq!
(
0x10u64
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
u64
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
0x10u64
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
0x10u64
.strict_shr(
129
);
Source
pub const unsafe fn
unchecked_shr
(self, rhs:
u32
) ->
u64
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
u64
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
0x10u64
.unbounded_shr(
4
),
0x1
);
assert_eq!
(
0x10u64
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
u64
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
2u64
.checked_pow(
5
),
Some
(
32
));
assert_eq!
(u64::MAX.checked_pow(
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
u64
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
2u64
.strict_pow(
5
),
32
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= u64::MAX.strict_pow(
2
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_add
(self, rhs:
u64
) ->
u64
Saturating integer addition. Computes
self + rhs
, saturating at
the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100u64
.saturating_add(
1
),
101
);
assert_eq!
(u64::MAX.saturating_add(
127
), u64::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_add_signed
(self, rhs:
i64
) ->
u64
Saturating addition with a signed integer. Computes
self + rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
1u64
.saturating_add_signed(
2
),
3
);
assert_eq!
(
1u64
.saturating_add_signed(-
2
),
0
);
assert_eq!
((u64::MAX -
2
).saturating_add_signed(
4
), u64::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
u64
) ->
u64
Saturating integer subtraction. Computes
self - rhs
, saturating
at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100u64
.saturating_sub(
27
),
73
);
assert_eq!
(
13u64
.saturating_sub(
127
),
0
);
Source
pub const fn
saturating_sub_signed
(self, rhs:
i64
) ->
u64
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
1u64
.saturating_sub_signed(
2
),
0
);
assert_eq!
(
1u64
.saturating_sub_signed(-
2
),
3
);
assert_eq!
((u64::MAX -
2
).saturating_sub_signed(-
4
), u64::MAX);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
u64
) ->
u64
Saturating integer multiplication. Computes
self * rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
2u64
.saturating_mul(
10
),
20
);
assert_eq!
((u64::MAX).saturating_mul(
10
), u64::MAX);
1.58.0 (const: 1.58.0)
·
Source
pub const fn
saturating_div
(self, rhs:
u64
) ->
u64
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
5u64
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
u64
Saturating integer exponentiation. Computes
self.pow(exp)
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
4u64
.saturating_pow(
3
),
64
);
assert_eq!
(u64::MAX.saturating_pow(
2
), u64::MAX);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_add
(self, rhs:
u64
) ->
u64
Wrapping (modular) addition. Computes
self + rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
200u64
.wrapping_add(
55
),
255
);
assert_eq!
(
200u64
.wrapping_add(u64::MAX),
199
);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_add_signed
(self, rhs:
i64
) ->
u64
Wrapping (modular) addition with a signed integer. Computes
self + rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
1u64
.wrapping_add_signed(
2
),
3
);
assert_eq!
(
1u64
.wrapping_add_signed(-
2
), u64::MAX);
assert_eq!
((u64::MAX -
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
u64
) ->
u64
Wrapping (modular) subtraction. Computes
self - rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100u64
.wrapping_sub(
100
),
0
);
assert_eq!
(
100u64
.wrapping_sub(u64::MAX),
101
);
Source
pub const fn
wrapping_sub_signed
(self, rhs:
i64
) ->
u64
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
1u64
.wrapping_sub_signed(
2
), u64::MAX);
assert_eq!
(
1u64
.wrapping_sub_signed(-
2
),
3
);
assert_eq!
((u64::MAX -
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
u64
) ->
u64
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
u64
) ->
u64
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
100u64
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
u64
) ->
u64
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
100u64
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
u64
) ->
u64
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
100u64
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
u64
) ->
u64
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
100u64
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
u64
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
0_u64
.wrapping_neg(),
0
);
assert_eq!
(u64::MAX.wrapping_neg(),
1
);
assert_eq!
(
13_u64
.wrapping_neg(), (!
13
) +
1
);
assert_eq!
(
42_u64
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
u64
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
1u64
.wrapping_shl(
7
),
128
);
assert_eq!
(
1u64
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
u64
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
128u64
.wrapping_shr(
7
),
1
);
assert_eq!
(
128u64
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
u64
Wrapping (modular) exponentiation. Computes
self.pow(exp)
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
3u64
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
u64
) -> (
u64
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
5u64
.overflowing_add(
2
), (
7
,
false
));
assert_eq!
(u64::MAX.overflowing_add(
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
u64
, carry:
bool
) -> (
u64
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
This can be thought of as a 64-bit “full adder”, in the electronics sense.
If the input carry is false, this method is equivalent to
overflowing_add
, and the output carry is
equal to the overflow flag. Note that although carry and overflow
flags are similar for unsigned integers, they are different for
signed integers.
§
Examples
#![feature(bigint_helper_methods)]
//    3  MAX    (a = 3 × 2^64 + 2^64 - 1)
// +  5    7    (b = 5 × 2^64 + 7)
// ---------
//    9    6    (sum = 9 × 2^64 + 6)
let
(a1, a0): (u64, u64) = (
3
, u64::MAX);
let
(b1, b0): (u64, u64) = (
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
i64
) -> (
u64
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
1u64
.overflowing_add_signed(
2
), (
3
,
false
));
assert_eq!
(
1u64
.overflowing_add_signed(-
2
), (u64::MAX,
true
));
assert_eq!
((u64::MAX -
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
u64
) -> (
u64
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
5u64
.overflowing_sub(
2
), (
3
,
false
));
assert_eq!
(
0u64
.overflowing_sub(
1
), (u64::MAX,
true
));
Source
pub const fn
borrowing_sub
(self, rhs:
u64
, borrow:
bool
) -> (
u64
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
//    9    6    (a = 9 × 2^64 + 6)
// -  5    7    (b = 5 × 2^64 + 7)
// ---------
//    3  MAX    (diff = 3 × 2^64 + 2^64 - 1)
let
(a1, a0): (u64, u64) = (
9
,
6
);
let
(b1, b0): (u64, u64) = (
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
, u64::MAX));
Source
pub const fn
overflowing_sub_signed
(self, rhs:
i64
) -> (
u64
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
1u64
.overflowing_sub_signed(
2
), (u64::MAX,
true
));
assert_eq!
(
1u64
.overflowing_sub_signed(-
2
), (
3
,
false
));
assert_eq!
((u64::MAX -
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
u64
) ->
u64
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
100u64
.abs_diff(
80
),
20u64
);
assert_eq!
(
100u64
.abs_diff(
110
),
10u64
);
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_mul
(self, rhs:
u64
) -> (
u64
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
u64
) -> (
u64
,
u64
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
u64
, carry:
u64
) -> (
u64
,
u64
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
(u64::MAX.carrying_mul(u64::MAX, u64::MAX), (
0
, u64::MAX));
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
(
    self,
    rhs:
u64
,
    carry:
u64
,
    add:
u64
,
) -> (
u64
,
u64
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
(u64::MAX.carrying_mul_add(u64::MAX, u64::MAX, u64::MAX), (u64::MAX, u64::MAX));
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
u64
) -> (
u64
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
5u64
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
u64
) -> (
u64
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
5u64
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
u64
) -> (
u64
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
5u64
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
u64
) -> (
u64
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
5u64
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
u64
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
0u64
.overflowing_neg(), (
0
,
false
));
assert_eq!
(
2u64
.overflowing_neg(), (-
2i32
as
u64,
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
u64
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
0x1u64
.overflowing_shl(
4
), (
0x10
,
false
));
assert_eq!
(
0x1u64
.overflowing_shl(
132
), (
0x10
,
true
));
assert_eq!
(
0x10u64
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
u64
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
0x10u64
.overflowing_shr(
4
), (
0x1
,
false
));
assert_eq!
(
0x10u64
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
u64
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
3u64
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
u64
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
assert_eq!
(
2u64
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
u64
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
assert_eq!
(
10u64
.isqrt(),
3
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
div_euclid
(self, rhs:
u64
) ->
u64
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
7u64
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
u64
) ->
u64
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
7u64
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
u64
) ->
u64
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
7_u64
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
u64
) ->
u64
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
7_u64
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
u64
) ->
u64
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
16_u64
.next_multiple_of(
8
),
16
);
assert_eq!
(
23_u64
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
u64
) ->
Option
<
u64
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
16_u64
.checked_next_multiple_of(
8
),
Some
(
16
));
assert_eq!
(
23_u64
.checked_next_multiple_of(
8
),
Some
(
24
));
assert_eq!
(
1_u64
.checked_next_multiple_of(
0
),
None
);
assert_eq!
(u64::MAX.checked_next_multiple_of(
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
u64
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
6_u64
.is_multiple_of(
2
));
assert!
(!
5_u64
.is_multiple_of(
2
));
assert!
(
0_u64
.is_multiple_of(
0
));
assert!
(!
6_u64
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
16u64
.is_power_of_two());
assert!
(!
10u64
.is_power_of_two());
1.0.0 (const: 1.50.0)
·
Source
pub const fn
next_power_of_two
(self) ->
u64
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
2u64
.next_power_of_two(),
2
);
assert_eq!
(
3u64
.next_power_of_two(),
4
);
assert_eq!
(
0u64
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
u64
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
2u64
.checked_next_power_of_two(),
Some
(
2
));
assert_eq!
(
3u64
.checked_next_power_of_two(),
Some
(
4
));
assert_eq!
(u64::MAX.checked_next_power_of_two(),
None
);
Source
pub const fn
wrapping_next_power_of_two
(self) ->
u64
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
2u64
.wrapping_next_power_of_two(),
2
);
assert_eq!
(
3u64
.wrapping_next_power_of_two(),
4
);
assert_eq!
(u64::MAX.wrapping_next_power_of_two(),
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
8
]
Returns the memory representation of this integer as a byte array in
big-endian (network) byte order.
§
Examples
let
bytes =
0x1234567890123456u64
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
§
Examples
let
bytes =
0x1234567890123456u64
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
§
Examples
let
bytes =
0x1234567890123456u64
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
u64
Creates a native endian integer value from its representation
as a byte array in big endian.
§
Examples
let
value = u64::from_be_bytes([
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
read_be_u64(input:
&mut &
[u8]) -> u64 {
let
(int_bytes, rest) = input.split_at(size_of::<u64>());
*
input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
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
u64
Creates a native endian integer value from its representation
as a byte array in little endian.
§
Examples
let
value = u64::from_le_bytes([
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
read_le_u64(input:
&mut &
[u8]) -> u64 {
let
(int_bytes, rest) = input.split_at(size_of::<u64>());
*
input = rest;
    u64::from_le_bytes(int_bytes.try_into().unwrap())
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
u64
Creates a native endian integer value from its memory representation
as a byte array in native endianness.
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
value = u64::from_ne_bytes(
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
read_ne_u64(input:
&mut &
[u8]) -> u64 {
let
(int_bytes, rest) = input.split_at(size_of::<u64>());
*
input = rest;
    u64::from_ne_bytes(int_bytes.try_into().unwrap())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
min_value
() ->
u64
👎
Deprecating in a future version: replaced by the
MIN
associated constant on this type
New code should prefer to use
u64::MIN
instead.
Returns the smallest value that can be represented by this integer type.
1.0.0 (const: 1.32.0)
·
Source
pub const fn
max_value
() ->
u64
👎
Deprecating in a future version: replaced by the
MAX
associated constant on this type
New code should prefer to use
u64::MAX
instead.
Returns the largest value that can be represented by this integer type.
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
u64
) ->
u64
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
0u64
.midpoint(
4
),
2
);
assert_eq!
(
1u64
.midpoint(
4
),
2
);
Source
§
impl
u64
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
u64
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
(u64::from_str_radix(
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
(u64::from_str_radix(
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
u64
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
(u64::from_ascii(
b"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(u64::from_ascii(
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
u64
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
(u64::from_ascii_radix(
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
(u64::from_ascii_radix(
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
AddAssign
for
u64
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
1.0.0
·
Source
§
impl
Binary
for
u64
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
BitAndAssign
for
u64
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
1.0.0
·
Source
§
impl
BitOr
<&
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
BitOrAssign
for
u64
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
1.0.0
·
Source
§
impl
BitXor
<&
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
BitXorAssign
for
u64
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
Source
§
impl
CarryingMulAdd
for
u64
Source
§
type
Unsigned
=
u64
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Source
§
const fn
carrying_mul_add
(self, a:
u64
, b:
u64
, c:
u64
) -> (
u64
,
u64
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
u64
Source
§
fn
clone
(&self) ->
u64
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
u64
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
u64
Source
§
fn
default
() ->
u64
Returns the default value of
0
Source
§
impl
DisjointBitOr
for
u64
Source
§
const unsafe fn
disjoint_bitor
(self, other:
u64
) ->
u64
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
u64
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
1.0.0
·
Source
§
impl<'a>
Div
<
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
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
u64
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
DivAssign
for
u64
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
Source
§
impl
From
<
AsciiChar
> for
u64
Source
§
fn
from
(chr:
AsciiChar
) ->
u64
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
u64
Source
§
fn
from
(small:
bool
) ->
u64
Converts a
bool
to
u64
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
(u64::from(
true
),
1
);
assert_eq!
(u64::from(
false
),
0
);
1.51.0
·
Source
§
impl
From
<
char
> for
u64
Source
§
fn
from
(c:
char
) ->
u64
Converts a
char
into a
u64
.
§
Examples
let
c =
'👤'
;
let
u = u64::from(c);
assert!
(
8
== size_of_val(
&
u))
1.5.0
·
Source
§
impl
From
<
u16
> for
u64
Source
§
fn
from
(small:
u16
) ->
u64
Converts
u16
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
u32
> for
u64
Source
§
fn
from
(small:
u32
) ->
u64
Converts
u32
to
u64
losslessly.
1.34.0
·
Source
§
impl
From
<
u64
> for
AtomicU64
Source
§
fn
from
(v:
u64
) ->
AtomicU64
Converts an
u64
into an
AtomicU64
.
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
u64
> for
u128
Source
§
fn
from
(small:
u64
) ->
u128
Converts
u64
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
1.0.0
·
Source
§
impl
FromStr
for
u64
Source
§
fn
from_str
(src: &
str
) ->
Result
<
u64
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
(u64::from_str(
"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(u64::from_str(
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
u64
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
u64
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
u64
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
u64
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
MulAssign
for
u64
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
1.0.0
·
Source
§
impl
Not
for &
u64
Source
§
type
Output
= <
u64
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
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
u64
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
u64
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
u64
Source
§
fn
cmp
(&self, other: &
u64
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
u64
Source
§
fn
eq
(&self, other: &
u64
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
u64
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
u64
Source
§
fn
partial_cmp
(&self, other: &
u64
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
u64
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
u64
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
u64
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
u64
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
u64
> for
u64
Source
§
fn
product
<I>(iter: I) ->
u64
where
    I:
Iterator
<Item = &'a
u64
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
u64
Source
§
fn
product
<I>(iter: I) ->
u64
where
    I:
Iterator
<Item =
u64
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
u64
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
u64
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
u64
Source
§
const
MIN
:
u64
= 0u64
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
u64
= 18_446_744_073_709_551_615u64
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
u64
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
1.0.0
·
Source
§
impl<'a>
Rem
<
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
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
u64
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
RemAssign
for
u64
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
i16
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u128
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u64
> for &'lhs
Simd
<
u64
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
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shl
<&
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
i16
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
i32
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
i64
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
i8
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
u128
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
u16
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
u32
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
u64
) -> <
u64
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
usize
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
impl<const N:
usize
>
Shl
<&
u64
> for
Simd
<
u64
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
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u64
) -> <
Simd
<
u64
, N> as
Shl
<&
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
u64
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
i16
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
i32
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
i64
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
i8
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
u64
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
u128
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
u16
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
u32
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
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
1.0.0
·
Source
§
impl
Shl
<&
u64
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
usize
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
usize
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
u64
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
u64
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
impl<'a>
Shl
<
i16
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
u64
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
i16
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
impl<'a>
Shl
<
i32
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
u64
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
i32
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
impl<'a>
Shl
<
i64
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
u64
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
i64
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
impl<'a>
Shl
<
i8
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
u64
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
i8
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
impl<'a>
Shl
<
u128
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
u64
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
u128
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
impl<'a>
Shl
<
u16
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
u64
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
u16
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
impl<'a>
Shl
<
u32
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
u64
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
u32
) ->
u64
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
u64
> for &'lhs
Simd
<
u64
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
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u64
) -> <&'lhs
Simd
<
u64
, N> as
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
impl<'a>
Shl
<
u64
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
i16
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
impl<'a>
Shl
<
u64
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
i32
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
impl<'a>
Shl
<
u64
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
i64
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
impl<'a>
Shl
<
u64
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
i8
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
impl<'a>
Shl
<
u64
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
u128
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
impl<'a>
Shl
<
u64
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
u16
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
impl<'a>
Shl
<
u64
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
u32
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
impl<'a>
Shl
<
u64
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
u64
) -> <
u64
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
impl<'a>
Shl
<
u64
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
usize
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
impl<const N:
usize
>
Shl
<
u64
> for
Simd
<
u64
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
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u64
) -> <
Simd
<
u64
, N> as
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
impl
Shl
<
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
impl
Shl
<
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
1.0.0
·
Source
§
impl
Shl
<
u64
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
u64
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
impl<'a>
Shl
<
usize
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
u64
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
usize
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
for
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
u64
) ->
u64
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
i16
> for
u64
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
u64
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
u64
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
u64
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
u128
> for
u64
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
u64
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
u64
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
u64
> for
i16
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
u64
> for
i32
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
u64
> for
i64
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
u64
> for
i8
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
u64
> for
u128
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
u64
> for
u16
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
u64
> for
u32
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
u64
> for
u64
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
u64
> for
usize
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
usize
> for
u64
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
i16
> for
u64
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
u64
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
u64
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
u64
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
u128
> for
u64
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
u64
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
u64
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
u64
> for
i16
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
u64
> for
i32
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
u64
> for
i64
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
u64
> for
i8
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
u64
> for
u128
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
u64
> for
u16
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
u64
> for
u32
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
u64
> for
usize
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
usize
> for
u64
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
u64
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
i16
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u128
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u64
> for &'lhs
Simd
<
u64
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
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shr
<&
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
i16
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
i32
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
i64
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
i8
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
u128
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
u16
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
u32
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
u64
) -> <
u64
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
usize
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
impl<const N:
usize
>
Shr
<&
u64
> for
Simd
<
u64
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
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u64
) -> <
Simd
<
u64
, N> as
Shr
<&
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
u64
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
i16
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
i32
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
i64
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
i8
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
u64
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
u128
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
u16
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
u32
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
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
1.0.0
·
Source
§
impl
Shr
<&
u64
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
usize
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
usize
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
u64
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
u64
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
impl<'a>
Shr
<
i16
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
u64
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
i16
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
impl<'a>
Shr
<
i32
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
u64
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
i32
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
impl<'a>
Shr
<
i64
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
u64
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
i64
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
impl<'a>
Shr
<
i8
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
u64
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
i8
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
impl<'a>
Shr
<
u128
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
u64
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
u128
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
impl<'a>
Shr
<
u16
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
u64
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
u16
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
impl<'a>
Shr
<
u32
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
u64
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
u32
) ->
u64
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
u64
> for &'lhs
Simd
<
u64
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
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u64
) -> <&'lhs
Simd
<
u64
, N> as
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
impl<'a>
Shr
<
u64
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
i16
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
impl<'a>
Shr
<
u64
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
i32
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
impl<'a>
Shr
<
u64
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
i64
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
impl<'a>
Shr
<
u64
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
i8
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
impl<'a>
Shr
<
u64
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
u128
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
impl<'a>
Shr
<
u64
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
u16
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
impl<'a>
Shr
<
u64
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
u32
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
impl<'a>
Shr
<
u64
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
u64
) -> <
u64
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
impl<'a>
Shr
<
u64
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
usize
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
impl<const N:
usize
>
Shr
<
u64
> for
Simd
<
u64
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
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u64
) -> <
Simd
<
u64
, N> as
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
impl
Shr
<
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
impl
Shr
<
u64
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
u64
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
u64
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
u64
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
u64
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
u64
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
1.0.0
·
Source
§
impl
Shr
<
u64
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
u64
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
impl<'a>
Shr
<
usize
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
u64
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
usize
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
for
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
u64
) ->
u64
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
i16
> for
u64
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
u64
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
u64
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
u64
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
u128
> for
u64
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
u64
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
u64
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
u64
> for
i16
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
u64
> for
i32
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
u64
> for
i64
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
u64
> for
i8
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
u64
> for
u128
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
u64
> for
u16
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
u64
> for
u32
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
u64
> for
u64
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
u64
> for
usize
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
usize
> for
u64
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
i16
> for
u64
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
u64
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
u64
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
u64
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
u128
> for
u64
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
u64
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
u64
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
u64
> for
i16
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
u64
> for
i32
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
u64
> for
i64
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
u64
> for
i8
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
u64
> for
u128
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
u64
> for
u16
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
u64
> for
u32
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
u64
> for
usize
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
usize
> for
u64
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
u64
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
Source
§
impl
SimdElement
for
u64
Source
§
type
Mask
=
i64
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
u64
Source
§
fn
forward
(start:
u64
, n:
usize
) ->
u64
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
u64
, n:
usize
) ->
u64
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
u64
, n:
usize
) ->
u64
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
u64
, n:
usize
) ->
u64
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
u64
, end: &
u64
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
u64
, n:
usize
) ->
Option
<
u64
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
u64
, n:
usize
) ->
Option
<
u64
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
u64
> for &
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
> for &'a
u64
Source
§
type
Output
= <
u64
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
u64
) -> <
u64
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
u64
Source
§
type
Output
=
u64
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
u64
) ->
u64
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
u64
> for
u64
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
1.8.0
·
Source
§
impl
SubAssign
for
u64
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
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
u64
> for
u64
Source
§
fn
sum
<I>(iter: I) ->
u64
where
    I:
Iterator
<Item = &'a
u64
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
u64
Source
§
fn
sum
<I>(iter: I) ->
u64
where
    I:
Iterator
<Item =
u64
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
i16
> for
u64
Source
§
fn
try_from
(u:
i16
) ->
Result
<
u64
, <
u64
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
u64
Source
§
fn
try_from
(u:
i32
) ->
Result
<
u64
, <
u64
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
u64
Source
§
fn
try_from
(u:
i64
) ->
Result
<
u64
, <
u64
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
u64
Source
§
fn
try_from
(u:
i8
) ->
Result
<
u64
, <
u64
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
u128
> for
u64
Source
§
fn
try_from
(u:
u128
) ->
Result
<
u64
, <
u64
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
1.34.0
·
Source
§
impl
TryFrom
<
u64
> for
i16
Source
§
fn
try_from
(u:
u64
) ->
Result
<
i16
, <
i16
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
u64
> for
i32
Source
§
fn
try_from
(u:
u64
) ->
Result
<
i32
, <
i32
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
u64
> for
i64
Source
§
fn
try_from
(u:
u64
) ->
Result
<
i64
, <
i64
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
u64
> for
i8
Source
§
fn
try_from
(u:
u64
) ->
Result
<
i8
, <
i8
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
u64
> for
u16
Source
§
fn
try_from
(u:
u64
) ->
Result
<
u16
, <
u16
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
u64
> for
u32
Source
§
fn
try_from
(u:
u64
) ->
Result
<
u32
, <
u32
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
1.34.0
·
Source
§
impl
TryFrom
<
u64
> for
usize
Source
§
fn
try_from
(value:
u64
) ->
Result
<
usize
, <
usize
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
u64
Source
§
fn
try_from
(value:
usize
) ->
Result
<
u64
, <
u64
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
u64
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
u64
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
u64
1.0.0
·
Source
§
impl
Copy
for
u64
1.0.0
·
Source
§
impl
Eq
for
u64
Source
§
impl
FloatToInt
<
u64
> for
f128
Source
§
impl
FloatToInt
<
u64
> for
f16
Source
§
impl
FloatToInt
<
u64
> for
f32
Source
§
impl
FloatToInt
<
u64
> for
f64
Source
§
impl
SimdCast
for
u64
Source
§
impl
StructuralPartialEq
for
u64
Source
§
impl
TrustedStep
for
u64
Source
§
impl
UnsizedConstParamTy
for
u64
Source
§
impl
UseCloned
for
u64
Source
§
impl
ZeroablePrimitive
for
u64
Auto Trait Implementations
§
§
impl
Freeze
for
u64
§
impl
RefUnwindSafe
for
u64
§
impl
Send
for
u64
§
impl
Sync
for
u64
§
impl
Unpin
for
u64
§
impl
UnwindSafe
for
u64
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