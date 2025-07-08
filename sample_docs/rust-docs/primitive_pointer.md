pointer - Rust
Primitive Type
pointer
Copy item path
1.0.0
Expand description
Raw, unsafe pointers,
*const T
, and
*mut T
.
See also the
std::ptr
module
.
Working with raw pointers in Rust is uncommon, typically limited to a few patterns. Raw pointers
can be out-of-bounds, unaligned, or
null
. However, when loading from or storing to a raw
pointer, it must be
valid
for the given access and aligned. When using a field expression,
tuple index expression, or array/slice index expression on a raw pointer, it follows the rules
of
in-bounds pointer arithmetic
.
Storing through a raw pointer using
*ptr = data
calls
drop
on the old value, so
write
must be used if the type has drop glue and memory is not already
initialized - otherwise
drop
would be called on the uninitialized memory.
Use the
null
and
null_mut
functions to create null pointers, and the
is_null
method of the
*const T
and
*mut T
types to check for null.
The
*const T
and
*mut T
types also define the
offset
method, for
pointer math.
§
Common ways to create raw pointers
§
1. Coerce a reference (
&T
) or mutable reference (
&mut T
).
let
my_num: i32 =
10
;
let
my_num_ptr:
*const
i32 =
&
my_num;
let
mut
my_speed: i32 =
88
;
let
my_speed_ptr:
*mut
i32 =
&mut
my_speed;
To get a pointer to a boxed value, dereference the box:
let
my_num: Box<i32> = Box::new(
10
);
let
my_num_ptr:
*const
i32 =
&*
my_num;
let
mut
my_speed: Box<i32> = Box::new(
88
);
let
my_speed_ptr:
*mut
i32 =
&mut *
my_speed;
This does not take ownership of the original allocation
and requires no resource management later,
but you must not use the pointer after its lifetime.
§
2. Consume a box (
Box<T>
).
The
into_raw
function consumes a box and returns
the raw pointer. It doesn’t destroy
T
or deallocate any memory.
let
my_speed: Box<i32> = Box::new(
88
);
let
my_speed:
*mut
i32 = Box::into_raw(my_speed);
// By taking ownership of the original `Box<T>` though
// we are obligated to put it together later to be destroyed.
unsafe
{
    drop(Box::from_raw(my_speed));
}
Note that here the call to
drop
is for clarity - it indicates
that we are done with the given value and it should be destroyed.
§
3. Create it using
&raw
Instead of coercing a reference to a raw pointer, you can use the raw borrow
operators
&raw const
(for
*const T
) and
&raw mut
(for
*mut T
).
These operators allow you to create raw pointers to fields to which you cannot
create a reference (without causing undefined behavior), such as an
unaligned field. This might be necessary if packed structs or uninitialized
memory is involved.
#[derive(Debug, Default, Copy, Clone)]
#[repr(C, packed)]
struct
S {
    aligned: u8,
    unaligned: u32,
}
let
s = S::default();
let
p =
&
raw
const
s.unaligned;
// not allowed with coercion
§
4. Get it from C.
#[allow(unused_extern_crates)]
extern crate
libc;
unsafe
{
let
my_num:
*mut
i32 = libc::malloc(size_of::<i32>())
as
*mut
i32;
if
my_num.is_null() {
panic!
(
"failed to allocate memory"
);
    }
    libc::free(my_num
as
*mut
core::ffi::c_void);
}
Usually you wouldn’t literally use
malloc
and
free
from Rust,
but C APIs hand out a lot of pointers generally, so are a common source
of raw pointers in Rust.
Implementations
§
Source
§
impl<T>
*const T
where
    T: ?
Sized
,
1.0.0 (const: 1.84.0)
·
Source
pub const fn
is_null
(self) ->
bool
Returns
true
if the pointer is null.
Note that unsized types have many possible null pointers, as only the
raw data pointer is considered, not their length, vtable, etc.
Therefore, two pointers that are null may still not compare equal to
each other.
§
Panics during const evaluation
If this method is used during const evaluation, and
self
is a pointer
that is offset beyond the bounds of the memory it initially pointed to,
then there might not be enough information to determine whether the
pointer is null. This is because the absolute address in memory is not
known at compile time. If the nullness of the pointer cannot be
determined, this method will panic.
In-bounds pointers are never null, so the method will never panic for
such pointers.
§
Examples
let
s:
&
str =
"Follow the rabbit"
;
let
ptr:
*const
u8 = s.as_ptr();
assert!
(!ptr.is_null());
1.38.0 (const: 1.38.0)
·
Source
pub const fn
cast
<U>(self) ->
*const U
Casts to a pointer of another type.
Source
pub const fn
with_metadata_of
<U>(self, meta:
*const U
) ->
*const U
where
    U: ?
Sized
,
🔬
This is a nightly-only experimental API. (
set_ptr_value
#75091
)
Uses the address value in a new pointer of another type.
This operation will ignore the address part of its
meta
operand and discard existing
metadata of
self
. For pointers to a sized types (thin pointers), this has the same effect
as a simple cast. For pointers to an unsized type (fat pointers) this recombines the address
with new metadata such as slice lengths or
dyn
-vtable.
The resulting pointer will have provenance of
self
. This operation is semantically the
same as creating a new pointer with the data pointer value of
self
but the metadata of
meta
, being fat or thin depending on the
meta
operand.
§
Examples
This function is primarily useful for enabling pointer arithmetic on potentially fat
pointers. The pointer is cast to a sized pointee to utilize offset operations and then
recombined with its own original metadata.
#![feature(set_ptr_value)]
let
arr: [i32;
3
] = [
1
,
2
,
3
];
let
mut
ptr = arr.as_ptr()
as
*const
dyn
Debug;
let
thin = ptr
as
*const
u8;
unsafe
{
    ptr = thin.add(
8
).with_metadata_of(ptr);
println!
(
"{:?}"
,
&*
ptr);
// will print "3"
}
§
Incorrect
usage
The provenance from pointers is
not
combined. The result must only be used to refer to the
address allowed by
self
.
#![feature(set_ptr_value)]
let
x =
0u32
;
let
y =
1u32
;
let
x = (
&
x)
as
*const
u32;
let
y = (
&
y)
as
*const
u32;
let
offset = (x
as
usize - y
as
usize) /
4
;
let
bad = x.wrapping_add(offset).with_metadata_of(y);
// This dereference is UB. The pointer only has provenance for `x` but points to `y`.
println!
(
"{:?}"
,
unsafe
{
&*
bad });
1.65.0 (const: 1.65.0)
·
Source
pub const fn
cast_mut
(self) ->
*mut T
Changes constness without changing the type.
This is a bit safer than
as
because it wouldn’t silently change the type if the code is
refactored.
1.84.0
·
Source
pub fn
addr
(self) ->
usize
Gets the “address” portion of the pointer.
This is similar to
self as usize
, except that the
provenance
of
the pointer is discarded and not
exposed
. This means that
casting the returned address back to a pointer yields a
pointer without
provenance
, which is undefined behavior to dereference. To properly
restore the lost information and obtain a dereferenceable pointer, use
with_addr
or
map_addr
.
If using those APIs is not possible because there is no way to preserve a pointer with the
required provenance, then Strict Provenance might not be for you. Use pointer-integer casts
or
expose_provenance
and
with_exposed_provenance
instead. However, note that this makes your code less portable and less amenable to tools
that check for compliance with the Rust memory model.
On most platforms this will produce a value with the same bytes as the original
pointer, because all the bytes are dedicated to describing the address.
Platforms which need to store additional information in the pointer may
perform a change of representation to produce a value containing only the address
portion of the pointer. What that means is up to the platform to define.
This is a
Strict Provenance
API.
1.84.0
·
Source
pub fn
expose_provenance
(self) ->
usize
Exposes the
“provenance”
part of the pointer for future use in
with_exposed_provenance
and returns the “address” portion.
This is equivalent to
self as usize
, which semantically discards provenance information.
Furthermore, this (like the
as
cast) has the implicit side-effect of marking the
provenance as ‘exposed’, so on platforms that support it you can later call
with_exposed_provenance
to reconstitute the original pointer including its provenance.
Due to its inherent ambiguity,
with_exposed_provenance
may not be supported by tools
that help you to stay conformant with the Rust memory model. It is recommended to use
Strict Provenance
APIs such as
with_addr
wherever possible, in which case
addr
should be used instead of
expose_provenance
.
On most platforms this will produce a value with the same bytes as the original pointer,
because all the bytes are dedicated to describing the address. Platforms which need to store
additional information in the pointer may not support this operation, since the ‘expose’
side-effect which is required for
with_exposed_provenance
to work is typically not
available.
This is an
Exposed Provenance
API.
1.84.0
·
Source
pub fn
with_addr
(self, addr:
usize
) ->
*const T
Creates a new pointer with the given address and the
provenance
of
self
.
This is similar to a
addr as *const T
cast, but copies
the
provenance
of
self
to the new pointer.
This avoids the inherent ambiguity of the unary cast.
This is equivalent to using
wrapping_offset
to offset
self
to the given address, and therefore has all the same capabilities and restrictions.
This is a
Strict Provenance
API.
1.84.0
·
Source
pub fn
map_addr
(self, f: impl
FnOnce
(
usize
) ->
usize
) ->
*const T
Creates a new pointer by mapping
self
’s address to a new one, preserving the
provenance
of
self
.
This is a convenience for
with_addr
, see that method for details.
This is a
Strict Provenance
API.
Source
pub const fn
to_raw_parts
(self) -> (
*const
()
, <T as
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
from_raw_parts
.
1.9.0 (const: 1.84.0)
·
Source
pub const unsafe fn
as_ref
<'a>(self) ->
Option
<
&'a T
>
Returns
None
if the pointer is null, or else returns a shared reference to
the value wrapped in
Some
. If the value may be uninitialized,
as_uninit_ref
must be used instead.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
let
ptr:
*const
u8 =
&
10u8
as
*const
u8;
unsafe
{
if let
Some
(val_back) = ptr.as_ref() {
assert_eq!
(val_back,
&
10
);
    }
}
§
Null-unchecked version
If you are sure the pointer can never be null and are looking for some kind of
as_ref_unchecked
that returns the
&T
instead of
Option<&T>
, know that you can
dereference the pointer directly.
let
ptr:
*const
u8 =
&
10u8
as
*const
u8;
unsafe
{
let
val_back =
&*
ptr;
assert_eq!
(val_back,
&
10
);
}
Source
pub const unsafe fn
as_ref_unchecked
<'a>(self) ->
&'a T
🔬
This is a nightly-only experimental API. (
ptr_as_ref_unchecked
#122034
)
Returns a shared reference to the value behind the pointer.
If the pointer may be null or the value may be uninitialized,
as_uninit_ref
must be used instead.
If the pointer may be null, but the value is known to have been initialized,
as_ref
must be used instead.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
§
Examples
#![feature(ptr_as_ref_unchecked)]
let
ptr:
*const
u8 =
&
10u8
as
*const
u8;
unsafe
{
assert_eq!
(ptr.as_ref_unchecked(),
&
10
);
}
Source
pub const unsafe fn
as_uninit_ref
<'a>(self) ->
Option
<&'a
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a shared reference to
the value wrapped in
Some
. In contrast to
as_ref
, this does not require
that the value has to be initialized.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
#![feature(ptr_as_uninit)]
let
ptr:
*const
u8 =
&
10u8
as
*const
u8;
unsafe
{
if let
Some
(val_back) = ptr.as_uninit_ref() {
assert_eq!
(val_back.assume_init(),
10
);
    }
}
1.0.0 (const: 1.61.0)
·
Source
pub const unsafe fn
offset
(self, count:
isize
) ->
*const T
Adds a signed offset to a pointer.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
allocated object
, and the entire memory range between
self
and the result must be in
bounds of that allocated object. In particular, this range must not “wrap around” the edge
of the address space. Note that “range” here refers to a half-open range as usual in Rust,
i.e.,
self..result
for non-negative offsets and
result..self
for negative offsets.
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
Consider using
wrapping_offset
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
let
s:
&
str =
"123"
;
let
ptr:
*const
u8 = s.as_ptr();
unsafe
{
assert_eq!
(
*
ptr.offset(
1
)
as
char,
'2'
);
assert_eq!
(
*
ptr.offset(
2
)
as
char,
'3'
);
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_offset
(self, count:
isize
) ->
*const T
Adds a signed offset in bytes to a pointer.
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
1.16.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_offset
(self, count:
isize
) ->
*const T
Adds a signed offset to a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_offset((y as isize) - (x as isize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
offset
, this method basically delays the requirement of staying within the
same allocated object:
offset
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_offset
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
offset
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_offset(o).wrapping_offset(o.wrapping_neg())
is always the same as
x
. In other
words, leaving the allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements
let
data = [
1u8
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
mut
ptr:
*const
u8 = data.as_ptr();
let
step =
2
;
let
end_rounded_up = ptr.wrapping_offset(
6
);
let
mut
out = String::new();
while
ptr != end_rounded_up {
unsafe
{
write!
(
&mut
out,
"{}, "
,
*
ptr)
?
;
    }
    ptr = ptr.wrapping_offset(step);
}
assert_eq!
(out.as_str(),
"1, 3, 5, "
);
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_offset
(self, count:
isize
) ->
*const T
Adds a signed offset in bytes to a pointer using wrapping arithmetic.
count
is in units of
bytes
.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_offset
on it. See that method
for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
Source
pub fn
mask
(self, mask:
usize
) ->
*const T
🔬
This is a nightly-only experimental API. (
ptr_mask
#98290
)
Masks out bits of the pointer according to a mask.
This is convenience for
ptr.map_addr(|a| a & mask)
.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
§
Examples
#![feature(ptr_mask)]
let
v =
17_u32
;
let
ptr:
*const
u32 =
&
v;
// `u32` is 4 bytes aligned,
// which means that lower 2 bits are always 0.
let
tag_mask =
0b11
;
let
ptr_mask = !tag_mask;
// We can store something in these lower bits
let
tagged_ptr = ptr.map_addr(|a| a |
0b10
);
// Get the "tag" back
let
tag = tagged_ptr.addr() & tag_mask;
assert_eq!
(tag,
0b10
);
// Note that `tagged_ptr` is unaligned, it's UB to read from it.
// To get original pointer `mask` can be used:
let
masked_ptr = tagged_ptr.mask(ptr_mask);
assert_eq!
(
unsafe
{
*
masked_ptr },
17
);
1.47.0 (const: 1.65.0)
·
Source
pub const unsafe fn
offset_from
(self, origin:
*const T
) ->
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
let
a = [
0
;
5
];
let
ptr1:
*const
i32 =
&
a[
1
];
let
ptr2:
*const
i32 =
&
a[
3
];
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
let
ptr1 = Box::into_raw(Box::new(
0u8
))
as
*const
u8;
let
ptr2 = Box::into_raw(Box::new(
1u8
))
as
*const
u8;
let
diff = (ptr2
as
isize).wrapping_sub(ptr1
as
isize);
// Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
let
ptr2_other = (ptr1
as
*const
u8).wrapping_offset(diff).wrapping_offset(
1
);
assert_eq!
(ptr2
as
usize, ptr2_other
as
usize);
// Since ptr2_other and ptr2 are derived from pointers to different objects,
// computing their offset is undefined behavior, even though
// they point to addresses that are in-bounds of the same object!
unsafe
{
let
one = ptr2_other.offset_from(ptr2);
// Undefined Behavior! ⚠️
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_offset_from
<U>(self, origin:
*const U
) ->
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
(self, origin:
*const T
) ->
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
This method can be thought of as recovering the
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
let
a = [
0
;
5
];
let
ptr1:
*const
i32 =
&
a[
1
];
let
ptr2:
*const
i32 =
&
a[
3
];
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
*const U
,
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
Source
pub const fn
guaranteed_eq
(self, other:
*const T
) ->
Option
<
bool
>
🔬
This is a nightly-only experimental API. (
const_raw_ptr_comparison
#53020
)
Returns whether two pointers are guaranteed to be equal.
At runtime this function behaves like
Some(self == other)
.
However, in some contexts (e.g., compile-time evaluation),
it is not always possible to determine equality of two pointers, so this function may
spuriously return
None
for pointers that later actually turn out to have its equality known.
But when it returns
Some
, the pointers’ equality is guaranteed to be known.
The return value may change from
Some
to
None
and vice versa depending on the compiler
version and unsafe code must not
rely on the result of this function for soundness. It is suggested to only use this function
for performance optimizations where spurious
None
return values by this function do not
affect the outcome, but just the performance.
The consequences of using this method to make runtime and compile-time code behave
differently have not been explored. This method should not be used to introduce such
differences, and it should also not be stabilized before we have a better understanding
of this issue.
Source
pub const fn
guaranteed_ne
(self, other:
*const T
) ->
Option
<
bool
>
🔬
This is a nightly-only experimental API. (
const_raw_ptr_comparison
#53020
)
Returns whether two pointers are guaranteed to be inequal.
At runtime this function behaves like
Some(self != other)
.
However, in some contexts (e.g., compile-time evaluation),
it is not always possible to determine inequality of two pointers, so this function may
spuriously return
None
for pointers that later actually turn out to have its inequality known.
But when it returns
Some
, the pointers’ inequality is guaranteed to be known.
The return value may change from
Some
to
None
and vice versa depending on the compiler
version and unsafe code must not
rely on the result of this function for soundness. It is suggested to only use this function
for performance optimizations where spurious
None
return values by this function do not
affect the outcome, but just the performance.
The consequences of using this method to make runtime and compile-time code behave
differently have not been explored. This method should not be used to introduce such
differences, and it should also not be stabilized before we have a better understanding
of this issue.
1.26.0 (const: 1.61.0)
·
Source
pub const unsafe fn
add
(self, count:
usize
) ->
*const T
Adds an unsigned offset to a pointer.
This can only move the pointer forward (or not move it). If you need to move forward or
backward depending on the value, then you might want
offset
instead
which takes a signed offset.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
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
Consider using
wrapping_add
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
let
s:
&
str =
"123"
;
let
ptr:
*const
u8 = s.as_ptr();
unsafe
{
assert_eq!
(
*
ptr.add(
1
),
b'2'
);
assert_eq!
(
*
ptr.add(
2
),
b'3'
);
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_add
(self, count:
usize
) ->
*const T
Adds an unsigned offset in bytes to a pointer.
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
1.26.0 (const: 1.61.0)
·
Source
pub const unsafe fn
sub
(self, count:
usize
) ->
*const T
Subtracts an unsigned offset from a pointer.
This can only move the pointer backward (or not move it). If you need to move forward or
backward depending on the value, then you might want
offset
instead
which takes a signed offset.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
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
Consider using
wrapping_sub
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
let
s:
&
str =
"123"
;
unsafe
{
let
end:
*const
u8 = s.as_ptr().add(
3
);
assert_eq!
(
*
end.sub(
1
),
b'3'
);
assert_eq!
(
*
end.sub(
2
),
b'2'
);
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_sub
(self, count:
usize
) ->
*const T
Subtracts an unsigned offset in bytes from a pointer.
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
1.26.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_add
(self, count:
usize
) ->
*const T
Adds an unsigned offset to a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_add((y as usize) - (x as usize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
add
, this method basically delays the requirement of staying within the
same allocated object:
add
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_add
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
add
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_add(o).wrapping_sub(o)
is always the same as
x
. In other words, leaving the
allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements
let
data = [
1u8
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
mut
ptr:
*const
u8 = data.as_ptr();
let
step =
2
;
let
end_rounded_up = ptr.wrapping_add(
6
);
let
mut
out = String::new();
while
ptr != end_rounded_up {
unsafe
{
write!
(
&mut
out,
"{}, "
,
*
ptr)
?
;
    }
    ptr = ptr.wrapping_add(step);
}
assert_eq!
(out,
"1, 3, 5, "
);
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_add
(self, count:
usize
) ->
*const T
Adds an unsigned offset in bytes to a pointer using wrapping arithmetic.
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_add
on it. See that method for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.26.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_sub
(self, count:
usize
) ->
*const T
Subtracts an unsigned offset from a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_sub((x as usize) - (y as usize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
sub
, this method basically delays the requirement of staying within the
same allocated object:
sub
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_sub
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
sub
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_add(o).wrapping_sub(o)
is always the same as
x
. In other words, leaving the
allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements (backwards)
let
data = [
1u8
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
mut
ptr:
*const
u8 = data.as_ptr();
let
start_rounded_down = ptr.wrapping_sub(
2
);
ptr = ptr.wrapping_add(
4
);
let
step =
2
;
let
mut
out = String::new();
while
ptr != start_rounded_down {
unsafe
{
write!
(
&mut
out,
"{}, "
,
*
ptr)
?
;
    }
    ptr = ptr.wrapping_sub(step);
}
assert_eq!
(out,
"5, 3, 1, "
);
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_sub
(self, count:
usize
) ->
*const T
Subtracts an unsigned offset in bytes from a pointer using wrapping arithmetic.
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_sub
on it. See that method for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.26.0 (const: 1.71.0)
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
1.26.0
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
1.26.0 (const: 1.71.0)
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to
(self, dest:
*mut T
, count:
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to_nonoverlapping
(self, dest:
*mut T
, count:
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
1.36.0
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
elements, and not bytes. The value returned can be
used with the
wrapping_add
method.
There are no guarantees whatsoever that offsetting the pointer will not overflow or go
beyond the allocation that the pointer points into. It is up to the caller to ensure that
the returned offset is correct in all terms other than alignment.
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
ptr = x.as_ptr();
let
offset = ptr.align_offset(align_of::<u16>());
if
offset < x.len() -
1
{
let
u16_ptr = ptr.add(offset).cast::<u16>();
assert!
(
*
u16_ptr == u16::from_ne_bytes([
5
,
6
]) ||
*
u16_ptr == u16::from_ne_bytes([
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
(ptr.is_aligned());
assert!
(!ptr.wrapping_byte_add(
1
).is_aligned());
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
*const
[T]
1.79.0 (const: 1.79.0)
·
Source
pub const fn
len
(self) ->
usize
Returns the length of a raw slice.
The returned value is the number of
elements
, not the number of bytes.
This function is safe, even when the raw slice cannot be cast to a slice
reference because the pointer is null or unaligned.
§
Examples
use
std::ptr;
let
slice:
*const
[i8] = ptr::slice_from_raw_parts(ptr::null(),
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
if the raw slice has a length of 0.
§
Examples
use
std::ptr;
let
slice:
*const
[i8] = ptr::slice_from_raw_parts(ptr::null(),
3
);
assert!
(!slice.is_empty());
Source
pub const fn
as_ptr
(self) ->
*const T
🔬
This is a nightly-only experimental API. (
slice_ptr_get
#74265
)
Returns a raw pointer to the slice’s buffer.
This is equivalent to casting
self
to
*const T
, but more type-safe.
§
Examples
#![feature(slice_ptr_get)]
use
std::ptr;
let
slice:
*const
[i8] = ptr::slice_from_raw_parts(ptr::null(),
3
);
assert_eq!
(slice.as_ptr(), ptr::null());
Source
pub const fn
as_array
<const N:
usize
>(self) ->
Option
<
*const
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a raw pointer to the underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
Source
pub unsafe fn
get_unchecked
<I>(
    self,
    index: I,
) ->
*const
<I as
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
let
x =
&
[
1
,
2
,
4
]
as
*const
[i32];
unsafe
{
assert_eq!
(x.get_unchecked(
1
), x.as_ptr().add(
1
));
}
Source
pub const unsafe fn
as_uninit_slice
<'a>(self) ->
Option
<&'a [
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a shared slice to
the value wrapped in
Some
. In contrast to
as_ref
, this does not require
that the value has to be initialized.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
all of the following is true:
The pointer must be
valid
for reads for
ptr.len() * size_of::<T>()
many bytes,
and it must be properly aligned. This means in particular:
The entire memory range of this slice must be contained within a single
allocated object
!
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
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
Source
§
impl<T, const N:
usize
>
*const
[T; N]
Source
pub const fn
as_ptr
(self) ->
*const T
🔬
This is a nightly-only experimental API. (
array_ptr_get
#119834
)
Returns a raw pointer to the array’s buffer.
This is equivalent to casting
self
to
*const T
, but more type-safe.
§
Examples
#![feature(array_ptr_get)]
use
std::ptr;
let
arr:
*const
[i8;
3
] = ptr::null();
assert_eq!
(arr.as_ptr(), ptr::null());
Source
pub const fn
as_slice
(self) ->
*const
[T]
🔬
This is a nightly-only experimental API. (
array_ptr_get
#119834
)
Returns a raw pointer to a slice containing the entire array.
§
Examples
#![feature(array_ptr_get)]
let
arr:
*const
[i32;
3
] =
&
[
1
,
2
,
4
]
as
*const
[i32;
3
];
let
slice:
*const
[i32] = arr.as_slice();
assert_eq!
(slice.len(),
3
);
Source
§
impl<T>
*mut T
where
    T: ?
Sized
,
1.0.0 (const: 1.84.0)
·
Source
pub const fn
is_null
(self) ->
bool
Returns
true
if the pointer is null.
Note that unsized types have many possible null pointers, as only the
raw data pointer is considered, not their length, vtable, etc.
Therefore, two pointers that are null may still not compare equal to
each other.
§
Panics during const evaluation
If this method is used during const evaluation, and
self
is a pointer
that is offset beyond the bounds of the memory it initially pointed to,
then there might not be enough information to determine whether the
pointer is null. This is because the absolute address in memory is not
known at compile time. If the nullness of the pointer cannot be
determined, this method will panic.
In-bounds pointers are never null, so the method will never panic for
such pointers.
§
Examples
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
ptr:
*mut
u32 = s.as_mut_ptr();
assert!
(!ptr.is_null());
1.38.0 (const: 1.38.0)
·
Source
pub const fn
cast
<U>(self) ->
*mut U
Casts to a pointer of another type.
Source
pub const fn
with_metadata_of
<U>(self, meta:
*const U
) ->
*mut U
where
    U: ?
Sized
,
🔬
This is a nightly-only experimental API. (
set_ptr_value
#75091
)
Uses the address value in a new pointer of another type.
This operation will ignore the address part of its
meta
operand and discard existing
metadata of
self
. For pointers to a sized types (thin pointers), this has the same effect
as a simple cast. For pointers to an unsized type (fat pointers) this recombines the address
with new metadata such as slice lengths or
dyn
-vtable.
The resulting pointer will have provenance of
self
. This operation is semantically the
same as creating a new pointer with the data pointer value of
self
but the metadata of
meta
, being fat or thin depending on the
meta
operand.
§
Examples
This function is primarily useful for enabling pointer arithmetic on potentially fat
pointers. The pointer is cast to a sized pointee to utilize offset operations and then
recombined with its own original metadata.
#![feature(set_ptr_value)]
let
mut
arr: [i32;
3
] = [
1
,
2
,
3
];
let
mut
ptr = arr.as_mut_ptr()
as
*mut
dyn
Debug;
let
thin = ptr
as
*mut
u8;
unsafe
{
    ptr = thin.add(
8
).with_metadata_of(ptr);
println!
(
"{:?}"
,
&*
ptr);
// will print "3"
}
§
Incorrect
usage
The provenance from pointers is
not
combined. The result must only be used to refer to the
address allowed by
self
.
#![feature(set_ptr_value)]
let
mut
x =
0u32
;
let
mut
y =
1u32
;
let
x = (
&mut
x)
as
*mut
u32;
let
y = (
&mut
y)
as
*mut
u32;
let
offset = (x
as
usize - y
as
usize) /
4
;
let
bad = x.wrapping_add(offset).with_metadata_of(y);
// This dereference is UB. The pointer only has provenance for `x` but points to `y`.
println!
(
"{:?}"
,
unsafe
{
&*
bad });
1.65.0 (const: 1.65.0)
·
Source
pub const fn
cast_const
(self) ->
*const T
Changes constness without changing the type.
This is a bit safer than
as
because it wouldn’t silently change the type if the code is
refactored.
While not strictly required (
*mut T
coerces to
*const T
), this is provided for symmetry
with
cast_mut
on
*const T
and may have documentation value if used instead of implicit
coercion.
1.84.0
·
Source
pub fn
addr
(self) ->
usize
Gets the “address” portion of the pointer.
This is similar to
self as usize
, except that the
provenance
of
the pointer is discarded and not
exposed
. This means that
casting the returned address back to a pointer yields a
pointer without
provenance
, which is undefined behavior to dereference. To properly
restore the lost information and obtain a dereferenceable pointer, use
with_addr
or
map_addr
.
If using those APIs is not possible because there is no way to preserve a pointer with the
required provenance, then Strict Provenance might not be for you. Use pointer-integer casts
or
expose_provenance
and
with_exposed_provenance
instead. However, note that this makes your code less portable and less amenable to tools
that check for compliance with the Rust memory model.
On most platforms this will produce a value with the same bytes as the original
pointer, because all the bytes are dedicated to describing the address.
Platforms which need to store additional information in the pointer may
perform a change of representation to produce a value containing only the address
portion of the pointer. What that means is up to the platform to define.
This is a
Strict Provenance
API.
1.84.0
·
Source
pub fn
expose_provenance
(self) ->
usize
Exposes the
“provenance”
part of the pointer for future use in
with_exposed_provenance_mut
and returns the “address” portion.
This is equivalent to
self as usize
, which semantically discards provenance information.
Furthermore, this (like the
as
cast) has the implicit side-effect of marking the
provenance as ‘exposed’, so on platforms that support it you can later call
with_exposed_provenance_mut
to reconstitute the original pointer including its provenance.
Due to its inherent ambiguity,
with_exposed_provenance_mut
may not be supported by tools
that help you to stay conformant with the Rust memory model. It is recommended to use
Strict Provenance
APIs such as
with_addr
wherever possible, in which case
addr
should be used instead of
expose_provenance
.
On most platforms this will produce a value with the same bytes as the original pointer,
because all the bytes are dedicated to describing the address. Platforms which need to store
additional information in the pointer may not support this operation, since the ‘expose’
side-effect which is required for
with_exposed_provenance_mut
to work is typically not
available.
This is an
Exposed Provenance
API.
1.84.0
·
Source
pub fn
with_addr
(self, addr:
usize
) ->
*mut T
Creates a new pointer with the given address and the
provenance
of
self
.
This is similar to a
addr as *mut T
cast, but copies
the
provenance
of
self
to the new pointer.
This avoids the inherent ambiguity of the unary cast.
This is equivalent to using
wrapping_offset
to offset
self
to the given address, and therefore has all the same capabilities and restrictions.
This is a
Strict Provenance
API.
1.84.0
·
Source
pub fn
map_addr
(self, f: impl
FnOnce
(
usize
) ->
usize
) ->
*mut T
Creates a new pointer by mapping
self
’s address to a new one, preserving the original
pointer’s
provenance
.
This is a convenience for
with_addr
, see that method for details.
This is a
Strict Provenance
API.
Source
pub const fn
to_raw_parts
(self) -> (
*mut
()
, <T as
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
from_raw_parts_mut
.
1.9.0 (const: 1.84.0)
·
Source
pub const unsafe fn
as_ref
<'a>(self) ->
Option
<
&'a T
>
Returns
None
if the pointer is null, or else returns a shared reference to
the value wrapped in
Some
. If the value may be uninitialized,
as_uninit_ref
must be used instead.
For the mutable counterpart see
as_mut
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
let
ptr:
*mut
u8 =
&mut
10u8
as
*mut
u8;
unsafe
{
if let
Some
(val_back) = ptr.as_ref() {
println!
(
"We got back the value: {val_back}!"
);
    }
}
§
Null-unchecked version
If you are sure the pointer can never be null and are looking for some kind of
as_ref_unchecked
that returns the
&T
instead of
Option<&T>
, know that you can
dereference the pointer directly.
let
ptr:
*mut
u8 =
&mut
10u8
as
*mut
u8;
unsafe
{
let
val_back =
&*
ptr;
println!
(
"We got back the value: {val_back}!"
);
}
Source
pub const unsafe fn
as_ref_unchecked
<'a>(self) ->
&'a T
🔬
This is a nightly-only experimental API. (
ptr_as_ref_unchecked
#122034
)
Returns a shared reference to the value behind the pointer.
If the pointer may be null or the value may be uninitialized,
as_uninit_ref
must be used instead.
If the pointer may be null, but the value is known to have been initialized,
as_ref
must be used instead.
For the mutable counterpart see
as_mut_unchecked
.
§
Safety
When calling this method, you have to ensure that the pointer is
convertible to a reference
.
§
Examples
#![feature(ptr_as_ref_unchecked)]
let
ptr:
*mut
u8 =
&mut
10u8
as
*mut
u8;
unsafe
{
println!
(
"We got back the value: {}!"
, ptr.as_ref_unchecked());
}
Source
pub const unsafe fn
as_uninit_ref
<'a>(self) ->
Option
<&'a
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a shared reference to
the value wrapped in
Some
. In contrast to
as_ref
, this does not require
that the value has to be initialized.
For the mutable counterpart see
as_uninit_mut
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
Note that because the created reference is to
MaybeUninit<T>
, the
source pointer can point to uninitialized memory.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
#![feature(ptr_as_uninit)]
let
ptr:
*mut
u8 =
&mut
10u8
as
*mut
u8;
unsafe
{
if let
Some
(val_back) = ptr.as_uninit_ref() {
println!
(
"We got back the value: {}!"
, val_back.assume_init());
    }
}
1.0.0 (const: 1.61.0)
·
Source
pub const unsafe fn
offset
(self, count:
isize
) ->
*mut T
Adds a signed offset to a pointer.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
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
Consider using
wrapping_offset
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
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
ptr:
*mut
u32 = s.as_mut_ptr();
unsafe
{
assert_eq!
(
2
,
*
ptr.offset(
1
));
assert_eq!
(
3
,
*
ptr.offset(
2
));
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_offset
(self, count:
isize
) ->
*mut T
Adds a signed offset in bytes to a pointer.
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
1.16.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_offset
(self, count:
isize
) ->
*mut T
Adds a signed offset to a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_offset((y as isize) - (x as isize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
offset
, this method basically delays the requirement of staying within the
same allocated object:
offset
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_offset
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
offset
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_offset(o).wrapping_offset(o.wrapping_neg())
is always the same as
x
. In other
words, leaving the allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements
let
mut
data = [
1u8
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
mut
ptr:
*mut
u8 = data.as_mut_ptr();
let
step =
2
;
let
end_rounded_up = ptr.wrapping_offset(
6
);
while
ptr != end_rounded_up {
unsafe
{
*
ptr =
0
;
    }
    ptr = ptr.wrapping_offset(step);
}
assert_eq!
(
&
data,
&
[
0
,
2
,
0
,
4
,
0
]);
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_offset
(self, count:
isize
) ->
*mut T
Adds a signed offset in bytes to a pointer using wrapping arithmetic.
count
is in units of
bytes
.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_offset
on it. See that method
for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
Source
pub fn
mask
(self, mask:
usize
) ->
*mut T
🔬
This is a nightly-only experimental API. (
ptr_mask
#98290
)
Masks out bits of the pointer according to a mask.
This is convenience for
ptr.map_addr(|a| a & mask)
.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
§
Examples
#![feature(ptr_mask)]
let
mut
v =
17_u32
;
let
ptr:
*mut
u32 =
&mut
v;
// `u32` is 4 bytes aligned,
// which means that lower 2 bits are always 0.
let
tag_mask =
0b11
;
let
ptr_mask = !tag_mask;
// We can store something in these lower bits
let
tagged_ptr = ptr.map_addr(|a| a |
0b10
);
// Get the "tag" back
let
tag = tagged_ptr.addr() & tag_mask;
assert_eq!
(tag,
0b10
);
// Note that `tagged_ptr` is unaligned, it's UB to read from/write to it.
// To get original pointer `mask` can be used:
let
masked_ptr = tagged_ptr.mask(ptr_mask);
assert_eq!
(
unsafe
{
*
masked_ptr },
17
);
unsafe
{
*
masked_ptr =
0
};
assert_eq!
(v,
0
);
1.9.0 (const: 1.84.0)
·
Source
pub const unsafe fn
as_mut
<'a>(self) ->
Option
<
&'a mut T
>
Returns
None
if the pointer is null, or else returns a unique reference to
the value wrapped in
Some
. If the value may be uninitialized,
as_uninit_mut
must be used instead.
For the shared counterpart see
as_ref
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
§
Examples
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
ptr:
*mut
u32 = s.as_mut_ptr();
let
first_value =
unsafe
{ ptr.as_mut().unwrap() };
*
first_value =
4
;
println!
(
"{s:?}"
);
// It'll print: "[4, 2, 3]".
§
Null-unchecked version
If you are sure the pointer can never be null and are looking for some kind of
as_mut_unchecked
that returns the
&mut T
instead of
Option<&mut T>
, know that
you can dereference the pointer directly.
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
ptr:
*mut
u32 = s.as_mut_ptr();
let
first_value =
unsafe
{
&mut *
ptr };
*
first_value =
4
;
println!
(
"{s:?}"
);
// It'll print: "[4, 2, 3]".
Source
pub const unsafe fn
as_mut_unchecked
<'a>(self) ->
&'a mut T
🔬
This is a nightly-only experimental API. (
ptr_as_ref_unchecked
#122034
)
Returns a unique reference to the value behind the pointer.
If the pointer may be null or the value may be uninitialized,
as_uninit_mut
must be used instead.
If the pointer may be null, but the value is known to have been initialized,
as_mut
must be used instead.
For the shared counterpart see
as_ref_unchecked
.
§
Safety
When calling this method, you have to ensure that
the pointer is
convertible to a reference
.
§
Examples
#![feature(ptr_as_ref_unchecked)]
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
ptr:
*mut
u32 = s.as_mut_ptr();
let
first_value =
unsafe
{ ptr.as_mut_unchecked() };
*
first_value =
4
;
println!
(
"{s:?}"
);
// It'll print: "[4, 2, 3]".
Source
pub const unsafe fn
as_uninit_mut
<'a>(self) ->
Option
<&'a mut
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a unique reference to
the value wrapped in
Some
. In contrast to
as_mut
, this does not require
that the value has to be initialized.
For the shared counterpart see
as_uninit_ref
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
the pointer is
convertible to a reference
.
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
Source
pub const fn
guaranteed_eq
(self, other:
*mut T
) ->
Option
<
bool
>
🔬
This is a nightly-only experimental API. (
const_raw_ptr_comparison
#53020
)
Returns whether two pointers are guaranteed to be equal.
At runtime this function behaves like
Some(self == other)
.
However, in some contexts (e.g., compile-time evaluation),
it is not always possible to determine equality of two pointers, so this function may
spuriously return
None
for pointers that later actually turn out to have its equality known.
But when it returns
Some
, the pointers’ equality is guaranteed to be known.
The return value may change from
Some
to
None
and vice versa depending on the compiler
version and unsafe code must not
rely on the result of this function for soundness. It is suggested to only use this function
for performance optimizations where spurious
None
return values by this function do not
affect the outcome, but just the performance.
The consequences of using this method to make runtime and compile-time code behave
differently have not been explored. This method should not be used to introduce such
differences, and it should also not be stabilized before we have a better understanding
of this issue.
Source
pub const fn
guaranteed_ne
(self, other:
*mut T
) ->
Option
<
bool
>
🔬
This is a nightly-only experimental API. (
const_raw_ptr_comparison
#53020
)
Returns whether two pointers are guaranteed to be inequal.
At runtime this function behaves like
Some(self != other)
.
However, in some contexts (e.g., compile-time evaluation),
it is not always possible to determine inequality of two pointers, so this function may
spuriously return
None
for pointers that later actually turn out to have its inequality known.
But when it returns
Some
, the pointers’ inequality is guaranteed to be known.
The return value may change from
Some
to
None
and vice versa depending on the compiler
version and unsafe code must not
rely on the result of this function for soundness. It is suggested to only use this function
for performance optimizations where spurious
None
return values by this function do not
affect the outcome, but just the performance.
The consequences of using this method to make runtime and compile-time code behave
differently have not been explored. This method should not be used to introduce such
differences, and it should also not be stabilized before we have a better understanding
of this issue.
1.47.0 (const: 1.65.0)
·
Source
pub const unsafe fn
offset_from
(self, origin:
*const T
) ->
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
let
mut
a = [
0
;
5
];
let
ptr1:
*mut
i32 =
&mut
a[
1
];
let
ptr2:
*mut
i32 =
&mut
a[
3
];
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
let
ptr1 = Box::into_raw(Box::new(
0u8
));
let
ptr2 = Box::into_raw(Box::new(
1u8
));
let
diff = (ptr2
as
isize).wrapping_sub(ptr1
as
isize);
// Make ptr2_other an "alias" of ptr2.add(1), but derived from ptr1.
let
ptr2_other = (ptr1
as
*mut
u8).wrapping_offset(diff).wrapping_offset(
1
);
assert_eq!
(ptr2
as
usize, ptr2_other
as
usize);
// Since ptr2_other and ptr2 are derived from pointers to different objects,
// computing their offset is undefined behavior, even though
// they point to addresses that are in-bounds of the same object!
unsafe
{
let
one = ptr2_other.offset_from(ptr2);
// Undefined Behavior! ⚠️
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_offset_from
<U>(self, origin:
*const U
) ->
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
(self, origin:
*const T
) ->
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
This method can be thought of as recovering the
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
let
mut
a = [
0
;
5
];
let
p:
*mut
i32 = a.as_mut_ptr();
unsafe
{
let
ptr1:
*mut
i32 = p.add(
1
);
let
ptr2:
*mut
i32 = p.add(
3
);
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
// ptr1.offset_from(ptr2)
1.87.0 (const: 1.87.0)
·
Source
pub const unsafe fn
byte_offset_from_unsigned
<U>(self, origin:
*mut U
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
1.26.0 (const: 1.61.0)
·
Source
pub const unsafe fn
add
(self, count:
usize
) ->
*mut T
Adds an unsigned offset to a pointer.
This can only move the pointer forward (or not move it). If you need to move forward or
backward depending on the value, then you might want
offset
instead
which takes a signed offset.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
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
Consider using
wrapping_add
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
let
s:
&
str =
"123"
;
let
ptr:
*const
u8 = s.as_ptr();
unsafe
{
assert_eq!
(
'2'
,
*
ptr.add(
1
)
as
char);
assert_eq!
(
'3'
,
*
ptr.add(
2
)
as
char);
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_add
(self, count:
usize
) ->
*mut T
Adds an unsigned offset in bytes to a pointer.
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
1.26.0 (const: 1.61.0)
·
Source
pub const unsafe fn
sub
(self, count:
usize
) ->
*mut T
Subtracts an unsigned offset from a pointer.
This can only move the pointer backward (or not move it). If you need to move forward or
backward depending on the value, then you might want
offset
instead
which takes a signed offset.
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
The offset in bytes,
count * size_of::<T>()
, computed on mathematical integers (without
“wrapping around”), must fit in an
isize
.
If the computed offset is non-zero, then
self
must be
derived from
a pointer to some
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
Consider using
wrapping_sub
instead if these constraints are
difficult to satisfy. The only advantage of this method is that it
enables more aggressive compiler optimizations.
§
Examples
let
s:
&
str =
"123"
;
unsafe
{
let
end:
*const
u8 = s.as_ptr().add(
3
);
assert_eq!
(
'3'
,
*
end.sub(
1
)
as
char);
assert_eq!
(
'2'
,
*
end.sub(
2
)
as
char);
}
1.75.0 (const: 1.75.0)
·
Source
pub const unsafe fn
byte_sub
(self, count:
usize
) ->
*mut T
Subtracts an unsigned offset in bytes from a pointer.
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
1.26.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_add
(self, count:
usize
) ->
*mut T
Adds an unsigned offset to a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_add((y as usize) - (x as usize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
add
, this method basically delays the requirement of staying within the
same allocated object:
add
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_add
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
add
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_add(o).wrapping_sub(o)
is always the same as
x
. In other words, leaving the
allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements
let
data = [
1u8
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
mut
ptr:
*const
u8 = data.as_ptr();
let
step =
2
;
let
end_rounded_up = ptr.wrapping_add(
6
);
// This loop prints "1, 3, 5, "
while
ptr != end_rounded_up {
unsafe
{
print!
(
"{}, "
,
*
ptr);
    }
    ptr = ptr.wrapping_add(step);
}
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_add
(self, count:
usize
) ->
*mut T
Adds an unsigned offset in bytes to a pointer using wrapping arithmetic.
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_add
on it. See that method for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.26.0 (const: 1.61.0)
·
Source
pub const fn
wrapping_sub
(self, count:
usize
) ->
*mut T
Subtracts an unsigned offset from a pointer using wrapping arithmetic.
count
is in units of T; e.g., a
count
of 3 represents a pointer
offset of
3 * size_of::<T>()
bytes.
§
Safety
This operation itself is always safe, but using the resulting pointer is not.
The resulting pointer “remembers” the
allocated object
that
self
points to; it must not
be used to read or write other allocated objects.
In other words,
let z = x.wrapping_sub((x as usize) - (y as usize))
does
not
make
z
the same as
y
even if we assume
T
has size
1
and there is no overflow:
z
is still
attached to the object
x
is attached to, and dereferencing it is Undefined Behavior unless
x
and
y
point into the same allocated object.
Compared to
sub
, this method basically delays the requirement of staying within the
same allocated object:
sub
is immediate Undefined Behavior when crossing object
boundaries;
wrapping_sub
produces a pointer but still leads to Undefined Behavior if a
pointer is dereferenced when it is out-of-bounds of the object it is attached to.
sub
can be optimized better and is thus preferable in performance-sensitive code.
The delayed check only considers the value of the pointer that was dereferenced, not the
intermediate values used during the computation of the final result. For example,
x.wrapping_add(o).wrapping_sub(o)
is always the same as
x
. In other words, leaving the
allocated object and then re-entering it later is permitted.
§
Examples
// Iterate using a raw pointer in increments of two elements (backwards)
let
data = [
1u8
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
mut
ptr:
*const
u8 = data.as_ptr();
let
start_rounded_down = ptr.wrapping_sub(
2
);
ptr = ptr.wrapping_add(
4
);
let
step =
2
;
// This loop prints "5, 3, 1, "
while
ptr != start_rounded_down {
unsafe
{
print!
(
"{}, "
,
*
ptr);
    }
    ptr = ptr.wrapping_sub(step);
}
1.75.0 (const: 1.75.0)
·
Source
pub const fn
wrapping_byte_sub
(self, count:
usize
) ->
*mut T
Subtracts an unsigned offset in bytes from a pointer using wrapping arithmetic.
count
is in units of bytes.
This is purely a convenience for casting to a
u8
pointer and
using
wrapping_sub
on it. See that method for documentation.
For non-
Sized
pointees this operation changes only the data pointer,
leaving the metadata untouched.
1.26.0 (const: 1.71.0)
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
1.26.0
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
1.26.0 (const: 1.71.0)
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to
(self, dest:
*mut T
, count:
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_to_nonoverlapping
(self, dest:
*mut T
, count:
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_from
(self, src:
*const T
, count:
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
1.26.0 (const: 1.83.0)
·
Source
pub const unsafe fn
copy_from_nonoverlapping
(self, src:
*const T
, count:
usize
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
1.26.0
·
Source
pub unsafe fn
drop_in_place
(self)
Executes the destructor (if any) of the pointed-to value.
See
ptr::drop_in_place
for safety concerns and examples.
1.26.0 (const: 1.83.0)
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
1.26.0 (const: 1.83.0)
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
1.26.0
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
1.26.0 (const: 1.83.0)
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
1.26.0
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
1.26.0 (const: 1.85.0)
·
Source
pub const unsafe fn
swap
(self, with:
*mut T
)
Swaps the values at two mutable locations of the same type, without
deinitializing either. They may overlap, unlike
mem::swap
which is
otherwise equivalent.
See
ptr::swap
for safety concerns and examples.
1.36.0
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
elements, and not bytes. The value returned can be
used with the
wrapping_add
method.
There are no guarantees whatsoever that offsetting the pointer will not overflow or go
beyond the allocation that the pointer points into. It is up to the caller to ensure that
the returned offset is correct in all terms other than alignment.
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
let
mut
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
ptr = x.as_mut_ptr();
let
offset = ptr.align_offset(align_of::<u16>());
if
offset < x.len() -
1
{
let
u16_ptr = ptr.add(offset).cast::<u16>();
*
u16_ptr =
0
;
assert!
(x == [
0
,
0
,
7
,
8
,
9
] || x == [
5
,
0
,
0
,
8
,
9
]);
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
// On some platforms, the alignment of i32 is less than 4.
#[repr(align(
4
))]
struct
AlignedI32(i32);
let
mut
data = AlignedI32(
42
);
let
ptr =
&mut
data
as
*mut
AlignedI32;
assert!
(ptr.is_aligned());
assert!
(!ptr.wrapping_byte_add(
1
).is_aligned());
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
mut
data = AlignedI32(
42
);
let
ptr =
&mut
data
as
*mut
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
*mut
[T]
1.79.0 (const: 1.79.0)
·
Source
pub const fn
len
(self) ->
usize
Returns the length of a raw slice.
The returned value is the number of
elements
, not the number of bytes.
This function is safe, even when the raw slice cannot be cast to a slice
reference because the pointer is null or unaligned.
§
Examples
use
std::ptr;
let
slice:
*mut
[i8] = ptr::slice_from_raw_parts_mut(ptr::null_mut(),
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
if the raw slice has a length of 0.
§
Examples
use
std::ptr;
let
slice:
*mut
[i8] = ptr::slice_from_raw_parts_mut(ptr::null_mut(),
3
);
assert!
(!slice.is_empty());
Source
pub const fn
as_mut_array
<const N:
usize
>(self) ->
Option
<
*mut
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a raw, mutable pointer to the underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
Source
pub unsafe fn
split_at_mut
(self, mid:
usize
) -> (
*mut
[T]
,
*mut
[T]
)
🔬
This is a nightly-only experimental API. (
raw_slice_split
#95595
)
Divides one mutable raw slice into two at an index.
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
.
§
Safety
mid
must be
in-bounds
of the underlying
allocated object
.
Which means
self
must be dereferenceable and span a single allocation
that is at least
mid * size_of::<T>()
bytes long. Not upholding these
requirements is
undefined behavior
even if the resulting pointers are not used.
Since
len
being in-bounds it is not a safety invariant of
*mut [T]
the
safety requirements of this method are the same as for
split_at_mut_unchecked
.
The explicit bounds check is only as useful as
len
is correct.
§
Examples
#![feature(raw_slice_split)]
#![feature(slice_ptr_get)]
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
ptr =
&mut
v
as
*mut
[
_
];
unsafe
{
let
(left, right) = ptr.split_at_mut(
2
);
assert_eq!
(
&*
left, [
1
,
0
]);
assert_eq!
(
&*
right, [
3
,
0
,
5
,
6
]);
}
Source
pub unsafe fn
split_at_mut_unchecked
(self, mid:
usize
) -> (
*mut
[T]
,
*mut
[T]
)
🔬
This is a nightly-only experimental API. (
raw_slice_split
#95595
)
Divides one mutable raw slice into two at an index, without doing bounds checking.
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
Safety
mid
must be
in-bounds
of the underlying [allocated object].
Which means
self
must be dereferenceable and span a single allocation
that is at least
mid * size_of::<T>()
bytes long. Not upholding these
requirements is
undefined behavior
even if the resulting pointers are not used.
§
Examples
#![feature(raw_slice_split)]
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
ptr =
&mut
v
as
*mut
[
_
];
let
(left, right) = ptr.split_at_mut_unchecked(
2
);
assert_eq!
(
&*
left, [
1
,
0
]);
assert_eq!
(
&*
right, [
3
,
0
,
5
,
6
]);
    (
&mut *
left)[
1
] =
2
;
    (
&mut *
right)[
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
This is equivalent to casting
self
to
*mut T
, but more type-safe.
§
Examples
#![feature(slice_ptr_get)]
use
std::ptr;
let
slice:
*mut
[i8] = ptr::slice_from_raw_parts_mut(ptr::null_mut(),
3
);
assert_eq!
(slice.as_mut_ptr(), ptr::null_mut());
Source
pub unsafe fn
get_unchecked_mut
<I>(
    self,
    index: I,
) ->
*mut
<I as
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
🔬
This is a nightly-only experimental API. (
slice_ptr_get
#74265
)
Returns a raw pointer to an element or subslice, without doing bounds
checking.
Calling this method with an
out-of-bounds index
or when
self
is not dereferenceable
is
undefined behavior
even if the resulting pointer is not used.
§
Examples
#![feature(slice_ptr_get)]
let
x =
&mut
[
1
,
2
,
4
]
as
*mut
[i32];
unsafe
{
assert_eq!
(x.get_unchecked_mut(
1
), x.as_mut_ptr().add(
1
));
}
Source
pub const unsafe fn
as_uninit_slice
<'a>(self) ->
Option
<&'a [
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a shared slice to
the value wrapped in
Some
. In contrast to
as_ref
, this does not require
that the value has to be initialized.
For the mutable counterpart see
as_uninit_slice_mut
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
all of the following is true:
The pointer must be
valid
for reads for
ptr.len() * size_of::<T>()
many bytes,
and it must be properly aligned. This means in particular:
The entire memory range of this slice must be contained within a single
allocated object
!
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
§
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
Source
pub const unsafe fn
as_uninit_slice_mut
<'a>(
    self,
) ->
Option
<&'a mut [
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
ptr_as_uninit
#75402
)
Returns
None
if the pointer is null, or else returns a unique slice to
the value wrapped in
Some
. In contrast to
as_mut
, this does not require
that the value has to be initialized.
For the shared counterpart see
as_uninit_slice
.
§
Safety
When calling this method, you have to ensure that
either
the pointer is null
or
all of the following is true:
The pointer must be
valid
for reads and writes for
ptr.len() * size_of::<T>()
many bytes, and it must be properly aligned. This means in particular:
The entire memory range of this slice must be contained within a single
allocated object
!
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
Panics during const evaluation
This method will panic during const evaluation if the pointer cannot be
determined to be null or not. See
is_null
for more information.
Source
§
impl<T, const N:
usize
>
*mut
[T; N]
Source
pub const fn
as_mut_ptr
(self) ->
*mut T
🔬
This is a nightly-only experimental API. (
array_ptr_get
#119834
)
Returns a raw pointer to the array’s buffer.
This is equivalent to casting
self
to
*mut T
, but more type-safe.
§
Examples
#![feature(array_ptr_get)]
use
std::ptr;
let
arr:
*mut
[i8;
3
] = ptr::null_mut();
assert_eq!
(arr.as_mut_ptr(), ptr::null_mut());
Source
pub const fn
as_mut_slice
(self) ->
*mut
[T]
🔬
This is a nightly-only experimental API. (
array_ptr_get
#119834
)
Returns a raw pointer to a mutable slice containing the entire array.
§
Examples
#![feature(array_ptr_get)]
let
mut
arr = [
1
,
2
,
5
];
let
ptr:
*mut
[i32;
3
] =
&mut
arr;
unsafe
{
    (
&mut *
ptr.as_mut_slice())[..
2
].copy_from_slice(
&
[
3
,
4
]);
}
assert_eq!
(arr, [
3
,
4
,
5
]);
Trait Implementations
§
Source
§
impl<P, T>
AggregateRawPtr
<
*const T
> for
*const P
where
    T:
Thin
,
    P: ?
Sized
,
Source
§
type
Metadata
= <P as
Pointee
>::
Metadata
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Source
§
impl<P, T>
AggregateRawPtr
<
*mut T
> for
*mut P
where
    T:
Thin
,
    P: ?
Sized
,
Source
§
type
Metadata
= <P as
Pointee
>::
Metadata
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
1.0.0
·
Source
§
impl<T>
Clone
for
*const T
where
    T: ?
Sized
,
Source
§
fn
clone
(&self) ->
*const T
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
1.0.0
·
Source
§
impl<T>
Clone
for
*mut T
where
    T: ?
Sized
,
Source
§
fn
clone
(&self) ->
*mut T
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
1.0.0
·
Source
§
impl<T>
Debug
for
*const T
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
1.0.0
·
Source
§
impl<T>
Debug
for
*mut T
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
1.23.0
·
Source
§
impl<T>
From
<
*mut T
> for
AtomicPtr
<T>
Source
§
fn
from
(p:
*mut T
) ->
AtomicPtr
<T>
Converts a
*mut T
into an
AtomicPtr<T>
.
1.0.0
·
Source
§
impl<T>
Hash
for
*const T
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
1.0.0
·
Source
§
impl<T>
Hash
for
*mut T
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
1.0.0
·
Source
§
impl<T>
Ord
for
*const T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
[
<*const T>::addr
](pointer::addr)
method.
Source
§
fn
cmp
(&self, other: &
*const T
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
1.0.0
·
Source
§
impl<T>
Ord
for
*mut T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
<*mut T>::addr
method.
Source
§
fn
cmp
(&self, other: &
*mut T
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
1.0.0
·
Source
§
impl<T>
PartialEq
for
*const T
where
    T: ?
Sized
,
Pointer equality is by address, as produced by the
<*const T>::addr
method.
Source
§
fn
eq
(&self, other: &
*const T
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
1.0.0
·
Source
§
impl<T>
PartialEq
for
*mut T
where
    T: ?
Sized
,
Pointer equality is by address, as produced by the
<*mut T>::addr
method.
Source
§
fn
eq
(&self, other: &
*mut T
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
1.0.0
·
Source
§
impl<T>
PartialOrd
for
*const T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
[
<*const T>::addr
](pointer::addr)
method.
Source
§
fn
partial_cmp
(&self, other: &
*const T
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
Source
§
fn
lt
(&self, other: &
*const T
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
Source
§
fn
le
(&self, other: &
*const T
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
Source
§
fn
gt
(&self, other: &
*const T
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
Source
§
fn
ge
(&self, other: &
*const T
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
1.0.0
·
Source
§
impl<T>
PartialOrd
for
*mut T
where
    T: ?
Sized
,
Pointer comparison is by address, as produced by the
<*mut T>::addr
method.
Source
§
fn
partial_cmp
(&self, other: &
*mut T
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
Source
§
fn
lt
(&self, other: &
*mut T
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
Source
§
fn
le
(&self, other: &
*mut T
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
Source
§
fn
gt
(&self, other: &
*mut T
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
Source
§
fn
ge
(&self, other: &
*mut T
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
1.0.0
·
Source
§
impl<T>
Pointer
for
*const T
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
1.0.0
·
Source
§
impl<T>
Pointer
for
*mut T
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
impl<T>
SimdElement
for
*const T
where
    T:
Pointee
<Metadata =
()
>,
Source
§
type
Mask
=
isize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask element type corresponding to this element type.
Source
§
impl<T>
SimdElement
for
*mut T
where
    T:
Pointee
<Metadata =
()
>,
Source
§
type
Mask
=
isize
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
The mask element type corresponding to this element type.
Source
§
impl<'a, T, U>
CoerceUnsized
<
*const U
> for
&'a T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
*const U
> for
&'a mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
*const U
> for
*const T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
*const U
> for
*mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<'a, T, U>
CoerceUnsized
<
*mut U
> for
&'a mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
CoerceUnsized
<
*mut U
> for
*mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Copy
for
*const T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Copy
for
*mut T
where
    T: ?
Sized
,
Source
§
impl<T, U>
DispatchFromDyn
<
*const U
> for
*const T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, U>
DispatchFromDyn
<
*mut U
> for
*mut T
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Eq
for
*const T
where
    T: ?
Sized
,
Pointer equality is an equivalence relation.
1.0.0
·
Source
§
impl<T>
Eq
for
*mut T
where
    T: ?
Sized
,
Pointer equality is an equivalence relation.
Source
§
impl<T>
Freeze
for
*const T
where
    T: ?
Sized
,
Source
§
impl<T>
Freeze
for
*mut T
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
*const T
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
*mut T
where
    T: ?
Sized
,
Source
§
impl<T>
PointerLike
for
*const T
Source
§
impl<T>
PointerLike
for
*mut T
1.0.0
·
Source
§
impl<T> !
Send
for
*const T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Send
for
*mut T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Sync
for
*const T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T> !
Sync
for
*mut T
where
    T: ?
Sized
,
1.38.0
·
Source
§
impl<T>
Unpin
for
*const T
where
    T: ?
Sized
,
1.38.0
·
Source
§
impl<T>
Unpin
for
*mut T
where
    T: ?
Sized
,
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
*const T
where
    T:
RefUnwindSafe
+ ?
Sized
,
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
*mut T
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
RefUnwindSafe
for
*const T
where
    T:
RefUnwindSafe
+ ?
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