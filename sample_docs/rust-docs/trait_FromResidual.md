FromResidual in std::ops - Rust
std
::
ops
Trait
FromResidual
Copy item path
Source
pub trait FromResidual<R = <Self as
Try
>::
Residual
> {
    // Required method
    fn
from_residual
(residual: R) -> Self;
}
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Expand description
Used to specify which residuals can be converted into which
crate::ops::Try
types.
Every
Try
type needs to be recreatable from its own associated
Residual
type, but can also have additional
FromResidual
implementations
to support interconversion with other
Try
types.
Required Methods
§
Source
fn
from_residual
(residual: R) -> Self
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
This should be implemented consistently with the
branch
method such
that applying the
?
operator will get back an equivalent residual:
FromResidual::from_residual(r).branch() --> ControlFlow::Break(r)
.
(The residual is not mandated to be
identical
when interconversion is involved.)
§
Examples
#![feature(try_trait_v2)]
use
std::ops::{ControlFlow, FromResidual};
assert_eq!
(
Result
::<String, i64>::from_residual(
Err
(
3_u8
)),
Err
(
3
));
assert_eq!
(
Option
::<String>::from_residual(
None
),
None
);
assert_eq!
(
    ControlFlow::<
_
, String>::from_residual(ControlFlow::Break(
5
)),
    ControlFlow::Break(
5
),
);
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
impl<B, C>
FromResidual
<
ControlFlow
<B,
Infallible
>> for
ControlFlow
<B, C>
Source
§
impl<T>
FromResidual
<
Option
<
Infallible
>> for
Option
<T>
Source
§
impl<T>
FromResidual
<
Yeet
<
()
>> for
Option
<T>
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Result
<T, F>
where
    F:
From
<E>,
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Option
<
Result
<T, F>>>
where
    F:
From
<E>,
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Result
<T, F>>
where
    F:
From
<E>,
Source
§
impl<T, E, F>
FromResidual
<
Yeet
<E>> for
Result
<T, F>
where
    F:
From
<E>,