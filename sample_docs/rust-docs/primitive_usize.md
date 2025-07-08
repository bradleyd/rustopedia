usize - Rust
Primitive Type
usize
Copy item path
1.0.0
Expand description
The pointer-sized unsigned integer type.
The size of this primitive is how many bytes it takes to reference any
location in memory. For example, on a 32 bit target, this is 4 bytes
and on a 64 bit target, this is 8 bytes.
Implementations
§
Source
§
impl
usize
1.43.0
·
Source
pub const
MIN
:
usize
= 0usize
The smallest value that can be represented by this integer type.
§
Examples
Basic usage:
assert_eq!
(usize::MIN,
0
);
1.43.0
·
Source
pub const
MAX
:
usize
= 18_446_744_073_709_551_615usize
The largest value that can be represented by this integer type
(2
64
− 1 on 64-bit targets).
§
Examples
Basic usage:
assert_eq!
(usize::MAX,
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
(usize::BITS,
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
0b01001100usize
;
assert_eq!
(n.count_ones(),
3
);
let
max = usize::MAX;
assert_eq!
(max.count_ones(),
64
);
let
zero =
0usize
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
0usize
;
assert_eq!
(zero.count_zeros(),
64
);
let
max = usize::MAX;
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
n = usize::MAX >>
2
;
assert_eq!
(n.leading_zeros(),
2
);
let
zero =
0usize
;
assert_eq!
(zero.leading_zeros(),
64
);
let
max = usize::MAX;
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
0b0101000usize
;
assert_eq!
(n.trailing_zeros(),
3
);
let
zero =
0usize
;
assert_eq!
(zero.trailing_zeros(),
64
);
let
max = usize::MAX;
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
n = !(usize::MAX >>
2
);
assert_eq!
(n.leading_ones(),
2
);
let
zero =
0usize
;
assert_eq!
(zero.leading_ones(),
0
);
let
max = usize::MAX;
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
0b1010111usize
;
assert_eq!
(n.trailing_ones(),
3
);
let
zero =
0usize
;
assert_eq!
(zero.trailing_ones(),
0
);
let
max = usize::MAX;
assert_eq!
(max.trailing_ones(),
64
);
Source
pub const fn
isolate_most_significant_one
(self) ->
usize
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
n: usize =
0b_01100100
;
assert_eq!
(n.isolate_most_significant_one(),
0b_01000000
);
assert_eq!
(
0_usize
.isolate_most_significant_one(),
0
);
Source
pub const fn
isolate_least_significant_one
(self) ->
usize
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
n: usize =
0b_01100100
;
assert_eq!
(n.isolate_least_significant_one(),
0b_00000100
);
assert_eq!
(
0_usize
.isolate_least_significant_one(),
0
);
1.87.0 (const: 1.87.0)
·
Source
pub const fn
cast_signed
(self) ->
isize
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
n = usize::MAX;
assert_eq!
(n.cast_signed(), -
1isize
);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
rotate_left
(self, n:
u32
) ->
usize
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
0xaa00000000006e1usize
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
usize
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
0x6e10aausize
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
usize
Reverses the byte order of the integer.
§
Examples
Basic usage:
let
n =
0x1234567890123456usize
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
usize
Reverses the order of bits in the integer. The least significant bit becomes the most significant bit,
second least-significant bit becomes second most-significant bit, etc.
§
Examples
Basic usage:
let
n =
0x1234567890123456usize
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
0usize
.reverse_bits());
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_be
(x:
usize
) ->
usize
Converts an integer from big endian to the target’s endianness.
On big endian this is a no-op. On little endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Ausize
;
if
cfg!
(target_endian =
"big"
) {
assert_eq!
(usize::from_be(n), n)
}
else
{
assert_eq!
(usize::from_be(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
from_le
(x:
usize
) ->
usize
Converts an integer from little endian to the target’s endianness.
On little endian this is a no-op. On big endian the bytes are
swapped.
§
Examples
Basic usage:
let
n =
0x1Ausize
;
if
cfg!
(target_endian =
"little"
) {
assert_eq!
(usize::from_le(n), n)
}
else
{
assert_eq!
(usize::from_le(n), n.swap_bytes())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
to_be
(self) ->
usize
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
0x1Ausize
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
usize
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
0x1Ausize
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
usize
) ->
Option
<
usize
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
((usize::MAX -
2
).checked_add(
1
),
Some
(usize::MAX -
1
));
assert_eq!
((usize::MAX -
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
usize
) ->
usize
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
((usize::MAX -
2
).strict_add(
1
), usize::MAX -
1
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= (usize::MAX -
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
usize
) ->
usize
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
self + rhs > usize::MAX
or
self + rhs < usize::MIN
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
isize
) ->
Option
<
usize
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
1usize
.checked_add_signed(
2
),
Some
(
3
));
assert_eq!
(
1usize
.checked_add_signed(-
2
),
None
);
assert_eq!
((usize::MAX -
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
isize
) ->
usize
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
1usize
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
1usize
.strict_add_signed(-
2
);
ⓘ
#![feature(strict_overflow_ops)]
let _
= (usize::MAX -
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
usize
) ->
Option
<
usize
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
1usize
.checked_sub(
1
),
Some
(
0
));
assert_eq!
(
0usize
.checked_sub(
1
),
None
);
Source
pub const fn
strict_sub
(self, rhs:
usize
) ->
usize
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
1usize
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
0usize
.strict_sub(
1
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_sub
(self, rhs:
usize
) ->
usize
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
self - rhs > usize::MAX
or
self - rhs < usize::MIN
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
isize
) ->
Option
<
usize
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
1usize
.checked_sub_signed(
2
),
None
);
assert_eq!
(
1usize
.checked_sub_signed(-
2
),
Some
(
3
));
assert_eq!
((usize::MAX -
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
usize
) ->
Option
<
isize
>
🔬
This is a nightly-only experimental API. (
unsigned_signed_diff
#126041
)
Checked integer subtraction. Computes
self - rhs
and checks if the result fits into an
isize
, returning
None
if overflow occurred.
§
Examples
Basic usage:
#![feature(unsigned_signed_diff)]
assert_eq!
(
10usize
.checked_signed_diff(
2
),
Some
(
8
));
assert_eq!
(
2usize
.checked_signed_diff(
10
),
Some
(-
8
));
assert_eq!
(usize::MAX.checked_signed_diff(isize::MAX
as
usize),
None
);
assert_eq!
((isize::MAX
as
usize).checked_signed_diff(usize::MAX),
Some
(isize::MIN));
assert_eq!
((isize::MAX
as
usize +
1
).checked_signed_diff(
0
),
None
);
assert_eq!
(usize::MAX.checked_signed_diff(usize::MAX),
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
usize
) ->
Option
<
usize
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
5usize
.checked_mul(
1
),
Some
(
5
));
assert_eq!
(usize::MAX.checked_mul(
2
),
None
);
Source
pub const fn
strict_mul
(self, rhs:
usize
) ->
usize
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
5usize
.strict_mul(
1
),
5
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= usize::MAX.strict_mul(
2
);
1.79.0 (const: 1.79.0)
·
Source
pub const unsafe fn
unchecked_mul
(self, rhs:
usize
) ->
usize
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
self * rhs > usize::MAX
or
self * rhs < usize::MIN
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
usize
) ->
Option
<
usize
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
128usize
.checked_div(
2
),
Some
(
64
));
assert_eq!
(
1usize
.checked_div(
0
),
None
);
Source
pub const fn
strict_div
(self, rhs:
usize
) ->
usize
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
100usize
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
1usize
).strict_div(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_div_euclid
(self, rhs:
usize
) ->
Option
<
usize
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
128usize
.checked_div_euclid(
2
),
Some
(
64
));
assert_eq!
(
1usize
.checked_div_euclid(
0
),
None
);
Source
pub const fn
strict_div_euclid
(self, rhs:
usize
) ->
usize
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
100usize
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
1usize
).strict_div_euclid(
0
);
1.7.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem
(self, rhs:
usize
) ->
Option
<
usize
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
5usize
.checked_rem(
2
),
Some
(
1
));
assert_eq!
(
5usize
.checked_rem(
0
),
None
);
Source
pub const fn
strict_rem
(self, rhs:
usize
) ->
usize
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
100usize
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
5usize
.strict_rem(
0
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
checked_rem_euclid
(self, rhs:
usize
) ->
Option
<
usize
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
5usize
.checked_rem_euclid(
2
),
Some
(
1
));
assert_eq!
(
5usize
.checked_rem_euclid(
0
),
None
);
Source
pub const fn
strict_rem_euclid
(self, rhs:
usize
) ->
usize
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
100usize
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
5usize
.strict_rem_euclid(
0
);
Source
pub const unsafe fn
unchecked_disjoint_bitor
(self, other:
usize
) ->
usize
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
1_usize
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
usize
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
5usize
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
2usize
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
10usize
.ilog10(),
1
);
1.67.0 (const: 1.67.0)
·
Source
pub const fn
checked_ilog
(self, base:
usize
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
5usize
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
2usize
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
10usize
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
usize
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
0usize
.checked_neg(),
Some
(
0
));
assert_eq!
(
1usize
.checked_neg(),
None
);
Source
pub const fn
strict_neg
(self) ->
usize
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
0usize
.strict_neg(),
0
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
=
1usize
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
usize
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
0x1usize
.checked_shl(
4
),
Some
(
0x10
));
assert_eq!
(
0x10usize
.checked_shl(
129
),
None
);
assert_eq!
(
0x10usize
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
usize
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
0x1usize
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
0x10usize
.strict_shl(
129
);
Source
pub const unsafe fn
unchecked_shl
(self, rhs:
u32
) ->
usize
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
usize
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
0x1usize
.unbounded_shl(
4
),
0x10
);
assert_eq!
(
0x1usize
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
usize
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
0x10usize
.checked_shr(
4
),
Some
(
0x1
));
assert_eq!
(
0x10usize
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
usize
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
0x10usize
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
0x10usize
.strict_shr(
129
);
Source
pub const unsafe fn
unchecked_shr
(self, rhs:
u32
) ->
usize
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
usize
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
0x10usize
.unbounded_shr(
4
),
0x1
);
assert_eq!
(
0x10usize
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
usize
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
2usize
.checked_pow(
5
),
Some
(
32
));
assert_eq!
(usize::MAX.checked_pow(
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
usize
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
2usize
.strict_pow(
5
),
32
);
The following panics because of overflow:
ⓘ
#![feature(strict_overflow_ops)]
let _
= usize::MAX.strict_pow(
2
);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_add
(self, rhs:
usize
) ->
usize
Saturating integer addition. Computes
self + rhs
, saturating at
the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100usize
.saturating_add(
1
),
101
);
assert_eq!
(usize::MAX.saturating_add(
127
), usize::MAX);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
saturating_add_signed
(self, rhs:
isize
) ->
usize
Saturating addition with a signed integer. Computes
self + rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
1usize
.saturating_add_signed(
2
),
3
);
assert_eq!
(
1usize
.saturating_add_signed(-
2
),
0
);
assert_eq!
((usize::MAX -
2
).saturating_add_signed(
4
), usize::MAX);
1.0.0 (const: 1.47.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
usize
) ->
usize
Saturating integer subtraction. Computes
self - rhs
, saturating
at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
100usize
.saturating_sub(
27
),
73
);
assert_eq!
(
13usize
.saturating_sub(
127
),
0
);
Source
pub const fn
saturating_sub_signed
(self, rhs:
isize
) ->
usize
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
1usize
.saturating_sub_signed(
2
),
0
);
assert_eq!
(
1usize
.saturating_sub_signed(-
2
),
3
);
assert_eq!
((usize::MAX -
2
).saturating_sub_signed(-
4
), usize::MAX);
1.7.0 (const: 1.47.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
usize
) ->
usize
Saturating integer multiplication. Computes
self * rhs
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
2usize
.saturating_mul(
10
),
20
);
assert_eq!
((usize::MAX).saturating_mul(
10
), usize::MAX);
1.58.0 (const: 1.58.0)
·
Source
pub const fn
saturating_div
(self, rhs:
usize
) ->
usize
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
5usize
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
usize
Saturating integer exponentiation. Computes
self.pow(exp)
,
saturating at the numeric bounds instead of overflowing.
§
Examples
Basic usage:
assert_eq!
(
4usize
.saturating_pow(
3
),
64
);
assert_eq!
(usize::MAX.saturating_pow(
2
), usize::MAX);
1.0.0 (const: 1.32.0)
·
Source
pub const fn
wrapping_add
(self, rhs:
usize
) ->
usize
Wrapping (modular) addition. Computes
self + rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
200usize
.wrapping_add(
55
),
255
);
assert_eq!
(
200usize
.wrapping_add(usize::MAX),
199
);
1.66.0 (const: 1.66.0)
·
Source
pub const fn
wrapping_add_signed
(self, rhs:
isize
) ->
usize
Wrapping (modular) addition with a signed integer. Computes
self + rhs
, wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
1usize
.wrapping_add_signed(
2
),
3
);
assert_eq!
(
1usize
.wrapping_add_signed(-
2
), usize::MAX);
assert_eq!
((usize::MAX -
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
usize
) ->
usize
Wrapping (modular) subtraction. Computes
self - rhs
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
100usize
.wrapping_sub(
100
),
0
);
assert_eq!
(
100usize
.wrapping_sub(usize::MAX),
101
);
Source
pub const fn
wrapping_sub_signed
(self, rhs:
isize
) ->
usize
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
1usize
.wrapping_sub_signed(
2
), usize::MAX);
assert_eq!
(
1usize
.wrapping_sub_signed(-
2
),
3
);
assert_eq!
((usize::MAX -
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
usize
) ->
usize
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
usize
) ->
usize
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
100usize
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
usize
) ->
usize
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
100usize
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
usize
) ->
usize
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
100usize
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
usize
) ->
usize
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
100usize
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
usize
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
0_usize
.wrapping_neg(),
0
);
assert_eq!
(usize::MAX.wrapping_neg(),
1
);
assert_eq!
(
13_usize
.wrapping_neg(), (!
13
) +
1
);
assert_eq!
(
42_usize
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
usize
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
1usize
.wrapping_shl(
7
),
128
);
assert_eq!
(
1usize
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
usize
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
128usize
.wrapping_shr(
7
),
1
);
assert_eq!
(
128usize
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
usize
Wrapping (modular) exponentiation. Computes
self.pow(exp)
,
wrapping around at the boundary of the type.
§
Examples
Basic usage:
assert_eq!
(
3usize
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
usize
) -> (
usize
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
5usize
.overflowing_add(
2
), (
7
,
false
));
assert_eq!
(usize::MAX.overflowing_add(
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
usize
, carry:
bool
) -> (
usize
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
(a1, a0): (usize, usize) = (
3
, usize::MAX);
let
(b1, b0): (usize, usize) = (
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
isize
) -> (
usize
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
1usize
.overflowing_add_signed(
2
), (
3
,
false
));
assert_eq!
(
1usize
.overflowing_add_signed(-
2
), (usize::MAX,
true
));
assert_eq!
((usize::MAX -
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
usize
) -> (
usize
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
5usize
.overflowing_sub(
2
), (
3
,
false
));
assert_eq!
(
0usize
.overflowing_sub(
1
), (usize::MAX,
true
));
Source
pub const fn
borrowing_sub
(self, rhs:
usize
, borrow:
bool
) -> (
usize
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
(a1, a0): (usize, usize) = (
9
,
6
);
let
(b1, b0): (usize, usize) = (
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
, usize::MAX));
Source
pub const fn
overflowing_sub_signed
(self, rhs:
isize
) -> (
usize
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
1usize
.overflowing_sub_signed(
2
), (usize::MAX,
true
));
assert_eq!
(
1usize
.overflowing_sub_signed(-
2
), (
3
,
false
));
assert_eq!
((usize::MAX -
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
usize
) ->
usize
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
100usize
.abs_diff(
80
),
20usize
);
assert_eq!
(
100usize
.abs_diff(
110
),
10usize
);
1.7.0 (const: 1.32.0)
·
Source
pub const fn
overflowing_mul
(self, rhs:
usize
) -> (
usize
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
usize
) -> (
usize
,
usize
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
usize
, carry:
usize
) -> (
usize
,
usize
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
(usize::MAX.carrying_mul(usize::MAX, usize::MAX), (
0
, usize::MAX));
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
usize
,
    carry:
usize
,
    add:
usize
,
) -> (
usize
,
usize
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
(usize::MAX.carrying_mul_add(usize::MAX, usize::MAX, usize::MAX), (usize::MAX, usize::MAX));
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
usize
) -> (
usize
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
5usize
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
usize
) -> (
usize
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
5usize
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
usize
) -> (
usize
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
5usize
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
usize
) -> (
usize
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
5usize
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
usize
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
0usize
.overflowing_neg(), (
0
,
false
));
assert_eq!
(
2usize
.overflowing_neg(), (-
2i32
as
usize,
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
usize
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
0x1usize
.overflowing_shl(
4
), (
0x10
,
false
));
assert_eq!
(
0x1usize
.overflowing_shl(
132
), (
0x10
,
true
));
assert_eq!
(
0x10usize
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
usize
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
0x10usize
.overflowing_shr(
4
), (
0x1
,
false
));
assert_eq!
(
0x10usize
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
usize
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
3usize
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
usize
Raises self to the power of
exp
, using exponentiation by squaring.
§
Examples
Basic usage:
assert_eq!
(
2usize
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
usize
Returns the square root of the number, rounded down.
§
Examples
Basic usage:
assert_eq!
(
10usize
.isqrt(),
3
);
1.38.0 (const: 1.52.0)
·
Source
pub const fn
div_euclid
(self, rhs:
usize
) ->
usize
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
7usize
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
usize
) ->
usize
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
7usize
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
usize
) ->
usize
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
7_usize
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
usize
) ->
usize
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
7_usize
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
usize
) ->
usize
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
16_usize
.next_multiple_of(
8
),
16
);
assert_eq!
(
23_usize
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
usize
) ->
Option
<
usize
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
16_usize
.checked_next_multiple_of(
8
),
Some
(
16
));
assert_eq!
(
23_usize
.checked_next_multiple_of(
8
),
Some
(
24
));
assert_eq!
(
1_usize
.checked_next_multiple_of(
0
),
None
);
assert_eq!
(usize::MAX.checked_next_multiple_of(
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
usize
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
6_usize
.is_multiple_of(
2
));
assert!
(!
5_usize
.is_multiple_of(
2
));
assert!
(
0_usize
.is_multiple_of(
0
));
assert!
(!
6_usize
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
16usize
.is_power_of_two());
assert!
(!
10usize
.is_power_of_two());
1.0.0 (const: 1.50.0)
·
Source
pub const fn
next_power_of_two
(self) ->
usize
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
2usize
.next_power_of_two(),
2
);
assert_eq!
(
3usize
.next_power_of_two(),
4
);
assert_eq!
(
0usize
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
usize
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
2usize
.checked_next_power_of_two(),
Some
(
2
));
assert_eq!
(
3usize
.checked_next_power_of_two(),
Some
(
4
));
assert_eq!
(usize::MAX.checked_next_power_of_two(),
None
);
Source
pub const fn
wrapping_next_power_of_two
(self) ->
usize
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
2usize
.wrapping_next_power_of_two(),
2
);
assert_eq!
(
3usize
.wrapping_next_power_of_two(),
4
);
assert_eq!
(usize::MAX.wrapping_next_power_of_two(),
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
Note
: This function returns an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
bytes =
0x1234567890123456usize
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
0x1234567890123456usize
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
0x1234567890123456usize
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
usize
Creates a native endian integer value from its representation
as a byte array in big endian.
Note
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = usize::from_be_bytes([
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
read_be_usize(input:
&mut &
[u8]) -> usize {
let
(int_bytes, rest) = input.split_at(size_of::<usize>());
*
input = rest;
    usize::from_be_bytes(int_bytes.try_into().unwrap())
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
usize
Creates a native endian integer value from its representation
as a byte array in little endian.
Note
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = usize::from_le_bytes([
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
read_le_usize(input:
&mut &
[u8]) -> usize {
let
(int_bytes, rest) = input.split_at(size_of::<usize>());
*
input = rest;
    usize::from_le_bytes(int_bytes.try_into().unwrap())
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
usize
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
: This function takes an array of length 2, 4 or 8 bytes
depending on the target pointer size.
§
Examples
let
value = usize::from_ne_bytes(
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
read_ne_usize(input:
&mut &
[u8]) -> usize {
let
(int_bytes, rest) = input.split_at(size_of::<usize>());
*
input = rest;
    usize::from_ne_bytes(int_bytes.try_into().unwrap())
}
1.0.0 (const: 1.32.0)
·
Source
pub const fn
min_value
() ->
usize
👎
Deprecating in a future version: replaced by the
MIN
associated constant on this type
New code should prefer to use
usize::MIN
instead.
Returns the smallest value that can be represented by this integer type.
1.0.0 (const: 1.32.0)
·
Source
pub const fn
max_value
() ->
usize
👎
Deprecating in a future version: replaced by the
MAX
associated constant on this type
New code should prefer to use
usize::MAX
instead.
Returns the largest value that can be represented by this integer type.
1.85.0 (const: 1.85.0)
·
Source
pub const fn
midpoint
(self, rhs:
usize
) ->
usize
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
0usize
.midpoint(
4
),
2
);
assert_eq!
(
1usize
.midpoint(
4
),
2
);
Source
§
impl
usize
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
usize
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
(usize::from_str_radix(
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
(usize::from_str_radix(
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
usize
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
(usize::from_ascii(
b"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(usize::from_ascii(
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
usize
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
(usize::from_ascii_radix(
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
(usize::from_ascii_radix(
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
+
operator.
Source
§
const fn
add
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
AddAssign
<&
usize
> for
usize
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
usize
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
1.0.0
·
Source
§
impl
Binary
for
usize
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
BitAndAssign
<&
usize
> for
usize
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
usize
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
1.0.0
·
Source
§
impl
BitOr
<&
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
BitOrAssign
<&
usize
> for
usize
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
usize
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
1.0.0
·
Source
§
impl
BitXor
<&
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
BitXorAssign
<&
usize
> for
usize
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
usize
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
Source
§
impl
CarryingMulAdd
for
usize
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
usize
, b:
usize
, c:
usize
) -> (
usize
,
usize
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
usize
Source
§
fn
clone
(&self) ->
usize
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
usize
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
usize
Source
§
fn
default
() ->
usize
Returns the default value of
0
Source
§
impl
DisjointBitOr
for
usize
Source
§
const unsafe fn
disjoint_bitor
(self, other:
usize
) ->
usize
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
usize
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
1.0.0
·
Source
§
impl<'a>
Div
<
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
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
usize
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
DivAssign
<&
usize
> for
usize
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
usize
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
Source
§
impl
From
<
Alignment
> for
usize
Source
§
fn
from
(align:
Alignment
) ->
usize
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
usize
Source
§
fn
from
(small:
bool
) ->
usize
Converts a
bool
to
usize
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
(usize::from(
true
),
1
);
assert_eq!
(usize::from(
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
u16
> for
usize
Source
§
fn
from
(small:
u16
) ->
usize
Converts
u16
to
usize
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
1.23.0
·
Source
§
impl
From
<
usize
> for
AtomicUsize
Source
§
fn
from
(v:
usize
) ->
AtomicUsize
Converts an
usize
into an
AtomicUsize
.
1.0.0
·
Source
§
impl
FromStr
for
usize
Source
§
fn
from_str
(src: &
str
) ->
Result
<
usize
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
(usize::from_str(
"+10"
),
Ok
(
10
));
Trailing space returns error:
assert!
(usize::from_str(
"1 "
).is_err());
Source
§
type
Err
=
ParseIntError
The associated error which can be returned from parsing.
Source
§
impl
GetDisjointMutIndex
for
usize
Source
§
fn
is_in_bounds
(&self, len:
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
get_disjoint_mut_helpers
)
Returns
true
if
self
is in bounds for
len
slice elements.
Source
§
fn
is_overlapping
(&self, other: &
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
get_disjoint_mut_helpers
)
Returns
true
if
self
overlaps with
other
.
Read more
1.0.0
·
Source
§
impl
Hash
for
usize
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
usize
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
Source
§
impl
Index
<
usize
> for
ByteStr
Source
§
type
Output
=
u8
The returned type after indexing.
Source
§
fn
index
(&self, idx:
usize
) -> &
u8
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
usize
> for
ByteString
Source
§
type
Output
=
u8
The returned type after indexing.
Source
§
fn
index
(&self, idx:
usize
) -> &
u8
Performs the indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl<T, A>
Index
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Output
= T
The returned type after indexing.
Source
§
fn
index
(&self, index:
usize
) ->
&T
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
usize
> for
ByteStr
Source
§
fn
index_mut
(&mut self, idx:
usize
) -> &mut
u8
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
usize
> for
ByteString
Source
§
fn
index_mut
(&mut self, idx:
usize
) -> &mut
u8
Performs the mutable indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl<T, A>
IndexMut
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
fn
index_mut
(&mut self, index:
usize
) ->
&mut T
Performs the mutable indexing (
container[index]
) operation.
Read more
1.42.0
·
Source
§
impl
LowerExp
for
usize
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
usize
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
MulAssign
<&
usize
> for
usize
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
usize
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
1.0.0
·
Source
§
impl
Not
for &
usize
Source
§
type
Output
= <
usize
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
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
usize
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
usize
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
usize
Source
§
fn
cmp
(&self, other: &
usize
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
usize
Source
§
fn
eq
(&self, other: &
usize
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
usize
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
usize
Source
§
fn
partial_cmp
(&self, other: &
usize
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
usize
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
usize
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
usize
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
usize
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
usize
> for
usize
Source
§
fn
product
<I>(iter: I) ->
usize
where
    I:
Iterator
<Item = &'a
usize
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
usize
Source
§
fn
product
<I>(iter: I) ->
usize
where
    I:
Iterator
<Item =
usize
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
usize
Source
§
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) ->
usize
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
usize
Source
§
const
MIN
:
usize
= 0usize
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
usize
= 18_446_744_073_709_551_615usize
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
usize
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
1.0.0
·
Source
§
impl<'a>
Rem
<
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
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
usize
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
RemAssign
<&
usize
> for
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
usize
> for &'lhs
Simd
<
usize
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
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shl
<&
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
i16
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
i32
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
i64
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
i8
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
u128
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
u16
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
u32
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
usize
) -> <
usize
as
Shl
>::
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
usize
> for
Simd
<
usize
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
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
usize
) -> <
Simd
<
usize
, N> as
Shl
<&
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
impl
Shl
<&
usize
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
i16
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
i32
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
i64
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
i8
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
impl
Shl
<&
usize
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
u128
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
u16
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
u32
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
impl
Shl
<&
usize
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
usize
) -> <
usize
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
usize
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
i16
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
i32
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
usize
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
i32
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
i64
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
usize
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
i64
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
i8
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
usize
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
i8
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
usize
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
u128
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
u16
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
usize
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
u16
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
u32
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
usize
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
u32
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
Source
§
impl<'lhs, const N:
usize
>
Shl
<
usize
> for &'lhs
Simd
<
usize
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
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
usize
) -> <&'lhs
Simd
<
usize
, N> as
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
impl<'a>
Shl
<
usize
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
i16
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
usize
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
i32
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
usize
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
i64
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
usize
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
i8
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
impl<'a>
Shl
<
usize
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
u128
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
usize
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
u16
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
usize
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
u32
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
impl<'a>
Shl
<
usize
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
usize
) -> <
usize
as
Shl
>::
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
usize
> for
Simd
<
usize
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
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
usize
) -> <
Simd
<
usize
, N> as
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
<
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
<
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
) ->
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
i16
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
i32
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
i64
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
i8
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
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
u128
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
u16
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
u32
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
1.22.0
·
Source
§
impl
ShlAssign
<&
usize
> for
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
<
usize
> for
i16
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
i32
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
i64
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
i8
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
<
usize
> for
u128
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
u16
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
u32
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
usize
> for &'lhs
Simd
<
usize
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
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shr
<&
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
i16
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
i32
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
i64
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
i8
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
u128
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
u16
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
u32
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
usize
) -> <
usize
as
Shr
>::
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
usize
> for
Simd
<
usize
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
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
usize
) -> <
Simd
<
usize
, N> as
Shr
<&
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
impl
Shr
<&
usize
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
i16
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
i32
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
i64
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
i8
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
impl
Shr
<&
usize
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
u128
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
u16
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
u32
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
impl
Shr
<&
usize
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
usize
) -> <
usize
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
usize
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
i16
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
i32
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
usize
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
i32
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
i64
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
usize
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
i64
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
i8
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
usize
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
i8
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
usize
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
u128
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
u16
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
usize
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
u16
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
u32
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
usize
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
u32
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
Source
§
impl<'lhs, const N:
usize
>
Shr
<
usize
> for &'lhs
Simd
<
usize
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
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
usize
) -> <&'lhs
Simd
<
usize
, N> as
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
impl<'a>
Shr
<
usize
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
i16
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
usize
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
i32
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
usize
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
i64
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
usize
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
i8
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
impl<'a>
Shr
<
usize
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
u128
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
usize
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
u16
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
usize
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
u32
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
impl<'a>
Shr
<
usize
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
usize
) -> <
usize
as
Shr
>::
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
usize
> for
Simd
<
usize
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
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
usize
) -> <
Simd
<
usize
, N> as
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
<
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
<
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
) ->
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i16
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
i32
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
i64
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
i8
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
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u128
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
u16
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
u32
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
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
usize
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
<
usize
> for
i16
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
i32
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
i64
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
i8
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
<
usize
> for
u128
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
u16
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
u32
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
usize
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
Source
§
impl
SimdElement
for
usize
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
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
usize
The methods
index
and
index_mut
panic if the index is out of bounds.
Source
§
type
Output
= T
The output type returned by methods.
Source
§
fn
get
(self, slice: &
[T]
) ->
Option
<
&T
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(self, slice: &mut
[T]
) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(self, slice:
*const
[T]
) ->
*const T
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(self, slice:
*mut
[T]
) ->
*mut T
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
[T]
) ->
&T
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(self, slice: &mut
[T]
) ->
&mut T
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
Source
§
impl
Step
for
usize
Source
§
fn
forward
(start:
usize
, n:
usize
) ->
usize
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
usize
, n:
usize
) ->
usize
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
usize
, n:
usize
) ->
usize
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
usize
, n:
usize
) ->
usize
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
usize
, end: &
usize
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
usize
, n:
usize
) ->
Option
<
usize
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
usize
, n:
usize
) ->
Option
<
usize
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
usize
> for &
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
> for &'a
usize
Source
§
type
Output
= <
usize
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
usize
) -> <
usize
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
usize
Source
§
type
Output
=
usize
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
usize
) ->
usize
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
1.22.0
·
Source
§
impl
SubAssign
<&
usize
> for
usize
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
usize
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
1.12.0
·
Source
§
impl<'a>
Sum
<&'a
usize
> for
usize
Source
§
fn
sum
<I>(iter: I) ->
usize
where
    I:
Iterator
<Item = &'a
usize
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
usize
Source
§
fn
sum
<I>(iter: I) ->
usize
where
    I:
Iterator
<Item =
usize
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
i16
> for
usize
Source
§
fn
try_from
(u:
i16
) ->
Result
<
usize
, <
usize
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
usize
Source
§
fn
try_from
(u:
i32
) ->
Result
<
usize
, <
usize
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
usize
Source
§
fn
try_from
(u:
i64
) ->
Result
<
usize
, <
usize
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
usize
Source
§
fn
try_from
(u:
i8
) ->
Result
<
usize
, <
usize
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
usize
Source
§
fn
try_from
(u:
u128
) ->
Result
<
usize
, <
usize
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
u32
> for
usize
Source
§
fn
try_from
(value:
u32
) ->
Result
<
usize
, <
usize
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
Source
§
impl
TryFrom
<
usize
> for
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
usize
,
) ->
Result
<
Alignment
, <
Alignment
as
TryFrom
<
usize
>>::
Error
>
Performs the conversion.
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
i16
Source
§
fn
try_from
(u:
usize
) ->
Result
<
i16
, <
i16
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
i32
Source
§
fn
try_from
(u:
usize
) ->
Result
<
i32
, <
i32
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
i64
Source
§
fn
try_from
(u:
usize
) ->
Result
<
i64
, <
i64
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
i8
Source
§
fn
try_from
(u:
usize
) ->
Result
<
i8
, <
i8
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
u128
Source
§
fn
try_from
(value:
usize
) ->
Result
<
u128
, <
u128
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
u16
Source
§
fn
try_from
(u:
usize
) ->
Result
<
u16
, <
u16
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
1.34.0
·
Source
§
impl
TryFrom
<
usize
> for
u32
Source
§
fn
try_from
(u:
usize
) ->
Result
<
u32
, <
u32
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
usize
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
usize
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
usize
1.0.0
·
Source
§
impl
Copy
for
usize
1.0.0
·
Source
§
impl
Eq
for
usize
Source
§
impl
FloatToInt
<
usize
> for
f128
Source
§
impl
FloatToInt
<
usize
> for
f16
Source
§
impl
FloatToInt
<
usize
> for
f32
Source
§
impl
FloatToInt
<
usize
> for
f64
Source
§
impl
PointerLike
for
usize
Source
§
impl
SimdCast
for
usize
Source
§
impl
StructuralPartialEq
for
usize
Source
§
impl
TrustedStep
for
usize
Source
§
impl
UnsizedConstParamTy
for
usize
Source
§
impl
UseCloned
for
usize
Source
§
impl
ZeroablePrimitive
for
usize
Auto Trait Implementations
§
§
impl
Freeze
for
usize
§
impl
RefUnwindSafe
for
usize
§
impl
Send
for
usize
§
impl
Sync
for
usize
§
impl
Unpin
for
usize
§
impl
UnwindSafe
for
usize
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