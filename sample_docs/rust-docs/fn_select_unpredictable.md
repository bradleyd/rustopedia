select_unpredictable in std::intrinsics - Rust
std
::
intrinsics
Function
select_unpredictable
Copy item path
Source
pub fn select_unpredictable<T>(b:
bool
, true_val: T, false_val: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns either
true_val
or
false_val
depending on condition
b
with a
hint to the compiler that this condition is unlikely to be correctly
predicted by a CPU’s branch predictor (e.g. a binary search).
This is otherwise functionally equivalent to
if b { true_val } else { false_val }
.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The public form of this instrinsic is
bool::select_unpredictable
.