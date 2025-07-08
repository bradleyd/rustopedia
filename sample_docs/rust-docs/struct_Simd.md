Simd in std::simd::prelude - Rust
std
::
simd
::
prelude
Struct
Simd
Copy item path
Source
#[repr(simd, packed(1))]
pub struct Simd<T, const N:
usize
>(
/* private fields */
)
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD vector with the shape of
[T; N]
but the operations of
T
.
Simd<T, N>
supports the operators (+, *, etc.) that
T
does in “elementwise” fashion.
These take the element at each index from the left-hand side and right-hand side,
perform the operation, then return the result in the same index in a vector of equal size.
However,
Simd
differs from normal iteration and normal arrays:
Simd<T, N>
executes
N
operations in a single step with no
break
s
Simd<T, N>
can have an alignment greater than
T
, for better mechanical sympathy
By always imposing these constraints on
Simd
, it is easier to compile elementwise operations
into machine instructions that can themselves be executed in parallel.
let
a: [i32;
4
] = [-
2
,
0
,
2
,
4
];
let
b = [
10
,
9
,
8
,
7
];
let
sum = array::from_fn(|i| a[i] + b[i]);
let
prod = array::from_fn(|i| a[i] * b[i]);
// `Simd<T, N>` implements `From<[T; N]>`
let
(v, w) = (Simd::from(a), Simd::from(b));
// Which means arrays implement `Into<Simd<T, N>>`.
assert_eq!
(v + w, sum.into());
assert_eq!
(v * w, prod.into());
Simd
with integer elements treats operators as wrapping, as if
T
was
Wrapping<T>
.
Thus,
Simd
does not implement
wrapping_add
, because that is the default behavior.
This means there is no warning on overflows, even in “debug” builds.
For most applications where
Simd
is appropriate, it is “not a bug” to wrap,
and even “debug builds” are unlikely to tolerate the loss of performance.
You may want to consider using explicitly checked arithmetic if such is required.
Division by zero on integers still causes a panic, so
you may want to consider using
f32
or
f64
if that is unacceptable.
§
Layout
Simd<T, N>
has a layout similar to
[T; N]
(identical “shapes”), with a greater alignment.
[T; N]
is aligned to
T
, but
Simd<T, N>
will have an alignment based on both
T
and
N
.
Thus it is sound to
transmute
Simd<T, N>
to
[T; N]
and should optimize to “zero cost”,
but the reverse transmutation may require a copy the compiler cannot simply elide.
§
ABI “Features”
Due to Rust’s safety guarantees,
Simd<T, N>
is currently passed and returned via memory,
not SIMD registers, except as an optimization. Using
#[inline]
on functions that accept
Simd<T, N>
or return it is recommended, at the cost of code generation time, as
inlining SIMD-using functions can omit a large function prolog or epilog and thus
improve both speed and code size. The need for this may be corrected in the future.
Using
#[inline(always)]
still requires additional care.
§
Safe SIMD with Unsafe Rust
Operations with
Simd
are typically safe, but there are many reasons to want to combine SIMD with
unsafe
code.
Care must be taken to respect differences between
Simd
and other types it may be transformed into or derived from.
In particular, the layout of
Simd<T, N>
may be similar to
[T; N]
, and may allow some transmutations,
but references to
[T; N]
are not interchangeable with those to
Simd<T, N>
.
Thus, when using
unsafe
Rust to read and write
Simd<T, N>
through
raw pointers
, it is a good idea to first try with
read_unaligned
and
write_unaligned
. This is because:
read
and
write
require full alignment (in this case,
Simd<T, N>
’s alignment)
Simd<T, N>
is often read from or written to
[T]
and other types aligned to
T
combining these actions violates the
unsafe
contract and explodes the program into
a puff of
undefined behavior
the compiler can implicitly adjust layouts to make unaligned reads or writes fully aligned
if it sees the optimization
most contemporary processors with “aligned” and “unaligned” read and write instructions
exhibit no performance difference if the “unaligned” variant is aligned at runtime
Less obligations mean unaligned reads and writes are less likely to make the program unsound,
and may be just as fast as stricter alternatives.
When trying to guarantee alignment,
[T]::as_simd
is an option for
converting
[T]
to
[Simd<T, N>]
, and allows soundly operating on an aligned SIMD body,
but it may cost more time when handling the scalar head and tail.
If these are not enough, it is most ideal to design data structures to be already aligned
to
align_of::<Simd<T, N>>()
before using
unsafe
Rust to read or write.
Other ways to compensate for these facts, like materializing
Simd
to or from an array first,
are handled by safe methods like
Simd::from_array
and
Simd::from_slice
.
Implementations
§
Source
§
impl<T, const N:
usize
>
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
pub fn
reverse
(self) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverse the order of the elements in the vector.
Source
pub fn
rotate_elements_left
<const OFFSET:
usize
>(self) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rotates the vector such that the first
OFFSET
elements of the slice move to the end
while the last
self.len() - OFFSET
elements move to the front. After calling
rotate_elements_left
,
the element previously at index
OFFSET
will become the first element in the slice.
let
a = Simd::from_array([
0
,
1
,
2
,
3
]);
let
x = a.rotate_elements_left::<
3
>();
assert_eq!
(x.to_array(), [
3
,
0
,
1
,
2
]);
let
y = a.rotate_elements_left::<
7
>();
assert_eq!
(y.to_array(), [
3
,
0
,
1
,
2
]);
Source
pub fn
rotate_elements_right
<const OFFSET:
usize
>(self) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rotates the vector such that the first
self.len() - OFFSET
elements of the vector move to
the end while the last
OFFSET
elements move to the front. After calling
rotate_elements_right
,
the element previously at index
self.len() - OFFSET
will become the first element in the slice.
let
a = Simd::from_array([
0
,
1
,
2
,
3
]);
let
x = a.rotate_elements_right::<
3
>();
assert_eq!
(x.to_array(), [
1
,
2
,
3
,
0
]);
let
y = a.rotate_elements_right::<
7
>();
assert_eq!
(y.to_array(), [
1
,
2
,
3
,
0
]);
Source
pub fn
shift_elements_left
<const OFFSET:
usize
>(self, padding: T) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Shifts the vector elements to the left by
OFFSET
, filling in with
padding
from the right.
let
a = Simd::from_array([
0
,
1
,
2
,
3
]);
let
x = a.shift_elements_left::<
3
>(
255
);
assert_eq!
(x.to_array(), [
3
,
255
,
255
,
255
]);
let
y = a.shift_elements_left::<
7
>(
255
);
assert_eq!
(y.to_array(), [
255
,
255
,
255
,
255
]);
Source
pub fn
shift_elements_right
<const OFFSET:
usize
>(self, padding: T) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Shifts the vector elements to the right by
OFFSET
, filling in with
padding
from the left.
let
a = Simd::from_array([
0
,
1
,
2
,
3
]);
let
x = a.shift_elements_right::<
3
>(
255
);
assert_eq!
(x.to_array(), [
255
,
255
,
255
,
0
]);
let
y = a.shift_elements_right::<
7
>(
255
);
assert_eq!
(y.to_array(), [
255
,
255
,
255
,
255
]);
Source
pub fn
interleave
(self, other:
Simd
<T, N>) -> (
Simd
<T, N>,
Simd
<T, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Interleave two vectors.
The resulting vectors contain elements taken alternatively from
self
and
other
, first
filling the first result, and then the second.
The reverse of this operation is
Simd::deinterleave
.
let
a = Simd::from_array([
0
,
1
,
2
,
3
]);
let
b = Simd::from_array([
4
,
5
,
6
,
7
]);
let
(x, y) = a.interleave(b);
assert_eq!
(x.to_array(), [
0
,
4
,
1
,
5
]);
assert_eq!
(y.to_array(), [
2
,
6
,
3
,
7
]);
Source
pub fn
deinterleave
(self, other:
Simd
<T, N>) -> (
Simd
<T, N>,
Simd
<T, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Deinterleave two vectors.
The first result takes every other element of
self
and then
other
, starting with
the first element.
The second result takes every other element of
self
and then
other
, starting with
the second element.
The reverse of this operation is
Simd::interleave
.
let
a = Simd::from_array([
0
,
4
,
1
,
5
]);
let
b = Simd::from_array([
2
,
6
,
3
,
7
]);
let
(x, y) = a.deinterleave(b);
assert_eq!
(x.to_array(), [
0
,
1
,
2
,
3
]);
assert_eq!
(y.to_array(), [
4
,
5
,
6
,
7
]);
Source
pub fn
resize
<const M:
usize
>(self, value: T) ->
Simd
<T, M>
where
LaneCount
<M>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Resize a vector.
If
M
>
N
, extends the length of a vector, setting the new elements to
value
.
If
M
<
N
, truncates the vector to the first
M
elements.
let
x = u32x4::from_array([
0
,
1
,
2
,
3
]);
assert_eq!
(x.resize::<
8
>(
9
).to_array(), [
0
,
1
,
2
,
3
,
9
,
9
,
9
,
9
]);
assert_eq!
(x.resize::<
2
>(
9
).to_array(), [
0
,
1
]);
Source
pub fn
extract
<const START:
usize
, const LEN:
usize
>(self) ->
Simd
<T, LEN>
where
LaneCount
<LEN>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Extract a vector from another vector.
let
x = u32x4::from_array([
0
,
1
,
2
,
3
]);
assert_eq!
(x.extract::<
1
,
2
>().to_array(), [
1
,
2
]);
Source
§
impl<const N:
usize
>
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
pub fn
swizzle_dyn
(self, idxs:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Swizzle a vector of bytes according to the index vector.
Indices within range select the appropriate byte.
Indices “out of bounds” instead select 0.
Note that the current implementation is selected during build-time
of the standard library, so
cargo build -Zbuild-std
may be necessary
to unlock better performance, especially for larger vectors.
A planned compiler improvement will enable using
#[target_feature]
instead.
Source
§
impl<T, const N:
usize
>
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
pub const
LEN
:
usize
= N
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Number of elements in this vector.
Source
pub const fn
len
(&self) ->
usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of elements in this SIMD vector.
§
Examples
let
v = u32x4::splat(
0
);
assert_eq!
(v.len(),
4
);
Source
pub const fn
splat
(value: T) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Constructs a new SIMD vector with all elements set to the given value.
§
Examples
let
v = u32x4::splat(
8
);
assert_eq!
(v.as_array(),
&
[
8
,
8
,
8
,
8
]);
Source
pub const fn
as_array
(&self) -> &
[T; N]
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns an array reference containing the entire SIMD vector.
§
Examples
let
v: u64x4 = Simd::from_array([
0
,
1
,
2
,
3
]);
assert_eq!
(v.as_array(),
&
[
0
,
1
,
2
,
3
]);
Source
pub fn
as_mut_array
(&mut self) -> &mut
[T; N]
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns a mutable array reference containing the entire SIMD vector.
Source
pub const fn
from_array
(array:
[T; N]
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an array to a SIMD vector.
Source
pub const fn
to_array
(self) ->
[T; N]
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts a SIMD vector to an array.
Source
pub const fn
from_slice
(slice: &
[T]
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts a slice to a SIMD vector containing
slice[..N]
.
§
Panics
Panics if the slice’s length is less than the vector’s
Simd::N
.
Use
load_or_default
for an alternative that does not panic.
§
Example
let
source =
vec!
[
1
,
2
,
3
,
4
,
5
,
6
];
let
v = u32x4::from_slice(
&
source);
assert_eq!
(v.as_array(),
&
[
1
,
2
,
3
,
4
]);
Source
pub fn
copy_to_slice
(self, slice: &mut
[T]
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Writes a SIMD vector to the first
N
elements of a slice.
§
Panics
Panics if the slice’s length is less than the vector’s
Simd::N
.
§
Example
let
mut
dest =
vec!
[
0
;
6
];
let
v = u32x4::from_array([
1
,
2
,
3
,
4
]);
v.copy_to_slice(
&mut
dest);
assert_eq!
(
&
dest,
&
[
1
,
2
,
3
,
4
,
0
,
0
]);
Source
pub fn
load_or_default
(slice: &
[T]
) ->
Simd
<T, N>
where
    T:
Default
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements from
slice
. Elements are read so long as they’re in-bounds for
the
slice
. Otherwise, the default value for the element type is returned.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
];
let
result = Simd::<i32,
4
>::load_or_default(
&
vec);
assert_eq!
(result, Simd::from_array([
10
,
11
,
0
,
0
]));
Source
pub fn
load_or
(slice: &
[T]
, or:
Simd
<T, N>) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements from
slice
. Elements are read so long as they’re in-bounds for
the
slice
. Otherwise, the corresponding value from
or
is passed through.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
];
let
or = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
result = Simd::load_or(
&
vec, or);
assert_eq!
(result, Simd::from_array([
10
,
11
, -
3
, -
2
]));
Source
pub fn
load_select_or_default
(
    slice: &
[T]
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
) ->
Simd
<T, N>
where
    T:
Default
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements from
slice
. Each element is read from memory if its
corresponding element in
enable
is
true
.
When the element is disabled or out of bounds for the slice, that memory location
is not accessed and the corresponding value from
or
is passed through.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
enable = Mask::from_array([
true
,
true
,
false
,
true
]);
let
or = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
result = Simd::load_select(
&
vec, enable, or);
assert_eq!
(result, Simd::from_array([
10
,
11
, -
3
,
13
]));
Source
pub fn
load_select
(
    slice: &
[T]
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements from
slice
. Each element is read from memory if its
corresponding element in
enable
is
true
.
When the element is disabled or out of bounds for the slice, that memory location
is not accessed and the corresponding value from
or
is passed through.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
enable = Mask::from_array([
true
,
true
,
false
,
true
]);
let
or = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
result = Simd::load_select(
&
vec, enable, or);
assert_eq!
(result, Simd::from_array([
10
,
11
, -
3
,
13
]));
Source
pub unsafe fn
load_select_unchecked
(
    slice: &
[T]
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements from
slice
. Each element is read from memory if its
corresponding element in
enable
is
true
.
When the element is disabled, that memory location is not accessed and the corresponding
value from
or
is passed through.
§
Safety
Enabled loads must not exceed the length of
slice
.
Source
pub unsafe fn
load_select_ptr
(
    ptr:
*const T
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads contiguous elements starting at
ptr
. Each element is read from memory if its
corresponding element in
enable
is
true
.
When the element is disabled, that memory location is not accessed and the corresponding
value from
or
is passed through.
§
Safety
Enabled
ptr
elements must be safe to read as if by
std::ptr::read
.
Source
pub fn
gather_or
(
    slice: &
[T]
,
    idxs:
Simd
<
usize
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads from potentially discontiguous indices in
slice
to construct a SIMD vector.
If an index is out-of-bounds, the element is instead selected from the
or
vector.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
5
]);
// Note the index that is out-of-bounds
let
alt = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
result = Simd::gather_or(
&
vec, idxs, alt);
assert_eq!
(result, Simd::from_array([-
5
,
13
,
10
,
15
]));
Source
pub fn
gather_or_default
(slice: &
[T]
, idxs:
Simd
<
usize
, N>) ->
Simd
<T, N>
where
    T:
Default
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads from indices in
slice
to construct a SIMD vector.
If an index is out-of-bounds, the element is set to the default given by
T: Default
.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
5
]);
// Note the index that is out-of-bounds
let
result = Simd::gather_or_default(
&
vec, idxs);
assert_eq!
(result, Simd::from_array([
0
,
13
,
10
,
15
]));
Source
pub fn
gather_select
(
    slice: &
[T]
,
    enable:
Mask
<
isize
, N>,
    idxs:
Simd
<
usize
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads from indices in
slice
to construct a SIMD vector.
The mask
enable
s all
true
indices and disables all
false
indices.
If an index is disabled or is out-of-bounds, the element is selected from the
or
vector.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
5
]);
// Includes an out-of-bounds index
let
alt = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
enable = Mask::from_array([
true
,
true
,
true
,
false
]);
// Includes a masked element
let
result = Simd::gather_select(
&
vec, enable, idxs, alt);
assert_eq!
(result, Simd::from_array([-
5
,
13
,
10
, -
2
]));
Source
pub unsafe fn
gather_select_unchecked
(
    slice: &
[T]
,
    enable:
Mask
<
isize
, N>,
    idxs:
Simd
<
usize
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads from indices in
slice
to construct a SIMD vector.
The mask
enable
s all
true
indices and disables all
false
indices.
If an index is disabled, the element is selected from the
or
vector.
§
Safety
Calling this function with an
enable
d out-of-bounds index is
undefined behavior
even if the resulting value is not used.
§
Examples
let
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
5
]);
// Includes an out-of-bounds index
let
alt = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
enable = Mask::from_array([
true
,
true
,
true
,
false
]);
// Includes a masked element
// If this mask was used to gather, it would be unsound. Let's fix that.
let
enable = enable & idxs.simd_lt(Simd::splat(vec.len()));
// The out-of-bounds index has been masked, so it's safe to gather now.
let
result =
unsafe
{ Simd::gather_select_unchecked(
&
vec, enable, idxs, alt) };
assert_eq!
(result, Simd::from_array([-
5
,
13
,
10
, -
2
]));
Source
pub unsafe fn
gather_ptr
(source:
Simd
<
*const T
, N>) ->
Simd
<T, N>
where
    T:
Default
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reads elementwise from pointers into a SIMD vector.
§
Safety
Each read must satisfy the same conditions as
core::ptr::read
.
§
Example
let
values = [
6
,
2
,
4
,
9
];
let
offsets = Simd::from_array([
1
,
0
,
0
,
3
]);
let
source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
let
gathered =
unsafe
{ Simd::gather_ptr(source) };
assert_eq!
(gathered, Simd::from_array([
2
,
6
,
6
,
9
]));
Source
pub unsafe fn
gather_select_ptr
(
    source:
Simd
<
*const T
, N>,
    enable:
Mask
<
isize
, N>,
    or:
Simd
<T, N>,
) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Conditionally read elementwise from pointers into a SIMD vector.
The mask
enable
s all
true
pointers and disables all
false
pointers.
If a pointer is disabled, the element is selected from the
or
vector,
and no read is performed.
§
Safety
Enabled elements must satisfy the same conditions as
core::ptr::read
.
§
Example
let
values = [
6
,
2
,
4
,
9
];
let
enable = Mask::from_array([
true
,
true
,
false
,
true
]);
let
offsets = Simd::from_array([
1
,
0
,
0
,
3
]);
let
source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
let
gathered =
unsafe
{ Simd::gather_select_ptr(source, enable, Simd::splat(
0
)) };
assert_eq!
(gathered, Simd::from_array([
2
,
6
,
0
,
9
]));
Source
pub fn
store_select
(
    self,
    slice: &mut
[T]
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Conditionally write contiguous elements to
slice
. The
enable
mask controls
which elements are written, as long as they’re in-bounds of the
slice
.
If the element is disabled or out of bounds, no memory access to that location
is made.
§
Examples
let
mut
arr = [
0i32
;
4
];
let
write = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
enable = Mask::from_array([
false
,
true
,
true
,
true
]);

write.store_select(
&mut
arr[..
3
], enable);
assert_eq!
(arr, [
0
, -
4
, -
3
,
0
]);
Source
pub unsafe fn
store_select_unchecked
(
    self,
    slice: &mut
[T]
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Conditionally write contiguous elements to
slice
. The
enable
mask controls
which elements are written.
§
Safety
Every enabled element must be in bounds for the
slice
.
§
Examples
let
mut
arr = [
0i32
;
4
];
let
write = Simd::from_array([-
5
, -
4
, -
3
, -
2
]);
let
enable = Mask::from_array([
false
,
true
,
true
,
true
]);
unsafe
{ write.store_select_unchecked(
&mut
arr, enable) };
assert_eq!
(arr, [
0
, -
4
, -
3
, -
2
]);
Source
pub unsafe fn
store_select_ptr
(
    self,
    ptr:
*mut T
,
    enable:
Mask
<<T as
SimdElement
>::
Mask
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Conditionally write contiguous elements starting from
ptr
.
The
enable
mask controls which elements are written.
When disabled, the memory location corresponding to that element is not accessed.
§
Safety
Memory addresses for element are calculated
pointer::wrapping_offset
and
each enabled element must satisfy the same conditions as
core::ptr::write
.
Source
pub fn
scatter
(self, slice: &mut
[T]
, idxs:
Simd
<
usize
, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Writes the values in a SIMD vector to potentially discontiguous indices in
slice
.
If an index is out-of-bounds, the write is suppressed without panicking.
If two elements in the scattered vector would write to the same index
only the last element is guaranteed to actually be written.
§
Examples
let
mut
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
0
]);
// Note the duplicate index.
let
vals = Simd::from_array([-
27
,
82
, -
41
,
124
]);

vals.scatter(
&mut
vec, idxs);
// two logical writes means the last wins.
assert_eq!
(vec,
vec!
[
124
,
11
,
12
,
82
,
14
,
15
,
16
,
17
,
18
]);
Source
pub fn
scatter_select
(
    self,
    slice: &mut
[T]
,
    enable:
Mask
<
isize
, N>,
    idxs:
Simd
<
usize
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Writes values from a SIMD vector to multiple potentially discontiguous indices in
slice
.
The mask
enable
s all
true
indices and disables all
false
indices.
If an enabled index is out-of-bounds, the write is suppressed without panicking.
If two enabled elements in the scattered vector would write to the same index,
only the last element is guaranteed to actually be written.
§
Examples
let
mut
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
0
]);
// Includes an out-of-bounds index
let
vals = Simd::from_array([-
27
,
82
, -
41
,
124
]);
let
enable = Mask::from_array([
true
,
true
,
true
,
false
]);
// Includes a masked element
vals.scatter_select(
&mut
vec, enable, idxs);
// The last write is masked, thus omitted.
assert_eq!
(vec,
vec!
[-
41
,
11
,
12
,
82
,
14
,
15
,
16
,
17
,
18
]);
Source
pub unsafe fn
scatter_select_unchecked
(
    self,
    slice: &mut
[T]
,
    enable:
Mask
<
isize
, N>,
    idxs:
Simd
<
usize
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Writes values from a SIMD vector to multiple potentially discontiguous indices in
slice
.
The mask
enable
s all
true
indices and disables all
false
indices.
If two enabled elements in the scattered vector would write to the same index,
only the last element is guaranteed to actually be written.
§
Safety
Calling this function with an enabled out-of-bounds index is
undefined behavior
,
and may lead to memory corruption.
§
Examples
let
mut
vec: Vec<i32> =
vec!
[
10
,
11
,
12
,
13
,
14
,
15
,
16
,
17
,
18
];
let
idxs = Simd::from_array([
9
,
3
,
0
,
0
]);
let
vals = Simd::from_array([-
27
,
82
, -
41
,
124
]);
let
enable = Mask::from_array([
true
,
true
,
true
,
false
]);
// Masks the final index
// If this mask was used to scatter, it would be unsound. Let's fix that.
let
enable = enable & idxs.simd_lt(Simd::splat(vec.len()));
// We have masked the OOB index, so it's safe to scatter now.
unsafe
{ vals.scatter_select_unchecked(
&mut
vec, enable, idxs); }
// The second write to index 0 was masked, thus omitted.
assert_eq!
(vec,
vec!
[-
41
,
11
,
12
,
82
,
14
,
15
,
16
,
17
,
18
]);
Source
pub unsafe fn
scatter_ptr
(self, dest:
Simd
<
*mut T
, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Writes pointers elementwise into a SIMD vector.
§
Safety
Each write must satisfy the same conditions as
core::ptr::write
.
§
Example
let
mut
values = [
0
;
4
];
let
offset = Simd::from_array([
3
,
2
,
1
,
0
]);
let
ptrs = Simd::splat(values.as_mut_ptr()).wrapping_add(offset);
unsafe
{ Simd::from_array([
6
,
3
,
5
,
7
]).scatter_ptr(ptrs); }
assert_eq!
(values, [
7
,
5
,
3
,
6
]);
Source
pub unsafe fn
scatter_select_ptr
(
    self,
    dest:
Simd
<
*mut T
, N>,
    enable:
Mask
<
isize
, N>,
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Conditionally write pointers elementwise into a SIMD vector.
The mask
enable
s all
true
pointers and disables all
false
pointers.
If a pointer is disabled, the write to its pointee is skipped.
§
Safety
Enabled pointers must satisfy the same conditions as
core::ptr::write
.
§
Example
let
mut
values = [
0
;
4
];
let
offset = Simd::from_array([
3
,
2
,
1
,
0
]);
let
ptrs = Simd::splat(values.as_mut_ptr()).wrapping_add(offset);
let
enable = Mask::from_array([
true
,
true
,
false
,
false
]);
unsafe
{ Simd::from_array([
6
,
3
,
5
,
7
]).scatter_select_ptr(ptrs, enable); }
assert_eq!
(values, [
0
,
0
,
3
,
6
]);
Trait Implementations
§
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Add
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Add
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Add
<&'rhs
Simd
<T, N>>>::
Output
Performs the
+
operation.
Read more
Source
§
impl<T, const N:
usize
>
Add
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Add
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Add
<&
Simd
<T, N>>>::
Output
Performs the
+
operation.
Read more
Source
§
impl<T, const N:
usize
>
Add
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Add
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Add
<
Simd
<T, N>>>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<const N:
usize
>
Add
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Add
>::
Output
Performs the
+
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
AddAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Add
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
add_assign
(&mut self, rhs: U)
Performs the
+=
operation.
Read more
Source
§
impl<T, const N:
usize
>
AsMut
<
[T]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
as_mut
(&mut self) -> &mut
[T]
Converts this type into a mutable reference of the (usually inferred) input type.
Source
§
impl<T, const N:
usize
>
AsMut
<
[T; N]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
as_mut
(&mut self) -> &mut
[T; N]
Converts this type into a mutable reference of the (usually inferred) input type.
Source
§
impl<T, const N:
usize
>
AsRef
<
[T]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
as_ref
(&self) -> &
[T]
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl<T, const N:
usize
>
AsRef
<
[T; N]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
as_ref
(&self) -> &
[T; N]
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
BitAnd
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitAnd
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
BitAnd
<&'rhs
Simd
<T, N>>>::
Output
Performs the
&
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAnd
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitAnd
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
BitAnd
<&
Simd
<T, N>>>::
Output
Performs the
&
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAnd
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitAnd
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
BitAnd
<
Simd
<T, N>>>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<const N:
usize
>
BitAnd
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
BitAnd
>::
Output
Performs the
&
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
BitAndAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
BitAnd
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitand_assign
(&mut self, rhs: U)
Performs the
&=
operation.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
BitOr
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitOr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
BitOr
<&'rhs
Simd
<T, N>>>::
Output
Performs the
|
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOr
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitOr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
BitOr
<&
Simd
<T, N>>>::
Output
Performs the
|
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOr
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitOr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
BitOr
<
Simd
<T, N>>>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<const N:
usize
>
BitOr
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
BitOr
>::
Output
Performs the
|
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
BitOrAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
BitOr
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitor_assign
(&mut self, rhs: U)
Performs the
|=
operation.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
BitXor
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitXor
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
BitXor
<&'rhs
Simd
<T, N>>>::
Output
Performs the
^
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXor
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitXor
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
BitXor
<&
Simd
<T, N>>>::
Output
Performs the
^
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXor
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
BitXor
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
BitXor
<
Simd
<T, N>>>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<const N:
usize
>
BitXor
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
BitXor
>::
Output
Performs the
^
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
BitXorAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
BitXor
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
bitxor_assign
(&mut self, rhs: U)
Performs the
^=
operation.
Read more
Source
§
impl<T, const N:
usize
>
Clone
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
clone
(&self) ->
Simd
<T, N>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
Source
§
impl<T, const N:
usize
>
Debug
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Debug
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
A
Simd<T, N>
has a debug format like the one for
[T]
:
let
floats = Simd::<f32,
4
>::splat(-
1.0
);
assert_eq!
(
format!
(
"{:?}"
, [-
1.0
;
4
]),
format!
(
"{:?}"
, floats));
Source
§
impl<T, const N:
usize
>
Default
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Default
,
Source
§
fn
default
() ->
Simd
<T, N>
Returns the “default value” for a type.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Div
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Div
<&'rhs
Simd
<T, N>>>::
Output
Performs the
/
operation.
Read more
Source
§
impl<T, const N:
usize
>
Div
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Div
<&
Simd
<T, N>>>::
Output
Performs the
/
operation.
Read more
Source
§
impl<T, const N:
usize
>
Div
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Div
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Div
<
Simd
<T, N>>>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<const N:
usize
>
Div
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Div
>::
Output
Performs the
/
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
DivAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Div
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
div_assign
(&mut self, rhs: U)
Performs the
/=
operation.
Read more
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
from
(array:
[T; N]
) ->
Simd
<T, N>
Converts to this type from the input type.
Source
§
impl<T, const N:
usize
>
From
<Mask<T, N>> for
Simd
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
from
(value: Mask<T, N>) ->
Simd
<T, N>
Converts to this type from the input type.
Source
§
impl<T, const N:
usize
>
From
<
Simd
<T, N>> for
[T; N]
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
fn
from
(vector:
Simd
<T, N>) ->
[T; N]
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
f32
, 2>> for
float32x2_t
Source
§
fn
from
(value:
Simd
<
f32
, 2>) ->
float32x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
f32
, 4>> for
float32x4_t
Source
§
fn
from
(value:
Simd
<
f32
, 4>) ->
float32x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
f64
, 1>> for
float64x1_t
Source
§
fn
from
(value:
Simd
<
f64
, 1>) ->
float64x1_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
f64
, 2>> for
float64x2_t
Source
§
fn
from
(value:
Simd
<
f64
, 2>) ->
float64x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i16
, 4>> for
int16x4_t
Source
§
fn
from
(value:
Simd
<
i16
, 4>) ->
int16x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i16
, 8>> for
int16x8_t
Source
§
fn
from
(value:
Simd
<
i16
, 8>) ->
int16x8_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i32
, 2>> for
int32x2_t
Source
§
fn
from
(value:
Simd
<
i32
, 2>) ->
int32x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i32
, 4>> for
int32x4_t
Source
§
fn
from
(value:
Simd
<
i32
, 4>) ->
int32x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i64
, 1>> for
int64x1_t
Source
§
fn
from
(value:
Simd
<
i64
, 1>) ->
int64x1_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i64
, 2>> for
int64x2_t
Source
§
fn
from
(value:
Simd
<
i64
, 2>) ->
int64x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i8
, 16>> for
int8x16_t
Source
§
fn
from
(value:
Simd
<
i8
, 16>) ->
int8x16_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
i8
, 8>> for
int8x8_t
Source
§
fn
from
(value:
Simd
<
i8
, 8>) ->
int8x8_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u16
, 4>> for
poly16x4_t
Source
§
fn
from
(value:
Simd
<
u16
, 4>) ->
poly16x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u16
, 4>> for
uint16x4_t
Source
§
fn
from
(value:
Simd
<
u16
, 4>) ->
uint16x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u16
, 8>> for
poly16x8_t
Source
§
fn
from
(value:
Simd
<
u16
, 8>) ->
poly16x8_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u16
, 8>> for
uint16x8_t
Source
§
fn
from
(value:
Simd
<
u16
, 8>) ->
uint16x8_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u32
, 2>> for
uint32x2_t
Source
§
fn
from
(value:
Simd
<
u32
, 2>) ->
uint32x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u32
, 4>> for
uint32x4_t
Source
§
fn
from
(value:
Simd
<
u32
, 4>) ->
uint32x4_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u64
, 1>> for
poly64x1_t
Source
§
fn
from
(value:
Simd
<
u64
, 1>) ->
poly64x1_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u64
, 1>> for
uint64x1_t
Source
§
fn
from
(value:
Simd
<
u64
, 1>) ->
uint64x1_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u64
, 2>> for
poly64x2_t
Source
§
fn
from
(value:
Simd
<
u64
, 2>) ->
poly64x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u64
, 2>> for
uint64x2_t
Source
§
fn
from
(value:
Simd
<
u64
, 2>) ->
uint64x2_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u8
, 16>> for
poly8x16_t
Source
§
fn
from
(value:
Simd
<
u8
, 16>) ->
poly8x16_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u8
, 16>> for
uint8x16_t
Source
§
fn
from
(value:
Simd
<
u8
, 16>) ->
uint8x16_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u8
, 8>> for
poly8x8_t
Source
§
fn
from
(value:
Simd
<
u8
, 8>) ->
poly8x8_t
Converts to this type from the input type.
Source
§
impl
From
<
Simd
<
u8
, 8>> for
uint8x8_t
Source
§
fn
from
(value:
Simd
<
u8
, 8>) ->
uint8x8_t
Converts to this type from the input type.
Source
§
impl
From
<
float32x2_t
> for
Simd
<
f32
, 2>
Source
§
fn
from
(value:
float32x2_t
) ->
Simd
<
f32
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
float32x4_t
> for
Simd
<
f32
, 4>
Source
§
fn
from
(value:
float32x4_t
) ->
Simd
<
f32
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
float64x1_t
> for
Simd
<
f64
, 1>
Source
§
fn
from
(value:
float64x1_t
) ->
Simd
<
f64
, 1>
Converts to this type from the input type.
Source
§
impl
From
<
float64x2_t
> for
Simd
<
f64
, 2>
Source
§
fn
from
(value:
float64x2_t
) ->
Simd
<
f64
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
int16x4_t
> for
Simd
<
i16
, 4>
Source
§
fn
from
(value:
int16x4_t
) ->
Simd
<
i16
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
int16x8_t
> for
Simd
<
i16
, 8>
Source
§
fn
from
(value:
int16x8_t
) ->
Simd
<
i16
, 8>
Converts to this type from the input type.
Source
§
impl
From
<
int32x2_t
> for
Simd
<
i32
, 2>
Source
§
fn
from
(value:
int32x2_t
) ->
Simd
<
i32
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
int32x4_t
> for
Simd
<
i32
, 4>
Source
§
fn
from
(value:
int32x4_t
) ->
Simd
<
i32
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
int64x1_t
> for
Simd
<
i64
, 1>
Source
§
fn
from
(value:
int64x1_t
) ->
Simd
<
i64
, 1>
Converts to this type from the input type.
Source
§
impl
From
<
int64x2_t
> for
Simd
<
i64
, 2>
Source
§
fn
from
(value:
int64x2_t
) ->
Simd
<
i64
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
int8x16_t
> for
Simd
<
i8
, 16>
Source
§
fn
from
(value:
int8x16_t
) ->
Simd
<
i8
, 16>
Converts to this type from the input type.
Source
§
impl
From
<
int8x8_t
> for
Simd
<
i8
, 8>
Source
§
fn
from
(value:
int8x8_t
) ->
Simd
<
i8
, 8>
Converts to this type from the input type.
Source
§
impl
From
<
poly16x4_t
> for
Simd
<
u16
, 4>
Source
§
fn
from
(value:
poly16x4_t
) ->
Simd
<
u16
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
poly16x8_t
> for
Simd
<
u16
, 8>
Source
§
fn
from
(value:
poly16x8_t
) ->
Simd
<
u16
, 8>
Converts to this type from the input type.
Source
§
impl
From
<
poly64x1_t
> for
Simd
<
u64
, 1>
Source
§
fn
from
(value:
poly64x1_t
) ->
Simd
<
u64
, 1>
Converts to this type from the input type.
Source
§
impl
From
<
poly64x2_t
> for
Simd
<
u64
, 2>
Source
§
fn
from
(value:
poly64x2_t
) ->
Simd
<
u64
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
poly8x16_t
> for
Simd
<
u8
, 16>
Source
§
fn
from
(value:
poly8x16_t
) ->
Simd
<
u8
, 16>
Converts to this type from the input type.
Source
§
impl
From
<
poly8x8_t
> for
Simd
<
u8
, 8>
Source
§
fn
from
(value:
poly8x8_t
) ->
Simd
<
u8
, 8>
Converts to this type from the input type.
Source
§
impl
From
<
uint16x4_t
> for
Simd
<
u16
, 4>
Source
§
fn
from
(value:
uint16x4_t
) ->
Simd
<
u16
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
uint16x8_t
> for
Simd
<
u16
, 8>
Source
§
fn
from
(value:
uint16x8_t
) ->
Simd
<
u16
, 8>
Converts to this type from the input type.
Source
§
impl
From
<
uint32x2_t
> for
Simd
<
u32
, 2>
Source
§
fn
from
(value:
uint32x2_t
) ->
Simd
<
u32
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
uint32x4_t
> for
Simd
<
u32
, 4>
Source
§
fn
from
(value:
uint32x4_t
) ->
Simd
<
u32
, 4>
Converts to this type from the input type.
Source
§
impl
From
<
uint64x1_t
> for
Simd
<
u64
, 1>
Source
§
fn
from
(value:
uint64x1_t
) ->
Simd
<
u64
, 1>
Converts to this type from the input type.
Source
§
impl
From
<
uint64x2_t
> for
Simd
<
u64
, 2>
Source
§
fn
from
(value:
uint64x2_t
) ->
Simd
<
u64
, 2>
Converts to this type from the input type.
Source
§
impl
From
<
uint8x16_t
> for
Simd
<
u8
, 16>
Source
§
fn
from
(value:
uint8x16_t
) ->
Simd
<
u8
, 16>
Converts to this type from the input type.
Source
§
impl
From
<
uint8x8_t
> for
Simd
<
u8
, 8>
Source
§
fn
from
(value:
uint8x8_t
) ->
Simd
<
u8
, 8>
Converts to this type from the input type.
Source
§
impl<T, const N:
usize
>
Hash
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Hash
,
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
Source
§
impl<I, T, const N:
usize
>
Index
<I> for
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
    I:
SliceIndex
<
[T]
>,
Source
§
type
Output
= <I as
SliceIndex
<
[T]
>>::
Output
The returned type after indexing.
Source
§
fn
index
(&self, index: I) -> &<
Simd
<T, N> as
Index
<I>>::
Output
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl<I, T, const N:
usize
>
IndexMut
<I> for
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
    I:
SliceIndex
<
[T]
>,
Source
§
fn
index_mut
(&mut self, index: I) -> &mut <
Simd
<T, N> as
Index
<I>>::
Output
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Mul
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Mul
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Mul
<&'rhs
Simd
<T, N>>>::
Output
Performs the
*
operation.
Read more
Source
§
impl<T, const N:
usize
>
Mul
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Mul
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Mul
<&
Simd
<T, N>>>::
Output
Performs the
*
operation.
Read more
Source
§
impl<T, const N:
usize
>
Mul
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Mul
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Mul
<
Simd
<T, N>>>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<const N:
usize
>
Mul
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Mul
>::
Output
Performs the
*
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
MulAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Mul
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
mul_assign
(&mut self, rhs: U)
Performs the
*=
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
f32
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
f64
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
i16
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
i32
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
i64
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
i8
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Neg
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
-
operator.
Source
§
fn
neg
(self) -> <
Simd
<
isize
, N> as
Neg
>::
Output
Performs the unary
-
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
i16
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
i32
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
i64
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
i8
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
isize
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
u16
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
u32
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
u64
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
u8
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<const N:
usize
>
Not
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Simd
<
usize
, N> as
Not
>::
Output
Performs the unary
!
operation.
Read more
Source
§
impl<T, const N:
usize
>
Ord
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Ord
,
Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
Source
§
fn
cmp
(&self, other: &
Simd
<T, N>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
Source
§
impl<T, const N:
usize
>
PartialEq
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
PartialEq
,
Source
§
fn
eq
(&self, other: &
Simd
<T, N>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Simd
<T, N>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<T, const N:
usize
>
PartialOrd
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
PartialOrd
,
Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
Source
§
fn
partial_cmp
(&self, other: &
Simd
<T, N>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
f32
, N>> for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
f32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
f32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
f64
, N>> for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
f64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
f64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
i16
, N>> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i16
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i16
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
i32
, N>> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
i64
, N>> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
i8
, N>> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i8
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i8
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
isize
, N>> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
isize
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
isize
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
u16
, N>> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u16
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u16
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
u32
, N>> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
u64
, N>> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
u8
, N>> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u8
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u8
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'a, const N:
usize
>
Product
<&'a
Simd
<
usize
, N>> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
usize
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
usize
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
f32
, N>
where
    I:
Iterator
<Item =
Simd
<
f32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
f64
, N>
where
    I:
Iterator
<Item =
Simd
<
f64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i16
, N>
where
    I:
Iterator
<Item =
Simd
<
i16
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i32
, N>
where
    I:
Iterator
<Item =
Simd
<
i32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i64
, N>
where
    I:
Iterator
<Item =
Simd
<
i64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
i8
, N>
where
    I:
Iterator
<Item =
Simd
<
i8
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
isize
, N>
where
    I:
Iterator
<Item =
Simd
<
isize
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u16
, N>
where
    I:
Iterator
<Item =
Simd
<
u16
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u32
, N>
where
    I:
Iterator
<Item =
Simd
<
u32
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u64
, N>
where
    I:
Iterator
<Item =
Simd
<
u64
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
u8
, N>
where
    I:
Iterator
<Item =
Simd
<
u8
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<const N:
usize
>
Product
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
product
<I>(iter: I) ->
Simd
<
usize
, N>
where
    I:
Iterator
<Item =
Simd
<
usize
, N>>,
Takes an iterator and generates
Self
from the elements by multiplying
the items.
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Rem
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Rem
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Rem
<&'rhs
Simd
<T, N>>>::
Output
Performs the
%
operation.
Read more
Source
§
impl<T, const N:
usize
>
Rem
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Rem
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Rem
<&
Simd
<T, N>>>::
Output
Performs the
%
operation.
Read more
Source
§
impl<T, const N:
usize
>
Rem
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Rem
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Rem
<
Simd
<T, N>>>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<const N:
usize
>
Rem
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
%
operator.
Source
§
fn
rem
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Rem
>::
Output
Performs the
%
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
RemAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Rem
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
rem_assign
(&mut self, rhs: U)
Performs the
%=
operation.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Shl
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shl
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Shl
<&'rhs
Simd
<T, N>>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<T, const N:
usize
>
Shl
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shl
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Shl
<&
Simd
<T, N>>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
i16
> for &'lhs
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i16
) -> <&'lhs
Simd
<
i16
, N> as
Shl
<&
i16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
i16
> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i16
) -> <
Simd
<
i16
, N> as
Shl
<&
i16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
i32
> for &'lhs
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i32
) -> <&'lhs
Simd
<
i32
, N> as
Shl
<&
i32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
i32
> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i32
) -> <
Simd
<
i32
, N> as
Shl
<&
i32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
i64
> for &'lhs
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i64
) -> <&'lhs
Simd
<
i64
, N> as
Shl
<&
i64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
i64
> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i64
) -> <
Simd
<
i64
, N> as
Shl
<&
i64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
i8
> for &'lhs
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i8
) -> <&'lhs
Simd
<
i8
, N> as
Shl
<&
i8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
i8
> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
i8
) -> <
Simd
<
i8
, N> as
Shl
<&
i8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
isize
> for &'lhs
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shl
<&
isize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
isize
> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
isize
) -> <
Simd
<
isize
, N> as
Shl
<&
isize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u16
> for &'lhs
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u16
) -> <&'lhs
Simd
<
u16
, N> as
Shl
<&
u16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
u16
> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u16
) -> <
Simd
<
u16
, N> as
Shl
<&
u16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u32
> for &'lhs
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u32
) -> <&'lhs
Simd
<
u32
, N> as
Shl
<&
u32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
u32
> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u32
) -> <
Simd
<
u32
, N> as
Shl
<&
u32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u64
> for &'lhs
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shl
<&
u64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
u64
> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u64
) -> <
Simd
<
u64
, N> as
Shl
<&
u64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shl
<&
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
u8
) -> <
Simd
<
u8
, N> as
Shl
<&
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<&
usize
> for &'lhs
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shl
<&
usize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<&
usize
> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs: &
usize
) -> <
Simd
<
usize
, N> as
Shl
<&
usize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<T, const N:
usize
>
Shl
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shl
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Shl
<
Simd
<T, N>>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
i16
> for &'lhs
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i16
) -> <&'lhs
Simd
<
i16
, N> as
Shl
<
i16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
i16
> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i16
) -> <
Simd
<
i16
, N> as
Shl
<
i16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
i32
> for &'lhs
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i32
) -> <&'lhs
Simd
<
i32
, N> as
Shl
<
i32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
i32
> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i32
) -> <
Simd
<
i32
, N> as
Shl
<
i32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
i64
> for &'lhs
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i64
) -> <&'lhs
Simd
<
i64
, N> as
Shl
<
i64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
i64
> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i64
) -> <
Simd
<
i64
, N> as
Shl
<
i64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
i8
> for &'lhs
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i8
) -> <&'lhs
Simd
<
i8
, N> as
Shl
<
i8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
i8
> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
i8
) -> <
Simd
<
i8
, N> as
Shl
<
i8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
isize
> for &'lhs
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shl
<
isize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
isize
> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
isize
) -> <
Simd
<
isize
, N> as
Shl
<
isize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
u16
> for &'lhs
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u16
) -> <&'lhs
Simd
<
u16
, N> as
Shl
<
u16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
u16
> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u16
) -> <
Simd
<
u16
, N> as
Shl
<
u16
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
u32
> for &'lhs
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u32
) -> <&'lhs
Simd
<
u32
, N> as
Shl
<
u32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
u32
> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u32
) -> <
Simd
<
u32
, N> as
Shl
<
u32
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
u64
> for &'lhs
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shl
<
u64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
u64
> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u64
) -> <
Simd
<
u64
, N> as
Shl
<
u64
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shl
<
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
u8
) -> <
Simd
<
u8
, N> as
Shl
<
u8
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shl
<
usize
> for &'lhs
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
<
usize
> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
usize
) -> <
Simd
<
usize
, N> as
Shl
<
usize
>>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<const N:
usize
>
Shl
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
<<
operator.
Source
§
fn
shl
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Shl
>::
Output
Performs the
<<
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
ShlAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Shl
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
shl_assign
(&mut self, rhs: U)
Performs the
<<=
operation.
Read more
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Shr
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Shr
<&'rhs
Simd
<T, N>>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<T, const N:
usize
>
Shr
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Shr
<&
Simd
<T, N>>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
i16
> for &'lhs
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i16
) -> <&'lhs
Simd
<
i16
, N> as
Shr
<&
i16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
i16
> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i16
) -> <
Simd
<
i16
, N> as
Shr
<&
i16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
i32
> for &'lhs
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i32
) -> <&'lhs
Simd
<
i32
, N> as
Shr
<&
i32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
i32
> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i32
) -> <
Simd
<
i32
, N> as
Shr
<&
i32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
i64
> for &'lhs
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i64
) -> <&'lhs
Simd
<
i64
, N> as
Shr
<&
i64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
i64
> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i64
) -> <
Simd
<
i64
, N> as
Shr
<&
i64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
i8
> for &'lhs
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i8
) -> <&'lhs
Simd
<
i8
, N> as
Shr
<&
i8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
i8
> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
i8
) -> <
Simd
<
i8
, N> as
Shr
<&
i8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
isize
> for &'lhs
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shr
<&
isize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
isize
> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
isize
) -> <
Simd
<
isize
, N> as
Shr
<&
isize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u16
> for &'lhs
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u16
) -> <&'lhs
Simd
<
u16
, N> as
Shr
<&
u16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
u16
> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u16
) -> <
Simd
<
u16
, N> as
Shr
<&
u16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u32
> for &'lhs
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u32
) -> <&'lhs
Simd
<
u32
, N> as
Shr
<&
u32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
u32
> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u32
) -> <
Simd
<
u32
, N> as
Shr
<&
u32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u64
> for &'lhs
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shr
<&
u64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
u64
> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u64
) -> <
Simd
<
u64
, N> as
Shr
<&
u64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shr
<&
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
u8
) -> <
Simd
<
u8
, N> as
Shr
<&
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<&
usize
> for &'lhs
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shr
<&
usize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<&
usize
> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs: &
usize
) -> <
Simd
<
usize
, N> as
Shr
<&
usize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<T, const N:
usize
>
Shr
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Shr
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Shr
<
Simd
<T, N>>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
i16
> for &'lhs
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i16
) -> <&'lhs
Simd
<
i16
, N> as
Shr
<
i16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
i16
> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i16
) -> <
Simd
<
i16
, N> as
Shr
<
i16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
i32
> for &'lhs
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i32
) -> <&'lhs
Simd
<
i32
, N> as
Shr
<
i32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
i32
> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i32
) -> <
Simd
<
i32
, N> as
Shr
<
i32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
i64
> for &'lhs
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i64
) -> <&'lhs
Simd
<
i64
, N> as
Shr
<
i64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
i64
> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i64
) -> <
Simd
<
i64
, N> as
Shr
<
i64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
i8
> for &'lhs
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i8
) -> <&'lhs
Simd
<
i8
, N> as
Shr
<
i8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
i8
> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
i8
) -> <
Simd
<
i8
, N> as
Shr
<
i8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
isize
> for &'lhs
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
isize
) -> <&'lhs
Simd
<
isize
, N> as
Shr
<
isize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
isize
> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
isize
) -> <
Simd
<
isize
, N> as
Shr
<
isize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
u16
> for &'lhs
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u16
) -> <&'lhs
Simd
<
u16
, N> as
Shr
<
u16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
u16
> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u16
) -> <
Simd
<
u16
, N> as
Shr
<
u16
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
u32
> for &'lhs
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u32
) -> <&'lhs
Simd
<
u32
, N> as
Shr
<
u32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
u32
> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u32
) -> <
Simd
<
u32
, N> as
Shr
<
u32
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
u64
> for &'lhs
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u64
) -> <&'lhs
Simd
<
u64
, N> as
Shr
<
u64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
u64
> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u64
) -> <
Simd
<
u64
, N> as
Shr
<
u64
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
u8
> for &'lhs
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u8
) -> <&'lhs
Simd
<
u8
, N> as
Shr
<
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
u8
> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
u8
) -> <
Simd
<
u8
, N> as
Shr
<
u8
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<'lhs, const N:
usize
>
Shr
<
usize
> for &'lhs
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
usize
) -> <&'lhs
Simd
<
usize
, N> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
<
usize
> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
usize
) -> <
Simd
<
usize
, N> as
Shr
<
usize
>>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<const N:
usize
>
Shr
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
>>
operator.
Source
§
fn
shr
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Shr
>::
Output
Performs the
>>
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
ShrAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Shr
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
shr_assign
(&mut self, rhs: U)
Performs the
>>=
operation.
Read more
Source
§
impl<T, const N:
usize
>
SimdConstPtr
for
Simd
<
*const T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Usize
=
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
usize
with the same number of elements.
Source
§
type
Isize
=
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
isize
with the same number of elements.
Source
§
type
CastPtr
<U> =
Simd
<
*const U
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of const pointers with the same number of elements.
Source
§
type
MutPtr
=
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of mutable pointers to the same type.
Source
§
type
Mask
=
Mask
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
fn
is_null
(self) -> <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns
true
for each element that is null.
Source
§
fn
cast
<U>(self) -> <
Simd
<
*const T
, N> as
SimdConstPtr
>::
CastPtr
<U>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Casts to a pointer of another type.
Read more
Source
§
fn
cast_mut
(self) -> <
Simd
<
*const T
, N> as
SimdConstPtr
>::
MutPtr
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Changes constness without changing the type.
Read more
Source
§
fn
addr
(self) -> <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Gets the “address” portion of the pointer.
Read more
Source
§
fn
without_provenance
(
    addr: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address to a pointer without giving it any provenance.
Read more
Source
§
fn
with_addr
(
    self,
    addr: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new pointer with the given address.
Read more
Source
§
fn
expose_provenance
(self) -> <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Exposes the “provenance” part of the pointer for future use in
Self::with_exposed_provenance
and returns the “address” portion.
Source
§
fn
with_exposed_provenance
(
    addr: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address back to a pointer, picking up a previously “exposed” provenance.
Read more
Source
§
fn
wrapping_offset
(
    self,
    count: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Isize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
fn
wrapping_add
(
    self,
    count: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
fn
wrapping_sub
(
    self,
    count: <
Simd
<
*const T
, N> as
SimdConstPtr
>::
Usize
,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
impl<const N:
usize
>
SimdFloat
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i32
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
f32
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Bits
=
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Bit representation of this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
unsafe fn
to_int_unchecked
<I>(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Cast
<I>
where
    I:
SimdCast
,
    <
Simd
<
f32
, N> as
SimdFloat
>::
Scalar
:
FloatToInt
<I>,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds toward zero and converts to the same-width integer type, assuming that
the value is finite and fits in that type.
Read more
Source
§
fn
to_bits
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation to an unsigned integer vector type with the
same size and number of elements.
Source
§
fn
from_bits
(bits:
Simd
<
u32
, N>) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation from an unsigned integer vector type with the
same size and number of elements.
Source
§
fn
abs
(self) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the absolute value of the
equivalently-indexed element in
self
.
Source
§
fn
recip
(self) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Takes the reciprocal (inverse) of each element,
1/x
.
Source
§
fn
to_degrees
(self) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from radians to degrees.
Source
§
fn
to_radians
(self) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from degrees to radians.
Source
§
fn
is_sign_positive
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a positive sign, including
+0.0
,
NaN
s with positive sign bit and positive infinity.
Source
§
fn
is_sign_negative
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a negative sign, including
-0.0
,
NaN
s with negative sign bit and negative infinity.
Source
§
fn
is_nan
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is
NaN
.
Source
§
fn
is_infinite
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is positive infinity or negative infinity.
Source
§
fn
is_finite
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither infinite nor
NaN
.
Source
§
fn
is_subnormal
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is subnormal.
Source
§
fn
is_normal
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither zero, infinite,
subnormal, nor
NaN
.
Source
§
fn
signum
(self) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Replaces each element with a number that represents its sign.
Read more
Source
§
fn
copysign
(self, sign:
Simd
<
f32
, N>) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns each element with the magnitude of
self
and the sign of
sign
.
Read more
Source
§
fn
simd_min
(self, other:
Simd
<
f32
, N>) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum of each element.
Read more
Source
§
fn
simd_max
(self, other:
Simd
<
f32
, N>) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum of each element.
Read more
Source
§
fn
simd_clamp
(self, min:
Simd
<
f32
, N>, max:
Simd
<
f32
, N>) ->
Simd
<
f32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval unless it is NaN.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reducing multiply.  Returns the product of the elements of the vector.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
f32
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
impl<const N:
usize
>
SimdFloat
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i64
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
f64
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Bits
=
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Bit representation of this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
unsafe fn
to_int_unchecked
<I>(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Cast
<I>
where
    I:
SimdCast
,
    <
Simd
<
f64
, N> as
SimdFloat
>::
Scalar
:
FloatToInt
<I>,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds toward zero and converts to the same-width integer type, assuming that
the value is finite and fits in that type.
Read more
Source
§
fn
to_bits
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation to an unsigned integer vector type with the
same size and number of elements.
Source
§
fn
from_bits
(bits:
Simd
<
u64
, N>) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Raw transmutation from an unsigned integer vector type with the
same size and number of elements.
Source
§
fn
abs
(self) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the absolute value of the
equivalently-indexed element in
self
.
Source
§
fn
recip
(self) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Takes the reciprocal (inverse) of each element,
1/x
.
Source
§
fn
to_degrees
(self) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from radians to degrees.
Source
§
fn
to_radians
(self) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts each element from degrees to radians.
Source
§
fn
is_sign_positive
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a positive sign, including
+0.0
,
NaN
s with positive sign bit and positive infinity.
Source
§
fn
is_sign_negative
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if it has a negative sign, including
-0.0
,
NaN
s with negative sign bit and negative infinity.
Source
§
fn
is_nan
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is
NaN
.
Source
§
fn
is_infinite
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is positive infinity or negative infinity.
Source
§
fn
is_finite
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither infinite nor
NaN
.
Source
§
fn
is_subnormal
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is subnormal.
Source
§
fn
is_normal
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each element if its value is neither zero, infinite,
subnormal, nor
NaN
.
Source
§
fn
signum
(self) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Replaces each element with a number that represents its sign.
Read more
Source
§
fn
copysign
(self, sign:
Simd
<
f64
, N>) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns each element with the magnitude of
self
and the sign of
sign
.
Read more
Source
§
fn
simd_min
(self, other:
Simd
<
f64
, N>) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum of each element.
Read more
Source
§
fn
simd_max
(self, other:
Simd
<
f64
, N>) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum of each element.
Read more
Source
§
fn
simd_clamp
(self, min:
Simd
<
f64
, N>, max:
Simd
<
f64
, N>) ->
Simd
<
f64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval unless it is NaN.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reducing multiply.  Returns the product of the elements of the vector.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
f64
, N> as
SimdFloat
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i16
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
i16
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Unsigned
=
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
i16
, N>) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
i16
, N>) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
saturating_abs
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
saturating_neg
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
is_positive
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
§
fn
is_negative
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
§
fn
signum
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
fn
reduce_and
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) -> <
Simd
<
i16
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i32
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
i32
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Unsigned
=
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
i32
, N>) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
i32
, N>) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
saturating_abs
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
saturating_neg
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
is_positive
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
§
fn
is_negative
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
§
fn
signum
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
fn
reduce_and
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) -> <
Simd
<
i32
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i64
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
i64
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Unsigned
=
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
i64
, N>) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
i64
, N>) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
saturating_abs
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
saturating_neg
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
is_positive
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
§
fn
is_negative
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
§
fn
signum
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
fn
reduce_and
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) -> <
Simd
<
i64
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i8
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
i8
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Unsigned
=
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
i8
, N>) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
i8
, N>) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
saturating_abs
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
saturating_neg
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
is_positive
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
§
fn
is_negative
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
§
fn
signum
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
fn
reduce_and
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) -> <
Simd
<
i8
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdInt
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
isize
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
type
Scalar
=
isize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Unsigned
=
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector of unsigned integers with the same element size.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
isize
, N>) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
isize
, N>) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute value, implemented in Rust.
Every element becomes its absolute value.
Read more
Source
§
fn
abs_diff
(
    self,
    second:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
saturating_abs
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating absolute value, implemented in Rust.
As abs(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
saturating_neg
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating negation, implemented in Rust.
As neg(), except the MIN value becomes MAX instead of itself.
Read more
Source
§
fn
is_positive
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each positive element and false if it is zero or negative.
Source
§
fn
is_negative
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true for each negative element and false if it is zero or positive.
Source
§
fn
signum
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns numbers representing the sign of each element.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Read more
Source
§
fn
reduce_product
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Read more
Source
§
fn
reduce_max
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Read more
Source
§
fn
reduce_min
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Read more
Source
§
fn
reduce_and
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) -> <
Simd
<
isize
, N> as
SimdInt
>::
Unsigned
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<T, const N:
usize
>
SimdMutPtr
for
Simd
<
*mut T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Usize
=
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
usize
with the same number of elements.
Source
§
type
Isize
=
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
isize
with the same number of elements.
Source
§
type
CastPtr
<U> =
Simd
<
*mut U
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of const pointers with the same number of elements.
Source
§
type
ConstPtr
=
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of constant pointers to the same type.
Source
§
type
Mask
=
Mask
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Source
§
fn
is_null
(self) -> <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns
true
for each element that is null.
Source
§
fn
cast
<U>(self) -> <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
CastPtr
<U>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Casts to a pointer of another type.
Read more
Source
§
fn
cast_const
(self) -> <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
ConstPtr
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Changes constness without changing the type.
Read more
Source
§
fn
addr
(self) -> <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Gets the “address” portion of the pointer.
Read more
Source
§
fn
without_provenance
(
    addr: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address to a pointer without giving it any provenance.
Read more
Source
§
fn
with_addr
(
    self,
    addr: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new pointer with the given address.
Read more
Source
§
fn
expose_provenance
(self) -> <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Exposes the “provenance” part of the pointer for future use in
Self::with_exposed_provenance
and returns the “address” portion.
Source
§
fn
with_exposed_provenance
(
    addr: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address back to a pointer, picking up a previously “exposed” provenance.
Read more
Source
§
fn
wrapping_offset
(
    self,
    count: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Isize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
fn
wrapping_add
(
    self,
    count: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
fn
wrapping_sub
(
    self,
    count: <
Simd
<
*mut T
, N> as
SimdMutPtr
>::
Usize
,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Read more
Source
§
impl<T, const N:
usize
>
SimdOrd
for
Simd
<
*const T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
*const T
, N>) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
*const T
, N>) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(
    self,
    min:
Simd
<
*const T
, N>,
    max:
Simd
<
*const T
, N>,
) ->
Simd
<
*const T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<T, const N:
usize
>
SimdOrd
for
Simd
<
*mut T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
*mut T
, N>) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
*mut T
, N>) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(
    self,
    min:
Simd
<
*mut T
, N>,
    max:
Simd
<
*mut T
, N>,
) ->
Simd
<
*mut T
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
i16
, N>) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
i16
, N>) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
i16
, N>, max:
Simd
<
i16
, N>) ->
Simd
<
i16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
i32
, N>) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
i32
, N>) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
i32
, N>, max:
Simd
<
i32
, N>) ->
Simd
<
i32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
i64
, N>) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
i64
, N>) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
i64
, N>, max:
Simd
<
i64
, N>) ->
Simd
<
i64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
i8
, N>) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
i8
, N>) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
i8
, N>, max:
Simd
<
i8
, N>) ->
Simd
<
i8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
isize
, N>) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
isize
, N>) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
isize
, N>, max:
Simd
<
isize
, N>) ->
Simd
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
u16
, N>, max:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
u32
, N>, max:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
u64
, N>, max:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
u8
, N>, max:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<const N:
usize
>
SimdOrd
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_max
(self, other:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
§
fn
simd_min
(self, other:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
§
fn
simd_clamp
(self, min:
Simd
<
usize
, N>, max:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
Read more
Source
§
impl<T, const N:
usize
>
SimdPartialEq
for
Simd
<
*const T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<T, const N:
usize
>
SimdPartialEq
for
Simd
<
*mut T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
isize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
f32
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
f64
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i16
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i32
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i64
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i8
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
isize
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u16
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u32
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u64
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u8
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
usize
as
SimdElement
>::
Mask
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Source
§
fn
simd_eq
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
§
fn
simd_ne
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
Source
§
impl<T, const N:
usize
>
SimdPartialOrd
for
Simd
<
*const T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(
    self,
    other:
Simd
<
*const T
, N>,
) -> <
Simd
<
*const T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<T, const N:
usize
>
SimdPartialOrd
for
Simd
<
*mut T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(
    self,
    other:
Simd
<
*mut T
, N>,
) -> <
Simd
<
*mut T
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(
    self,
    other:
Simd
<
isize
, N>,
) -> <
Simd
<
isize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(self, other:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdPartialOrd
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
simd_lt
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
§
fn
simd_le
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
§
fn
simd_gt
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
§
fn
simd_ge
(
    self,
    other:
Simd
<
usize
, N>,
) -> <
Simd
<
usize
, N> as
SimdPartialEq
>::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
other
.
Source
§
impl<const N:
usize
>
SimdUint
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u16
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
wrapping_neg
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
u16
, N>) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Source
§
fn
reduce_product
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Source
§
fn
reduce_max
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Source
§
fn
reduce_min
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Source
§
fn
reduce_and
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
u16
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) ->
Simd
<
u16
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdUint
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u32
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
wrapping_neg
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
u32
, N>) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Source
§
fn
reduce_product
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Source
§
fn
reduce_max
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Source
§
fn
reduce_min
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Source
§
fn
reduce_and
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
u32
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) ->
Simd
<
u32
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdUint
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u64
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
wrapping_neg
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
u64
, N>) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Source
§
fn
reduce_product
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Source
§
fn
reduce_max
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Source
§
fn
reduce_min
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Source
§
fn
reduce_and
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
u64
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) ->
Simd
<
u64
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdUint
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
u8
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
wrapping_neg
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
u8
, N>) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Source
§
fn
reduce_product
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Source
§
fn
reduce_max
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Source
§
fn
reduce_min
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Source
§
fn
reduce_and
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
u8
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) ->
Simd
<
u8
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
SimdUint
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Scalar
=
usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Scalar type contained by this SIMD vector type.
Source
§
type
Cast
<T:
SimdElement
> =
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
A SIMD vector with a different element type.
Source
§
fn
cast
<T>(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Cast
<T>
where
    T:
SimdCast
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Performs elementwise conversion of this vector’s elements to another SIMD-valid type.
Read more
Source
§
fn
wrapping_neg
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Wrapping negation.
Read more
Source
§
fn
saturating_add
(self, second:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating add.
Read more
Source
§
fn
saturating_sub
(self, second:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise saturating subtract.
Read more
Source
§
fn
abs_diff
(self, second:
Simd
<
usize
, N>) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Lanewise absolute difference.
Every element becomes the absolute difference of
self
and
second
.
Read more
Source
§
fn
reduce_sum
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the sum of the elements of the vector, with wrapping addition.
Source
§
fn
reduce_product
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the product of the elements of the vector, with wrapping multiplication.
Source
§
fn
reduce_max
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the maximum element in the vector.
Source
§
fn
reduce_min
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the minimum element in the vector.
Source
§
fn
reduce_and
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “and” across the elements of the vector.
Source
§
fn
reduce_or
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “or” across the elements of the vector.
Source
§
fn
reduce_xor
(self) -> <
Simd
<
usize
, N> as
SimdUint
>::
Scalar
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the cumulative bitwise “xor” across the elements of the vector.
Source
§
fn
swap_bytes
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the byte order of each element.
Source
§
fn
reverse_bits
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverses the order of bits in each elemnent.
The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
Source
§
fn
count_ones
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of ones in the binary representation of each element.
Source
§
fn
count_zeros
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of zeros in the binary representation of each element.
Source
§
fn
leading_zeros
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading zeros in the binary representation of each element.
Source
§
fn
trailing_zeros
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing zeros in the binary representation of each element.
Source
§
fn
leading_ones
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of leading ones in the binary representation of each element.
Source
§
fn
trailing_ones
(self) ->
Simd
<
usize
, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the number of trailing ones in the binary representation of each element.
Source
§
impl<const N:
usize
>
StdFloat
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
fract
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s fractional value, with its integer part removed.
Source
§
fn
sin
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the sine of the value
in the equivalently-indexed element in
self
.
Source
§
fn
cos
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the cosine of the value
in the equivalently-indexed element in
self
.
Source
§
fn
exp
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base e) of the value
in the equivalently-indexed element in
self
.
Source
§
fn
exp2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base 2) of the value
in the equivalently-indexed element in
self
.
Source
§
fn
ln
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the natural logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
log2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-2 logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
log10
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-10 logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
mul_add
(self, a: Self, b: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Elementwise fused multiply-add. Computes
(self * a) + b
with only one rounding error,
yielding a more accurate result than an unfused multiply-add.
Read more
Source
§
fn
sqrt
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the square root value
of the equivalently-indexed element in
self
Source
§
fn
log
(self, base: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the logarithm with respect to an arbitrary
in the equivalently-indexed elements in
self
and
base
.
Source
§
fn
ceil
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the smallest integer greater than or equal to each element.
Source
§
fn
floor
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the largest integer value less than or equal to each element.
Source
§
fn
round
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds to the nearest integer value. Ties round toward zero.
Source
§
fn
trunc
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s integer value, with its fractional part removed.
Source
§
impl<const N:
usize
>
StdFloat
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
fract
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s fractional value, with its integer part removed.
Source
§
fn
sin
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the sine of the value
in the equivalently-indexed element in
self
.
Source
§
fn
cos
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the cosine of the value
in the equivalently-indexed element in
self
.
Source
§
fn
exp
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base e) of the value
in the equivalently-indexed element in
self
.
Source
§
fn
exp2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base 2) of the value
in the equivalently-indexed element in
self
.
Source
§
fn
ln
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the natural logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
log2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-2 logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
log10
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-10 logarithm of the value
in the equivalently-indexed element in
self
.
Source
§
fn
mul_add
(self, a: Self, b: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Elementwise fused multiply-add. Computes
(self * a) + b
with only one rounding error,
yielding a more accurate result than an unfused multiply-add.
Read more
Source
§
fn
sqrt
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the square root value
of the equivalently-indexed element in
self
Source
§
fn
log
(self, base: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the logarithm with respect to an arbitrary
in the equivalently-indexed elements in
self
and
base
.
Source
§
fn
ceil
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the smallest integer greater than or equal to each element.
Source
§
fn
floor
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the largest integer value less than or equal to each element.
Source
§
fn
round
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds to the nearest integer value. Ties round toward zero.
Source
§
fn
trunc
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s integer value, with its fractional part removed.
Source
§
impl<'lhs, 'rhs, T, const N:
usize
>
Sub
<&'rhs
Simd
<T, N>> for &'lhs
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Sub
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(
    self,
    rhs: &'rhs
Simd
<T, N>,
) -> <&'lhs
Simd
<T, N> as
Sub
<&'rhs
Simd
<T, N>>>::
Output
Performs the
-
operation.
Read more
Source
§
impl<T, const N:
usize
>
Sub
<&
Simd
<T, N>> for
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Sub
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs: &
Simd
<T, N>) -> <
Simd
<T, N> as
Sub
<&
Simd
<T, N>>>::
Output
Performs the
-
operation.
Read more
Source
§
impl<T, const N:
usize
>
Sub
<
Simd
<T, N>> for &
Simd
<T, N>
where
    T:
SimdElement
,
Simd
<T, N>:
Sub
<Output =
Simd
<T, N>>,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<T, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<T, N>) -> <&
Simd
<T, N> as
Sub
<
Simd
<T, N>>>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
f32
, N>
where
f32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f32
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
f32
, N>) -> <
Simd
<
f32
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
f64
, N>
where
f64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
f64
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
f64
, N>) -> <
Simd
<
f64
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
i16
, N>
where
i16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i16
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
i16
, N>) -> <
Simd
<
i16
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
i32
, N>
where
i32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i32
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
i32
, N>) -> <
Simd
<
i32
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
i64
, N>
where
i64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i64
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
i64
, N>) -> <
Simd
<
i64
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
i8
, N>
where
i8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
i8
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
i8
, N>) -> <
Simd
<
i8
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
isize
, N>
where
isize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
isize
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
isize
, N>) -> <
Simd
<
isize
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
u16
, N>
where
u16
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u16
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
u16
, N>) -> <
Simd
<
u16
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
u32
, N>
where
u32
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u32
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
u32
, N>) -> <
Simd
<
u32
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
u64
, N>
where
u64
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u64
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
u64
, N>) -> <
Simd
<
u64
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
u8
, N>
where
u8
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
u8
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
u8
, N>) -> <
Simd
<
u8
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<const N:
usize
>
Sub
for
Simd
<
usize
, N>
where
usize
:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Output
=
Simd
<
usize
, N>
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Simd
<
usize
, N>) -> <
Simd
<
usize
, N> as
Sub
>::
Output
Performs the
-
operation.
Read more
Source
§
impl<T, U, const N:
usize
>
SubAssign
<U> for
Simd
<T, N>
where
Simd
<T, N>:
Sub
<U, Output =
Simd
<T, N>>,
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sub_assign
(&mut self, rhs: U)
Performs the
-=
operation.
Read more
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
f32
, N>> for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
f32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
f32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
f64
, N>> for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
f64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
f64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
i16
, N>> for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i16
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i16
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
i32
, N>> for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
i64
, N>> for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
i8
, N>> for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i8
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
i8
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
isize
, N>> for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
isize
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
isize
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
u16
, N>> for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u16
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u16
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
u32
, N>> for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u32
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
u64
, N>> for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u64
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
u8
, N>> for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u8
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
u8
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<'a, const N:
usize
>
Sum
<&'a
Simd
<
usize
, N>> for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
usize
, N>
where
    I:
Iterator
<Item = &'a
Simd
<
usize
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
f32
, N>
where
    I:
Iterator
<Item =
Simd
<
f32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
f64
, N>
where
    I:
Iterator
<Item =
Simd
<
f64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i16
, N>
where
    I:
Iterator
<Item =
Simd
<
i16
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i32
, N>
where
    I:
Iterator
<Item =
Simd
<
i32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i64
, N>
where
    I:
Iterator
<Item =
Simd
<
i64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
i8
, N>
where
    I:
Iterator
<Item =
Simd
<
i8
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
isize
, N>
where
    I:
Iterator
<Item =
Simd
<
isize
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u16
, N>
where
    I:
Iterator
<Item =
Simd
<
u16
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u32
, N>
where
    I:
Iterator
<Item =
Simd
<
u32
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u64
, N>
where
    I:
Iterator
<Item =
Simd
<
u64
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
u8
, N>
where
    I:
Iterator
<Item =
Simd
<
u8
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl<const N:
usize
>
Sum
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
sum
<I>(iter: I) ->
Simd
<
usize
, N>
where
    I:
Iterator
<Item =
Simd
<
usize
, N>>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
Source
§
impl
ToBytes
for
Simd
<
f32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#52}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#56}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#53}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#54}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#55}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#57}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#58}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#59}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
f64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#60}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
f64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
f64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#33}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#37}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#34}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#38}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#35}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i16
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#36}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#39}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#43}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#40}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#41}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#42}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#44}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#45}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#46}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#47}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#26}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#30}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#27}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#31}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#28}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 64>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#32}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
i8
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#29}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
i8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
i8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
isize
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#48}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
isize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
isize
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#49}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
isize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
isize
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#50}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
isize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
isize
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#51}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
isize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
isize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#7}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#11}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#8}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#12}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#9}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u16
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#10}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u16
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u16
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u32
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#13}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u32
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u32
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#17}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u32
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u32
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#14}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u32
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u32
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#15}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u32
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u32
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#16}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u32
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u32
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u64
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#18}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u64
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u64
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#19}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u64
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u64
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#20}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u64
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u64
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#21}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u64
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u64
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#0}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 16>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#4}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 16> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 16>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#1}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 32>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#5}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 32> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 32>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#2}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 64>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#6}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 64> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 64>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
u8
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#3}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
u8
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
u8
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
usize
, 1>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#22}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
usize
, 1> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 1>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
usize
, 2>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#23}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
usize
, 2> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 2>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
usize
, 4>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#24}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
usize
, 4> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 4>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl
ToBytes
for
Simd
<
usize
, 8>
Source
§
type
Bytes
=
Simd
<
u8
, core::::core_simd::to_bytes::{impl#25}::Bytes::{constant#0}>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
This type, reinterpreted as bytes.
Source
§
fn
to_ne_bytes
(self) -> <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in native byte
order.
Source
§
fn
to_be_bytes
(self) -> <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in big-endian
(network) byte order.
Source
§
fn
to_le_bytes
(self) -> <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the memory representation of this integer as a byte array in little-endian
byte order.
Source
§
fn
from_ne_bytes
(bytes: <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a native endian integer value from its memory representation as a byte array
in native endianness.
Source
§
fn
from_be_bytes
(bytes: <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in big endian.
Source
§
fn
from_le_bytes
(bytes: <
Simd
<
usize
, 8> as
ToBytes
>::
Bytes
) ->
Simd
<
usize
, 8>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates an integer value from its representation as a byte array in little endian.
Source
§
impl<T, const N:
usize
>
TryFrom
<&
[T]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
type
Error
=
TryFromSliceError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(slice: &
[T]
) ->
Result
<
Simd
<T, N>,
TryFromSliceError
>
Performs the conversion.
Source
§
impl<T, const N:
usize
>
TryFrom
<&mut
[T]
> for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
type
Error
=
TryFromSliceError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(slice: &mut
[T]
) ->
Result
<
Simd
<T, N>,
TryFromSliceError
>
Performs the conversion.
Source
§
impl<T, const N:
usize
>
Copy
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
,
Source
§
impl<T, const N:
usize
>
Eq
for
Simd
<T, N>
where
LaneCount
<N>:
SupportedLaneCount
,
    T:
SimdElement
+
Eq
,
Auto Trait Implementations
§
§
impl<T, const N:
usize
>
Freeze
for
Simd
<T, N>
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for
Simd
<T, N>
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
Send
for
Simd
<T, N>
where
    T:
Send
,
§
impl<T, const N:
usize
>
Sync
for
Simd
<T, N>
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Unpin
for
Simd
<T, N>
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
UnwindSafe
for
Simd
<T, N>
where
    T:
UnwindSafe
,
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.