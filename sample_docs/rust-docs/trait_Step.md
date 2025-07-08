Step in std::range - Rust
std
::
range
Trait
Step
Copy item path
Source
pub trait Step:
Sized
+
Clone
+
PartialOrd
{
    // Required methods
    fn
steps_between
(start: &Self, end: &Self) -> (
usize
,
Option
<
usize
>);
fn
forward_checked
(start: Self, count:
usize
) ->
Option
<Self>;
fn
backward_checked
(start: Self, count:
usize
) ->
Option
<Self>;

    // Provided methods
    fn
forward
(start: Self, count:
usize
) -> Self { ... }
unsafe fn
forward_unchecked
(start: Self, count:
usize
) -> Self { ... }
fn
backward
(start: Self, count:
usize
) -> Self { ... }
unsafe fn
backward_unchecked
(start: Self, count:
usize
) -> Self { ... }
}
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Expand description
Objects that have a notion of
successor
and
predecessor
operations.
The
successor
operation moves towards values that compare greater.
The
predecessor
operation moves towards values that compare lesser.
Required Methods
§
Source
fn
steps_between
(start: &Self, end: &Self) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the bounds on the number of
successor
steps required to get from
start
to
end
like
Iterator::size_hint()
.
Returns
(usize::MAX, None)
if the number of steps would overflow
usize
, or is infinite.
§
Invariants
For any
a
,
b
, and
n
:
steps_between(&a, &b) == (n, Some(n))
if and only if
Step::forward_checked(&a, n) == Some(b)
steps_between(&a, &b) == (n, Some(n))
if and only if
Step::backward_checked(&b, n) == Some(a)
steps_between(&a, &b) == (n, Some(n))
only if
a <= b
Corollary:
steps_between(&a, &b) == (0, Some(0))
if and only if
a == b
steps_between(&a, &b) == (0, None)
if
a > b
Source
fn
forward_checked
(start: Self, count:
usize
) ->
Option
<Self>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
If this would overflow the range of values supported by
Self
, returns
None
.
§
Invariants
For any
a
,
n
, and
m
:
Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == Step::forward_checked(a, m).and_then(|x| Step::forward_checked(x, n))
Step::forward_checked(a, n).and_then(|x| Step::forward_checked(x, m)) == try { Step::forward_checked(a, n.checked_add(m)) }
For any
a
and
n
:
Step::forward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::forward_checked(&x, 1))
Corollary:
Step::forward_checked(a, 0) == Some(a)
Source
fn
backward_checked
(start: Self, count:
usize
) ->
Option
<Self>
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
If this would overflow the range of values supported by
Self
, returns
None
.
§
Invariants
For any
a
,
n
, and
m
:
Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == n.checked_add(m).and_then(|x| Step::backward_checked(a, x))
Step::backward_checked(a, n).and_then(|x| Step::backward_checked(x, m)) == try { Step::backward_checked(a, n.checked_add(m)?) }
For any
a
and
n
:
Step::backward_checked(a, n) == (0..n).try_fold(a, |x, _| Step::backward_checked(x, 1))
Corollary:
Step::backward_checked(a, 0) == Some(a)
Provided Methods
§
Source
fn
forward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
If this would overflow the range of values supported by
Self
,
this function is allowed to panic, wrap, or saturate.
The suggested behavior is to panic when debug assertions are enabled,
and to wrap or saturate otherwise.
Unsafe code should not rely on the correctness of behavior after overflow.
§
Invariants
For any
a
,
n
, and
m
, where no overflow occurs:
Step::forward(Step::forward(a, n), m) == Step::forward(a, n + m)
For any
a
and
n
, where no overflow occurs:
Step::forward_checked(a, n) == Some(Step::forward(a, n))
Step::forward(a, n) == (0..n).fold(a, |x, _| Step::forward(x, 1))
Corollary:
Step::forward(a, 0) == a
Step::forward(a, n) >= a
Step::backward(Step::forward(a, n), n) == a
Source
unsafe fn
forward_unchecked
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
successor
of
self
count
times.
§
Safety
It is undefined behavior for this operation to overflow the
range of values supported by
Self
. If you cannot guarantee that this
will not overflow, use
forward
or
forward_checked
instead.
§
Invariants
For any
a
:
if there exists
b
such that
b > a
, it is safe to call
Step::forward_unchecked(a, 1)
if there exists
b
,
n
such that
steps_between(&a, &b) == Some(n)
,
it is safe to call
Step::forward_unchecked(a, m)
for any
m <= n
.
Corollary:
Step::forward_unchecked(a, 0)
is always safe.
For any
a
and
n
, where no overflow occurs:
Step::forward_unchecked(a, n)
is equivalent to
Step::forward(a, n)
Source
fn
backward
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
If this would overflow the range of values supported by
Self
,
this function is allowed to panic, wrap, or saturate.
The suggested behavior is to panic when debug assertions are enabled,
and to wrap or saturate otherwise.
Unsafe code should not rely on the correctness of behavior after overflow.
§
Invariants
For any
a
,
n
, and
m
, where no overflow occurs:
Step::backward(Step::backward(a, n), m) == Step::backward(a, n + m)
For any
a
and
n
, where no overflow occurs:
Step::backward_checked(a, n) == Some(Step::backward(a, n))
Step::backward(a, n) == (0..n).fold(a, |x, _| Step::backward(x, 1))
Corollary:
Step::backward(a, 0) == a
Step::backward(a, n) <= a
Step::forward(Step::backward(a, n), n) == a
Source
unsafe fn
backward_unchecked
(start: Self, count:
usize
) -> Self
🔬
This is a nightly-only experimental API. (
step_trait
#42168
)
Returns the value that would be obtained by taking the
predecessor
of
self
count
times.
§
Safety
It is undefined behavior for this operation to overflow the
range of values supported by
Self
. If you cannot guarantee that this
will not overflow, use
backward
or
backward_checked
instead.
§
Invariants
For any
a
:
if there exists
b
such that
b < a
, it is safe to call
Step::backward_unchecked(a, 1)
if there exists
b
,
n
such that
steps_between(&b, &a) == (n, Some(n))
,
it is safe to call
Step::backward_unchecked(a, m)
for any
m <= n
.
Corollary:
Step::backward_unchecked(a, 0)
is always safe.
For any
a
and
n
, where no overflow occurs:
Step::backward_unchecked(a, n)
is equivalent to
Step::backward(a, n)
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
Step
for
AsciiChar
Source
§
impl
Step
for
char
Source
§
impl
Step
for
i8
Source
§
impl
Step
for
i16
Source
§
impl
Step
for
i32
Source
§
impl
Step
for
i64
Source
§
impl
Step
for
i128
Source
§
impl
Step
for
isize
Source
§
impl
Step
for
u8
Source
§
impl
Step
for
u16
Source
§
impl
Step
for
u32
Source
§
impl
Step
for
u64
Source
§
impl
Step
for
u128
Source
§
impl
Step
for
usize
Source
§
impl
Step
for
Ipv4Addr
Source
§
impl
Step
for
Ipv6Addr