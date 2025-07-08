Alignment in std::ptr - Rust
std
::
ptr
Struct
Alignment
Copy item path
Source
pub struct Alignment(
/* private fields */
);
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Expand description
A type storing a
usize
which is a power of two, and thus
represents a possible alignment in the Rust abstract machine.
Note that particularly large alignments, while representable in this type,
are likely not to be supported by actual allocators and linkers.
Implementations
§
Source
§
impl
Alignment
Source
pub const
MIN
:
Alignment
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
The smallest possible alignment, 1.
All addresses are always aligned at least this much.
§
Examples
#![feature(ptr_alignment_type)]
use
std::ptr::Alignment;
assert_eq!
(Alignment::MIN.as_usize(),
1
);
Source
pub const fn
of
<T>() ->
Alignment
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Returns the alignment for a type.
This provides the same numerical value as
align_of
,
but in an
Alignment
instead of a
usize
.
Source
pub const fn
new
(align:
usize
) ->
Option
<
Alignment
>
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Creates an
Alignment
from a
usize
, or returns
None
if it’s
not a power of two.
Note that
0
is not a power of two, nor a valid alignment.
Source
pub const unsafe fn
new_unchecked
(align:
usize
) ->
Alignment
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Creates an
Alignment
from a power-of-two
usize
.
§
Safety
align
must be a power of two.
Equivalently, it must be
1 << exp
for some
exp
in
0..usize::BITS
.
It must
not
be zero.
Source
pub const fn
as_usize
(self) ->
usize
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Returns the alignment as a
usize
.
Source
pub const fn
as_nonzero
(self) ->
NonZero
<
usize
>
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Returns the alignment as a
NonZero
<
usize
>
.
Source
pub const fn
log2
(self) ->
u32
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Returns the base-2 logarithm of the alignment.
This is always exact, as
self
represents a power of two.
§
Examples
#![feature(ptr_alignment_type)]
use
std::ptr::Alignment;
assert_eq!
(Alignment::of::<u8>().log2(),
0
);
assert_eq!
(Alignment::new(
1024
).unwrap().log2(),
10
);
Source
pub const fn
mask
(self) ->
usize
🔬
This is a nightly-only experimental API. (
ptr_alignment_type
#102070
)
Returns a bit mask that can be used to match this alignment.
This is equivalent to
!(self.as_usize() - 1)
.
§
Examples
#![feature(ptr_alignment_type)]
#![feature(ptr_mask)]
use
std::ptr::{Alignment, NonNull};
#[repr(align(
1
))]
struct
Align1(u8);
#[repr(align(
2
))]
struct
Align2(u16);
#[repr(align(
4
))]
struct
Align4(u32);
let
one = <NonNull<Align1>>::dangling().as_ptr();
let
two = <NonNull<Align2>>::dangling().as_ptr();
let
four = <NonNull<Align4>>::dangling().as_ptr();
assert_eq!
(four.mask(Alignment::of::<Align1>().mask()), four);
assert_eq!
(four.mask(Alignment::of::<Align2>().mask()), four);
assert_eq!
(four.mask(Alignment::of::<Align4>().mask()), four);
assert_ne!
(one.mask(Alignment::of::<Align4>().mask()), one);
Trait Implementations
§
Source
§
impl
Clone
for
Alignment
Source
§
fn
clone
(&self) ->
Alignment
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
Alignment
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
Alignment
Returns
Alignment::MIN
, which is valid for any type.
Source
§
fn
default
() ->
Alignment
Returns the “default value” for a type.
Read more
Source
§
impl
From
<
Alignment
> for
NonZero
<
usize
>
Source
§
fn
from
(align:
Alignment
) ->
NonZero
<
usize
>
Converts to this type from the input type.
Source
§
impl
From
<
Alignment
> for
usize
Source
§
fn
from
(align:
Alignment
) ->
usize
Converts to this type from the input type.
Source
§
impl
Hash
for
Alignment
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
Ord
for
Alignment
Source
§
fn
cmp
(&self, other: &
Alignment
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
impl
PartialEq
for
Alignment
Source
§
fn
eq
(&self, other: &
Alignment
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
PartialOrd
for
Alignment
Source
§
fn
partial_cmp
(&self, other: &
Alignment
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
impl
TryFrom
<
NonZero
<
usize
>> for
Alignment
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    align:
NonZero
<
usize
>,
) ->
Result
<
Alignment
, <
Alignment
as
TryFrom
<
NonZero
<
usize
>>>::
Error
>
Performs the conversion.
Source
§
impl
TryFrom
<
usize
> for
Alignment
Source
§
type
Error
=
TryFromIntError
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    align:
usize
,
) ->
Result
<
Alignment
, <
Alignment
as
TryFrom
<
usize
>>::
Error
>
Performs the conversion.
Source
§
impl
Copy
for
Alignment
Source
§
impl
Eq
for
Alignment
Source
§
impl
StructuralPartialEq
for
Alignment
Auto Trait Implementations
§
§
impl
Freeze
for
Alignment
§
impl
RefUnwindSafe
for
Alignment
§
impl
Send
for
Alignment
§
impl
Sync
for
Alignment
§
impl
Unpin
for
Alignment
§
impl
UnwindSafe
for
Alignment
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