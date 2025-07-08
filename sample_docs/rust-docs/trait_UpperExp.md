UpperExp in std::fmt - Rust
std
::
fmt
Trait
UpperExp
Copy item path
1.0.0
·
Source
pub trait UpperExp {
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
E
formatting.
The
UpperExp
trait should format its output in scientific notation with an upper-case
E
.
For more information on formatters, see
the module-level documentation
.
§
Examples
Basic usage with
f64
:
let
x =
42.0
;
// 42.0 is '4.2E1' in scientific notation
assert_eq!
(
format!
(
"{x:E}"
),
"4.2E1"
);
Implementing
UpperExp
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::UpperExp
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
val = f64::from(
self
.
0
);
        fmt::UpperExp::fmt(
&
val, f)
// delegate to f64's implementation
}
}
let
l = Length(
100
);
assert_eq!
(
format!
(
"l in scientific notation is: {l:E}"
),
"l in scientific notation is: 1E2"
);
assert_eq!
(
format!
(
"l in scientific notation is: {l:05E}"
),
"l in scientific notation is: 001E2"
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
UpperExp
for
f32
1.0.0
·
Source
§
impl
UpperExp
for
f64
1.42.0
·
Source
§
impl
UpperExp
for
i8
1.42.0
·
Source
§
impl
UpperExp
for
i16
1.42.0
·
Source
§
impl
UpperExp
for
i32
1.42.0
·
Source
§
impl
UpperExp
for
i64
1.42.0
·
Source
§
impl
UpperExp
for
i128
1.42.0
·
Source
§
impl
UpperExp
for
isize
1.42.0
·
Source
§
impl
UpperExp
for
u8
1.42.0
·
Source
§
impl
UpperExp
for
u16
1.42.0
·
Source
§
impl
UpperExp
for
u32
1.42.0
·
Source
§
impl
UpperExp
for
u64
1.42.0
·
Source
§
impl
UpperExp
for
u128
1.42.0
·
Source
§
impl
UpperExp
for
usize
1.0.0
·
Source
§
impl<T>
UpperExp
for
&T
where
    T:
UpperExp
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
UpperExp
for
&mut T
where
    T:
UpperExp
+ ?
Sized
,
1.84.0
·
Source
§
impl<T>
UpperExp
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
UpperExp
,