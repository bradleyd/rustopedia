RandomSource in std::random - Rust
std
::
random
Trait
RandomSource
Copy item path
Source
pub trait RandomSource {
    // Required method
    fn
fill_bytes
(&mut self, bytes: &mut [
u8
]);
}
🔬
This is a nightly-only experimental API. (
random
#130703
)
Expand description
A source of randomness.
Required Methods
§
Source
fn
fill_bytes
(&mut self, bytes: &mut [
u8
])
🔬
This is a nightly-only experimental API. (
random
#130703
)
Fills
bytes
with random bytes.
Implementors
§
Source
§
impl
RandomSource
for
DefaultRandomSource