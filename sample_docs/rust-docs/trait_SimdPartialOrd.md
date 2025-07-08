SimdPartialOrd in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdPartialOrd
Copy item path
Source
pub trait SimdPartialOrd:
SimdPartialEq
{
    // Required methods
    fn
simd_lt
(self, other: Self) -> Self::
Mask
;
fn
simd_le
(self, other: Self) -> Self::
Mask
;
fn
simd_gt
(self, other: Self) -> Self::
Mask
;
fn
simd_ge
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
PartialOrd
.
Required Methods
§
Source
fn
simd_lt
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than the corresponding element in
other
.
Source
fn
simd_le
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is less than or equal to the corresponding element in
other
.
Source
fn
simd_gt
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than the corresponding element in
other
.
Source
fn
simd_ge
(self, other: Self) -> Self::
Mask
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Test if each element is greater than or equal to the corresponding element in
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
SimdPartialOrd
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
impl<T, const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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
impl<const N:
usize
>
SimdPartialOrd
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