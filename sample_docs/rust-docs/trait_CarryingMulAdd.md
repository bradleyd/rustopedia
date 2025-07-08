CarryingMulAdd in std::intrinsics::fallback - Rust
std
::
intrinsics
::
fallback
Trait
CarryingMulAdd
Copy item path
Source
pub trait CarryingMulAdd:
Copy
+ 'static {
    type
Unsigned
:
Copy
+ 'static;

    // Required method
    const fn
carrying_mul_add
(
        self,
        multiplicand: Self,
        addend: Self,
        carry: Self,
    ) -> (Self::
Unsigned
, Self);
}
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Required Associated Types
§
Source
type
Unsigned
:
Copy
+ 'static
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
Required Methods
§
Source
const fn
carrying_mul_add
(
    self,
    multiplicand: Self,
    addend: Self,
    carry: Self,
) -> (Self::
Unsigned
, Self)
🔬
This is a nightly-only experimental API. (
core_intrinsics_fallbacks
)
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
CarryingMulAdd
for
i8
Source
§
type
Unsigned
=
u8
Source
§
impl
CarryingMulAdd
for
i16
Source
§
type
Unsigned
=
u16
Source
§
impl
CarryingMulAdd
for
i32
Source
§
type
Unsigned
=
u32
Source
§
impl
CarryingMulAdd
for
i64
Source
§
type
Unsigned
=
u64
Source
§
impl
CarryingMulAdd
for
i128
Source
§
type
Unsigned
=
u128
Source
§
impl
CarryingMulAdd
for
isize
Source
§
type
Unsigned
=
usize
Source
§
impl
CarryingMulAdd
for
u8
Source
§
type
Unsigned
=
u8
Source
§
impl
CarryingMulAdd
for
u16
Source
§
type
Unsigned
=
u16
Source
§
impl
CarryingMulAdd
for
u32
Source
§
type
Unsigned
=
u32
Source
§
impl
CarryingMulAdd
for
u64
Source
§
type
Unsigned
=
u64
Source
§
impl
CarryingMulAdd
for
u128
Source
§
type
Unsigned
=
u128
Source
§
impl
CarryingMulAdd
for
usize
Source
§
type
Unsigned
=
usize