IntoFuture in std::future - Rust
std
::
future
Trait
IntoFuture
Copy item path
1.64.0
·
Source
pub trait IntoFuture {
    type
Output
;
    type
IntoFuture
:
Future
<Output = Self::
Output
>;

    // Required method
    fn
into_future
(self) -> Self::
IntoFuture
;
}
Expand description
Conversion into a
Future
.
By implementing
IntoFuture
for a type, you define how it will be
converted to a future.
§
.await
desugaring
The
.await
keyword desugars into a call to
IntoFuture::into_future
first before polling the future to completion.
IntoFuture
is implemented
for all
T: Future
which means the
into_future
method will be available
on all futures.
use
std::future::IntoFuture;
let
v =
async
{
"meow"
};
let
mut
fut = v.into_future();
assert_eq!
(
"meow"
, fut.
await
);
§
Async builders
When implementing futures manually there will often be a choice between
implementing
Future
or
IntoFuture
for a type. Implementing
Future
is a
good choice in most cases. But implementing
IntoFuture
is most useful when
implementing “async builder” types, which allow their values to be modified
multiple times before being
.await
ed.
use
std::future::{ready, Ready, IntoFuture};
/// Eventually multiply two numbers
pub struct
Multiply {
    num: u16,
    factor: u16,
}
impl
Multiply {
/// Constructs a new instance of `Multiply`.
pub fn
new(num: u16, factor: u16) ->
Self
{
Self
{ num, factor }
    }
/// Set the number to multiply by the factor.
pub fn
number(
mut
self
, num: u16) ->
Self
{
self
.num = num;
self
}
/// Set the factor to multiply the number with.
pub fn
factor(
mut
self
, factor: u16) ->
Self
{
self
.factor = factor;
self
}
}
impl
IntoFuture
for
Multiply {
type
Output = u16;
type
IntoFuture = Ready<
Self
::Output>;
fn
into_future(
self
) ->
Self
::IntoFuture {
        ready(
self
.num *
self
.factor)
    }
}
// NOTE: Rust does not yet have an `async fn main` function, that functionality
// currently only exists in the ecosystem.
async fn
run() {
let
num = Multiply::new(
0
,
0
)
// initialize the builder to number: 0, factor: 0
.number(
2
)
// change the number to 2
.factor(
2
)
// change the factor to 2
.
await
;
// convert to future and .await
assert_eq!
(num,
4
);
}
§
Usage in trait bounds
Using
IntoFuture
in trait bounds allows a function to be generic over both
Future
and
IntoFuture
. This is convenient for users of the function, so
when they are using it they don’t have to make an extra call to
IntoFuture::into_future
to obtain an instance of
Future
:
use
std::future::IntoFuture;
/// Converts the output of a future to a string.
async fn
fut_to_string<Fut>(fut: Fut) -> String
where
Fut: IntoFuture,
    Fut::Output: std::fmt::Debug,
{
format!
(
"{:?}"
, fut.
await
)
}
Required Associated Types
§
1.64.0
·
Source
type
Output
The output that the future will produce on completion.
1.64.0
·
Source
type
IntoFuture
:
Future
<Output = Self::
Output
>
Which kind of future are we turning this into?
Required Methods
§
1.64.0
·
Source
fn
into_future
(self) -> Self::
IntoFuture
Creates a future from a value.
§
Examples
Basic usage:
use
std::future::IntoFuture;
let
v =
async
{
"meow"
};
let
mut
fut = v.into_future();
assert_eq!
(
"meow"
, fut.
await
);
Implementors
§
1.64.0
·
Source
§
impl<F>
IntoFuture
for F
where
    F:
Future
,
Source
§
type
Output
= <F as
Future
>::
Output
Source
§
type
IntoFuture
= F