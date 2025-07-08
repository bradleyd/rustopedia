fence in std::sync::atomic - Rust
std
::
sync
::
atomic
Function
fence
Copy item path
1.0.0
·
Source
pub fn fence(order:
Ordering
)
Expand description
An atomic fence.
Fences create synchronization between themselves and atomic operations or fences in other
threads. To achieve this, a fence prevents the compiler and CPU from reordering certain types of
memory operations around it.
A fence ‘A’ which has (at least)
Release
ordering semantics, synchronizes
with a fence ‘B’ with (at least)
Acquire
semantics, if and only if there
exist operations X and Y, both operating on some atomic object ‘m’ such
that A is sequenced before X, Y is sequenced before B and Y observes
the change to m. This provides a happens-before dependence between A and B.
Thread 1                                          Thread 2

fence(Release);      A --------------
m.store(3, Relaxed); X ---------    |
                               |    |
                               |    |
                               -------------> Y  if m.load(Relaxed) == 3 {
                                    |-------> B      fence(Acquire);
                                                     ...
                                                 }
Note that in the example above, it is crucial that the accesses to
m
are atomic. Fences cannot
be used to establish synchronization among non-atomic accesses in different threads. However,
thanks to the happens-before relationship between A and B, any non-atomic accesses that
happen-before A are now also properly synchronized with any non-atomic accesses that
happen-after B.
Atomic operations with
Release
or
Acquire
semantics can also synchronize
with a fence.
A fence which has
SeqCst
ordering, in addition to having both
Acquire
and
Release
semantics, participates in the global program order of the
other
SeqCst
operations and/or fences.
Accepts
Acquire
,
Release
,
AcqRel
and
SeqCst
orderings.
§
Panics
Panics if
order
is
Relaxed
.
§
Examples
use
std::sync::atomic::AtomicBool;
use
std::sync::atomic::fence;
use
std::sync::atomic::Ordering;
// A mutual exclusion primitive based on spinlock.
pub struct
Mutex {
    flag: AtomicBool,
}
impl
Mutex {
pub fn
new() -> Mutex {
        Mutex {
            flag: AtomicBool::new(
false
),
        }
    }
pub fn
lock(
&
self
) {
// Wait until the old value is `false`.
while
self
.flag
            .compare_exchange_weak(
false
,
true
, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {}
// This fence synchronizes-with store in `unlock`.
fence(Ordering::Acquire);
    }
pub fn
unlock(
&
self
) {
self
.flag.store(
false
, Ordering::Release);
    }
}