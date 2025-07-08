swap_nonoverlapping in std::ptr - Rust
std
::
ptr
Function
swap_nonoverlapping
Copy item path
1.27.0 (const:
unstable
)
·
Source
pub unsafe fn swap_nonoverlapping<T>(x:
*mut T
, y:
*mut T
, count:
usize
)
Expand description
Swaps
count * size_of::<T>()
bytes between the two regions of memory
beginning at
x
and
y
. The two regions must
not
overlap.
The operation is “untyped” in the sense that data may be uninitialized or otherwise violate the
requirements of
T
. The initialization state is preserved exactly.
§
Safety
Behavior is undefined if any of the following conditions are violated:
Both
x
and
y
must be
valid
for both reads and writes of
count * size_of::<T>()
bytes.
Both
x
and
y
must be properly aligned.
The region of memory beginning at
x
with a size of
count * size_of::<T>()
bytes must
not
overlap with the region of memory
beginning at
y
with the same size.
Note that even if the effectively copied size (
count * size_of::<T>()
) is
0
,
the pointers must be properly aligned.
§
Examples
Basic usage:
use
std::ptr;
let
mut
x = [
1
,
2
,
3
,
4
];
let
mut
y = [
7
,
8
,
9
];
unsafe
{
    ptr::swap_nonoverlapping(x.as_mut_ptr(), y.as_mut_ptr(),
2
);
}
assert_eq!
(x, [
7
,
8
,
3
,
4
]);
assert_eq!
(y, [
1
,
2
,
9
]);