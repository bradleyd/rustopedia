Variance in std::marker - Rust
std
::
marker
Trait
Variance
Copy item path
Source
pub trait Variance: Sealed +
Default
{ }
🔬
This is a nightly-only experimental API. (
phantom_variance_markers
#135806
)
Expand description
A marker trait for phantom variance types.
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
Variance
for
PhantomContravariantLifetime
<'_>
Source
§
impl
Variance
for
PhantomCovariantLifetime
<'_>
Source
§
impl
Variance
for
PhantomInvariantLifetime
<'_>
Source
§
impl<T>
Variance
for
PhantomContravariant
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Variance
for
PhantomCovariant
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
Variance
for
PhantomInvariant
<T>
where
    T: ?
Sized
,