Random in std::random - Rust
std
::
random
Trait
Random
Copy item path
Source
pub trait Random:
Sized
{
    // Required method
    fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) -> Self;
}
🔬
This is a nightly-only experimental API. (
random
#130703
)
Expand description
A trait for getting a random value for a type.
Warning:
Be careful when manipulating random values! The
random
method on integers samples them with a uniform
distribution, so a value of 1 is just as likely as
i32::MAX
. By using
modulo operations, some of the resulting values can become more likely than
others. Use audited crates when in doubt.
Required Methods
§
Source
fn
random
(source: &mut (impl
RandomSource
+ ?
Sized
)) -> Self
🔬
This is a nightly-only experimental API. (
random
#130703
)
Generates a random value.
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
Random
for
bool
Source
§
impl
Random
for
i8
Source
§
impl
Random
for
i16
Source
§
impl
Random
for
i32
Source
§
impl
Random
for
i64
Source
§
impl
Random
for
i128
Source
§
impl
Random
for
isize
Source
§
impl
Random
for
u8
Source
§
impl
Random
for
u16
Source
§
impl
Random
for
u32
Source
§
impl
Random
for
u64
Source
§
impl
Random
for
u128
Source
§
impl
Random
for
usize