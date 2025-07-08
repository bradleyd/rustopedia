fn_addr_eq in std::ptr - Rust
std
::
ptr
Function
fn_addr_eq
Copy item path
1.85.0
·
Source
pub fn fn_addr_eq<T, U>(f: T, g: U) ->
bool
where
    T:
FnPtr
,
    U:
FnPtr
,
Expand description
Compares the
addresses
of the two function pointers for equality.
This is the same as
f == g
, but using this function makes clear that the potentially
surprising semantics of function pointer comparison are involved.
There are
very few guarantees
about how functions are compiled and they have no intrinsic
“identity”; in particular, this comparison:
May return
true
unexpectedly, in cases where functions are equivalent.
For example, the following program is likely (but not guaranteed) to print
(true, true)
when compiled with optimization:
let
f:
fn
(i32) -> i32 = |x| x;
let
g:
fn
(i32) -> i32 = |x| x +
0
;
// different closure, different body
let
h:
fn
(u32) -> u32 = |x| x +
0
;
// different signature too
dbg!
(std::ptr::fn_addr_eq(f, g), std::ptr::fn_addr_eq(f, h));
// not guaranteed to be equal
May return
false
in any case.
This is particularly likely with generic functions but may happen with any function.
(From an implementation perspective, this is possible because functions may sometimes be
processed more than once by the compiler, resulting in duplicate machine code.)
Despite these false positives and false negatives, this comparison can still be useful.
Specifically, if
T
is the same type as
U
,
T
is a
subtype
of
U
, or
U
is a
subtype
of
T
, and
ptr::fn_addr_eq(f, g)
returns true,
then calling
f
and calling
g
will be equivalent.
§
Examples
use
std::ptr;
fn
a() {
println!
(
"a"
); }
fn
b() {
println!
(
"b"
); }
assert!
(!ptr::fn_addr_eq(a
as fn
(), b
as fn
()));