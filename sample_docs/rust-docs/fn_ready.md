ready in std::future - Rust
std
::
future
Function
ready
Copy item path
1.48.0
·
Source
pub fn ready<T>(t: T) ->
Ready
<T>
ⓘ
Expand description
Creates a future that is immediately ready with a value.
Futures created through this function are functionally similar to those
created through
async {}
. The main difference is that futures created
through this function are named and implement
Unpin
.
§
Examples
use
std::future;
let
a = future::ready(
1
);
assert_eq!
(a.
await
,
1
);