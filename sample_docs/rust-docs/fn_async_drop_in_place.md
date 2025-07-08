async_drop_in_place in std::future - Rust
std
::
future
Function
async_drop_in_place
Copy item path
Source
pub unsafe fn async_drop_in_place<T>(to_drop:
*mut T
) ->
AsyncDropInPlace
<T>
ⓘ
where
    T: ?
Sized
,
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
Expand description
Creates the asynchronous destructor of the pointed-to value.
§
Safety
Behavior is undefined if any of the following conditions are violated:
to_drop
must be
valid
for both reads and writes.
to_drop
must be properly aligned, even if
T
has size 0.
to_drop
must be nonnull, even if
T
has size 0.
The value
to_drop
points to must be valid for async dropping,
which may mean it must uphold additional invariants. These
invariants depend on the type of the value being dropped. For
instance, when dropping a Box, the box’s pointer to the heap must
be valid.
While
async_drop_in_place
is executing or the returned async
destructor is alive, the only way to access parts of
to_drop
is through the
self: Pin<&mut Self>
references supplied to
the
AsyncDrop::async_drop
methods that
async_drop_in_place
or
AsyncDropInPlace<T>::poll
invokes. This usually means the
returned future stores the
to_drop
pointer and user is required
to guarantee that dropped value doesn’t move.