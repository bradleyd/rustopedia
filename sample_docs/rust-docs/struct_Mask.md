Mask in std::simd::prelude - Rust
std
::
simd
::
prelude
Struct
Mask
Copy item path
Source
pub struct Mask<T, const N:
usize
>(
/* private fields */
)
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
;
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
A SIMD vector mask for
N
elements of width specified by
Element
.
Masks represent boolean inclusion/exclusion on a per-element basis.
The layout of this type is unspecified, and may change between platforms
and/or Rust versions, and code should not assume that it is equivalent to
[T; N]
.
Implementations
§
Source
§
impl<T, const N:
usize
>
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
pub fn
reverse
(self) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Reverse the order of the elements in the mask.
Source
pub fn
rotate_elements_left
<const OFFSET:
usize
>(self) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rotates the mask such that the first
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
Source
pub fn
rotate_elements_right
<const OFFSET:
usize
>(self) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rotates the mask such that the first
self.len() - OFFSET
elements of the mask move to
the end while the last
OFFSET
elements move to the front. After calling
rotate_elements_right
,
the element previously at index
self.len() - OFFSET
will become the first element in the slice.
Source
pub fn
shift_elements_left
<const OFFSET:
usize
>(
    self,
    padding:
bool
,
) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Shifts the mask elements to the left by
OFFSET
, filling in with
padding
from the right.
Source
pub fn
shift_elements_right
<const OFFSET:
usize
>(
    self,
    padding:
bool
,
) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Shifts the mask elements to the right by
OFFSET
, filling in with
padding
from the left.
Source
pub fn
interleave
(self, other:
Mask
<T, N>) -> (
Mask
<T, N>,
Mask
<T, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Interleave two masks.
The resulting masks contain elements taken alternatively from
self
and
other
, first
filling the first result, and then the second.
The reverse of this operation is
Mask::deinterleave
.
let
a = mask32x4::from_array([
false
,
true
,
false
,
true
]);
let
b = mask32x4::from_array([
false
,
false
,
true
,
true
]);
let
(x, y) = a.interleave(b);
assert_eq!
(x.to_array(), [
false
,
false
,
true
,
false
]);
assert_eq!
(y.to_array(), [
false
,
true
,
true
,
true
]);
Source
pub fn
deinterleave
(self, other:
Mask
<T, N>) -> (
Mask
<T, N>,
Mask
<T, N>)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Deinterleave two masks.
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
Mask::interleave
.
let
a = mask32x4::from_array([
false
,
true
,
false
,
true
]);
let
b = mask32x4::from_array([
false
,
false
,
true
,
true
]);
let
(x, y) = a.deinterleave(b);
assert_eq!
(x.to_array(), [
false
,
false
,
false
,
true
]);
assert_eq!
(y.to_array(), [
true
,
true
,
false
,
true
]);
Source
pub fn
resize
<const M:
usize
>(self, value:
bool
) ->
Mask
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
Resize a mask.
If
M
>
N
, extends the length of a mask, setting the new elements to
value
.
If
M
<
N
, truncates the mask to the first
M
elements.
let
x = mask32x4::from_array([
false
,
true
,
true
,
false
]);
assert_eq!
(x.resize::<
8
>(
true
).to_array(), [
false
,
true
,
true
,
false
,
true
,
true
,
true
,
true
]);
assert_eq!
(x.resize::<
2
>(
true
).to_array(), [
false
,
true
]);
Source
pub fn
extract
<const START:
usize
, const LEN:
usize
>(self) ->
Mask
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
x = mask32x4::from_array([
false
,
true
,
true
,
false
]);
assert_eq!
(x.extract::<
1
,
2
>().to_array(), [
true
,
true
]);
Source
§
impl<T, const N:
usize
>
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
pub fn
splat
(value:
bool
) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Constructs a mask by setting all elements to the given value.
Source
pub fn
from_array
(array: [
bool
;
N
]) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an array of bools to a SIMD mask.
Source
pub fn
to_array
(self) -> [
bool
;
N
]
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts a SIMD mask to an array of bools.
Source
pub unsafe fn
from_int_unchecked
(value:
Simd
<T, N>) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts a vector of integers to a mask, where 0 represents
false
and -1
represents
true
.
§
Safety
All elements must be either 0 or -1.
Source
pub fn
from_int
(value:
Simd
<T, N>) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts a vector of integers to a mask, where 0 represents
false
and -1
represents
true
.
§
Panics
Panics if any element is not 0 or -1.
Source
pub fn
to_int
(self) ->
Simd
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts the mask to a vector of integers, where 0 represents
false
and -1
represents
true
.
Source
pub fn
cast
<U>(self) ->
Mask
<U, N>
where
    U:
MaskElement
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts the mask to a mask of any other element size.
Source
pub unsafe fn
test_unchecked
(&self, index:
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Tests the value of the specified element.
§
Safety
index
must be less than
self.len()
.
Source
pub fn
test
(&self, index:
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Tests the value of the specified element.
§
Panics
Panics if
index
is greater than or equal to the number of elements in the vector.
Source
pub unsafe fn
set_unchecked
(&mut self, index:
usize
, value:
bool
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Sets the value of the specified element.
§
Safety
index
must be less than
self.len()
.
Source
pub fn
set
(&mut self, index:
usize
, value:
bool
)
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Sets the value of the specified element.
§
Panics
Panics if
index
is greater than or equal to the number of elements in the vector.
Source
pub fn
any
(self) ->
bool
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true if any element is set, or false otherwise.
Source
pub fn
all
(self) ->
bool
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns true if all elements are set, or false otherwise.
Source
pub fn
to_bitmask
(self) ->
u64
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a bitmask from a mask.
Each bit is set if the corresponding element in the mask is
true
.
If the mask contains more than 64 elements, the bitmask is truncated to the first 64.
Source
pub fn
from_bitmask
(bitmask:
u64
) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a mask from a bitmask.
For each bit, if it is set, the corresponding element in the mask is set to
true
.
If the mask contains more than 64 elements, the remainder are set to
false
.
Source
pub fn
first_set
(self) ->
Option
<
usize
>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Finds the index of the first set element.
assert_eq!
(mask32x8::splat(
false
).first_set(),
None
);
assert_eq!
(mask32x8::splat(
true
).first_set(),
Some
(
0
));
let
mask = mask32x8::from_array([
false
,
true
,
false
,
false
,
true
,
false
,
false
,
true
]);
assert_eq!
(mask.first_set(),
Some
(
1
));
Source
§
impl<T, const N:
usize
>
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
pub fn
select
<U>(
    self,
    true_values:
Simd
<U, N>,
    false_values:
Simd
<U, N>,
) ->
Simd
<U, N>
where
    U:
SimdElement
<Mask = T>,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Choose elements from two vectors.
For each element in the mask, choose the corresponding element from
true_values
if
that element mask is true, and
false_values
if that element mask is false.
§
Examples
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
mask = Mask::from_array([
true
,
false
,
false
,
true
]);
let
c = mask.select(a, b);
assert_eq!
(c.to_array(), [
0
,
5
,
6
,
3
]);
Source
pub fn
select_mask
(
    self,
    true_values:
Mask
<T, N>,
    false_values:
Mask
<T, N>,
) ->
Mask
<T, N>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Choose elements from two masks.
For each element in the mask, choose the corresponding element from
true_values
if
that element mask is true, and
false_values
if that element mask is false.
§
Examples
let
a = Mask::<i32,
4
>::from_array([
true
,
true
,
false
,
false
]);
let
b = Mask::<i32,
4
>::from_array([
false
,
false
,
true
,
true
]);
let
mask = Mask::<i32,
4
>::from_array([
true
,
false
,
false
,
true
]);
let
c = mask.select_mask(a, b);
assert_eq!
(c.to_array(), [
true
,
false
,
true
,
false
]);
Trait Implementations
§
Source
§
impl<T, const N:
usize
>
BitAnd
<
Mask
<T, N>> for
bool
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
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
bool
> for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
bool
) ->
Mask
<T, N>
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
for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
&
operator.
Source
§
fn
bitand
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
Performs the
&
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAndAssign
<
bool
> for
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
bitand_assign
(&mut self, rhs:
bool
)
Performs the
&=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitAndAssign
for
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
bitand_assign
(&mut self, rhs:
Mask
<T, N>)
Performs the
&=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOr
<
Mask
<T, N>> for
bool
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
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
bool
> for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
bool
) ->
Mask
<T, N>
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
for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
|
operator.
Source
§
fn
bitor
(self, rhs:
Mask
<T, N>) ->
Mask
<T, N>
Performs the
|
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOrAssign
<
bool
> for
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
bitor_assign
(&mut self, rhs:
bool
)
Performs the
|=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitOrAssign
for
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
bitor_assign
(&mut self, rhs:
Mask
<T, N>)
Performs the
|=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXor
<
Mask
<T, N>> for
bool
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Mask
<T, N>) -> <
bool
as
BitXor
<
Mask
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
bool
> for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
bool
) -> <
Mask
<T, N> as
BitXor
<
bool
>>::
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
for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
^
operator.
Source
§
fn
bitxor
(self, rhs:
Mask
<T, N>) -> <
Mask
<T, N> as
BitXor
>::
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
BitXorAssign
<
bool
> for
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
bitxor_assign
(&mut self, rhs:
bool
)
Performs the
^=
operation.
Read more
Source
§
impl<T, const N:
usize
>
BitXorAssign
for
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
bitxor_assign
(&mut self, rhs:
Mask
<T, N>)
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
clone
(&self) ->
Mask
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
Mask
<T, N>
where
    T:
MaskElement
+
Debug
,
LaneCount
<N>:
SupportedLaneCount
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
Source
§
impl<T, const N:
usize
>
Default
for
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
default
() ->
Mask
<T, N>
Returns the “default value” for a type.
Read more
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
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
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
from
(value:
Mask
<
i16
, N>) ->
Mask
<
i32
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
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
from
(value:
Mask
<
i16
, N>) ->
Mask
<
i64
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
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
from
(value:
Mask
<
i16
, N>) ->
Mask
<
i8
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i16
, N>> for
Mask
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
from
(value:
Mask
<
i16
, N>) ->
Mask
<
isize
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
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
from
(value:
Mask
<
i32
, N>) ->
Mask
<
i16
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
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
from
(value:
Mask
<
i32
, N>) ->
Mask
<
i64
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
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
from
(value:
Mask
<
i32
, N>) ->
Mask
<
i8
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i32
, N>> for
Mask
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
from
(value:
Mask
<
i32
, N>) ->
Mask
<
isize
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
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
from
(value:
Mask
<
i64
, N>) ->
Mask
<
i16
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
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
from
(value:
Mask
<
i64
, N>) ->
Mask
<
i32
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
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
from
(value:
Mask
<
i64
, N>) ->
Mask
<
i8
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i64
, N>> for
Mask
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
from
(value:
Mask
<
i64
, N>) ->
Mask
<
isize
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
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
from
(value:
Mask
<
i8
, N>) ->
Mask
<
i16
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
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
from
(value:
Mask
<
i8
, N>) ->
Mask
<
i32
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
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
from
(value:
Mask
<
i8
, N>) ->
Mask
<
i64
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
i8
, N>> for
Mask
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
from
(value:
Mask
<
i8
, N>) ->
Mask
<
isize
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
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
from
(value:
Mask
<
isize
, N>) ->
Mask
<
i16
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
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
from
(value:
Mask
<
isize
, N>) ->
Mask
<
i32
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
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
from
(value:
Mask
<
isize
, N>) ->
Mask
<
i64
, N>
Converts to this type from the input type.
Source
§
impl<const N:
usize
>
From
<
Mask
<
isize
, N>> for
Mask
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
from
(value:
Mask
<
isize
, N>) ->
Mask
<
i8
, N>
Converts to this type from the input type.
Source
§
impl<T, const N:
usize
>
Not
for
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
type
Output
=
Mask
<T, N>
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) -> <
Mask
<T, N> as
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
PartialEq
for
Mask
<T, N>
where
    T:
MaskElement
+
PartialEq
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
eq
(&self, other: &
Mask
<T, N>) ->
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
impl<T, const N:
usize
>
PartialOrd
for
Mask
<T, N>
where
    T:
MaskElement
+
PartialOrd
,
LaneCount
<N>:
SupportedLaneCount
,
Source
§
fn
partial_cmp
(&self, other: &
Mask
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
impl<const N:
usize
>
SimdOrd
for
Mask
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
Mask
<
i16
, N>) ->
Mask
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
Mask
<
i16
, N>) ->
Mask
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
Mask
<
i16
, N>, max:
Mask
<
i16
, N>) ->
Mask
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
Mask
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
Mask
<
i32
, N>) ->
Mask
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
Mask
<
i32
, N>) ->
Mask
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
Mask
<
i32
, N>, max:
Mask
<
i32
, N>) ->
Mask
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
Mask
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
Mask
<
i64
, N>) ->
Mask
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
Mask
<
i64
, N>) ->
Mask
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
Mask
<
i64
, N>, max:
Mask
<
i64
, N>) ->
Mask
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
Mask
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
Mask
<
i8
, N>) ->
Mask
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
Mask
<
i8
, N>) ->
Mask
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
Mask
<
i8
, N>, max:
Mask
<
i8
, N>) ->
Mask
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
Mask
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
Mask
<
isize
, N>) ->
Mask
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
Mask
<
isize
, N>) ->
Mask
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
Mask
<
isize
, N>, max:
Mask
<
isize
, N>) ->
Mask
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
SimdPartialEq
for
Mask
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
<
i16
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
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
<
i32
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
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
<
i64
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
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
<
i8
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
SimdPartialOrd
for
Mask
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
<
i16
, N>) -> <
Mask
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
Mask
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
<
i32
, N>) -> <
Mask
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
Mask
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
<
i64
, N>) -> <
Mask
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
Mask
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
<
i8
, N>) -> <
Mask
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
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
Mask
<
isize
, N>,
) -> <
Mask
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
impl<T, const N:
usize
>
Copy
for
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
Auto Trait Implementations
§
§
impl<T, const N:
usize
>
Freeze
for
Mask
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
Mask
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
Mask
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
Mask
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
Mask
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
Mask
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