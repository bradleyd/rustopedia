LowerExp in std::fmt - Rust
std
::
fmt
Trait
LowerExp
Copy item path
1.0.0
·
Source
pub trait LowerExp {
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
e
formatting.
The
LowerExp
trait should format its output in scientific notation with a lower-case
e
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
// 42.0 is '4.2e1' in scientific notation
assert_eq!
(
format!
(
"{x:e}"
),
"4.2e1"
);
Implementing
LowerExp
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::LowerExp
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
        fmt::LowerExp::fmt(
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
"l in scientific notation is: {l:e}"
),
"l in scientific notation is: 1e2"
);
assert_eq!
(
format!
(
"l in scientific notation is: {l:05e}"
),
"l in scientific notation is: 001e2"
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
LowerExp
for
f32
1.0.0
·
Source
§
impl
LowerExp
for
f64
1.42.0
·
Source
§
impl
LowerExp
for
i8
1.42.0
·
Source
§
impl
LowerExp
for
i16
1.42.0
·
Source
§
impl
LowerExp
for
i32
1.42.0
·
Source
§
impl
LowerExp
for
i64
1.42.0
·
Source
§
impl
LowerExp
for
i128
1.42.0
·
Source
§
impl
LowerExp
for
isize
1.42.0
·
Source
§
impl
LowerExp
for
u8
1.42.0
·
Source
§
impl
LowerExp
for
u16
1.42.0
·
Source
§
impl
LowerExp
for
u32
1.42.0
·
Source
§
impl
LowerExp
for
u64
1.42.0
·
Source
§
impl
LowerExp
for
u128
1.42.0
·
Source
§
impl
LowerExp
for
usize
1.0.0
·
Source
§
impl<T>
LowerExp
for
&T
where
    T:
LowerExp
+ ?
Sized
,
1.0.0
·
Source
§
impl<T>
LowerExp
for
&mut T
where
    T:
LowerExp
+ ?
Sized
,
1.84.0
·
Source
§
impl<T>
LowerExp
for
NonZero
<T>
where
    T:
ZeroablePrimitive
+
LowerExp
,