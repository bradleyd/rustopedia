from_fn in std::fmt - Rust
std
::
fmt
Function
from_fn
Copy item path
Source
pub fn from_fn<F>(f: F) ->
FromFn
<F>
where
    F:
Fn
(&mut
Formatter
<'_>) ->
Result
<
()
,
Error
>,
🔬
This is a nightly-only experimental API. (
debug_closure_helpers
#117729
)
Expand description
Creates a type whose
fmt::Debug
and
fmt::Display
impls are provided with the function
f
.
§
Examples
#![feature(debug_closure_helpers)]
use
std::fmt;
let
value =
'a'
;
assert_eq!
(
format!
(
"{}"
, value),
"a"
);
assert_eq!
(
format!
(
"{:?}"
, value),
"'a'"
);
let
wrapped = fmt::from_fn(|f|
write!
(f,
"{value:?}"
));
assert_eq!
(
format!
(
"{}"
, wrapped),
"'a'"
);
assert_eq!
(
format!
(
"{:?}"
, wrapped),
"'a'"
);