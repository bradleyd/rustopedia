fmuladdf16 in std::intrinsics - Rust
std
::
intrinsics
Function
fmuladdf16
Copy item path
Source
pub unsafe fn fmuladdf16(a:
f16
, b:
f16
, c:
f16
) ->
f16
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f16
values, non-deterministically executing
either a fused multiply-add or two operations with rounding of the
intermediate result.
The operation is fused if the code generator determines that target
instruction set has support for a fused operation, and that the fused
operation is more efficient than the equivalent, separate pair of mul
and add instructions. It is unspecified whether or not a fused operation
is selected, and that may depend on optimization level and context, for
example.