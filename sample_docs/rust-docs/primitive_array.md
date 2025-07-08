array - Rust
Primitive Type
array
Copy item path
1.0.0
Expand description
A fixed-size array, denoted
[T; N]
, for the element type,
T
, and the
non-negative compile-time constant size,
N
.
There are two syntactic forms for creating an array:
A list with each element, i.e.,
[x, y, z]
.
A repeat expression
[expr; N]
where
N
is how many times to repeat
expr
in the array.
expr
must either be:
A value of a type implementing the
Copy
trait
A
const
value
Note that
[expr; 0]
is allowed, and produces an empty array.
This will still evaluate
expr
, however, and immediately drop the resulting value, so
be mindful of side effects.
Arrays of
any
size implement the following traits if the element type allows it:
Copy
Clone
Debug
IntoIterator
(implemented for
[T; N]
,
&[T; N]
and
&mut [T; N]
)
PartialEq
,
PartialOrd
,
Eq
,
Ord
Hash
AsRef
,
AsMut
Borrow
,
BorrowMut
Arrays of sizes from 0 to 32 (inclusive) implement the
Default
trait
if the element type allows it. As a stopgap, trait implementations are
statically generated up to size 32.
Arrays of sizes from 1 to 12 (inclusive) implement
From<Tuple>
, where
Tuple
is a homogeneous
tuple
of appropriate length.
Arrays coerce to
slices (
[T]
)
, so a slice method may be called on
an array. Indeed, this provides most of the API for working with arrays.
Slices have a dynamic size and do not coerce to arrays. Instead, use
slice.try_into().unwrap()
or
<ArrayType>::try_from(slice).unwrap()
.
Array’s
try_from(slice)
implementations (and the corresponding
slice.try_into()
array implementations) succeed if the input slice length is the same as the result
array length. They optimize especially well when the optimizer can easily determine
the slice length, e.g.
<[u8; 4]>::try_from(&slice[4..8]).unwrap()
. Array implements
TryFrom
returning:
[T; N]
copies from the slice’s elements
&[T; N]
references the original slice’s elements
&mut [T; N]
references the original slice’s elements
You can move elements out of an array with a
slice pattern
. If you want
one element, see
mem::replace
.
§
Examples
let
mut
array: [i32;
3
] = [
0
;
3
];

array[
1
] =
1
;
array[
2
] =
2
;
assert_eq!
([
1
,
2
],
&
array[
1
..]);
// This loop prints: 0 1 2
for
x
in
array {
print!
(
"{x} "
);
}
You can also iterate over reference to the array’s elements:
let
array: [i32;
3
] = [
0
;
3
];
for
x
in
&
array { }
You can use
<ArrayType>::try_from(slice)
or
slice.try_into()
to get an array from
a slice:
let
bytes: [u8;
3
] = [
1
,
0
,
2
];
assert_eq!
(
1
, u16::from_le_bytes(<[u8;
2
]>::try_from(
&
bytes[
0
..
2
]).unwrap()));
assert_eq!
(
512
, u16::from_le_bytes(bytes[
1
..
3
].try_into().unwrap()));
You can use a
slice pattern
to move elements out of an array:
fn
move_away(
_
: String) {
/* Do interesting things. */
}
let
[john, roa] = [
"John"
.to_string(),
"Roa"
.to_string()];
move_away(john);
move_away(roa);
Arrays can be created from homogeneous tuples of appropriate length:
let
tuple: (u32, u32, u32) = (
1
,
2
,
3
);
let
array: [u32;
3
] = tuple.into();
§
Editions
Prior to Rust 1.53, arrays did not implement
IntoIterator
by value, so the method call
array.into_iter()
auto-referenced into a
slice iterator
. Right now, the old
behavior is preserved in the 2015 and 2018 editions of Rust for compatibility, ignoring
IntoIterator
by value. In the future, the behavior on the 2015 and 2018 edition
might be made consistent to the behavior of later editions.
ⓘ
// Rust 2015 and 2018:
let
array: [i32;
3
] = [
0
;
3
];
// This creates a slice iterator, producing references to each value.
for
item
in
array.into_iter().enumerate() {
let
(i, x): (usize,
&
i32) = item;
println!
(
"array[{i}] = {x}"
);
}
// The `array_into_iter` lint suggests this change for future compatibility:
for
item
in
array.iter().enumerate() {
let
(i, x): (usize,
&
i32) = item;
println!
(
"array[{i}] = {x}"
);
}
// You can explicitly iterate an array by value using `IntoIterator::into_iter`
for
item
in
IntoIterator::into_iter(array).enumerate() {
let
(i, x): (usize, i32) = item;
println!
(
"array[{i}] = {x}"
);
}
Starting in the 2021 edition,
array.into_iter()
uses
IntoIterator
normally to iterate
by value, and
iter()
should be used to iterate by reference like previous editions.
ⓘ
// Rust 2021:
let
array: [i32;
3
] = [
0
;
3
];
// This iterates by reference:
for
item
in
array.iter().enumerate() {
let
(i, x): (usize,
&
i32) = item;
println!
(
"array[{i}] = {x}"
);
}
// This iterates by value:
for
item
in
array.into_iter().enumerate() {
let
(i, x): (usize, i32) = item;
println!
(
"array[{i}] = {x}"
);
}
Future language versions might start treating the
array.into_iter()
syntax on editions 2015 and 2018 the same as on edition 2021. So code using
those older editions should still be written with this change in mind, to
prevent breakage in the future. The safest way to accomplish this is to
avoid the
into_iter
syntax on those editions. If an edition update is not
viable/desired, there are multiple alternatives:
use
iter
, equivalent to the old behavior, creating references
use
IntoIterator::into_iter
, equivalent to the post-2021 behavior (Rust 1.53+)
replace
for ... in array.into_iter() {
with
for ... in array {
,
equivalent to the post-2021 behavior (Rust 1.53+)
ⓘ
// Rust 2015 and 2018:
let
array: [i32;
3
] = [
0
;
3
];
// This iterates by reference:
for
item
in
array.iter() {
let
x:
&
i32 = item;
println!
(
"{x}"
);
}
// This iterates by value:
for
item
in
IntoIterator::into_iter(array) {
let
x: i32 = item;
println!
(
"{x}"
);
}
// This iterates by value:
for
item
in
array {
let
x: i32 = item;
println!
(
"{x}"
);
}
// IntoIter can also start a chain.
// This iterates by value:
for
item
in
IntoIterator::into_iter(array).enumerate() {
let
(i, x): (usize, i32) = item;
println!
(
"array[{i}] = {x}"
);
}
Implementations
§
Source
§
impl<T, const N:
usize
> [
MaybeUninit
<T>;
N
]
Source
pub const fn
transpose
(self) ->
MaybeUninit
<
[T; N]
>
🔬
This is a nightly-only experimental API. (
maybe_uninit_uninit_array_transpose
#96097
)
Transposes a
[MaybeUninit<T>; N]
into a
MaybeUninit<[T; N]>
.
§
Examples
#![feature(maybe_uninit_uninit_array_transpose)]
let
data = [MaybeUninit::<u8>::uninit();
1000
];
let
data: MaybeUninit<[u8;
1000
]> = data.transpose();
Source
§
impl<const N:
usize
> [
u8
;
N
]
Source
pub const fn
as_ascii
(&self) ->
Option
<&[
AsciiChar
;
N
]>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Converts this array of bytes into an array of ASCII characters,
or returns
None
if any of the characters is non-ASCII.
§
Examples
#![feature(ascii_char)]
const
HEX_DIGITS: [std::ascii::Char;
16
] =
*
b"0123456789abcdef"
.as_ascii().unwrap();
assert_eq!
(HEX_DIGITS[
1
].as_str(),
"1"
);
assert_eq!
(HEX_DIGITS[
10
].as_str(),
"a"
);
Source
pub const unsafe fn
as_ascii_unchecked
(&self) -> &[
AsciiChar
;
N
]
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Converts this array of bytes into an array of ASCII characters,
without checking whether they’re valid.
§
Safety
Every byte in the array must be in
0..=127
, or else this is UB.
Source
§
impl<T, const N:
usize
>
[T; N]
1.55.0
·
Source
pub fn
map
<F, U>(self, f: F) ->
[U; N]
where
    F:
FnMut
(T) -> U,
Returns an array of the same size as
self
, with function
f
applied to each element
in order.
If you don’t necessarily need a new fixed-size array, consider using
Iterator::map
instead.
§
Note on performance and stack usage
Unfortunately, usages of this method are currently not always optimized
as well as they could be. This mainly concerns large arrays, as mapping
over small arrays seem to be optimized just fine. Also note that in
debug mode (i.e. without any optimizations), this method can use a lot
of stack space (a few times the size of the array or more).
Therefore, in performance-critical code, try to avoid using this method
on large arrays or check the emitted code. Also try to avoid chained
maps (e.g.
arr.map(...).map(...)
).
In many cases, you can instead use
Iterator::map
by calling
.iter()
or
.into_iter()
on your array.
[T; N]::map
is only necessary if you
really need a new array of the same size as the result. Rust’s lazy
iterators tend to get optimized very well.
§
Examples
let
x = [
1
,
2
,
3
];
let
y = x.map(|v| v +
1
);
assert_eq!
(y, [
2
,
3
,
4
]);
let
x = [
1
,
2
,
3
];
let
mut
temp =
0
;
let
y = x.map(|v| { temp +=
1
; v * temp });
assert_eq!
(y, [
1
,
4
,
9
]);
let
x = [
"Ferris"
,
"Bueller's"
,
"Day"
,
"Off"
];
let
y = x.map(|v| v.len());
assert_eq!
(y, [
6
,
9
,
3
,
3
]);
Source
pub fn
try_map
<R>(
    self,
    f: impl
FnMut
(T) -> R,
) -> <<R as
Try
>::
Residual
as
Residual
<[<R as
Try
>::
Output
;
N
]>>::
TryType
where
    R:
Try
,
    <R as
Try
>::
Residual
:
Residual
<[<R as
Try
>::
Output
;
N
]>,
🔬
This is a nightly-only experimental API. (
array_try_map
#79711
)
A fallible function
f
applied to each element on array
self
in order to
return an array the same size as
self
or the first error encountered.
The return type of this function depends on the return type of the closure.
If you return
Result<T, E>
from the closure, you’ll get a
Result<[T; N], E>
.
If you return
Option<T>
from the closure, you’ll get an
Option<[T; N]>
.
§
Examples
#![feature(array_try_map)]
let
a = [
"1"
,
"2"
,
"3"
];
let
b = a.try_map(|v| v.parse::<u32>()).unwrap().map(|v| v +
1
);
assert_eq!
(b, [
2
,
3
,
4
]);
let
a = [
"1"
,
"2a"
,
"3"
];
let
b = a.try_map(|v| v.parse::<u32>());
assert!
(b.is_err());
use
std::num::NonZero;
let
z = [
1
,
2
,
0
,
3
,
4
];
assert_eq!
(z.try_map(NonZero::new),
None
);
let
a = [
1
,
2
,
3
];
let
b = a.try_map(NonZero::new);
let
c = b.map(|x| x.map(NonZero::get));
assert_eq!
(c,
Some
(a));
1.57.0 (const: 1.57.0)
·
Source
pub const fn
as_slice
(&self) -> &
[T]
Returns a slice containing the entire array. Equivalent to
&s[..]
.
1.57.0 (const:
unstable
)
·
Source
pub fn
as_mut_slice
(&mut self) -> &mut
[T]
Returns a mutable slice containing the entire array. Equivalent to
&mut s[..]
.
1.77.0 (const:
unstable
)
·
Source
pub fn
each_ref
(&self) -> [
&T
;
N
]
Borrows each element and returns an array of references with the same
size as
self
.
§
Example
let
floats = [
3.1
,
2.7
, -
1.0
];
let
float_refs: [
&
f64;
3
] = floats.each_ref();
assert_eq!
(float_refs, [
&
3.1
,
&
2.7
,
&
-
1.0
]);
This method is particularly useful if combined with other methods, like
map
. This way, you can avoid moving the original
array if its elements are not
Copy
.
let
strings = [
"Ferris"
.to_string(),
"♥"
.to_string(),
"Rust"
.to_string()];
let
is_ascii = strings.each_ref().map(|s| s.is_ascii());
assert_eq!
(is_ascii, [
true
,
false
,
true
]);
// We can still access the original array: it has not been moved.
assert_eq!
(strings.len(),
3
);
1.77.0 (const:
unstable
)
·
Source
pub fn
each_mut
(&mut self) -> [
&mut T
;
N
]
Borrows each element mutably and returns an array of mutable references
with the same size as
self
.
§
Example
let
mut
floats = [
3.1
,
2.7
, -
1.0
];
let
float_refs: [
&mut
f64;
3
] = floats.each_mut();
*
float_refs[
0
] =
0.0
;
assert_eq!
(float_refs, [
&mut
0.0
,
&mut
2.7
,
&mut
-
1.0
]);
assert_eq!
(floats, [
0.0
,
2.7
, -
1.0
]);
Source
pub fn
split_array_ref
<const M:
usize
>(&self) -> (&
[T; M]
, &
[T]
)
🔬
This is a nightly-only experimental API. (
split_array
#90091
)
Divides one array reference into two at an index.
The first will contain all indices from
[0, M)
(excluding
the index
M
itself) and the second will contain all
indices from
[M, N)
(excluding the index
N
itself).
§
Panics
Panics if
M > N
.
§
Examples
#![feature(split_array)]
let
v = [
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

{
let
(left, right) = v.split_array_ref::<
0
>();
assert_eq!
(left,
&
[]);
assert_eq!
(right,
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
5
,
6
]);
}

{
let
(left, right) = v.split_array_ref::<
2
>();
assert_eq!
(left,
&
[
1
,
2
]);
assert_eq!
(right,
&
[
3
,
4
,
5
,
6
]);
}

{
let
(left, right) = v.split_array_ref::<
6
>();
assert_eq!
(left,
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
5
,
6
]);
assert_eq!
(right,
&
[]);
}
Source
pub fn
split_array_mut
<const M:
usize
>(&mut self) -> (&mut
[T; M]
, &mut
[T]
)
🔬
This is a nightly-only experimental API. (
split_array
#90091
)
Divides one mutable array reference into two at an index.
The first will contain all indices from
[0, M)
(excluding
the index
M
itself) and the second will contain all
indices from
[M, N)
(excluding the index
N
itself).
§
Panics
Panics if
M > N
.
§
Examples
#![feature(split_array)]
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
let
(left, right) = v.split_array_mut::<
2
>();
assert_eq!
(left,
&mut
[
1
,
0
][..]);
assert_eq!
(right,
&mut
[
3
,
0
,
5
,
6
]);
left[
1
] =
2
;
right[
1
] =
4
;
assert_eq!
(v, [
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
]);
Source
pub fn
rsplit_array_ref
<const M:
usize
>(&self) -> (&
[T]
, &
[T; M]
)
🔬
This is a nightly-only experimental API. (
split_array
#90091
)
Divides one array reference into two at an index from the end.
The first will contain all indices from
[0, N - M)
(excluding
the index
N - M
itself) and the second will contain all
indices from
[N - M, N)
(excluding the index
N
itself).
§
Panics
Panics if
M > N
.
§
Examples
#![feature(split_array)]
let
v = [
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

{
let
(left, right) = v.rsplit_array_ref::<
0
>();
assert_eq!
(left,
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
5
,
6
]);
assert_eq!
(right,
&
[]);
}

{
let
(left, right) = v.rsplit_array_ref::<
2
>();
assert_eq!
(left,
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
assert_eq!
(right,
&
[
5
,
6
]);
}

{
let
(left, right) = v.rsplit_array_ref::<
6
>();
assert_eq!
(left,
&
[]);
assert_eq!
(right,
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
5
,
6
]);
}
Source
pub fn
rsplit_array_mut
<const M:
usize
>(&mut self) -> (&mut
[T]
, &mut
[T; M]
)
🔬
This is a nightly-only experimental API. (
split_array
#90091
)
Divides one mutable array reference into two at an index from the end.
The first will contain all indices from
[0, N - M)
(excluding
the index
N - M
itself) and the second will contain all
indices from
[N - M, N)
(excluding the index
N
itself).
§
Panics
Panics if
M > N
.
§
Examples
#![feature(split_array)]
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
let
(left, right) = v.rsplit_array_mut::<
4
>();
assert_eq!
(left,
&mut
[
1
,
0
]);
assert_eq!
(right,
&mut
[
3
,
0
,
5
,
6
][..]);
left[
1
] =
2
;
right[
1
] =
4
;
assert_eq!
(v, [
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
]);
Source
§
impl<T, const N:
usize
> [
Option
<T>;
N
]
Source
pub fn
transpose
(self) ->
Option
<
[T; N]
>
🔬
This is a nightly-only experimental API. (
option_array_transpose
#130828
)
Transposes a
[Option<T>; N]
into a
Option<[T; N]>
.
§
Examples
#![feature(option_array_transpose)]
let
data = [
Some
(
0
);
1000
];
let
data:
Option
<[u8;
1000
]> = data.transpose();
assert_eq!
(data,
Some
([
0
;
1000
]));
let
data = [
Some
(
0
),
None
];
let
data:
Option
<[u8;
2
]> = data.transpose();
assert_eq!
(data,
None
);
Trait Implementations
§
1.0.0
·
Source
§
impl<T, const N:
usize
>
AsMut
<
[T]
> for
[T; N]
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
1.0.0
·
Source
§
impl<T, const N:
usize
>
AsRef
<
[T]
> for
[T; N]
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
1.4.0
·
Source
§
impl<T, const N:
usize
>
Borrow
<
[T]
> for
[T; N]
Source
§
fn
borrow
(&self) -> &
[T]
Immutably borrows from an owned value.
Read more
1.4.0
·
Source
§
impl<T, const N:
usize
>
BorrowMut
<
[T]
> for
[T; N]
Source
§
fn
borrow_mut
(&mut self) -> &mut
[T]
Mutably borrows from an owned value.
Read more
1.58.0
·
Source
§
impl<T, const N:
usize
>
Clone
for
[T; N]
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
[T; N]
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, other: &
[T; N]
)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<T, const N:
usize
>
Debug
for
[T; N]
where
    T:
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
Formats the value using the given formatter.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 32]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 32]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 31]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 31]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 30]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 30]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 29]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 29]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 28]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 28]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 27]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 27]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 26]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 26]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 25]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 25]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 24]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 24]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 23]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 23]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 22]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 22]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 21]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 21]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 20]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 20]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 19]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 19]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 18]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 18]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 17]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 17]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 16]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 16]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 15]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 15]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 14]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 14]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 13]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 13]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 12]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 12]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 11]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 11]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 10]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 10]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 9]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 9]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 8]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 8]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 7]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 7]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 6]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 6]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 5]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 5]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 4]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 4]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 3]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 3]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 2]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 2]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 1]
where
    T:
Default
,
Source
§
fn
default
() ->
[T; 1]
Returns the “default value” for a type.
Read more
1.4.0
·
Source
§
impl<T>
Default
for
[T; 0]
Source
§
fn
default
() ->
[T; 0]
Returns the “default value” for a type.
Read more
1.77.0
·
Source
§
impl<'a, T, const N:
usize
>
From
<&'a
[T; N]
> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(s: &'a
[T; N]
) ->
Cow
<'a,
[T]
>
Creates a
Borrowed
variant of
Cow
from a reference to an array.
This conversion does not allocate or clone the data.
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<&
[T; N]
> for
Vec
<T>
where
    T:
Clone
,
Source
§
fn
from
(s: &
[T; N]
) ->
Vec
<T>
Allocates a
Vec<T>
and fills it by cloning
s
’s items.
§
Examples
assert_eq!
(Vec::from(
&
[
1
,
2
,
3
]),
vec!
[
1
,
2
,
3
]);
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<&mut
[T; N]
> for
Vec
<T>
where
    T:
Clone
,
Source
§
fn
from
(s: &mut
[T; N]
) ->
Vec
<T>
Allocates a
Vec<T>
and fills it by cloning
s
’s items.
§
Examples
assert_eq!
(Vec::from(
&mut
[
1
,
2
,
3
]),
vec!
[
1
,
2
,
3
]);
1.56.0
·
Source
§
impl<K, V, const N:
usize
>
From
<[
(K, V)
;
N
]> for
BTreeMap
<K, V>
where
    K:
Ord
,
Source
§
fn
from
(arr: [
(K, V)
;
N
]) ->
BTreeMap
<K, V>
Converts a
[(K, V); N]
into a
BTreeMap<K, V>
.
If any entries in the array have equal keys,
all but one of the corresponding values will be dropped.
use
std::collections::BTreeMap;
let
map1 = BTreeMap::from([(
1
,
2
), (
3
,
4
)]);
let
map2: BTreeMap<
_
,
_
> = [(
1
,
2
), (
3
,
4
)].into();
assert_eq!
(map1, map2);
1.56.0
·
Source
§
impl<K, V, const N:
usize
>
From
<[
(K, V)
;
N
]> for
HashMap
<K, V,
RandomState
>
where
    K:
Eq
+
Hash
,
Source
§
fn
from
(arr: [
(K, V)
;
N
]) -> Self
Converts a
[(K, V); N]
into a
HashMap<K, V>
.
If any entries in the array have equal keys,
all but one of the corresponding values will be dropped.
§
Examples
use
std::collections::HashMap;
let
map1 = HashMap::from([(
1
,
2
), (
3
,
4
)]);
let
map2: HashMap<
_
,
_
> = [(
1
,
2
), (
3
,
4
)].into();
assert_eq!
(map1, map2);
1.71.0
·
Source
§
impl<T>
From
<
[T; N]
> for
(T₁, T₂, …, Tₙ)
This trait is implemented for tuples up to twelve items long.
Source
§
fn
from
(array:
[T; 1]
) ->
(T,)
Converts to this type from the input type.
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Arc
<
[T]
>
Source
§
fn
from
(v:
[T; N]
) ->
Arc
<
[T]
>
Converts a
[T; N]
into an
Arc<[T]>
.
The conversion moves the array into a newly allocated
Arc
.
§
Example
let
original: [i32;
3
] = [
1
,
2
,
3
];
let
shared: Arc<[i32]> = Arc::from(original);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
BTreeSet
<T>
where
    T:
Ord
,
Source
§
fn
from
(arr:
[T; N]
) ->
BTreeSet
<T>
Converts a
[T; N]
into a
BTreeSet<T>
.
If the array contains any equal values,
all but one will be dropped.
§
Examples
use
std::collections::BTreeSet;
let
set1 = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
set2: BTreeSet<
_
> = [
1
,
2
,
3
,
4
].into();
assert_eq!
(set1, set2);
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
BinaryHeap
<T>
where
    T:
Ord
,
Source
§
fn
from
(arr:
[T; N]
) ->
BinaryHeap
<T>
use
std::collections::BinaryHeap;
let
mut
h1 = BinaryHeap::from([
1
,
4
,
2
,
3
]);
let
mut
h2: BinaryHeap<
_
> = [
1
,
4
,
2
,
3
].into();
while let
Some
((a, b)) = h1.pop().zip(h2.pop()) {
assert_eq!
(a, b);
}
1.45.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Box
<
[T]
>
Source
§
fn
from
(array:
[T; N]
) ->
Box
<
[T]
>
Converts a
[T; N]
into a
Box<[T]>
This conversion moves the array to newly heap-allocated memory.
§
Examples
let
boxed: Box<[u8]> = Box::from([
4
,
2
]);
println!
(
"{boxed:?}"
);
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
HashSet
<T,
RandomState
>
where
    T:
Eq
+
Hash
,
Source
§
fn
from
(arr:
[T; N]
) -> Self
Converts a
[T; N]
into a
HashSet<T>
.
If the array contains any equal values,
all but one will be dropped.
§
Examples
use
std::collections::HashSet;
let
set1 = HashSet::from([
1
,
2
,
3
,
4
]);
let
set2: HashSet<
_
> = [
1
,
2
,
3
,
4
].into();
assert_eq!
(set1, set2);
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
LinkedList
<T>
Source
§
fn
from
(arr:
[T; N]
) ->
LinkedList
<T>
Converts a
[T; N]
into a
LinkedList<T>
.
use
std::collections::LinkedList;
let
list1 = LinkedList::from([
1
,
2
,
3
,
4
]);
let
list2: LinkedList<
_
> = [
1
,
2
,
3
,
4
].into();
assert_eq!
(list1, list2);
1.74.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Rc
<
[T]
>
Source
§
fn
from
(v:
[T; N]
) ->
Rc
<
[T]
>
Converts a
[T; N]
into an
Rc<[T]>
.
The conversion moves the array into a newly allocated
Rc
.
§
Example
let
original: [i32;
3
] = [
1
,
2
,
3
];
let
shared: Rc<[i32]> = Rc::from(original);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
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
1.44.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
Vec
<T>
Source
§
fn
from
(s:
[T; N]
) ->
Vec
<T>
Allocates a
Vec<T>
and moves
s
’s items into it.
§
Examples
assert_eq!
(Vec::from([
1
,
2
,
3
]),
vec!
[
1
,
2
,
3
]);
1.56.0
·
Source
§
impl<T, const N:
usize
>
From
<
[T; N]
> for
VecDeque
<T>
Source
§
fn
from
(arr:
[T; N]
) ->
VecDeque
<T>
Converts a
[T; N]
into a
VecDeque<T>
.
use
std::collections::VecDeque;
let
deq1 = VecDeque::from([
1
,
2
,
3
,
4
]);
let
deq2: VecDeque<
_
> = [
1
,
2
,
3
,
4
].into();
assert_eq!
(deq1, deq2);
Source
§
impl<T, const N:
usize
>
From
<[
bool
;
N
]> for
Mask
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
(array: [
bool
;
N
]) ->
Mask
<T, N>
Converts to this type from the input type.
1.17.0
·
Source
§
impl
From
<[
u16
;
8
]> for
IpAddr
Source
§
fn
from
(segments: [
u16
;
8
]) ->
IpAddr
Creates an
IpAddr::V6
from an eight element 16-bit array.
§
Examples
use
std::net::{IpAddr, Ipv6Addr};
let
addr = IpAddr::from([
0x20du16
,
0x20cu16
,
0x20bu16
,
0x20au16
,
0x209u16
,
0x208u16
,
0x207u16
,
0x206u16
,
]);
assert_eq!
(
    IpAddr::V6(Ipv6Addr::new(
0x20d
,
0x20c
,
0x20b
,
0x20a
,
0x209
,
0x208
,
0x207
,
0x206
,
    )),
    addr
);
1.16.0
·
Source
§
impl
From
<[
u16
;
8
]> for
Ipv6Addr
Source
§
fn
from
(segments: [
u16
;
8
]) ->
Ipv6Addr
Creates an
Ipv6Addr
from an eight element 16-bit array.
§
Examples
use
std::net::Ipv6Addr;
let
addr = Ipv6Addr::from([
0x20du16
,
0x20cu16
,
0x20bu16
,
0x20au16
,
0x209u16
,
0x208u16
,
0x207u16
,
0x206u16
,
]);
assert_eq!
(
    Ipv6Addr::new(
0x20d
,
0x20c
,
0x20b
,
0x20a
,
0x209
,
0x208
,
0x207
,
0x206
,
    ),
    addr
);
1.17.0
·
Source
§
impl
From
<[
u8
;
16
]> for
IpAddr
Source
§
fn
from
(octets: [
u8
;
16
]) ->
IpAddr
Creates an
IpAddr::V6
from a sixteen element byte array.
§
Examples
use
std::net::{IpAddr, Ipv6Addr};
let
addr = IpAddr::from([
0x19u8
,
0x18u8
,
0x17u8
,
0x16u8
,
0x15u8
,
0x14u8
,
0x13u8
,
0x12u8
,
0x11u8
,
0x10u8
,
0x0fu8
,
0x0eu8
,
0x0du8
,
0x0cu8
,
0x0bu8
,
0x0au8
,
]);
assert_eq!
(
    IpAddr::V6(Ipv6Addr::new(
0x1918
,
0x1716
,
0x1514
,
0x1312
,
0x1110
,
0x0f0e
,
0x0d0c
,
0x0b0a
,
    )),
    addr
);
1.9.0
·
Source
§
impl
From
<[
u8
;
16
]> for
Ipv6Addr
Source
§
fn
from
(octets: [
u8
;
16
]) ->
Ipv6Addr
Creates an
Ipv6Addr
from a sixteen element byte array.
§
Examples
use
std::net::Ipv6Addr;
let
addr = Ipv6Addr::from([
0x19u8
,
0x18u8
,
0x17u8
,
0x16u8
,
0x15u8
,
0x14u8
,
0x13u8
,
0x12u8
,
0x11u8
,
0x10u8
,
0x0fu8
,
0x0eu8
,
0x0du8
,
0x0cu8
,
0x0bu8
,
0x0au8
,
]);
assert_eq!
(
    Ipv6Addr::new(
0x1918
,
0x1716
,
0x1514
,
0x1312
,
0x1110
,
0x0f0e
,
0x0d0c
,
0x0b0a
,
    ),
    addr
);
1.17.0
·
Source
§
impl
From
<[
u8
;
4
]> for
IpAddr
Source
§
fn
from
(octets: [
u8
;
4
]) ->
IpAddr
Creates an
IpAddr::V4
from a four element byte array.
§
Examples
use
std::net::{IpAddr, Ipv4Addr};
let
addr = IpAddr::from([
13u8
,
12u8
,
11u8
,
10u8
]);
assert_eq!
(IpAddr::V4(Ipv4Addr::new(
13
,
12
,
11
,
10
)), addr);
1.9.0
·
Source
§
impl
From
<[
u8
;
4
]> for
Ipv4Addr
Source
§
fn
from
(octets: [
u8
;
4
]) ->
Ipv4Addr
Creates an
Ipv4Addr
from a four element byte array.
§
Examples
use
std::net::Ipv4Addr;
let
addr = Ipv4Addr::from([
13u8
,
12u8
,
11u8
,
10u8
]);
assert_eq!
(Ipv4Addr::new(
13
,
12
,
11
,
10
), addr);
1.71.0
·
Source
§
impl<T>
From
<
(T₁, T₂, …, Tₙ)
> for
[T; N]
This trait is implemented for tuples up to twelve items long.
Source
§
fn
from
(tuple:
(T,)
) ->
[T; 1]
Converts to this type from the input type.
Source
§
impl<T, const N:
usize
>
From
<
Mask
<T, N>> for [
bool
;
N
]
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
(vector:
Mask
<T, N>) -> [
bool
;
N
]
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
1.0.0
·
Source
§
impl<T, const N:
usize
>
Hash
for
[T; N]
where
    T:
Hash
,
The hash of an array is the same as that of the corresponding slice,
as required by the
Borrow
implementation.
use
std::hash::BuildHasher;
let
b = std::hash::RandomState::new();
let
a: [u8;
3
] = [
0xa8
,
0x3c
,
0x09
];
let
s:
&
[u8] =
&
[
0xa8
,
0x3c
,
0x09
];
assert_eq!
(b.hash_one(a), b.hash_one(s));
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
1.50.0
·
Source
§
impl<T, I, const N:
usize
>
Index
<I> for
[T; N]
where
[T]
:
Index
<I>,
Source
§
type
Output
= <
[T]
as
Index
<I>>::
Output
The returned type after indexing.
Source
§
fn
index
(&self, index: I) -> &<
[T; N]
as
Index
<I>>::
Output
Performs the indexing (
container[index]
) operation.
Read more
1.50.0
·
Source
§
impl<T, I, const N:
usize
>
IndexMut
<I> for
[T; N]
where
[T]
:
IndexMut
<I>,
Source
§
fn
index_mut
(&mut self, index: I) -> &mut <
[T; N]
as
Index
<I>>::
Output
Performs the mutable indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl<'a, T, const N:
usize
>
IntoIterator
for &'a
[T; N]
Source
§
type
Item
=
&'a T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
1.0.0
·
Source
§
impl<'a, T, const N:
usize
>
IntoIterator
for &'a mut
[T; N]
Source
§
type
Item
=
&'a mut T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IterMut
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
IterMut
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
1.53.0
·
Source
§
impl<T, const N:
usize
>
IntoIterator
for
[T; N]
Source
§
fn
into_iter
(self) -> <
[T; N]
as
IntoIterator
>::
IntoIter
Creates a consuming iterator, that is, one that moves each value out of
the array (from start to end).
The array cannot be used after calling this unless
T
implements
Copy
, so the whole array is copied.
Arrays have special behavior when calling
.into_iter()
prior to the
2021 edition – see the
array
Editions section for more information.
Source
§
type
Item
= T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IntoIter
<T, N>
Which kind of iterator are we turning this into?
1.0.0
·
Source
§
impl<T, const N:
usize
>
Ord
for
[T; N]
where
    T:
Ord
,
Implements comparison of arrays
lexicographically
.
Source
§
fn
cmp
(&self, other: &
[T; N]
) ->
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
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<&
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&
[U]
) ->
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
(&self, other: &&
[U]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&
[U; N]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&
[U; N]
) ->
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
(&self, other: &&
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&
[U; N]
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<&[
u8
;
N
]> for
ByteStr
Source
§
fn
eq
(&self, other: &&[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<&[
u8
;
N
]> for
ByteString
Source
§
fn
eq
(&self, other: &&[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<&mut
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&mut
[U]
) ->
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
(&self, other: &&mut
[U]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&mut
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&mut
[U; N]
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U]
> for
[T; N]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U]
) ->
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
[U]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for &
[T]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for &mut
[T]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for
[T]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, const N:
usize
>
PartialEq
<
[U; N]
> for
[T; N]
where
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<
[U; N]
> for
Vec
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
[U; N]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<[
u8
;
N
]> for
ByteStr
Source
§
fn
eq
(&self, other: &[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<[
u8
;
N
]> for
ByteString
Source
§
fn
eq
(&self, other: &[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteStr
> for &[
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteStr
> for [
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for &[
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for [
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, const N:
usize
>
PartialOrd
for
[T; N]
where
    T:
PartialOrd
,
Implements comparison of arrays
lexicographically
.
Source
§
fn
partial_cmp
(&self, other: &
[T; N]
) ->
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
Source
§
fn
lt
(&self, other: &
[T; N]
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
Source
§
fn
le
(&self, other: &
[T; N]
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
Source
§
fn
ge
(&self, other: &
[T; N]
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
fn
gt
(&self, other: &
[T; N]
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
Source
§
impl<'b, const N:
usize
>
Pattern
for &'b [
char
;
N
]
Searches for chars that are equal to any of the
char
s in the array.
§
Examples
assert_eq!
(
"Hello world"
.find(
&
[
'o'
,
'l'
]),
Some
(
2
));
assert_eq!
(
"Hello world"
.find(
&
[
'h'
,
'w'
]),
Some
(
6
));
Source
§
type
Searcher
<'a> =
CharArrayRefSearcher
<'a, 'b, N>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
<'a>(self, haystack: &'a
str
) ->
CharArrayRefSearcher
<'a, 'b, N>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
CharArrayRefSearcher
<'a, 'b, N>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
CharArrayRefSearcher
<'a, 'b, N>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
Source
§
impl<const N:
usize
>
Pattern
for [
char
;
N
]
Searches for chars that are equal to any of the
char
s in the array.
§
Examples
assert_eq!
(
"Hello world"
.find([
'o'
,
'l'
]),
Some
(
2
));
assert_eq!
(
"Hello world"
.find([
'h'
,
'w'
]),
Some
(
6
));
Source
§
type
Searcher
<'a> =
CharArraySearcher
<'a, N>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
<'a>(self, haystack: &'a
str
) ->
CharArraySearcher
<'a, N>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
CharArraySearcher
<'a, N>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
CharArraySearcher
<'a, N>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
1.51.0
·
Source
§
impl<T, const N:
usize
>
SlicePattern
for
[T; N]
Source
§
type
Item
= T
🔬
This is a nightly-only experimental API. (
slice_pattern
#56345
)
The element type of the slice being matched on.
Source
§
fn
as_slice
(&self) -> &[<
[T; N]
as
SlicePattern
>::
Item
]
🔬
This is a nightly-only experimental API. (
slice_pattern
#56345
)
Currently, the consumers of
SlicePattern
need a slice.
1.34.0
·
Source
§
impl<'a, T, const N:
usize
>
TryFrom
<&'a
[T]
> for &'a
[T; N]
Tries to create an array ref
&[T; N]
from a slice ref
&[T]
. Succeeds if
slice.len() == N
.
let
bytes: [u8;
3
] = [
1
,
0
,
2
];
let
bytes_head:
&
[u8;
2
] = <
&
[u8;
2
]>::try_from(
&
bytes[
0
..
2
]).unwrap();
assert_eq!
(
1
, u16::from_le_bytes(
*
bytes_head));
let
bytes_tail:
&
[u8;
2
] = bytes[
1
..
3
].try_into().unwrap();
assert_eq!
(
512
, u16::from_le_bytes(
*
bytes_tail));
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
(slice: &'a
[T]
) ->
Result
<&'a
[T; N]
,
TryFromSliceError
>
Performs the conversion.
1.34.0
·
Source
§
impl<T, const N:
usize
>
TryFrom
<&
[T]
> for
[T; N]
where
    T:
Copy
,
Tries to create an array
[T; N]
by copying from a slice
&[T]
.
Succeeds if
slice.len() == N
.
let
bytes: [u8;
3
] = [
1
,
0
,
2
];
let
bytes_head: [u8;
2
] = <[u8;
2
]>::try_from(
&
bytes[
0
..
2
]).unwrap();
assert_eq!
(
1
, u16::from_le_bytes(bytes_head));
let
bytes_tail: [u8;
2
] = bytes[
1
..
3
].try_into().unwrap();
assert_eq!
(
512
, u16::from_le_bytes(bytes_tail));
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
[T; N]
,
TryFromSliceError
>
Performs the conversion.
1.34.0
·
Source
§
impl<'a, T, const N:
usize
>
TryFrom
<&'a mut
[T]
> for &'a mut
[T; N]
Tries to create a mutable array ref
&mut [T; N]
from a mutable slice ref
&mut [T]
. Succeeds if
slice.len() == N
.
let
mut
bytes: [u8;
3
] = [
1
,
0
,
2
];
let
bytes_head:
&mut
[u8;
2
] = <
&mut
[u8;
2
]>::try_from(
&mut
bytes[
0
..
2
]).unwrap();
assert_eq!
(
1
, u16::from_le_bytes(
*
bytes_head));
let
bytes_tail:
&mut
[u8;
2
] = (
&mut
bytes[
1
..
3
]).try_into().unwrap();
assert_eq!
(
512
, u16::from_le_bytes(
*
bytes_tail));
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
(slice: &'a mut
[T]
) ->
Result
<&'a mut
[T; N]
,
TryFromSliceError
>
Performs the conversion.
1.59.0
·
Source
§
impl<T, const N:
usize
>
TryFrom
<&mut
[T]
> for
[T; N]
where
    T:
Copy
,
Tries to create an array
[T; N]
by copying from a mutable slice
&mut [T]
.
Succeeds if
slice.len() == N
.
let
mut
bytes: [u8;
3
] = [
1
,
0
,
2
];
let
bytes_head: [u8;
2
] = <[u8;
2
]>::try_from(
&mut
bytes[
0
..
2
]).unwrap();
assert_eq!
(
1
, u16::from_le_bytes(bytes_head));
let
bytes_tail: [u8;
2
] = (
&mut
bytes[
1
..
3
]).try_into().unwrap();
assert_eq!
(
512
, u16::from_le_bytes(bytes_tail));
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
[T; N]
,
TryFromSliceError
>
Performs the conversion.
1.43.0
·
Source
§
impl<T, const N:
usize
>
TryFrom
<
Box
<
[T]
>> for
Box
<
[T; N]
>
Source
§
fn
try_from
(
    boxed_slice:
Box
<
[T]
>,
) ->
Result
<
Box
<
[T; N]
>, <
Box
<
[T; N]
> as
TryFrom
<
Box
<
[T]
>>>::
Error
>
Attempts to convert a
Box<[T]>
into a
Box<[T; N]>
.
The conversion occurs in-place and does not require a
new memory allocation.
§
Errors
Returns the old
Box<[T]>
in the
Err
variant if
boxed_slice.len()
does not equal
N
.
Source
§
type
Error
=
Box
<
[T]
>
The type returned in the event of a conversion error.
1.66.0
·
Source
§
impl<T, const N:
usize
>
TryFrom
<
Vec
<T>> for
Box
<
[T; N]
>
Source
§
fn
try_from
(
    vec:
Vec
<T>,
) ->
Result
<
Box
<
[T; N]
>, <
Box
<
[T; N]
> as
TryFrom
<
Vec
<T>>>::
Error
>
Attempts to convert a
Vec<T>
into a
Box<[T; N]>
.
Like
Vec::into_boxed_slice
, this is in-place if
vec.capacity() == N
,
but will require a reallocation otherwise.
§
Errors
Returns the original
Vec<T>
in the
Err
variant if
boxed_slice.len()
does not equal
N
.
§
Examples
This can be used with
vec!
to create an array on the heap:
let
state: Box<[f32;
100
]> =
vec!
[
1.0
;
100
].try_into().unwrap();
assert_eq!
(state.len(),
100
);
Source
§
type
Error
=
Vec
<T>
The type returned in the event of a conversion error.
1.48.0
·
Source
§
impl<T, A, const N:
usize
>
TryFrom
<
Vec
<T, A>> for
[T; N]
where
    A:
Allocator
,
Source
§
fn
try_from
(vec:
Vec
<T, A>) ->
Result
<
[T; N]
,
Vec
<T, A>>
Gets the entire contents of the
Vec<T>
as an array,
if its size exactly matches that of the requested array.
§
Examples
assert_eq!
(
vec!
[
1
,
2
,
3
].try_into(),
Ok
([
1
,
2
,
3
]));
assert_eq!
(<Vec<i32>>::new().try_into(),
Ok
([]));
If the length doesn’t match, the input comes back in
Err
:
let
r:
Result
<[i32;
4
],
_
> = (
0
..
10
).collect::<Vec<
_
>>().try_into();
assert_eq!
(r,
Err
(
vec!
[
0
,
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
,
7
,
8
,
9
]));
If you’re fine with just getting a prefix of the
Vec<T>
,
you can call
.truncate(N)
first.
let
mut
v = String::from(
"hello world"
).into_bytes();
v.sort();
v.truncate(
2
);
let
[a, b]: [
_
;
2
] = v.try_into().unwrap();
assert_eq!
(a,
b' '
);
assert_eq!
(b,
b'd'
);
Source
§
type
Error
=
Vec
<T, A>
The type returned in the event of a conversion error.
Source
§
impl<T, const N:
usize
>
ConstParamTy_
for
[T; N]
where
    T:
ConstParamTy_
,
1.58.0
·
Source
§
impl<T, const N:
usize
>
Copy
for
[T; N]
where
    T:
Copy
,
1.0.0
·
Source
§
impl<T, const N:
usize
>
Eq
for
[T; N]
where
    T:
Eq
,
Source
§
impl<T, const N:
usize
>
StructuralPartialEq
for
[T; N]
Source
§
impl<T, const N:
usize
>
UnsizedConstParamTy
for
[T; N]
where
    T:
UnsizedConstParamTy
,
Auto Trait Implementations
§
§
impl<T, const N:
usize
>
Freeze
for [
MaybeUninit
<T>;
N
]
where
    T:
Freeze
,
§
impl<T, const N:
usize
>
RefUnwindSafe
for [
MaybeUninit
<T>;
N
]
where
    T:
RefUnwindSafe
,
§
impl<T, const N:
usize
>
Send
for [
MaybeUninit
<T>;
N
]
where
    T:
Send
,
§
impl<T, const N:
usize
>
Sync
for [
MaybeUninit
<T>;
N
]
where
    T:
Sync
,
§
impl<T, const N:
usize
>
Unpin
for [
MaybeUninit
<T>;
N
]
where
    T:
Unpin
,
§
impl<T, const N:
usize
>
UnwindSafe
for [
MaybeUninit
<T>;
N
]
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