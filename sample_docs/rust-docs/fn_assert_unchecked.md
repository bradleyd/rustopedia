assert_unchecked in std::hint - Rust
std
::
hint
Function
assert_unchecked
Copy item path
1.81.0 (const: 1.81.0)
·
Source
pub const unsafe fn assert_unchecked(cond:
bool
)
Expand description
Makes a
soundness
promise to the compiler that
cond
holds.
This may allow the optimizer to simplify things, but it might also make the generated code
slower. Either way, calling it will most likely make compilation take longer.
You may know this from other places as
llvm.assume
or, in C,
__builtin_assume
.
This promotes a correctness requirement to a soundness requirement. Don’t do that without
very good reason.
§
Usage
This is a situational tool for micro-optimization, and is allowed to do nothing. Any use
should come with a repeatable benchmark to show the value, with the expectation to drop it
later should the optimizer get smarter and no longer need it.
The more complicated the condition, the less likely this is to be useful. For example,
assert_unchecked(foo.is_sorted())
is a complex enough value that the compiler is unlikely
to be able to take advantage of it.
There’s also no need to
assert_unchecked
basic properties of things.  For example, the
compiler already knows the range of
count_ones
, so there is no benefit to
let n = u32::count_ones(x); assert_unchecked(n <= u32::BITS);
.
assert_unchecked
is logically equivalent to
if !cond { unreachable_unchecked(); }
. If
ever you are tempted to write
assert_unchecked(false)
, you should instead use
unreachable_unchecked()
directly.
§
Safety
cond
must be
true
. It is immediate UB to call this with
false
.
§
Example
use
core::hint;
/// # Safety
///
/// `p` must be nonnull and valid
pub unsafe fn
next_value(p:
*const
i32) -> i32 {
// SAFETY: caller invariants guarantee that `p` is not null
unsafe
{ hint::assert_unchecked(!p.is_null()) }
if
p.is_null() {
return
-
1
;
    }
else
{
// SAFETY: caller invariants guarantee that `p` is valid
unsafe
{
*
p +
1
}
    }
}
Without the
assert_unchecked
, the above function produces the following with optimizations
enabled:
next_value:
        test    rdi, rdi
        je      .LBB0_1
        mov     eax, dword ptr [rdi]
        inc     eax
        ret
.LBB0_1:
        mov     eax, -1
        ret
Adding the assertion allows the optimizer to remove the extra check:
next_value:
        mov     eax, dword ptr [rdi]
        inc     eax
        ret
This example is quite unlike anything that would be used in the real world: it is redundant
to put an assertion right next to code that checks the same thing, and dereferencing a
pointer already has the builtin assumption that it is nonnull. However, it illustrates the
kind of changes the optimizer can make even when the behavior is less obviously related.