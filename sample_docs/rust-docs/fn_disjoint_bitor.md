disjoint_bitor in std::intrinsics - Rust
std
::
intrinsics
Function
disjoint_bitor
Copy item path
Source
pub const unsafe fn disjoint_bitor<T>(a: T, b: T) -> T
where
    T:
DisjointBitOr
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Combine two values which have no bits in common.
This allows the backend to implement it as
a + b
or
a | b
,
depending which is easier to implement on a specific target.
§
Safety
Requires that
(a & b) == 0
, or equivalently that
(a | b) == (a + b)
.
Otherwise it’s immediate UB.