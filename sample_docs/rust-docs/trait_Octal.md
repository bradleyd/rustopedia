Octal in std::fmt - Rust
std
::
fmt
Trait
Octal
Copy item path
1.0.0
·
Source
pub trait Octal {
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
o
formatting.
The
Octal
trait should format its output as a number in base-8.
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
0o
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
// 42 is '52' in octal
assert_eq!
(
format!
(
"{x:o}"
),
"52"
);
assert_eq!
(
format!
(
"{x:#o}"
),
"0o52"
);
assert_eq!
(
format!
(
"{:o}"
, -
16
),
"37777777760"
);
Implementing
Octal
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::Octal
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

        fmt::Octal::fmt(
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
"l as octal is: {l:o}"
),
"l as octal is: 11"
);
assert_eq!
(
format!
(
"l as octal is: {l:#06o}"
),
"l as octal is: 0o0011"
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
Octal
for
i8
1.0.0
·
Source
§
impl
Octal
for
i16
1.0.0
·
Source
§
impl
Octal
for
i32
1.0.0
·
Source
§
impl
Octal
for
i64
1.0.0
·
Source
§
impl
Octal
for
i128
1.0.0
·
Source
§
impl
Octal
for
isize
1.0.0
·
Source
§
impl
Octal
for
u8
1.0.0
·
Source
§
impl
Octal
for
u16
1.0.0
·
Source
§
impl
Octal
for
u32
1.0.0
·
Source
§
impl
Octal
for
u64
1.0.0
·
Source
§
impl
Octal
for
u128
1.0.0
·
Source
§
impl
Octal
for
usize
1.0.0
·
Source
§
impl<T>
Octal
for
&T
where
    T:
Octal
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
Octal
for
&mut T
where
    T:
Octal
+ ?
Sized
,
1.28.0
·
Source
§
impl<T>
Octal
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
Octal
,
1.74.0
·
Source
§
impl<T>
Octal
for
Saturating
<T>
where
    T:
Octal
,
1.11.0
·
Source
§
impl<T>
Octal
for
Wrapping
<T>
where
    T:
Octal
,