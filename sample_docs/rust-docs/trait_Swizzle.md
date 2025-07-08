Swizzle in std::simd - Rust
std
::
simd
Trait
Swizzle
Copy item path
Source
pub trait Swizzle<const N:
usize
> {
    const
INDEX
: [
usize
;
N
];

    // Provided methods
    fn
swizzle
<T, const M:
usize
>(vector:
Simd
<T, M>) ->
Simd
<T, N>
where T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
{ ... }
fn
concat_swizzle
<T, const M:
usize
>(
        first:
Simd
<T, M>,
        second:
Simd
<T, M>,
    ) ->
Simd
<T, N>
where T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
{ ... }
fn
swizzle_mask
<T, const M:
usize
>(mask:
Mask
<T, M>) ->
Mask
<T, N>
where T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
{ ... }
fn
concat_swizzle_mask
<T, const M:
usize
>(
        first:
Mask
<T, M>,
        second:
Mask
<T, M>,
    ) ->
Mask
<T, N>
where T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
{ ... }
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Creates a vector from the elements of another vector.
Required Associated Constants
§
Source
const
INDEX
: [
usize
;
N
]
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Map from the elements of the input vector to the output vector.
Provided Methods
§
Source
fn
swizzle
<T, const M:
usize
>(vector:
Simd
<T, M>) ->
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new vector from the elements of
vector
.
Lane
i
of the output is
vector[Self::INDEX[i]]
.
Source
fn
concat_swizzle
<T, const M:
usize
>(
    first:
Simd
<T, M>,
    second:
Simd
<T, M>,
) ->
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new vector from the elements of
first
and
second
.
Lane
i
of the output is
concat[Self::INDEX[i]]
, where
concat
is the concatenation of
first
and
second
.
Source
fn
swizzle_mask
<T, const M:
usize
>(mask:
Mask
<T, M>) ->
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new mask from the elements of
mask
.
Element
i
of the output is
mask[Self::INDEX[i]]
.
Source
fn
concat_swizzle_mask
<T, const M:
usize
>(
    first:
Mask
<T, M>,
    second:
Mask
<T, M>,
) ->
Mask
<T, N>
where
    T:
MaskElement
,
LaneCount
<N>:
SupportedLaneCount
,
LaneCount
<M>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Creates a new mask from the elements of
first
and
second
.
Element
i
of the output is
concat[Self::INDEX[i]]
, where
concat
is the concatenation of
first
and
second
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