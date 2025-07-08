aggregate_raw_ptr in std::intrinsics - Rust
std
::
intrinsics
Function
aggregate_raw_ptr
Copy item path
Source
pub const fn aggregate_raw_ptr<P, D, M>(data: D, meta: M) -> P
where
    P:
AggregateRawPtr
<D, Metadata = M>,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Lowers in MIR to
Rvalue::Aggregate
with
AggregateKind::RawPtr
.
This is used to implement functions like
slice::from_raw_parts_mut
and
ptr::from_raw_parts
in a way compatible with the compiler being able to
change the possible layouts of pointers.