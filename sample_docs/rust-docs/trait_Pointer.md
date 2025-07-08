Pointer in std::fmt - Rust
std
::
fmt
Trait
Pointer
Copy item path
1.0.0
·
Source
pub trait Pointer {
    // Required method
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
>;
}
Expand description
p
formatting.
The
Pointer
trait should format its output as a memory location. This is commonly presented
as hexadecimal. For more information on formatters, see
the module-level documentation
.
Printing of pointers is not a reliable way to discover how Rust programs are implemented.
The act of reading an address changes the program itself, and may change how the data is represented
in memory, and may affect which optimizations are applied to the code.
The printed pointer values are not guaranteed to be stable nor unique identifiers of objects.
Rust allows moving values to different memory locations, and may reuse the same memory locations
for different purposes.
There is no guarantee that the printed value can be converted back to a pointer.
§
Examples
Basic usage with
&i32
:
let
x =
&
42
;
let
address =
format!
(
"{x:p}"
);
// this produces something like '0x7f06092ac6d0'
Implementing
Pointer
on a type:
use
std::fmt;
struct
Length(i32);
impl
fmt::Pointer
for
Length {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
// use `as` to convert to a `*const T`, which implements Pointer, which we can use
let
ptr =
self
as
*const
Self
;
        fmt::Pointer::fmt(
&
ptr, f)
    }
}
let
l = Length(
42
);
println!
(
"l is in memory here: {l:p}"
);
let
l_ptr =
format!
(
"{l:018p}"
);
assert_eq!
(l_ptr.len(),
18
);
assert_eq!
(
&
l_ptr[..
2
],
"0x"
);
Required Methods
§
1.0.0
·
Source
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
§
Errors
This function should return
Err
if, and only if, the provided
Formatter
returns
Err
.
String formatting is considered an infallible operation; this function only
returns a
Result
because writing to the underlying stream might fail and it must
provide a way to propagate the fact that an error has occurred back up the stack.
Implementors
§
1.4.0
·
Source
§
impl<F>
Pointer
for F
where
    F:
FnPtr
,
1.33.0
·
Source
§
impl<Ptr>
Pointer
for
Pin
<Ptr>
where
    Ptr:
Pointer
,
1.0.0
·
Source
§
impl<T>
Pointer
for
*const T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Pointer
for
*mut T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Pointer
for
&T
where
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T>
Pointer
for
&mut T
where
    T: ?
Sized
,
1.25.0
·
Source
§
impl<T>
Pointer
for
NonNull
<T>
where
    T: ?
Sized
,
1.24.0
·
Source
§
impl<T>
Pointer
for
AtomicPtr
<T>
1.0.0
·
Source
§
impl<T, A>
Pointer
for
Box
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T, A>
Pointer
for
Rc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A>
Pointer
for
UniqueRc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T, A>
Pointer
for
Arc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,