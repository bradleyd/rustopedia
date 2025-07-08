TrustedStep in std::iter - Rust
std
::
iter
Trait
TrustedStep
Copy item path
Source
pub unsafe trait TrustedStep:
Step
+
Copy
{ }
🔬
This is a nightly-only experimental API. (
trusted_step
#85731
)
Expand description
A type that upholds all invariants of
Step
.
The invariants of
Step::steps_between()
are a superset of the invariants
of
TrustedLen
. As such,
TrustedLen
is implemented for all range
types with the same generic type argument.
§
Safety
The implementation of
Step
for the given type must guarantee all
invariants of all methods are upheld. See the
Step
trait’s documentation
for details. Consumers are free to rely on the invariants in unsafe code.
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
impl
TrustedStep
for
AsciiChar
Source
§
impl
TrustedStep
for
char
Source
§
impl
TrustedStep
for
i8
Source
§
impl
TrustedStep
for
i16
Source
§
impl
TrustedStep
for
i32
Source
§
impl
TrustedStep
for
i64
Source
§
impl
TrustedStep
for
i128
Source
§
impl
TrustedStep
for
isize
Source
§
impl
TrustedStep
for
u8
Source
§
impl
TrustedStep
for
u16
Source
§
impl
TrustedStep
for
u32
Source
§
impl
TrustedStep
for
u64
Source
§
impl
TrustedStep
for
u128
Source
§
impl
TrustedStep
for
usize
Source
§
impl
TrustedStep
for
Ipv4Addr
Source
§
impl
TrustedStep
for
Ipv6Addr