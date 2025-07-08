TransmuteFrom in std::mem - Rust
std
::
mem
Trait
TransmuteFrom
Copy item path
Source
pub unsafe trait TransmuteFrom<Src, const ASSUME:
Assume
= core::::mem::transmutability::TransmuteFrom::{constant#0}>
where
    Src: ?
Sized
,
{
    // Provided method
    unsafe fn
transmute
(src: Src) -> Self
where Self:
Sized
{ ... }
}
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
Expand description
Marks that
Src
is transmutable into
Self
.
§
Implementation
This trait cannot be implemented explicitly. It is implemented on-the-fly by
the compiler for all types
Src
and
Self
such that, given a set of safety
obligations on the programmer (see
Assume
), the compiler has proved that
the bits of a value of type
Src
can be soundly reinterpreted as a
Self
.
§
Safety
If
Dst: TransmuteFrom<Src, ASSUMPTIONS>
, the compiler guarantees that
Src
is soundly
union-transmutable
into a value of type
Dst
, provided
that the programmer has guaranteed that the given
ASSUMPTIONS
are satisfied.
A union-transmute is any bit-reinterpretation conversion in the form of:
pub unsafe fn
transmute_via_union<Src, Dst>(src: Src) -> Dst {
use
core::mem::ManuallyDrop;
#[repr(C)]
union
Transmute<Src, Dst> {
        src: ManuallyDrop<Src>,
        dst: ManuallyDrop<Dst>,
    }
let
transmute = Transmute {
        src: ManuallyDrop::new(src),
    };
let
dst =
unsafe
{ transmute.dst };

    ManuallyDrop::into_inner(dst)
}
Note that this construction is more permissive than
mem::transmute_copy
; union-transmutes permit
conversions that extend the bits of
Src
with trailing padding to fill
trailing uninitialized bytes of
Self
; e.g.:
#![feature(transmutability)]
use
core::mem::{Assume, TransmuteFrom};
let
src =
42u8
;
// size = 1
#[repr(C, align(
2
))]
struct
Dst(u8);
// size = 2
let _
=
unsafe
{
    <Dst
as
TransmuteFrom<u8, { Assume::SAFETY }>>::transmute(src)
};
§
Caveats
§
Portability
Implementations of this trait do not provide any guarantee of portability
across toolchains, targets or compilations. This trait may be implemented
for certain combinations of
Src
,
Self
and
ASSUME
on some toolchains,
targets or compilations, but not others. For example, if the layouts of
Src
or
Self
are non-deterministic, the presence or absence of an
implementation of this trait may also be non-deterministic. Even if
Src
and
Self
have deterministic layouts (e.g., they are
repr(C)
structs),
Rust does not specify the alignments of its primitive integer types, and
layouts that involve these types may vary across toolchains, targets or
compilations.
§
Stability
Implementations of this trait do not provide any guarantee of SemVer
stability across the crate versions that define the
Src
and
Self
types.
If SemVer stability is crucial to your application, you must consult the
documentation of
Src
and
Self
s’ defining crates. Note that the presence
of
repr(C)
, alone, does not carry a safety invariant of SemVer stability.
Furthermore, stability does not imply portability. For example, the size of
usize
is stable, but not portable.
Provided Methods
§
Source
unsafe fn
transmute
(src: Src) -> Self
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
transmutability
#99571
)
Transmutes a
Src
value into a
Self
.
§
Safety
The safety obligations of the caller depend on the value of
ASSUME
:
If
ASSUME.alignment
, the caller must guarantee
that the addresses of references in the returned
Self
satisfy the
alignment requirements of their referent types.
If
ASSUME.lifetimes
, the caller must guarantee
that references in the returned
Self
will not outlive their
referents.
If
ASSUME.safety
, the returned value might not
satisfy the library safety invariants of
Self
, and the caller must
guarantee that undefined behavior does not arise from uses of the
returned value.
If
ASSUME.validity
, the caller must guarantee
that
src
is a bit-valid instance of
Self
.
When satisfying the above obligations (if any), the caller must
not
assume that this trait provides any inherent guarantee of layout
portability
or
stability
.
Implementors
§