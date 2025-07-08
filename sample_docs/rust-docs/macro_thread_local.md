thread_local in std - Rust
std
Macro
thread_local
Copy item path
1.0.0
·
Source
macro_rules! thread_local {
    () => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const $init:block; $($rest:tt)*) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const $init:block) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr) => { ... };
}
Expand description
Declare a new thread local storage key of type
std::thread::LocalKey
.
§
Syntax
The macro wraps any number of static declarations and makes them thread local.
Publicity and attributes for each static are allowed. Example:
use
std::cell::{Cell, RefCell};
thread_local!
{
pub static
FOO: Cell<u32> =
const
{ Cell::new(
1
) };
static
BAR: RefCell<Vec<f32>> = RefCell::new(
vec!
[
1.0
,
2.0
]);
}
assert_eq!
(FOO.get(),
1
);
BAR.with_borrow(|v|
assert_eq!
(v[
1
],
2.0
));
Note that only shared references (
&T
) to the inner data may be obtained, so a
type such as
Cell
or
RefCell
is typically used to allow mutating access.
This macro supports a special
const {}
syntax that can be used
when the initialization expression can be evaluated as a constant.
This can enable a more efficient thread local implementation that
can avoid lazy initialization. For types that do not
need to be dropped
, this can enable an
even more efficient implementation that does not need to
track any additional state.
use
std::cell::RefCell;
thread_local!
{
pub static
FOO: RefCell<Vec<u32>> =
const
{ RefCell::new(Vec::new()) };
}

FOO.with_borrow(|v|
assert_eq!
(v.len(),
0
));
See
LocalKey
documentation
for more
information.