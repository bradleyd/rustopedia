swap in std::mem - Rust
std
::
mem
Function
swap
Copy item path
1.0.0 (const: 1.85.0)
·
Source
pub const fn swap<T>(x:
&mut T
, y:
&mut T
)
Expand description
Swaps the values at two mutable locations, without deinitializing either one.
If you want to swap with a default or dummy value, see
take
.
If you want to swap with a passed value, returning the old value, see
replace
.
§
Examples
use
std::mem;
let
mut
x =
5
;
let
mut
y =
42
;

mem::swap(
&mut
x,
&mut
y);
assert_eq!
(
42
, x);
assert_eq!
(
5
, y);