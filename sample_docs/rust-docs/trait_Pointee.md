Pointee in std::ptr - Rust
std
::
ptr
Trait
Pointee
Copy item path
Source
pub trait Pointee {
    type
Metadata
:
Debug
+
Copy
+
Send
+
Sync
+
Ord
+
Hash
+
Unpin
+
Freeze
;
}
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Expand description
Provides the pointer metadata type of any pointed-to type.
§
Pointer metadata
Raw pointer types and reference types in Rust can be thought of as made of two parts:
a data pointer that contains the memory address of the value, and some metadata.
For statically-sized types (that implement the
Sized
traits)
as well as for
extern
types,
pointers are said to be “thin”: metadata is zero-sized and its type is
()
.
Pointers to
dynamically-sized types
are said to be “wide” or “fat”,
they have non-zero-sized metadata:
For structs whose last field is a DST, metadata is the metadata for the last field
For the
str
type, metadata is the length in bytes as
usize
For slice types like
[T]
, metadata is the length in items as
usize
For trait objects like
dyn SomeTrait
, metadata is
DynMetadata<Self>
(e.g.
DynMetadata<dyn SomeTrait>
)
In the future, the Rust language may gain new kinds of types
that have different pointer metadata.
§
The
Pointee
trait
The point of this trait is its
Metadata
associated type,
which is
()
or
usize
or
DynMetadata<_>
as described above.
It is automatically implemented for every type.
It can be assumed to be implemented in a generic context, even without a corresponding bound.
§
Usage
Raw pointers can be decomposed into the data pointer and metadata components
with their
to_raw_parts
method.
Alternatively, metadata alone can be extracted with the
metadata
function.
A reference can be passed to
metadata
and implicitly coerced.
A (possibly-wide) pointer can be put back together from its data pointer and metadata
with
from_raw_parts
or
from_raw_parts_mut
.
Required Associated Types
§
Source
type
Metadata
:
Debug
+
Copy
+
Send
+
Sync
+
Ord
+
Hash
+
Unpin
+
Freeze
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
The type for metadata in pointers and references to
Self
.
Implementors
§