catch_unwind in std::intrinsics - Rust
std
::
intrinsics
Function
catch_unwind
Copy item path
Source
pub unsafe fn catch_unwind(
    _try_fn:
fn
(_:
*mut
u8
),
    _data:
*mut
u8
,
    _catch_fn:
fn
(_:
*mut
u8
, _:
*mut
u8
),
) ->
i32
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Rust’s “try catch” construct for unwinding. Invokes the function pointer
try_fn
with the
data pointer
data
, and calls
catch_fn
if unwinding occurs while
try_fn
runs.
Returns
1
if unwinding occurred and
catch_fn
was called; returns
0
otherwise.
catch_fn
must not unwind.
The third argument is a function called if an unwind occurs (both Rust
panic
and foreign
unwinds). This function takes the data pointer and a pointer to the target- and
runtime-specific exception object that was caught.
Note that in the case of a foreign unwinding operation, the exception object data may not be
safely usable from Rust, and should not be directly exposed via the standard library. To
prevent unsafe access, the library implementation may either abort the process or present an
opaque error type to the user.
For more information, see the compiler’s source, as well as the documentation for the stable
version of this intrinsic,
std::panic::catch_unwind
.