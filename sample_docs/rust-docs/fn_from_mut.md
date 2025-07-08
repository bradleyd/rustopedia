from_mut in std::ptr - Rust
std
::
ptr
Function
from_mut
Copy item path
1.76.0 (const: 1.76.0)
·
Source
pub const fn from_mut<T>(r:
&mut T
) ->
*mut T
where
    T: ?
Sized
,
Expand description
Converts a mutable reference to a raw pointer.
For
r: &mut T
,
from_mut(r)
is equivalent to
r as *mut T
(except for the caveat noted
below), but is a bit safer since it will never silently change type or mutability, in particular
if the code is refactored.
The caller must ensure that the pointee outlives the pointer this function returns, or else it
will end up dangling.
§
Interaction with lifetime extension
Note that this has subtle interactions with the rules for lifetime extension of temporaries in
tail expressions. This code is valid, albeit in a non-obvious way:
// The temporary holding the return value of `foo` has its lifetime extended,
// because the surrounding expression involves no function call.
let
p =
&mut
foo()
as
*mut
T;
unsafe
{ p.write(T::default()) };
Naively replacing the cast with
from_mut
is not valid:
// The temporary holding the return value of `foo` does *not* have its lifetime extended,
// because the surrounding expression involves a function call.
let
p = ptr::from_mut(
&mut
foo());
unsafe
{ p.write(T::default()) };
// UB! Writing to a dangling pointer ⚠️
The recommended way to write this code is to avoid relying on lifetime extension
when raw pointers are involved:
let
mut
x = foo();
let
p = ptr::from_mut(
&mut
x);
unsafe
{ p.write(T::default()) };