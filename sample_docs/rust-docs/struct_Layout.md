Layout in std::alloc - Rust
std
::
alloc
Struct
Layout
Copy item path
1.28.0
·
Source
pub struct Layout {
/* private fields */
}
Expand description
Layout of a block of memory.
An instance of
Layout
describes a particular layout of memory.
You build a
Layout
up as an input to give to an allocator.
All layouts have an associated size and a power-of-two alignment. The size, when rounded up to
the nearest multiple of
align
, does not overflow
isize
(i.e., the rounded value will always be
less than or equal to
isize::MAX
).
(Note that layouts are
not
required to have non-zero size,
even though
GlobalAlloc
requires that all memory requests
be non-zero in size. A caller must either ensure that conditions
like this are met, use specific allocators with looser
requirements, or use the more lenient
Allocator
interface.)
Implementations
§
Source
§
impl
Layout
1.28.0 (const: 1.50.0)
·
Source
pub const fn
from_size_align
(
    size:
usize
,
    align:
usize
,
) ->
Result
<
Layout
,
LayoutError
>
Constructs a
Layout
from a given
size
and
align
,
or returns
LayoutError
if any of the following conditions
are not met:
align
must not be zero,
align
must be a power of two,
size
, when rounded up to the nearest multiple of
align
,
must not overflow
isize
(i.e., the rounded value must be
less than or equal to
isize::MAX
).
1.28.0 (const: 1.36.0)
·
Source
pub const unsafe fn
from_size_align_unchecked
(
    size:
usize
,
    align:
usize
,
) ->
Layout
Creates a layout, bypassing all checks.
§
Safety
This function is unsafe as it does not verify the preconditions from
Layout::from_size_align
.
1.28.0 (const: 1.50.0)
·
Source
pub const fn
size
(&self) ->
usize
The minimum size in bytes for a memory block of this layout.
1.28.0 (const: 1.50.0)
·
Source
pub const fn
align
(&self) ->
usize
The minimum byte alignment for a memory block of this layout.
The returned alignment is guaranteed to be a power of two.
1.28.0 (const: 1.42.0)
·
Source
pub const fn
new
<T>() ->
Layout
Constructs a
Layout
suitable for holding a value of type
T
.
1.28.0 (const: 1.85.0)
·
Source
pub const fn
for_value
<T>(t:
&T
) ->
Layout
where
    T: ?
Sized
,
Produces layout describing a record that could be used to
allocate backing structure for
T
(which could be a trait
or other unsized type like a slice).
Source
pub const unsafe fn
for_value_raw
<T>(t:
*const T
) ->
Layout
where
    T: ?
Sized
,
🔬
This is a nightly-only experimental API. (
layout_for_ptr
#69835
)
Produces layout describing a record that could be used to
allocate backing structure for
T
(which could be a trait
or other unsized type like a slice).
§
Safety
This function is only safe to call if the following conditions hold:
If
T
is
Sized
, this function is always safe to call.
If the unsized tail of
T
is:
a
slice
, then the length of the slice tail must be an initialized
integer, and the size of the
entire value
(dynamic tail length + statically sized prefix) must fit in
isize
.
For the special case where the dynamic tail length is 0, this function
is safe to call.
a
trait object
, then the vtable part of the pointer must point
to a valid vtable for the type
T
acquired by an unsizing coercion,
and the size of the
entire value
(dynamic tail length + statically sized prefix) must fit in
isize
.
an (unstable)
extern type
, then this function is always safe to
call, but may panic or otherwise return the wrong value, as the
extern type’s layout is not known. This is the same behavior as
Layout::for_value
on a reference to an extern type tail.
otherwise, it is conservatively not allowed to call this function.
Source
pub const fn
dangling
(&self) ->
NonNull
<
u8
>
🔬
This is a nightly-only experimental API. (
alloc_layout_extra
#55724
)
Creates a
NonNull
that is dangling, but well-aligned for this Layout.
Note that the pointer value may potentially represent a valid pointer,
which means this must not be used as a “not yet initialized”
sentinel value. Types that lazily allocate must track initialization by
some other means.
1.44.0 (const: 1.85.0)
·
Source
pub const fn
align_to
(&self, align:
usize
) ->
Result
<
Layout
,
LayoutError
>
Creates a layout describing the record that can hold a value
of the same layout as
self
, but that also is aligned to
alignment
align
(measured in bytes).
If
self
already meets the prescribed alignment, then returns
self
.
Note that this method does not add any padding to the overall
size, regardless of whether the returned layout has a different
alignment. In other words, if
K
has size 16,
K.align_to(32)
will
still
have size 16.
Returns an error if the combination of
self.size()
and the given
align
violates the conditions listed in
Layout::from_size_align
.
Source
pub const fn
padding_needed_for
(&self, align:
usize
) ->
usize
🔬
This is a nightly-only experimental API. (
alloc_layout_extra
#55724
)
Returns the amount of padding we must insert after
self
to ensure that the following address will satisfy
align
(measured in bytes).
e.g., if
self.size()
is 9, then
self.padding_needed_for(4)
returns 3, because that is the minimum number of bytes of
padding required to get a 4-aligned address (assuming that the
corresponding memory block starts at a 4-aligned address).
The return value of this function has no meaning if
align
is
not a power-of-two.
Note that the utility of the returned value requires
align
to be less than or equal to the alignment of the starting
address for the whole allocated block of memory. One way to
satisfy this constraint is to ensure
align <= self.align()
.
1.44.0 (const: 1.85.0)
·
Source
pub const fn
pad_to_align
(&self) ->
Layout
Creates a layout by rounding the size of this layout up to a multiple
of the layout’s alignment.
This is equivalent to adding the result of
padding_needed_for
to the layout’s current size.
Source
pub const fn
repeat
(&self, n:
usize
) ->
Result
<(
Layout
,
usize
),
LayoutError
>
🔬
This is a nightly-only experimental API. (
alloc_layout_extra
#55724
)
Creates a layout describing the record for
n
instances of
self
, with a suitable amount of padding between each to
ensure that each instance is given its requested size and
alignment. On success, returns
(k, offs)
where
k
is the
layout of the array and
offs
is the distance between the start
of each element in the array.
(That distance between elements is sometimes known as “stride”.)
On arithmetic overflow, returns
LayoutError
.
§
Examples
#![feature(alloc_layout_extra)]
use
std::alloc::Layout;
// All rust types have a size that's a multiple of their alignment.
let
normal = Layout::from_size_align(
12
,
4
).unwrap();
let
repeated = normal.repeat(
3
).unwrap();
assert_eq!
(repeated, (Layout::from_size_align(
36
,
4
).unwrap(),
12
));
// But you can manually make layouts which don't meet that rule.
let
padding_needed = Layout::from_size_align(
6
,
4
).unwrap();
let
repeated = padding_needed.repeat(
3
).unwrap();
assert_eq!
(repeated, (Layout::from_size_align(
24
,
4
).unwrap(),
8
));
1.44.0 (const: 1.85.0)
·
Source
pub const fn
extend
(&self, next:
Layout
) ->
Result
<(
Layout
,
usize
),
LayoutError
>
Creates a layout describing the record for
self
followed by
next
, including any necessary padding to ensure that
next
will be properly aligned, but
no trailing padding
.
In order to match C representation layout
repr(C)
, you should
call
pad_to_align
after extending the layout with all fields.
(There is no way to match the default Rust representation
layout
repr(Rust)
, as it is unspecified.)
Note that the alignment of the resulting layout will be the maximum of
those of
self
and
next
, in order to ensure alignment of both parts.
Returns
Ok((k, offset))
, where
k
is layout of the concatenated
record and
offset
is the relative location, in bytes, of the
start of the
next
embedded within the concatenated record
(assuming that the record itself starts at offset 0).
On arithmetic overflow, returns
LayoutError
.
§
Examples
To calculate the layout of a
#[repr(C)]
structure and the offsets of
the fields from its fields’ layouts:
pub fn
repr_c(fields:
&
[Layout]) ->
Result
<(Layout, Vec<usize>), LayoutError> {
let
mut
offsets = Vec::new();
let
mut
layout = Layout::from_size_align(
0
,
1
)
?
;
for
&
field
in
fields {
let
(new_layout, offset) = layout.extend(field)
?
;
        layout = new_layout;
        offsets.push(offset);
    }
// Remember to finalize with `pad_to_align`!
Ok
((layout.pad_to_align(), offsets))
}
Source
pub const fn
repeat_packed
(&self, n:
usize
) ->
Result
<
Layout
,
LayoutError
>
🔬
This is a nightly-only experimental API. (
alloc_layout_extra
#55724
)
Creates a layout describing the record for
n
instances of
self
, with no padding between each instance.
Note that, unlike
repeat
,
repeat_packed
does not guarantee
that the repeated instances of
self
will be properly
aligned, even if a given instance of
self
is properly
aligned. In other words, if the layout returned by
repeat_packed
is used to allocate an array, it is not
guaranteed that all elements in the array will be properly
aligned.
On arithmetic overflow, returns
LayoutError
.
Source
pub const fn
extend_packed
(&self, next:
Layout
) ->
Result
<
Layout
,
LayoutError
>
🔬
This is a nightly-only experimental API. (
alloc_layout_extra
#55724
)
Creates a layout describing the record for
self
followed by
next
with no additional padding between the two. Since no
padding is inserted, the alignment of
next
is irrelevant,
and is not incorporated
at all
into the resulting layout.
On arithmetic overflow, returns
LayoutError
.
1.44.0 (const: 1.85.0)
·
Source
pub const fn
array
<T>(n:
usize
) ->
Result
<
Layout
,
LayoutError
>
Creates a layout describing the record for a
[T; n]
.
On arithmetic overflow or when the total size would exceed
isize::MAX
, returns
LayoutError
.
Trait Implementations
§
1.28.0
·
Source
§
impl
Clone
for
Layout
Source
§
fn
clone
(&self) ->
Layout
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
1.28.0
·
Source
§
impl
Debug
for
Layout
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
1.28.0
·
Source
§
impl
Hash
for
Layout
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
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
1.28.0
·
Source
§
impl
PartialEq
for
Layout
Source
§
fn
eq
(&self, other: &
Layout
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
1.28.0
·
Source
§
impl
Copy
for
Layout
1.28.0
·
Source
§
impl
Eq
for
Layout
1.28.0
·
Source
§
impl
StructuralPartialEq
for
Layout
Auto Trait Implementations
§
§
impl
Freeze
for
Layout
§
impl
RefUnwindSafe
for
Layout
§
impl
Send
for
Layout
§
impl
Sync
for
Layout
§
impl
Unpin
for
Layout
§
impl
UnwindSafe
for
Layout
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