ptr_metadata in std::intrinsics - Rust
std
::
intrinsics
Function
ptr_metadata
Copy item path
Source
pub const fn ptr_metadata<P, M>(ptr:
*const P
) -> M
where
    P:
Pointee
<Metadata = M> + ?
Sized
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Lowers in MIR to
Rvalue::UnaryOp
with
UnOp::PtrMetadata
.
This is used to implement functions like
ptr::metadata
.