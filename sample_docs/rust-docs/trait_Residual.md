Residual in std::ops - Rust
std
::
ops
Trait
Residual
Copy item path
Source
pub trait Residual<O> {
    type
TryType
:
Try
<Output = O, Residual = Self>;
}
🔬
This is a nightly-only experimental API. (
try_trait_v2_residual
#91285
)
Expand description
Allows retrieving the canonical type implementing
Try
that has this type
as its residual and allows it to hold an
O
as its output.
If you think of the
Try
trait as splitting a type into its
Try::Output
and
Try::Residual
components, this allows putting them back together.
For example,
Result<T, E>: Try<Output = T, Residual = Result<Infallible, E>>
,
and in the other direction,
<Result<Infallible, E> as Residual<T>>::TryType = Result<T, E>
.
Required Associated Types
§
Source
type
TryType
:
Try
<Output = O, Residual = Self>
🔬
This is a nightly-only experimental API. (
try_trait_v2_residual
#91285
)
The “return” type of this meta-function.
Implementors
§
Source
§
impl<B, C>
Residual
<C> for
ControlFlow
<B,
Infallible
>
Source
§
type
TryType
=
ControlFlow
<B, C>
Source
§
impl<T>
Residual
<T> for
Option
<
Infallible
>
Source
§
type
TryType
=
Option
<T>
Source
§
impl<T, E>
Residual
<T> for
Result
<
Infallible
, E>
Source
§
type
TryType
=
Result
<T, E>