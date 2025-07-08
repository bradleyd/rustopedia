simd_swizzle in std::simd::prelude - Rust
std
::
simd
::
prelude
Macro
simd_swizzle
Copy item path
Source
pub macro simd_swizzle {
    ($vector:expr, $index:expr $(,)?) => { ... },
    ($first:expr, $second:expr, $index:expr $(,)?) => { ... },
}
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Expand description
Constructs a new SIMD vector by copying elements from selected elements in other vectors.
When swizzling one vector, elements are selected like
Swizzle::swizzle
.
When swizzling two vectors, elements are selected like
Swizzle::concat_swizzle
.
§
Examples
With a single SIMD vector, the const array specifies element indices in that vector:
let
v = u32x4::from_array([
10
,
11
,
12
,
13
]);
// Keeping the same size
let
r: u32x4 =
simd_swizzle!
(v, [
3
,
0
,
1
,
2
]);
assert_eq!
(r.to_array(), [
13
,
10
,
11
,
12
]);
// Changing the number of elements
let
r: u32x2 =
simd_swizzle!
(v, [
3
,
1
]);
assert_eq!
(r.to_array(), [
13
,
11
]);
With two input SIMD vectors, the const array specifies element indices in the concatenation of
those vectors:
let
a = u32x4::from_array([
0
,
1
,
2
,
3
]);
let
b = u32x4::from_array([
4
,
5
,
6
,
7
]);
// Keeping the same size
let
r: u32x4 =
simd_swizzle!
(a, b, [
0
,
1
,
6
,
7
]);
assert_eq!
(r.to_array(), [
0
,
1
,
6
,
7
]);
// Changing the number of elements
let
r: u32x2 =
simd_swizzle!
(a, b, [
0
,
4
]);
assert_eq!
(r.to_array(), [
0
,
4
]);