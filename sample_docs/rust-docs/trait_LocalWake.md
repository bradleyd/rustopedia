LocalWake in std::task - Rust
std
::
task
Trait
LocalWake
Copy item path
Source
pub trait LocalWake {
    // Required method
    fn
wake
(self:
Rc
<Self>);

    // Provided method
    fn
wake_by_ref
(self: &
Rc
<Self>) { ... }
}
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Expand description
An analogous trait to
Wake
but used to construct a
LocalWaker
.
This API works in exactly the same way as
Wake
,
except that it uses an
Rc
instead of an
Arc
,
and the result is a
LocalWaker
instead of a
Waker
.
The benefits of using
LocalWaker
over
Waker
are that it allows the local waker
to hold data that does not implement
Send
and
Sync
. Additionally, it saves calls
to
Arc::clone
, which requires atomic synchronization.
§
Examples
This is a simplified example of a
spawn
and a
block_on
function. The
spawn
function
is used to push new tasks onto the run queue, while the block on function will remove them
and poll them. When a task is woken, it will put itself back on the run queue to be polled
by the executor.
Note:
This example trades correctness for simplicity. A real world example would interleave
poll calls with calls to an io reactor to wait for events instead of spinning on a loop.
#![feature(local_waker)]
use
std::task::{LocalWake, ContextBuilder, LocalWaker, Waker};
use
std::future::Future;
use
std::pin::Pin;
use
std::rc::Rc;
use
std::cell::RefCell;
use
std::collections::VecDeque;
thread_local!
{
// A queue containing all tasks ready to do progress
static
RUN_QUEUE: RefCell<VecDeque<Rc<Task>>> = RefCell::default();
}
type
BoxedFuture = Pin<Box<
dyn
Future<Output = ()>>>;
struct
Task(RefCell<BoxedFuture>);
impl
LocalWake
for
Task {
fn
wake(
self
: Rc<
Self
>) {
        RUN_QUEUE.with_borrow_mut(|queue| {
            queue.push_back(
self
)
        })
    }
}
fn
spawn<F>(future: F)
where
F: Future<Output=()> +
'static
+ Send + Sync
{
let
task = RefCell::new(Box::pin(future));
    RUN_QUEUE.with_borrow_mut(|queue| {
        queue.push_back(Rc::new(Task(task)));
    });
}
fn
block_on<F>(future: F)
where
F: Future<Output=()> +
'static
+ Sync + Send
{
    spawn(future);
loop
{
let
Some
(task) = RUN_QUEUE.with_borrow_mut(|queue| queue.pop_front())
else
{
// we exit, since there are no more tasks remaining on the queue
return
;
        };
// cast the Rc<Task> into a `LocalWaker`
let
local_waker: LocalWaker = task.clone().into();
// Build the context using `ContextBuilder`
let
mut
cx = ContextBuilder::from_waker(Waker::noop())
            .local_waker(
&
local_waker)
            .build();
// Poll the task
let _
= task.
0
.borrow_mut()
            .as_mut()
            .poll(
&mut
cx);
    }
}

block_on(
async
{
println!
(
"hello world"
);
});
Required Methods
§
Source
fn
wake
(self:
Rc
<Self>)
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Wake this task.
Provided Methods
§
Source
fn
wake_by_ref
(self: &
Rc
<Self>)
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Wake this task without consuming the local waker.
If an executor supports a cheaper way to wake without consuming the
waker, it should override this method. By default, it clones the
Rc
and calls
wake
on the clone.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§