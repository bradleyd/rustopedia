three_way_compare in std::intrinsics - Rust
std
::
intrinsics
Function
three_way_compare
Copy item path
Source
pub const fn three_way_compare<T>(lhs: T, rhss: T) ->
Ordering
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Does a three-way comparison between the two integer arguments.
This is included as an intrinsic as it’s useful to let it be one thing
in MIR, rather than the multiple checks and switches that make its IR
large and difficult to optimize.
The stabilized version of this intrinsic is
Ord::cmp
.