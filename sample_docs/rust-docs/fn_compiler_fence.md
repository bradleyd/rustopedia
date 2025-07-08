compiler_fence in std::sync::atomic - Rust
std
::
sync
::
atomic
Function
compiler_fence
Copy item path
1.21.0
·
Source
pub fn compiler_fence(order:
Ordering
)
Expand description
A “compiler-only” atomic fence.
Like
fence
, this function establishes synchronization with other atomic operations and
fences. However, unlike
fence
,
compiler_fence
only establishes synchronization with
operations
in the same thread
. This may at first sound rather useless, since code within a
thread is typically already totally ordered and does not need any further synchronization.
However, there are cases where code can run on the same thread without being ordered:
The most common case is that of a
signal handler
: a signal handler runs in the same thread
as the code it interrupted, but it is not ordered with respect to that code.
compiler_fence
can be used to establish synchronization between a thread and its signal handler, the same way
that
fence
can be used to establish synchronization across threads.
Similar situations can arise in embedded programming with interrupt handlers, or in custom
implementations of preemptive green threads. In general,
compiler_fence
can establish
synchronization with code that is guaranteed to run on the same hardware CPU.
See
fence
for how a fence can be used to achieve synchronization. Note that just like
fence
, synchronization still requires atomic operations to be used in both threads – it is
not possible to perform synchronization entirely with fences and non-atomic operations.
compiler_fence
does not emit any machine code, but restricts the kinds of memory re-ordering
the compiler is allowed to do.
compiler_fence
corresponds to
atomic_signal_fence
in C and
C++.
§
Panics
Panics if
order
is
Relaxed
.
§
Examples
Without the two
compiler_fence
calls, the read of
IMPORTANT_VARIABLE
in
signal_handler
is
undefined behavior
due to a data race, despite everything happening in a single thread.
This is because the signal handler is considered to run concurrently with its associated
thread, and explicit synchronization is required to pass data between a thread and its
signal handler. The code below uses two
compiler_fence
calls to establish the usual
release-acquire synchronization pattern (see
fence
for an image).
use
std::sync::atomic::AtomicBool;
use
std::sync::atomic::Ordering;
use
std::sync::atomic::compiler_fence;
static
mut
IMPORTANT_VARIABLE: usize =
0
;
static
IS_READY: AtomicBool = AtomicBool::new(
false
);
fn
main() {
unsafe
{ IMPORTANT_VARIABLE =
42
};
// Marks earlier writes as being released with future relaxed stores.
compiler_fence(Ordering::Release);
    IS_READY.store(
true
, Ordering::Relaxed);
}
fn
signal_handler() {
if
IS_READY.load(Ordering::Relaxed) {
// Acquires writes that were released with relaxed stores that we read from.
compiler_fence(Ordering::Acquire);
assert_eq!
(
unsafe
{ IMPORTANT_VARIABLE },
42
);
    }
}