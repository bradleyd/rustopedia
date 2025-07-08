try_from_fn in std::array - Rust
std
::
array
Function
try_from_fn
Copy item path
Source
pub fn try_from_fn<R, const N:
usize
, F>(
    cb: F,
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
    F:
FnMut
(
usize
) -> R,
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
array_try_from_fn
#89379
)
Expand description
Creates an array
[T; N]
where each fallible array element
T
is returned by the
cb
call.
Unlike
from_fn
, where the element creation can’t fail, this version will return an error
if any element creation was unsuccessful.
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
Arguments
cb
: Callback where the passed argument is the current array index.
§
Example
#![feature(array_try_from_fn)]
let
array:
Result
<[u8;
5
],
_
> = std::array::try_from_fn(|i| i.try_into());
assert_eq!
(array,
Ok
([
0
,
1
,
2
,
3
,
4
]));
let
array:
Result
<[i8;
200
],
_
> = std::array::try_from_fn(|i| i.try_into());
assert!
(array.is_err());
let
array:
Option
<[
_
;
4
]> = std::array::try_from_fn(|i| i.checked_add(
100
));
assert_eq!
(array,
Some
([
100
,
101
,
102
,
103
]));
let
array:
Option
<[
_
;
4
]> = std::array::try_from_fn(|i| i.checked_sub(
100
));
assert_eq!
(array,
None
);