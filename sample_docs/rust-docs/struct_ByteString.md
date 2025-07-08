ByteString in std::bstr - Rust
std
::
bstr
Struct
ByteString
Copy item path
Source
#[repr(transparent)]
pub struct ByteString(pub
Vec
<
u8
>);
🔬
This is a nightly-only experimental API. (
bstr
#134915
)
Expand description
A wrapper for
Vec<u8>
representing a human-readable string that’s conventionally, but not
always, UTF-8.
Unlike
String
, this type permits non-UTF-8 contents, making it suitable for user input,
non-native filenames (as
Path
only supports native filenames), and other applications that
need to round-trip whatever data the user provides.
A
ByteString
owns its contents and can grow and shrink, like a
Vec
or
String
. For a
borrowed byte string, see
ByteStr
.
ByteString
implements
Deref
to
&Vec<u8>
, so all methods available on
&Vec<u8>
are
available on
ByteString
. Similarly,
ByteString
implements
DerefMut
to
&mut Vec<u8>
,
so you can modify a
ByteString
using any method available on
&mut Vec<u8>
.
The
Debug
and
Display
implementations for
ByteString
are the same as those for
ByteStr
,
showing invalid UTF-8 as hex escapes or the Unicode replacement character, respectively.
Tuple Fields
§
§
0:
Vec
<
u8
>
🔬
This is a nightly-only experimental API. (
bstr
#134915
)
Methods from
Deref
<Target =
Vec
<
u8
>>
§
1.0.0
·
Source
pub fn
capacity
(&self) ->
usize
Returns the total number of elements the vector can hold without
reallocating.
§
Examples
let
mut
vec: Vec<i32> = Vec::with_capacity(
10
);
vec.push(
42
);
assert!
(vec.capacity() >=
10
);
A vector with zero-sized elements will always have a capacity of usize::MAX:
#[derive(Clone)]
struct
ZeroSized;
fn
main() {
assert_eq!
(std::mem::size_of::<ZeroSized>(),
0
);
let
v =
vec!
[ZeroSized;
0
];
assert_eq!
(v.capacity(), usize::MAX);
}
1.0.0
·
Source
pub fn
reserve
(&mut self, additional:
usize
)
Reserves capacity for at least
additional
more elements to be inserted
in the given
Vec<T>
. The collection may reserve more space to
speculatively avoid frequent reallocations. After calling
reserve
,
capacity will be greater than or equal to
self.len() + additional
.
Does nothing if capacity is already sufficient.
§
Panics
Panics if the new capacity exceeds
isize::MAX
bytes
.
§
Examples
let
mut
vec =
vec!
[
1
];
vec.reserve(
10
);
assert!
(vec.capacity() >=
11
);
1.0.0
·
Source
pub fn
reserve_exact
(&mut self, additional:
usize
)
Reserves the minimum capacity for at least
additional
more elements to
be inserted in the given
Vec<T>
. Unlike
reserve
, this will not
deliberately over-allocate to speculatively avoid frequent allocations.
After calling
reserve_exact
, capacity will be greater than or equal to
self.len() + additional
. Does nothing if the capacity is already
sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
reserve
if future insertions are expected.
§
Panics
Panics if the new capacity exceeds
isize::MAX
bytes
.
§
Examples
let
mut
vec =
vec!
[
1
];
vec.reserve_exact(
10
);
assert!
(vec.capacity() >=
11
);
1.57.0
·
Source
pub fn
try_reserve
(&mut self, additional:
usize
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve capacity for at least
additional
more elements to be inserted
in the given
Vec<T>
. The collection may reserve more space to speculatively avoid
frequent reallocations. After calling
try_reserve
, capacity will be
greater than or equal to
self.len() + additional
if it returns
Ok(())
. Does nothing if capacity is already sufficient. This method
preserves the contents even if an error occurs.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
fn
process_data(data:
&
[u32]) ->
Result
<Vec<u32>, TryReserveError> {
let
mut
output = Vec::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
output.extend(data.iter().map(|
&
val| {
        val *
2
+
5
// very complicated
}));
Ok
(output)
}
1.57.0
·
Source
pub fn
try_reserve_exact
(
    &mut self,
    additional:
usize
,
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve the minimum capacity for at least
additional
elements to be inserted in the given
Vec<T>
. Unlike
try_reserve
,
this will not deliberately over-allocate to speculatively avoid frequent
allocations. After calling
try_reserve_exact
, capacity will be greater
than or equal to
self.len() + additional
if it returns
Ok(())
.
Does nothing if the capacity is already sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
try_reserve
if future insertions are expected.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
fn
process_data(data:
&
[u32]) ->
Result
<Vec<u32>, TryReserveError> {
let
mut
output = Vec::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve_exact(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
output.extend(data.iter().map(|
&
val| {
        val *
2
+
5
// very complicated
}));
Ok
(output)
}
1.0.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Shrinks the capacity of the vector as much as possible.
The behavior of this method depends on the allocator, which may either shrink the vector
in-place or reallocate. The resulting vector might still have some excess capacity, just as
is the case for
with_capacity
. See
Allocator::shrink
for more details.
§
Examples
let
mut
vec = Vec::with_capacity(
10
);
vec.extend([
1
,
2
,
3
]);
assert!
(vec.capacity() >=
10
);
vec.shrink_to_fit();
assert!
(vec.capacity() >=
3
);
1.56.0
·
Source
pub fn
shrink_to
(&mut self, min_capacity:
usize
)
Shrinks the capacity of the vector with a lower bound.
The capacity will remain at least as large as both the length
and the supplied value.
If the current capacity is less than the lower limit, this is a no-op.
§
Examples
let
mut
vec = Vec::with_capacity(
10
);
vec.extend([
1
,
2
,
3
]);
assert!
(vec.capacity() >=
10
);
vec.shrink_to(
4
);
assert!
(vec.capacity() >=
4
);
vec.shrink_to(
0
);
assert!
(vec.capacity() >=
3
);
1.0.0
·
Source
pub fn
truncate
(&mut self, len:
usize
)
Shortens the vector, keeping the first
len
elements and dropping
the rest.
If
len
is greater or equal to the vector’s current length, this has
no effect.
The
drain
method can emulate
truncate
, but causes the excess
elements to be returned instead of dropped.
Note that this method has no effect on the allocated capacity
of the vector.
§
Examples
Truncating a five element vector to two elements:
let
mut
vec =
vec!
[
1
,
2
,
3
,
4
,
5
];
vec.truncate(
2
);
assert_eq!
(vec, [
1
,
2
]);
No truncation occurs when
len
is greater than the vector’s current
length:
let
mut
vec =
vec!
[
1
,
2
,
3
];
vec.truncate(
8
);
assert_eq!
(vec, [
1
,
2
,
3
]);
Truncating when
len == 0
is equivalent to calling the
clear
method.
let
mut
vec =
vec!
[
1
,
2
,
3
];
vec.truncate(
0
);
assert_eq!
(vec, []);
1.7.0
·
Source
pub fn
as_slice
(&self) -> &
[T]
Extracts a slice containing the entire vector.
Equivalent to
&s[..]
.
§
Examples
use
std::io::{
self
, Write};
let
buffer =
vec!
[
1
,
2
,
3
,
5
,
8
];
io::sink().write(buffer.as_slice()).unwrap();
1.7.0
·
Source
pub fn
as_mut_slice
(&mut self) -> &mut
[T]
Extracts a mutable slice of the entire vector.
Equivalent to
&mut s[..]
.
§
Examples
use
std::io::{
self
, Read};
let
mut
buffer =
vec!
[
0
;
3
];
io::repeat(
0b101
).read_exact(buffer.as_mut_slice()).unwrap();
1.37.0
·
Source
pub fn
as_ptr
(&self) ->
*const T
Returns a raw pointer to the vector’s buffer, or a dangling raw pointer
valid for zero sized reads if the vector didn’t allocate.
The caller must ensure that the vector outlives the pointer this
function returns, or else it will end up dangling.
Modifying the vector may cause its buffer to be reallocated,
which would also make any pointers to it invalid.
The caller must also ensure that the memory the pointer (non-transitively) points to
is never written to (except inside an
UnsafeCell
) using this pointer or any pointer
derived from it. If you need to mutate the contents of the slice, use
as_mut_ptr
.
This method guarantees that for the purpose of the aliasing model, this method
does not materialize a reference to the underlying slice, and thus the returned pointer
will remain valid when mixed with other calls to
as_ptr
,
as_mut_ptr
,
and
as_non_null
.
Note that calling other methods that materialize mutable references to the slice,
or mutable references to specific elements you are planning on accessing through this pointer,
as well as writing to those elements, may still invalidate this pointer.
See the second example below for how this guarantee can be used.
§
Examples
let
x =
vec!
[
1
,
2
,
4
];
let
x_ptr = x.as_ptr();
unsafe
{
for
i
in
0
..x.len() {
assert_eq!
(
*
x_ptr.add(i),
1
<< i);
    }
}
Due to the aliasing guarantee, the following code is legal:
unsafe
{
let
mut
v =
vec!
[
0
,
1
,
2
];
let
ptr1 = v.as_ptr();
let _
= ptr1.read();
let
ptr2 = v.as_mut_ptr().offset(
2
);
    ptr2.write(
2
);
// Notably, the write to `ptr2` did *not* invalidate `ptr1`
    // because it mutated a different element:
let _
= ptr1.read();
}
1.37.0
·
Source
pub fn
as_mut_ptr
(&mut self) ->
*mut T
Returns a raw mutable pointer to the vector’s buffer, or a dangling
raw pointer valid for zero sized reads if the vector didn’t allocate.
The caller must ensure that the vector outlives the pointer this
function returns, or else it will end up dangling.
Modifying the vector may cause its buffer to be reallocated,
which would also make any pointers to it invalid.
This method guarantees that for the purpose of the aliasing model, this method
does not materialize a reference to the underlying slice, and thus the returned pointer
will remain valid when mixed with other calls to
as_ptr
,
as_mut_ptr
,
and
as_non_null
.
Note that calling other methods that materialize references to the slice,
or references to specific elements you are planning on accessing through this pointer,
may still invalidate this pointer.
See the second example below for how this guarantee can be used.
§
Examples
// Allocate vector big enough for 4 elements.
let
size =
4
;
let
mut
x: Vec<i32> = Vec::with_capacity(size);
let
x_ptr = x.as_mut_ptr();
// Initialize elements via raw pointer writes, then set length.
unsafe
{
for
i
in
0
..size {
*
x_ptr.add(i) = i
as
i32;
    }
    x.set_len(size);
}
assert_eq!
(
&*
x,
&
[
0
,
1
,
2
,
3
]);
Due to the aliasing guarantee, the following code is legal:
unsafe
{
let
mut
v =
vec!
[
0
];
let
ptr1 = v.as_mut_ptr();
    ptr1.write(
1
);
let
ptr2 = v.as_mut_ptr();
    ptr2.write(
2
);
// Notably, the write to `ptr2` did *not* invalidate `ptr1`:
ptr1.write(
3
);
}
Source
pub fn
as_non_null
(&mut self) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
box_vec_non_null
#130364
)
Returns a
NonNull
pointer to the vector’s buffer, or a dangling
NonNull
pointer valid for zero sized reads if the vector didn’t allocate.
The caller must ensure that the vector outlives the pointer this
function returns, or else it will end up dangling.
Modifying the vector may cause its buffer to be reallocated,
which would also make any pointers to it invalid.
This method guarantees that for the purpose of the aliasing model, this method
does not materialize a reference to the underlying slice, and thus the returned pointer
will remain valid when mixed with other calls to
as_ptr
,
as_mut_ptr
,
and
as_non_null
.
Note that calling other methods that materialize references to the slice,
or references to specific elements you are planning on accessing through this pointer,
may still invalidate this pointer.
See the second example below for how this guarantee can be used.
§
Examples
#![feature(box_vec_non_null)]
// Allocate vector big enough for 4 elements.
let
size =
4
;
let
mut
x: Vec<i32> = Vec::with_capacity(size);
let
x_ptr = x.as_non_null();
// Initialize elements via raw pointer writes, then set length.
unsafe
{
for
i
in
0
..size {
        x_ptr.add(i).write(i
as
i32);
    }
    x.set_len(size);
}
assert_eq!
(
&*
x,
&
[
0
,
1
,
2
,
3
]);
Due to the aliasing guarantee, the following code is legal:
#![feature(box_vec_non_null)]
unsafe
{
let
mut
v =
vec!
[
0
];
let
ptr1 = v.as_non_null();
    ptr1.write(
1
);
let
ptr2 = v.as_non_null();
    ptr2.write(
2
);
// Notably, the write to `ptr2` did *not* invalidate `ptr1`:
ptr1.write(
3
);
}
Source
pub fn
allocator
(&self) ->
&A
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Returns a reference to the underlying allocator.
1.0.0
·
Source
pub unsafe fn
set_len
(&mut self, new_len:
usize
)
Forces the length of the vector to
new_len
.
This is a low-level operation that maintains none of the normal
invariants of the type. Normally changing the length of a vector
is done using one of the safe operations instead, such as
truncate
,
resize
,
extend
, or
clear
.
§
Safety
new_len
must be less than or equal to
capacity()
.
The elements at
old_len..new_len
must be initialized.
§
Examples
See
spare_capacity_mut()
for an example with safe
initialization of capacity elements and use of this method.
set_len()
can be useful for situations in which the vector
is serving as a buffer for other code, particularly over FFI:
pub fn
get_dictionary(
&
self
) ->
Option
<Vec<u8>> {
// Per the FFI method's docs, "32768 bytes is always enough".
let
mut
dict = Vec::with_capacity(
32_768
);
let
mut
dict_length =
0
;
// SAFETY: When `deflateGetDictionary` returns `Z_OK`, it holds that:
    // 1. `dict_length` elements were initialized.
    // 2. `dict_length` <= the capacity (32_768)
    // which makes `set_len` safe to call.
unsafe
{
// Make the FFI call...
let
r = deflateGetDictionary(
self
.strm, dict.as_mut_ptr(),
&mut
dict_length);
if
r == Z_OK {
// ...and update the length to what was initialized.
dict.set_len(dict_length);
Some
(dict)
        }
else
{
None
}
    }
}
While the following example is sound, there is a memory leak since
the inner vectors were not freed prior to the
set_len
call:
let
mut
vec =
vec!
[
vec!
[
1
,
0
,
0
],
vec!
[
0
,
1
,
0
],
vec!
[
0
,
0
,
1
]];
// SAFETY:
// 1. `old_len..0` is empty so no elements need to be initialized.
// 2. `0 <= capacity` always holds whatever `capacity` is.
unsafe
{
    vec.set_len(
0
);
}
Normally, here, one would use
clear
instead to correctly drop
the contents and thus not leak memory.
1.0.0
·
Source
pub fn
swap_remove
(&mut self, index:
usize
) -> T
Removes an element from the vector and returns it.
The removed element is replaced by the last element of the vector.
This does not preserve ordering of the remaining elements, but is
O
(1).
If you need to preserve the element order, use
remove
instead.
§
Panics
Panics if
index
is out of bounds.
§
Examples
let
mut
v =
vec!
[
"foo"
,
"bar"
,
"baz"
,
"qux"
];
assert_eq!
(v.swap_remove(
1
),
"bar"
);
assert_eq!
(v, [
"foo"
,
"qux"
,
"baz"
]);
assert_eq!
(v.swap_remove(
0
),
"foo"
);
assert_eq!
(v, [
"baz"
,
"qux"
]);
1.0.0
·
Source
pub fn
insert
(&mut self, index:
usize
, element: T)
Inserts an element at position
index
within the vector, shifting all
elements after it to the right.
§
Panics
Panics if
index > len
.
§
Examples
let
mut
vec =
vec!
[
'a'
,
'b'
,
'c'
];
vec.insert(
1
,
'd'
);
assert_eq!
(vec, [
'a'
,
'd'
,
'b'
,
'c'
]);
vec.insert(
4
,
'e'
);
assert_eq!
(vec, [
'a'
,
'd'
,
'b'
,
'c'
,
'e'
]);
§
Time complexity
Takes
O
(
Vec::len
) time. All items after the insertion index must be
shifted to the right. In the worst case, all elements are shifted when
the insertion index is 0.
1.0.0
·
Source
pub fn
remove
(&mut self, index:
usize
) -> T
Removes and returns the element at position
index
within the vector,
shifting all elements after it to the left.
Note: Because this shifts over the remaining elements, it has a
worst-case performance of
O
(
n
). If you don’t need the order of elements
to be preserved, use
swap_remove
instead. If you’d like to remove
elements from the beginning of the
Vec
, consider using
VecDeque::pop_front
instead.
§
Panics
Panics if
index
is out of bounds.
§
Examples
let
mut
v =
vec!
[
'a'
,
'b'
,
'c'
];
assert_eq!
(v.remove(
1
),
'b'
);
assert_eq!
(v, [
'a'
,
'c'
]);
1.0.0
·
Source
pub fn
retain
<F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) ->
bool
,
Retains only the elements specified by the predicate.
In other words, remove all elements
e
for which
f(&e)
returns
false
.
This method operates in place, visiting each element exactly once in the
original order, and preserves the order of the retained elements.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
,
4
];
vec.retain(|
&
x| x %
2
==
0
);
assert_eq!
(vec, [
2
,
4
]);
Because the elements are visited exactly once in the original order,
external state may be used to decide which elements to keep.
let
mut
vec =
vec!
[
1
,
2
,
3
,
4
,
5
];
let
keep = [
false
,
true
,
true
,
false
,
true
];
let
mut
iter = keep.iter();
vec.retain(|
_
|
*
iter.next().unwrap());
assert_eq!
(vec, [
2
,
3
,
5
]);
1.61.0
·
Source
pub fn
retain_mut
<F>(&mut self, f: F)
where
    F:
FnMut
(
&mut T
) ->
bool
,
Retains only the elements specified by the predicate, passing a mutable reference to it.
In other words, remove all elements
e
such that
f(&mut e)
returns
false
.
This method operates in place, visiting each element exactly once in the
original order, and preserves the order of the retained elements.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
,
4
];
vec.retain_mut(|x|
if
*
x <=
3
{
*
x +=
1
;
true
}
else
{
false
});
assert_eq!
(vec, [
2
,
3
,
4
]);
1.16.0
·
Source
pub fn
dedup_by_key
<F, K>(&mut self, key: F)
where
    F:
FnMut
(
&mut T
) -> K,
    K:
PartialEq
,
Removes all but the first of consecutive elements in the vector that resolve to the same
key.
If the vector is sorted, this removes all duplicates.
§
Examples
let
mut
vec =
vec!
[
10
,
20
,
21
,
30
,
20
];

vec.dedup_by_key(|i|
*
i /
10
);
assert_eq!
(vec, [
10
,
20
,
30
,
20
]);
1.16.0
·
Source
pub fn
dedup_by
<F>(&mut self, same_bucket: F)
where
    F:
FnMut
(
&mut T
,
&mut T
) ->
bool
,
Removes all but the first of consecutive elements in the vector satisfying a given equality
relation.
The
same_bucket
function is passed references to two elements from the vector and
must determine if the elements compare equal. The elements are passed in opposite order
from their order in the slice, so if
same_bucket(a, b)
returns
true
,
a
is removed.
If the vector is sorted, this removes all duplicates.
§
Examples
let
mut
vec =
vec!
[
"foo"
,
"bar"
,
"Bar"
,
"baz"
,
"bar"
];

vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
assert_eq!
(vec, [
"foo"
,
"bar"
,
"baz"
,
"bar"
]);
1.0.0
·
Source
pub fn
push
(&mut self, value: T)
Appends an element to the back of a collection.
§
Panics
Panics if the new capacity exceeds
isize::MAX
bytes
.
§
Examples
let
mut
vec =
vec!
[
1
,
2
];
vec.push(
3
);
assert_eq!
(vec, [
1
,
2
,
3
]);
§
Time complexity
Takes amortized
O
(1) time. If the vector’s length would exceed its
capacity after the push,
O
(
capacity
) time is taken to copy the
vector’s elements to a larger allocation. This expensive operation is
offset by the
capacity
O
(1) insertions it allows.
Source
pub fn
push_within_capacity
(&mut self, value: T) ->
Result
<
()
, T>
🔬
This is a nightly-only experimental API. (
vec_push_within_capacity
#100486
)
Appends an element if there is sufficient spare capacity, otherwise an error is returned
with the element.
Unlike
push
this method will not reallocate when there’s insufficient capacity.
The caller should use
reserve
or
try_reserve
to ensure that there is enough capacity.
§
Examples
A manual, panic-free alternative to
FromIterator
:
#![feature(vec_push_within_capacity)]
use
std::collections::TryReserveError;
fn
from_iter_fallible<T>(iter:
impl
Iterator<Item=T>) ->
Result
<Vec<T>, TryReserveError> {
let
mut
vec = Vec::new();
for
value
in
iter {
if let
Err
(value) = vec.push_within_capacity(value) {
            vec.try_reserve(
1
)
?
;
// this cannot fail, the previous line either returned or added at least 1 free slot
let _
= vec.push_within_capacity(value);
        }
    }
Ok
(vec)
}
assert_eq!
(from_iter_fallible(
0
..
100
),
Ok
(Vec::from_iter(
0
..
100
)));
§
Time complexity
Takes
O
(1) time.
1.0.0
·
Source
pub fn
pop
(&mut self) ->
Option
<T>
Removes the last element from a vector and returns it, or
None
if it
is empty.
If you’d like to pop the first element, consider using
VecDeque::pop_front
instead.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
];
assert_eq!
(vec.pop(),
Some
(
3
));
assert_eq!
(vec, [
1
,
2
]);
§
Time complexity
Takes
O
(1) time.
1.86.0
·
Source
pub fn
pop_if
(&mut self, predicate: impl
FnOnce
(
&mut T
) ->
bool
) ->
Option
<T>
Removes and returns the last element from a vector if the predicate
returns
true
, or
None
if the predicate returns false or the vector
is empty (the predicate will not be called in that case).
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
,
4
];
let
pred = |x:
&mut
i32|
*
x %
2
==
0
;
assert_eq!
(vec.pop_if(pred),
Some
(
4
));
assert_eq!
(vec, [
1
,
2
,
3
]);
assert_eq!
(vec.pop_if(pred),
None
);
1.4.0
·
Source
pub fn
append
(&mut self, other: &mut
Vec
<T, A>)
Moves all the elements of
other
into
self
, leaving
other
empty.
§
Panics
Panics if the new capacity exceeds
isize::MAX
bytes
.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
];
let
mut
vec2 =
vec!
[
4
,
5
,
6
];
vec.append(
&mut
vec2);
assert_eq!
(vec, [
1
,
2
,
3
,
4
,
5
,
6
]);
assert_eq!
(vec2, []);
1.6.0
·
Source
pub fn
drain
<R>(&mut self, range: R) ->
Drain
<'_, T, A>
ⓘ
where
    R:
RangeBounds
<
usize
>,
Removes the subslice indicated by the given range from the vector,
returning a double-ended iterator over the removed subslice.
If the iterator is dropped before being fully consumed,
it drops the remaining removed elements.
The returned iterator keeps a mutable borrow on the vector to optimize
its implementation.
§
Panics
Panics if the starting point is greater than the end point or if
the end point is greater than the length of the vector.
§
Leaking
If the returned iterator goes out of scope without being dropped (due to
mem::forget
, for example), the vector may have lost and leaked
elements arbitrarily, including elements outside the range.
§
Examples
let
mut
v =
vec!
[
1
,
2
,
3
];
let
u: Vec<
_
> = v.drain(
1
..).collect();
assert_eq!
(v,
&
[
1
]);
assert_eq!
(u,
&
[
2
,
3
]);
// A full range clears the vector, like `clear()` does
v.drain(..);
assert_eq!
(v,
&
[]);
1.0.0
·
Source
pub fn
clear
(&mut self)
Clears the vector, removing all values.
Note that this method has no effect on the allocated capacity
of the vector.
§
Examples
let
mut
v =
vec!
[
1
,
2
,
3
];

v.clear();
assert!
(v.is_empty());
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the vector, also referred to
as its ‘length’.
§
Examples
let
a =
vec!
[
1
,
2
,
3
];
assert_eq!
(a.len(),
3
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the vector contains no elements.
§
Examples
let
mut
v = Vec::new();
assert!
(v.is_empty());

v.push(
1
);
assert!
(!v.is_empty());
1.4.0
·
Source
pub fn
split_off
(&mut self, at:
usize
) ->
Vec
<T, A>
where
    A:
Clone
,
Splits the collection into two at the given index.
Returns a newly allocated vector containing the elements in the range
[at, len)
. After the call, the original vector will be left containing
the elements
[0, at)
with its previous capacity unchanged.
If you want to take ownership of the entire contents and capacity of
the vector, see
mem::take
or
mem::replace
.
If you don’t need the returned vector at all, see
Vec::truncate
.
If you want to take ownership of an arbitrary subslice, or you don’t
necessarily want to store the removed items in a vector, see
Vec::drain
.
§
Panics
Panics if
at > len
.
§
Examples
let
mut
vec =
vec!
[
'a'
,
'b'
,
'c'
];
let
vec2 = vec.split_off(
1
);
assert_eq!
(vec, [
'a'
]);
assert_eq!
(vec2, [
'b'
,
'c'
]);
1.33.0
·
Source
pub fn
resize_with
<F>(&mut self, new_len:
usize
, f: F)
where
    F:
FnMut
() -> T,
Resizes the
Vec
in-place so that
len
is equal to
new_len
.
If
new_len
is greater than
len
, the
Vec
is extended by the
difference, with each additional slot filled with the result of
calling the closure
f
. The return values from
f
will end up
in the
Vec
in the order they have been generated.
If
new_len
is less than
len
, the
Vec
is simply truncated.
This method uses a closure to create new values on every push. If
you’d rather
Clone
a given value, use
Vec::resize
. If you
want to use the
Default
trait to generate values, you can
pass
Default::default
as the second argument.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
3
];
vec.resize_with(
5
, Default::default);
assert_eq!
(vec, [
1
,
2
,
3
,
0
,
0
]);
let
mut
vec =
vec!
[];
let
mut
p =
1
;
vec.resize_with(
4
, || { p
*
=
2
; p });
assert_eq!
(vec, [
2
,
4
,
8
,
16
]);
1.60.0
·
Source
pub fn
spare_capacity_mut
(&mut self) -> &mut [
MaybeUninit
<T>]
Returns the remaining spare capacity of the vector as a slice of
MaybeUninit<T>
.
The returned slice can be used to fill the vector with data (e.g. by
reading from a file) before marking the data as initialized using the
set_len
method.
§
Examples
// Allocate vector big enough for 10 elements.
let
mut
v = Vec::with_capacity(
10
);
// Fill in the first 3 elements.
let
uninit = v.spare_capacity_mut();
uninit[
0
].write(
0
);
uninit[
1
].write(
1
);
uninit[
2
].write(
2
);
// Mark the first 3 elements of the vector as being initialized.
unsafe
{
    v.set_len(
3
);
}
assert_eq!
(
&
v,
&
[
0
,
1
,
2
]);
Source
pub fn
split_at_spare_mut
(&mut self) -> (&mut
[T]
, &mut [
MaybeUninit
<T>])
🔬
This is a nightly-only experimental API. (
vec_split_at_spare
#81944
)
Returns vector content as a slice of
T
, along with the remaining spare
capacity of the vector as a slice of
MaybeUninit<T>
.
The returned spare capacity slice can be used to fill the vector with data
(e.g. by reading from a file) before marking the data as initialized using
the
set_len
method.
Note that this is a low-level API, which should be used with care for
optimization purposes. If you need to append data to a
Vec
you can use
push
,
extend
,
extend_from_slice
,
extend_from_within
,
insert
,
append
,
resize
or
resize_with
, depending on your exact needs.
§
Examples
#![feature(vec_split_at_spare)]
let
mut
v =
vec!
[
1
,
1
,
2
];
// Reserve additional space big enough for 10 elements.
v.reserve(
10
);
let
(init, uninit) = v.split_at_spare_mut();
let
sum = init.iter().copied().sum::<u32>();
// Fill in the next 4 elements.
uninit[
0
].write(sum);
uninit[
1
].write(sum *
2
);
uninit[
2
].write(sum *
3
);
uninit[
3
].write(sum *
4
);
// Mark the 4 elements of the vector as being initialized.
unsafe
{
let
len = v.len();
    v.set_len(len +
4
);
}
assert_eq!
(
&
v,
&
[
1
,
1
,
2
,
4
,
8
,
12
,
16
]);
1.5.0
·
Source
pub fn
resize
(&mut self, new_len:
usize
, value: T)
Resizes the
Vec
in-place so that
len
is equal to
new_len
.
If
new_len
is greater than
len
, the
Vec
is extended by the
difference, with each additional slot filled with
value
.
If
new_len
is less than
len
, the
Vec
is simply truncated.
This method requires
T
to implement
Clone
,
in order to be able to clone the passed value.
If you need more flexibility (or want to rely on
Default
instead of
Clone
), use
Vec::resize_with
.
If you only need to resize to a smaller size, use
Vec::truncate
.
§
Examples
let
mut
vec =
vec!
[
"hello"
];
vec.resize(
3
,
"world"
);
assert_eq!
(vec, [
"hello"
,
"world"
,
"world"
]);
let
mut
vec =
vec!
[
'a'
,
'b'
,
'c'
,
'd'
];
vec.resize(
2
,
'_'
);
assert_eq!
(vec, [
'a'
,
'b'
]);
1.6.0
·
Source
pub fn
extend_from_slice
(&mut self, other: &
[T]
)
Clones and appends all elements in a slice to the
Vec
.
Iterates over the slice
other
, clones each element, and then appends
it to this
Vec
. The
other
slice is traversed in-order.
Note that this function is the same as
extend
,
except that it also works with slice elements that are Clone but not Copy.
If Rust gets specialization this function may be deprecated.
§
Examples
let
mut
vec =
vec!
[
1
];
vec.extend_from_slice(
&
[
2
,
3
,
4
]);
assert_eq!
(vec, [
1
,
2
,
3
,
4
]);
1.53.0
·
Source
pub fn
extend_from_within
<R>(&mut self, src: R)
where
    R:
RangeBounds
<
usize
>,
Given a range
src
, clones a slice of elements in that range and appends it to the end.
src
must be a range that can form a valid subslice of the
Vec
.
§
Panics
Panics if starting index is greater than the end index
or if the index is greater than the length of the vector.
§
Examples
let
mut
characters =
vec!
[
'a'
,
'b'
,
'c'
,
'd'
,
'e'
];
characters.extend_from_within(
2
..);
assert_eq!
(characters, [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'c'
,
'd'
,
'e'
]);
let
mut
numbers =
vec!
[
0
,
1
,
2
,
3
,
4
];
numbers.extend_from_within(..
2
);
assert_eq!
(numbers, [
0
,
1
,
2
,
3
,
4
,
0
,
1
]);
let
mut
strings =
vec!
[String::from(
"hello"
), String::from(
"world"
), String::from(
"!"
)];
strings.extend_from_within(
1
..=
2
);
assert_eq!
(strings, [
"hello"
,
"world"
,
"!"
,
"world"
,
"!"
]);
1.0.0
·
Source
pub fn
dedup
(&mut self)
Removes consecutive repeated elements in the vector according to the
PartialEq
trait implementation.
If the vector is sorted, this removes all duplicates.
§
Examples
let
mut
vec =
vec!
[
1
,
2
,
2
,
3
,
2
];

vec.dedup();
assert_eq!
(vec, [
1
,
2
,
3
,
2
]);
1.21.0
·
Source
pub fn
splice
<R, I>(
    &mut self,
    range: R,
    replace_with: I,
) ->
Splice
<'_, <I as
IntoIterator
>::
IntoIter
, A>
ⓘ
where
    R:
RangeBounds
<
usize
>,
    I:
IntoIterator
<Item = T>,
Creates a splicing iterator that replaces the specified range in the vector
with the given
replace_with
iterator and yields the removed items.
replace_with
does not need to be the same length as
range
.
range
is removed even if the
Splice
iterator is not consumed before it is dropped.
It is unspecified how many elements are removed from the vector
if the
Splice
value is leaked.
The input iterator
replace_with
is only consumed when the
Splice
value is dropped.
This is optimal if:
The tail (elements in the vector after
range
) is empty,
or
replace_with
yields fewer or equal elements than
range
’s length
or the lower bound of its
size_hint()
is exact.
Otherwise, a temporary vector is allocated and the tail is moved twice.
§
Panics
Panics if the starting point is greater than the end point or if
the end point is greater than the length of the vector.
§
Examples
let
mut
v =
vec!
[
1
,
2
,
3
,
4
];
let
new = [
7
,
8
,
9
];
let
u: Vec<
_
> = v.splice(
1
..
3
, new).collect();
assert_eq!
(v, [
1
,
7
,
8
,
9
,
4
]);
assert_eq!
(u, [
2
,
3
]);
Using
splice
to insert new items into a vector efficiently at a specific position
indicated by an empty range:
let
mut
v =
vec!
[
1
,
5
];
let
new = [
2
,
3
,
4
];
v.splice(
1
..
1
, new);
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
]);
1.87.0
·
Source
pub fn
extract_if
<F, R>(
    &mut self,
    range: R,
    filter: F,
) ->
ExtractIf
<'_, T, F, A>
ⓘ
where
    F:
FnMut
(
&mut T
) ->
bool
,
    R:
RangeBounds
<
usize
>,
Creates an iterator which uses a closure to determine if element in the range should be removed.
If the closure returns true, then the element is removed and yielded.
If the closure returns false, the element will remain in the vector and will not be yielded
by the iterator.
Only elements that fall in the provided range are considered for extraction, but any elements
after the range will still have to be moved if any element has been extracted.
If the returned
ExtractIf
is not exhausted, e.g. because it is dropped without iterating
or the iteration short-circuits, then the remaining elements will be retained.
Use
retain
with a negated predicate if you do not need the returned iterator.
Using this method is equivalent to the following code:
let
mut
i = range.start;
while
i < min(vec.len(), range.end) {
if
some_predicate(
&mut
vec[i]) {
let
val = vec.remove(i);
// your code here
}
else
{
        i +=
1
;
    }
}
But
extract_if
is easier to use.
extract_if
is also more efficient,
because it can backshift the elements of the array in bulk.
Note that
extract_if
also lets you mutate the elements passed to the filter closure,
regardless of whether you choose to keep or remove them.
§
Panics
If
range
is out of bounds.
§
Examples
Splitting an array into evens and odds, reusing the original allocation:
let
mut
numbers =
vec!
[
1
,
2
,
3
,
4
,
5
,
6
,
8
,
9
,
11
,
13
,
14
,
15
];
let
evens = numbers.extract_if(.., |x|
*
x %
2
==
0
).collect::<Vec<
_
>>();
let
odds = numbers;
assert_eq!
(evens,
vec!
[
2
,
4
,
6
,
8
,
14
]);
assert_eq!
(odds,
vec!
[
1
,
3
,
5
,
9
,
11
,
13
,
15
]);
Using the range argument to only process a part of the vector:
let
mut
items =
vec!
[
0
,
0
,
0
,
0
,
0
,
0
,
0
,
1
,
2
,
1
,
2
,
1
,
2
];
let
ones = items.extract_if(
7
.., |x|
*
x ==
1
).collect::<Vec<
_
>>();
assert_eq!
(items,
vec!
[
0
,
0
,
0
,
0
,
0
,
0
,
0
,
2
,
2
,
2
]);
assert_eq!
(ones.len(),
3
);
Methods from
Deref
<Target =
[T]
>
§
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the slice.
§
Examples
let
a = [
1
,
2
,
3
];
assert_eq!
(a.len(),
3
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the slice has a length of 0.
§
Examples
let
a = [
1
,
2
,
3
];
assert!
(!a.is_empty());
let
b:
&
[i32] =
&
[];
assert!
(b.is_empty());
1.0.0
·
Source
pub fn
first
(&self) ->
Option
<
&T
>
Returns the first element of the slice, or
None
if it is empty.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
10
), v.first());
let
w:
&
[i32] =
&
[];
assert_eq!
(
None
, w.first());
1.0.0
·
Source
pub fn
first_mut
(&mut self) ->
Option
<
&mut T
>
Returns a mutable reference to the first element of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(first) = x.first_mut() {
*
first =
5
;
}
assert_eq!
(x,
&
[
5
,
1
,
2
]);
let
y:
&mut
[i32] =
&mut
[];
assert_eq!
(
None
, y.first_mut());
1.5.0
·
Source
pub fn
split_first
(&self) ->
Option
<(
&T
, &
[T]
)>
Returns the first and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first() {
assert_eq!
(first,
&
0
);
assert_eq!
(elements,
&
[
1
,
2
]);
}
1.5.0
·
Source
pub fn
split_first_mut
(&mut self) ->
Option
<(
&mut T
, &mut
[T]
)>
Returns the first and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_mut() {
*
first =
3
;
    elements[
0
] =
4
;
    elements[
1
] =
5
;
}
assert_eq!
(x,
&
[
3
,
4
,
5
]);
1.5.0
·
Source
pub fn
split_last
(&self) ->
Option
<(
&T
, &
[T]
)>
Returns the last and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((last, elements)) = x.split_last() {
assert_eq!
(last,
&
2
);
assert_eq!
(elements,
&
[
0
,
1
]);
}
1.5.0
·
Source
pub fn
split_last_mut
(&mut self) ->
Option
<(
&mut T
, &mut
[T]
)>
Returns the last and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((last, elements)) = x.split_last_mut() {
*
last =
3
;
    elements[
0
] =
4
;
    elements[
1
] =
5
;
}
assert_eq!
(x,
&
[
4
,
5
,
3
]);
1.0.0
·
Source
pub fn
last
(&self) ->
Option
<
&T
>
Returns the last element of the slice, or
None
if it is empty.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
30
), v.last());
let
w:
&
[i32] =
&
[];
assert_eq!
(
None
, w.last());
1.0.0
·
Source
pub fn
last_mut
(&mut self) ->
Option
<
&mut T
>
Returns a mutable reference to the last item in the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(last) = x.last_mut() {
*
last =
10
;
}
assert_eq!
(x,
&
[
0
,
1
,
10
]);
let
y:
&mut
[i32] =
&mut
[];
assert_eq!
(
None
, y.last_mut());
1.77.0
·
Source
pub fn
first_chunk
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
Returns an array reference to the first
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
u = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
[
10
,
40
]), u.first_chunk::<
2
>());
let
v:
&
[i32] =
&
[
10
];
assert_eq!
(
None
, v.first_chunk::<
2
>());
let
w:
&
[i32] =
&
[];
assert_eq!
(
Some
(
&
[]), w.first_chunk::<
0
>());
1.77.0
·
Source
pub fn
first_chunk_mut
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
Returns a mutable array reference to the first
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(first) = x.first_chunk_mut::<
2
>() {
    first[
0
] =
5
;
    first[
1
] =
4
;
}
assert_eq!
(x,
&
[
5
,
4
,
2
]);
assert_eq!
(
None
, x.first_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
split_first_chunk
<const N:
usize
>(&self) ->
Option
<(&
[T; N]
, &
[T]
)>
Returns an array reference to the first
N
items in the slice and the remaining slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_chunk::<
2
>() {
assert_eq!
(first,
&
[
0
,
1
]);
assert_eq!
(elements,
&
[
2
]);
}
assert_eq!
(
None
, x.split_first_chunk::<
4
>());
1.77.0
·
Source
pub fn
split_first_chunk_mut
<const N:
usize
>(
    &mut self,
) ->
Option
<(&mut
[T; N]
, &mut
[T]
)>
Returns a mutable array reference to the first
N
items in the slice and the remaining
slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_chunk_mut::<
2
>() {
    first[
0
] =
3
;
    first[
1
] =
4
;
    elements[
0
] =
5
;
}
assert_eq!
(x,
&
[
3
,
4
,
5
]);
assert_eq!
(
None
, x.split_first_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
split_last_chunk
<const N:
usize
>(&self) ->
Option
<(&
[T]
, &
[T; N]
)>
Returns an array reference to the last
N
items in the slice and the remaining slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((elements, last)) = x.split_last_chunk::<
2
>() {
assert_eq!
(elements,
&
[
0
]);
assert_eq!
(last,
&
[
1
,
2
]);
}
assert_eq!
(
None
, x.split_last_chunk::<
4
>());
1.77.0
·
Source
pub fn
split_last_chunk_mut
<const N:
usize
>(
    &mut self,
) ->
Option
<(&mut
[T]
, &mut
[T; N]
)>
Returns a mutable array reference to the last
N
items in the slice and the remaining
slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((elements, last)) = x.split_last_chunk_mut::<
2
>() {
    last[
0
] =
3
;
    last[
1
] =
4
;
    elements[
0
] =
5
;
}
assert_eq!
(x,
&
[
5
,
3
,
4
]);
assert_eq!
(
None
, x.split_last_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
last_chunk
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
Returns an array reference to the last
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
u = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
[
40
,
30
]), u.last_chunk::<
2
>());
let
v:
&
[i32] =
&
[
10
];
assert_eq!
(
None
, v.last_chunk::<
2
>());
let
w:
&
[i32] =
&
[];
assert_eq!
(
Some
(
&
[]), w.last_chunk::<
0
>());
1.77.0
·
Source
pub fn
last_chunk_mut
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
Returns a mutable array reference to the last
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(last) = x.last_chunk_mut::<
2
>() {
    last[
0
] =
10
;
    last[
1
] =
20
;
}
assert_eq!
(x,
&
[
0
,
10
,
20
]);
assert_eq!
(
None
, x.last_chunk_mut::<
4
>());
1.0.0
·
Source
pub fn
get
<I>(&self, index: I) ->
Option
<&<I as
SliceIndex
<
[T]
>>::
Output
>
where
    I:
SliceIndex
<
[T]
>,
Returns a reference to an element or subslice depending on the type of
index.
If given a position, returns a reference to the element at that
position or
None
if out of bounds.
If given a range, returns the subslice corresponding to that range,
or
None
if out of bounds.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
40
), v.get(
1
));
assert_eq!
(
Some
(
&
[
10
,
40
][..]), v.get(
0
..
2
));
assert_eq!
(
None
, v.get(
3
));
assert_eq!
(
None
, v.get(
0
..
4
));
1.0.0
·
Source
pub fn
get_mut
<I>(
    &mut self,
    index: I,
) ->
Option
<&mut <I as
SliceIndex
<
[T]
>>::
Output
>
where
    I:
SliceIndex
<
[T]
>,
Returns a mutable reference to an element or subslice depending on the
type of index (see
get
) or
None
if the index is out of bounds.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(elem) = x.get_mut(
1
) {
*
elem =
42
;
}
assert_eq!
(x,
&
[
0
,
42
,
2
]);
1.0.0
·
Source
pub unsafe fn
get_unchecked
<I>(
    &self,
    index: I,
) -> &<I as
SliceIndex
<
[T]
>>::
Output
where
    I:
SliceIndex
<
[T]
>,
Returns a reference to an element or subslice, without doing bounds
checking.
For a safe alternative see
get
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used.
You can think of this like
.get(index).unwrap_unchecked()
.  It’s UB
to call
.get_unchecked(len)
, even if you immediately convert to a
pointer.  And it’s UB to call
.get_unchecked(..len + 1)
,
.get_unchecked(..=len)
, or similar.
§
Examples
let
x =
&
[
1
,
2
,
4
];
unsafe
{
assert_eq!
(x.get_unchecked(
1
),
&
2
);
}
1.0.0
·
Source
pub unsafe fn
get_unchecked_mut
<I>(
    &mut self,
    index: I,
) -> &mut <I as
SliceIndex
<
[T]
>>::
Output
where
    I:
SliceIndex
<
[T]
>,
Returns a mutable reference to an element or subslice, without doing
bounds checking.
For a safe alternative see
get_mut
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used.
You can think of this like
.get_mut(index).unwrap_unchecked()
.  It’s
UB to call
.get_unchecked_mut(len)
, even if you immediately convert
to a pointer.  And it’s UB to call
.get_unchecked_mut(..len + 1)
,
.get_unchecked_mut(..=len)
, or similar.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
unsafe
{
let
elem = x.get_unchecked_mut(
1
);
*
elem =
13
;
}
assert_eq!
(x,
&
[
1
,
13
,
4
]);
1.0.0
·
Source
pub fn
as_ptr
(&self) ->
*const T
Returns a raw pointer to the slice’s buffer.
The caller must ensure that the slice outlives the pointer this
function returns, or else it will end up dangling.
The caller must also ensure that the memory the pointer (non-transitively) points to
is never written to (except inside an
UnsafeCell
) using this pointer or any pointer
derived from it. If you need to mutate the contents of the slice, use
as_mut_ptr
.
Modifying the container referenced by this slice may cause its buffer
to be reallocated, which would also make any pointers to it invalid.
§
Examples
let
x =
&
[
1
,
2
,
4
];
let
x_ptr = x.as_ptr();
unsafe
{
for
i
in
0
..x.len() {
assert_eq!
(x.get_unchecked(i),
&*
x_ptr.add(i));
    }
}
1.0.0
·
Source
pub fn
as_mut_ptr
(&mut self) ->
*mut T
Returns an unsafe mutable pointer to the slice’s buffer.
The caller must ensure that the slice outlives the pointer this
function returns, or else it will end up dangling.
Modifying the container referenced by this slice may cause its buffer
to be reallocated, which would also make any pointers to it invalid.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
let
x_ptr = x.as_mut_ptr();
unsafe
{
for
i
in
0
..x.len() {
*
x_ptr.add(i) +=
2
;
    }
}
assert_eq!
(x,
&
[
3
,
4
,
6
]);
1.48.0
·
Source
pub fn
as_ptr_range
(&self) ->
Range
<
*const T
>
ⓘ
Returns the two raw pointers spanning the slice.
The returned range is half-open, which means that the end pointer
points
one past
the last element of the slice. This way, an empty
slice is represented by two equal pointers, and the difference between
the two pointers represents the size of the slice.
See
as_ptr
for warnings on using these pointers. The end pointer
requires extra caution, as it does not point to a valid element in the
slice.
This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.
It can also be useful to check if a pointer to an element refers to an
element of this slice:
let
a = [
1
,
2
,
3
];
let
x =
&
a[
1
]
as
*const
_
;
let
y =
&
5
as
*const
_
;
assert!
(a.as_ptr_range().contains(
&
x));
assert!
(!a.as_ptr_range().contains(
&
y));
1.48.0
·
Source
pub fn
as_mut_ptr_range
(&mut self) ->
Range
<
*mut T
>
ⓘ
Returns the two unsafe mutable pointers spanning the slice.
The returned range is half-open, which means that the end pointer
points
one past
the last element of the slice. This way, an empty
slice is represented by two equal pointers, and the difference between
the two pointers represents the size of the slice.
See
as_mut_ptr
for warnings on using these pointers. The end
pointer requires extra caution, as it does not point to a valid element
in the slice.
This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.
Source
pub fn
as_array
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a reference to the underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
Source
pub fn
as_mut_array
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a mutable reference to the slice’s underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
1.0.0
·
Source
pub fn
swap
(&mut self, a:
usize
, b:
usize
)
Swaps two elements in the slice.
If
a
equals to
b
, it’s guaranteed that elements won’t change value.
§
Arguments
a - The index of the first element
b - The index of the second element
§
Panics
Panics if
a
or
b
are out of bounds.
§
Examples
let
mut
v = [
"a"
,
"b"
,
"c"
,
"d"
,
"e"
];
v.swap(
2
,
4
);
assert!
(v == [
"a"
,
"b"
,
"e"
,
"d"
,
"c"
]);
Source
pub unsafe fn
swap_unchecked
(&mut self, a:
usize
, b:
usize
)
🔬
This is a nightly-only experimental API. (
slice_swap_unchecked
#88539
)
Swaps two elements in the slice, without doing bounds checking.
For a safe alternative see
swap
.
§
Arguments
a - The index of the first element
b - The index of the second element
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
.
The caller has to ensure that
a < self.len()
and
b < self.len()
.
§
Examples
#![feature(slice_swap_unchecked)]
let
mut
v = [
"a"
,
"b"
,
"c"
,
"d"
];
// SAFETY: we know that 1 and 3 are both indices of the slice
unsafe
{ v.swap_unchecked(
1
,
3
) };
assert!
(v == [
"a"
,
"d"
,
"c"
,
"b"
]);
1.0.0
·
Source
pub fn
reverse
(&mut self)
Reverses the order of elements in the slice, in place.
§
Examples
let
mut
v = [
1
,
2
,
3
];
v.reverse();
assert!
(v == [
3
,
2
,
1
]);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns an iterator over the slice.
The iterator yields all items from start to end.
§
Examples
let
x =
&
[
1
,
2
,
4
];
let
mut
iterator = x.iter();
assert_eq!
(iterator.next(),
Some
(
&
1
));
assert_eq!
(iterator.next(),
Some
(
&
2
));
assert_eq!
(iterator.next(),
Some
(
&
4
));
assert_eq!
(iterator.next(),
None
);
1.0.0
·
Source
pub fn
iter_mut
(&mut self) ->
IterMut
<'_, T>
ⓘ
Returns an iterator that allows modifying each value.
The iterator yields all items from start to end.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
for
elem
in
x.iter_mut() {
*
elem +=
2
;
}
assert_eq!
(x,
&
[
3
,
4
,
6
]);
1.0.0
·
Source
pub fn
windows
(&self, size:
usize
) ->
Windows
<'_, T>
ⓘ
Returns an iterator over all contiguous windows of length
size
. The windows overlap. If the slice is shorter than
size
, the iterator returns no values.
§
Panics
Panics if
size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.windows(
3
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
,
'r'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
,
'e'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
,
'm'
]);
assert!
(iter.next().is_none());
If the slice is shorter than
size
:
let
slice = [
'f'
,
'o'
,
'o'
];
let
mut
iter = slice.windows(
4
);
assert!
(iter.next().is_none());
Because the
Iterator
trait cannot represent the required lifetimes,
there is no
windows_mut
analog to
windows
;
[0,1,2].windows_mut(2).collect()
would violate
the rules of references
(though a
LendingIterator
analog is possible). You can sometimes use
Cell::as_slice_of_cells
in
conjunction with
windows
instead:
use
std::cell::Cell;
let
mut
array = [
'R'
,
'u'
,
's'
,
't'
,
' '
,
'2'
,
'0'
,
'1'
,
'5'
];
let
slice =
&mut
array[..];
let
slice_of_cells:
&
[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
for
w
in
slice_of_cells.windows(
3
) {
    Cell::swap(
&
w[
0
],
&
w[
2
]);
}
assert_eq!
(array, [
's'
,
't'
,
' '
,
'2'
,
'0'
,
'1'
,
'5'
,
'u'
,
'R'
]);
1.0.0
·
Source
pub fn
chunks
(&self, chunk_size:
usize
) ->
Chunks
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last chunk will not have length
chunk_size
.
See
chunks_exact
for a variant of this iterator that returns chunks of always exactly
chunk_size
elements, and
rchunks
for the same iterator but starting at the end of the
slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.chunks(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'm'
]);
assert!
(iter.next().is_none());
1.0.0
·
Source
pub fn
chunks_mut
(&mut self, chunk_size:
usize
) ->
ChunksMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last chunk will not have length
chunk_size
.
See
chunks_exact_mut
for a variant of this iterator that returns chunks of always
exactly
chunk_size
elements, and
rchunks_mut
for the same iterator but starting at
the end of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.chunks_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
3
]);
1.31.0
·
Source
pub fn
chunks_exact
(&self, chunk_size:
usize
) ->
ChunksExact
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last up to
chunk_size-1
elements will be omitted and can be retrieved
from the
remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks
.
See
chunks
for a variant of this iterator that also returns the remainder as a smaller
chunk, and
rchunks_exact
for the same iterator but starting at the end of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.chunks_exact(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'm'
]);
1.31.0
·
Source
pub fn
chunks_exact_mut
(&mut self, chunk_size:
usize
) ->
ChunksExactMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last up to
chunk_size-1
elements will be omitted and can be
retrieved from the
into_remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks_mut
.
See
chunks_mut
for a variant of this iterator that also returns the remainder as a
smaller chunk, and
rchunks_exact_mut
for the same iterator but starting at the end of
the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.chunks_exact_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
0
]);
Source
pub unsafe fn
as_chunks_unchecked
<const N:
usize
>(&self) -> &[
[T; N]
]
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
assuming that there’s no remainder.
§
Safety
This may only be called when
The slice splits exactly into
N
-element chunks (aka
self.len() % N == 0
).
N != 0
.
§
Examples
#![feature(slice_as_chunks)]
let
slice:
&
[char] =
&
[
'l'
,
'o'
,
'r'
,
'e'
,
'm'
,
'!'
];
let
chunks:
&
[[char;
1
]] =
// SAFETY: 1-element chunks never have remainder
unsafe
{ slice.as_chunks_unchecked() };
assert_eq!
(chunks,
&
[[
'l'
], [
'o'
], [
'r'
], [
'e'
], [
'm'
], [
'!'
]]);
let
chunks:
&
[[char;
3
]] =
// SAFETY: The slice length (6) is a multiple of 3
unsafe
{ slice.as_chunks_unchecked() };
assert_eq!
(chunks,
&
[[
'l'
,
'o'
,
'r'
], [
'e'
,
'm'
,
'!'
]]);
// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed
Source
pub fn
as_chunks
<const N:
usize
>(&self) -> (&[
[T; N]
], &
[T]
)
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the beginning of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
(chunks, remainder) = slice.as_chunks();
assert_eq!
(chunks,
&
[[
'l'
,
'o'
], [
'r'
,
'e'
]]);
assert_eq!
(remainder,
&
[
'm'
]);
If you expect the slice to be an exact multiple, you can combine
let
-
else
with an empty slice pattern:
#![feature(slice_as_chunks)]
let
slice = [
'R'
,
'u'
,
's'
,
't'
];
let
(chunks, []) = slice.as_chunks::<
2
>()
else
{
panic!
(
"slice didn't have even length"
)
};
assert_eq!
(chunks,
&
[[
'R'
,
'u'
], [
's'
,
't'
]]);
Source
pub fn
as_rchunks
<const N:
usize
>(&self) -> (&
[T]
, &[
[T; N]
])
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the end of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
(remainder, chunks) = slice.as_rchunks();
assert_eq!
(remainder,
&
[
'l'
]);
assert_eq!
(chunks,
&
[[
'o'
,
'r'
], [
'e'
,
'm'
]]);
Source
pub fn
array_chunks
<const N:
usize
>(&self) ->
ArrayChunks
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_chunks
#74985
)
Returns an iterator over
N
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are array references and do not overlap. If
N
does not divide the
length of the slice, then the last up to
N-1
elements will be omitted and can be
retrieved from the
remainder
function of the iterator.
This method is the const generic equivalent of
chunks_exact
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.array_chunks();
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'm'
]);
Source
pub unsafe fn
as_chunks_unchecked_mut
<const N:
usize
>(
    &mut self,
) -> &mut [
[T; N]
]
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
assuming that there’s no remainder.
§
Safety
This may only be called when
The slice splits exactly into
N
-element chunks (aka
self.len() % N == 0
).
N != 0
.
§
Examples
#![feature(slice_as_chunks)]
let
slice:
&mut
[char] =
&mut
[
'l'
,
'o'
,
'r'
,
'e'
,
'm'
,
'!'
];
let
chunks:
&mut
[[char;
1
]] =
// SAFETY: 1-element chunks never have remainder
unsafe
{ slice.as_chunks_unchecked_mut() };
chunks[
0
] = [
'L'
];
assert_eq!
(chunks,
&
[[
'L'
], [
'o'
], [
'r'
], [
'e'
], [
'm'
], [
'!'
]]);
let
chunks:
&mut
[[char;
3
]] =
// SAFETY: The slice length (6) is a multiple of 3
unsafe
{ slice.as_chunks_unchecked_mut() };
chunks[
1
] = [
'a'
,
'x'
,
'?'
];
assert_eq!
(slice,
&
[
'L'
,
'o'
,
'r'
,
'a'
,
'x'
,
'?'
]);
// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked_mut() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked_mut() // Zero-length chunks are never allowed
Source
pub fn
as_chunks_mut
<const N:
usize
>(&mut self) -> (&mut [
[T; N]
], &mut
[T]
)
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the beginning of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
let
(chunks, remainder) = v.as_chunks_mut();
remainder[
0
] =
9
;
for
chunk
in
chunks {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
9
]);
Source
pub fn
as_rchunks_mut
<const N:
usize
>(&mut self) -> (&mut
[T]
, &mut [
[T; N]
])
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the end of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
let
(remainder, chunks) = v.as_rchunks_mut();
remainder[
0
] =
9
;
for
chunk
in
chunks {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
9
,
1
,
1
,
2
,
2
]);
Source
pub fn
array_chunks_mut
<const N:
usize
>(&mut self) ->
ArrayChunksMut
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_chunks
#74985
)
Returns an iterator over
N
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable array references and do not overlap. If
N
does not divide
the length of the slice, then the last up to
N-1
elements will be omitted and
can be retrieved from the
into_remainder
function of the iterator.
This method is the const generic equivalent of
chunks_exact_mut
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.array_chunks_mut() {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
0
]);
Source
pub fn
array_windows
<const N:
usize
>(&self) ->
ArrayWindows
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_windows
#75027
)
Returns an iterator over overlapping windows of
N
elements of a slice,
starting at the beginning of the slice.
This is the const generic equivalent of
windows
.
If
N
is greater than the size of the slice, it will return no windows.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_windows)]
let
slice = [
0
,
1
,
2
,
3
];
let
mut
iter = slice.array_windows();
assert_eq!
(iter.next().unwrap(),
&
[
0
,
1
]);
assert_eq!
(iter.next().unwrap(),
&
[
1
,
2
]);
assert_eq!
(iter.next().unwrap(),
&
[
2
,
3
]);
assert!
(iter.next().is_none());
1.31.0
·
Source
pub fn
rchunks
(&self, chunk_size:
usize
) ->
RChunks
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last chunk will not have length
chunk_size
.
See
rchunks_exact
for a variant of this iterator that returns chunks of always exactly
chunk_size
elements, and
chunks
for the same iterator but starting at the beginning
of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.rchunks(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'e'
,
'm'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
]);
assert!
(iter.next().is_none());
1.31.0
·
Source
pub fn
rchunks_mut
(&mut self, chunk_size:
usize
) ->
RChunksMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last chunk will not have length
chunk_size
.
See
rchunks_exact_mut
for a variant of this iterator that returns chunks of always
exactly
chunk_size
elements, and
chunks_mut
for the same iterator but starting at the
beginning of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.rchunks_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
3
,
2
,
2
,
1
,
1
]);
1.31.0
·
Source
pub fn
rchunks_exact
(&self, chunk_size:
usize
) ->
RChunksExact
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
end of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last up to
chunk_size-1
elements will be omitted and can be retrieved
from the
remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
rchunks
.
See
rchunks
for a variant of this iterator that also returns the remainder as a smaller
chunk, and
chunks_exact
for the same iterator but starting at the beginning of the
slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.rchunks_exact(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'e'
,
'm'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'l'
]);
1.31.0
·
Source
pub fn
rchunks_exact_mut
(&mut self, chunk_size:
usize
) ->
RChunksExactMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last up to
chunk_size-1
elements will be omitted and can be
retrieved from the
into_remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks_mut
.
See
rchunks_mut
for a variant of this iterator that also returns the remainder as a
smaller chunk, and
chunks_exact_mut
for the same iterator but starting at the beginning
of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.rchunks_exact_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
0
,
2
,
2
,
1
,
1
]);
1.77.0
·
Source
pub fn
chunk_by
<F>(&self, pred: F) ->
ChunkBy
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
,
&T
) ->
bool
,
Returns an iterator over the slice producing non-overlapping runs
of elements using the predicate to separate them.
The predicate is called for every pair of consecutive elements,
meaning that it is called on
slice[0]
and
slice[1]
,
followed by
slice[1]
and
slice[2]
, and so on.
§
Examples
let
slice =
&
[
1
,
1
,
1
,
3
,
3
,
2
,
2
,
2
];
let
mut
iter = slice.chunk_by(|a, b| a == b);
assert_eq!
(iter.next(),
Some
(
&
[
1
,
1
,
1
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
3
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
2
,
2
][..]));
assert_eq!
(iter.next(),
None
);
This method can be used to extract the sorted subslices:
let
slice =
&
[
1
,
1
,
2
,
3
,
2
,
3
,
2
,
3
,
4
];
let
mut
iter = slice.chunk_by(|a, b| a <= b);
assert_eq!
(iter.next(),
Some
(
&
[
1
,
1
,
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
3
,
4
][..]));
assert_eq!
(iter.next(),
None
);
1.77.0
·
Source
pub fn
chunk_by_mut
<F>(&mut self, pred: F) ->
ChunkByMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
,
&T
) ->
bool
,
Returns an iterator over the slice producing non-overlapping mutable
runs of elements using the predicate to separate them.
The predicate is called for every pair of consecutive elements,
meaning that it is called on
slice[0]
and
slice[1]
,
followed by
slice[1]
and
slice[2]
, and so on.
§
Examples
let
slice =
&mut
[
1
,
1
,
1
,
3
,
3
,
2
,
2
,
2
];
let
mut
iter = slice.chunk_by_mut(|a, b| a == b);
assert_eq!
(iter.next(),
Some
(
&mut
[
1
,
1
,
1
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
3
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
2
,
2
][..]));
assert_eq!
(iter.next(),
None
);
This method can be used to extract the sorted subslices:
let
slice =
&mut
[
1
,
1
,
2
,
3
,
2
,
3
,
2
,
3
,
4
];
let
mut
iter = slice.chunk_by_mut(|a, b| a <= b);
assert_eq!
(iter.next(),
Some
(
&mut
[
1
,
1
,
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
3
,
4
][..]));
assert_eq!
(iter.next(),
None
);
1.0.0
·
Source
pub fn
split_at
(&self, mid:
usize
) -> (&
[T]
, &
[T]
)
Divides one slice into two at an index.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
§
Panics
Panics if
mid > len
.  For a non-panicking alternative see
split_at_checked
.
§
Examples
let
v = [
'a'
,
'b'
,
'c'
];

{
let
(left, right) = v.split_at(
0
);
assert_eq!
(left, []);
assert_eq!
(right, [
'a'
,
'b'
,
'c'
]);
}

{
let
(left, right) = v.split_at(
2
);
assert_eq!
(left, [
'a'
,
'b'
]);
assert_eq!
(right, [
'c'
]);
}

{
let
(left, right) = v.split_at(
3
);
assert_eq!
(left, [
'a'
,
'b'
,
'c'
]);
assert_eq!
(right, []);
}
1.0.0
·
Source
pub fn
split_at_mut
(&mut self, mid:
usize
) -> (&mut
[T]
, &mut
[T]
)
Divides one mutable slice into two at an index.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
§
Panics
Panics if
mid > len
.  For a non-panicking alternative see
split_at_mut_checked
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
let
(left, right) = v.split_at_mut(
2
);
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
left[
1
] =
2
;
right[
1
] =
4
;
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
1.79.0
·
Source
pub unsafe fn
split_at_unchecked
(&self, mid:
usize
) -> (&
[T]
, &
[T]
)
Divides one slice into two at an index, without doing bounds checking.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
For a safe alternative see
split_at
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used. The caller has to ensure that
0 <= mid <= self.len()
.
§
Examples
let
v = [
'a'
,
'b'
,
'c'
];
unsafe
{
let
(left, right) = v.split_at_unchecked(
0
);
assert_eq!
(left, []);
assert_eq!
(right, [
'a'
,
'b'
,
'c'
]);
}
unsafe
{
let
(left, right) = v.split_at_unchecked(
2
);
assert_eq!
(left, [
'a'
,
'b'
]);
assert_eq!
(right, [
'c'
]);
}
unsafe
{
let
(left, right) = v.split_at_unchecked(
3
);
assert_eq!
(left, [
'a'
,
'b'
,
'c'
]);
assert_eq!
(right, []);
}
1.79.0
·
Source
pub unsafe fn
split_at_mut_unchecked
(
    &mut self,
    mid:
usize
,
) -> (&mut
[T]
, &mut
[T]
)
Divides one mutable slice into two at an index, without doing bounds checking.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
For a safe alternative see
split_at_mut
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used. The caller has to ensure that
0 <= mid <= self.len()
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
// scoped to restrict the lifetime of the borrows
unsafe
{
let
(left, right) = v.split_at_mut_unchecked(
2
);
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
    left[
1
] =
2
;
    right[
1
] =
4
;
}
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
1.80.0
·
Source
pub fn
split_at_checked
(&self, mid:
usize
) ->
Option
<(&
[T]
, &
[T]
)>
Divides one slice into two at an index, returning
None
if the slice is
too short.
If
mid ≤ len
returns a pair of slices where the first will contain all
indices from
[0, mid)
(excluding the index
mid
itself) and the
second will contain all indices from
[mid, len)
(excluding the index
len
itself).
Otherwise, if
mid > len
, returns
None
.
§
Examples
let
v = [
1
, -
2
,
3
, -
4
,
5
, -
6
];

{
let
(left, right) = v.split_at_checked(
0
).unwrap();
assert_eq!
(left, []);
assert_eq!
(right, [
1
, -
2
,
3
, -
4
,
5
, -
6
]);
}

{
let
(left, right) = v.split_at_checked(
2
).unwrap();
assert_eq!
(left, [
1
, -
2
]);
assert_eq!
(right, [
3
, -
4
,
5
, -
6
]);
}

{
let
(left, right) = v.split_at_checked(
6
).unwrap();
assert_eq!
(left, [
1
, -
2
,
3
, -
4
,
5
, -
6
]);
assert_eq!
(right, []);
}
assert_eq!
(
None
, v.split_at_checked(
7
));
1.80.0
·
Source
pub fn
split_at_mut_checked
(
    &mut self,
    mid:
usize
,
) ->
Option
<(&mut
[T]
, &mut
[T]
)>
Divides one mutable slice into two at an index, returning
None
if the
slice is too short.
If
mid ≤ len
returns a pair of slices where the first will contain all
indices from
[0, mid)
(excluding the index
mid
itself) and the
second will contain all indices from
[mid, len)
(excluding the index
len
itself).
Otherwise, if
mid > len
, returns
None
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
if let
Some
((left, right)) = v.split_at_mut_checked(
2
) {
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
    left[
1
] =
2
;
    right[
1
] =
4
;
}
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
assert_eq!
(
None
, v.split_at_mut_checked(
7
));
1.0.0
·
Source
pub fn
split
<F>(&self, pred: F) ->
Split
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
. The matched element is not contained in the subslices.
§
Examples
let
slice = [
10
,
40
,
33
,
20
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
If the first element is matched, an empty slice will be the first item
returned by the iterator. Similarly, if the last element in the slice
is matched, an empty slice will be the last item returned by the
iterator:
let
slice = [
10
,
40
,
33
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
]);
assert_eq!
(iter.next().unwrap(),
&
[]);
assert!
(iter.next().is_none());
If two matched elements are directly adjacent, an empty slice will be
present between them:
let
slice = [
10
,
6
,
33
,
20
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
]);
assert_eq!
(iter.next().unwrap(),
&
[]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
1.0.0
·
Source
pub fn
split_mut
<F>(&mut self, pred: F) ->
SplitMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
. The matched element is not contained in the subslices.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.split_mut(|num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(v, [
1
,
40
,
30
,
1
,
60
,
1
]);
1.51.0
·
Source
pub fn
split_inclusive
<F>(&self, pred: F) ->
SplitInclusive
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
. The matched element is contained in the end of the previous
subslice as a terminator.
§
Examples
let
slice = [
10
,
40
,
33
,
20
];
let
mut
iter = slice.split_inclusive(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
,
33
]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
If the last element of the slice is matched,
that element will be considered the terminator of the preceding slice.
That slice will be the last item returned by the iterator.
let
slice = [
3
,
10
,
40
,
33
];
let
mut
iter = slice.split_inclusive(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
3
]);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
,
33
]);
assert!
(iter.next().is_none());
1.51.0
·
Source
pub fn
split_inclusive_mut
<F>(&mut self, pred: F) ->
SplitInclusiveMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
. The matched element is contained in the previous
subslice as a terminator.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.split_inclusive_mut(|num|
*
num %
3
==
0
) {
let
terminator_idx = group.len()-
1
;
    group[terminator_idx] =
1
;
}
assert_eq!
(v, [
10
,
40
,
1
,
20
,
1
,
1
]);
1.27.0
·
Source
pub fn
rsplit
<F>(&self, pred: F) ->
RSplit
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
, starting at the end of the slice and working backwards.
The matched element is not contained in the subslices.
§
Examples
let
slice = [
11
,
22
,
33
,
0
,
44
,
55
];
let
mut
iter = slice.rsplit(|num|
*
num ==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
44
,
55
]);
assert_eq!
(iter.next().unwrap(),
&
[
11
,
22
,
33
]);
assert_eq!
(iter.next(),
None
);
As with
split()
, if the first or last element is matched, an empty
slice will be the first (or last) item returned by the iterator.
let
v =
&
[
0
,
1
,
1
,
2
,
3
,
5
,
8
];
let
mut
it = v.rsplit(|n|
*
n %
2
==
0
);
assert_eq!
(it.next().unwrap(),
&
[]);
assert_eq!
(it.next().unwrap(),
&
[
3
,
5
]);
assert_eq!
(it.next().unwrap(),
&
[
1
,
1
]);
assert_eq!
(it.next().unwrap(),
&
[]);
assert_eq!
(it.next(),
None
);
1.27.0
·
Source
pub fn
rsplit_mut
<F>(&mut self, pred: F) ->
RSplitMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
, starting at the end of the slice and working
backwards. The matched element is not contained in the subslices.
§
Examples
let
mut
v = [
100
,
400
,
300
,
200
,
600
,
500
];
let
mut
count =
0
;
for
group
in
v.rsplit_mut(|num|
*
num %
3
==
0
) {
    count +=
1
;
    group[
0
] = count;
}
assert_eq!
(v, [
3
,
400
,
300
,
2
,
600
,
1
]);
1.0.0
·
Source
pub fn
splitn
<F>(&self, n:
usize
, pred: F) ->
SplitN
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
, limited to returning at most
n
items. The matched element is
not contained in the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
Print the slice split once by numbers divisible by 3 (i.e.,
[10, 40]
,
[20, 60, 50]
):
let
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.splitn(
2
, |num|
*
num %
3
==
0
) {
println!
(
"{group:?}"
);
}
1.0.0
·
Source
pub fn
splitn_mut
<F>(&mut self, n:
usize
, pred: F) ->
SplitNMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that match
pred
, limited to returning at most
n
items. The matched element is
not contained in the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.splitn_mut(
2
, |num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(v, [
1
,
40
,
30
,
1
,
60
,
50
]);
1.0.0
·
Source
pub fn
rsplitn
<F>(&self, n:
usize
, pred: F) ->
RSplitN
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
limited to returning at most
n
items. This starts at the end of
the slice and works backwards. The matched element is not contained in
the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
Print the slice split once, starting from the end, by numbers divisible
by 3 (i.e.,
[50]
,
[10, 40, 30, 20]
):
let
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.rsplitn(
2
, |num|
*
num %
3
==
0
) {
println!
(
"{group:?}"
);
}
1.0.0
·
Source
pub fn
rsplitn_mut
<F>(&mut self, n:
usize
, pred: F) ->
RSplitNMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
limited to returning at most
n
items. This starts at the end of
the slice and works backwards. The matched element is not contained in
the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
let
mut
s = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
s.rsplitn_mut(
2
, |num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(s, [
1
,
40
,
30
,
20
,
60
,
1
]);
Source
pub fn
split_once
<F>(&self, pred: F) ->
Option
<(&
[T]
, &
[T]
)>
where
    F:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_split_once
#112811
)
Splits the slice on the first element that matches the specified
predicate.
If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returns
None
.
§
Examples
#![feature(slice_split_once)]
let
s = [
1
,
2
,
3
,
2
,
4
];
assert_eq!
(s.split_once(|
&
x| x ==
2
),
Some
((
&
[
1
][..],
&
[
3
,
2
,
4
][..]
)));
assert_eq!
(s.split_once(|
&
x| x ==
0
),
None
);
Source
pub fn
rsplit_once
<F>(&self, pred: F) ->
Option
<(&
[T]
, &
[T]
)>
where
    F:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_split_once
#112811
)
Splits the slice on the last element that matches the specified
predicate.
If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returns
None
.
§
Examples
#![feature(slice_split_once)]
let
s = [
1
,
2
,
3
,
2
,
4
];
assert_eq!
(s.rsplit_once(|
&
x| x ==
2
),
Some
((
&
[
1
,
2
,
3
][..],
&
[
4
][..]
)));
assert_eq!
(s.rsplit_once(|
&
x| x ==
0
),
None
);
1.0.0
·
Source
pub fn
contains
(&self, x:
&T
) ->
bool
where
    T:
PartialEq
,
Returns
true
if the slice contains an element with the given value.
This operation is
O
(
n
).
Note that if you have a sorted slice,
binary_search
may be faster.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.contains(
&
30
));
assert!
(!v.contains(
&
50
));
If you do not have a
&T
, but some other value that you can compare
with one (for example,
String
implements
PartialEq<str>
), you can
use
iter().any
:
let
v = [String::from(
"hello"
), String::from(
"world"
)];
// slice of `String`
assert!
(v.iter().any(|e| e ==
"hello"
));
// search with `&str`
assert!
(!v.iter().any(|e| e ==
"hi"
));
1.0.0
·
Source
pub fn
starts_with
(&self, needle: &
[T]
) ->
bool
where
    T:
PartialEq
,
Returns
true
if
needle
is a prefix of the slice or equal to the slice.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.starts_with(
&
[
10
]));
assert!
(v.starts_with(
&
[
10
,
40
]));
assert!
(v.starts_with(
&
v));
assert!
(!v.starts_with(
&
[
50
]));
assert!
(!v.starts_with(
&
[
10
,
50
]));
Always returns
true
if
needle
is an empty slice:
let
v =
&
[
10
,
40
,
30
];
assert!
(v.starts_with(
&
[]));
let
v:
&
[u8] =
&
[];
assert!
(v.starts_with(
&
[]));
1.0.0
·
Source
pub fn
ends_with
(&self, needle: &
[T]
) ->
bool
where
    T:
PartialEq
,
Returns
true
if
needle
is a suffix of the slice or equal to the slice.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.ends_with(
&
[
30
]));
assert!
(v.ends_with(
&
[
40
,
30
]));
assert!
(v.ends_with(
&
v));
assert!
(!v.ends_with(
&
[
50
]));
assert!
(!v.ends_with(
&
[
50
,
30
]));
Always returns
true
if
needle
is an empty slice:
let
v =
&
[
10
,
40
,
30
];
assert!
(v.ends_with(
&
[]));
let
v:
&
[u8] =
&
[];
assert!
(v.ends_with(
&
[]));
1.51.0
·
Source
pub fn
strip_prefix
<P>(&self, prefix:
&P
) ->
Option
<&
[T]
>
where
    P:
SlicePattern
<Item = T> + ?
Sized
,
    T:
PartialEq
,
Returns a subslice with the prefix removed.
If the slice starts with
prefix
, returns the subslice after the prefix, wrapped in
Some
.
If
prefix
is empty, simply returns the original slice. If
prefix
is equal to the
original slice, returns an empty slice.
If the slice does not start with
prefix
, returns
None
.
§
Examples
let
v =
&
[
10
,
40
,
30
];
assert_eq!
(v.strip_prefix(
&
[
10
]),
Some
(
&
[
40
,
30
][..]));
assert_eq!
(v.strip_prefix(
&
[
10
,
40
]),
Some
(
&
[
30
][..]));
assert_eq!
(v.strip_prefix(
&
[
10
,
40
,
30
]),
Some
(
&
[][..]));
assert_eq!
(v.strip_prefix(
&
[
50
]),
None
);
assert_eq!
(v.strip_prefix(
&
[
10
,
50
]),
None
);
let
prefix :
&
str =
"he"
;
assert_eq!
(
b"hello"
.strip_prefix(prefix.as_bytes()),
Some
(
b"llo"
.as_ref()));
1.51.0
·
Source
pub fn
strip_suffix
<P>(&self, suffix:
&P
) ->
Option
<&
[T]
>
where
    P:
SlicePattern
<Item = T> + ?
Sized
,
    T:
PartialEq
,
Returns a subslice with the suffix removed.
If the slice ends with
suffix
, returns the subslice before the suffix, wrapped in
Some
.
If
suffix
is empty, simply returns the original slice. If
suffix
is equal to the
original slice, returns an empty slice.
If the slice does not end with
suffix
, returns
None
.
§
Examples
let
v =
&
[
10
,
40
,
30
];
assert_eq!
(v.strip_suffix(
&
[
30
]),
Some
(
&
[
10
,
40
][..]));
assert_eq!
(v.strip_suffix(
&
[
40
,
30
]),
Some
(
&
[
10
][..]));
assert_eq!
(v.strip_suffix(
&
[
10
,
40
,
30
]),
Some
(
&
[][..]));
assert_eq!
(v.strip_suffix(
&
[
50
]),
None
);
assert_eq!
(v.strip_suffix(
&
[
50
,
30
]),
None
);
1.0.0
·
Source
pub fn
binary_search
(&self, x:
&T
) ->
Result
<
usize
,
usize
>
where
    T:
Ord
,
Binary searches this slice for a given element.
If the slice is not sorted, the returned result is unspecified and
meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search_by
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
assert_eq!
(s.binary_search(
&
13
),
Ok
(
9
));
assert_eq!
(s.binary_search(
&
4
),
Err
(
7
));
assert_eq!
(s.binary_search(
&
100
),
Err
(
13
));
let
r = s.binary_search(
&
1
);
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
If you want to find that whole
range
of matching items, rather than
an arbitrary matching one, that can be done using
partition_point
:
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
low = s.partition_point(|x| x <
&
1
);
assert_eq!
(low,
1
);
let
high = s.partition_point(|x| x <=
&
1
);
assert_eq!
(high,
5
);
let
r = s.binary_search(
&
1
);
assert!
((low..high).contains(
&
r.unwrap()));
assert!
(s[..low].iter().all(|
&
x| x <
1
));
assert!
(s[low..high].iter().all(|
&
x| x ==
1
));
assert!
(s[high..].iter().all(|
&
x| x >
1
));
// For something not found, the "range" of equal items is empty
assert_eq!
(s.partition_point(|x| x <
&
11
),
9
);
assert_eq!
(s.partition_point(|x| x <=
&
11
),
9
);
assert_eq!
(s.binary_search(
&
11
),
Err
(
9
));
If you want to insert an item to a sorted vector, while maintaining
sort order, consider using
partition_point
:
let
mut
s =
vec!
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
num =
42
;
let
idx = s.partition_point(|
&
x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` will allow `insert`
// to shift less elements.
s.insert(idx, num);
assert_eq!
(s, [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
1.0.0
·
Source
pub fn
binary_search_by
<'a, F>(&'a self, f: F) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) ->
Ordering
,
Binary searches this slice with a comparator function.
The comparator function should return an order code that indicates
whether its argument is
Less
,
Equal
or
Greater
the desired
target.
If the slice is not sorted or if the comparator function does not
implement an order consistent with the sort order of the underlying
slice, the returned result is unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
seek =
13
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Ok
(
9
));
let
seek =
4
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Err
(
7
));
let
seek =
100
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Err
(
13
));
let
seek =
1
;
let
r = s.binary_search_by(|probe| probe.cmp(
&
seek));
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
1.10.0
·
Source
pub fn
binary_search_by_key
<'a, B, F>(
    &'a self,
    b:
&B
,
    f: F,
) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) -> B,
    B:
Ord
,
Binary searches this slice with a key extraction function.
Assumes that the slice is sorted by the key, for instance with
sort_by_key
using the same key extraction function.
If the slice is not sorted by the key, the returned result is
unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search
,
binary_search_by
, and
partition_point
.
§
Examples
Looks up a series of four elements in a slice of pairs sorted by
their second elements. The first is found, with a uniquely
determined position; the second and third are not found; the
fourth could match any position in
[1, 4]
.
let
s = [(
0
,
0
), (
2
,
1
), (
4
,
1
), (
5
,
1
), (
3
,
1
),
         (
1
,
2
), (
2
,
3
), (
4
,
5
), (
5
,
8
), (
3
,
13
),
         (
1
,
21
), (
2
,
34
), (
4
,
55
)];
assert_eq!
(s.binary_search_by_key(
&
13
, |
&
(a, b)| b),
Ok
(
9
));
assert_eq!
(s.binary_search_by_key(
&
4
, |
&
(a, b)| b),
Err
(
7
));
assert_eq!
(s.binary_search_by_key(
&
100
, |
&
(a, b)| b),
Err
(
13
));
let
r = s.binary_search_by_key(
&
1
, |
&
(a, b)| b);
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
1.20.0
·
Source
pub fn
sort_unstable
(&mut self)
where
    T:
Ord
,
Sorts the slice
without
preserving the initial order of equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the implementation of
Ord
for
T
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if the implementation of
Ord
for
T
panics.
Sorting types that only implement
PartialOrd
such as
f32
and
f64
require
additional precautions. For example,
f32::NAN != f32::NAN
, which doesn’t fulfill the
reflexivity requirement of
Ord
. By using an alternative comparison function with
slice::sort_unstable_by
such as
f32::total_cmp
or
f64::total_cmp
that defines a
total order
users can sort slices containing floating-point values. Alternatively, if all
values in the slice are guaranteed to be in a subset for which
PartialOrd::partial_cmp
forms a
total order
, it’s possible to sort the slice with
sort_unstable_by(|a, b| a.partial_cmp(b).unwrap())
.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the implementation of
Ord
for
T
does not implement a
total order
, or if
the
Ord
implementation panics.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];

v.sort_unstable();
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
1.20.0
·
Source
pub fn
sort_unstable_by
<F>(&mut self, compare: F)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Sorts the slice with a comparison function,
without
preserving the initial order of
equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the comparison function
compare
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if
compare
panics.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the
compare
does not implement a
total order
, or if
the
compare
itself panics.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];
v.sort_unstable_by(|a, b| a.cmp(b));
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
// reverse sorting
v.sort_unstable_by(|a, b| b.cmp(a));
assert_eq!
(v, [
4
,
2
,
1
, -
3
, -
5
]);
1.20.0
·
Source
pub fn
sort_unstable_by_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function,
without
preserving the initial order of
equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if the implementation of
Ord
for
K
panics.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation panics.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
];

v.sort_unstable_by_key(|k| k.abs());
assert_eq!
(v, [
1
,
2
, -
3
,
4
, -
5
]);
1.49.0
·
Source
pub fn
select_nth_unstable
(
    &mut self,
    index:
usize
,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    T:
Ord
,
Reorders the slice such that the element at
index
is at a sort-order position. All
elements before
index
will be
<=
to this value, and all elements after will be
>=
to
it.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple that partitions the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
x <= self[index]
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
x >= self[index]
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, and so always panics on empty slices.
May panic if the implementation of
Ord
for
T
does not implement a
total order
.
§
Examples
let
mut
v = [-
5i32
,
4
,
2
, -
3
,
1
];
// Find the items `<=` to the median, the median itself, and the items `>=` to it.
let
(lesser, median, greater) = v.select_nth_unstable(
2
);
assert!
(lesser == [-
3
, -
5
] || lesser == [-
5
, -
3
]);
assert_eq!
(median,
&mut
1
);
assert!
(greater == [
4
,
2
] || greater == [
2
,
4
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [-
3
, -
5
,
1
,
2
,
4
] ||
        v == [-
5
, -
3
,
1
,
2
,
4
] ||
        v == [-
3
, -
5
,
1
,
4
,
2
] ||
        v == [-
5
, -
3
,
1
,
4
,
2
]);
1.49.0
·
Source
pub fn
select_nth_unstable_by
<F>(
    &mut self,
    index:
usize
,
    compare: F,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Reorders the slice with a comparator function such that the element at
index
is at a
sort-order position. All elements before
index
will be
<=
to this value, and all
elements after will be
>=
to it, according to the comparator function.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple partitioning the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
compare(x, self[index]).is_le()
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
compare(x, self[index]).is_ge()
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, and so always panics on empty slices.
May panic if
compare
does not implement a
total order
.
§
Examples
let
mut
v = [-
5i32
,
4
,
2
, -
3
,
1
];
// Find the items `>=` to the median, the median itself, and the items `<=` to it, by using
// a reversed comparator.
let
(before, median, after) = v.select_nth_unstable_by(
2
, |a, b| b.cmp(a));
assert!
(before == [
4
,
2
] || before == [
2
,
4
]);
assert_eq!
(median,
&mut
1
);
assert!
(after == [-
3
, -
5
] || after == [-
5
, -
3
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [
2
,
4
,
1
, -
5
, -
3
] ||
        v == [
2
,
4
,
1
, -
3
, -
5
] ||
        v == [
4
,
2
,
1
, -
5
, -
3
] ||
        v == [
4
,
2
,
1
, -
3
, -
5
]);
1.49.0
·
Source
pub fn
select_nth_unstable_by_key
<K, F>(
    &mut self,
    index:
usize
,
    f: F,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Reorders the slice with a key extraction function such that the element at
index
is at a
sort-order position. All elements before
index
will have keys
<=
to the key at
index
,
and all elements after will have keys
>=
to it.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple partitioning the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
f(x) <= f(self[index])
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
f(x) >= f(self[index])
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, meaning it always panics on empty slices.
May panic if
K: Ord
does not implement a total order.
§
Examples
let
mut
v = [-
5i32
,
4
,
1
, -
3
,
2
];
// Find the items `<=` to the absolute median, the absolute median itself, and the items
// `>=` to it.
let
(lesser, median, greater) = v.select_nth_unstable_by_key(
2
, |a| a.abs());
assert!
(lesser == [
1
,
2
] || lesser == [
2
,
1
]);
assert_eq!
(median,
&mut
-
3
);
assert!
(greater == [
4
, -
5
] || greater == [-
5
,
4
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [
1
,
2
, -
3
,
4
, -
5
] ||
        v == [
1
,
2
, -
3
, -
5
,
4
] ||
        v == [
2
,
1
, -
3
,
4
, -
5
] ||
        v == [
2
,
1
, -
3
, -
5
,
4
]);
Source
pub fn
partition_dedup
(&mut self) -> (&mut
[T]
, &mut
[T]
)
where
    T:
PartialEq
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all consecutive repeated elements to the end of the slice according to the
PartialEq
trait implementation.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
1
,
2
,
2
,
3
,
3
,
2
,
1
,
1
];
let
(dedup, duplicates) = slice.partition_dedup();
assert_eq!
(dedup, [
1
,
2
,
3
,
2
,
1
]);
assert_eq!
(duplicates, [
2
,
3
,
1
]);
Source
pub fn
partition_dedup_by
<F>(&mut self, same_bucket: F) -> (&mut
[T]
, &mut
[T]
)
where
    F:
FnMut
(
&mut T
,
&mut T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all but the first of consecutive elements to the end of the slice satisfying
a given equality relation.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
The
same_bucket
function is passed references to two elements from the slice and
must determine if the elements compare equal. The elements are passed in opposite order
from their order in the slice, so if
same_bucket(a, b)
returns
true
,
a
is moved
at the end of the slice.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
"foo"
,
"Foo"
,
"BAZ"
,
"Bar"
,
"bar"
,
"baz"
,
"BAZ"
];
let
(dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));
assert_eq!
(dedup, [
"foo"
,
"BAZ"
,
"Bar"
,
"baz"
]);
assert_eq!
(duplicates, [
"bar"
,
"Foo"
,
"BAZ"
]);
Source
pub fn
partition_dedup_by_key
<K, F>(&mut self, key: F) -> (&mut
[T]
, &mut
[T]
)
where
    F:
FnMut
(
&mut T
) -> K,
    K:
PartialEq
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all but the first of consecutive elements to the end of the slice that resolve
to the same key.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
10
,
20
,
21
,
30
,
30
,
20
,
11
,
13
];
let
(dedup, duplicates) = slice.partition_dedup_by_key(|i|
*
i /
10
);
assert_eq!
(dedup, [
10
,
20
,
30
,
20
,
11
]);
assert_eq!
(duplicates, [
21
,
30
,
13
]);
1.26.0
·
Source
pub fn
rotate_left
(&mut self, mid:
usize
)
Rotates the slice in-place such that the first
mid
elements of the
slice move to the end while the last
self.len() - mid
elements move to
the front.
After calling
rotate_left
, the element previously at index
mid
will
become the first element in the slice.
§
Panics
This function will panic if
mid
is greater than the length of the
slice. Note that
mid == self.len()
does
not
panic and is a no-op
rotation.
§
Complexity
Takes linear (in
self.len()
) time.
§
Examples
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a.rotate_left(
2
);
assert_eq!
(a, [
'c'
,
'd'
,
'e'
,
'f'
,
'a'
,
'b'
]);
Rotating a subslice:
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a[
1
..
5
].rotate_left(
1
);
assert_eq!
(a, [
'a'
,
'c'
,
'd'
,
'e'
,
'b'
,
'f'
]);
1.26.0
·
Source
pub fn
rotate_right
(&mut self, k:
usize
)
Rotates the slice in-place such that the first
self.len() - k
elements of the slice move to the end while the last
k
elements move
to the front.
After calling
rotate_right
, the element previously at index
self.len() - k
will become the first element in the slice.
§
Panics
This function will panic if
k
is greater than the length of the
slice. Note that
k == self.len()
does
not
panic and is a no-op
rotation.
§
Complexity
Takes linear (in
self.len()
) time.
§
Examples
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a.rotate_right(
2
);
assert_eq!
(a, [
'e'
,
'f'
,
'a'
,
'b'
,
'c'
,
'd'
]);
Rotating a subslice:
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a[
1
..
5
].rotate_right(
1
);
assert_eq!
(a, [
'a'
,
'e'
,
'b'
,
'c'
,
'd'
,
'f'
]);
1.50.0
·
Source
pub fn
fill
(&mut self, value: T)
where
    T:
Clone
,
Fills
self
with elements by cloning
value
.
§
Examples
let
mut
buf =
vec!
[
0
;
10
];
buf.fill(
1
);
assert_eq!
(buf,
vec!
[
1
;
10
]);
1.51.0
·
Source
pub fn
fill_with
<F>(&mut self, f: F)
where
    F:
FnMut
() -> T,
Fills
self
with elements returned by calling a closure repeatedly.
This method uses a closure to create new values. If you’d rather
Clone
a given value, use
fill
. If you want to use the
Default
trait to generate values, you can pass
Default::default
as the
argument.
§
Examples
let
mut
buf =
vec!
[
1
;
10
];
buf.fill_with(Default::default);
assert_eq!
(buf,
vec!
[
0
;
10
]);
1.7.0
·
Source
pub fn
clone_from_slice
(&mut self, src: &
[T]
)
where
    T:
Clone
,
Copies the elements from
src
into
self
.
The length of
src
must be the same as
self
.
§
Panics
This function will panic if the two slices have different lengths.
§
Examples
Cloning two elements from a slice into another:
let
src = [
1
,
2
,
3
,
4
];
let
mut
dst = [
0
,
0
];
// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.clone_from_slice(
&
src[
2
..]);
assert_eq!
(src, [
1
,
2
,
3
,
4
]);
assert_eq!
(dst, [
3
,
4
]);
Rust enforces that there can only be one mutable reference with no
immutable references to a particular piece of data in a particular
scope. Because of this, attempting to use
clone_from_slice
on a
single slice will result in a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

slice[..
2
].clone_from_slice(
&
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.clone_from_slice(
&
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
4
,
5
]);
1.9.0
·
Source
pub fn
copy_from_slice
(&mut self, src: &
[T]
)
where
    T:
Copy
,
Copies all elements from
src
into
self
, using a memcpy.
The length of
src
must be the same as
self
.
If
T
does not implement
Copy
, use
clone_from_slice
.
§
Panics
This function will panic if the two slices have different lengths.
§
Examples
Copying two elements from a slice into another:
let
src = [
1
,
2
,
3
,
4
];
let
mut
dst = [
0
,
0
];
// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.copy_from_slice(
&
src[
2
..]);
assert_eq!
(src, [
1
,
2
,
3
,
4
]);
assert_eq!
(dst, [
3
,
4
]);
Rust enforces that there can only be one mutable reference with no
immutable references to a particular piece of data in a particular
scope. Because of this, attempting to use
copy_from_slice
on a
single slice will result in a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

slice[..
2
].copy_from_slice(
&
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.copy_from_slice(
&
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
4
,
5
]);
1.37.0
·
Source
pub fn
copy_within
<R>(&mut self, src: R, dest:
usize
)
where
    R:
RangeBounds
<
usize
>,
    T:
Copy
,
Copies elements from one part of the slice to another part of itself,
using a memmove.
src
is the range within
self
to copy from.
dest
is the starting
index of the range within
self
to copy to, which will have the same
length as
src
. The two ranges may overlap. The ends of the two ranges
must be less than or equal to
self.len()
.
§
Panics
This function will panic if either range exceeds the end of the slice,
or if the end of
src
is before the start.
§
Examples
Copying four bytes within a slice:
let
mut
bytes =
*
b"Hello, World!"
;

bytes.copy_within(
1
..
5
,
8
);
assert_eq!
(
&
bytes,
b"Hello, Wello!"
);
1.27.0
·
Source
pub fn
swap_with_slice
(&mut self, other: &mut
[T]
)
Swaps all elements in
self
with those in
other
.
The length of
other
must be the same as
self
.
§
Panics
This function will panic if the two slices have different lengths.
§
Example
Swapping two elements across slices:
let
mut
slice1 = [
0
,
0
];
let
mut
slice2 = [
1
,
2
,
3
,
4
];

slice1.swap_with_slice(
&mut
slice2[
2
..]);
assert_eq!
(slice1, [
3
,
4
]);
assert_eq!
(slice2, [
1
,
2
,
0
,
0
]);
Rust enforces that there can only be one mutable reference to a
particular piece of data in a particular scope. Because of this,
attempting to use
swap_with_slice
on a single slice will result in
a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];
slice[..
2
].swap_with_slice(
&mut
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
mutable sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.swap_with_slice(
&mut
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
1
,
2
]);
1.30.0
·
Source
pub unsafe fn
align_to
<U>(&self) -> (&
[T]
, &
[U]
, &
[T]
)
Transmutes the slice to a slice of another type, ensuring alignment of the types is
maintained.
This method splits the slice into three distinct slices: prefix, correctly aligned middle
slice of a new type, and the suffix slice. The middle part will be as big as possible under
the given alignment constraint and element size.
This method has no purpose when either input element
T
or output element
U
are
zero-sized and will return the original slice without splitting anything.
§
Safety
This method is essentially a
transmute
with respect to the elements in the returned
middle slice, so all the usual caveats pertaining to
transmute::<T, U>
also apply here.
§
Examples
Basic usage:
unsafe
{
let
bytes: [u8;
7
] = [
1
,
2
,
3
,
4
,
5
,
6
,
7
];
let
(prefix, shorts, suffix) = bytes.align_to::<u16>();
// less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
1.30.0
·
Source
pub unsafe fn
align_to_mut
<U>(&mut self) -> (&mut
[T]
, &mut
[U]
, &mut
[T]
)
Transmutes the mutable slice to a mutable slice of another type, ensuring alignment of the
types is maintained.
This method splits the slice into three distinct slices: prefix, correctly aligned middle
slice of a new type, and the suffix slice. The middle part will be as big as possible under
the given alignment constraint and element size.
This method has no purpose when either input element
T
or output element
U
are
zero-sized and will return the original slice without splitting anything.
§
Safety
This method is essentially a
transmute
with respect to the elements in the returned
middle slice, so all the usual caveats pertaining to
transmute::<T, U>
also apply here.
§
Examples
Basic usage:
unsafe
{
let
mut
bytes: [u8;
7
] = [
1
,
2
,
3
,
4
,
5
,
6
,
7
];
let
(prefix, shorts, suffix) = bytes.align_to_mut::<u16>();
// less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
Source
pub fn
as_simd
<const LANES:
usize
>(&self) -> (&
[T]
, &[
Simd
<T, LANES>], &
[T]
)
where
Simd
<T, LANES>:
AsRef
<
[T; LANES]
>,
    T:
SimdElement
,
LaneCount
<LANES>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.
This is a safe wrapper around
slice::align_to
, so inherits the same
guarantees as that method.
§
Panics
This will panic if the size of the SIMD type is different from
LANES
times that of the scalar.
At the time of writing, the trait restrictions on
Simd<T, LANES>
keeps
that from ever happening, as only power-of-two numbers of lanes are
supported.  It’s possible that, in the future, those restrictions might
be lifted in a way that would make it possible to see panics from this
method for something like
LANES == 3
.
§
Examples
#![feature(portable_simd)]
use
core::simd::prelude::
*
;
let
short =
&
[
1
,
2
,
3
];
let
(prefix, middle, suffix) = short.as_simd::<
4
>();
assert_eq!
(middle, []);
// Not enough elements for anything in the middle

// They might be split in any possible way between prefix and suffix
let
it = prefix.iter().chain(suffix).copied();
assert_eq!
(it.collect::<Vec<
_
>>(),
vec!
[
1
,
2
,
3
]);
fn
basic_simd_sum(x:
&
[f32]) -> f32 {
use
std::ops::Add;
let
(prefix, middle, suffix) = x.as_simd();
let
sums = f32x4::from_array([
        prefix.iter().copied().sum(),
0.0
,
0.0
,
        suffix.iter().copied().sum(),
    ]);
let
sums = middle.iter().copied().fold(sums, f32x4::add);
    sums.reduce_sum()
}
let
numbers: Vec<f32> = (
1
..
101
).map(|x| x
as _
).collect();
assert_eq!
(basic_simd_sum(
&
numbers[
1
..
99
]),
4949.0
);
Source
pub fn
as_simd_mut
<const LANES:
usize
>(
    &mut self,
) -> (&mut
[T]
, &mut [
Simd
<T, LANES>], &mut
[T]
)
where
Simd
<T, LANES>:
AsMut
<
[T; LANES]
>,
    T:
SimdElement
,
LaneCount
<LANES>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Splits a mutable slice into a mutable prefix, a middle of aligned SIMD types,
and a mutable suffix.
This is a safe wrapper around
slice::align_to_mut
, so inherits the same
guarantees as that method.
This is the mutable version of
slice::as_simd
; see that for examples.
§
Panics
This will panic if the size of the SIMD type is different from
LANES
times that of the scalar.
At the time of writing, the trait restrictions on
Simd<T, LANES>
keeps
that from ever happening, as only power-of-two numbers of lanes are
supported.  It’s possible that, in the future, those restrictions might
be lifted in a way that would make it possible to see panics from this
method for something like
LANES == 3
.
1.82.0
·
Source
pub fn
is_sorted
(&self) ->
bool
where
    T:
PartialOrd
,
Checks if the elements of this slice are sorted.
That is, for each element
a
and its following element
b
,
a <= b
must hold. If the
slice yields exactly zero or one element,
true
is returned.
Note that if
Self::Item
is only
PartialOrd
, but not
Ord
, the above definition
implies that this function returns
false
if any two consecutive items are not
comparable.
§
Examples
let
empty: [i32;
0
] = [];
assert!
([
1
,
2
,
2
,
9
].is_sorted());
assert!
(![
1
,
3
,
2
,
4
].is_sorted());
assert!
([
0
].is_sorted());
assert!
(empty.is_sorted());
assert!
(![
0.0
,
1.0
, f32::NAN].is_sorted());
1.82.0
·
Source
pub fn
is_sorted_by
<'a, F>(&'a self, compare: F) ->
bool
where
    F:
FnMut
(
&'a T
,
&'a T
) ->
bool
,
Checks if the elements of this slice are sorted using the given comparator function.
Instead of using
PartialOrd::partial_cmp
, this function uses the given
compare
function to determine whether two elements are to be considered in sorted order.
§
Examples
assert!
([
1
,
2
,
2
,
9
].is_sorted_by(|a, b| a <= b));
assert!
(![
1
,
2
,
2
,
9
].is_sorted_by(|a, b| a < b));
assert!
([
0
].is_sorted_by(|a, b|
true
));
assert!
([
0
].is_sorted_by(|a, b|
false
));
let
empty: [i32;
0
] = [];
assert!
(empty.is_sorted_by(|a, b|
false
));
assert!
(empty.is_sorted_by(|a, b|
true
));
1.82.0
·
Source
pub fn
is_sorted_by_key
<'a, F, K>(&'a self, f: F) ->
bool
where
    F:
FnMut
(
&'a T
) -> K,
    K:
PartialOrd
,
Checks if the elements of this slice are sorted using the given key extraction function.
Instead of comparing the slice’s elements directly, this function compares the keys of the
elements, as determined by
f
. Apart from that, it’s equivalent to
is_sorted
; see its
documentation for more information.
§
Examples
assert!
([
"c"
,
"bb"
,
"aaa"
].is_sorted_by_key(|s| s.len()));
assert!
(![-
2i32
, -
1
,
0
,
3
].is_sorted_by_key(|n| n.abs()));
1.52.0
·
Source
pub fn
partition_point
<P>(&self, pred: P) ->
usize
where
    P:
FnMut
(
&T
) ->
bool
,
Returns the index of the partition point according to the given predicate
(the index of the first element of the second partition).
The slice is assumed to be partitioned according to the given predicate.
This means that all elements for which the predicate returns true are at the start of the slice
and all elements for which the predicate returns false are at the end.
For example,
[7, 15, 3, 5, 4, 12, 6]
is partitioned under the predicate
x % 2 != 0
(all odd numbers are at the start, all even at the end).
If this slice is not partitioned, the returned result is unspecified and meaningless,
as this method performs a kind of binary search.
See also
binary_search
,
binary_search_by
, and
binary_search_by_key
.
§
Examples
let
v = [
1
,
2
,
3
,
3
,
5
,
6
,
7
];
let
i = v.partition_point(|
&
x| x <
5
);
assert_eq!
(i,
4
);
assert!
(v[..i].iter().all(|
&
x| x <
5
));
assert!
(v[i..].iter().all(|
&
x| !(x <
5
)));
If all elements of the slice match the predicate, including if the slice
is empty, then the length of the slice will be returned:
let
a = [
2
,
4
,
8
];
assert_eq!
(a.partition_point(|x| x <
&
100
), a.len());
let
a: [i32;
0
] = [];
assert_eq!
(a.partition_point(|x| x <
&
100
),
0
);
If you want to insert an item to a sorted vector, while maintaining
sort order:
let
mut
s =
vec!
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
num =
42
;
let
idx = s.partition_point(|
&
x| x <= num);
s.insert(idx, num);
assert_eq!
(s, [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
1.87.0
·
Source
pub fn
split_off
<'a, R>(self: &mut &'a
[T]
, range: R) ->
Option
<&'a
[T]
>
where
    R:
OneSidedRange
<
usize
>,
Removes the subslice corresponding to the given range
and returns a reference to it.
Returns
None
and does not modify the slice if the given
range is out of bounds.
Note that this method only accepts one-sided ranges such as
2..
or
..6
, but not
2..6
.
§
Examples
Splitting off the first three elements of a slice:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
first_three = slice.split_off(..
3
).unwrap();
assert_eq!
(slice,
&
[
'd'
]);
assert_eq!
(first_three,
&
[
'a'
,
'b'
,
'c'
]);
Splitting off the last two elements of a slice:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
tail = slice.split_off(
2
..).unwrap();
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(tail,
&
[
'c'
,
'd'
]);
Getting
None
when
range
is out of bounds:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
None
, slice.split_off(
5
..));
assert_eq!
(
None
, slice.split_off(..
5
));
assert_eq!
(
None
, slice.split_off(..=
4
));
let
expected:
&
[char] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
Some
(expected), slice.split_off(..
4
));
1.87.0
·
Source
pub fn
split_off_mut
<'a, R>(
    self: &mut &'a mut
[T]
,
    range: R,
) ->
Option
<&'a mut
[T]
>
where
    R:
OneSidedRange
<
usize
>,
Removes the subslice corresponding to the given range
and returns a mutable reference to it.
Returns
None
and does not modify the slice if the given
range is out of bounds.
Note that this method only accepts one-sided ranges such as
2..
or
..6
, but not
2..6
.
§
Examples
Splitting off the first three elements of a slice:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
first_three = slice.split_off_mut(..
3
).unwrap();
assert_eq!
(slice,
&mut
[
'd'
]);
assert_eq!
(first_three,
&mut
[
'a'
,
'b'
,
'c'
]);
Taking the last two elements of a slice:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
tail = slice.split_off_mut(
2
..).unwrap();
assert_eq!
(slice,
&mut
[
'a'
,
'b'
]);
assert_eq!
(tail,
&mut
[
'c'
,
'd'
]);
Getting
None
when
range
is out of bounds:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
None
, slice.split_off_mut(
5
..));
assert_eq!
(
None
, slice.split_off_mut(..
5
));
assert_eq!
(
None
, slice.split_off_mut(..=
4
));
let
expected:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
Some
(expected), slice.split_off_mut(..
4
));
1.87.0
·
Source
pub fn
split_off_first
<'a>(self: &mut &'a
[T]
) ->
Option
<
&'a T
>
Removes the first element of the slice and returns a reference
to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
];
let
first = slice.split_off_first().unwrap();
assert_eq!
(slice,
&
[
'b'
,
'c'
]);
assert_eq!
(first,
&
'a'
);
1.87.0
·
Source
pub fn
split_off_first_mut
<'a>(self: &mut &'a mut
[T]
) ->
Option
<
&'a mut T
>
Removes the first element of the slice and returns a mutable
reference to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
];
let
first = slice.split_off_first_mut().unwrap();
*
first =
'd'
;
assert_eq!
(slice,
&
[
'b'
,
'c'
]);
assert_eq!
(first,
&
'd'
);
1.87.0
·
Source
pub fn
split_off_last
<'a>(self: &mut &'a
[T]
) ->
Option
<
&'a T
>
Removes the last element of the slice and returns a reference
to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
];
let
last = slice.split_off_last().unwrap();
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(last,
&
'c'
);
1.87.0
·
Source
pub fn
split_off_last_mut
<'a>(self: &mut &'a mut
[T]
) ->
Option
<
&'a mut T
>
Removes the last element of the slice and returns a mutable
reference to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
];
let
last = slice.split_off_last_mut().unwrap();
*
last =
'd'
;
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(last,
&
'd'
);
1.86.0
·
Source
pub unsafe fn
get_disjoint_unchecked_mut
<I, const N:
usize
>(
    &mut self,
    indices:
[I; N]
,
) -> [&mut <I as
SliceIndex
<
[T]
>>::
Output
;
N
]
where
    I:
GetDisjointMutIndex
+
SliceIndex
<
[T]
>,
Returns mutable references to many indices at once, without doing any checks.
An index can be either a
usize
, a
Range
or a
RangeInclusive
. Note
that this method takes an array, so all indices must be of the same type.
If passed an array of
usize
s this method gives back an array of mutable references
to single elements, while if passed an array of ranges it gives back an array of
mutable references to slices.
For a safe alternative see
get_disjoint_mut
.
§
Safety
Calling this method with overlapping or out-of-bounds indices is
undefined behavior
even if the resulting references are not used.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
0
,
2
]);
*
a
*
=
10
;
*
b
*
=
100
;
}
assert_eq!
(x,
&
[
10
,
2
,
400
]);
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
0
..
1
,
1
..
3
]);
    a[
0
] =
8
;
    b[
0
] =
88
;
    b[
1
] =
888
;
}
assert_eq!
(x,
&
[
8
,
88
,
888
]);
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
1
..=
2
,
0
..=
0
]);
    a[
0
] =
11
;
    a[
1
] =
111
;
    b[
0
] =
1
;
}
assert_eq!
(x,
&
[
1
,
11
,
111
]);
1.86.0
·
Source
pub fn
get_disjoint_mut
<I, const N:
usize
>(
    &mut self,
    indices:
[I; N]
,
) ->
Result
<[&mut <I as
SliceIndex
<
[T]
>>::
Output
;
N
],
GetDisjointMutError
>
where
    I:
GetDisjointMutIndex
+
SliceIndex
<
[T]
>,
Returns mutable references to many indices at once.
An index can be either a
usize
, a
Range
or a
RangeInclusive
. Note
that this method takes an array, so all indices must be of the same type.
If passed an array of
usize
s this method gives back an array of mutable references
to single elements, while if passed an array of ranges it gives back an array of
mutable references to slices.
Returns an error if any index is out-of-bounds, or if there are overlapping indices.
An empty range is not considered to overlap if it is located at the beginning or at
the end of another range, but is considered to overlap if it is located in the middle.
This method does a O(n^2) check to check that there are no overlapping indices, so be careful
when passing many indices.
§
Examples
let
v =
&mut
[
1
,
2
,
3
];
if let
Ok
([a, b]) = v.get_disjoint_mut([
0
,
2
]) {
*
a =
413
;
*
b =
612
;
}
assert_eq!
(v,
&
[
413
,
2
,
612
]);
if let
Ok
([a, b]) = v.get_disjoint_mut([
0
..
1
,
1
..
3
]) {
    a[
0
] =
8
;
    b[
0
] =
88
;
    b[
1
] =
888
;
}
assert_eq!
(v,
&
[
8
,
88
,
888
]);
if let
Ok
([a, b]) = v.get_disjoint_mut([
1
..=
2
,
0
..=
0
]) {
    a[
0
] =
11
;
    a[
1
] =
111
;
    b[
0
] =
1
;
}
assert_eq!
(v,
&
[
1
,
11
,
111
]);
Source
pub fn
element_offset
(&self, element:
&T
) ->
Option
<
usize
>
🔬
This is a nightly-only experimental API. (
substr_range
#126769
)
Returns the index that an element reference points to.
Returns
None
if
element
does not point to the start of an element within the slice.
This method is useful for extending slice iterators like
slice::split
.
Note that this uses pointer arithmetic and
does not compare elements
.
To find the index of an element via comparison, use
.iter().position()
instead.
§
Panics
Panics if
T
is zero-sized.
§
Examples
Basic usage:
#![feature(substr_range)]
let
nums:
&
[u32] =
&
[
1
,
7
,
1
,
1
];
let
num =
&
nums[
2
];
assert_eq!
(num,
&
1
);
assert_eq!
(nums.element_offset(num),
Some
(
2
));
Returning
None
with an unaligned element:
#![feature(substr_range)]
let
arr:
&
[[u32;
2
]] =
&
[[
0
,
1
], [
2
,
3
]];
let
flat_arr:
&
[u32] = arr.as_flattened();
let
ok_elm:
&
[u32;
2
] = flat_arr[
0
..
2
].try_into().unwrap();
let
weird_elm:
&
[u32;
2
] = flat_arr[
1
..
3
].try_into().unwrap();
assert_eq!
(ok_elm,
&
[
0
,
1
]);
assert_eq!
(weird_elm,
&
[
1
,
2
]);
assert_eq!
(arr.element_offset(ok_elm),
Some
(
0
));
// Points to element 0
assert_eq!
(arr.element_offset(weird_elm),
None
);
// Points between element 0 and 1
Source
pub fn
subslice_range
(&self, subslice: &
[T]
) ->
Option
<
Range
<
usize
>>
🔬
This is a nightly-only experimental API. (
substr_range
#126769
)
Returns the range of indices that a subslice points to.
Returns
None
if
subslice
does not point within the slice or if it is not aligned with the
elements in the slice.
This method
does not compare elements
. Instead, this method finds the location in the slice that
subslice
was obtained from. To find the index of a subslice via comparison, instead use
.windows()
.position()
.
This method is useful for extending slice iterators like
slice::split
.
Note that this may return a false positive (either
Some(0..0)
or
Some(self.len()..self.len())
)
if
subslice
has a length of zero and points to the beginning or end of another, separate, slice.
§
Panics
Panics if
T
is zero-sized.
§
Examples
Basic usage:
#![feature(substr_range)]
let
nums =
&
[
0
,
5
,
10
,
0
,
0
,
5
];
let
mut
iter = nums
    .split(|t|
*
t ==
0
)
    .map(|n| nums.subslice_range(n).unwrap());
assert_eq!
(iter.next(),
Some
(
0
..
0
));
assert_eq!
(iter.next(),
Some
(
1
..
3
));
assert_eq!
(iter.next(),
Some
(
4
..
4
));
assert_eq!
(iter.next(),
Some
(
5
..
6
));
1.0.0
·
Source
pub fn
sort
(&mut self)
where
    T:
Ord
,
Sorts the slice, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
n
* log(
n
))
worst-case.
If the implementation of
Ord
for
T
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
When applicable, unstable sorting is preferred because it is generally faster than stable
sorting and it doesn’t allocate auxiliary memory. See
sort_unstable
. The exception are partially sorted slices, which
may be better served with
slice::sort
.
Sorting types that only implement
PartialOrd
such as
f32
and
f64
require
additional precautions. For example,
f32::NAN != f32::NAN
, which doesn’t fulfill the
reflexivity requirement of
Ord
. By using an alternative comparison function with
slice::sort_by
such as
f32::total_cmp
or
f64::total_cmp
that defines a
total
order
users can sort slices containing floating-point values. Alternatively, if all values
in the slice are guaranteed to be in a subset for which
PartialOrd::partial_cmp
forms a
total order
, it’s possible to sort the slice with
sort_by(|a, b| a.partial_cmp(b).unwrap())
.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if the implementation of
Ord
for
T
does not implement a
total order
, or if
the
Ord
implementation itself panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];

v.sort();
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
1.0.0
·
Source
pub fn
sort_by
<F>(&mut self, compare: F)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Sorts the slice with a comparison function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
n
* log(
n
))
worst-case.
If the comparison function
compare
does not implement a
total order
, the function may
panic; even if the function exits normally, the resulting order of elements in the slice is
unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if
compare
does not implement a
total order
, or if
compare
itself panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];
v.sort_by(|a, b| a.cmp(b));
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
// reverse sorting
v.sort_by(|a, b| b.cmp(a));
assert_eq!
(v, [
4
,
2
,
1
, -
3
, -
5
]);
1.7.0
·
Source
pub fn
sort_by_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
m
*
n
* log(
n
))
worst-case, where the key function is
O
(
m
).
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation or the key-function
f
panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
];

v.sort_by_key(|k| k.abs());
assert_eq!
(v, [
1
,
2
, -
3
,
4
, -
5
]);
1.34.0
·
Source
pub fn
sort_by_cached_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
m
*
n
+
n
*
log(
n
)) worst-case, where the key function is
O
(
m
).
During sorting, the key function is called at most once per element, by using temporary
storage to remember the results of key evaluation. The order of calls to the key function is
unspecified and may change in future versions of the standard library.
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For simple key functions (e.g., functions that are property accesses or basic operations),
sort_by_key
is likely to be faster.
§
Current implementation
The current implementation is based on
instruction-parallel-network sort
by Lukas
Bergdoll, which combines the fast average case of randomized quicksort with the fast worst
case of heapsort, while achieving linear time on fully sorted and reversed inputs. And
O
(
k
* log(
n
)) where
k
is the number of distinct elements in the input. It leverages
superscalar out-of-order execution capabilities commonly found in CPUs, to efficiently
perform the operation.
In the worst case, the algorithm allocates temporary storage in a
Vec<(K, usize)>
the
length of the slice.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
,
10
];
// Strings are sorted by lexicographical order.
v.sort_by_cached_key(|k| k.to_string());
assert_eq!
(v, [-
3
, -
5
,
1
,
10
,
2
,
4
]);
1.0.0
·
Source
pub fn
to_vec
(&self) ->
Vec
<T>
where
    T:
Clone
,
Copies
self
into a new
Vec
.
§
Examples
let
s = [
10
,
40
,
30
];
let
x = s.to_vec();
// Here, `s` and `x` can be modified independently.
Source
pub fn
to_vec_in
<A>(&self, alloc: A) ->
Vec
<T, A>
where
    A:
Allocator
,
    T:
Clone
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Copies
self
into a new
Vec
with an allocator.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
s = [
10
,
40
,
30
];
let
x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
1.40.0
·
Source
pub fn
repeat
(&self, n:
usize
) ->
Vec
<T>
where
    T:
Copy
,
Creates a vector by copying a slice
n
times.
§
Panics
This function will panic if the capacity would overflow.
§
Examples
Basic usage:
assert_eq!
([
1
,
2
].repeat(
3
),
vec!
[
1
,
2
,
1
,
2
,
1
,
2
]);
A panic upon overflow:
ⓘ
// this will panic at runtime
b"0123456789abcdef"
.repeat(usize::MAX);
1.0.0
·
Source
pub fn
concat
<Item>(&self) -> <
[T]
as
Concat
<Item>>::
Output
ⓘ
where
[T]
:
Concat
<Item>,
    Item: ?
Sized
,
Flattens a slice of
T
into a single value
Self::Output
.
§
Examples
assert_eq!
([
"hello"
,
"world"
].concat(),
"helloworld"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].concat(), [
1
,
2
,
3
,
4
]);
1.3.0
·
Source
pub fn
join
<Separator>(
    &self,
    sep: Separator,
) -> <
[T]
as
Join
<Separator>>::
Output
ⓘ
where
[T]
:
Join
<Separator>,
Flattens a slice of
T
into a single value
Self::Output
, placing a
given separator between each.
§
Examples
assert_eq!
([
"hello"
,
"world"
].join(
" "
),
"hello world"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].join(
&
0
), [
1
,
2
,
0
,
3
,
4
]);
assert_eq!
([[
1
,
2
], [
3
,
4
]].join(
&
[
0
,
0
][..]), [
1
,
2
,
0
,
0
,
3
,
4
]);
1.0.0
·
Source
pub fn
connect
<Separator>(
    &self,
    sep: Separator,
) -> <
[T]
as
Join
<Separator>>::
Output
ⓘ
where
[T]
:
Join
<Separator>,
👎
Deprecated since 1.3.0: renamed to join
Flattens a slice of
T
into a single value
Self::Output
, placing a
given separator between each.
§
Examples
assert_eq!
([
"hello"
,
"world"
].connect(
" "
),
"hello world"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].connect(
&
0
), [
1
,
2
,
0
,
3
,
4
]);
Trait Implementations
§
Source
§
impl
AsMut
<[
u8
]> for
ByteString
Source
§
fn
as_mut
(&mut self) -> &mut [
u8
]
ⓘ
Converts this type into a mutable reference of the (usually inferred) input type.
Source
§
impl
AsMut
<
ByteStr
> for
ByteString
Source
§
fn
as_mut
(&mut self) -> &mut
ByteStr
Converts this type into a mutable reference of the (usually inferred) input type.
Source
§
impl
AsRef
<[
u8
]> for
ByteString
Source
§
fn
as_ref
(&self) -> &[
u8
]
ⓘ
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl
AsRef
<
ByteStr
> for
ByteString
Source
§
fn
as_ref
(&self) -> &
ByteStr
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl
Borrow
<[
u8
]> for
ByteString
Source
§
fn
borrow
(&self) -> &[
u8
]
ⓘ
Immutably borrows from an owned value.
Read more
Source
§
impl
Borrow
<
ByteStr
> for
ByteString
Source
§
fn
borrow
(&self) -> &
ByteStr
Immutably borrows from an owned value.
Read more
Source
§
impl
BorrowMut
<[
u8
]> for
ByteString
Source
§
fn
borrow_mut
(&mut self) -> &mut [
u8
]
ⓘ
Mutably borrows from an owned value.
Read more
Source
§
impl
BorrowMut
<
ByteStr
> for
ByteString
Source
§
fn
borrow_mut
(&mut self) -> &mut
ByteStr
Mutably borrows from an owned value.
Read more
Source
§
impl
Clone
for
ByteString
Source
§
fn
clone
(&self) ->
ByteString
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
Source
§
impl
Debug
for
ByteString
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
Source
§
impl
Default
for
ByteString
Source
§
fn
default
() ->
ByteString
Returns the “default value” for a type.
Read more
Source
§
impl
Deref
for
ByteString
Source
§
type
Target
=
Vec
<
u8
>
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &<
ByteString
as
Deref
>::
Target
Dereferences the value.
Source
§
impl
DerefMut
for
ByteString
Source
§
fn
deref_mut
(&mut self) -> &mut <
ByteString
as
Deref
>::
Target
Mutably dereferences the value.
Source
§
impl
Display
for
ByteString
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
Source
§
impl<'a>
From
<&'a
ByteStr
> for
ByteString
Source
§
fn
from
(s: &'a
ByteStr
) ->
ByteString
Converts to this type from the input type.
Source
§
impl<'a>
From
<&'a
ByteString
> for
Cow
<'a,
ByteStr
>
Source
§
fn
from
(s: &'a
ByteString
) ->
Cow
<'a,
ByteStr
>
Converts to this type from the input type.
Source
§
impl<'a>
From
<
ByteString
> for
Cow
<'a,
ByteStr
>
Source
§
fn
from
(s:
ByteString
) ->
Cow
<'a,
ByteStr
>
Converts to this type from the input type.
Source
§
impl
From
<
ByteString
> for
Vec
<
u8
>
Source
§
fn
from
(s:
ByteString
) ->
Vec
<
u8
>
ⓘ
Converts to this type from the input type.
Source
§
impl<'a>
FromIterator
<&'a [
u8
]> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item = &'a [
u8
]>,
Creates a value from an iterator.
Read more
Source
§
impl<'a>
FromIterator
<&'a
ByteStr
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item = &'a
ByteStr
>,
Creates a value from an iterator.
Read more
Source
§
impl<'a>
FromIterator
<&'a
str
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item = &'a
str
>,
Creates a value from an iterator.
Read more
Source
§
impl
FromIterator
<
ByteString
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item =
ByteString
>,
Creates a value from an iterator.
Read more
Source
§
impl
FromIterator
<
char
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item =
char
>,
Creates a value from an iterator.
Read more
Source
§
impl
FromIterator
<
u8
> for
ByteString
Source
§
fn
from_iter
<T>(iter: T) ->
ByteString
where
    T:
IntoIterator
<Item =
u8
>,
Creates a value from an iterator.
Read more
Source
§
impl
FromStr
for
ByteString
Source
§
type
Err
=
Infallible
The associated error which can be returned from parsing.
Source
§
fn
from_str
(s: &
str
) ->
Result
<
ByteString
, <
ByteString
as
FromStr
>::
Err
>
Parses a string
s
to return a value of this type.
Read more
Source
§
impl
Hash
for
ByteString
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
Source
§
impl
Index
<
Range
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, r:
Range
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeFrom
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, r:
RangeFrom
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeFull
> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, _:
RangeFull
) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeInclusive
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, r:
RangeInclusive
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeTo
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, r:
RangeTo
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeToInclusive
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, r:
RangeToInclusive
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
usize
> for
ByteString
Source
§
type
Output
=
u8
The returned type after indexing.
Source
§
fn
index
(&self, idx:
usize
) -> &
u8
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
Range
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
Range
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeFrom
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
RangeFrom
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeFull
> for
ByteString
Source
§
fn
index_mut
(&mut self, _:
RangeFull
) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeInclusive
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
RangeInclusive
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeTo
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
RangeTo
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeToInclusive
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
RangeToInclusive
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
usize
> for
ByteString
Source
§
fn
index_mut
(&mut self, idx:
usize
) -> &mut
u8
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
Ord
for
ByteString
Source
§
fn
cmp
(&self, other: &
ByteString
) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
Source
§
impl<'a>
PartialEq
<&[
u8
]> for
ByteString
Source
§
fn
eq
(&self, other: &&[
u8
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<&[
u8
;
N
]> for
ByteString
Source
§
fn
eq
(&self, other: &&[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<&
ByteStr
> for
ByteString
Source
§
fn
eq
(&self, other: &&
ByteStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<&
str
> for
ByteString
Source
§
fn
eq
(&self, other: &&
str
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<[
u8
]> for
ByteString
Source
§
fn
eq
(&self, other: &[
u8
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<[
u8
;
N
]> for
ByteString
Source
§
fn
eq
(&self, other: &[
u8
;
N
]) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteStr
> for
ByteString
Source
§
fn
eq
(&self, other: &
ByteStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for &[
u8
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for &[
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for &
ByteStr
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for &
str
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for [
u8
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<const N:
usize
>
PartialEq
<
ByteString
> for [
u8
;
N
]
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
ByteStr
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_, [
u8
]>
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_,
ByteStr
>
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_,
str
>
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
String
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Vec
<
u8
>
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
str
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_, [
u8
]>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_,
ByteStr
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
Cow
<'_,
str
>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_,
str
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
String
> for
ByteString
Source
§
fn
eq
(&self, other: &
String
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
Vec
<
u8
>> for
ByteString
Source
§
fn
eq
(&self, other: &
Vec
<
u8
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
str
> for
ByteString
Source
§
fn
eq
(&self, other: &
str
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl
PartialEq
for
ByteString
Source
§
fn
eq
(&self, other: &
ByteString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialOrd
<&
ByteStr
> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &&
ByteStr
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteStr
> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
ByteStr
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteString
> for &
ByteStr
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
ByteStr
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_, [
u8
]>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
ByteStr
>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
str
>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_, [
u8
]>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_,
ByteStr
>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
str
>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_,
str
>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl
PartialOrd
for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
ByteString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<'a>
TryFrom
<&'a
ByteString
> for &'a
str
Source
§
type
Error
=
Utf8Error
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    s: &'a
ByteString
,
) ->
Result
<&'a
str
, <&'a
str
as
TryFrom
<&'a
ByteString
>>::
Error
>
Performs the conversion.
Source
§
impl
TryFrom
<
ByteString
> for
String
Source
§
type
Error
=
FromUtf8Error
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    s:
ByteString
,
) ->
Result
<
String
, <
String
as
TryFrom
<
ByteString
>>::
Error
>
Performs the conversion.
Source
§
impl
DerefPure
for
ByteString
Source
§
impl
Eq
for
ByteString
Auto Trait Implementations
§
§
impl
Freeze
for
ByteString
§
impl
RefUnwindSafe
for
ByteString
§
impl
Send
for
ByteString
§
impl
Sync
for
ByteString
§
impl
Unpin
for
ByteString
§
impl
UnwindSafe
for
ByteString
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.