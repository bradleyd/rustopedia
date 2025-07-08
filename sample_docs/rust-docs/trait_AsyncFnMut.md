AsyncFnMut in std::ops - Rust
std
::
ops
Trait
AsyncFnMut
Copy item path
1.85.0
·
Source
pub trait AsyncFnMut<Args>:
AsyncFnOnce
<Args>
where
    Args:
Tuple
,
{
    type
CallRefFuture
<'a>:
Future
<Output = Self::
Output
>
where Self: 'a
;

    // Required method
    extern "rust-call" fn
async_call_mut
(
        &mut self,
        args: Args,
    ) -> Self::
CallRefFuture
<'_>;
}
Expand description
An async-aware version of the
FnMut
trait.
All
async fn
and functions returning futures implement this trait.
Required Associated Types
§
Source
type
CallRefFuture
<'a>:
Future
<Output = Self::
Output
>
where
    Self: 'a
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Future returned by
AsyncFnMut::async_call_mut
and
AsyncFn::async_call
.
Required Methods
§
Source
extern "rust-call" fn
async_call_mut
(
    &mut self,
    args: Args,
) -> Self::
CallRefFuture
<'_>
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFnMut
, returning a future which may borrow from the called closure.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.85.0
·
Source
§
impl<A, F>
AsyncFnMut
<A> for
&F
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
CallRefFuture
<'a> = <F as
AsyncFnMut
<A>>::
CallRefFuture
<'a>
where
&F
: 'a
1.85.0
·
Source
§
impl<A, F>
AsyncFnMut
<A> for
&mut F
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
CallRefFuture
<'a> = <F as
AsyncFnMut
<A>>::
CallRefFuture
<'a>
where
&mut F
: 'a
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFnMut
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFnMut
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
CallRefFuture
<'a> = <F as
AsyncFnMut
<Args>>::
CallRefFuture
<'a>
where
Box
<F, A>: 'a