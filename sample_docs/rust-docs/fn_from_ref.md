from_ref in std::ptr - Rust
std
::
ptr
Function
from_ref
Copy item path
1.76.0 (const: 1.76.0)
·
Source
pub const fn from_ref<T>(r:
&T
) ->
*const T
where
    T: ?
Sized
,
Expand description
Converts a reference to a raw pointer.
For
r: &T
,
from_ref(r)
is equivalent to
r as *const T
(except for the caveat noted below),
but is a bit safer since it will never silently change type or mutability, in particular if the
code is refactored.
The caller must ensure that the pointee outlives the pointer this function returns, or else it
will end up dangling.
The caller must also ensure that the memory the pointer (non-transitively) points to is never
written to (except inside an
UnsafeCell
) using this pointer or any pointer derived from it. If
you need to mutate the pointee, use
from_mut
. Specifically, to turn a mutable reference
m: &mut T
into
*const T
, prefer
from_mut(m).cast_const()
to obtain a pointer that can later be
used for mutation.
§
Interaction with lifetime extension
Note that this has subtle interactions with the rules for lifetime extension of temporaries in
tail expressions. This code is valid, albeit in a non-obvious way:
// The temporary holding the return value of `foo` has its lifetime extended,
// because the surrounding expression involves no function call.
let
p =
&
foo()
as
*const
T;
unsafe
{ p.read() };
Naively replacing the cast with
from_ref
is not valid:
// The temporary holding the return value of `foo` does *not* have its lifetime extended,
// because the surrounding expression involves a function call.
let
p = ptr::from_ref(
&
foo());
unsafe
{ p.read() };
// UB! Reading from a dangling pointer ⚠️
The recommended way to write this code is to avoid relying on lifetime extension
when raw pointers are involved:
let
x = foo();
let
p = ptr::from_ref(
&
x);
unsafe
{ p.read() };