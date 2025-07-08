addr_eq in std::ptr - Rust
std
::
ptr
Function
addr_eq
Copy item path
1.76.0
·
Source
pub fn addr_eq<T, U>(p:
*const T
, q:
*const U
) ->
bool
where
    T: ?
Sized
,
    U: ?
Sized
,
Expand description
Compares the
addresses
of the two pointers for equality,
ignoring any metadata in fat pointers.
If the arguments are thin pointers of the same type,
then this is the same as
eq
.
§
Examples
use
std::ptr;
let
whole:
&
[i32;
3
] =
&
[
1
,
2
,
3
];
let
first:
&
i32 =
&
whole[
0
];
assert!
(ptr::addr_eq(whole, first));
assert!
(!ptr::eq::<
dyn
std::fmt::Debug>(whole, first));