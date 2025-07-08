pending in std::future - Rust
std
::
future
Function
pending
Copy item path
1.48.0
·
Source
pub fn pending<T>() ->
Pending
<T>
ⓘ
Expand description
Creates a future which never resolves, representing a computation that never
finishes.
§
Examples
use
std::future;
let
future = future::pending();
let
() = future.
await
;
unreachable!
();