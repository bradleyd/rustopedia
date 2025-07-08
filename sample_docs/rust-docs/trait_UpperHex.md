UpperHex in std::fmt - Rust
std
::
fmt
Trait
UpperHex
Copy item path
1.0.0
·
Source
pub trait UpperHex {
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
X
formatting.
The
UpperHex
trait should format its output as a number in hexadecimal, with
A
through
F
in upper case.
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
// 42 is '2A' in hex
assert_eq!
(
format!
(
"{y:X}"
),
"2A"
);
assert_eq!
(
format!
(
"{y:#X}"
),
"0x2A"
);
assert_eq!
(
format!
(
"{:X}"
, -
16
),
"FFFFFFF0"
);
Implementing
UpperHex
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::UpperHex
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

        fmt::UpperHex::fmt(
&
val, f)
// delegate to i32's implementation
}
}
let
l = Length(i32::MAX);
assert_eq!
(
format!
(
"l as hex is: {l:X}"
),
"l as hex is: 7FFFFFFF"
);
assert_eq!
(
format!
(
"l as hex is: {l:#010X}"
),
"l as hex is: 0x7FFFFFFF"
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
UpperHex
for
i8
1.0.0
·
Source
§
impl
UpperHex
for
i16
1.0.0
·
Source
§
impl
UpperHex
for
i32
1.0.0
·
Source
§
impl
UpperHex
for
i64
1.0.0
·
Source
§
impl
UpperHex
for
i128
1.0.0
·
Source
§
impl
UpperHex
for
isize
1.0.0
·
Source
§
impl
UpperHex
for
u8
1.0.0
·
Source
§
impl
UpperHex
for
u16
1.0.0
·
Source
§
impl
UpperHex
for
u32
1.0.0
·
Source
§
impl
UpperHex
for
u64
1.0.0
·
Source
§
impl
UpperHex
for
u128
1.0.0
·
Source
§
impl
UpperHex
for
usize
1.0.0
·
Source
§
impl<T>
UpperHex
for
&T
where
    T:
UpperHex
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
UpperHex
for
&mut T
where
    T:
UpperHex
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
UpperHex
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
UpperHex
,
1.74.0
·
Source
§
impl<T>
UpperHex
for
Saturating
<T>
where
    T:
UpperHex
,
1.11.0
·
Source
§
impl<T>
UpperHex
for
Wrapping
<T>
where
    T:
UpperHex
,