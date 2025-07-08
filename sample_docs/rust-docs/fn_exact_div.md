exact_div in std::intrinsics - Rust
std
::
intrinsics
Function
exact_div
Copy item path
Source
pub const unsafe fn exact_div<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs an exact division, resulting in undefined behavior where
x % y != 0
or
y == 0
or
x == T::MIN && y == -1
This intrinsic does not have a stable counterpart.