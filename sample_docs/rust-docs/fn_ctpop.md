ctpop in std::intrinsics - Rust
std
::
intrinsics
Function
ctpop
Copy item path
Source
pub const fn ctpop<T>(x: T) ->
u32
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the number of bits set in an integer type
T
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
count_ones
method. For example,
u32::count_ones