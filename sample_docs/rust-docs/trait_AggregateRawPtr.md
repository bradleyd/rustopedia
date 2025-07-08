AggregateRawPtr in std::intrinsics - Rust
std
::
intrinsics
Trait
AggregateRawPtr
Copy item path
Source
pub trait AggregateRawPtr<D> {
    type
Metadata
:
Copy
;
}
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Required Associated Types
§
Source
type
Metadata
:
Copy
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Implementors
§
Source
§
impl<P, T>
AggregateRawPtr
<
*const T
> for
*const P
where
    T:
Thin
,
    P: ?
Sized
,
Source
§
type
Metadata
= <P as
Pointee
>::
Metadata
Source
§
impl<P, T>
AggregateRawPtr
<
*mut T
> for
*mut P
where
    T:
Thin
,
    P: ?
Sized
,
Source
§
type
Metadata
= <P as
Pointee
>::
Metadata