read_volatile in std::ptr - Rust
std
::
ptr
Function
read_volatile
Copy item path
1.9.0
·
Source
pub unsafe fn read_volatile<T>(src:
*const T
) -> T
Expand description
Performs a volatile read of the value from
src
without moving it. This
leaves the memory in
src
unchanged.
Volatile operations are intended to act on I/O memory, and are guaranteed
to not be elided or reordered by the compiler across other volatile
operations.
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
read_volatile
) are noops
and may be ignored.
§
Safety
Behavior is undefined if any of the following conditions are violated:
src
must be
valid
for reads.
src
must be properly aligned.
src
must point to a properly initialized value of type
T
.
Like
read
,
read_volatile
creates a bitwise copy of
T
, regardless of
whether
T
is
Copy
. If
T
is not
Copy
, using both the returned
value and the value at
*src
can
violate memory safety
.
However, storing non-
Copy
types in volatile memory is almost certainly
incorrect.
Note that even if
T
has size
0
, the pointer must be properly aligned.
Just like in C, whether an operation is volatile has no bearing whatsoever
on questions involving concurrent access from multiple threads. Volatile
accesses behave exactly like non-atomic accesses in that regard. In particular,
a race between a
read_volatile
and any write operation to the same location
is undefined behavior.
§
Examples
Basic usage:
let
x =
12
;
let
y =
&
x
as
*const
i32;
unsafe
{
assert_eq!
(std::ptr::read_volatile(y),
12
);
}