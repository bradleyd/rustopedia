unsafe - Rust
Keyword
unsafe
Copy item path
Source
Expand description
Code or interfaces whose
memory safety
cannot be verified by the type
system.
The
unsafe
keyword has two uses:
to declare the existence of contracts the compiler can’t check (
unsafe fn
and
unsafe trait
),
and to declare that a programmer has checked that these contracts have been upheld (
unsafe {}
and
unsafe impl
, but also
unsafe fn
– see below).
They are not mutually exclusive, as can be seen in
unsafe fn
: the body of an
unsafe fn
is,
by default, treated like an unsafe block. The
unsafe_op_in_unsafe_fn
lint can be enabled to
change that.
§
Unsafe abilities
No matter what, Safe Rust can’t cause Undefined Behavior
. This is
referred to as
soundness
: a well-typed program actually has the desired
properties. The
Nomicon
has a more detailed explanation
on the subject.
To ensure soundness, Safe Rust is restricted enough that it can be
automatically checked. Sometimes, however, it is necessary to write code
that is correct for reasons which are too clever for the compiler to
understand. In those cases, you need to use Unsafe Rust.
Here are the abilities Unsafe Rust has in addition to Safe Rust:
Dereference
raw pointers
Implement
unsafe
trait
s
Call
unsafe
functions
Mutate
static
s (including
extern
al ones)
Access fields of
union
s
However, this extra power comes with extra responsibilities: it is now up to
you to ensure soundness. The
unsafe
keyword helps by clearly marking the
pieces of code that need to worry about this.
§
The different meanings of
unsafe
Not all uses of
unsafe
are equivalent: some are here to mark the existence
of a contract the programmer must check, others are to say “I have checked
the contract, go ahead and do this”. The following
discussion on Rust Internals
has more in-depth explanations about this but
here is a summary of the main points:
unsafe fn
: calling this function means abiding by a contract the
compiler cannot enforce.
unsafe trait
: implementing the
trait
means abiding by a
contract the compiler cannot enforce.
unsafe {}
: the contract necessary to call the operations inside the
block has been checked by the programmer and is guaranteed to be respected.
unsafe impl
: the contract necessary to implement the trait has been
checked by the programmer and is guaranteed to be respected.
By default,
unsafe fn
also acts like an
unsafe {}
block
around the code inside the function. This means it is not just a signal to
the caller, but also promises that the preconditions for the operations
inside the function are upheld. Mixing these two meanings can be confusing, so the
unsafe_op_in_unsafe_fn
lint can be enabled to warn against that and require explicit unsafe
blocks even inside
unsafe fn
.
See the
Rustonomicon
and the
Reference
for more information.
§
Examples
§
Marking elements as
unsafe
unsafe
can be used on functions. Note that functions and statics declared
in
extern
blocks are implicitly marked as
unsafe
(but not functions
declared as
extern "something" fn ...
). Mutable statics are always unsafe,
wherever they are declared. Methods can also be declared as
unsafe
:
static
mut
FOO:
&
str =
"hello"
;
unsafe fn
unsafe_fn() {}
unsafe extern
"C"
{
fn
unsafe_extern_fn();
static
BAR:
*mut
u32;
}
trait
SafeTraitWithUnsafeMethod {
unsafe fn
unsafe_method(
&
self
);
}
struct
S;
impl
S {
unsafe fn
unsafe_method_on_struct() {}
}
Traits can also be declared as
unsafe
:
unsafe trait
UnsafeTrait {}
Since
unsafe fn
and
unsafe trait
indicate that there is a safety
contract that the compiler cannot enforce, documenting it is important. The
standard library has many examples of this, like the following which is an
extract from
Vec::set_len
. The
# Safety
section explains the contract
that must be fulfilled to safely call the function.
ⓘ
/// Forces the length of the vector to `new_len`.
///
/// This is a low-level operation that maintains none of the normal
/// invariants of the type. Normally changing the length of a vector
/// is done using one of the safe operations instead, such as
/// `truncate`, `resize`, `extend`, or `clear`.
///
/// # Safety
///
/// - `new_len` must be less than or equal to `capacity()`.
/// - The elements at `old_len..new_len` must be initialized.
pub unsafe fn
set_len(
&mut
self
, new_len: usize)
§
Using
unsafe {}
blocks and
impl
s
Performing
unsafe
operations requires an
unsafe {}
block:
#![deny(unsafe_op_in_unsafe_fn)]
/// Dereference the given pointer.
///
/// # Safety
///
/// `ptr` must be aligned and must not be dangling.
unsafe fn
deref_unchecked(ptr:
*const
i32) -> i32 {
// SAFETY: the caller is required to ensure that `ptr` is aligned and dereferenceable.
unsafe
{
*
ptr }
}
let
a =
3
;
let
b =
&
a
as
*const
_
;
// SAFETY: `a` has not been dropped and references are always aligned,
// so `b` is a valid address.
unsafe
{
assert_eq!
(
*
b, deref_unchecked(b)); };
§
unsafe
and traits
The interactions of
unsafe
and traits can be surprising, so let us contrast the
two combinations of safe
fn
in
unsafe trait
and
unsafe fn
in safe trait using two
examples:
/// # Safety
///
/// `make_even` must return an even number.
unsafe trait
MakeEven {
fn
make_even(
&
self
) -> i32;
}
// SAFETY: Our `make_even` always returns something even.
unsafe impl
MakeEven
for
i32 {
fn
make_even(
&
self
) -> i32 {
self
<<
1
}
}
fn
use_make_even(x:
impl
MakeEven) {
if
x.make_even() %
2
==
1
{
// SAFETY: this can never happen, because all `MakeEven` implementations
        // ensure that `make_even` returns something even.
unsafe
{ std::hint::unreachable_unchecked() };
    }
}
Note how the safety contract of the trait is upheld by the implementation, and is itself used to
uphold the safety contract of the unsafe function
unreachable_unchecked
called by
use_make_even
.
make_even
itself is a safe function because its
callers
do not have to
worry about any contract, only the
implementation
of
MakeEven
is required to uphold a
certain contract.
use_make_even
is safe because it can use the promise made by
MakeEven
implementations to uphold the safety contract of the
unsafe fn unreachable_unchecked
it calls.
It is also possible to have
unsafe fn
in a regular safe
trait
:
#![deny(unsafe_op_in_unsafe_fn)]
trait
Indexable {
const
LEN: usize;
/// # Safety
    ///
    /// The caller must ensure that `idx < LEN`.
unsafe fn
idx_unchecked(
&
self
, idx: usize) -> i32;
}
// The implementation for `i32` doesn't need to do any contract reasoning.
impl
Indexable
for
i32 {
const
LEN: usize =
1
;
unsafe fn
idx_unchecked(
&
self
, idx: usize) -> i32 {
debug_assert_eq!
(idx,
0
);
*
self
}
}
// The implementation for arrays exploits the function contract to
// make use of `get_unchecked` on slices and avoid a run-time check.
impl
Indexable
for
[i32;
42
] {
const
LEN: usize =
42
;
unsafe fn
idx_unchecked(
&
self
, idx: usize) -> i32 {
// SAFETY: As per this trait's documentation, the caller ensures
        // that `idx < 42`.
unsafe
{
*
self
.get_unchecked(idx) }
    }
}
// The implementation for the never type declares a length of 0,
// which means `idx_unchecked` can never be called.
impl
Indexable
for
! {
const
LEN: usize =
0
;
unsafe fn
idx_unchecked(
&
self
, idx: usize) -> i32 {
// SAFETY: As per this trait's documentation, the caller ensures
        // that `idx < 0`, which is impossible, so this is dead code.
unsafe
{ std::hint::unreachable_unchecked() }
    }
}
fn
use_indexable<I: Indexable>(x: I, idx: usize) -> i32 {
if
idx < I::LEN {
// SAFETY: We have checked that `idx < I::LEN`.
unsafe
{ x.idx_unchecked(idx) }
    }
else
{
panic!
(
"index out-of-bounds"
)
    }
}
This time,
use_indexable
is safe because it uses a run-time check to discharge the safety
contract of
idx_unchecked
. Implementing
Indexable
is safe because when writing
idx_unchecked
, we don’t have to worry: our
callers
need to discharge a proof obligation
(like
use_indexable
does), but the
implementation
of
get_unchecked
has no proof obligation
to contend with. Of course, the implementation of
Indexable
may choose to call other unsafe
operations, and then it needs an
unsafe
block
to indicate it discharged the proof
obligations of its callees. (We enabled
unsafe_op_in_unsafe_fn
, so the body of
idx_unchecked
is not implicitly an unsafe block.) For that purpose it can make use of the contract that all
its callers must uphold – the fact that
idx < LEN
.
Formally speaking, an
unsafe fn
in a trait is a function with
preconditions
that go beyond
those encoded by the argument types (such as
idx < LEN
), whereas an
unsafe trait
can declare
that some of its functions have
postconditions
that go beyond those encoded in the return type
(such as returning an even integer). If a trait needs a function with both extra precondition
and extra postcondition, then it needs an
unsafe fn
in an
unsafe trait
.