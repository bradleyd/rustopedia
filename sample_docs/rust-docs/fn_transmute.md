transmute in std::mem - Rust
std
::
mem
Function
transmute
Copy item path
1.0.0 (const: 1.56.0)
·
Source
pub const unsafe fn transmute<Src, Dst>(src: Src) -> Dst
Expand description
Reinterprets the bits of a value of one type as another type.
Both types must have the same size. Compilation will fail if this is not guaranteed.
transmute
is semantically equivalent to a bitwise move of one type
into another. It copies the bits from the source value into the
destination value, then forgets the original. Note that source and destination
are passed by-value, which means if
Src
or
Dst
contain padding, that padding
is
not
guaranteed to be preserved by
transmute
.
Both the argument and the result must be
valid
at
their given type. Violating this condition leads to
undefined behavior
. The compiler
will generate code
assuming that you, the programmer, ensure that there will never be
undefined behavior
. It is therefore your responsibility to guarantee that every value
passed to
transmute
is valid at both types
Src
and
Dst
. Failing to uphold this condition
may lead to unexpected and unstable compilation results. This makes
transmute
incredibly
unsafe
.
transmute
should be the absolute last resort.
Because
transmute
is a by-value operation, alignment of the
transmuted values
themselves
is not a concern. As with any other function, the compiler already ensures
both
Src
and
Dst
are properly aligned. However, when transmuting values that
point
elsewhere
(such as pointers, references, boxes…), the caller has to ensure proper
alignment of the pointed-to values.
The
nomicon
has additional documentation.
§
Transmutation between pointers and integers
Special care has to be taken when transmuting between pointers and integers, e.g.
transmuting between
*const ()
and
usize
.
Transmuting
pointers to integers
in a
const
context is
undefined behavior
, unless
the pointer was originally created
from
an integer. (That includes this function
specifically, integer-to-pointer casts, and helpers like
dangling
,
but also semantically-equivalent conversions such as punning through
repr(C)
union
fields.) Any attempt to use the resulting value for integer operations will abort
const-evaluation. (And even outside
const
, such transmutation is touching on many
unspecified aspects of the Rust memory model and should be avoided. See below for
alternatives.)
Transmuting
integers to pointers
is a largely unspecified operation. It is likely
not
equivalent to an
as
cast. Doing non-zero-sized memory accesses with a pointer constructed
this way is currently considered undefined behavior.
All this also applies when the integer is nested inside an array, tuple, struct, or enum.
However,
MaybeUninit<usize>
is not considered an integer type for the purpose of this
section. Transmuting
*const ()
to
MaybeUninit<usize>
is fine—but then calling
assume_init()
on that result is considered as completing the pointer-to-integer transmute
and thus runs into the issues discussed above.
In particular, doing a pointer-to-integer-to-pointer roundtrip via
transmute
is
not
a
lossless process. If you want to round-trip a pointer through an integer in a way that you
can get back the original pointer, you need to use
as
casts, or replace the integer type
by
MaybeUninit<$int>
(and never call
assume_init()
). If you are looking for a way to
store data of arbitrary type, also use
MaybeUninit<T>
(that will also handle uninitialized
memory due to padding). If you specifically need to store something that is “either an
integer or a pointer”, use
*mut ()
: integers can be converted to pointers and back without
any loss (via
as
casts or via
transmute
).
§
Examples
There are a few things that
transmute
is really useful for.
Turning a pointer into a function pointer. This is
not
portable to
machines where function pointers and data pointers have different sizes.
fn
foo() -> i32 {
0
}
// Crucially, we `as`-cast to a raw pointer before `transmute`ing to a function pointer.
// This avoids an integer-to-pointer `transmute`, which can be problematic.
// Transmuting between raw pointers and function pointers (i.e., two pointer types) is fine.
let
pointer = foo
as
*const
();
let
function =
unsafe
{
    std::mem::transmute::<
*const
(),
fn
() -> i32>(pointer)
};
assert_eq!
(function(),
0
);
Extending a lifetime, or shortening an invariant lifetime. This is
advanced, very unsafe Rust!
struct
R<
'a
>(
&
'a
i32);
unsafe fn
extend_lifetime<
'b
>(r: R<
'b
>) -> R<
'static
> {
unsafe
{ std::mem::transmute::<R<
'b
>, R<
'static
>>(r) }
}
unsafe fn
shorten_invariant_lifetime<
'b
,
'c
>(r:
&
'b
mut
R<
'static
>)
                                             ->
&
'b
mut
R<
'c
> {
unsafe
{ std::mem::transmute::<
&
'b
mut
R<
'static
>,
&
'b
mut
R<
'c
>>(r) }
}
§
Alternatives
Don’t despair: many uses of
transmute
can be achieved through other means.
Below are common applications of
transmute
which can be replaced with safer
constructs.
Turning raw bytes (
[u8; SZ]
) into
u32
,
f64
, etc.:
let
raw_bytes = [
0x78
,
0x56
,
0x34
,
0x12
];
let
num =
unsafe
{
    std::mem::transmute::<[u8;
4
], u32>(raw_bytes)
};
// use `u32::from_ne_bytes` instead
let
num = u32::from_ne_bytes(raw_bytes);
// or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
let
num = u32::from_le_bytes(raw_bytes);
assert_eq!
(num,
0x12345678
);
let
num = u32::from_be_bytes(raw_bytes);
assert_eq!
(num,
0x78563412
);
Turning a pointer into a
usize
:
let
ptr =
&
0
;
let
ptr_num_transmute =
unsafe
{
    std::mem::transmute::<
&
i32, usize>(ptr)
};
// Use an `as` cast instead
let
ptr_num_cast = ptr
as
*const
i32
as
usize;
Note that using
transmute
to turn a pointer to a
usize
is (as noted above)
undefined
behavior
in
const
contexts. Also outside of consts, this operation might not behave
as expected – this is touching on many unspecified aspects of the Rust memory model.
Depending on what the code is doing, the following alternatives are preferable to
pointer-to-integer transmutation:
If the code just wants to store data of arbitrary type in some buffer and needs to pick a
type for that buffer, it can use
MaybeUninit
.
If the code actually wants to work on the address the pointer points to, it can use
as
casts or
ptr.addr()
.
Turning a
*mut T
into a
&mut T
:
let
ptr:
*mut
i32 =
&mut
0
;
let
ref_transmuted =
unsafe
{
    std::mem::transmute::<
*mut
i32,
&mut
i32>(ptr)
};
// Use a reborrow instead
let
ref_casted =
unsafe
{
&mut *
ptr };
Turning a
&mut T
into a
&mut U
:
let
ptr =
&mut
0
;
let
val_transmuted =
unsafe
{
    std::mem::transmute::<
&mut
i32,
&mut
u32>(ptr)
};
// Now, put together `as` and reborrowing - note the chaining of `as`
// `as` is not transitive
let
val_casts =
unsafe
{
&mut *
(ptr
as
*mut
i32
as
*mut
u32) };
Turning a
&str
into a
&[u8]
:
// this is not a good way to do this.
let
slice =
unsafe
{ std::mem::transmute::<
&
str,
&
[u8]>(
"Rust"
) };
assert_eq!
(slice,
&
[
82
,
117
,
115
,
116
]);
// You could use `str::as_bytes`
let
slice =
"Rust"
.as_bytes();
assert_eq!
(slice,
&
[
82
,
117
,
115
,
116
]);
// Or, just use a byte string, if you have control over the string
// literal
assert_eq!
(
b"Rust"
,
&
[
82
,
117
,
115
,
116
]);
Turning a
Vec<&T>
into a
Vec<Option<&T>>
.
To transmute the inner type of the contents of a container, you must make sure to not
violate any of the container’s invariants. For
Vec
, this means that both the size
and alignment
of the inner types have to match. Other containers might rely on the
size of the type, alignment, or even the
TypeId
, in which case transmuting wouldn’t
be possible at all without violating the container invariants.
let
store = [
0
,
1
,
2
,
3
];
let
v_orig = store.iter().collect::<Vec<
&
i32>>();
// clone the vector as we will reuse them later
let
v_clone = v_orig.clone();
// Using transmute: this relies on the unspecified data layout of `Vec`, which is a
// bad idea and could cause Undefined Behavior.
// However, it is no-copy.
let
v_transmuted =
unsafe
{
    std::mem::transmute::<Vec<
&
i32>, Vec<
Option
<
&
i32>>>(v_clone)
};
let
v_clone = v_orig.clone();
// This is the suggested, safe way.
// It may copy the entire vector into a new one though, but also may not.
let
v_collected = v_clone.into_iter()
                         .map(
Some
)
                         .collect::<Vec<
Option
<
&
i32>>>();
let
v_clone = v_orig.clone();
// This is the proper no-copy, unsafe way of "transmuting" a `Vec`, without relying on the
// data layout. Instead of literally calling `transmute`, we perform a pointer cast, but
// in terms of converting the original inner type (`&i32`) to the new one (`Option<&i32>`),
// this has all the same caveats. Besides the information provided above, also consult the
// [`from_raw_parts`] documentation.
let
v_from_raw =
unsafe
{
// Ensure the original vector is not dropped.
let
mut
v_clone = std::mem::ManuallyDrop::new(v_clone);
    Vec::from_raw_parts(v_clone.as_mut_ptr()
as
*mut
Option
<
&
i32>,
                        v_clone.len(),
                        v_clone.capacity())
};
Implementing
split_at_mut
:
use
std::{slice, mem};
// There are multiple ways to do this, and there are multiple problems
// with the following (transmute) way.
fn
split_at_mut_transmute<T>(slice:
&mut
[T], mid: usize)
                             -> (
&mut
[T],
&mut
[T]) {
let
len = slice.len();
assert!
(mid <= len);
unsafe
{
let
slice2 = mem::transmute::<
&mut
[T],
&mut
[T]>(slice);
// first: transmute is not type safe; all it checks is that T and
        // U are of the same size. Second, right here, you have two
        // mutable references pointing to the same memory.
(
&mut
slice[
0
..mid],
&mut
slice2[mid..len])
    }
}
// This gets rid of the type safety problems; `&mut *` will *only* give
// you a `&mut T` from a `&mut T` or `*mut T`.
fn
split_at_mut_casts<T>(slice:
&mut
[T], mid: usize)
                         -> (
&mut
[T],
&mut
[T]) {
let
len = slice.len();
assert!
(mid <= len);
unsafe
{
let
slice2 =
&mut *
(slice
as
*mut
[T]);
// however, you still have two mutable references pointing to
        // the same memory.
(
&mut
slice[
0
..mid],
&mut
slice2[mid..len])
    }
}
// This is how the standard library does it. This is the best method, if
// you need to do something like this
fn
split_at_stdlib<T>(slice:
&mut
[T], mid: usize)
                      -> (
&mut
[T],
&mut
[T]) {
let
len = slice.len();
assert!
(mid <= len);
unsafe
{
let
ptr = slice.as_mut_ptr();
// This now has three mutable references pointing at the same
        // memory. `slice`, the rvalue ret.0, and the rvalue ret.1.
        // `slice` is never used after `let ptr = ...`, and so one can
        // treat it as "dead", and therefore, you only have two real
        // mutable slices.
(slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}