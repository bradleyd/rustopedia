zeroed in std::mem - Rust
std
::
mem
Function
zeroed
Copy item path
1.0.0 (const: 1.75.0)
·
Source
pub const unsafe fn zeroed<T>() -> T
Expand description
Returns the value of type
T
represented by the all-zero byte-pattern.
This means that, for example, the padding byte in
(u8, u16)
is not
necessarily zeroed.
There is no guarantee that an all-zero byte-pattern represents a valid value
of some type
T
. For example, the all-zero byte-pattern is not a valid value
for reference types (
&T
,
&mut T
) and function pointers. Using
zeroed
on such types causes immediate
undefined behavior
because
the Rust
compiler assumes
that there always is a valid value in a variable it
considers initialized.
This has the same effect as
MaybeUninit::zeroed().assume_init()
.
It is useful for FFI sometimes, but should generally be avoided.
§
Examples
Correct usage of this function: initializing an integer with zero.
use
std::mem;
let
x: i32 =
unsafe
{ mem::zeroed() };
assert_eq!
(
0
, x);
Incorrect
usage of this function: initializing a reference with zero.
use
std::mem;
let
_x:
&
i32 =
unsafe
{ mem::zeroed() };
// Undefined behavior!
let
_y:
fn
() =
unsafe
{ mem::zeroed() };
// And again!