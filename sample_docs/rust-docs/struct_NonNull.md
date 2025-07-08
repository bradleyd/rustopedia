NonNull in std::ptr - Rust
std
::
ptr
Struct
NonNull
Copy item path
1.25.0
·
Source
pub struct NonNull<T>
where
    T: ?
Sized
,
{
/* private fields */
}
Expand description
*mut T
but non-zero and
covariant
.
This is often the correct thing to use when building data structures using
raw pointers, but is ultimately more dangerous to use because of its additional
properties. If you’re not sure if you should use
NonNull<T>
, just use
*mut T
!
Unlike
*mut T
, the pointer must always be non-null, even if the pointer
is never dereferenced. This is so that enums may use this forbidden value
as a discriminant –
Option<NonNull<T>>
has the same size as
*mut T
.
However the pointer may still dangle if it isn’t dereferenced.
Unlike
*mut T
,
NonNull<T>
was chosen to be covariant over
T
. This makes it
possible to use
NonNull<T>
when building covariant types, but introduces the
risk of unsoundness if used in a type that shouldn’t actually be covariant.
(The opposite choice was made for
*mut T
even though technically the unsoundness
could only be caused by calling unsafe functions.)
Covariance is correct for most safe abstractions, such as
Box
,
Rc
,
Arc
,
Vec
,
and
LinkedList
. This is the case because they provide a public API that follows the
normal shared XOR mutable rules of Rust.
If your type cannot safely be covariant, you must ensure it contains some
additional field to provide invariance. Often this field will be a
PhantomData
type like
PhantomData<Cell<T>>
or
PhantomData<&'a mut T>
.
Notice that
NonNull<T>
has a
From
instance for
&T
. However, this does
not change the fact that mutating through a (pointer derived from a) shared
reference is undefined behavior unless the mutation happens inside an
UnsafeCell<T>
. The same goes for creating a mutable reference from a shared
reference. When using this
From
instance without an
UnsafeCell<T>
,
it is your responsibility to ensure that
as_mut
is never called, and
as_ptr
is never used for mutation.
§
Representation
Thanks to the
null pointer optimization
,
NonNull<T>
and
Option<NonNull<T>>
are guaranteed to have the same size and alignment:
use
std::ptr::NonNull;
assert_eq!
(size_of::<NonNull<i16>>(), size_of::<
Option
<NonNull<i16>>>());
assert_eq!
(align_of::<NonNull<i16>>(), align_of::<
Option
<NonNull<i16>>>());
assert_eq!
(size_of::<NonNull<str>>(), size_of::<
Option
<NonNull<str>>>());
assert_eq!
(align_of::<NonNull<str>>(), align_of::<
Option
<NonNull<str>>>());
Implementations
§
Source
§
impl<T>
NonNull
<T>
Source
pub const fn
without_provenance
(addr:
NonZero
<
usize
>) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
nonnull_provenance
#135243
)
Creates a pointer with the given address and no
provenance
.
For more details, see the equivalent method on a raw pointer,
ptr::without_provenance_mut
.
This is a
Strict Provenance
API.
1.25.0 (const: 1.36.0)
·
Source
pub const fn
dangling
() ->
NonNull
<T>
Creates a new
NonNull
that is dangling, but well-aligned.
This is useful for initializing types which lazily allocate, like
Vec::new
does.
Note that the pointer value may potentially represent a valid pointer to
a
T
, which means this must not be used as a “not yet initialized”
sentinel value. Types that lazily allocate must track initialization by
some other means.
§
Examples
use
std::ptr::NonNull;
let
ptr = NonNull::<u32>::dangling();
// Important: don't try to access the value of `ptr` without
// initializing it first! The pointer is not null but isn't valid either!
Source
pub fn
with_exposed_provenance
(addr:
NonZero
<
usize
>) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
nonnull_provenance
#135243
)
Converts an address back to a mutable pointer, picking up some previously ‘exposed’
provenance
.
For more details, see the equivalent method on a raw pointer,
ptr::with_exposed_provenance_mut
.
This is an
Exposed Provenance
API.
Source
pub const unsafe fn
as_uninit_ref
<'a>(self) -> &'a
MaybeUninit
<T>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns a shared references to the value. In contrast to
as_ref
, this does not require
that the value has to be initialized.
For the mutable counterpart see
as_uninit_mut
.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
Note that because the created reference is to
MaybeUninit<T>
, the
source pointer can point to uninitialized memory.
Source
pub const unsafe fn
as_uninit_mut
<'a>(self) -> &'a mut
MaybeUninit
<T>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns a unique references to the value. In contrast to
as_mut
, this does not require
that the value has to be initialized.
For the shared counterpart see
as_uninit_ref
.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
Note that because the created reference is to
MaybeUninit<T>
, the
source pointer can point to uninitialized memory.
Source
§
impl<T>
NonNull
<T>
where
    T: ?
Sized
,
1.25.0 (const: 1.25.0)
·
Source
pub const unsafe fn
new_unchecked
(ptr:
*mut T
) ->
NonNull
<T>
Creates a new
NonNull
.
§
Safety
ptr
must be non-null.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
ptr =
unsafe
{ NonNull::new_unchecked(
&mut
x
as
*mut
_
) };
Incorrect
usage of this function:
use
std::ptr::NonNull;
// NEVER DO THAT!!! This is undefined behavior. ⚠️
let
ptr =
unsafe
{ NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
1.25.0 (const: 1.85.0)
·
Source
pub const fn
new
(ptr:
*mut T
) ->
Option
<
NonNull
<T>>
Creates a new
NonNull
if
ptr
is non-null.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
ptr = NonNull::<u32>::new(
&mut
x
as
*mut
_
).expect(
"ptr is null!"
);
if let
Some
(ptr) = NonNull::<u32>::new(std::ptr::null_mut()) {
unreachable!
();
}
Source
pub const fn
from_ref
(r:
&T
) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
non_null_from_ref
#130823
)
Converts a reference to a
NonNull
pointer.
Source
pub const fn
from_mut
(r:
&mut T
) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
non_null_from_ref
#130823
)
Converts a mutable reference to a
NonNull
pointer.
Source
pub const fn
from_raw_parts
(
    data_pointer:
NonNull
<impl
Thin
>,
    metadata: <T as
Pointee
>::
Metadata
,
) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Performs the same functionality as
std::ptr::from_raw_parts
, except that a
NonNull
pointer is returned, as opposed to a raw
*const
pointer.
See the documentation of
std::ptr::from_raw_parts
for more details.
Source
pub const fn
to_raw_parts
(self) -> (
NonNull
<
()
>, <T as
Pointee
>::
Metadata
)
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Decompose a (possibly wide) pointer into its data pointer and metadata components.
The pointer can be later reconstructed with
NonNull::from_raw_parts
.
1.84.0
·
Source
pub fn
addr
(self) ->
NonZero
<
usize
>
Gets the “address” portion of the pointer.
For more details, see the equivalent method on a raw pointer,
pointer::addr
.
This is a
Strict Provenance
API.
Source
pub fn
expose_provenance
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
nonnull_provenance
#135243
)
Exposes the
“provenance”
part of the pointer for future use in
with_exposed_provenance
and returns the “address” portion.
For more details, see the equivalent method on a raw pointer,
pointer::expose_provenance
.
This is an
Exposed Provenance
API.
1.84.0
·
Source
pub fn
with_addr
(self, addr:
NonZero
<
usize
>) ->
NonNull
<T>
Creates a new pointer with the given address and the
provenance
of
self
.
For more details, see the equivalent method on a raw pointer,
pointer::with_addr
.
This is a
Strict Provenance
API.
1.84.0
·
Source
pub fn
map_addr
(
    self,
    f: impl
FnOnce
(
NonZero
<
usize
>) ->
NonZero
<
usize
>,
) ->
NonNull
<T>
Creates a new pointer by mapping
self
’s address to a new one, preserving the
provenance
of
self
.
For more details, see the equivalent method on a raw pointer,
pointer::map_addr
.
This is a
Strict Provenance
API.
1.25.0 (const: 1.32.0)
·
Source
pub const fn
as_ptr
(self) ->
*mut T
Acquires the underlying
*mut
pointer.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
ptr = NonNull::new(
&mut
x).expect(
"ptr is null!"
);
let
x_value =
unsafe
{
*
ptr.as_ptr() };
assert_eq!
(x_value,
0
);
unsafe
{
*
ptr.as_ptr() +=
2
; }
let
x_value =
unsafe
{
*
ptr.as_ptr() };
assert_eq!
(x_value,
2
);
1.25.0 (const: 1.73.0)
·
Source
pub const unsafe fn
as_ref
<'a>(&self) ->
&'a T
Returns a shared reference to the value. If the value may be uninitialized,
as_uninit_ref
must be used instead.
For the mutable counterpart see
as_mut
.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
ptr = NonNull::new(
&mut
x
as
*mut
_
).expect(
"ptr is null!"
);
let
ref_x =
unsafe
{ ptr.as_ref() };
println!
(
"{ref_x}"
);
1.25.0 (const: 1.83.0)
·
Source
pub const unsafe fn
as_mut
<'a>(&mut self) ->
&'a mut T
Returns a unique reference to the value. If the value may be uninitialized,
as_uninit_mut
must be used instead.
For the shared counterpart see
as_ref
.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
mut
ptr = NonNull::new(
&mut
x).expect(
"null pointer"
);
let
x_ref =
unsafe
{ ptr.as_mut() };
assert_eq!
(
*
x_ref,
0
);
*
x_ref +=
2
;
assert_eq!
(
*
x_ref,
2
);
1.27.0 (const: 1.36.0)
·
Source
pub const fn
cast
<U>(self) ->
NonNull
<U>
Casts to a pointer of another type.
§
Examples
use
std::ptr::NonNull;
let
mut
x =
0u32
;
let
ptr = NonNull::new(
&mut
x
as
*mut
_
).expect(
"null pointer"
);
let
casted_ptr = ptr.cast::<i8>();
let
raw_ptr:
*mut
i8 = casted_ptr.as_ptr();
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
offset
(self, count:
isize
) ->
NonNull
<T>
Adds an offset to a pointer.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
If any of the following conditions are violated, the result is Undefined Behavior:
The computed offset,
count * size_of::<T>()
bytes, must not overflow
isize
.
If the computed offset is non-zero, then
self
must be derived from a pointer to some
allocated object
, and the entire memory range between
self
and the result must be in
bounds of that allocated object. In particular, this range must not “wrap around” the edge
of the address space.
Allocated objects can never be larger than
isize::MAX
bytes, so if the computed offset
stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
This implies, for instance, that
vec.as_ptr().add(vec.len())
(for
vec: Vec<T>
) is always
safe.
§
Examples
use
std::ptr::NonNull;
let
mut
s = [
1
,
2
,
3
];
let
ptr: NonNull<u32> = NonNull::new(s.as_mut_ptr()).unwrap();
unsafe
{
println!
(
"{}"
, ptr.offset(
1
).read());
println!
(
"{}"
, ptr.offset(
2
).read());
}
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
byte_offset
(self, count:
isize
) ->
NonNull
<T>
Calculates the offset from a pointer in bytes.
count
is in units of
bytes
.
This is purely a convenience for casting to a
u8
pointer and
using
offset
on it. See that method for documentation
and safety requirements.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
add
(self, count:
usize
) ->
NonNull
<T>
Adds an offset to a pointer (convenience for
.offset(count as isize)
).
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
If any of the following conditions are violated, the result is Undefined Behavior:
The computed offset,
count * size_of::<T>()
bytes, must not overflow
isize
.
If the computed offset is non-zero, then
self
must be derived from a pointer to some
allocated object
, and the entire memory range between
self
and the result must be in
bounds of that allocated object. In particular, this range must not “wrap around” the edge
of the address space.
Allocated objects can never be larger than
isize::MAX
bytes, so if the computed offset
stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
This implies, for instance, that
vec.as_ptr().add(vec.len())
(for
vec: Vec<T>
) is always
safe.
§
Examples
use
std::ptr::NonNull;
let
s:
&
str =
"123"
;
let
ptr: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap();
unsafe
{
println!
(
"{}"
, ptr.add(
1
).read()
as
char);
println!
(
"{}"
, ptr.add(
2
).read()
as
char);
}
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
byte_add
(self, count:
usize
) ->
NonNull
<T>
Calculates the offset from a pointer in bytes (convenience for
.byte_offset(count as isize)
).
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
add
on it. See that method for documentation
and safety requirements.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
sub
(self, count:
usize
) ->
NonNull
<T>
Subtracts an offset from a pointer (convenience for
.offset((count as isize).wrapping_neg())
).
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
If any of the following conditions are violated, the result is Undefined Behavior:
The computed offset,
count * size_of::<T>()
bytes, must not overflow
isize
.
If the computed offset is non-zero, then
self
must be derived from a pointer to some
allocated object
, and the entire memory range between
self
and the result must be in
bounds of that allocated object. In particular, this range must not “wrap around” the edge
of the address space.
Allocated objects can never be larger than
isize::MAX
bytes, so if the computed offset
stays in bounds of the allocated object, it is guaranteed to satisfy the first requirement.
This implies, for instance, that
vec.as_ptr().add(vec.len())
(for
vec: Vec<T>
) is always
safe.
§
Examples
use
std::ptr::NonNull;
let
s:
&
str =
"123"
;
unsafe
{
let
end: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap().add(
3
);
println!
(
"{}"
, end.sub(
1
).read()
as
char);
println!
(
"{}"
, end.sub(
2
).read()
as
char);
}
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
byte_sub
(self, count:
usize
) ->
NonNull
<T>
Calculates the offset from a pointer in bytes (convenience for
.byte_offset((count as isize).wrapping_neg())
).
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
sub
on it. See that method for documentation
and safety requirements.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
offset_from
(self, origin:
NonNull
<T>) ->
isize
Calculates the distance between two pointers within the same allocation. The returned value is in
units of T: the distance in bytes divided by
size_of::<T>()
.
This is equivalent to
(self as isize - origin as isize) / (size_of::<T>() as isize)
,
except that it has a lot more opportunities for UB, in exchange for the compiler
better understanding what you are doing.
The primary motivation of this method is for computing the
len
of an array/slice
of
T
that you are currently representing as a “start” and “end” pointer
(and “end” is “one past the end” of the array).
In that case,
end.offset_from(start)
gets you the length of the array.
All of the following safety requirements are trivially satisfied for this usecase.
§
Safety
If any of the following conditions are violated, the result is Undefined Behavior:
self
and
origin
must either
point to the same address, or
both be
derived from
a pointer to the same
allocated object
, and the memory range between
the two pointers must be in bounds of that object. (See below for an example.)
The distance between the pointers, in bytes, must be an exact multiple
of the size of
T
.
As a consequence, the absolute distance between the pointers, in bytes, computed on
mathematical integers (without “wrapping around”), cannot overflow an
isize
. This is
implied by the in-bounds requirement, and the fact that no allocated object can be larger
than
isize::MAX
bytes.
The requirement for pointers to be derived from the same allocated object is primarily
needed for
const
-compatibility: the distance between pointers into
different
allocated
objects is not known at compile-time. However, the requirement also exists at
runtime and may be exploited by optimizations. If you wish to compute the difference between
pointers that are not guaranteed to be from the same allocation, use
(self as isize - origin as isize) / size_of::<T>()
.
§
Panics
This function panics if
T
is a Zero-Sized Type (“ZST”).
§
Examples
Basic usage:
use
std::ptr::NonNull;
let
a = [
0
;
5
];
let
ptr1: NonNull<u32> = NonNull::from(
&
a[
1
]);
let
ptr2: NonNull<u32> = NonNull::from(
&
a[
3
]);
unsafe
{
assert_eq!
(ptr2.offset_from(ptr1),
2
);
assert_eq!
(ptr1.offset_from(ptr2), -
2
);
assert_eq!
(ptr1.offset(
2
), ptr2);
assert_eq!
(ptr2.offset(-
2
), ptr1);
}
Incorrect
usage:
use
std::ptr::NonNull;
let
ptr1 = NonNull::new(Box::into_raw(Box::new(
0u8
))).unwrap();
let
ptr2 = NonNull::new(Box::into_raw(Box::new(
1u8
))).unwrap();
let
diff = (ptr2.addr().get()
as
isize).wrapping_sub(ptr1.addr().get()
as
isize);
// Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
let
diff_plus_1 = diff.wrapping_add(
1
);
let
ptr2_other = NonNull::new(ptr1.as_ptr().wrapping_byte_offset(diff_plus_1)).unwrap();
assert_eq!
(ptr2.addr(), ptr2_other.addr());
// Since ptr2_other and ptr2 are derived from pointers to different objects,
// computing their offset is undefined behavior, even though
// they point to addresses that are in-bounds of the same object!
let
one =
unsafe
{ ptr2_other.offset_from(ptr2) };
// Undefined Behavior! ⚠️
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
byte_offset_from
<U>(self, origin:
NonNull
<U>) ->
isize
where
    U: ?
Sized
,
Calculates the distance between two pointers within the same allocation. The returned value is in
units of
bytes
.
This is purely a convenience for casting to a
u8
pointer and
using
offset_from
on it. See that method for
documentation and safety requirements.
For non-
Sized
pointees this operation considers only the data pointers,
ignoring the metadata.
1.87.0 (const: 1.87.0)
·
Source
pub const unsafe fn
offset_from_unsigned
(self, subtracted:
NonNull
<T>) ->
usize
Calculates the distance between two pointers within the same allocation,
where it’s known that
self
is equal to or greater than
origin
. The returned value is in
units of T: the distance in bytes is divided by
size_of::<T>()
.
This computes the same value that
offset_from
would compute, but with the added precondition that the offset is
guaranteed to be non-negative.  This method is equivalent to
usize::try_from(self.offset_from(origin)).unwrap_unchecked()
,
but it provides slightly more information to the optimizer, which can
sometimes allow it to optimize slightly better with some backends.
This method can be though of as recovering the
count
that was passed
to
add
(or, with the parameters in the other order,
to
sub
).  The following are all equivalent, assuming
that their safety preconditions are met:
ptr.offset_from_unsigned(origin) == count
origin.add(count) == ptr
ptr.sub(count) == origin
§
Safety
The distance between the pointers must be non-negative (
self >= origin
)
All
the safety conditions of
offset_from
apply to this method as well; see it for the full details.
Importantly, despite the return type of this method being able to represent
a larger offset, it’s still
not permitted
to pass pointers which differ
by more than
isize::MAX
bytes
.  As such, the result of this method will
always be less than or equal to
isize::MAX as usize
.
§
Panics
This function panics if
T
is a Zero-Sized Type (“ZST”).
§
Examples
use
std::ptr::NonNull;
let
a = [
0
;
5
];
let
ptr1: NonNull<u32> = NonNull::from(
&
a[
1
]);
let
ptr2: NonNull<u32> = NonNull::from(
&
a[
3
]);
unsafe
{
assert_eq!
(ptr2.offset_from_unsigned(ptr1),
2
);
assert_eq!
(ptr1.add(
2
), ptr2);
assert_eq!
(ptr2.sub(
2
), ptr1);
assert_eq!
(ptr2.offset_from_unsigned(ptr2),
0
);
}
// This would be incorrect, as the pointers are not correctly ordered:
// ptr1.offset_from_unsigned(ptr2)
1.87.0 (const: 1.87.0)
·
Source
pub const unsafe fn
byte_offset_from_unsigned
<U>(
    self,
    origin:
NonNull
<U>,
) ->
usize
where
    U: ?
Sized
,
Calculates the distance between two pointers within the same allocation,
where it’s known that
self
is equal to or greater than
origin
. The returned value is in
units of
bytes
.
This is purely a convenience for casting to a
u8
pointer and
using
sub_ptr
on it. See that method for
documentation and safety requirements.
For non-
Sized
pointees this operation considers only the data pointers,
ignoring the metadata.
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
read
(self) -> T
Reads the value from
self
without moving it. This leaves the
memory in
self
unchanged.
See
ptr::read
for safety concerns and examples.
1.80.0
·
Source
pub unsafe fn
read_volatile
(self) -> T
Performs a volatile read of the value from
self
without moving it. This
leaves the memory in
self
unchanged.
Volatile operations are intended to act on I/O memory, and are guaranteed
to not be elided or reordered by the compiler across other volatile
operations.
See
ptr::read_volatile
for safety concerns and examples.
1.80.0 (const: 1.80.0)
·
Source
pub const unsafe fn
read_unaligned
(self) -> T
Reads the value from
self
without moving it. This leaves the
memory in
self
unchanged.
Unlike
read
, the pointer may be unaligned.
See
ptr::read_unaligned
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to
(self, dest:
NonNull
<T>, count:
usize
)
Copies
count * size_of::<T>()
bytes from
self
to
dest
. The source
and destination may overlap.
NOTE: this has the
same
argument order as
ptr::copy
.
See
ptr::copy
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to_nonoverlapping
(self, dest:
NonNull
<T>, count:
usize
)
Copies
count * size_of::<T>()
bytes from
self
to
dest
. The source
and destination may
not
overlap.
NOTE: this has the
same
argument order as
ptr::copy_nonoverlapping
.
See
ptr::copy_nonoverlapping
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_from
(self, src:
NonNull
<T>, count:
usize
)
Copies
count * size_of::<T>()
bytes from
src
to
self
. The source
and destination may overlap.
NOTE: this has the
opposite
argument order of
ptr::copy
.
See
ptr::copy
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_from_nonoverlapping
(
    self,
    src:
NonNull
<T>,
    count:
usize
,
)
Copies
count * size_of::<T>()
bytes from
src
to
self
. The source
and destination may
not
overlap.
NOTE: this has the
opposite
argument order of
ptr::copy_nonoverlapping
.
See
ptr::copy_nonoverlapping
for safety concerns and examples.
1.80.0
·
Source
pub unsafe fn
drop_in_place
(self)
Executes the destructor (if any) of the pointed-to value.
See
ptr::drop_in_place
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
write
(self, val: T)
Overwrites a memory location with the given value without reading or
dropping the old value.
See
ptr::write
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
write_bytes
(self, val:
u8
, count:
usize
)
Invokes memset on the specified pointer, setting
count * size_of::<T>()
bytes of memory starting at
self
to
val
.
See
ptr::write_bytes
for safety concerns and examples.
1.80.0
·
Source
pub unsafe fn
write_volatile
(self, val: T)
Performs a volatile write of a memory location with the given value without
reading or dropping the old value.
Volatile operations are intended to act on I/O memory, and are guaranteed
to not be elided or reordered by the compiler across other volatile
operations.
See
ptr::write_volatile
for safety concerns and examples.
1.80.0 (const: 1.83.0)
·
Source
pub const unsafe fn
write_unaligned
(self, val: T)
Overwrites a memory location with the given value without reading or
dropping the old value.
Unlike
write
, the pointer may be unaligned.
See
ptr::write_unaligned
for safety concerns and examples.
1.80.0
·
Source
pub unsafe fn
replace
(self, src: T) -> T
Replaces the value at
self
with
src
, returning the old
value, without dropping either.
See
ptr::replace
for safety concerns and examples.
1.80.0 (const: 1.85.0)
·
Source
pub const unsafe fn
swap
(self, with:
NonNull
<T>)
Swaps the values at two mutable locations of the same type, without
deinitializing either. They may overlap, unlike
mem::swap
which is
otherwise equivalent.
See
ptr::swap
for safety concerns and examples.
1.80.0
·
Source
pub fn
align_offset
(self, align:
usize
) ->
usize
Computes the offset that needs to be applied to the pointer in order to make it aligned to
align
.
If it is not possible to align the pointer, the implementation returns
usize::MAX
.
The offset is expressed in number of
T
elements, and not bytes.
There are no guarantees whatsoever that offsetting the pointer will not overflow or go
beyond the allocation that the pointer points into. It is up to the caller to ensure that
the returned offset is correct in all terms other than alignment.
When this is called during compile-time evaluation (which is unstable), the implementation
may return
usize::MAX
in cases where that can never happen at runtime. This is because the
actual alignment of pointers is not known yet during compile-time, so an offset with
guaranteed alignment can sometimes not be computed. For example, a buffer declared as
[u8; N]
might be allocated at an odd or an even address, but at compile-time this is not yet
known, so the execution has to be correct for either choice. It is therefore impossible to
find an offset that is guaranteed to be 2-aligned. (This behavior is subject to change, as usual
for unstable APIs.)
§
Panics
The function panics if
align
is not a power-of-two.
§
Examples
Accessing adjacent
u8
as
u16
use
std::ptr::NonNull;
let
x = [
5_u8
,
6
,
7
,
8
,
9
];
let
ptr = NonNull::new(x.as_ptr()
as
*mut
u8).unwrap();
let
offset = ptr.align_offset(align_of::<u16>());
if
offset < x.len() -
1
{
let
u16_ptr = ptr.add(offset).cast::<u16>();
assert!
(u16_ptr.read() == u16::from_ne_bytes([
5
,
6
]) || u16_ptr.read() == u16::from_ne_bytes([
6
,
7
]));
}
else
{
// while the pointer can be aligned via `offset`, it would point
    // outside the allocation
}
1.79.0
·
Source
pub fn
is_aligned
(self) ->
bool
Returns whether the pointer is properly aligned for
T
.
§
Examples
use
std::ptr::NonNull;
// On some platforms, the alignment of i32 is less than 4.
#[repr(align(
4
))]
struct
AlignedI32(i32);
let
data = AlignedI32(
42
);
let
ptr = NonNull::<AlignedI32>::from(
&
data);
assert!
(ptr.is_aligned());
assert!
(!NonNull::new(ptr.as_ptr().wrapping_byte_add(
1
)).unwrap().is_aligned());
Source
pub fn
is_aligned_to
(self, align:
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
pointer_is_aligned_to
#96284
)
Returns whether the pointer is aligned to
align
.
For non-
Sized
pointees this operation considers only the data pointer,
ignoring the metadata.
§
Panics
The function panics if
align
is not a power-of-two (this includes 0).
§
Examples
#![feature(pointer_is_aligned_to)]
// On some platforms, the alignment of i32 is less than 4.
#[repr(align(
4
))]
struct
AlignedI32(i32);
let
data = AlignedI32(
42
);
let
ptr =
&
data
as
*const
AlignedI32;
assert!
(ptr.is_aligned_to(
1
));
assert!
(ptr.is_aligned_to(
2
));
assert!
(ptr.is_aligned_to(
4
));
assert!
(ptr.wrapping_byte_add(
2
).is_aligned_to(
2
));
assert!
(!ptr.wrapping_byte_add(
2
).is_aligned_to(
4
));
assert_ne!
(ptr.is_aligned_to(
8
), ptr.wrapping_add(
1
).is_aligned_to(
8
));
Source
§
impl<T>
NonNull
<
[T]
>
1.70.0 (const: 1.83.0)
·
Source
pub const fn
slice_from_raw_parts
(data:
NonNull
<T>, len:
usize
) ->
NonNull
<
[T]
>
Creates a non-null raw slice from a thin pointer and a length.
The
len
argument is the number of
elements
, not the number of bytes.
This function is safe, but dereferencing the return value is unsafe.
See the documentation of
slice::from_raw_parts
for slice safety requirements.
§
Examples
use
std::ptr::NonNull;
// create a slice pointer when starting out with a pointer to the first element
let
mut
x = [
5
,
6
,
7
];
let
nonnull_pointer = NonNull::new(x.as_mut_ptr()).unwrap();
let
slice = NonNull::slice_from_raw_parts(nonnull_pointer,
3
);
assert_eq!
(
unsafe
{ slice.as_ref()[
2
] },
7
);
(Note that this example artificially demonstrates a use of this method,
but
let slice = NonNull::from(&x[..]);
would be a better way to write code like this.)
1.63.0 (const: 1.63.0)
·
Source
pub const fn
len
(self) ->
usize
Returns the length of a non-null raw slice.
The returned value is the number of
elements
, not the number of bytes.
This function is safe, even when the non-null raw slice cannot be dereferenced to a slice
because the pointer does not have a valid address.
§
Examples
use
std::ptr::NonNull;
let
slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(),
3
);
assert_eq!
(slice.len(),
3
);
1.79.0 (const: 1.79.0)
·
Source
pub const fn
is_empty
(self) ->
bool
Returns
true
if the non-null raw slice has a length of 0.
§
Examples
use
std::ptr::NonNull;
let
slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(),
3
);
assert!
(!slice.is_empty());
Source
pub const fn
as_non_null_ptr
(self) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
slice_ptr_get
#74265
)
Returns a non-null pointer to the slice’s buffer.
§
Examples
#![feature(slice_ptr_get)]
use
std::ptr::NonNull;
let
slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(),
3
);
assert_eq!
(slice.as_non_null_ptr(), NonNull::<i8>::dangling());
Source
pub const fn
as_mut_ptr
(self) ->
*mut T
🔬
This is a nightly-only experimental API. (
slice_ptr_get
#74265
)
Returns a raw pointer to the slice’s buffer.
§
Examples
#![feature(slice_ptr_get)]
use
std::ptr::NonNull;
let
slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(),
3
);
assert_eq!
(slice.as_mut_ptr(), NonNull::<i8>::dangling().as_ptr());
Source
pub const unsafe fn
as_uninit_slice
<'a>(self) -> &'a [
MaybeUninit
<T>]
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns a shared reference to a slice of possibly uninitialized values. In contrast to
as_ref
, this does not require that the value has to be initialized.
For the mutable counterpart see
as_uninit_slice_mut
.
§
Safety
When calling this method, you have to ensure that all of the following is true:
The pointer must be
valid
for reads for
ptr.len() * size_of::<T>()
many bytes,
and it must be properly aligned. This means in particular:
The entire memory range of this slice must be contained within a single allocated object!
Slices can never span across multiple allocated objects.
The pointer must be aligned even for zero-length slices. One
reason for this is that enum layout optimizations may rely on references
(including slices of any length) being aligned and non-null to distinguish
them from other data. You can obtain a pointer that is usable as
data
for zero-length slices using
NonNull::dangling()
.
The total size
ptr.len() * size_of::<T>()
of the slice must be no larger than
isize::MAX
.
See the safety documentation of
pointer::offset
.
You must enforce Rust’s aliasing rules, since the returned lifetime
'a
is
arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
In particular, while this reference exists, the memory the pointer points to must
not get mutated (except inside
UnsafeCell
).
This applies even if the result of this method is unused!
See also
slice::from_raw_parts
.
Source
pub const unsafe fn
as_uninit_slice_mut
<'a>(self) -> &'a mut [
MaybeUninit
<T>]
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns a unique reference to a slice of possibly uninitialized values. In contrast to
as_mut
, this does not require that the value has to be initialized.
For the shared counterpart see
as_uninit_slice
.
§
Safety
When calling this method, you have to ensure that all of the following is true:
The pointer must be
valid
for reads and writes for
ptr.len() * size_of::<T>()
many bytes, and it must be properly aligned. This means in particular:
The entire memory range of this slice must be contained within a single allocated object!
Slices can never span across multiple allocated objects.
The pointer must be aligned even for zero-length slices. One
reason for this is that enum layout optimizations may rely on references
(including slices of any length) being aligned and non-null to distinguish
them from other data. You can obtain a pointer that is usable as
data
for zero-length slices using
NonNull::dangling()
.
The total size
ptr.len() * size_of::<T>()
of the slice must be no larger than
isize::MAX
.
See the safety documentation of
pointer::offset
.
You must enforce Rust’s aliasing rules, since the returned lifetime
'a
is
arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
In particular, while this reference exists, the memory the pointer points to must
not get accessed (read or written) through any other pointer.
This applies even if the result of this method is unused!
See also
slice::from_raw_parts_mut
.
§
Examples
#![feature(allocator_api, ptr_as_uninit)]
use
std::alloc::{Allocator, Layout, Global};
use
std::mem::MaybeUninit;
use
std::ptr::NonNull;
let
memory: NonNull<[u8]> = Global.allocate(Layout::new::<[u8;
32
]>())
?
;
// This is safe as `memory` is valid for reads and writes for `memory.len()` many bytes.
// Note that calling `memory.as_mut()` is not allowed here as the content may be uninitialized.
let
slice:
&mut
[MaybeUninit<u8>] =
unsafe
{ memory.as_uninit_slice_mut() };
Source
pub unsafe fn
get_unchecked_mut
<I>(
    self,
    index: I,
) ->
NonNull
<<I as
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
🔬
This is a nightly-only experimental API. (
slice_ptr_get
#74265
)
Returns a raw pointer to an element or subslice, without doing bounds
checking.
Calling this method with an out-of-bounds index or when
self
is not dereferenceable
is
undefined behavior
even if the resulting pointer is not used.
§
Examples
#![feature(slice_ptr_get)]
use
std::ptr::NonNull;
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
x = NonNull::slice_from_raw_parts(NonNull::new(x.as_mut_ptr()).unwrap(), x.len());
unsafe
{
assert_eq!
(x.get_unchecked_mut(
1
).as_ptr(), x.as_non_null_ptr().as_ptr().add(
1
));
}
Trait Implementations
§
1.25.0
·
Source
§
impl<T>
Clone
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
clone
(&self) ->
NonNull
<T>
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
1.25.0
·
Source
§
impl<T>
Debug
for
NonNull
<T>
where
    T: ?
Sized
,
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
1.25.0
·
Source
§
impl<T>
From
<
&T
> for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
from
(r:
&T
) ->
NonNull
<T>
Converts a
&T
to a
NonNull<T>
.
This conversion is safe and infallible since references cannot be null.
1.25.0
·
Source
§
impl<T>
From
<
&mut T
> for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
from
(r:
&mut T
) ->
NonNull
<T>
Converts a
&mut T
to a
NonNull<T>
.
This conversion is safe and infallible since references cannot be null.
1.25.0
·
Source
§
impl<T>
Hash
for
NonNull
<T>
where
    T: ?
Sized
,
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
1.25.0
·
Source
§
impl<T>
Ord
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
cmp
(&self, other: &
NonNull
<T>) ->
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
1.25.0
·
Source
§
impl<T>
PartialEq
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
eq
(&self, other: &
NonNull
<T>) ->
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
1.25.0
·
Source
§
impl<T>
PartialOrd
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
NonNull
<T>) ->
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
1.25.0
·
Source
§
impl<T>
Pointer
for
NonNull
<T>
where
    T: ?
Sized
,
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
impl<T, U>
CoerceUnsized
<
NonNull
<U>> for
NonNull
<T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
1.25.0
·
Source
§
impl<T>
Copy
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
impl<T, U>
DispatchFromDyn
<
NonNull
<U>> for
NonNull
<T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
1.25.0
·
Source
§
impl<T>
Eq
for
NonNull
<T>
where
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
NonNull
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PointerLike
for
NonNull
<T>
1.25.0
·
Source
§
impl<T> !
Send
for
NonNull
<T>
where
    T: ?
Sized
,
NonNull
pointers are not
Send
because the data they reference may be aliased.
1.25.0
·
Source
§
impl<T> !
Sync
for
NonNull
<T>
where
    T: ?
Sized
,
NonNull
pointers are not
Sync
because the data they reference may be aliased.
1.25.0
·
Source
§
impl<T>
UnwindSafe
for
NonNull
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
Auto Trait Implementations
§
§
impl<T>
Freeze
for
NonNull
<T>
where
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
NonNull
<T>
where
    T:
RefUnwindSafe
+ ?
Sized
,
§
impl<T>
Unpin
for
NonNull
<T>
where
    T: ?
Sized
,
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