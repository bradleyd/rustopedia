AsyncFnOnce in std::ops - Rust
std
::
ops
Trait
AsyncFnOnce
Copy item path
1.85.0
·
Source
pub trait AsyncFnOnce<Args>
where
    Args:
Tuple
,
{
    type
CallOnceFuture
:
Future
<Output = Self::
Output
>;
    type
Output
;

    // Required method
    extern "rust-call" fn
async_call_once
(
        self,
        args: Args,
    ) -> Self::
CallOnceFuture
;
}
Expand description
An async-aware version of the
FnOnce
trait.
All
async fn
and functions returning futures implement this trait.
Required Associated Types
§
Source
type
CallOnceFuture
:
Future
<Output = Self::
Output
>
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Future returned by
AsyncFnOnce::async_call_once
.
Source
type
Output
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Output type of the called closure’s future.
Required Methods
§
Source
extern "rust-call" fn
async_call_once
(
    self,
    args: Args,
) -> Self::
CallOnceFuture
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFnOnce
, returning a future which may move out of the called closure.
Implementors
§
1.85.0
·
Source
§
impl<'a, A, F>
AsyncFnOnce
<A> for
&'a F
where
    A:
Tuple
,
    F:
AsyncFn
<A> + ?
Sized
,
Source
§
type
Output
= <F as
AsyncFnOnce
<A>>::
Output
Source
§
type
CallOnceFuture
= <F as
AsyncFnMut
<A>>::
CallRefFuture
<'a>
1.85.0
·
Source
§
impl<'a, A, F>
AsyncFnOnce
<A> for
&'a mut F
where
    A:
Tuple
,
    F:
AsyncFnMut
<A> + ?
Sized
,
Source
§
type
Output
= <F as
AsyncFnOnce
<A>>::
Output
Source
§
type
CallOnceFuture
= <F as
AsyncFnMut
<A>>::
CallRefFuture
<'a>
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFnOnce
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFnOnce
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
Output
= <F as
AsyncFnOnce
<Args>>::
Output
Source
§
type
CallOnceFuture
= <F as
AsyncFnOnce
<Args>>::
CallOnceFuture