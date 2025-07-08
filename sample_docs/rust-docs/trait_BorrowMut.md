BorrowMut in std::borrow - Rust
std
::
borrow
Trait
BorrowMut
Copy item path
1.0.0
·
Source
pub trait BorrowMut<Borrowed>:
Borrow
<Borrowed>
where
    Borrowed: ?
Sized
,
{
    // Required method
    fn
borrow_mut
(&mut self) ->
&mut Borrowed
;
}
Expand description
A trait for mutably borrowing data.
As a companion to
Borrow<T>
this trait allows a type to borrow as
an underlying type by providing a mutable reference. See
Borrow<T>
for more information on borrowing as another type.
Required Methods
§
1.0.0
·
Source
fn
borrow_mut
(&mut self) ->
&mut Borrowed
Mutably borrows from an owned value.
§
Examples
use
std::borrow::BorrowMut;
fn
check<T: BorrowMut<[i32]>>(
mut
v: T) {
assert_eq!
(
&mut
[
1
,
2
,
3
], v.borrow_mut());
}
let
v =
vec!
[
1
,
2
,
3
];

check(v);
Implementors
§
1.36.0
·
Source
§
impl
BorrowMut
<
str
> for
String
Source
§
impl
BorrowMut
<
ByteStr
> for
ByteString
Source
§
impl
BorrowMut
<[
u8
]> for
ByteStr
Source
§
impl
BorrowMut
<[
u8
]> for
ByteString
1.0.0
·
Source
§
impl<T>
BorrowMut
<T> for
&mut T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T, A>
BorrowMut
<
[T]
> for
Vec
<T, A>
where
    A:
Allocator
,
1.1.0
·
Source
§
impl<T, A>
BorrowMut
<T> for
Box
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A>
BorrowMut
<T> for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
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