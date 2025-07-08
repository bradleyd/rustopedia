needs_drop in std::mem - Rust
std
::
mem
Function
needs_drop
Copy item path
1.21.0 (const: 1.36.0)
·
Source
pub const fn needs_drop<T>() ->
bool
where
    T: ?
Sized
,
Expand description
Returns
true
if dropping values of type
T
matters.
This is purely an optimization hint, and may be implemented conservatively:
it may return
true
for types that don’t actually need to be dropped.
As such always returning
true
would be a valid implementation of
this function. However if this function actually returns
false
, then you
can be certain dropping
T
has no side effect.
Low level implementations of things like collections, which need to manually
drop their data, should use this function to avoid unnecessarily
trying to drop all their contents when they are destroyed. This might not
make a difference in release builds (where a loop that has no side-effects
is easily detected and eliminated), but is often a big win for debug builds.
Note that
drop_in_place
already performs this check, so if your workload
can be reduced to some small number of
drop_in_place
calls, using this is
unnecessary. In particular note that you can
drop_in_place
a slice, and that
will do a single needs_drop check for all the values.
Types like Vec therefore just
drop_in_place(&mut self[..])
without using
needs_drop
explicitly. Types like
HashMap
, on the other hand, have to drop
values one at a time and should use this API.
§
Examples
Here’s an example of how a collection might make use of
needs_drop
:
use
std::{mem, ptr};
pub struct
MyCollection<T> {
/* ... */
}
impl
<T> Drop
for
MyCollection<T> {
fn
drop(
&mut
self
) {
unsafe
{
// drop the data
if
mem::needs_drop::<T>() {
for
x
in
self
.iter_mut() {
                    ptr::drop_in_place(x);
                }
            }
self
.free_buffer();
        }
    }
}