DiscriminantKind in std::marker - Rust
std
::
marker
Trait
DiscriminantKind
Copy item path
Source
pub trait DiscriminantKind {
    type
Discriminant
:
Clone
+
Copy
+
Debug
+
Eq
+
PartialEq
+
Hash
+
Send
+
Sync
+
Unpin
;
}
🔬
This is a nightly-only experimental API. (
discriminant_kind
)
Expand description
Compiler-internal trait used to indicate the type of enum discriminants.
This trait is automatically implemented for every type and does not add any
guarantees to
mem::Discriminant
. It is
undefined behavior
to transmute
between
DiscriminantKind::Discriminant
and
mem::Discriminant
.
Required Associated Types
§
Source
type
Discriminant
:
Clone
+
Copy
+
Debug
+
Eq
+
PartialEq
+
Hash
+
Send
+
Sync
+
Unpin
🔬
This is a nightly-only experimental API. (
discriminant_kind
)
The type of the discriminant, which must satisfy the trait
bounds required by
mem::Discriminant
.
Implementors
§