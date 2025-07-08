Assume in std::mem - Rust
std
::
mem
Struct
Assume
Copy item path
Source
pub struct Assume {
    pub alignment:
bool
,
    pub lifetimes:
bool
,
    pub safety:
bool
,
    pub validity:
bool
,
}
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
Expand description
Configurable proof assumptions of
TransmuteFrom
.
When
false
, the respective proof obligation belongs to the compiler. When
true
, the onus of the safety proof belongs to the programmer.
Fields
§
§
alignment:
bool
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
When
false
,
TransmuteFrom
is not implemented for transmutations
that might violate the alignment requirements of references; e.g.:
ⓘ
#![feature(transmutability)]
use
core::mem::TransmuteFrom;
assert_eq!
(align_of::<[u8;
2
]>(),
1
);
assert_eq!
(align_of::<u16>(),
2
);
let
src:
&
[u8;
2
] =
&
[
0xFF
,
0xFF
];
// SAFETY: No safety obligations.
let
dst:
&
u16 =
unsafe
{
    <
_ as
TransmuteFrom<
_
>>::transmute(src)
};
When
true
,
TransmuteFrom
assumes that
you
have ensured
that references in the transmuted value satisfy the alignment
requirements of their referent types; e.g.:
#![feature(pointer_is_aligned_to, transmutability)]
use
core::mem::{Assume, TransmuteFrom};
let
src:
&
[u8;
2
] =
&
[
0xFF
,
0xFF
];
let
maybe_dst:
Option
<
&
u16> =
if
<
*const
_
>::is_aligned_to(src, align_of::<u16>()) {
// SAFETY: We have checked above that the address of `src` satisfies the
    // alignment requirements of `u16`.
Some
(
unsafe
{
        <
_ as
TransmuteFrom<
_
, { Assume::ALIGNMENT }>>::transmute(src)
    })
}
else
{
None
};
assert!
(
matches!
(maybe_dst,
Some
(
&
u16::MAX) |
None
));
§
lifetimes:
bool
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
When
false
,
TransmuteFrom
is not implemented for transmutations
that extend the lifetimes of references.
When
true
,
TransmuteFrom
assumes that
you
have ensured that
references in the transmuted value do not outlive their referents.
§
safety:
bool
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
When
false
,
TransmuteFrom
is not implemented for transmutations
that might violate the library safety invariants of the destination
type; e.g.:
ⓘ
#![feature(transmutability)]
use
core::mem::TransmuteFrom;
let
src: u8 =
3
;
struct
EvenU8 {
// SAFETY: `val` must be an even number.
val: u8,
}
// SAFETY: No safety obligations.
let
dst: EvenU8 =
unsafe
{
    <
_ as
TransmuteFrom<
_
>>::transmute(src)
};
When
true
,
TransmuteFrom
assumes that
you
have ensured
that undefined behavior does not arise from using the transmuted value;
e.g.:
#![feature(transmutability)]
use
core::mem::{Assume, TransmuteFrom};
let
src: u8 =
42
;
struct
EvenU8 {
// SAFETY: `val` must be an even number.
val: u8,
}
let
maybe_dst:
Option
<EvenU8> =
if
src %
2
==
0
{
// SAFETY: We have checked above that the value of `src` is even.
Some
(
unsafe
{
        <
_ as
TransmuteFrom<
_
, { Assume::SAFETY }>>::transmute(src)
    })
}
else
{
None
};
assert!
(
matches!
(maybe_dst,
Some
(EvenU8 { val:
42
})));
§
validity:
bool
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
When
false
,
TransmuteFrom
is not implemented for transmutations
that might violate the language-level bit-validity invariant of the
destination type; e.g.:
ⓘ
#![feature(transmutability)]
use
core::mem::TransmuteFrom;
let
src: u8 =
3
;
// SAFETY: No safety obligations.
let
dst: bool =
unsafe
{
    <
_ as
TransmuteFrom<
_
>>::transmute(src)
};
When
true
,
TransmuteFrom
assumes that
you
have ensured
that the value being transmuted is a bit-valid instance of the
transmuted value; e.g.:
#![feature(transmutability)]
use
core::mem::{Assume, TransmuteFrom};
let
src: u8 =
1
;
let
maybe_dst:
Option
<bool> =
if
src ==
0
|| src ==
1
{
// SAFETY: We have checked above that the value of `src` is a bit-valid
    // instance of `bool`.
Some
(
unsafe
{
        <
_ as
TransmuteFrom<
_
, { Assume::VALIDITY }>>::transmute(src)
    })
}
else
{
None
};
assert_eq!
(maybe_dst,
Some
(
true
));
Implementations
§
Source
§
impl
Assume
Source
pub const
NOTHING
:
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
With this,
TransmuteFrom
does not assume you have ensured any safety
obligations are met, and relies only upon its own analysis to (dis)prove
transmutability.
Source
pub const
ALIGNMENT
:
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
With this,
TransmuteFrom
assumes only that you have ensured that
references in the transmuted value satisfy the alignment requirements of
their referent types. See
Assume::alignment
for examples.
Source
pub const
LIFETIMES
:
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
With this,
TransmuteFrom
assumes only that you have ensured that
references in the transmuted value do not outlive their referents. See
Assume::lifetimes
for examples.
Source
pub const
SAFETY
:
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
With this,
TransmuteFrom
assumes only that you have ensured that
undefined behavior does not arise from using the transmuted value. See
Assume::safety
for examples.
Source
pub const
VALIDITY
:
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
With this,
TransmuteFrom
assumes only that you have ensured that the
value being transmuted is a bit-valid instance of the transmuted value.
See
Assume::validity
for examples.
Source
pub const fn
and
(self, other_assumptions:
Assume
) ->
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
Combine the assumptions of
self
and
other_assumptions
.
This is especially useful for extending
Assume
in generic contexts;
e.g.:
#![feature(
    adt_const_params,
    generic_const_exprs,
    pointer_is_aligned_to,
    transmutability,
)]
#![allow(incomplete_features)]
use
core::mem::{Assume, TransmuteFrom};
/// Attempts to transmute `src` to `&Dst`.
///
/// Returns `None` if `src` violates the alignment requirements of `&Dst`.
///
/// # Safety
///
/// The caller guarantees that the obligations required by `ASSUME`, except
/// alignment, are satisfied.
unsafe fn
try_transmute_ref<
'a
, Src, Dst,
const
ASSUME: Assume>(src:
&
'a
Src) ->
Option
<
&
'a
Dst>
where
&
'a
Dst: TransmuteFrom<
&
'a
Src, { ASSUME.and(Assume::ALIGNMENT) }>,
{
if
<
*const
_
>::is_aligned_to(src, align_of::<Dst>()) {
// SAFETY: By the above dynamic check, we have ensured that the address
        // of `src` satisfies the alignment requirements of `&Dst`. By contract
        // on the caller, the safety obligations required by `ASSUME` have also
        // been satisfied.
Some
(
unsafe
{
            <
_ as
TransmuteFrom<
_
, { ASSUME.and(Assume::ALIGNMENT) }>>::transmute(src)
        })
    }
else
{
None
}
}
let
src:
&
[u8;
2
] =
&
[
0xFF
,
0xFF
];
// SAFETY: No safety obligations.
let
maybe_dst:
Option
<
&
u16> =
unsafe
{
    try_transmute_ref::<
_
,
_
, { Assume::NOTHING }>(src)
};
Source
pub const fn
but_not
(self, other_assumptions:
Assume
) ->
Assume
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
Remove
other_assumptions
the obligations of
self
; e.g.:
#![feature(transmutability)]
use
core::mem::Assume;
let
assumptions = Assume::ALIGNMENT.and(Assume::SAFETY);
let
to_be_removed = Assume::SAFETY.and(Assume::VALIDITY);
assert_eq!
(
    assumptions.but_not(to_be_removed),
    Assume::ALIGNMENT,
);
Trait Implementations
§
Source
§
impl
Add
for
Assume
Source
§
type
Output
=
Assume
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, other_assumptions:
Assume
) ->
Assume
Performs the
+
operation.
Read more
Source
§
impl
Clone
for
Assume
Source
§
fn
clone
(&self) ->
Assume
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
Assume
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
PartialEq
for
Assume
Source
§
fn
eq
(&self, other: &
Assume
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
Sub
for
Assume
Source
§
type
Output
=
Assume
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other_assumptions:
Assume
) ->
Assume
Performs the
-
operation.
Read more
Source
§
impl
ConstParamTy_
for
Assume
Source
§
impl
Copy
for
Assume
Source
§
impl
Eq
for
Assume
Source
§
impl
StructuralPartialEq
for
Assume
Source
§
impl
UnsizedConstParamTy
for
Assume
Auto Trait Implementations
§
§
impl
Freeze
for
Assume
§
impl
RefUnwindSafe
for
Assume
§
impl
Send
for
Assume
§
impl
Sync
for
Assume
§
impl
Unpin
for
Assume
§
impl
UnwindSafe
for
Assume
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