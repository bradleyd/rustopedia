null_mut in std::ptr - Rust
std
::
ptr
Function
null_mut
Copy item path
1.0.0 (const: 1.24.0)
·
Source
pub const fn null_mut<T>() ->
*mut T
where
    T:
Thin
+ ?
Sized
,
Expand description
Creates a null mutable raw pointer.
This function is equivalent to zero-initializing the pointer:
MaybeUninit::<*mut T>::zeroed().assume_init()
.
The resulting pointer has the address 0.
§
Examples
use
std::ptr;
let
p:
*mut
i32 = ptr::null_mut();
assert!
(p.is_null());
assert_eq!
(p
as
usize,
0
);
// this pointer has the address 0