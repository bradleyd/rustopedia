SimdOrd in std::simd::prelude - Rust
std
::
simd
::
prelude
Trait
SimdOrd
Copy item path
Source
pub trait SimdOrd:
SimdPartialOrd
{
    // Required methods
    fn
simd_max
(self, other: Self) -> Self;
fn
simd_min
(self, other: Self) -> Self;
fn
simd_clamp
(self, min: Self, max: Self) -> Self;
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Parallel
Ord
.
Required Methods
§
Source
fn
simd_max
(self, other: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise maximum with
other
.
Source
fn
simd_min
(self, other: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Returns the element-wise minimum with
other
.
Source
fn
simd_clamp
(self, min: Self, max: Self) -> Self
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Restrict each element to a certain interval.
For each element, returns
max
if
self
is greater than
max
, and
min
if
self
is
less than
min
. Otherwise returns
self
.
§
Panics
Panics if
min > max
on any element.
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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
SimdOrd
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