LockResult in std::sync::poison - Rust
std
::
sync
::
poison
Type Alias
LockResult
Copy item path
Source
pub type LockResult<T> =
Result
<T,
PoisonError
<T>>;
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Expand description
A type alias for the result of a lock method which can be poisoned.
The
Ok
variant of this result indicates that the primitive was not
poisoned, and the operation result is contained within. The
Err
variant indicates
that the primitive was poisoned. Note that the
Err
variant
also
carries
an associated value assigned by the lock method, and it can be acquired through the
into_inner
method. The semantics of the associated value depends on the corresponding
lock method.
Aliased Type
§
enum LockResult<T> {
    Ok(T),
    Err(
PoisonError
<T>),
}
Variants
§
§
Ok(T)
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Contains the success value
§
Err(
PoisonError
<T>)
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Contains the error value