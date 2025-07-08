uninitialized in std::mem - Rust
std
::
mem
Function
uninitialized
Copy item path
1.0.0
·
Source
pub unsafe fn uninitialized<T>() -> T
👎
Deprecated since 1.39.0: use
mem::MaybeUninit
instead
Expand description
Bypasses Rust’s normal memory-initialization checks by pretending to
produce a value of type
T
, while doing nothing at all.
This function is deprecated.
Use
MaybeUninit<T>
instead.
It also might be slower than using
MaybeUninit<T>
due to mitigations that were put in place to
limit the potential harm caused by incorrect use of this function in legacy code.
The reason for deprecation is that the function basically cannot be used
correctly: it has the same effect as
MaybeUninit::uninit().assume_init()
.
As the
assume_init
documentation
explains,
the Rust compiler assumes
that values are properly initialized.
Truly uninitialized memory like what gets returned here
is special in that the compiler knows that it does not have a fixed value.
This makes it undefined behavior to have uninitialized data in a variable even
if that variable has an integer type.
Therefore, it is immediate undefined behavior to call this function on nearly all types,
including integer types and arrays of integer types, and even if the result is unused.