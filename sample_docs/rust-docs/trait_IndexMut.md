IndexMut in std::ops - Rust
std
::
ops
Trait
IndexMut
Copy item path
1.0.0
·
Source
pub trait IndexMut<Idx>:
Index
<Idx>
where
    Idx: ?
Sized
,
{
    // Required method
    fn
index_mut
(&mut self, index: Idx) -> &mut Self::
Output
;
}
Expand description
Used for indexing operations (
container[index]
) in mutable contexts.
container[index]
is actually syntactic sugar for
*container.index_mut(index)
, but only when used as a mutable value. If
an immutable value is requested, the
Index
trait is used instead. This
allows nice things such as
v[index] = value
.
§
Examples
A very simple implementation of a
Balance
struct that has two sides, where
each can be indexed mutably and immutably.
use
std::ops::{Index, IndexMut};
#[derive(Debug)]
enum
Side {
    Left,
    Right,
}
#[derive(Debug, PartialEq)]
enum
Weight {
    Kilogram(f32),
    Pound(f32),
}
struct
Balance {
pub
left: Weight,
pub
right: Weight,
}
impl
Index<Side>
for
Balance {
type
Output = Weight;
fn
index(
&
self
, index: Side) ->
&
Self
::Output {
println!
(
"Accessing {index:?}-side of balance immutably"
);
match
index {
            Side::Left =>
&
self
.left,
            Side::Right =>
&
self
.right,
        }
    }
}
impl
IndexMut<Side>
for
Balance {
fn
index_mut(
&mut
self
, index: Side) ->
&mut
Self
::Output {
println!
(
"Accessing {index:?}-side of balance mutably"
);
match
index {
            Side::Left =>
&mut
self
.left,
            Side::Right =>
&mut
self
.right,
        }
    }
}
let
mut
balance = Balance {
    right: Weight::Kilogram(
2.5
),
    left: Weight::Pound(
1.5
),
};
// In this case, `balance[Side::Right]` is sugar for
// `*balance.index(Side::Right)`, since we are only *reading*
// `balance[Side::Right]`, not writing it.
assert_eq!
(balance[Side::Right], Weight::Kilogram(
2.5
));
// However, in this case `balance[Side::Left]` is sugar for
// `*balance.index_mut(Side::Left)`, since we are writing
// `balance[Side::Left]`.
balance[Side::Left] = Weight::Kilogram(
3.0
);
Required Methods
§
1.0.0
·
Source
fn
index_mut
(&mut self, index: Idx) -> &mut Self::
Output
Performs the mutable indexing (
container[index]
) operation.
§
Panics
May panic if the index is out of bounds.
Implementors
§
Source
§
impl
IndexMut
<
usize
> for
ByteStr
Source
§
impl
IndexMut
<
usize
> for
ByteString
Source
§
impl
IndexMut
<
Range
<
usize
>> for
ByteStr
Source
§
impl
IndexMut
<
Range
<
usize
>> for
ByteString
Source
§
impl
IndexMut
<
RangeFrom
<
usize
>> for
ByteStr
Source
§
impl
IndexMut
<
RangeFrom
<
usize
>> for
ByteString
Source
§
impl
IndexMut
<
RangeFull
> for
ByteStr
Source
§
impl
IndexMut
<
RangeFull
> for
ByteString
1.44.0
·
Source
§
impl
IndexMut
<
RangeFull
> for
OsString
Source
§
impl
IndexMut
<
RangeInclusive
<
usize
>> for
ByteStr
Source
§
impl
IndexMut
<
RangeInclusive
<
usize
>> for
ByteString
Source
§
impl
IndexMut
<
RangeTo
<
usize
>> for
ByteStr
Source
§
impl
IndexMut
<
RangeTo
<
usize
>> for
ByteString
Source
§
impl
IndexMut
<
RangeToInclusive
<
usize
>> for
ByteStr
Source
§
impl
IndexMut
<
RangeToInclusive
<
usize
>> for
ByteString
1.0.0
·
Source
§
impl<I>
IndexMut
<I> for
str
where
    I:
SliceIndex
<
str
>,
1.0.0
·
Source
§
impl<I>
IndexMut
<I> for
String
where
    I:
SliceIndex
<
str
>,
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
1.0.0
·
Source
§
impl<T, A>
IndexMut
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, I>
IndexMut
<I> for
[T]
where
    I:
SliceIndex
<
[T]
>,
1.0.0
·
Source
§
impl<T, I, A>
IndexMut
<I> for
Vec
<T, A>
where
    I:
SliceIndex
<
[T]
>,
    A:
Allocator
,
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