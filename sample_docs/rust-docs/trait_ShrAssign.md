ShrAssign in std::ops - Rust
std
::
ops
Trait
ShrAssign
Copy item path
1.8.0
·
Source
pub trait ShrAssign<Rhs = Self> {
    // Required method
    fn
shr_assign
(&mut self, rhs: Rhs);
}
Expand description
The right shift assignment operator
>>=
.
§
Examples
An implementation of
ShrAssign
for a wrapper around
usize
.
use
std::ops::ShrAssign;
#[derive(Debug, PartialEq)]
struct
Scalar(usize);
impl
ShrAssign<usize>
for
Scalar {
fn
shr_assign(
&mut
self
, rhs: usize) {
self
.
0
>>= rhs;
    }
}
let
mut
scalar = Scalar(
16
);
scalar >>=
2
;
assert_eq!
(scalar, Scalar(
4
));
Required Methods
§
1.8.0
·
Source
fn
shr_assign
(&mut self, rhs: Rhs)
Performs the
>>=
operation.
§
Examples
let
mut
x: u8 =
5
;
x >>=
1
;
assert_eq!
(x,
2
);
let
mut
x: u8 =
2
;
x >>=
1
;
assert_eq!
(x,
1
);
Implementors
§
1.8.0
·
Source
§
impl
ShrAssign
for
i8
1.8.0
·
Source
§
impl
ShrAssign
for
i16
1.8.0
·
Source
§
impl
ShrAssign
for
i32
1.8.0
·
Source
§
impl
ShrAssign
for
i64
1.8.0
·
Source
§
impl
ShrAssign
for
i128
1.8.0
·
Source
§
impl
ShrAssign
for
isize
1.8.0
·
Source
§
impl
ShrAssign
for
u8
1.8.0
·
Source
§
impl
ShrAssign
for
u16
1.8.0
·
Source
§
impl
ShrAssign
for
u32
1.8.0
·
Source
§
impl
ShrAssign
for
u64
1.8.0
·
Source
§
impl
ShrAssign
for
u128
1.8.0
·
Source
§
impl
ShrAssign
for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
i8
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
i16
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
i32
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
i64
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
i128
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
isize
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
u8
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
u16
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
u32
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
u64
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
u128
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i8
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i16
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i32
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i64
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
i128
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
isize
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u8
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u16
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u32
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u64
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
u128
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
usize
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i8
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i16
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i32
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i64
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
i128
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
isize
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u8
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u16
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u32
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u64
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
u128
>
1.22.0
·
Source
§
impl
ShrAssign
<&
usize
> for
Wrapping
<
usize
>
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
i8
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
i16
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
i32
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
i64
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
i128
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
isize
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
u8
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
u16
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
u32
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
u64
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
u128
> for
usize
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
i8
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
i16
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
i32
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
i64
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
i128
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
isize
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
u8
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
u16
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
u32
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
u64
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
u128
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i8
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i16
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i32
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i64
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
i128
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
isize
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u8
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u16
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u32
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u64
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
u128
>
1.8.0
·
Source
§
impl
ShrAssign
<
usize
> for
Wrapping
<
usize
>
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