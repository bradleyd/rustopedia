is_val_statically_known in std::intrinsics - Rust
std
::
intrinsics
Function
is_val_statically_known
Copy item path
Source
pub const fn is_val_statically_known<T>(_arg: T) ->
bool
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns whether the argument’s value is statically known at
compile-time.
This is useful when there is a way of writing the code that will
be
faster
when some variables have known values, but
slower
in the general case: an
if is_val_statically_known(var)
can be used
to select between these two variants. The
if
will be optimized away
and only the desired branch remains.
Formally speaking, this function non-deterministically returns
true
or
false
, and the caller has to ensure sound behavior for both cases.
In other words, the following code has
Undefined Behavior
:
#![feature(core_intrinsics)]
use
std::hint::unreachable_unchecked;
use
std::intrinsics::is_val_statically_known;
if
!is_val_statically_known(
0
) {
unsafe
{ unreachable_unchecked(); } }
This also means that the following code’s behavior is unspecified; it
may panic, or it may not:
#![feature(core_intrinsics)]
use
std::intrinsics::is_val_statically_known;
assert_eq!
(is_val_statically_known(
0
), is_val_statically_known(
0
));
Unsafe code may not rely on
is_val_statically_known
returning any
particular value, ever. However, the compiler will generally make it
return
true
only if the value of the argument is actually known.
§
Stability concerns
While it is safe to call, this intrinsic may behave differently in
a
const
context than otherwise. See the
const_eval_select()
documentation for an explanation of the issues this can cause. Unlike
const_eval_select
, this intrinsic isn’t guaranteed to behave
deterministically even in a
const
context.
§
Type Requirements
T
must be either a
bool
, a
char
, a primitive numeric type (e.g.
f32
,
but not
NonZeroISize
), or any thin pointer (e.g.
*mut String
).
Any other argument types
may
cause a compiler error.
§
Pointers
When the input is a pointer, only the pointer itself is
ever considered. The pointee has no effect. Currently, these functions
behave identically:
#![feature(core_intrinsics)]
use
std::intrinsics::is_val_statically_known;
fn
foo(x:
&
i32) -> bool {
    is_val_statically_known(x)
}
fn
bar(x:
&
i32) -> bool {
    is_val_statically_known(
        (x
as
*const
i32).addr()
    )
}