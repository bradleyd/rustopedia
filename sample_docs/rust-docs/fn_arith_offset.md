arith_offset in std::intrinsics - Rust
std
::
intrinsics
Function
arith_offset
Copy item path
Source
pub const unsafe fn arith_offset<T>(dst:
*const T
, offset:
isize
) ->
*const T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Calculates the offset from a pointer, potentially wrapping.
This is implemented as an intrinsic to avoid converting to and from an
integer, since the conversion inhibits certain optimizations.
§
Safety
Unlike the
offset
intrinsic, this intrinsic does not restrict the
resulting pointer to point into or at the end of an allocated
object, and it wraps with two’s complement arithmetic. The resulting
value is not necessarily valid to be used to actually access memory.
The stabilized version of this intrinsic is
pointer::wrapping_offset
.