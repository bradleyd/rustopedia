AsyncIterator in std::async_iter - Rust
std
::
async_iter
Trait
AsyncIterator
Copy item path
Source
pub trait AsyncIterator {
    type
Item
;

    // Required method
    fn
poll_next
(
        self:
Pin
<&mut Self>,
        cx: &mut
Context
<'_>,
    ) ->
Poll
<
Option
<Self::
Item
>>;

    // Provided method
    fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>) { ... }
}
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Expand description
A trait for dealing with asynchronous iterators.
This is the main async iterator trait. For more about the concept of async iterators
generally, please see the
module-level documentation
. In particular, you
may want to know how to
implement
AsyncIterator
.
Required Associated Types
§
Source
type
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of items yielded by the async iterator.
Required Methods
§
Source
fn
poll_next
(
    self:
Pin
<&mut Self>,
    cx: &mut
Context
<'_>,
) ->
Poll
<
Option
<Self::
Item
>>
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Attempts to pull out the next value of this async iterator, registering the
current task for wakeup if the value is not yet available, and returning
None
if the async iterator is exhausted.
§
Return value
There are several possible return values, each indicating a distinct
async iterator state:
Poll::Pending
means that this async iterator’s next value is not ready
yet. Implementations will ensure that the current task will be notified
when the next value may be ready.
Poll::Ready(Some(val))
means that the async iterator has successfully
produced a value,
val
, and may produce further values on subsequent
poll_next
calls.
Poll::Ready(None)
means that the async iterator has terminated, and
poll_next
should not be invoked again.
§
Panics
Once an async iterator has finished (returned
Ready(None)
from
poll_next
), calling its
poll_next
method again may panic, block forever, or cause other kinds of
problems; the
AsyncIterator
trait places no requirements on the effects of
such a call. However, as the
poll_next
method is not marked
unsafe
,
Rust’s usual rules apply: calls must never cause undefined behavior
(memory corruption, incorrect use of
unsafe
functions, or the like),
regardless of the async iterator’s state.
Provided Methods
§
Source
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Returns the bounds on the remaining length of the async iterator.
Specifically,
size_hint()
returns a tuple where the first element
is the lower bound, and the second element is the upper bound.
The second half of the tuple that is returned is an
Option
<
usize
>
.
A
None
here means that either there is no known upper bound, or the
upper bound is larger than
usize
.
§
Implementation notes
It is not enforced that an async iterator implementation yields the declared
number of elements. A buggy async iterator may yield less than the lower bound
or more than the upper bound of elements.
size_hint()
is primarily intended to be used for optimizations such as
reserving space for the elements of the async iterator, but must not be
trusted to e.g., omit bounds checks in unsafe code. An incorrect
implementation of
size_hint()
should not lead to memory safety
violations.
That said, the implementation should provide a correct estimation,
because otherwise it would be a violation of the trait’s protocol.
The default implementation returns
(0,
None
)
which is correct for any
async iterator.
Implementors
§
Source
§
impl<I>
AsyncIterator
for
FromIter
<I>
where
    I:
Iterator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
Source
§
impl<P>
AsyncIterator
for
Pin
<P>
where
    P:
DerefMut
,
    <P as
Deref
>::
Target
:
AsyncIterator
,
Source
§
type
Item
= <<P as
Deref
>::
Target
as
AsyncIterator
>::
Item
Source
§
impl<S>
AsyncIterator
for
&mut S
where
    S:
AsyncIterator
+
Unpin
+ ?
Sized
,
Source
§
type
Item
= <S as
AsyncIterator
>::
Item
Source
§
impl<S>
AsyncIterator
for
Box
<S>
where
    S:
AsyncIterator
+
Unpin
+ ?
Sized
,
Source
§
type
Item
= <S as
AsyncIterator
>::
Item
Source
§
impl<S>
AsyncIterator
for
AssertUnwindSafe
<S>
where
    S:
AsyncIterator
,
Source
§
type
Item
= <S as
AsyncIterator
>::
Item