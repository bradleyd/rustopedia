AsyncFn in std::ops - Rust
std
::
ops
Trait
AsyncFn
Copy item path
1.85.0
·
Source
pub trait AsyncFn<Args>:
AsyncFnMut
<Args>
where
    Args:
Tuple
,
{
    // Required method
    extern "rust-call" fn
async_call
(
        &self,
        args: Args,
    ) -> Self::
CallRefFuture
<'_>;
}
Expand description
An async-aware version of the
Fn
trait.
All
async fn
and functions returning futures implement this trait.
Required Methods
§
Source
extern "rust-call" fn
async_call
(
    &self,
    args: Args,
) -> Self::
CallRefFuture
<'_>
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFn
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
AsyncFn
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
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFn
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFn
<Args> + ?
Sized
,
    A:
Allocator
,