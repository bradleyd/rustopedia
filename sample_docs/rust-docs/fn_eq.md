eq in std::ptr - Rust
std
::
ptr
Function
eq
Copy item path
1.17.0
·
Source
pub fn eq<T>(a:
*const T
, b:
*const T
) ->
bool
where
    T: ?
Sized
,
Expand description
Compares raw pointers for equality.
This is the same as using the
==
operator, but less generic:
the arguments have to be
*const T
raw pointers,
not anything that implements
PartialEq
.
This can be used to compare
&T
references (which coerce to
*const T
implicitly)
by their address rather than comparing the values they point to
(which is what the
PartialEq for &T
implementation does).
When comparing wide pointers, both the address and the metadata are tested for equality.
However, note that comparing trait object pointers (
*const dyn Trait
) is unreliable: pointers
to values of the same underlying type can compare inequal (because vtables are duplicated in
multiple codegen units), and pointers to values of
different
underlying type can compare equal
(since identical vtables can be deduplicated within a codegen unit).
§
Examples
use
std::ptr;
let
five =
5
;
let
other_five =
5
;
let
five_ref =
&
five;
let
same_five_ref =
&
five;
let
other_five_ref =
&
other_five;
assert!
(five_ref == same_five_ref);
assert!
(ptr::eq(five_ref, same_five_ref));
assert!
(five_ref == other_five_ref);
assert!
(!ptr::eq(five_ref, other_five_ref));
Slices are also compared by their length (fat pointers):
let
a = [
1
,
2
,
3
];
assert!
(std::ptr::eq(
&
a[..
3
],
&
a[..
3
]));
assert!
(!std::ptr::eq(
&
a[..
2
],
&
a[..
3
]));
assert!
(!std::ptr::eq(
&
a[
0
..
2
],
&
a[
1
..
3
]));