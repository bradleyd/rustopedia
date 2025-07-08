ptr_guaranteed_cmp in std::intrinsics - Rust
std
::
intrinsics
Function
ptr_guaranteed_cmp
Copy item path
Source
pub const fn ptr_guaranteed_cmp<T>(ptr:
*const T
, other:
*const T
) ->
u8
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
See documentation of
<*const T>::guaranteed_eq
for details.
Returns
2
if the result is unknown.
Returns
1
if the pointers are guaranteed equal.
Returns
0
if the pointers are guaranteed inequal.