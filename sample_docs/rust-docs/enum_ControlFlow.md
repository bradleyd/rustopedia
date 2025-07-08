ControlFlow in std::ops - Rust
std
::
ops
Enum
ControlFlow
Copy item path
1.55.0
·
Source
pub enum ControlFlow<B, C =
()
> {
    Continue(C),
    Break(B),
}
Expand description
Used to tell an operation whether it should exit early or go on as usual.
This is used when exposing things (like graph traversals or visitors) where
you want the user to be able to choose whether to exit early.
Having the enum makes it clearer – no more wondering “wait, what did
false
mean again?” – and allows including a value.
Similar to
Option
and
Result
, this enum can be used with the
?
operator
to return immediately if the
Break
variant is present or otherwise continue normally
with the value inside the
Continue
variant.
§
Examples
Early-exiting from
Iterator::try_for_each
:
use
std::ops::ControlFlow;
let
r = (
2
..
100
).try_for_each(|x| {
if
403
% x ==
0
{
return
ControlFlow::Break(x)
    }

    ControlFlow::Continue(())
});
assert_eq!
(r, ControlFlow::Break(
13
));
A basic tree traversal:
use
std::ops::ControlFlow;
pub struct
TreeNode<T> {
    value: T,
    left:
Option
<Box<TreeNode<T>>>,
    right:
Option
<Box<TreeNode<T>>>,
}
impl
<T> TreeNode<T> {
pub fn
traverse_inorder<B>(
&
self
, f:
&mut
impl
FnMut(
&
T) -> ControlFlow<B>) -> ControlFlow<B> {
if let
Some
(left) =
&
self
.left {
            left.traverse_inorder(f)
?
;
        }
        f(
&
self
.value)
?
;
if let
Some
(right) =
&
self
.right {
            right.traverse_inorder(f)
?
;
        }
        ControlFlow::Continue(())
    }
fn
leaf(value: T) ->
Option
<Box<TreeNode<T>>> {
Some
(Box::new(
Self
{ value, left:
None
, right:
None
}))
    }
}
let
node = TreeNode {
    value:
0
,
    left: TreeNode::leaf(
1
),
    right:
Some
(Box::new(TreeNode {
        value: -
1
,
        left: TreeNode::leaf(
5
),
        right: TreeNode::leaf(
2
),
    }))
};
let
mut
sum =
0
;
let
res = node.traverse_inorder(
&mut
|val| {
if
*
val <
0
{
        ControlFlow::Break(
*
val)
    }
else
{
        sum +=
*
val;
        ControlFlow::Continue(())
    }
});
assert_eq!
(res, ControlFlow::Break(-
1
));
assert_eq!
(sum,
6
);
Variants
§
§
1.55.0
Continue(C)
Move on to the next phase of the operation as normal.
§
1.55.0
Break(B)
Exit the operation without running subsequent phases.
Implementations
§
Source
§
impl<B, C>
ControlFlow
<B, C>
1.59.0
·
Source
pub fn
is_break
(&self) ->
bool
Returns
true
if this is a
Break
variant.
§
Examples
use
std::ops::ControlFlow;
assert!
(ControlFlow::<
&
str, i32>::Break(
"Stop right there!"
).is_break());
assert!
(!ControlFlow::<
&
str, i32>::Continue(
3
).is_break());
1.59.0
·
Source
pub fn
is_continue
(&self) ->
bool
Returns
true
if this is a
Continue
variant.
§
Examples
use
std::ops::ControlFlow;
assert!
(!ControlFlow::<
&
str, i32>::Break(
"Stop right there!"
).is_continue());
assert!
(ControlFlow::<
&
str, i32>::Continue(
3
).is_continue());
1.83.0
·
Source
pub fn
break_value
(self) ->
Option
<B>
Converts the
ControlFlow
into an
Option
which is
Some
if the
ControlFlow
was
Break
and
None
otherwise.
§
Examples
use
std::ops::ControlFlow;
assert_eq!
(ControlFlow::<
&
str, i32>::Break(
"Stop right there!"
).break_value(),
Some
(
"Stop right there!"
));
assert_eq!
(ControlFlow::<
&
str, i32>::Continue(
3
).break_value(),
None
);
1.83.0
·
Source
pub fn
map_break
<T>(self, f: impl
FnOnce
(B) -> T) ->
ControlFlow
<T, C>
Maps
ControlFlow<B, C>
to
ControlFlow<T, C>
by applying a function
to the break value in case it exists.
1.83.0
·
Source
pub fn
continue_value
(self) ->
Option
<C>
Converts the
ControlFlow
into an
Option
which is
Some
if the
ControlFlow
was
Continue
and
None
otherwise.
§
Examples
use
std::ops::ControlFlow;
assert_eq!
(ControlFlow::<
&
str, i32>::Break(
"Stop right there!"
).continue_value(),
None
);
assert_eq!
(ControlFlow::<
&
str, i32>::Continue(
3
).continue_value(),
Some
(
3
));
1.83.0
·
Source
pub fn
map_continue
<T>(self, f: impl
FnOnce
(C) -> T) ->
ControlFlow
<B, T>
Maps
ControlFlow<B, C>
to
ControlFlow<B, T>
by applying a function
to the continue value in case it exists.
Source
§
impl<T>
ControlFlow
<T, T>
Source
pub const fn
into_value
(self) -> T
🔬
This is a nightly-only experimental API. (
control_flow_into_value
#137461
)
Extracts the value
T
that is wrapped by
ControlFlow<T, T>
.
§
Examples
#![feature(control_flow_into_value)]
use
std::ops::ControlFlow;
assert_eq!
(ControlFlow::<i32, i32>::Break(
1024
).into_value(),
1024
);
assert_eq!
(ControlFlow::<i32, i32>::Continue(
512
).into_value(),
512
);
Source
§
impl<R>
ControlFlow
<R, <R as
Try
>::
Output
>
where
    R:
Try
,
These are used only as part of implementing the iterator adapters.
They have mediocre names and non-obvious semantics, so aren’t
currently on a path to potential stabilization.
This impl block contains no items.
Trait Implementations
§
1.55.0
·
Source
§
impl<B, C>
Clone
for
ControlFlow
<B, C>
where
    B:
Clone
,
    C:
Clone
,
Source
§
fn
clone
(&self) ->
ControlFlow
<B, C>
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
1.55.0
·
Source
§
impl<B, C>
Debug
for
ControlFlow
<B, C>
where
    B:
Debug
,
    C:
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
Source
§
impl<B, C>
FromResidual
<
ControlFlow
<B,
Infallible
>> for
ControlFlow
<B, C>
Source
§
fn
from_residual
(residual:
ControlFlow
<B,
Infallible
>) ->
ControlFlow
<B, C>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
1.55.0
·
Source
§
impl<B, C>
Hash
for
ControlFlow
<B, C>
where
    B:
Hash
,
    C:
Hash
,
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
1.55.0
·
Source
§
impl<B, C>
PartialEq
for
ControlFlow
<B, C>
where
    B:
PartialEq
,
    C:
PartialEq
,
Source
§
fn
eq
(&self, other: &
ControlFlow
<B, C>) ->
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
impl<B, C>
Residual
<C> for
ControlFlow
<B,
Infallible
>
Source
§
type
TryType
=
ControlFlow
<B, C>
🔬
This is a nightly-only experimental API. (
try_trait_v2_residual
#91285
)
The “return” type of this meta-function.
Source
§
impl<B, C>
Try
for
ControlFlow
<B, C>
Source
§
type
Output
= C
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value produced by
?
when
not
short-circuiting.
Source
§
type
Residual
=
ControlFlow
<B,
Infallible
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value passed to
FromResidual::from_residual
as part of
?
when short-circuiting.
Read more
Source
§
fn
from_output
(output: <
ControlFlow
<B, C> as
Try
>::
Output
) ->
ControlFlow
<B, C>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from its
Output
type.
Read more
Source
§
fn
branch
(
    self,
) ->
ControlFlow
<<
ControlFlow
<B, C> as
Try
>::
Residual
, <
ControlFlow
<B, C> as
Try
>::
Output
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Used in
?
to decide whether the operator should produce a value
(because this returned
ControlFlow::Continue
)
or propagate a value back to the caller
(because this returned
ControlFlow::Break
).
Read more
1.55.0
·
Source
§
impl<B, C>
Copy
for
ControlFlow
<B, C>
where
    B:
Copy
,
    C:
Copy
,
1.55.0
·
Source
§
impl<B, C>
Eq
for
ControlFlow
<B, C>
where
    B:
Eq
,
    C:
Eq
,
1.55.0
·
Source
§
impl<B, C>
StructuralPartialEq
for
ControlFlow
<B, C>
Auto Trait Implementations
§
§
impl<B, C>
Freeze
for
ControlFlow
<B, C>
where
    C:
Freeze
,
    B:
Freeze
,
§
impl<B, C>
RefUnwindSafe
for
ControlFlow
<B, C>
where
    C:
RefUnwindSafe
,
    B:
RefUnwindSafe
,
§
impl<B, C>
Send
for
ControlFlow
<B, C>
where
    C:
Send
,
    B:
Send
,
§
impl<B, C>
Sync
for
ControlFlow
<B, C>
where
    C:
Sync
,
    B:
Sync
,
§
impl<B, C>
Unpin
for
ControlFlow
<B, C>
where
    C:
Unpin
,
    B:
Unpin
,
§
impl<B, C>
UnwindSafe
for
ControlFlow
<B, C>
where
    C:
UnwindSafe
,
    B:
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