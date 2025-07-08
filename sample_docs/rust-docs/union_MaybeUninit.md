MaybeUninit in std::mem - Rust
std
::
mem
Union
MaybeUninit
Copy item path
1.36.0
·
Source
pub union MaybeUninit<T> {
/* private fields */
}
Expand description
A wrapper type to construct uninitialized instances of
T
.
§
Initialization invariant
The compiler, in general, assumes that a variable is properly initialized
according to the requirements of the variable’s type. For example, a variable of
reference type must be aligned and non-null. This is an invariant that must
always
be upheld, even in unsafe code. As a consequence, zero-initializing a
variable of reference type causes instantaneous
undefined behavior
,
no matter whether that reference ever gets used to access memory:
use
std::mem::{
self
, MaybeUninit};
let
x:
&
i32 =
unsafe
{ mem::zeroed() };
// undefined behavior! ⚠️
// The equivalent code with `MaybeUninit<&i32>`:
let
x:
&
i32 =
unsafe
{ MaybeUninit::zeroed().assume_init() };
// undefined behavior! ⚠️
This is exploited by the compiler for various optimizations, such as eliding
run-time checks and optimizing
enum
layout.
Similarly, entirely uninitialized memory may have any content, while a
bool
must
always be
true
or
false
. Hence, creating an uninitialized
bool
is undefined behavior:
use
std::mem::{
self
, MaybeUninit};
let
b: bool =
unsafe
{ mem::uninitialized() };
// undefined behavior! ⚠️
// The equivalent code with `MaybeUninit<bool>`:
let
b: bool =
unsafe
{ MaybeUninit::uninit().assume_init() };
// undefined behavior! ⚠️
Moreover, uninitialized memory is special in that it does not have a fixed value (“fixed”
meaning “it won’t change without being written to”). Reading the same uninitialized byte
multiple times can give different results. This makes it undefined behavior to have
uninitialized data in a variable even if that variable has an integer type, which otherwise can
hold any
fixed
bit pattern:
use
std::mem::{
self
, MaybeUninit};
let
x: i32 =
unsafe
{ mem::uninitialized() };
// undefined behavior! ⚠️
// The equivalent code with `MaybeUninit<i32>`:
let
x: i32 =
unsafe
{ MaybeUninit::uninit().assume_init() };
// undefined behavior! ⚠️
On top of that, remember that most types have additional invariants beyond merely
being considered initialized at the type level. For example, a
1
-initialized
Vec<T>
is considered initialized (under the current implementation; this does not constitute
a stable guarantee) because the only requirement the compiler knows about it
is that the data pointer must be non-null. Creating such a
Vec<T>
does not cause
immediate
undefined behavior, but will cause undefined behavior with most
safe operations (including dropping it).
§
Examples
MaybeUninit<T>
serves to enable unsafe code to deal with uninitialized data.
It is a signal to the compiler indicating that the data here might
not
be initialized:
use
std::mem::MaybeUninit;
// Create an explicitly uninitialized reference. The compiler knows that data inside
// a `MaybeUninit<T>` may be invalid, and hence this is not UB:
let
mut
x = MaybeUninit::<
&
i32>::uninit();
// Set it to a valid value.
x.write(
&
0
);
// Extract the initialized data -- this is only allowed *after* properly
// initializing `x`!
let
x =
unsafe
{ x.assume_init() };
The compiler then knows to not make any incorrect assumptions or optimizations on this code.
You can think of
MaybeUninit<T>
as being a bit like
Option<T>
but without
any of the run-time tracking and without any of the safety checks.
§
out-pointers
You can use
MaybeUninit<T>
to implement “out-pointers”: instead of returning data
from a function, pass it a pointer to some (uninitialized) memory to put the
result into. This can be useful when it is important for the caller to control
how the memory the result is stored in gets allocated, and you want to avoid
unnecessary moves.
use
std::mem::MaybeUninit;
unsafe fn
make_vec(out:
*mut
Vec<i32>) {
// `write` does not drop the old contents, which is important.
unsafe
{ out.write(
vec!
[
1
,
2
,
3
]); }
}
let
mut
v = MaybeUninit::uninit();
unsafe
{ make_vec(v.as_mut_ptr()); }
// Now we know `v` is initialized! This also makes sure the vector gets
// properly dropped.
let
v =
unsafe
{ v.assume_init() };
assert_eq!
(
&
v,
&
[
1
,
2
,
3
]);
§
Initializing an array element-by-element
MaybeUninit<T>
can be used to initialize a large array element-by-element:
use
std::mem::{
self
, MaybeUninit};
let
data = {
// Create an uninitialized array of `MaybeUninit`.
let
mut
data: [MaybeUninit<Vec<u32>>;
1000
] = [
const
{ MaybeUninit::uninit() };
1000
];
// Dropping a `MaybeUninit` does nothing, so if there is a panic during this loop,
    // we have a memory leak, but there is no memory safety issue.
for
elem
in
&mut
data[..] {
        elem.write(
vec!
[
42
]);
    }
// Everything is initialized. Transmute the array to the
    // initialized type.
unsafe
{ mem::transmute::<
_
, [Vec<u32>;
1000
]>(data) }
};
assert_eq!
(
&
data[
0
],
&
[
42
]);
You can also work with partially initialized arrays, which could
be found in low-level datastructures.
use
std::mem::MaybeUninit;
// Create an uninitialized array of `MaybeUninit`.
let
mut
data: [MaybeUninit<String>;
1000
] = [
const
{ MaybeUninit::uninit() };
1000
];
// Count the number of elements we have assigned.
let
mut
data_len: usize =
0
;
for
elem
in
&mut
data[
0
..
500
] {
    elem.write(String::from(
"hello"
));
    data_len +=
1
;
}
// For each item in the array, drop if we allocated it.
for
elem
in
&mut
data[
0
..data_len] {
unsafe
{ elem.assume_init_drop(); }
}
§
Initializing a struct field-by-field
You can use
MaybeUninit<T>
, and the
std::ptr::addr_of_mut
macro, to initialize structs field by field:
use
std::mem::MaybeUninit;
use
std::ptr::addr_of_mut;
#[derive(Debug, PartialEq)]
pub struct
Foo {
    name: String,
    list: Vec<u8>,
}
let
foo = {
let
mut
uninit: MaybeUninit<Foo> = MaybeUninit::uninit();
let
ptr = uninit.as_mut_ptr();
// Initializing the `name` field
    // Using `write` instead of assignment via `=` to not call `drop` on the
    // old, uninitialized value.
unsafe
{
addr_of_mut!
((
*
ptr).name).write(
"Bob"
.to_string()); }
// Initializing the `list` field
    // If there is a panic here, then the `String` in the `name` field leaks.
unsafe
{
addr_of_mut!
((
*
ptr).list).write(
vec!
[
0
,
1
,
2
]); }
// All the fields are initialized, so we call `assume_init` to get an initialized Foo.
unsafe
{ uninit.assume_init() }
};
assert_eq!
(
    foo,
    Foo {
        name:
"Bob"
.to_string(),
        list:
vec!
[
0
,
1
,
2
]
    }
);
§
Layout
MaybeUninit<T>
is guaranteed to have the same size, alignment, and ABI as
T
:
use
std::mem::MaybeUninit;
assert_eq!
(size_of::<MaybeUninit<u64>>(), size_of::<u64>());
assert_eq!
(align_of::<MaybeUninit<u64>>(), align_of::<u64>());
However remember that a type
containing
a
MaybeUninit<T>
is not necessarily the same
layout; Rust does not in general guarantee that the fields of a
Foo<T>
have the same order as
a
Foo<U>
even if
T
and
U
have the same size and alignment. Furthermore because any bit
value is valid for a
MaybeUninit<T>
the compiler can’t apply non-zero/niche-filling
optimizations, potentially resulting in a larger size:
assert_eq!
(size_of::<
Option
<bool>>(),
1
);
assert_eq!
(size_of::<
Option
<MaybeUninit<bool>>>(),
2
);
If
T
is FFI-safe, then so is
MaybeUninit<T>
.
While
MaybeUninit
is
#[repr(transparent)]
(indicating it guarantees the same size,
alignment, and ABI as
T
), this does
not
change any of the previous caveats.
Option<T>
and
Option<MaybeUninit<T>>
may still have different sizes, and types containing a field of type
T
may be laid out (and sized) differently than if that field were
MaybeUninit<T>
.
MaybeUninit
is a union type, and
#[repr(transparent)]
on unions is unstable (see
the
tracking issue
). Over time, the exact
guarantees of
#[repr(transparent)]
on unions may evolve, and
MaybeUninit
may or may not
remain
#[repr(transparent)]
. That said,
MaybeUninit<T>
will
always
guarantee that it has
the same size, alignment, and ABI as
T
; it’s just that the way
MaybeUninit
implements that
guarantee may evolve.
Note that even though
T
and
MaybeUninit<T>
are ABI compatible it is still unsound to
transmute
&mut T
to
&mut MaybeUninit<T>
and expose that to safe code because it would allow
safe code to access uninitialized memory:
use
core::mem::MaybeUninit;
fn
unsound_transmute<T>(val:
&mut
T) ->
&mut
MaybeUninit<T> {
unsafe
{ core::mem::transmute(val) }
}
fn
main() {
let
mut
code =
0
;
let
code =
&mut
code;
let
code2 = unsound_transmute(code);
*
code2 = MaybeUninit::uninit();
    std::process::exit(
*
code);
// UB! Accessing uninitialized memory.
}
Implementations
§
Source
§
impl<T>
MaybeUninit
<T>
1.36.0 (const: 1.36.0)
·
Source
pub const fn
new
(val: T) ->
MaybeUninit
<T>
Creates a new
MaybeUninit<T>
initialized with the given value.
It is safe to call
assume_init
on the return value of this function.
Note that dropping a
MaybeUninit<T>
will never call
T
’s drop code.
It is your responsibility to make sure
T
gets dropped if it got initialized.
§
Example
use
std::mem::MaybeUninit;
let
v: MaybeUninit<Vec<u8>> = MaybeUninit::new(
vec!
[
42
]);
1.36.0 (const: 1.36.0)
·
Source
pub const fn
uninit
() ->
MaybeUninit
<T>
Creates a new
MaybeUninit<T>
in an uninitialized state.
Note that dropping a
MaybeUninit<T>
will never call
T
’s drop code.
It is your responsibility to make sure
T
gets dropped if it got initialized.
See the
type-level documentation
for some examples.
§
Example
use
std::mem::MaybeUninit;
let
v: MaybeUninit<String> = MaybeUninit::uninit();
1.36.0 (const: 1.75.0)
·
Source
pub const fn
zeroed
() ->
MaybeUninit
<T>
Creates a new
MaybeUninit<T>
in an uninitialized state, with the memory being
filled with
0
bytes. It depends on
T
whether that already makes for
proper initialization. For example,
MaybeUninit<usize>::zeroed()
is initialized,
but
MaybeUninit<&'static i32>::zeroed()
is not because references must not
be null.
Note that if
T
has padding bytes, those bytes are
not
preserved when the
MaybeUninit<T>
value is returned from this function, so those bytes will
not
be zeroed.
Note that dropping a
MaybeUninit<T>
will never call
T
’s drop code.
It is your responsibility to make sure
T
gets dropped if it got initialized.
§
Example
Correct usage of this function: initializing a struct with zero, where all
fields of the struct can hold the bit-pattern 0 as a valid value.
use
std::mem::MaybeUninit;
let
x = MaybeUninit::<(u8, bool)>::zeroed();
let
x =
unsafe
{ x.assume_init() };
assert_eq!
(x, (
0
,
false
));
This can be used in const contexts, such as to indicate the end of static arrays for
plugin registration.
Incorrect
usage of this function: calling
x.zeroed().assume_init()
when
0
is not a valid bit-pattern for the type:
use
std::mem::MaybeUninit;
enum
NotZero { One =
1
, Two =
2
}
let
x = MaybeUninit::<(u8, NotZero)>::zeroed();
let
x =
unsafe
{ x.assume_init() };
// Inside a pair, we create a `NotZero` that does not have a valid discriminant.
// This is undefined behavior. ⚠️
1.55.0 (const: 1.85.0)
·
Source
pub const fn
write
(&mut self, val: T) ->
&mut T
Sets the value of the
MaybeUninit<T>
.
This overwrites any previous value without dropping it, so be careful
not to use this twice unless you want to skip running the destructor.
For your convenience, this also returns a mutable reference to the
(now safely initialized) contents of
self
.
As the content is stored inside a
MaybeUninit
, the destructor is not
run for the inner data if the MaybeUninit leaves scope without a call to
assume_init
,
assume_init_drop
, or similar. Code that receives
the mutable reference returned by this function needs to keep this in
mind. The safety model of Rust regards leaks as safe, but they are
usually still undesirable. This being said, the mutable reference
behaves like any other mutable reference would, so assigning a new value
to it will drop the old content.
§
Examples
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<Vec<u8>>::uninit();

{
let
hello = x.write((
&
b"Hello, world!"
).to_vec());
// Setting hello does not leak prior allocations, but drops them
*
hello = (
&
b"Hello"
).to_vec();
    hello[
0
] =
'h'
as
u8;
}
// x is initialized now:
let
s =
unsafe
{ x.assume_init() };
assert_eq!
(
b"hello"
, s.as_slice());
This usage of the method causes a leak:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<String>::uninit();

x.write(
"Hello"
.to_string());
// This leaks the contained string:
x.write(
"hello"
.to_string());
// x is initialized now:
let
s =
unsafe
{ x.assume_init() };
This method can be used to avoid unsafe in some cases. The example below
shows a part of an implementation of a fixed sized arena that lends out
pinned references.
With
write
, we can avoid the need to write through a raw pointer:
use
core::pin::Pin;
use
core::mem::MaybeUninit;
struct
PinArena<T> {
    memory: Box<[MaybeUninit<T>]>,
    len: usize,
}
impl
<T> PinArena<T> {
pub fn
capacity(
&
self
) -> usize {
self
.memory.len()
    }
pub fn
push(
&mut
self
, val: T) -> Pin<
&mut
T> {
if
self
.len >=
self
.capacity() {
panic!
(
"Attempted to push to a full pin arena!"
);
        }
let
ref_ =
self
.memory[
self
.len].write(val);
self
.len +=
1
;
unsafe
{ Pin::new_unchecked(ref_) }
    }
}
1.36.0 (const: 1.59.0)
·
Source
pub const fn
as_ptr
(&self) ->
*const T
Gets a pointer to the contained value. Reading from this pointer or turning it
into a reference is undefined behavior unless the
MaybeUninit<T>
is initialized.
Writing to memory that this pointer (non-transitively) points to is undefined behavior
(except inside an
UnsafeCell<T>
).
§
Examples
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<Vec<u32>>::uninit();
x.write(
vec!
[
0
,
1
,
2
]);
// Create a reference into the `MaybeUninit<T>`. This is okay because we initialized it.
let
x_vec =
unsafe
{
&*
x.as_ptr() };
assert_eq!
(x_vec.len(),
3
);
Incorrect
usage of this method:
use
std::mem::MaybeUninit;
let
x = MaybeUninit::<Vec<u32>>::uninit();
let
x_vec =
unsafe
{
&*
x.as_ptr() };
// We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
(Notice that the rules around references to uninitialized data are not finalized yet, but
until they are, it is advisable to avoid them.)
1.36.0 (const: 1.83.0)
·
Source
pub const fn
as_mut_ptr
(&mut self) ->
*mut T
Gets a mutable pointer to the contained value. Reading from this pointer or turning it
into a reference is undefined behavior unless the
MaybeUninit<T>
is initialized.
§
Examples
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<Vec<u32>>::uninit();
x.write(
vec!
[
0
,
1
,
2
]);
// Create a reference into the `MaybeUninit<Vec<u32>>`.
// This is okay because we initialized it.
let
x_vec =
unsafe
{
&mut *
x.as_mut_ptr() };
x_vec.push(
3
);
assert_eq!
(x_vec.len(),
4
);
Incorrect
usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<Vec<u32>>::uninit();
let
x_vec =
unsafe
{
&mut *
x.as_mut_ptr() };
// We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
(Notice that the rules around references to uninitialized data are not finalized yet, but
until they are, it is advisable to avoid them.)
1.36.0 (const: 1.59.0)
·
Source
pub const unsafe fn
assume_init
(self) -> T
Extracts the value from the
MaybeUninit<T>
container. This is a great way
to ensure that the data will get dropped, because the resulting
T
is
subject to the usual drop handling.
§
Safety
It is up to the caller to guarantee that the
MaybeUninit<T>
really is in an initialized
state. Calling this when the content is not yet fully initialized causes immediate undefined
behavior. The
type-level documentation
contains more information about
this initialization invariant.
On top of that, remember that most types have additional invariants beyond merely
being considered initialized at the type level. For example, a
1
-initialized
Vec<T>
is considered initialized (under the current implementation; this does not constitute
a stable guarantee) because the only requirement the compiler knows about it
is that the data pointer must be non-null. Creating such a
Vec<T>
does not cause
immediate
undefined behavior, but will cause undefined behavior with most
safe operations (including dropping it).
§
Examples
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<bool>::uninit();
x.write(
true
);
let
x_init =
unsafe
{ x.assume_init() };
assert_eq!
(x_init,
true
);
Incorrect
usage of this method:
use
std::mem::MaybeUninit;
let
x = MaybeUninit::<Vec<u32>>::uninit();
let
x_init =
unsafe
{ x.assume_init() };
// `x` had not been initialized yet, so this last line caused undefined behavior. ⚠️
1.60.0 (const: 1.75.0)
·
Source
pub const unsafe fn
assume_init_read
(&self) -> T
Reads the value from the
MaybeUninit<T>
container. The resulting
T
is subject
to the usual drop handling.
Whenever possible, it is preferable to use
assume_init
instead, which
prevents duplicating the content of the
MaybeUninit<T>
.
§
Safety
It is up to the caller to guarantee that the
MaybeUninit<T>
really is in an initialized
state. Calling this when the content is not yet fully initialized causes undefined
behavior. The
type-level documentation
contains more information about
this initialization invariant.
Moreover, similar to the
ptr::read
function, this function creates a
bitwise copy of the contents, regardless whether the contained type
implements the
Copy
trait or not. When using multiple copies of the
data (by calling
assume_init_read
multiple times, or first calling
assume_init_read
and then
assume_init
), it is your responsibility
to ensure that data may indeed be duplicated.
§
Examples
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<u32>::uninit();
x.write(
13
);
let
x1 =
unsafe
{ x.assume_init_read() };
// `u32` is `Copy`, so we may read multiple times.
let
x2 =
unsafe
{ x.assume_init_read() };
assert_eq!
(x1, x2);
let
mut
x = MaybeUninit::<
Option
<Vec<u32>>>::uninit();
x.write(
None
);
let
x1 =
unsafe
{ x.assume_init_read() };
// Duplicating a `None` value is okay, so we may read multiple times.
let
x2 =
unsafe
{ x.assume_init_read() };
assert_eq!
(x1, x2);
Incorrect
usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<
Option
<Vec<u32>>>::uninit();
x.write(
Some
(
vec!
[
0
,
1
,
2
]));
let
x1 =
unsafe
{ x.assume_init_read() };
let
x2 =
unsafe
{ x.assume_init_read() };
// We now created two copies of the same vector, leading to a double-free ⚠️ when
// they both get dropped!
1.60.0
·
Source
pub unsafe fn
assume_init_drop
(&mut self)
Drops the contained value in place.
If you have ownership of the
MaybeUninit
, you can also use
assume_init
as an alternative.
§
Safety
It is up to the caller to guarantee that the
MaybeUninit<T>
really is
in an initialized state. Calling this when the content is not yet fully
initialized causes undefined behavior.
On top of that, all additional invariants of the type
T
must be
satisfied, as the
Drop
implementation of
T
(or its members) may
rely on this. For example, setting a
Vec<T>
to an invalid but
non-null address makes it initialized (under the current implementation;
this does not constitute a stable guarantee), because the only
requirement the compiler knows about it is that the data pointer must be
non-null. Dropping such a
Vec<T>
however will cause undefined
behavior.
1.55.0 (const: 1.59.0)
·
Source
pub const unsafe fn
assume_init_ref
(&self) ->
&T
Gets a shared reference to the contained value.
This can be useful when we want to access a
MaybeUninit
that has been
initialized but don’t have ownership of the
MaybeUninit
(preventing the use
of
.assume_init()
).
§
Safety
Calling this when the content is not yet fully initialized causes undefined
behavior: it is up to the caller to guarantee that the
MaybeUninit<T>
really
is in an initialized state.
§
Examples
§
Correct usage of this method:
use
std::mem::MaybeUninit;
let
mut
x = MaybeUninit::<Vec<u32>>::uninit();
// Initialize `x`:
x.write(
vec!
[
1
,
2
,
3
]);
// Now that our `MaybeUninit<_>` is known to be initialized, it is okay to
// create a shared reference to it:
let
x:
&
Vec<u32> =
unsafe
{
// SAFETY: `x` has been initialized.
x.assume_init_ref()
};
assert_eq!
(x,
&
vec!
[
1
,
2
,
3
]);
§
Incorrect
usages of this method:
use
std::mem::MaybeUninit;
let
x = MaybeUninit::<Vec<u32>>::uninit();
let
x_vec:
&
Vec<u32> =
unsafe
{ x.assume_init_ref() };
// We have created a reference to an uninitialized vector! This is undefined behavior. ⚠️
use
std::{cell::Cell, mem::MaybeUninit};
let
b = MaybeUninit::<Cell<bool>>::uninit();
// Initialize the `MaybeUninit` using `Cell::set`:
unsafe
{
    b.assume_init_ref().set(
true
);
// ^^^^^^^^^^^^^^^
   // Reference to an uninitialized `Cell<bool>`: UB!
}
1.55.0 (const: 1.84.0)
·
Source
pub const unsafe fn
assume_init_mut
(&mut self) ->
&mut T
Gets a mutable (unique) reference to the contained value.
This can be useful when we want to access a
MaybeUninit
that has been
initialized but don’t have ownership of the
MaybeUninit
(preventing the use
of
.assume_init()
).
§
Safety
Calling this when the content is not yet fully initialized causes undefined
behavior: it is up to the caller to guarantee that the
MaybeUninit<T>
really
is in an initialized state. For instance,
.assume_init_mut()
cannot be used to
initialize a
MaybeUninit
.
§
Examples
§
Correct usage of this method:
use
std::mem::MaybeUninit;
extern
"C"
{
/// Initializes *all* the bytes of the input buffer.
fn
initialize_buffer(buf:
*mut
[u8;
1024
]);
}
let
mut
buf = MaybeUninit::<[u8;
1024
]>::uninit();
// Initialize `buf`:
unsafe
{ initialize_buffer(buf.as_mut_ptr()); }
// Now we know that `buf` has been initialized, so we could `.assume_init()` it.
// However, using `.assume_init()` may trigger a `memcpy` of the 1024 bytes.
// To assert our buffer has been initialized without copying it, we upgrade
// the `&mut MaybeUninit<[u8; 1024]>` to a `&mut [u8; 1024]`:
let
buf:
&mut
[u8;
1024
] =
unsafe
{
// SAFETY: `buf` has been initialized.
buf.assume_init_mut()
};
// Now we can use `buf` as a normal slice:
buf.sort_unstable();
assert!
(
    buf.windows(
2
).all(|pair| pair[
0
] <= pair[
1
]),
"buffer is sorted"
,
);
§
Incorrect
usages of this method:
You cannot use
.assume_init_mut()
to initialize a value:
use
std::mem::MaybeUninit;
let
mut
b = MaybeUninit::<bool>::uninit();
unsafe
{
*
b.assume_init_mut() =
true
;
// We have created a (mutable) reference to an uninitialized `bool`!
    // This is undefined behavior. ⚠️
}
For instance, you cannot
Read
into an uninitialized buffer:
use
std::{io, mem::MaybeUninit};
fn
read_chunk (reader:
&
'_
mut
dyn
io::Read) -> io::Result<[u8;
64
]>
{
let
mut
buffer = MaybeUninit::<[u8;
64
]>::uninit();
    reader.read_exact(
unsafe
{ buffer.assume_init_mut() })
?
;
// ^^^^^^^^^^^^^^^^^^^^^^^^
                            // (mutable) reference to uninitialized memory!
                            // This is undefined behavior.
Ok
(
unsafe
{ buffer.assume_init() })
}
Nor can you use direct field access to do field-by-field gradual initialization:
use
std::{mem::MaybeUninit, ptr};
struct
Foo {
    a: u32,
    b: u8,
}
let
foo: Foo =
unsafe
{
let
mut
foo = MaybeUninit::<Foo>::uninit();
    ptr::write(
&mut
foo.assume_init_mut().a
as
*mut
u32,
1337
);
// ^^^^^^^^^^^^^^^^^^^^^
                 // (mutable) reference to uninitialized memory!
                 // This is undefined behavior.
ptr::write(
&mut
foo.assume_init_mut().b
as
*mut
u8,
42
);
// ^^^^^^^^^^^^^^^^^^^^^
                 // (mutable) reference to uninitialized memory!
                 // This is undefined behavior.
foo.assume_init()
};
Source
pub const unsafe fn
array_assume_init
<const N:
usize
>(
    array: [
MaybeUninit
<T>;
N
],
) ->
[T; N]
🔬
This is a nightly-only experimental API. (
maybe_uninit_array_assume_init
#96097
)
Extracts the values from an array of
MaybeUninit
containers.
§
Safety
It is up to the caller to guarantee that all elements of the array are
in an initialized state.
§
Examples
#![feature(maybe_uninit_array_assume_init)]
use
std::mem::MaybeUninit;
let
mut
array: [MaybeUninit<i32>;
3
] = [MaybeUninit::uninit();
3
];
array[
0
].write(
0
);
array[
1
].write(
1
);
array[
2
].write(
2
);
// SAFETY: Now safe as we initialised all elements
let
array =
unsafe
{
    MaybeUninit::array_assume_init(array)
};
assert_eq!
(array, [
0
,
1
,
2
]);
Source
pub const fn
as_bytes
(&self) -> &[
MaybeUninit
<
u8
>]
🔬
This is a nightly-only experimental API. (
maybe_uninit_as_bytes
#93092
)
Returns the contents of this
MaybeUninit
as a slice of potentially uninitialized bytes.
Note that even if the contents of a
MaybeUninit
have been initialized, the value may still
contain padding bytes which are left uninitialized.
§
Examples
#![feature(maybe_uninit_as_bytes, maybe_uninit_slice)]
use
std::mem::MaybeUninit;
let
val =
0x12345678_i32
;
let
uninit = MaybeUninit::new(val);
let
uninit_bytes = uninit.as_bytes();
let
bytes =
unsafe
{ uninit_bytes.assume_init_ref() };
assert_eq!
(bytes, val.to_ne_bytes());
Source
pub const fn
as_bytes_mut
(&mut self) -> &mut [
MaybeUninit
<
u8
>]
🔬
This is a nightly-only experimental API. (
maybe_uninit_as_bytes
#93092
)
Returns the contents of this
MaybeUninit
as a mutable slice of potentially uninitialized
bytes.
Note that even if the contents of a
MaybeUninit
have been initialized, the value may still
contain padding bytes which are left uninitialized.
§
Examples
#![feature(maybe_uninit_as_bytes)]
use
std::mem::MaybeUninit;
let
val =
0x12345678_i32
;
let
mut
uninit = MaybeUninit::new(val);
let
uninit_bytes = uninit.as_bytes_mut();
if
cfg!
(target_endian =
"little"
) {
    uninit_bytes[
0
].write(
0xcd
);
}
else
{
    uninit_bytes[
3
].write(
0xcd
);
}
let
val2 =
unsafe
{ uninit.assume_init() };
assert_eq!
(val2,
0x123456cd
);
Source
pub const unsafe fn
slice_assume_init_ref
(slice: &[
MaybeUninit
<T>]) -> &
[T]
👎
Deprecated since 1.83.0: replaced by inherent assume_init_ref method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_slice
#63569
)
Deprecated version of
slice::assume_init_ref
.
Source
pub const unsafe fn
slice_assume_init_mut
(
    slice: &mut [
MaybeUninit
<T>],
) -> &mut
[T]
👎
Deprecated since 1.83.0: replaced by inherent assume_init_mut method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_slice
#63569
)
Deprecated version of
slice::assume_init_mut
.
Source
pub const fn
slice_as_ptr
(this: &[
MaybeUninit
<T>]) ->
*const T
🔬
This is a nightly-only experimental API. (
maybe_uninit_slice
#63569
)
Gets a pointer to the first element of the array.
Source
pub const fn
slice_as_mut_ptr
(this: &mut [
MaybeUninit
<T>]) ->
*mut T
🔬
This is a nightly-only experimental API. (
maybe_uninit_slice
#63569
)
Gets a mutable pointer to the first element of the array.
Source
pub fn
copy_from_slice
<'a>(
    this: &'a mut [
MaybeUninit
<T>],
    src: &
[T]
,
) -> &'a mut
[T]
where
    T:
Copy
,
👎
Deprecated since 1.83.0: replaced by inherent write_copy_of_slice method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_write_slice
#79995
)
Deprecated version of
slice::write_copy_of_slice
.
Source
pub fn
clone_from_slice
<'a>(
    this: &'a mut [
MaybeUninit
<T>],
    src: &
[T]
,
) -> &'a mut
[T]
where
    T:
Clone
,
👎
Deprecated since 1.83.0: replaced by inherent write_clone_of_slice method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_write_slice
#79995
)
Deprecated version of
slice::write_clone_of_slice
.
Source
pub fn
fill
<'a>(this: &'a mut [
MaybeUninit
<T>], value: T) -> &'a mut
[T]
where
    T:
Clone
,
👎
Deprecated since 1.83.0: replaced by inherent write_filled method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_fill
#117428
)
Deprecated version of
slice::write_filled
.
Source
pub fn
fill_with
<'a, F>(this: &'a mut [
MaybeUninit
<T>], f: F) -> &'a mut
[T]
where
    F:
FnMut
() -> T,
👎
Deprecated since 1.83.0: replaced by inherent write_with method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_fill
#117428
)
Deprecated version of
slice::write_with
.
Source
pub fn
fill_from
<'a, I>(
    this: &'a mut [
MaybeUninit
<T>],
    it: I,
) -> (&'a mut
[T]
, &'a mut [
MaybeUninit
<T>])
where
    I:
IntoIterator
<Item = T>,
👎
Deprecated since 1.83.0: replaced by inherent write_iter method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_fill
#117428
)
Deprecated version of
slice::write_iter
.
Source
pub fn
slice_as_bytes
(this: &[
MaybeUninit
<T>]) -> &[
MaybeUninit
<
u8
>]
👎
Deprecated since 1.83.0: replaced by inherent as_bytes method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_as_bytes
#93092
)
Deprecated version of
slice::as_bytes
.
Source
pub fn
slice_as_bytes_mut
(this: &mut [
MaybeUninit
<T>]) -> &mut [
MaybeUninit
<
u8
>]
👎
Deprecated since 1.83.0: replaced by inherent as_bytes_mut method; will eventually be removed
🔬
This is a nightly-only experimental API. (
maybe_uninit_as_bytes
#93092
)
Deprecated version of
slice::as_bytes_mut
.
Source
§
impl<T, const N:
usize
>
MaybeUninit
<
[T; N]
>
Source
pub const fn
transpose
(self) -> [
MaybeUninit
<T>;
N
]
🔬
This is a nightly-only experimental API. (
maybe_uninit_uninit_array_transpose
#96097
)
Transposes a
MaybeUninit<[T; N]>
into a
[MaybeUninit<T>; N]
.
§
Examples
#![feature(maybe_uninit_uninit_array_transpose)]
let
data: [MaybeUninit<u8>;
1000
] = MaybeUninit::uninit().transpose();
Source
§
impl<T, A>
Box
<
MaybeUninit
<T>, A>
where
    A:
Allocator
,
1.82.0
·
Source
pub unsafe fn
assume_init
(self) ->
Box
<T, A>
Converts to
Box<T, A>
.
§
Safety
As with
MaybeUninit::assume_init
,
it is up to the caller to guarantee that the value
really is in an initialized state.
Calling this when the content is not yet fully initialized
causes immediate undefined behavior.
§
Examples
let
mut
five = Box::<u32>::new_uninit();
// Deferred initialization:
five.write(
5
);
let
five: Box<u32> =
unsafe
{ five.assume_init() };
assert_eq!
(
*
five,
5
)
1.87.0
·
Source
pub fn
write
(boxed:
Box
<
MaybeUninit
<T>, A>, value: T) ->
Box
<T, A>
Writes the value and converts to
Box<T, A>
.
This method converts the box similarly to
Box::assume_init
but
writes
value
into it before conversion thus guaranteeing safety.
In some scenarios use of this method may improve performance because
the compiler may be able to optimize copying from stack.
§
Examples
let
big_box = Box::<[usize;
1024
]>::new_uninit();
let
mut
array = [
0
;
1024
];
for
(i, place)
in
array.iter_mut().enumerate() {
*
place = i;
}
// The optimizer may be able to elide this copy, so previous code writes
// to heap directly.
let
big_box = Box::write(big_box, array);
for
(i, x)
in
big_box.iter().enumerate() {
assert_eq!
(
*
x, i);
}
Trait Implementations
§
1.36.0
·
Source
§
impl<T>
Clone
for
MaybeUninit
<T>
where
    T:
Copy
,
Source
§
fn
clone
(&self) ->
MaybeUninit
<T>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.41.0
·
Source
§
impl<T>
Debug
for
MaybeUninit
<T>
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.36.0
·
Source
§
impl<T>
Copy
for
MaybeUninit
<T>
where
    T:
Copy
,
Auto Trait Implementations
§
§
impl<T>
Freeze
for
MaybeUninit
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
MaybeUninit
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
MaybeUninit
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
MaybeUninit
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
MaybeUninit
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
MaybeUninit
<T>
where
    T:
UnwindSafe
,
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.