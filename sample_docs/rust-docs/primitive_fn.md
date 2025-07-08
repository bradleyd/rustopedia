fn - Rust
Primitive Type
fn
Copy item path
1.0.0
Expand description
Function pointers, like
fn(usize) -> bool
.
See also the traits
Fn
,
FnMut
, and
FnOnce
.
Function pointers are pointers that point to
code
, not data. They can be called
just like functions. Like references, function pointers are, among other things, assumed to
not be null, so if you want to pass a function pointer over FFI and be able to accommodate null
pointers, make your type
Option<fn()>
with your required signature.
Note that FFI requires additional care to ensure that the ABI for both sides of the call match.
The exact requirements are not currently documented.
§
Safety
Plain function pointers are obtained by casting either plain functions, or closures that don’t
capture an environment:
fn
add_one(x: usize) -> usize {
    x +
1
}
let
ptr:
fn
(usize) -> usize = add_one;
assert_eq!
(ptr(
5
),
6
);
let
clos:
fn
(usize) -> usize = |x| x +
5
;
assert_eq!
(clos(
5
),
10
);
In addition to varying based on their signature, function pointers come in two flavors: safe
and unsafe. Plain
fn()
function pointers can only point to safe functions,
while
unsafe fn()
function pointers can point to safe or unsafe functions.
fn
add_one(x: usize) -> usize {
    x +
1
}
unsafe fn
add_one_unsafely(x: usize) -> usize {
    x +
1
}
let
safe_ptr:
fn
(usize) -> usize = add_one;
//ERROR: mismatched types: expected normal fn, found unsafe fn
//let bad_ptr: fn(usize) -> usize = add_one_unsafely;
let
unsafe_ptr:
unsafe fn
(usize) -> usize = add_one_unsafely;
let
really_safe_ptr:
unsafe fn
(usize) -> usize = add_one;
§
ABI
On top of that, function pointers can vary based on what ABI they use. This
is achieved by adding the
extern
keyword before the type, followed by the
ABI in question. The default ABI is “Rust”, i.e.,
fn()
is the exact same
type as
extern "Rust" fn()
. A pointer to a function with C ABI would have
type
extern "C" fn()
.
extern "ABI" { ... }
blocks declare functions with ABI “ABI”. The default
here is “C”, i.e., functions declared in an
extern {...}
block have “C”
ABI.
For more information and a list of supported ABIs, see
the nomicon’s
section on foreign calling conventions
.
§
Variadic functions
Extern function declarations with the “C” or “cdecl” ABIs can also be
variadic
, allowing them
to be called with a variable number of arguments. Normal Rust functions, even those with an
extern "ABI"
, cannot be variadic. For more information, see
the nomicon’s section on
variadic functions
.
§
Creating function pointers
When
bar
is the name of a function, then the expression
bar
is
not
a
function pointer. Rather, it denotes a value of an unnameable type that
uniquely identifies the function
bar
. The value is zero-sized because the
type already identifies the function. This has the advantage that “calling”
the value (it implements the
Fn*
traits) does not require dynamic
dispatch.
This zero-sized type
coerces
to a regular function pointer. For example:
fn
bar(x: i32) {}
let
not_bar_ptr = bar;
// `not_bar_ptr` is zero-sized, uniquely identifying `bar`
assert_eq!
(size_of_val(
&
not_bar_ptr),
0
);
let
bar_ptr:
fn
(i32) = not_bar_ptr;
// force coercion to function pointer
assert_eq!
(size_of_val(
&
bar_ptr), size_of::<usize>());
let
footgun =
&
bar;
// this is a shared reference to the zero-sized type identifying `bar`
The last line shows that
&bar
is not a function pointer either. Rather, it
is a reference to the function-specific ZST.
&bar
is basically never what you
want when
bar
is a function.
§
Casting to and from integers
You can cast function pointers directly to integers:
let
fnptr:
fn
(i32) -> i32 = |x| x+
2
;
let
fnptr_addr = fnptr
as
usize;
However, a direct cast back is not possible. You need to use
transmute
:
let
fnptr = fnptr_addr
as
*const
();
let
fnptr:
fn
(i32) -> i32 =
unsafe
{ std::mem::transmute(fnptr) };
assert_eq!
(fnptr(
40
),
42
);
Crucially, we
as
-cast to a raw pointer before
transmute
ing to a function pointer.
This avoids an integer-to-pointer
transmute
, which can be problematic.
Transmuting between raw pointers and function pointers (i.e., two pointer types) is fine.
Note that all of this is not portable to platforms where function pointers and data pointers
have different sizes.
§
ABI compatibility
Generally, when a function is declared with one signature and called via a function pointer with
a different signature, the two signatures must be
ABI-compatible
or else calling the function
via that function pointer is Undefined Behavior. ABI compatibility is a lot stricter than merely
having the same memory layout; for example, even if
i32
and
f32
have the same size and
alignment, they might be passed in different registers and hence not be ABI-compatible.
ABI compatibility as a concern only arises in code that alters the type of function pointers,
and code that imports functions via
extern
blocks. Altering the type of function pointers is
wildly unsafe (as in, a lot more unsafe than even
transmute_copy
), and
should only occur in the most exceptional circumstances. Most Rust code just imports functions
via
use
. So, most likely you do not have to worry about ABI compatibility.
But assuming such circumstances, what are the rules? For this section, we are only considering
the ABI of direct Rust-to-Rust calls (with both definition and callsite visible to the
Rust compiler), not linking in general – once functions are imported via
extern
blocks, there
are more things to consider that we do not go into here. Note that this also applies to
passing/calling functions across language boundaries via function pointers.
Nothing in this section should be taken as a guarantee for non-Rust-to-Rust calls, even with
types from
core::ffi
or
libc
.
For two signatures to be considered
ABI-compatible
, they must use a compatible ABI string,
must take the same number of arguments, and the individual argument types and the return types
must be ABI-compatible. The ABI string is declared via
extern "ABI" fn(...) -> ...
; note that
fn name(...) -> ...
implicitly uses the
"Rust"
ABI string and
extern fn name(...) -> ...
implicitly uses the
"C"
ABI string.
The ABI strings are guaranteed to be compatible if they are the same, or if the caller ABI
string is
$X-unwind
and the callee ABI string is
$X
, where
$X
is one of the following:
“C”, “aapcs”, “fastcall”, “stdcall”, “system”, “sysv64”, “thiscall”, “vectorcall”, “win64”.
The following types are guaranteed to be ABI-compatible:
*const T
,
*mut T
,
&T
,
&mut T
,
Box<T>
(specifically, only
Box<T, Global>
), and
NonNull<T>
are all ABI-compatible with each other for all
T
. They are also ABI-compatible
with each other for
different
T
if they have the same metadata type (
<T as Pointee>::Metadata
).
usize
is ABI-compatible with the
uN
integer type of the same size, and likewise
isize
is
ABI-compatible with the
iN
integer type of the same size.
char
is ABI-compatible with
u32
.
Any two
fn
(function pointer) types are ABI-compatible with each other if they have the same
ABI string or the ABI string only differs in a trailing
-unwind
, independent of the rest of
their signature. (This means you can pass
fn()
to a function expecting
fn(i32)
, and the
call will be valid ABI-wise. The callee receives the result of transmuting the function pointer
from
fn()
to
fn(i32)
; that transmutation is itself a well-defined operation, it’s just
almost certainly UB to later call that function pointer.)
Any two types with size 0 and alignment 1 are ABI-compatible.
A
repr(transparent)
type
T
is ABI-compatible with its unique non-trivial field, i.e., the
unique field that doesn’t have size 0 and alignment 1 (if there is such a field).
i32
is ABI-compatible with
NonZero<i32>
, and similar for all other integer types.
If
T
is guaranteed to be subject to the
null pointer
optimization
, and
E
is an enum satisfying the following
requirements, then
T
and
E
are ABI-compatible. Such an enum
E
is called “option-like”.
The enum
E
has exactly two variants.
One variant has exactly one field, of type
T
.
All fields of the other variant are zero-sized with 1-byte alignment.
Furthermore, ABI compatibility satisfies the following general properties:
Every type is ABI-compatible with itself.
If
T1
and
T2
are ABI-compatible and
T2
and
T3
are ABI-compatible, then so are
T1
and
T3
(i.e., ABI-compatibility is transitive).
If
T1
and
T2
are ABI-compatible, then so are
T2
and
T1
(i.e., ABI-compatibility is
symmetric).
More signatures can be ABI-compatible on specific targets, but that should not be relied upon
since it is not portable and not a stable guarantee.
Noteworthy cases of types
not
being ABI-compatible in general are:
bool
vs
u8
,
i32
vs
u32
,
char
vs
i32
: on some targets, the calling conventions for
these types differ in terms of what they guarantee for the remaining bits in the register that
are not used by the value.
i32
vs
f32
are not compatible either, as has already been mentioned above.
struct Foo(u32)
and
u32
are not compatible (without
repr(transparent)
) since structs are
aggregate types and often passed in a different way than primitives like
i32
.
Note that these rules describe when two completely known types are ABI-compatible. When
considering ABI compatibility of a type declared in another crate (including the standard
library), consider that any type that has a private field or the
#[non_exhaustive]
attribute
may change its layout as a non-breaking update unless documented otherwise – so for instance,
even if such a type is a 1-ZST or
repr(transparent)
right now, this might change with any
library version bump.
If the declared signature and the signature of the function pointer are ABI-compatible, then the
function call behaves as if every argument was
transmute
d
from the
type in the function pointer to the type at the function declaration, and the return value is
transmute
d
from the type in the declaration to the type in the
pointer. All the usual caveats and concerns around transmutation apply; for instance, if the
function expects a
NonZero<i32>
and the function pointer uses the ABI-compatible type
Option<NonZero<i32>>
, and the value used for the argument is
None
, then this call is Undefined
Behavior since transmuting
None::<NonZero<i32>>
to
NonZero<i32>
violates the non-zero
requirement.
§
Trait implementations
In this documentation the shorthand
fn(T₁, T₂, …, Tₙ)
is used to represent non-variadic
function pointers of varying length. Note that this is a convenience notation to avoid
repetitive documentation, not valid Rust syntax.
The following traits are implemented for function pointers with any number of arguments and
any ABI.
PartialEq
Eq
PartialOrd
Ord
Hash
Pointer
Debug
Clone
Copy
Send
Sync
Unpin
UnwindSafe
RefUnwindSafe
Note that while this type implements
PartialEq
, comparing function pointers is unreliable:
pointers to the same function can compare inequal (because functions are duplicated in multiple
codegen units), and pointers to
different
functions can compare equal (since identical
functions can be deduplicated within a codegen unit).
In addition, all
safe
function pointers implement
Fn
,
FnMut
, and
FnOnce
, because
these traits are specially known to the compiler.
Auto Trait Implementations
§
§
impl<Ret, T>
Freeze
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<Ret, T>
RefUnwindSafe
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<Ret, T>
Send
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<Ret, T>
Sync
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<Ret, T>
Unpin
for
fn(T₁, T₂, …, Tₙ)
-> Ret
§
impl<Ret, T>
UnwindSafe
for
fn(T₁, T₂, …, Tₙ)
-> Ret
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
impl<F>
Debug
for F
where
    F:
FnPtr
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
impl<F>
Hash
for F
where
    F:
FnPtr
,
Source
§
fn
hash
<HH>(&self, state:
&mut HH
)
where
    HH:
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
impl<F>
Ord
for F
where
    F:
FnPtr
,
Source
§
fn
cmp
(&self, other:
&F
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
impl<F>
PartialEq
for F
where
    F:
FnPtr
,
Source
§
fn
eq
(&self, other:
&F
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
impl<F>
PartialOrd
for F
where
    F:
FnPtr
,
Source
§
fn
partial_cmp
(&self, other:
&F
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
impl<F>
Pattern
for F
where
    F:
FnMut
(
char
) ->
bool
,
Source
§
type
Searcher
<'a> =
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
<'a>(self, haystack: &'a
str
) ->
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
Source
§
impl<F>
Pointer
for F
where
    F:
FnPtr
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
Source
§
impl<F>
Eq
for F
where
    F:
FnPtr
,