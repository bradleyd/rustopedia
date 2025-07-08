fmuladdf64 in std::intrinsics - Rust
std
::
intrinsics
Function
fmuladdf64
Copy item path
Source
pub unsafe fn fmuladdf64(a:
f64
, b:
f64
, c:
f64
) ->
f64
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f64
values, non-deterministically executing
either a fused multiply-add or two operations with rounding of the
intermediate result.
The operation is fused if the code generator determines that target
instruction set has support for a fused operation, and that the fused
operation is more efficient than the equivalent, separate pair of mul
and add instructions. It is unspecified whether or not a fused operation
is selected, and that may depend on optimization level and context, for
example.