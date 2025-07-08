TryLockResult in std::sync::poison - Rust
std
::
sync
::
poison
Type Alias
TryLockResult
Copy item path
Source
pub type TryLockResult<Guard> =
Result
<Guard,
TryLockError
<Guard>>;
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Expand description
A type alias for the result of a nonblocking locking method.
For more information, see
LockResult
. A
TryLockResult
doesn’t
necessarily hold the associated guard in the
Err
type as the lock might not
have been acquired for other reasons.
Aliased Type
§
enum TryLockResult<Guard> {
    Ok(Guard),
    Err(
TryLockError
<Guard>),
}
Variants
§
§
Ok(Guard)
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Contains the success value
§
Err(
TryLockError
<Guard>)
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Contains the error value