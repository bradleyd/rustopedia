forget in std::mem - Rust
std
::
mem
Function
forget
Copy item path
1.0.0 (const: 1.46.0)
·
Source
pub const fn forget<T>(t: T)
Expand description
Takes ownership and “forgets” about the value
without running its destructor
.
Any resources the value manages, such as heap memory or a file handle, will linger
forever in an unreachable state. However, it does not guarantee that pointers
to this memory will remain valid.
If you want to leak memory, see
Box::leak
.
If you want to obtain a raw pointer to the memory, see
Box::into_raw
.
If you want to dispose of a value properly, running its destructor, see
mem::drop
.
§
Safety
forget
is not marked as
unsafe
, because Rust’s safety guarantees
do not include a guarantee that destructors will always run. For example,
a program can create a reference cycle using
Rc
, or call
process::exit
to exit without running destructors. Thus, allowing
mem::forget
from safe code does not fundamentally change Rust’s safety
guarantees.
That said, leaking resources such as memory or I/O objects is usually undesirable.
The need comes up in some specialized use cases for FFI or unsafe code, but even
then,
ManuallyDrop
is typically preferred.
Because forgetting a value is allowed, any
unsafe
code you write must
allow for this possibility. You cannot return a value and expect that the
caller will necessarily run the value’s destructor.
§
Examples
The canonical safe use of
mem::forget
is to circumvent a value’s destructor
implemented by the
Drop
trait. For example, this will leak a
File
, i.e. reclaim
the space taken by the variable but never close the underlying system resource:
use
std::mem;
use
std::fs::File;
let
file = File::open(
"foo.txt"
).unwrap();
mem::forget(file);
This is useful when the ownership of the underlying resource was previously
transferred to code outside of Rust, for example by transmitting the raw
file descriptor to C code.
§
Relationship with
ManuallyDrop
While
mem::forget
can also be used to transfer
memory
ownership, doing so is error-prone.
ManuallyDrop
should be used instead. Consider, for example, this code:
use
std::mem;
let
mut
v =
vec!
[
65
,
122
];
// Build a `String` using the contents of `v`
let
s =
unsafe
{ String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
// leak `v` because its memory is now managed by `s`
mem::forget(v);
// ERROR - v is invalid and must not be passed to a function
assert_eq!
(s,
"Az"
);
// `s` is implicitly dropped and its memory deallocated.
There are two issues with the above example:
If more code were added between the construction of
String
and the invocation of
mem::forget()
, a panic within it would cause a double free because the same memory
is handled by both
v
and
s
.
After calling
v.as_mut_ptr()
and transmitting the ownership of the data to
s
,
the
v
value is invalid. Even when a value is just moved to
mem::forget
(which won’t
inspect it), some types have strict requirements on their values that
make them invalid when dangling or no longer owned. Using invalid values in any
way, including passing them to or returning them from functions, constitutes
undefined behavior and may break the assumptions made by the compiler.
Switching to
ManuallyDrop
avoids both issues:
use
std::mem::ManuallyDrop;
let
v =
vec!
[
65
,
122
];
// Before we disassemble `v` into its raw parts, make sure it
// does not get dropped!
let
mut
v = ManuallyDrop::new(v);
// Now disassemble `v`. These operations cannot panic, so there cannot be a leak.
let
(ptr, len, cap) = (v.as_mut_ptr(), v.len(), v.capacity());
// Finally, build a `String`.
let
s =
unsafe
{ String::from_raw_parts(ptr, len, cap) };
assert_eq!
(s,
"Az"
);
// `s` is implicitly dropped and its memory deallocated.
ManuallyDrop
robustly prevents double-free because we disable
v
’s destructor
before doing anything else.
mem::forget()
doesn’t allow this because it consumes its
argument, forcing us to call it only after extracting anything we need from
v
. Even
if a panic were introduced between construction of
ManuallyDrop
and building the
string (which cannot happen in the code as shown), it would result in a leak and not a
double free. In other words,
ManuallyDrop
errs on the side of leaking instead of
erring on the side of (double-)dropping.
Also,
ManuallyDrop
prevents us from having to “touch”
v
after transferring the
ownership to
s
— the final step of interacting with
v
to dispose of it without
running its destructor is entirely avoided.