raw_eq in std::intrinsics - Rust
std
::
intrinsics
Function
raw_eq
Copy item path
Source
pub const unsafe fn raw_eq<T>(a:
&T
, b:
&T
) ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Determines whether the raw bytes of the two values are equal.
This is particularly handy for arrays, since it allows things like just
comparing
i96
s instead of forcing
alloca
s for
[6 x i16]
.
Above some backend-decided threshold this will emit calls to
memcmp
,
like slice equality does, instead of causing massive code size.
Since this works by comparing the underlying bytes, the actual
T
is
not particularly important.  It will be used for its size and alignment,
but any validity restrictions will be ignored, not enforced.
§
Safety
It’s UB to call this if any of the
bytes
in
*a
or
*b
are uninitialized.
Note that this is a stricter criterion than just the
values
being
fully-initialized: if
T
has padding, it’s UB to call this intrinsic.
At compile-time, it is furthermore UB to call this if any of the bytes
in
*a
or
*b
have provenance.
(The implementation is allowed to branch on the results of comparisons,
which is UB if any of their inputs are
undef
.)