Destruct in std::marker - Rust
std
::
marker
Trait
Destruct
Copy item path
Source
pub trait Destruct { }
🔬
This is a nightly-only experimental API. (
const_destruct
#133214
)
Expand description
A marker for types that can be dropped.
This should be used for
~const
bounds,
as non-const bounds will always hold for every type.
Implementors
§