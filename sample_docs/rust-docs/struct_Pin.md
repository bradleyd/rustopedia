Pin in std::pin - Rust
std
::
pin
Struct
Pin
Copy item path
1.33.0
·
Source
#[repr(transparent)]
pub struct Pin<Ptr> {
/* private fields */
}
Expand description
A pointer which pins its pointee in place.
Pin
is a wrapper around some kind of pointer
Ptr
which makes that pointer “pin” its
pointee value in place, thus preventing the value referenced by that pointer from being moved
or otherwise invalidated at that place in memory unless it implements
Unpin
.
See the
pin
module
documentation for a more thorough exploration of pinning.
§
Pinning values with
Pin<Ptr>
In order to pin a value, we wrap a
pointer to that value
(of some type
Ptr
) in a
Pin<Ptr>
.
Pin<Ptr>
can wrap any pointer type, forming a promise that the
pointee
will not be
moved
or
otherwise invalidated
. If the pointee value’s type
implements
Unpin
, we are free to disregard these requirements entirely and can wrap any
pointer to that value in
Pin
directly via
Pin::new
. If the pointee value’s type does
not implement
Unpin
, then Rust will not let us use the
Pin::new
function directly and
we’ll need to construct a
Pin
-wrapped pointer in one of the more specialized manners
discussed below.
We call such a
Pin
-wrapped pointer a
pinning pointer
(or pinning ref, or pinning
Box
, etc.) because its existence is the thing that is pinning the underlying pointee in
place: it is the metaphorical “pin” securing the data in place on the pinboard (in memory).
It is important to stress that the thing in the
Pin
is not the value which we want to pin
itself, but rather a pointer to that value! A
Pin<Ptr>
does not pin the
Ptr
but rather
the pointer’s
pointee
value
.
The most common set of types which require pinning related guarantees for soundness are the
compiler-generated state machines that implement
Future
for the return value of
async fn
s. These compiler-generated
Future
s may contain self-referential pointers, one
of the most common use cases for
Pin
. More details on this point are provided in the
pin
module
docs, but suffice it to say they require the guarantees provided by pinning to
be implemented soundly.
This requirement for the implementation of
async fn
s means that the
Future
trait
requires all calls to
poll
to use a
self:
Pin
<&mut Self>
parameter instead
of the usual
&mut self
. Therefore, when manually polling a future, you will need to pin it
first.
You may notice that
async fn
-sourced
Future
s are only a small percentage of all
Future
s that exist, yet we had to modify the signature of
poll
for all
Future
s
to accommodate them. This is unfortunate, but there is a way that the language attempts to
alleviate the extra friction that this API choice incurs: the
Unpin
trait.
The vast majority of Rust types have no reason to ever care about being pinned. These
types implement the
Unpin
trait, which entirely opts all values of that type out of
pinning-related guarantees. For values of these types, pinning a value by pointing to it with a
Pin<Ptr>
will have no actual effect.
The reason this distinction exists is exactly to allow APIs like
Future::poll
to take a
Pin<Ptr>
as an argument for all types while only forcing
Future
types that actually
care about pinning guarantees pay the ergonomics cost. For the majority of
Future
types
that don’t have a reason to care about being pinned and therefore implement
Unpin
, the
Pin
<&mut Self>
will act exactly like a regular
&mut Self
, allowing direct
access to the underlying value. Only types that
don’t
implement
Unpin
will be restricted.
§
Pinning a value of a type that implements
Unpin
If the type of the value you need to “pin” implements
Unpin
, you can trivially wrap any
pointer to that value in a
Pin
by calling
Pin::new
.
use
std::pin::Pin;
// Create a value of a type that implements `Unpin`
let
mut
unpin_future = std::future::ready(
5
);
// Pin it by creating a pinning mutable reference to it (ready to be `poll`ed!)
let
my_pinned_unpin_future: Pin<
&mut
_
> = Pin::new(
&mut
unpin_future);
§
Pinning a value inside a
Box
The simplest and most flexible way to pin a value that does not implement
Unpin
is to put
that value inside a
Box
and then turn that
Box
into a “pinning
Box
” by wrapping it
in a
Pin
. You can do both of these in a single step using
Box::pin
. Let’s see an
example of using this flow to pin a
Future
returned from calling an
async fn
, a common
use case as described above.
use
std::pin::Pin;
async fn
add_one(x: u32) -> u32 {
    x +
1
}
// Call the async function to get a future back
let
fut = add_one(
42
);
// Pin the future inside a pinning box
let
pinned_fut: Pin<Box<
_
>> = Box::pin(fut);
If you have a value which is already boxed, for example a
Box<dyn Future>
, you can pin
that value in-place at its current memory address using
Box::into_pin
.
use
std::pin::Pin;
use
std::future::Future;
async fn
add_one(x: u32) -> u32 {
    x +
1
}
fn
boxed_add_one(x: u32) -> Box<
dyn
Future<Output = u32>> {
    Box::new(add_one(x))
}
let
boxed_fut = boxed_add_one(
42
);
// Pin the future inside the existing box
let
pinned_fut: Pin<Box<
_
>> = Box::into_pin(boxed_fut);
There are similar pinning methods offered on the other standard library smart pointer types
as well, like
Rc
and
Arc
.
§
Pinning a value on the stack using
pin!
There are some situations where it is desirable or even required (for example, in a
#[no_std]
context where you don’t have access to the standard library or allocation in general) to
pin a value which does not implement
Unpin
to its location on the stack. Doing so is
possible using the
pin!
macro. See its documentation for more.
§
Layout and ABI
Pin<Ptr>
is guaranteed to have the same memory layout and ABI
1
as
Ptr
.
There is a bit of nuance here that is still being decided about whether the
aliasing semantics of
Pin<&mut T>
should be different than
&mut T
, but this is true as of
today.
↩
Implementations
§
Source
§
impl<Ptr>
Pin
<Ptr>
where
    Ptr:
Deref
,
    <Ptr as
Deref
>::
Target
:
Unpin
,
1.33.0 (const: 1.84.0)
·
Source
pub const fn
new
(pointer: Ptr) ->
Pin
<Ptr>
Constructs a new
Pin<Ptr>
around a pointer to some data of a type that
implements
Unpin
.
Unlike
Pin::new_unchecked
, this method is safe because the pointer
Ptr
dereferences to an
Unpin
type, which cancels the pinning guarantees.
§
Examples
use
std::pin::Pin;
let
mut
val: u8 =
5
;
// Since `val` doesn't care about being moved, we can safely create a "facade" `Pin`
// which will allow `val` to participate in `Pin`-bound apis  without checking that
// pinning guarantees are actually upheld.
let
mut
pinned: Pin<
&mut
u8> = Pin::new(
&mut
val);
1.39.0 (const: 1.84.0)
·
Source
pub const fn
into_inner
(pin:
Pin
<Ptr>) -> Ptr
Unwraps this
Pin<Ptr>
, returning the underlying pointer.
Doing this operation safely requires that the data pointed at by this pinning pointer
implements
Unpin
so that we can ignore the pinning invariants when unwrapping it.
§
Examples
use
std::pin::Pin;
let
mut
val: u8 =
5
;
let
pinned: Pin<
&mut
u8> = Pin::new(
&mut
val);
// Unwrap the pin to get the underlying mutable reference to the value. We can do
// this because `val` doesn't care about being moved, so the `Pin` was just
// a "facade" anyway.
let
r = Pin::into_inner(pinned);
assert_eq!
(
*
r,
5
);
Source
§
impl<Ptr>
Pin
<Ptr>
where
    Ptr:
Deref
,
1.33.0 (const: 1.84.0)
·
Source
pub const unsafe fn
new_unchecked
(pointer: Ptr) ->
Pin
<Ptr>
Constructs a new
Pin<Ptr>
around a reference to some data of a type that
may or may not implement
Unpin
.
If
pointer
dereferences to an
Unpin
type,
Pin::new
should be used
instead.
§
Safety
This constructor is unsafe because we cannot guarantee that the data
pointed to by
pointer
is pinned. At its core, pinning a value means making the
guarantee that the value’s data will not be moved nor have its storage invalidated until
it gets dropped. For a more thorough explanation of pinning, see the
pin
module docs
.
If the caller that is constructing this
Pin<Ptr>
does not ensure that the data
Ptr
points to is pinned, that is a violation of the API contract and may lead to undefined
behavior in later (even safe) operations.
By using this method, you are also making a promise about the
Deref
,
DerefMut
, and
Drop
implementations of
Ptr
, if they exist. Most importantly, they
must not move out of their
self
arguments:
Pin::as_mut
and
Pin::as_ref
will call
DerefMut::deref_mut
and
Deref::deref
on the pointer type
Ptr
and expect these methods to uphold the pinning invariants.
Moreover, by calling this method you promise that the reference
Ptr
dereferences to will not be moved out of again; in particular, it
must not be possible to obtain a
&mut Ptr::Target
and then
move out of that reference (using, for example
mem::swap
).
For example, calling
Pin::new_unchecked
on an
&'a mut T
is unsafe because
while you are able to pin it for the given lifetime
'a
, you have no control
over whether it is kept pinned once
'a
ends, and therefore cannot uphold the
guarantee that a value, once pinned, remains pinned until it is dropped:
use
std::mem;
use
std::pin::Pin;
fn
move_pinned_ref<T>(
mut
a: T,
mut
b: T) {
unsafe
{
let
p: Pin<
&mut
T> = Pin::new_unchecked(
&mut
a);
// This should mean the pointee `a` can never move again.
}
    mem::swap(
&mut
a,
&mut
b);
// Potential UB down the road ⚠️
    // The address of `a` changed to `b`'s stack slot, so `a` got moved even
    // though we have previously pinned it! We have violated the pinning API contract.
}
A value, once pinned, must remain pinned until it is dropped (unless its type implements
Unpin
). Because
Pin<&mut T>
does not own the value, dropping the
Pin
will not drop
the value and will not end the pinning contract. So moving the value after dropping the
Pin<&mut T>
is still a violation of the API contract.
Similarly, calling
Pin::new_unchecked
on an
Rc<T>
is unsafe because there could be
aliases to the same data that are not subject to the pinning restrictions:
use
std::rc::Rc;
use
std::pin::Pin;
fn
move_pinned_rc<T>(
mut
x: Rc<T>) {
// This should mean the pointee can never move again.
let
pin =
unsafe
{ Pin::new_unchecked(Rc::clone(
&
x)) };
    {
let
p: Pin<
&
T> = pin.as_ref();
// ...
}
    drop(pin);
let
content = Rc::get_mut(
&mut
x).unwrap();
// Potential UB down the road ⚠️
    // Now, if `x` was the only reference, we have a mutable reference to
    // data that we pinned above, which we could use to move it as we have
    // seen in the previous example. We have violated the pinning API contract.
}
§
Pinning of closure captures
Particular care is required when using
Pin::new_unchecked
in a closure:
Pin::new_unchecked(&mut var)
where
var
is a by-value (moved) closure capture
implicitly makes the promise that the closure itself is pinned, and that
all
uses
of this closure capture respect that pinning.
use
std::pin::Pin;
use
std::task::Context;
use
std::future::Future;
fn
move_pinned_closure(
mut
x:
impl
Future, cx:
&mut
Context<
'_
>) {
// Create a closure that moves `x`, and then internally uses it in a pinned way.
let
mut
closure =
move
||
unsafe
{
let
_ignore = Pin::new_unchecked(
&mut
x).poll(cx);
    };
// Call the closure, so the future can assume it has been pinned.
closure();
// Move the closure somewhere else. This also moves `x`!
let
mut
moved = closure;
// Calling it again means we polled the future from two different locations,
    // violating the pinning API contract.
moved();
// Potential UB ⚠️
}
When passing a closure to another API, it might be moving the closure any time, so
Pin::new_unchecked
on closure captures may only be used if the API explicitly documents
that the closure is pinned.
The better alternative is to avoid all that trouble and do the pinning in the outer function
instead (here using the
pin!
macro):
use
std::pin::pin;
use
std::task::Context;
use
std::future::Future;
fn
move_pinned_closure(
mut
x:
impl
Future, cx:
&mut
Context<
'_
>) {
let
mut
x =
pin!
(x);
// Create a closure that captures `x: Pin<&mut _>`, which is safe to move.
let
mut
closure =
move
|| {
let
_ignore = x.as_mut().poll(cx);
    };
// Call the closure, so the future can assume it has been pinned.
closure();
// Move the closure somewhere else.
let
mut
moved = closure;
// Calling it again here is fine (except that we might be polling a future that already
    // returned `Poll::Ready`, but that is a separate problem).
moved();
}
1.33.0
·
Source
pub fn
as_ref
(&self) ->
Pin
<&<Ptr as
Deref
>::
Target
>
Gets a shared reference to the pinned value this
Pin
points to.
This is a generic method to go from
&Pin<Pointer<T>>
to
Pin<&T>
.
It is safe because, as part of the contract of
Pin::new_unchecked
,
the pointee cannot move after
Pin<Pointer<T>>
got created.
“Malicious” implementations of
Pointer::Deref
are likewise
ruled out by the contract of
Pin::new_unchecked
.
Source
§
impl<Ptr>
Pin
<Ptr>
where
    Ptr:
DerefMut
,
1.33.0
·
Source
pub fn
as_mut
(&mut self) ->
Pin
<&mut <Ptr as
Deref
>::
Target
>
Gets a mutable reference to the pinned value this
Pin<Ptr>
points to.
This is a generic method to go from
&mut Pin<Pointer<T>>
to
Pin<&mut T>
.
It is safe because, as part of the contract of
Pin::new_unchecked
,
the pointee cannot move after
Pin<Pointer<T>>
got created.
“Malicious” implementations of
Pointer::DerefMut
are likewise
ruled out by the contract of
Pin::new_unchecked
.
This method is useful when doing multiple calls to functions that consume the
pinning pointer.
§
Example
use
std::pin::Pin;
impl
Type {
fn
method(
self
: Pin<
&mut
Self
>) {
// do something
}
fn
call_method_twice(
mut
self
: Pin<
&mut
Self
>) {
// `method` consumes `self`, so reborrow the `Pin<&mut Self>` via `as_mut`.
self
.as_mut().method();
self
.as_mut().method();
    }
}
1.84.0
·
Source
pub fn
as_deref_mut
(
    self:
Pin
<&mut
Pin
<Ptr>>,
) ->
Pin
<&mut <Ptr as
Deref
>::
Target
>
Gets
Pin<&mut T>
to the underlying pinned value from this nested
Pin
-pointer.
This is a generic method to go from
Pin<&mut Pin<Pointer<T>>>
to
Pin<&mut T>
. It is
safe because the existence of a
Pin<Pointer<T>>
ensures that the pointee,
T
, cannot
move in the future, and this method does not enable the pointee to move. “Malicious”
implementations of
Ptr::DerefMut
are likewise ruled out by the contract of
Pin::new_unchecked
.
1.33.0
·
Source
pub fn
set
(&mut self, value: <Ptr as
Deref
>::
Target
)
where
    <Ptr as
Deref
>::
Target
:
Sized
,
Assigns a new value to the memory location pointed to by the
Pin<Ptr>
.
This overwrites pinned data, but that is okay: the original pinned value’s destructor gets
run before being overwritten and the new value is also a valid value of the same type, so
no pinning invariant is violated. See
the
pin
module documentation
for more information on how this upholds the pinning invariants.
§
Example
use
std::pin::Pin;
let
mut
val: u8 =
5
;
let
mut
pinned: Pin<
&mut
u8> = Pin::new(
&mut
val);
println!
(
"{}"
, pinned);
// 5
pinned.set(
10
);
println!
(
"{}"
, pinned);
// 10
Source
§
impl<Ptr>
Pin
<Ptr>
where
    Ptr:
Deref
,
1.39.0 (const: 1.84.0)
·
Source
pub const unsafe fn
into_inner_unchecked
(pin:
Pin
<Ptr>) -> Ptr
Unwraps this
Pin<Ptr>
, returning the underlying
Ptr
.
§
Safety
This function is unsafe. You must guarantee that you will continue to
treat the pointer
Ptr
as pinned after you call this function, so that
the invariants on the
Pin
type can be upheld. If the code using the
resulting
Ptr
does not continue to maintain the pinning invariants that
is a violation of the API contract and may lead to undefined behavior in
later (safe) operations.
Note that you must be able to guarantee that the data pointed to by
Ptr
will be treated as pinned all the way until its
drop
handler is complete!
For more information, see the
pin
module docs
If the underlying data is
Unpin
,
Pin::into_inner
should be used
instead.
Source
§
impl<'a, T>
Pin
<
&'a T
>
where
    T: ?
Sized
,
1.33.0
·
Source
pub unsafe fn
map_unchecked
<U, F>(self, func: F) ->
Pin
<
&'a U
>
where
    F:
FnOnce
(
&T
) ->
&U
,
    U: ?
Sized
,
Constructs a new pin by mapping the interior value.
For example, if you wanted to get a
Pin
of a field of something,
you could use this to get access to that field in one line of code.
However, there are several gotchas with these “pinning projections”;
see the
pin
module
documentation for further details on that topic.
§
Safety
This function is unsafe. You must guarantee that the data you return
will not move so long as the argument value does not move (for example,
because it is one of the fields of that value), and also that you do
not move out of the argument you receive to the interior function.
1.33.0 (const: 1.84.0)
·
Source
pub const fn
get_ref
(self) ->
&'a T
Gets a shared reference out of a pin.
This is safe because it is not possible to move out of a shared reference.
It may seem like there is an issue here with interior mutability: in fact,
it
is
possible to move a
T
out of a
&RefCell<T>
. However, this is
not a problem as long as there does not also exist a
Pin<&T>
pointing
to the inner
T
inside the
RefCell
, and
RefCell<T>
does not let you get a
Pin<&T>
pointer to its contents. See the discussion on
“pinning projections”
for further details.
Note:
Pin
also implements
Deref
to the target, which can be used
to access the inner value. However,
Deref
only provides a reference
that lives for as long as the borrow of the
Pin
, not the lifetime of
the reference contained in the
Pin
. This method allows turning the
Pin
into a reference
with the same lifetime as the reference it wraps.
Source
§
impl<'a, T>
Pin
<
&'a mut T
>
where
    T: ?
Sized
,
1.33.0 (const: 1.84.0)
·
Source
pub const fn
into_ref
(self) ->
Pin
<
&'a T
>
Converts this
Pin<&mut T>
into a
Pin<&T>
with the same lifetime.
1.33.0 (const: 1.84.0)
·
Source
pub const fn
get_mut
(self) ->
&'a mut T
where
    T:
Unpin
,
Gets a mutable reference to the data inside of this
Pin
.
This requires that the data inside this
Pin
is
Unpin
.
Note:
Pin
also implements
DerefMut
to the data, which can be used
to access the inner value. However,
DerefMut
only provides a reference
that lives for as long as the borrow of the
Pin
, not the lifetime of
the
Pin
itself. This method allows turning the
Pin
into a reference
with the same lifetime as the original
Pin
.
1.33.0 (const: 1.84.0)
·
Source
pub const unsafe fn
get_unchecked_mut
(self) ->
&'a mut T
Gets a mutable reference to the data inside of this
Pin
.
§
Safety
This function is unsafe. You must guarantee that you will never move
the data out of the mutable reference you receive when you call this
function, so that the invariants on the
Pin
type can be upheld.
If the underlying data is
Unpin
,
Pin::get_mut
should be used
instead.
1.33.0
·
Source
pub unsafe fn
map_unchecked_mut
<U, F>(self, func: F) ->
Pin
<
&'a mut U
>
where
    F:
FnOnce
(
&mut T
) ->
&mut U
,
    U: ?
Sized
,
Constructs a new pin by mapping the interior value.
For example, if you wanted to get a
Pin
of a field of something,
you could use this to get access to that field in one line of code.
However, there are several gotchas with these “pinning projections”;
see the
pin
module
documentation for further details on that topic.
§
Safety
This function is unsafe. You must guarantee that the data you return
will not move so long as the argument value does not move (for example,
because it is one of the fields of that value), and also that you do
not move out of the argument you receive to the interior function.
Source
§
impl<T>
Pin
<
&'static T
>
where
    T: ?
Sized
,
1.61.0 (const: 1.84.0)
·
Source
pub const fn
static_ref
(r:
&'static T
) ->
Pin
<
&'static T
>
Gets a pinning reference from a
&'static
reference.
This is safe because
T
is borrowed immutably for the
'static
lifetime, which
never ends.
Source
§
impl<T>
Pin
<
&'static mut T
>
where
    T: ?
Sized
,
1.61.0 (const: 1.84.0)
·
Source
pub const fn
static_mut
(r:
&'static mut T
) ->
Pin
<
&'static mut T
>
Gets a pinning mutable reference from a static mutable reference.
This is safe because
T
is borrowed for the
'static
lifetime, which
never ends.
Trait Implementations
§
Source
§
impl<P>
AsyncIterator
for
Pin
<P>
where
    P:
DerefMut
,
    <P as
Deref
>::
Target
:
AsyncIterator
,
Source
§
type
Item
= <<P as
Deref
>::
Target
as
AsyncIterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of items yielded by the async iterator.
Source
§
fn
poll_next
(
    self:
Pin
<&mut
Pin
<P>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<
Option
<<
Pin
<P> as
AsyncIterator
>::
Item
>>
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Attempts to pull out the next value of this async iterator, registering the
current task for wakeup if the value is not yet available, and returning
None
if the async iterator is exhausted.
Read more
Source
§
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Returns the bounds on the remaining length of the async iterator.
Read more
1.33.0
·
Source
§
impl<Ptr>
Clone
for
Pin
<Ptr>
where
    Ptr:
Clone
,
Source
§
fn
clone
(&self) ->
Pin
<Ptr>
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
impl<G, R>
Coroutine
<R> for
Pin
<
&mut G
>
where
    G:
Coroutine
<R> + ?
Sized
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
Read more
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
Read more
Source
§
fn
resume
(
    self:
Pin
<&mut
Pin
<
&mut G
>>,
    arg: R,
) ->
CoroutineState
<<
Pin
<
&mut G
> as
Coroutine
<R>>::
Yield
, <
Pin
<
&mut G
> as
Coroutine
<R>>::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
Read more
Source
§
impl<G, R, A>
Coroutine
<R> for
Pin
<
Box
<G, A>>
where
    G:
Coroutine
<R> + ?
Sized
,
    A:
Allocator
+ 'static,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
Read more
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
Read more
Source
§
fn
resume
(
    self:
Pin
<&mut
Pin
<
Box
<G, A>>>,
    arg: R,
) ->
CoroutineState
<<
Pin
<
Box
<G, A>> as
Coroutine
<R>>::
Yield
, <
Pin
<
Box
<G, A>> as
Coroutine
<R>>::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
Read more
1.33.0
·
Source
§
impl<Ptr>
Debug
for
Pin
<Ptr>
where
    Ptr:
Debug
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
1.33.0
·
Source
§
impl<Ptr>
Deref
for
Pin
<Ptr>
where
    Ptr:
Deref
,
Source
§
type
Target
= <Ptr as
Deref
>::
Target
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &<Ptr as
Deref
>::
Target
Dereferences the value.
1.33.0
·
Source
§
impl<Ptr>
DerefMut
for
Pin
<Ptr>
where
    Ptr:
DerefMut
,
    <Ptr as
Deref
>::
Target
:
Unpin
,
Source
§
fn
deref_mut
(&mut self) -> &mut <Ptr as
Deref
>::
Target
Mutably dereferences the value.
1.33.0
·
Source
§
impl<Ptr>
Display
for
Pin
<Ptr>
where
    Ptr:
Display
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
1.33.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Pin
<
Box
<T, A>>
where
    A:
Allocator
+ 'static,
    T: ?
Sized
,
Source
§
fn
from
(boxed:
Box
<T, A>) ->
Pin
<
Box
<T, A>>
Converts a
Box<T>
into a
Pin<Box<T>>
. If
T
does not implement
Unpin
, then
*boxed
will be pinned in memory and unable to be moved.
This conversion does not allocate on the heap and happens in place.
This is also available via
Box::into_pin
.
Constructing and pinning a
Box
with
<Pin<Box<T>>>::from(
Box::new
(x))
can also be written more concisely using
Box::pin
(x)
.
This
From
implementation is useful if you already have a
Box<T>
, or you are
constructing a (pinned)
Box
in a different way than with
Box::new
.
1.36.0
·
Source
§
impl<P>
Future
for
Pin
<P>
where
    P:
DerefMut
,
    <P as
Deref
>::
Target
:
Future
,
Source
§
type
Output
= <<P as
Deref
>::
Target
as
Future
>::
Output
The type of value produced on completion.
Source
§
fn
poll
(
    self:
Pin
<&mut
Pin
<P>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<<
Pin
<P> as
Future
>::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
1.41.0
·
Source
§
impl<Ptr>
Hash
for
Pin
<Ptr>
where
    Ptr:
Deref
,
    <Ptr as
Deref
>::
Target
:
Hash
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
1.41.0
·
Source
§
impl<Ptr>
Ord
for
Pin
<Ptr>
where
    Ptr:
Deref
,
    <Ptr as
Deref
>::
Target
:
Ord
,
Source
§
fn
cmp
(&self, other: &
Pin
<Ptr>) ->
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
1.41.0
·
Source
§
impl<Ptr, Q>
PartialEq
<
Pin
<Q>> for
Pin
<Ptr>
where
    Ptr:
Deref
,
    Q:
Deref
,
    <Ptr as
Deref
>::
Target
:
PartialEq
<<Q as
Deref
>::
Target
>,
Source
§
fn
eq
(&self, other: &
Pin
<Q>) ->
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
Pin
<Q>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.41.0
·
Source
§
impl<Ptr, Q>
PartialOrd
<
Pin
<Q>> for
Pin
<Ptr>
where
    Ptr:
Deref
,
    Q:
Deref
,
    <Ptr as
Deref
>::
Target
:
PartialOrd
<<Q as
Deref
>::
Target
>,
Source
§
fn
partial_cmp
(&self, other: &
Pin
<Q>) ->
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
Pin
<Q>) ->
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
Pin
<Q>) ->
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
Pin
<Q>) ->
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
Pin
<Q>) ->
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
1.33.0
·
Source
§
impl<Ptr>
Pointer
for
Pin
<Ptr>
where
    Ptr:
Pointer
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
1.33.0
·
Source
§
impl<Ptr, U>
CoerceUnsized
<
Pin
<U>> for
Pin
<Ptr>
where
    Ptr:
CoerceUnsized
<U> +
PinCoerceUnsized
,
    U:
PinCoerceUnsized
,
1.33.0
·
Source
§
impl<Ptr>
Copy
for
Pin
<Ptr>
where
    Ptr:
Copy
,
Source
§
impl<Ptr>
DerefPure
for
Pin
<Ptr>
where
    Ptr:
DerefPure
,
1.33.0
·
Source
§
impl<Ptr, U>
DispatchFromDyn
<
Pin
<U>> for
Pin
<Ptr>
where
    Ptr:
DispatchFromDyn
<U> +
PinCoerceUnsized
,
    U:
PinCoerceUnsized
,
1.41.0
·
Source
§
impl<Ptr>
Eq
for
Pin
<Ptr>
where
    Ptr:
Deref
,
    <Ptr as
Deref
>::
Target
:
Eq
,
1.33.0
·
Source
§
impl<T>
PinCoerceUnsized
for
Pin
<T>
where
    T:
PinCoerceUnsized
,
Source
§
impl<T>
PointerLike
for
Pin
<T>
where
    T:
PointerLike
,
Auto Trait Implementations
§
§
impl<Ptr>
Freeze
for
Pin
<Ptr>
where
    Ptr:
Freeze
,
§
impl<Ptr>
RefUnwindSafe
for
Pin
<Ptr>
where
    Ptr:
RefUnwindSafe
,
§
impl<Ptr>
Send
for
Pin
<Ptr>
where
    Ptr:
Send
,
§
impl<Ptr>
Sync
for
Pin
<Ptr>
where
    Ptr:
Sync
,
§
impl<Ptr>
Unpin
for
Pin
<Ptr>
where
    Ptr:
Unpin
,
§
impl<Ptr>
UnwindSafe
for
Pin
<Ptr>
where
    Ptr:
UnwindSafe
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
impl<I>
IntoAsyncIterator
for I
where
    I:
AsyncIterator
,
Source
§
type
Item
= <I as
AsyncIterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the item yielded by the iterator
Source
§
type
IntoAsyncIter
= I
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the resulting iterator
Source
§
fn
into_async_iter
(self) -> <I as
IntoAsyncIterator
>::
IntoAsyncIter
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Converts
self
into an async iterator
Source
§
impl<F>
IntoFuture
for F
where
    F:
Future
,
Source
§
type
Output
= <F as
Future
>::
Output
The output that the future will produce on completion.
Source
§
type
IntoFuture
= F
Which kind of future are we turning this into?
Source
§
fn
into_future
(self) -> <F as
IntoFuture
>::
IntoFuture
Creates a future from a value.
Read more
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