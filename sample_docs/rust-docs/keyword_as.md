as - Rust
Keyword
as
Copy item path
Source
Expand description
Cast between types, or rename an import.
as
is most commonly used to turn primitive types into other primitive types, but it has other
uses that include turning pointers into addresses, addresses into pointers, and pointers into
other pointers.
let
thing1: u8 =
89.0
as
u8;
assert_eq!
(
'B'
as
u32,
66
);
assert_eq!
(thing1
as
char,
'Y'
);
let
thing2: f32 = thing1
as
f32 +
10.5
;
assert_eq!
(
true
as
u8 + thing2
as
u8,
100
);
In general, any cast that can be performed via ascribing the type can also be done using
as
,
so instead of writing
let x: u32 = 123
, you can write
let x = 123 as u32
(note:
let x: u32 = 123
would be best in that situation). The same is not true in the other direction, however;
explicitly using
as
allows a few more coercions that aren’t allowed implicitly, such as
changing the type of a raw pointer or turning closures into raw pointers.
as
can be seen as the primitive for
From
and
Into
:
as
only works  with primitives
(
u8
,
bool
,
str
, pointers, …) whereas
From
and
Into
also works with types like
String
or
Vec
.
as
can also be used with the
_
placeholder when the destination type can be inferred. Note
that this can cause inference breakage and usually such code should use an explicit type for
both clarity and stability. This is most useful when converting pointers using
as *const _
or
as *mut _
though the
cast
method is recommended over
as *const _
and it is
the same
for
as *mut _
: those methods make the intent clearer.
as
is also used to rename imports in
use
and
extern crate
statements:
use
std::{mem
as
memory, net
as
network};
// Now you can use the names `memory` and `network` to refer to `std::mem` and `std::net`.
For more information on what
as
is capable of, see the
Reference
.