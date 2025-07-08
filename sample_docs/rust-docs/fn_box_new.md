box_new in std::boxed - Rust
std
::
boxed
Function
box_new
Copy item path
Source
pub fn box_new<T>(x: T) ->
Box
<T>
🔬
This is a nightly-only experimental API. (
liballoc_internals
)
Expand description
Constructs a
Box<T>
by calling the
exchange_malloc
lang item and moving the argument into
the newly allocated memory. This is an intrinsic to avoid unnecessary copies.
This is the surface syntax for
box <expr>
expressions.