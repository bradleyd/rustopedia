CastPtrToPtr in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
CastPtrToPtr
Copy item path
Source
pub fn CastPtrToPtr<T, U>(operand: T) -> U
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Emits a
CastKind::PtrToPtr
cast.
This allows bypassing normal validation to generate strange casts.