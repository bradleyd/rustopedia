forget_unsized in std::mem - Rust
std
::
mem
Function
forget_unsized
Copy item path
Source
pub fn forget_unsized<T>(t: T)
where
    T: ?
Sized
,
🔬
This is a nightly-only experimental API. (
forget_unsized
)
Expand description
Like
forget
, but also accepts unsized values.
This function is just a shim intended to be removed when the
unsized_locals
feature gets
stabilized.