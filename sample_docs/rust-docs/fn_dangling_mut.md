dangling_mut in std::ptr - Rust
std
::
ptr
Function
dangling_mut
Copy item path
1.84.0 (const: 1.84.0)
·
Source
pub const fn dangling_mut<T>() ->
*mut T
Expand description
Creates a new pointer that is dangling, but non-null and well-aligned.
This is useful for initializing types which lazily allocate, like
Vec::new
does.
Note that the pointer value may potentially represent a valid pointer to
a
T
, which means this must not be used as a “not yet initialized”
sentinel value. Types that lazily allocate must track initialization by
some other means.