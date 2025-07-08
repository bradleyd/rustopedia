define_opaque in std::prelude::v1 - Rust
std
::
prelude
::
v1
Attribute Macro
define_opaque
Copy item path
Source
#[define_opaque]
🔬
This is a nightly-only experimental API. (
type_alias_impl_trait
#63063
)
Expand description
Provide a list of type aliases and other opaque-type-containing type definitions.
This list will be used in the body of the item it is applied to define opaque
types’ hidden types.
Can only be applied to things that have bodies.