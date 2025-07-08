Allocator in std::alloc - Rust
std
::
alloc
Trait
Allocator
Copy item path
Source
pub unsafe trait Allocator {
    // Required methods
    fn
allocate
(&self, layout:
Layout
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>;
unsafe fn
deallocate
(&self, ptr:
NonNull
<
u8
>, layout:
Layout
);

    // Provided methods
    fn
allocate_zeroed
(
        &self,
        layout:
Layout
,
    ) ->
Result
<
NonNull
<[
u8
]>,
AllocError
> { ... }
unsafe fn
grow
(
        &self,
        ptr:
NonNull
<
u8
>,
        old_layout:
Layout
,
        new_layout:
Layout
,
    ) ->
Result
<
NonNull
<[
u8
]>,
AllocError
> { ... }
unsafe fn
grow_zeroed
(
        &self,
        ptr:
NonNull
<
u8
>,
        old_layout:
Layout
,
        new_layout:
Layout
,
    ) ->
Result
<
NonNull
<[
u8
]>,
AllocError
> { ... }
unsafe fn
shrink
(
        &self,
        ptr:
NonNull
<
u8
>,
        old_layout:
Layout
,
        new_layout:
Layout
,
    ) ->
Result
<
NonNull
<[
u8
]>,
AllocError
> { ... }
fn
by_ref
(&self) -> &Self
where Self:
Sized
{ ... }
}
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Expand description
An implementation of
Allocator
can allocate, grow, shrink, and deallocate arbitrary blocks of
data described via
Layout
.
Allocator
is designed to be implemented on ZSTs, references, or smart pointers.
An allocator for
MyAlloc([u8; N])
cannot be moved, without updating the pointers to the
allocated memory.
In contrast to
GlobalAlloc
,
Allocator
allows zero-sized allocations. If an underlying
allocator does not support this (like jemalloc) or responds by returning a null pointer
(such as
libc::malloc
), this must be caught by the implementation.
§
Currently allocated memory
Some of the methods require that a memory block is
currently allocated
by an allocator.
This means that:
the starting address for that memory block was previously
returned by
allocate
,
grow
, or
shrink
, and
the memory block has not subsequently been deallocated.
A memory block is deallocated by a call to
deallocate
,
or by a call to
grow
or
shrink
that returns
Ok
.
A call to
grow
or
shrink
that returns
Err
,
does not deallocate the memory block passed to it.
§
Memory fitting
Some of the methods require that a
layout
fit
a memory block or vice versa. This means that the
following conditions must hold:
the memory block must be
currently allocated
with alignment of
layout.align()
, and
layout.size()
must fall in the range
min ..= max
, where:
min
is the size of the layout used to allocate the block, and
max
is the actual size returned from
allocate
,
grow
, or
shrink
.
§
Safety
Memory blocks that are
currently allocated
by an allocator,
must point to valid memory, and retain their validity while until either:
the memory block is deallocated, or
the allocator is dropped.
Copying, cloning, or moving the allocator must not invalidate memory blocks returned from it.
A copied or cloned allocator must behave like the original allocator.
A memory block which is
currently allocated
may be passed to
any method of the allocator that accepts such an argument.
Required Methods
§
Source
fn
allocate
(&self, layout:
Layout
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to allocate a block of memory.
On success, returns a
NonNull<[u8]>
meeting the size and alignment guarantees of
layout
.
The returned block may have a larger size than specified by
layout.size()
, and may or may
not have its contents initialized.
The returned block of memory remains valid as long as it is [
currently allocated
] and the shorter of:
the borrow-checker lifetime of the allocator type itself.
as long as at the allocator and all its clones has not been dropped.
§
Errors
Returning
Err
indicates that either memory is exhausted or
layout
does not meet
allocator’s size or alignment constraints.
Implementations are encouraged to return
Err
on memory exhaustion rather than panicking or
aborting, but this is not a strict requirement. (Specifically: it is
legal
to implement
this trait atop an underlying native allocation library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an allocation error are encouraged to
call the
handle_alloc_error
function, rather than directly invoking
panic!
or similar.
Source
unsafe fn
deallocate
(&self, ptr:
NonNull
<
u8
>, layout:
Layout
)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Deallocates the memory referenced by
ptr
.
§
Safety
ptr
must denote a block of memory
currently allocated
via this allocator, and
layout
must
fit
that block of memory.
Provided Methods
§
Source
fn
allocate_zeroed
(&self, layout:
Layout
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Behaves like
allocate
, but also ensures that the returned memory is zero-initialized.
§
Errors
Returning
Err
indicates that either memory is exhausted or
layout
does not meet
allocator’s size or alignment constraints.
Implementations are encouraged to return
Err
on memory exhaustion rather than panicking or
aborting, but this is not a strict requirement. (Specifically: it is
legal
to implement
this trait atop an underlying native allocation library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an allocation error are encouraged to
call the
handle_alloc_error
function, rather than directly invoking
panic!
or similar.
Source
unsafe fn
grow
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to extend the memory block.
Returns a new
NonNull<[u8]>
containing a pointer and the actual size of the allocated
memory. The pointer is suitable for holding data described by
new_layout
. To accomplish
this, the allocator may extend the allocation referenced by
ptr
to fit the new layout.
If this returns
Ok
, then ownership of the memory block referenced by
ptr
has been
transferred to this allocator. Any access to the old
ptr
is Undefined Behavior, even if the
allocation was grown in-place. The newly returned pointer is the only valid pointer
for accessing this memory now.
If this method returns
Err
, then ownership of the memory block has not been transferred to
this allocator, and the contents of the memory block are unaltered.
§
Safety
ptr
must denote a block of memory
currently allocated
via this allocator.
old_layout
must
fit
that block of memory (The
new_layout
argument need not fit it.).
new_layout.size()
must be greater than or equal to
old_layout.size()
.
Note that
new_layout.align()
need not be the same as
old_layout.align()
.
§
Errors
Returns
Err
if the new layout does not meet the allocator’s size and alignment
constraints of the allocator, or if growing otherwise fails.
Implementations are encouraged to return
Err
on memory exhaustion rather than panicking or
aborting, but this is not a strict requirement. (Specifically: it is
legal
to implement
this trait atop an underlying native allocation library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an allocation error are encouraged to
call the
handle_alloc_error
function, rather than directly invoking
panic!
or similar.
Source
unsafe fn
grow_zeroed
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Behaves like
grow
, but also ensures that the new contents are set to zero before being
returned.
The memory block will contain the following contents after a successful call to
grow_zeroed
:
Bytes
0..old_layout.size()
are preserved from the original allocation.
Bytes
old_layout.size()..old_size
will either be preserved or zeroed, depending on
the allocator implementation.
old_size
refers to the size of the memory block prior
to the
grow_zeroed
call, which may be larger than the size that was originally
requested when it was allocated.
Bytes
old_size..new_size
are zeroed.
new_size
refers to the size of the memory
block returned by the
grow_zeroed
call.
§
Safety
ptr
must denote a block of memory
currently allocated
via this allocator.
old_layout
must
fit
that block of memory (The
new_layout
argument need not fit it.).
new_layout.size()
must be greater than or equal to
old_layout.size()
.
Note that
new_layout.align()
need not be the same as
old_layout.align()
.
§
Errors
Returns
Err
if the new layout does not meet the allocator’s size and alignment
constraints of the allocator, or if growing otherwise fails.
Implementations are encouraged to return
Err
on memory exhaustion rather than panicking or
aborting, but this is not a strict requirement. (Specifically: it is
legal
to implement
this trait atop an underlying native allocation library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an allocation error are encouraged to
call the
handle_alloc_error
function, rather than directly invoking
panic!
or similar.
Source
unsafe fn
shrink
(
    &self,
    ptr:
NonNull
<
u8
>,
    old_layout:
Layout
,
    new_layout:
Layout
,
) ->
Result
<
NonNull
<[
u8
]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Attempts to shrink the memory block.
Returns a new
NonNull<[u8]>
containing a pointer and the actual size of the allocated
memory. The pointer is suitable for holding data described by
new_layout
. To accomplish
this, the allocator may shrink the allocation referenced by
ptr
to fit the new layout.
If this returns
Ok
, then ownership of the memory block referenced by
ptr
has been
transferred to this allocator. Any access to the old
ptr
is Undefined Behavior, even if the
allocation was shrunk in-place. The newly returned pointer is the only valid pointer
for accessing this memory now.
If this method returns
Err
, then ownership of the memory block has not been transferred to
this allocator, and the contents of the memory block are unaltered.
§
Safety
ptr
must denote a block of memory
currently allocated
via this allocator.
old_layout
must
fit
that block of memory (The
new_layout
argument need not fit it.).
new_layout.size()
must be smaller than or equal to
old_layout.size()
.
Note that
new_layout.align()
need not be the same as
old_layout.align()
.
§
Errors
Returns
Err
if the new layout does not meet the allocator’s size and alignment
constraints of the allocator, or if shrinking otherwise fails.
Implementations are encouraged to return
Err
on memory exhaustion rather than panicking or
aborting, but this is not a strict requirement. (Specifically: it is
legal
to implement
this trait atop an underlying native allocation library that aborts on memory exhaustion.)
Clients wishing to abort computation in response to an allocation error are encouraged to
call the
handle_alloc_error
function, rather than directly invoking
panic!
or similar.
Source
fn
by_ref
(&self) -> &Self
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates a “by reference” adapter for this instance of
Allocator
.
The returned adapter also implements
Allocator
and will simply borrow this.
Implementors
§
Source
§
impl
Allocator
for
Global
Source
§
impl
Allocator
for
System
Source
§
impl<A>
Allocator
for
&A
where
    A:
Allocator
+ ?
Sized
,