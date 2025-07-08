reference - Rust
Primitive Type
reference
Copy item path
1.0.0
Expand description
References,
&T
and
&mut T
.
A reference represents a borrow of some owned value. You can get one by using the
&
or
&mut
operators on a value, or by using a
ref
or
ref
mut
pattern.
For those familiar with pointers, a reference is just a pointer that is assumed to be
aligned, not null, and pointing to memory containing a valid value of
T
- for example,
&
bool
can only point to an allocation containing the integer values
1
(
true
) or
0
(
false
), but
creating a
&
bool
that points to an allocation containing
the value
3
causes undefined behavior.
In fact,
Option
<&T>
has the same memory representation as a
nullable but aligned pointer, and can be passed across FFI boundaries as such.
In most cases, references can be used much like the original value. Field access, method
calling, and indexing work the same (save for mutability rules, of course). In addition, the
comparison operators transparently defer to the referent’s implementation, allowing references
to be compared the same as owned values.
References have a lifetime attached to them, which represents the scope for which the borrow is
valid. A lifetime is said to “outlive” another one if its representative scope is as long or
longer than the other. The
'static
lifetime is the longest lifetime, which represents the
total life of the program. For example, string literals have a
'static
lifetime because the
text data is embedded into the binary of the program, rather than in an allocation that needs
to be dynamically managed.
&mut T
references can be freely coerced into
&T
references with the same referent type, and
references with longer lifetimes can be freely coerced into references with shorter ones.
Reference equality by address, instead of comparing the values pointed to, is accomplished via
implicit reference-pointer coercion and raw pointer equality via
ptr::eq
, while
PartialEq
compares values.
use
std::ptr;
let
five =
5
;
let
other_five =
5
;
let
five_ref =
&
five;
let
same_five_ref =
&
five;
let
other_five_ref =
&
other_five;
assert!
(five_ref == same_five_ref);
assert!
(five_ref == other_five_ref);
assert!
(ptr::eq(five_ref, same_five_ref));
assert!
(!ptr::eq(five_ref, other_five_ref));
For more information on how to use references, see
the book’s section on “References and
Borrowing”
.
§
Trait implementations
The following traits are implemented for all
&T
, regardless of the type of its referent:
Copy
Clone
(Note that this will not defer to
T
’s
Clone
implementation if it exists!)
Deref
Borrow
fmt::Pointer
&mut T
references get all of the above except
Copy
and
Clone
(to prevent creating
multiple simultaneous mutable borrows), plus the following, regardless of the type of its
referent:
DerefMut
BorrowMut
The following traits are implemented on
&T
references if the underlying
T
also implements
that trait:
All the traits in
std::fmt
except
fmt::Pointer
(which is implemented regardless of the type of its referent) and
fmt::Write
PartialOrd
Ord
PartialEq
Eq
AsRef
Fn
(in addition,
&T
references get
FnMut
and
FnOnce
if
T: Fn
)
Hash
ToSocketAddrs
Sync
&mut T
references get all of the above except
ToSocketAddrs
, plus the following, if
T
implements that trait:
AsMut
FnMut
(in addition,
&mut T
references get
FnOnce
if
T: FnMut
)
fmt::Write
Iterator
DoubleEndedIterator
ExactSizeIterator
FusedIterator
TrustedLen
Send
io::Write
Read
Seek
BufRead
In addition,
&T
references implement
Send
if and only if
T
implements
Sync
.
Note that due to method call deref coercion, simply calling a trait method will act like they
work on references as well as they do on owned values! The implementations described here are
meant for generic contexts, where the final type
T
is a type parameter or otherwise not
locally known.
§
Safety
For all types,
T: ?Sized
, and for all
t: &T
or
t: &mut T
, when such values cross an API
boundary, the following invariants must generally be upheld:
t
is non-null
t
is aligned to
align_of_val(t)
if
size_of_val(t) > 0
, then
t
is dereferenceable for
size_of_val(t)
many bytes
If
t
points at address
a
, being “dereferenceable” for N bytes means that the memory range
[a, a + N)
is all contained within a single
allocated object
.
For instance, this means that unsafe code in a safe function may assume these invariants are
ensured of arguments passed by the caller, and it may assume that these invariants are ensured
of return values from any safe functions it calls.
For the other direction, things are more complicated: when unsafe code passes arguments
to safe functions or returns values from safe functions, they generally must
at least
not violate these invariants. The full requirements are stronger, as the reference generally
must point to data that is safe to use at type
T
.
It is not decided yet whether unsafe code may violate these invariants temporarily on internal
data. As a consequence, unsafe code which violates these invariants temporarily on internal data
may be unsound or become unsound in future versions of Rust depending on how this question is
decided.
Trait Implementations
§
1.0.0
·
Source
§
impl<A, B>
PartialEq
<
&B
> for
&A
where
    A:
PartialEq
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
eq
(&self, other: &
&B
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
&B
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
impl<A, B>
PartialEq
<
&B
> for
&mut A
where
    A:
PartialEq
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
eq
(&self, other: &
&B
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
&B
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
impl<A, B>
PartialEq
<
&mut B
> for
&A
where
    A:
PartialEq
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
eq
(&self, other: &
&mut B
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
&mut B
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
impl<A, B>
PartialEq
<
&mut B
> for
&mut A
where
    A:
PartialEq
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
eq
(&self, other: &
&mut B
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
&mut B
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
impl<A, B>
PartialOrd
<
&B
> for
&A
where
    A:
PartialOrd
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
&B
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
&B
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
&B
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
&B
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
&B
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
impl<A, B>
PartialOrd
<
&mut B
> for
&mut A
where
    A:
PartialOrd
<B> + ?
Sized
,
    B: ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
&mut B
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
&mut B
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
&mut B
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
&mut B
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
&mut B
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
impl<'a, 'b, T, U>
CoerceUnsized
<
&'a U
> for
&'b T
where
    'b: 'a,
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
impl<'a, 'b, T, U>
CoerceUnsized
<
&'a U
> for
&'b mut T
where
    'b: 'a,
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
&'a mut U
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
impl<'a, T, U>
DispatchFromDyn
<
&'a U
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
DispatchFromDyn
<
&'a mut U
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