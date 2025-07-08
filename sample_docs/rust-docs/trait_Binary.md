Binary in std::fmt - Rust
std
::
fmt
Trait
Binary
Copy item path
1.0.0
·
Source
pub trait Binary {
    // Required method
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
>;
}
Expand description
b
formatting.
The
Binary
trait should format its output as a number in binary.
For primitive signed integers (
i8
to
i128
, and
isize
),
negative values are formatted as the two’s complement representation.
The alternate flag,
#
, adds a
0b
in front of the output.
For more information on formatters, see
the module-level documentation
.
§
Examples
Basic usage with
i32
:
let
x =
42
;
// 42 is '101010' in binary
assert_eq!
(
format!
(
"{x:b}"
),
"101010"
);
assert_eq!
(
format!
(
"{x:#b}"
),
"0b101010"
);
assert_eq!
(
format!
(
"{:b}"
, -
16
),
"11111111111111111111111111110000"
);
Implementing
Binary
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::Binary
for
Length {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
let
val =
self
.
0
;

        fmt::Binary::fmt(
&
val, f)
// delegate to i32's implementation
}
}
let
l = Length(
107
);
assert_eq!
(
format!
(
"l as binary is: {l:b}"
),
"l as binary is: 1101011"
);
assert_eq!
(
// Note that the `0b` prefix added by `#` is included in the total width, so we
    // need to add two to correctly display all 32 bits.
format!
(
"l as binary is: {l:#034b}"
),
"l as binary is: 0b00000000000000000000000001101011"
);
Required Methods
§
1.0.0
·
Source
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
§
Errors
This function should return
Err
if, and only if, the provided
Formatter
returns
Err
.
String formatting is considered an infallible operation; this function only
returns a
Result
because writing to the underlying stream might fail and it must
provide a way to propagate the fact that an error has occurred back up the stack.
Implementors
§
1.0.0
·
Source
§
impl
Binary
for
i8
1.0.0
·
Source
§
impl
Binary
for
i16
1.0.0
·
Source
§
impl
Binary
for
i32
1.0.0
·
Source
§
impl
Binary
for
i64
1.0.0
·
Source
§
impl
Binary
for
i128
1.0.0
·
Source
§
impl
Binary
for
isize
1.0.0
·
Source
§
impl
Binary
for
u8
1.0.0
·
Source
§
impl
Binary
for
u16
1.0.0
·
Source
§
impl
Binary
for
u32
1.0.0
·
Source
§
impl
Binary
for
u64
1.0.0
·
Source
§
impl
Binary
for
u128
1.0.0
·
Source
§
impl
Binary
for
usize
1.0.0
·
Source
§
impl<T>
Binary
for
&T
where
    T:
Binary
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Binary
for
&mut T
where
    T:
Binary
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
Binary
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Binary
,
1.74.0
·
Source
§
impl<T>
Binary
for
Saturating
<T>
where
    T:
Binary
,
1.11.0
·
Source
§
impl<T>
Binary
for
Wrapping
<T>
where
    T:
Binary
,