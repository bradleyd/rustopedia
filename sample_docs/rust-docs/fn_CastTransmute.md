CastTransmute in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
CastTransmute
Copy item path
Source
pub fn CastTransmute<T, U>(operand: T) -> U
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Emits a
CastKind::Transmute
cast.
Needed to test the UB when
sizeof(T) != sizeof(U)
, which can’t be
generated via the normal
mem::transmute
.