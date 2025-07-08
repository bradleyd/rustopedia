carrying_mul_add in std::intrinsics - Rust
std
::
intrinsics
Function
carrying_mul_add
Copy item path
Source
pub const fn carrying_mul_add<T, U>(
    multiplier: T,
    multiplicand: T,
    addend: T,
    carry: T,
) ->
(U, T)
where
    T:
CarryingMulAdd
<Unsigned = U>,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs full-width multiplication and addition with a carry:
multiplier * multiplicand + addend + carry
.
This is possible without any overflow.  For
uN
:
MAX * MAX + MAX + MAX
=> (2ⁿ-1) × (2ⁿ-1) + (2ⁿ-1) + (2ⁿ-1)
=> (2²ⁿ - 2ⁿ⁺¹ + 1) + (2ⁿ⁺¹ - 2)
=> 2²ⁿ - 1
For
iN
, the upper bound is MIN * MIN + MAX + MAX => 2²ⁿ⁻² + 2ⁿ - 2,
and the lower bound is MAX * MIN + MIN + MIN => -2²ⁿ⁻² - 2ⁿ + 2ⁿ⁺¹.
This currently supports unsigned integers
only
, no signed ones.
The stabilized versions of this intrinsic are available on integers.