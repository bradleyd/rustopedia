transmute_unchecked in std::intrinsics - Rust
std
::
intrinsics
Function
transmute_unchecked
Copy item path
Source
pub const unsafe fn transmute_unchecked<Src, Dst>(src: Src) -> Dst
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Like
transmute
, but even less checked at compile-time: rather than
giving an error for
size_of::<Src>() != size_of::<Dst>()
, it’s
Undefined Behavior
at runtime.
Prefer normal
transmute
where possible, for the extra checking, since
both do exactly the same thing at runtime, if they both compile.
This is not expected to ever be exposed directly to users, rather it
may eventually be exposed through some more-constrained API.