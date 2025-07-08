CloneToUninit in std::clone - Rust
std
::
clone
Trait
CloneToUninit
Copy item path
Source
pub unsafe trait CloneToUninit {
    // Required method
    unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
);
}
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Expand description
A generalization of
Clone
to
dynamically-sized types
stored in arbitrary containers.
This trait is implemented for all types implementing
Clone
,
slices
of all
such types, and other dynamically-sized types in the standard library.
You may also implement this trait to enable cloning custom DSTs
(structures containing dynamically-sized fields), or use it as a supertrait to enable
cloning a
trait object
.
This trait is normally used via operations on container types which support DSTs,
so you should not typically need to call
.clone_to_uninit()
explicitly except when
implementing such a container or otherwise performing explicit management of an allocation,
or when implementing
CloneToUninit
itself.
§
Safety
Implementations must ensure that when
.clone_to_uninit(dest)
returns normally rather than
panicking, it always leaves
*dest
initialized as a valid value of type
Self
.
§
Examples
If you are defining a trait, you can add
CloneToUninit
as a supertrait to enable cloning of
dyn
values of your trait:
#![feature(clone_to_uninit)]
use
std::rc::Rc;
trait
Foo: std::fmt::Debug + std::clone::CloneToUninit {
fn
modify(
&mut
self
);
fn
value(
&
self
) -> i32;
}
impl
Foo
for
i32 {
fn
modify(
&mut
self
) {
*
self
*
=
10
;
    }
fn
value(
&
self
) -> i32 {
*
self
}
}
let
first: Rc<
dyn
Foo> = Rc::new(
1234
);
let
mut
second = first.clone();
Rc::make_mut(
&mut
second).modify();
// make_mut() will call clone_to_uninit()
assert_eq!
(first.value(),
1234
);
assert_eq!
(second.value(),
12340
);
The following is an example of implementing
CloneToUninit
for a custom DST.
(It is essentially a limited form of what
derive(CloneToUninit)
would do,
if such a derive macro existed.)
#![feature(clone_to_uninit)]
use
std::clone::CloneToUninit;
use
std::mem::offset_of;
use
std::rc::Rc;
#[derive(PartialEq)]
struct
MyDst<T:
?
Sized> {
    label: String,
    contents: T,
}
unsafe impl
<T:
?
Sized + CloneToUninit> CloneToUninit
for
MyDst<T> {
unsafe fn
clone_to_uninit(
&
self
, dest:
*mut
u8) {
// The offset of `self.contents` is dynamic because it depends on the alignment of T
        // which can be dynamic (if `T = dyn SomeTrait`). Therefore, we have to obtain it
        // dynamically by examining `self`, rather than using `offset_of!`.
        //
        // SAFETY: `self` by definition points somewhere before `&self.contents` in the same
        // allocation.
let
offset_of_contents =
unsafe
{
            (
&
raw
const
self
.contents).byte_offset_from_unsigned(
self
)
        };
// Clone the *sized* fields of `self` (just one, in this example).
        // (By cloning this first and storing it temporarily in a local variable, we avoid
        // leaking it in case of any panic, using the ordinary automatic cleanup of local
        // variables. Such a leak would be sound, but undesirable.)
let
label =
self
.label.clone();
// SAFETY: The caller must provide a `dest` such that these field offsets are valid
        // to write to.
unsafe
{
// Clone the unsized field directly from `self` to `dest`.
self
.contents.clone_to_uninit(dest.add(offset_of_contents));
// Now write all the sized fields.
            //
            // Note that we only do this once all of the clone() and clone_to_uninit() calls
            // have completed, and therefore we know that there are no more possible panics;
            // this ensures no memory leaks in case of panic.
dest.add(
offset_of!
(
Self
, label)).cast::<String>().write(label);
        }
// All fields of the struct have been initialized; therefore, the struct is initialized,
        // and we have satisfied our `unsafe impl CloneToUninit` obligations.
}
}
fn
main() {
// Construct MyDst<[u8; 4]>, then coerce to MyDst<[u8]>.
let
first: Rc<MyDst<[u8]>> = Rc::new(MyDst {
        label: String::from(
"hello"
),
        contents: [
1
,
2
,
3
,
4
],
    });
let
mut
second = first.clone();
// make_mut() will call clone_to_uninit().
for
elem
in
Rc::make_mut(
&mut
second).contents.iter_mut() {
*
elem
*
=
10
;
    }
assert_eq!
(first.contents, [
1
,
2
,
3
,
4
]);
assert_eq!
(second.contents, [
10
,
20
,
30
,
40
]);
assert_eq!
(second.label,
"hello"
);
}
§
See Also
Clone::clone_from
is a safe function which may be used instead when
Self: Sized
and the destination is already initialized; it may be able to reuse allocations owned by
the destination, whereas
clone_to_uninit
cannot, since its destination is assumed to be
uninitialized.
ToOwned
, which allocates a new destination container.
Required Methods
§
Source
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
This is analogous to
std::ptr::write(dest.cast(), self.clone())
,
except that
Self
may be a dynamically-sized type (
!Sized
).
Before this function is called,
dest
may point to uninitialized memory.
After this function is called,
dest
will point to initialized memory; it will be
sound to create a
&Self
reference from the pointer with the
pointer metadata
from
self
.
§
Safety
Behavior is undefined if any of the following conditions are violated:
dest
must be
valid
for writes for
size_of_val(self)
bytes.
dest
must be properly aligned to
align_of_val(self)
.
§
Panics
This function may panic. (For example, it might panic if memory allocation for a clone
of a value owned by
self
fails.)
If the call panics, then
*dest
should be treated as uninitialized memory; it must not be
read or dropped, because even if it was previously valid, it may have been partially
overwritten.
The caller may wish to to take care to deallocate the allocation pointed to by
dest
,
if applicable, to avoid a memory leak (but this is not a requirement).
Implementors should avoid leaking values by, upon unwinding, dropping all component values
that might have already been created. (For example, if a
[Foo]
of length 3 is being
cloned, and the second of the three calls to
Foo::clone()
unwinds, then the first
Foo
cloned should be dropped.)
Implementors
§
Source
§
impl
CloneToUninit
for
str
Source
§
impl
CloneToUninit
for
ByteStr
Source
§
impl
CloneToUninit
for
CStr
Source
§
impl
CloneToUninit
for
OsStr
Source
§
impl
CloneToUninit
for
Path
Source
§
impl<T>
CloneToUninit
for
[T]
where
    T:
Clone
,
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,