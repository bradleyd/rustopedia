DisjointBitOr in std::intrinsics::fallback - Rust
std
::
intrinsics
::
fallback
Trait
DisjointBitOr
Copy item path
Source
pub trait DisjointBitOr:
Copy
+ 'static {
    // Required method
    const unsafe fn
disjoint_bitor
(self, other: Self) -> Self;
}
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Required Methods
§
Source
const unsafe fn
disjoint_bitor
(self, other: Self) -> Self
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
See
super::disjoint_bitor
; we just need the trait indirection to handle
different types since calling intrinsics with generics doesn’t work.
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
DisjointBitOr
for
bool
Source
§
impl
DisjointBitOr
for
i8
Source
§
impl
DisjointBitOr
for
i16
Source
§
impl
DisjointBitOr
for
i32
Source
§
impl
DisjointBitOr
for
i64
Source
§
impl
DisjointBitOr
for
i128
Source
§
impl
DisjointBitOr
for
isize
Source
§
impl
DisjointBitOr
for
u8
Source
§
impl
DisjointBitOr
for
u16
Source
§
impl
DisjointBitOr
for
u32
Source
§
impl
DisjointBitOr
for
u64
Source
§
impl
DisjointBitOr
for
u128
Source
§
impl
DisjointBitOr
for
usize