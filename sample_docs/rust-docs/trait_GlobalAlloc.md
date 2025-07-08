GlobalAlloc in std::alloc - Rust
std
::
alloc
Trait
GlobalAlloc
Copy item path
1.28.0
·
Source
pub unsafe trait GlobalAlloc {
    // Required methods
    unsafe fn
alloc
(&self, layout:
Layout
) ->
*mut
u8
;
unsafe fn
dealloc
(&self, ptr:
*mut
u8
, layout:
Layout
);

    // Provided methods
    unsafe fn
alloc_zeroed
(&self, layout:
Layout
) ->
*mut
u8
{ ... }
unsafe fn
realloc
(
        &self,
        ptr:
*mut
u8
,
        layout:
Layout
,
        new_size:
usize
,
    ) ->
*mut
u8
{ ... }
}
Expand description
A memory allocator that can be registered as the standard library’s default
through the
#[global_allocator]
attribute.
Some of the methods require that a memory block be
currently
allocated
via an allocator. This means that:
the starting address for that memory block was previously
returned by a previous call to an allocation method
such as
alloc
, and
the memory block has not been subsequently deallocated, where
blocks are deallocated either by being passed to a deallocation
method such as
dealloc
or by being
passed to a reallocation method that returns a non-null pointer.
§
Example
use
std::alloc::{GlobalAlloc, Layout};
use
std::cell::UnsafeCell;
use
std::ptr::null_mut;
use
std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
const
ARENA_SIZE: usize =
128
*
1024
;
const
MAX_SUPPORTED_ALIGN: usize =
4096
;
#[repr(C, align(
4096
))]
// 4096 == MAX_SUPPORTED_ALIGN
struct
SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
// we allocate from the top, counting down
}
#[global_allocator]
static
ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([
0x55
; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};
unsafe impl
Sync
for
SimpleAllocator {}
unsafe impl
GlobalAlloc
for
SimpleAllocator {
unsafe fn
alloc(
&
self
, layout: Layout) ->
*mut
u8 {
let
size = layout.size();
let
align = layout.align();
// `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
let
align_mask_to_round_down = !(align -
1
);
if
align > MAX_SUPPORTED_ALIGN {
return
null_mut();
        }
let
mut
allocated =
0
;
if
self
.remaining
            .fetch_update(Relaxed, Relaxed, |
mut
remaining| {
if
size > remaining {
return
None
;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
Some
(remaining)
            })
            .is_err()
        {
return
null_mut();
        };
unsafe
{
self
.arena.get().cast::<u8>().add(allocated) }
    }
unsafe fn
dealloc(
&
self
, _ptr:
*mut
u8, _layout: Layout) {}
}
fn
main() {
let
_s =
format!
(
"allocating a string!"
);
let
currently = ALLOCATOR.remaining.load(Relaxed);
println!
(
"allocated so far: {}"
, ARENA_SIZE - currently);
}
§
Safety
The
GlobalAlloc
trait is an
unsafe
trait for a number of reasons, and
implementors must ensure that they adhere to these contracts:
It’s undefined behavior if global allocators unwind. This restriction may
be lifted in the future, but currently a panic from any of these
functions may lead to memory unsafety.
Layout
queries and calculations in general must be correct. Callers of
this trait are allowed to rely on the contracts defined on each method,
and implementors must ensure such contracts remain true.
You must not rely on allocations actually happening, even if there are explicit
heap allocations in the source. The optimizer may detect unused allocations that it can either
eliminate entirely or move to the stack and thus never invoke the allocator. The
optimizer may further assume that allocation is infallible, so code that used to fail due
to allocator failures may now suddenly work because the optimizer worked around the
need for an allocation. More concretely, the following code example is unsound, irrespective
of whether your custom allocator allows counting how many allocations have happened.
ⓘ
drop(Box::new(
42
));
let
number_of_heap_allocs =
/* call private allocator API */
;
unsafe
{ std::hint::assert_unchecked(number_of_heap_allocs >
0
); }
Note that the optimizations mentioned above are not the only
optimization that can be applied. You may generally not rely on heap allocations
happening if they can be removed without changing program behavior.
Whether allocations happen or not is not part of the program behavior, even if it
could be detected via an allocator that tracks allocations by printing or otherwise
having side effects.
Required Methods
§
1.28.0
·
Source
unsafe fn
alloc
(&self, layout:
Layout
) ->
*mut
u8
Allocates memory as described by the given
layout
.
Returns a pointer to newly-allocated memory,
or null to indicate allocation failure.
§
Safety
layout
must have non-zero size. Attempting to allocate for a zero-sized
layout
may
result in undefined behavior.
(Extension subtraits might provide more specific bounds on
behavior, e.g., guarantee a sentinel address or a null pointer
in response to a zero-size allocation request.)
The allocated block of memory may or may not be initialized.
§
Errors
Returning a null pointer indicates that either memory is exhausted
or
layout
does not meet this allocator’s size or alignment constraints.
Implementations are encouraged to return null on memory
exhaustion rather than aborting, but this is not
a strict requirement. (Specifically: it is
legal
to
implement this trait atop an underlying native allocation
library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an
allocation error are encouraged to call the
handle_alloc_error
function,
rather than directly invoking
panic!
or similar.
1.28.0
·
Source
unsafe fn
dealloc
(&self, ptr:
*mut
u8
, layout:
Layout
)
Deallocates the block of memory at the given
ptr
pointer with the given
layout
.
§
Safety
The caller must ensure:
ptr
is a block of memory currently allocated via this allocator and,
layout
is the same layout that was used to allocate that block of
memory.
Otherwise undefined behavior can result.
Provided Methods
§
1.28.0
·
Source
unsafe fn
alloc_zeroed
(&self, layout:
Layout
) ->
*mut
u8
Behaves like
alloc
, but also ensures that the contents
are set to zero before being returned.
§
Safety
The caller has to ensure that
layout
has non-zero size. Like
alloc
zero sized
layout
can result in undefined behavior.
However the allocated block of memory is guaranteed to be initialized.
§
Errors
Returning a null pointer indicates that either memory is exhausted
or
layout
does not meet allocator’s size or alignment constraints,
just as in
alloc
.
Clients wishing to abort computation in response to an
allocation error are encouraged to call the
handle_alloc_error
function,
rather than directly invoking
panic!
or similar.
1.28.0
·
Source
unsafe fn
realloc
(
    &self,
    ptr:
*mut
u8
,
    layout:
Layout
,
    new_size:
usize
,
) ->
*mut
u8
Shrinks or grows a block of memory to the given
new_size
in bytes.
The block is described by the given
ptr
pointer and
layout
.
If this returns a non-null pointer, then ownership of the memory block
referenced by
ptr
has been transferred to this allocator.
Any access to the old
ptr
is Undefined Behavior, even if the
allocation remained in-place. The newly returned pointer is the only valid pointer
for accessing this memory now.
The new memory block is allocated with
layout
,
but with the
size
updated to
new_size
in bytes.
This new layout must be used when deallocating the new memory block with
dealloc
.
The range
0..min(layout.size(), new_size)
of the new memory block is
guaranteed to have the same values as the original block.
If this method returns null, then ownership of the memory
block has not been transferred to this allocator, and the
contents of the memory block are unaltered.
§
Safety
The caller must ensure that:
ptr
is allocated via this allocator,
layout
is the same layout that was used
to allocate that block of memory,
new_size
is greater than zero.
new_size
, when rounded up to the nearest multiple of
layout.align()
,
does not overflow
isize
(i.e., the rounded value must be less than or
equal to
isize::MAX
).
If these are not followed, undefined behavior can result.
(Extension subtraits might provide more specific bounds on
behavior, e.g., guarantee a sentinel address or a null pointer
in response to a zero-size allocation request.)
§
Errors
Returns null if the new layout does not meet the size
and alignment constraints of the allocator, or if reallocation
otherwise fails.
Implementations are encouraged to return null on memory
exhaustion rather than panicking or aborting, but this is not
a strict requirement. (Specifically: it is
legal
to
implement this trait atop an underlying native allocation
library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to a
reallocation error are encouraged to call the
handle_alloc_error
function,
rather than directly invoking
panic!
or similar.
Implementors
§
1.28.0
·
Source
§
impl
GlobalAlloc
for
System