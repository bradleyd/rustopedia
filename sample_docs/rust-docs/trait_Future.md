Future in std::future - Rust
std
::
future
Trait
Future
Copy item path
1.36.0
·
Source
pub trait Future {
    type
Output
;

    // Required method
    fn
poll
(self:
Pin
<&mut Self>, cx: &mut
Context
<'_>) ->
Poll
<Self::
Output
>;
}
Expand description
A future represents an asynchronous computation obtained by use of
async
.
A future is a value that might not have finished computing yet. This kind of
“asynchronous value” makes it possible for a thread to continue doing useful
work while it waits for the value to become available.
§
The
poll
method
The core method of future,
poll
,
attempts
to resolve the future into a
final value. This method does not block if the value is not ready. Instead,
the current task is scheduled to be woken up when it’s possible to make
further progress by
poll
ing again. The
context
passed to the
poll
method can provide a
Waker
, which is a handle for waking up the current
task.
When using a future, you generally won’t call
poll
directly, but instead
.await
the value.
Required Associated Types
§
1.36.0
·
Source
type
Output
The type of value produced on completion.
Required Methods
§
1.36.0
·
Source
fn
poll
(self:
Pin
<&mut Self>, cx: &mut
Context
<'_>) ->
Poll
<Self::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
§
Return value
This function returns:
Poll::Pending
if the future is not ready yet
Poll::Ready(val)
with the result
val
of this future if it
finished successfully.
Once a future has finished, clients should not
poll
it again.
When a future is not ready yet,
poll
returns
Poll::Pending
and
stores a clone of the
Waker
copied from the current
Context
.
This
Waker
is then woken once the future can make progress.
For example, a future waiting for a socket to become
readable would call
.clone()
on the
Waker
and store it.
When a signal arrives elsewhere indicating that the socket is readable,
Waker::wake
is called and the socket future’s task is awoken.
Once a task has been woken up, it should attempt to
poll
the future
again, which may or may not produce a final value.
Note that on multiple calls to
poll
, only the
Waker
from the
Context
passed to the most recent call should be scheduled to
receive a wakeup.
§
Runtime characteristics
Futures alone are
inert
; they must be
actively
poll
ed to make
progress, meaning that each time the current task is woken up, it should
actively re-
poll
pending futures that it still has an interest in.
The
poll
function is not called repeatedly in a tight loop – instead,
it should only be called when the future indicates that it is ready to
make progress (by calling
wake()
). If you’re familiar with the
poll(2)
or
select(2)
syscalls on Unix it’s worth noting that futures
typically do
not
suffer the same problems of “all wakeups must poll
all events”; they are more like
epoll(4)
.
An implementation of
poll
should strive to return quickly, and should
not block. Returning quickly prevents unnecessarily clogging up
threads or event loops. If it is known ahead of time that a call to
poll
may end up taking a while, the work should be offloaded to a
thread pool (or something similar) to ensure that
poll
can return
quickly.
§
Panics
Once a future has completed (returned
Ready
from
poll
), calling its
poll
method again may panic, block forever, or cause other kinds of
problems; the
Future
trait places no requirements on the effects of
such a call. However, as the
poll
method is not marked
unsafe
,
Rust’s usual rules apply: calls must never cause undefined behavior
(memory corruption, incorrect use of
unsafe
functions, or the like),
regardless of the future’s state.
Implementors
§
1.36.0
·
Source
§
impl<F>
Future
for
&mut F
where
    F:
Future
+
Unpin
+ ?
Sized
,
Source
§
type
Output
= <F as
Future
>::
Output
1.36.0
·
Source
§
impl<F>
Future
for
AssertUnwindSafe
<F>
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
1.36.0
·
Source
§
impl<F, A>
Future
for
Box
<F, A>
where
    F:
Future
+
Unpin
+ ?
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
Future
>::
Output
1.36.0
·
Source
§
impl<P>
Future
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
Future
,
Source
§
type
Output
= <<P as
Deref
>::
Target
as
Future
>::
Output
Source
§
impl<T>
Future
for
Exclusive
<T>
where
    T:
Future
+ ?
Sized
,
Source
§
type
Output
= <T as
Future
>::
Output
Source
§
impl<T>
Future
for
AsyncDropInPlace
<T>
where
    T: ?
Sized
,
Source
§
type
Output
=
()
1.48.0
·
Source
§
impl<T>
Future
for
Pending
<T>
Source
§
type
Output
= T
1.48.0
·
Source
§
impl<T>
Future
for
Ready
<T>
Source
§
type
Output
= T
1.64.0
·
Source
§
impl<T, F>
Future
for
PollFn
<F>
where
    F:
FnMut
(&mut
Context
<'_>) ->
Poll
<T>,
Source
§
type
Output
= T