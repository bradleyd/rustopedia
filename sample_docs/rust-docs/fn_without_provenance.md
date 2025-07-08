without_provenance in std::ptr - Rust
std
::
ptr
Function
without_provenance
Copy item path
1.84.0 (const: 1.84.0)
·
Source
pub const fn without_provenance<T>(addr:
usize
) ->
*const T
Expand description
Creates a pointer with the given address and no
provenance
.
This is equivalent to
ptr::null().with_addr(addr)
.
Without provenance, this pointer is not associated with any actual allocation. Such a
no-provenance pointer may be used for zero-sized memory accesses (if suitably aligned), but
non-zero-sized memory accesses with a no-provenance pointer are UB. No-provenance pointers are
little more than a
usize
address in disguise.
This is different from
addr as *const T
, which creates a pointer that picks up a previously
exposed provenance. See
with_exposed_provenance
for more details on that operation.
This is a
Strict Provenance
API.