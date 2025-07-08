write_volatile in std::ptr - Rust
std
::
ptr
Function
write_volatile
Copy item path
1.9.0
·
Source
pub unsafe fn write_volatile<T>(dst:
*mut T
, src: T)
Expand description
Performs a volatile write of a memory location with the given value without
reading or dropping the old value.
Volatile operations are intended to act on I/O memory, and are guaranteed
to not be elided or reordered by the compiler across other volatile
operations.
write_volatile
does not drop the contents of
dst
. This is safe, but it
could leak allocations or resources, so care should be taken not to overwrite
an object that should be dropped.
Additionally, it does not drop
src
. Semantically,
src
is moved into the
location pointed to by
dst
.
§
Notes
Rust does not currently have a rigorously and formally defined memory model,
so the precise semantics of what “volatile” means here is subject to change
over time. That being said, the semantics will almost always end up pretty
similar to
C11’s definition of volatile
.
The compiler shouldn’t change the relative order or number of volatile
memory operations. However, volatile memory operations on zero-sized types
(e.g., if a zero-sized type is passed to
write_volatile
) are noops
and may be ignored.
§
Safety
Behavior is undefined if any of the following conditions are violated:
dst
must be
valid
for writes.
dst
must be properly aligned.
Note that even if
T
has size
0
, the pointer must be properly aligned.
Just like in C, whether an operation is volatile has no bearing whatsoever
on questions involving concurrent access from multiple threads. Volatile
accesses behave exactly like non-atomic accesses in that regard. In particular,
a race between a
write_volatile
and any other operation (reading or writing)
on the same location is undefined behavior.
§
Examples
Basic usage:
let
mut
x =
0
;
let
y =
&mut
x
as
*mut
i32;
let
z =
12
;
unsafe
{
    std::ptr::write_volatile(y, z);
assert_eq!
(std::ptr::read_volatile(y),
12
);
}