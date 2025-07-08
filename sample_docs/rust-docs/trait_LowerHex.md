LowerHex in std::fmt - Rust
std
::
fmt
Trait
LowerHex
Copy item path
1.0.0
·
Source
pub trait LowerHex {
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
x
formatting.
The
LowerHex
trait should format its output as a number in hexadecimal, with
a
through
f
in lower case.
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
0x
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
y =
42
;
// 42 is '2a' in hex
assert_eq!
(
format!
(
"{y:x}"
),
"2a"
);
assert_eq!
(
format!
(
"{y:#x}"
),
"0x2a"
);
assert_eq!
(
format!
(
"{:x}"
, -
16
),
"fffffff0"
);
Implementing
LowerHex
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::LowerHex
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

        fmt::LowerHex::fmt(
&
val, f)
// delegate to i32's implementation
}
}
let
l = Length(
9
);
assert_eq!
(
format!
(
"l as hex is: {l:x}"
),
"l as hex is: 9"
);
assert_eq!
(
format!
(
"l as hex is: {l:#010x}"
),
"l as hex is: 0x00000009"
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
LowerHex
for
i8
1.0.0
·
Source
§
impl
LowerHex
for
i16
1.0.0
·
Source
§
impl
LowerHex
for
i32
1.0.0
·
Source
§
impl
LowerHex
for
i64
1.0.0
·
Source
§
impl
LowerHex
for
i128
1.0.0
·
Source
§
impl
LowerHex
for
isize
1.0.0
·
Source
§
impl
LowerHex
for
u8
1.0.0
·
Source
§
impl
LowerHex
for
u16
1.0.0
·
Source
§
impl
LowerHex
for
u32
1.0.0
·
Source
§
impl
LowerHex
for
u64
1.0.0
·
Source
§
impl
LowerHex
for
u128
1.0.0
·
Source
§
impl
LowerHex
for
usize
1.0.0
·
Source
§
impl<T>
LowerHex
for
&T
where
    T:
LowerHex
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
LowerHex
for
&mut T
where
    T:
LowerHex
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
LowerHex
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
LowerHex
,
1.74.0
·
Source
§
impl<T>
LowerHex
for
Saturating
<T>
where
    T:
LowerHex
,
1.11.0
·
Source
§
impl<T>
LowerHex
for
Wrapping
<T>
where
    T:
LowerHex
,