StdFloat in std::simd - Rust
std
::
simd
Trait
StdFloat
Copy item path
Source
pub trait StdFloat: Sealed +
Sized
{
Show 15 methods
// Required methods
    fn
sin
(self) -> Self;
fn
cos
(self) -> Self;
fn
exp
(self) -> Self;
fn
exp2
(self) -> Self;
fn
ln
(self) -> Self;
fn
log2
(self) -> Self;
fn
log10
(self) -> Self;
fn
fract
(self) -> Self;

    // Provided methods
    fn
mul_add
(self, a: Self, b: Self) -> Self { ... }
fn
sqrt
(self) -> Self { ... }
fn
log
(self, base: Self) -> Self { ... }
fn
ceil
(self) -> Self { ... }
fn
floor
(self) -> Self { ... }
fn
round
(self) -> Self { ... }
fn
trunc
(self) -> Self { ... }
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
This trait provides a possibly-temporary implementation of float functions
that may, in the absence of hardware support, canonicalize to calling an
operating system’s
math.h
dynamically-loaded library (also known as a
shared object). As these conditionally require runtime support, they
should only appear in binaries built assuming OS support:
std
.
However, there is no reason SIMD types, in general, need OS support,
as for many architectures an embedded binary may simply configure that
support itself. This means these types must be visible in
core
but have these functions available in
std
.
f32
and
f64
achieve a similar trick by using “lang items”, but
due to compiler limitations, it is harder to implement this approach for
abstract data types like
Simd
. From that need, this trait is born.
It is possible this trait will be replaced in some manner in the future,
when either the compiler or its supporting runtime functions are improved.
For now this trait is available to permit experimentation with SIMD float
operations that may lack hardware support, such as
mul_add
.
Required Methods
§
Source
fn
sin
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the sine of the value
in the equivalently-indexed element in
self
.
Source
fn
cos
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the cosine of the value
in the equivalently-indexed element in
self
.
Source
fn
exp
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base e) of the value
in the equivalently-indexed element in
self
.
Source
fn
exp2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the exponential (base 2) of the value
in the equivalently-indexed element in
self
.
Source
fn
ln
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the natural logarithm of the value
in the equivalently-indexed element in
self
.
Source
fn
log2
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-2 logarithm of the value
in the equivalently-indexed element in
self
.
Source
fn
log10
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the base-10 logarithm of the value
in the equivalently-indexed element in
self
.
Source
fn
fract
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s fractional value, with its integer part removed.
Provided Methods
§
Source
fn
mul_add
(self, a: Self, b: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Elementwise fused multiply-add. Computes
(self * a) + b
with only one rounding error,
yielding a more accurate result than an unfused multiply-add.
Using
mul_add
may
be more performant than an unfused multiply-add if the target
architecture has a dedicated
fma
CPU instruction.  However, this is not always
true, and will be heavily dependent on designing algorithms with specific target
hardware in mind.
Source
fn
sqrt
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the square root value
of the equivalently-indexed element in
self
Source
fn
log
(self, base: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Produces a vector where every element has the logarithm with respect to an arbitrary
in the equivalently-indexed elements in
self
and
base
.
Source
fn
ceil
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the smallest integer greater than or equal to each element.
Source
fn
floor
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the largest integer value less than or equal to each element.
Source
fn
round
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Rounds to the nearest integer value. Ties round toward zero.
Source
fn
trunc
(self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the floating point’s integer value, with its fractional part removed.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl<const N:
usize
>
StdFloat
for
Simd
<
f32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
impl<const N:
usize
>
StdFloat
for
Simd
<
f64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,