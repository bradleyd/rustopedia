Try in std::ops - Rust
std
::
ops
Trait
Try
Copy item path
Source
pub trait Try:
FromResidual
{
    type
Output
;
    type
Residual
;

    // Required methods
    fn
from_output
(output: Self::
Output
) -> Self;
fn
branch
(self) ->
ControlFlow
<Self::
Residual
, Self::
Output
>;
}
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Expand description
The
?
operator and
try {}
blocks.
try_*
methods typically involve a type implementing this trait.  For
example, the closures passed to
Iterator::try_fold
and
Iterator::try_for_each
must return such a type.
Try
types are typically those containing two or more categories of values,
some subset of which are so commonly handled via early returns that it’s
worth providing a terse (but still visible) syntax to make that easy.
This is most often seen for error handling with
Result
and
Option
.
The quintessential implementation of this trait is on
ControlFlow
.
§
Using
Try
in Generic Code
Iterator::try_fold
was stabilized to call back in Rust 1.27, but
this trait is much newer.  To illustrate the various associated types and
methods, let’s implement our own version.
As a reminder, an infallible version of a fold looks something like this:
fn
simple_fold<A, T>(
    iter:
impl
Iterator<Item = T>,
mut
accum: A,
mut
f:
impl
FnMut(A, T) -> A,
) -> A {
for
x
in
iter {
        accum = f(accum, x);
    }
    accum
}
So instead of
f
returning just an
A
, we’ll need it to return some other
type that produces an
A
in the “don’t short circuit” path.  Conveniently,
that’s also the type we need to return from the function.
Let’s add a new generic parameter
R
for that type, and bound it to the
output type that we want:
fn
simple_try_fold_1<A, T, R: Try<Output = A>>(
    iter:
impl
Iterator<Item = T>,
mut
accum: A,
mut
f:
impl
FnMut(A, T) -> R,
) -> R {
todo!
()
}
If we get through the entire iterator, we need to wrap up the accumulator
into the return type using
Try::from_output
:
fn
simple_try_fold_2<A, T, R: Try<Output = A>>(
    iter:
impl
Iterator<Item = T>,
mut
accum: A,
mut
f:
impl
FnMut(A, T) -> R,
) -> R {
for
x
in
iter {
let
cf = f(accum, x).branch();
match
cf {
            ControlFlow::Continue(a) => accum = a,
            ControlFlow::Break(
_
) =>
todo!
(),
        }
    }
    R::from_output(accum)
}
We’ll also need
FromResidual::from_residual
to turn the residual back
into the original type.  But because it’s a supertrait of
Try
, we don’t
need to mention it in the bounds.  All types which implement
Try
can be
recreated from their corresponding residual, so we’ll just call it:
pub fn
simple_try_fold_3<A, T, R: Try<Output = A>>(
    iter:
impl
Iterator<Item = T>,
mut
accum: A,
mut
f:
impl
FnMut(A, T) -> R,
) -> R {
for
x
in
iter {
let
cf = f(accum, x).branch();
match
cf {
            ControlFlow::Continue(a) => accum = a,
            ControlFlow::Break(r) =>
return
R::from_residual(r),
        }
    }
    R::from_output(accum)
}
But this “call
branch
, then
match
on it, and
return
if it was a
Break
” is exactly what happens inside the
?
operator.  So rather than
do all this manually, we can just use
?
instead:
fn
simple_try_fold<A, T, R: Try<Output = A>>(
    iter:
impl
Iterator<Item = T>,
mut
accum: A,
mut
f:
impl
FnMut(A, T) -> R,
) -> R {
for
x
in
iter {
        accum = f(accum, x)
?
;
    }
    R::from_output(accum)
}
Required Associated Types
§
Source
type
Output
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value produced by
?
when
not
short-circuiting.
Source
type
Residual
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value passed to
FromResidual::from_residual
as part of
?
when short-circuiting.
This represents the possible values of the
Self
type which are
not
represented by the
Output
type.
§
Note to Implementors
The choice of this type is critical to interconversion.
Unlike the
Output
type, which will often be a raw generic type,
this type is typically a newtype of some sort to “color” the type
so that it’s distinguishable from the residuals of other types.
This is why
Result<T, E>::Residual
is not
E
, but
Result<Infallible, E>
.
That way it’s distinct from
ControlFlow<E>::Residual
, for example,
and thus
?
on
ControlFlow
cannot be used in a method returning
Result
.
If you’re making a generic type
Foo<T>
that implements
Try<Output = T>
,
then typically you can use
Foo<std::convert::Infallible>
as its
Residual
type: that type will have a “hole” in the correct place, and will maintain the
“foo-ness” of the residual so other types need to opt-in to interconversion.
Required Methods
§
Source
fn
from_output
(output: Self::
Output
) -> Self
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from its
Output
type.
This should be implemented consistently with the
branch
method
such that applying the
?
operator will get back the original value:
Try::from_output(x).branch() --> ControlFlow::Continue(x)
.
§
Examples
#![feature(try_trait_v2)]
use
std::ops::Try;
assert_eq!
(<
Result
<
_
, String>
as
Try>::from_output(
3
),
Ok
(
3
));
assert_eq!
(<
Option
<
_
>
as
Try>::from_output(
4
),
Some
(
4
));
assert_eq!
(
    <std::ops::ControlFlow<String,
_
>
as
Try>::from_output(
5
),
    std::ops::ControlFlow::Continue(
5
),
);
assert_eq!
(Option::from_output(
4
)
?
,
4
);
// This is used, for example, on the accumulator in `try_fold`:
let
r = std::iter::empty().try_fold(
4
, |
_
, ()| ->
Option
<
_
> {
unreachable!
() });
assert_eq!
(r,
Some
(
4
));
Source
fn
branch
(self) ->
ControlFlow
<Self::
Residual
, Self::
Output
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Used in
?
to decide whether the operator should produce a value
(because this returned
ControlFlow::Continue
)
or propagate a value back to the caller
(because this returned
ControlFlow::Break
).
§
Examples
#![feature(try_trait_v2)]
use
std::ops::{ControlFlow, Try};
assert_eq!
(
Ok
::<
_
, String>(
3
).branch(), ControlFlow::Continue(
3
));
assert_eq!
(
Err
::<String,
_
>(
3
).branch(), ControlFlow::Break(
Err
(
3
)));
assert_eq!
(
Some
(
3
).branch(), ControlFlow::Continue(
3
));
assert_eq!
(
None
::<String>.branch(), ControlFlow::Break(
None
));
assert_eq!
(ControlFlow::<String,
_
>::Continue(
3
).branch(), ControlFlow::Continue(
3
));
assert_eq!
(
    ControlFlow::<
_
, String>::Break(
3
).branch(),
    ControlFlow::Break(ControlFlow::Break(
3
)),
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
Try
for
ControlFlow
<B, C>
Source
§
type
Output
= C
Source
§
type
Residual
=
ControlFlow
<B,
Infallible
>
Source
§
impl<T>
Try
for
Option
<T>
Source
§
type
Output
= T
Source
§
type
Residual
=
Option
<
Infallible
>
Source
§
impl<T, E>
Try
for
Result
<T, E>
Source
§
type
Output
= T
Source
§
type
Residual
=
Result
<
Infallible
, E>
Source
§
impl<T, E>
Try
for
Poll
<
Option
<
Result
<T, E>>>
Source
§
type
Output
=
Poll
<
Option
<T>>
Source
§
type
Residual
=
Result
<
Infallible
, E>
Source
§
impl<T, E>
Try
for
Poll
<
Result
<T, E>>
Source
§
type
Output
=
Poll
<T>
Source
§
type
Residual
=
Result
<
Infallible
, E>