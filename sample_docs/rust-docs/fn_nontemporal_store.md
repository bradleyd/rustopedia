nontemporal_store in std::intrinsics - Rust
std
::
intrinsics
Function
nontemporal_store
Copy item path
Source
pub unsafe fn nontemporal_store<T>(ptr:
*mut T
, val: T)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Emits a
nontemporal
store, which gives a hint to the CPU that the data should not be held
in cache. Except for performance, this is fully equivalent to
ptr.write(val)
.
Not all architectures provide such an operation. For instance, x86 does not: while
MOVNT
exists, that operation is
not
equivalent to
ptr.write(val)
(
MOVNT
writes can be reordered
in ways that are not allowed for regular writes).