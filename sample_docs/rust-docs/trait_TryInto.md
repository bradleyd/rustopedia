TryInto in std::convert - Rust
std
::
convert
Trait
TryInto
Copy item path
1.34.0
·
Source
pub trait TryInto<T>:
Sized
{
    type
Error
;

    // Required method
    fn
try_into
(self) ->
Result
<T, Self::
Error
>;
}
Expand description
An attempted conversion that consumes
self
, which may or may not be
expensive.
Library authors should usually not directly implement this trait,
but should prefer implementing the
TryFrom
trait, which offers
greater flexibility and provides an equivalent
TryInto
implementation for free, thanks to a blanket implementation in the
standard library. For more information on this, see the
documentation for
Into
.
§
Implementing
TryInto
This suffers the same restrictions and reasoning as implementing
Into
, see there for details.
Required Associated Types
§
1.34.0
·
Source
type
Error
The type returned in the event of a conversion error.
Required Methods
§
1.34.0
·
Source
fn
try_into
(self) ->
Result
<T, Self::
Error
>
Performs the conversion.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.34.0
·
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error