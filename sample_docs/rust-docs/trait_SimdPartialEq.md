SimdPartialEq in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdPartialEq
Copy item path
Source
pub trait SimdPartialEq {
    type
Mask
;

    // Required methods
    fn
simd_eq
(self, other: Self) -> Self::
Mask
;
fn
simd_ne
(self, other: Self) -> Self::
Mask
;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Parallel
PartialEq
.
Required Associated Types
§
Source
type
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask type returned by each comparison.
Required Methods
§
Source
fn
simd_eq
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is equal to the corresponding element in
other
.
Source
fn
simd_ne
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is not equal to the corresponding element in
other
.
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
SimdPartialEq
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
Mask
=
Mask
<
isize
, N>
Source
§
impl<T, const N:
usize
>
SimdPartialEq
for
Simd
<
*mut T
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
isize
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Mask
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
i8
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Mask
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
i16
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Mask
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
i32
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Mask
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
i64
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Mask
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<
isize
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
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
type
Mask
=
Mask
<<
f32
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
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
Source
§
type
Mask
=
Mask
<<
f64
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i8
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i16
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i32
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
i64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
i64
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
isize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
isize
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u8
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u8
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u16
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u16
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u32
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u32
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
u64
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
u64
as
SimdElement
>::
Mask
, N>
Source
§
impl<const N:
usize
>
SimdPartialEq
for
Simd
<
usize
, N>
where
LaneCount
<N>:
SupportedLaneCount
,
Source
§
type
Mask
=
Mask
<<
usize
as
SimdElement
>::
Mask
, N>