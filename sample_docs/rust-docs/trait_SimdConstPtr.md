SimdConstPtr in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdConstPtr
Copy item path
Source
pub trait SimdConstPtr:
Copy
+ Sealed {
    type
Usize
;
    type
Isize
;
    type
CastPtr
<T>;
    type
MutPtr
;
    type
Mask
;

    // Required methods
    fn
is_null
(self) -> Self::
Mask
;
fn
cast
<T>(self) -> Self::
CastPtr
<T>;
fn
cast_mut
(self) -> Self::
MutPtr
;
fn
addr
(self) -> Self::
Usize
;
fn
without_provenance
(addr: Self::
Usize
) -> Self;
fn
with_addr
(self, addr: Self::
Usize
) -> Self;
fn
expose_provenance
(self) -> Self::
Usize
;
fn
with_exposed_provenance
(addr: Self::
Usize
) -> Self;
fn
wrapping_offset
(self, offset: Self::
Isize
) -> Self;
fn
wrapping_add
(self, count: Self::
Usize
) -> Self;
fn
wrapping_sub
(self, count: Self::
Usize
) -> Self;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Operations on SIMD vectors of constant pointers.
Required Associated Types
§
Source
type
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
usize
with the same number of elements.
Source
type
Isize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of
isize
with the same number of elements.
Source
type
CastPtr
<T>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of const pointers with the same number of elements.
Source
type
MutPtr
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Vector of mutable pointers to the same type.
Source
type
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Mask type used for manipulating this SIMD vector type.
Required Methods
§
Source
fn
is_null
(self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns
true
for each element that is null.
Source
fn
cast
<T>(self) -> Self::
CastPtr
<T>
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Casts to a pointer of another type.
Equivalent to calling
pointer::cast
on each element.
Source
fn
cast_mut
(self) -> Self::
MutPtr
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Changes constness without changing the type.
Equivalent to calling
pointer::cast_mut
on each element.
Source
fn
addr
(self) -> Self::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Gets the “address” portion of the pointer.
This method discards pointer semantic metadata, so the result cannot be
directly cast into a valid pointer.
This method semantically discards
provenance
and
address-space
information. To properly restore that information, use
Self::with_addr
.
Equivalent to calling
pointer::addr
on each element.
Source
fn
without_provenance
(addr: Self::
Usize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address to a pointer without giving it any provenance.
Without provenance, this pointer is not associated with any actual allocation. Such a
no-provenance pointer may be used for zero-sized memory accesses (if suitably aligned), but
non-zero-sized memory accesses with a no-provenance pointer are UB. No-provenance pointers
are little more than a usize address in disguise.
This is different from
Self::with_exposed_provenance
, which creates a pointer that picks up a
previously exposed provenance.
Equivalent to calling
core::ptr::without_provenance
on each element.
Source
fn
with_addr
(self, addr: Self::
Usize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new pointer with the given address.
This performs the same operation as a cast, but copies the
address-space
and
provenance
of
self
to the new pointer.
Equivalent to calling
pointer::with_addr
on each element.
Source
fn
expose_provenance
(self) -> Self::
Usize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Exposes the “provenance” part of the pointer for future use in
Self::with_exposed_provenance
and returns the “address” portion.
Source
fn
with_exposed_provenance
(addr: Self::
Usize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Converts an address back to a pointer, picking up a previously “exposed” provenance.
Equivalent to calling
core::ptr::with_exposed_provenance
on each element.
Source
fn
wrapping_offset
(self, offset: Self::
Isize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Equivalent to calling
pointer::wrapping_offset
on each element.
Source
fn
wrapping_add
(self, count: Self::
Usize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Equivalent to calling
pointer::wrapping_add
on each element.
Source
fn
wrapping_sub
(self, count: Self::
Usize
) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Calculates the offset from a pointer using wrapping arithmetic.
Equivalent to calling
pointer::wrapping_sub
on each element.
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
impl<T, const N:
usize
>
SimdConstPtr
for
Simd
<
*const T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Usize
=
Simd
<
usize
, N>
Source
§
type
Isize
=
Simd
<
isize
, N>
Source
§
type
CastPtr
<U> =
Simd
<
*const U
, N>
Source
§
type
MutPtr
=
Simd
<
*mut T
, N>
Source
§
type
Mask
=
Mask
<
isize
, N>