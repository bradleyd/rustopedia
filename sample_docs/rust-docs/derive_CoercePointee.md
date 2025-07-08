CoercePointee in std::marker - Rust
std
::
marker
Derive Macro
CoercePointee
Copy item path
Source
#[derive(CoercePointee)]
🔬
This is a nightly-only experimental API. (
derive_coerce_pointee
#123430
)
Expand description
Derive macro that makes a smart pointer usable with trait objects.
§
What this macro does
This macro is intended to be used with user-defined pointer types, and makes it possible to
perform coercions on the pointee of the user-defined pointer. There are two aspects to this:
§
Unsizing coercions of the pointee
By using the macro, the following example will compile:
#![feature(derive_coerce_pointee)]
use
std::marker::CoercePointee;
use
std::ops::Deref;
#[derive(CoercePointee)]
#[repr(transparent)]
struct
MySmartPointer<T:
?
Sized>(Box<T>);
impl
<T:
?
Sized> Deref
for
MySmartPointer<T> {
type
Target = T;
fn
deref(
&
self
) ->
&
T {
&
self
.
0
}
}
trait
MyTrait {}
impl
MyTrait
for
i32 {}
fn
main() {
let
ptr: MySmartPointer<i32> = MySmartPointer(Box::new(
4
));
// This coercion would be an error without the derive.
let
ptr: MySmartPointer<
dyn
MyTrait> = ptr;
}
Without the
#[derive(CoercePointee)]
macro, this example would fail with the following error:
error[E0308]: mismatched types
  --> src/main.rs:11:44
   |
11 |     let ptr: MySmartPointer<dyn MyTrait> = ptr;
   |              ---------------------------   ^^^ expected `MySmartPointer<dyn MyTrait>`, found `MySmartPointer<i32>`
   |              |
   |              expected due to this
   |
   = note: expected struct `MySmartPointer<dyn MyTrait>`
              found struct `MySmartPointer<i32>`
   = help: `i32` implements `MyTrait` so you could box the found value and coerce it to the trait object `Box<dyn MyTrait>`, you will have to change the expected type as well
§
Dyn compatibility
This macro allows you to dispatch on the user-defined pointer type. That is, traits using the
type as a receiver are dyn-compatible. For example, this compiles:
#![feature(arbitrary_self_types, derive_coerce_pointee)]
use
std::marker::CoercePointee;
use
std::ops::Deref;
#[derive(CoercePointee)]
#[repr(transparent)]
struct
MySmartPointer<T:
?
Sized>(Box<T>);
impl
<T:
?
Sized> Deref
for
MySmartPointer<T> {
type
Target = T;
fn
deref(
&
self
) ->
&
T {
&
self
.
0
}
}
// You can always define this trait. (as long as you have #![feature(arbitrary_self_types)])
trait
MyTrait {
fn
func(
self
: MySmartPointer<
Self
>);
}
// But using `dyn MyTrait` requires #[derive(CoercePointee)].
fn
call_func(value: MySmartPointer<
dyn
MyTrait>) {
    value.func();
}
If you remove the
#[derive(CoercePointee)]
annotation from the struct, then the above example
will fail with this error message:
error[E0038]: the trait `MyTrait` is not dyn compatible
  --> src/lib.rs:21:36
   |
17 |     fn func(self: MySmartPointer<Self>);
   |                   -------------------- help: consider changing method `func`'s `self` parameter to be `&self`: `&Self`
...
21 | fn call_func(value: MySmartPointer<dyn MyTrait>) {
   |                                    ^^^^^^^^^^^ `MyTrait` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> src/lib.rs:17:19
   |
16 | trait MyTrait {
   |       ------- this trait is not dyn compatible...
17 |     fn func(self: MySmartPointer<Self>);
   |                   ^^^^^^^^^^^^^^^^^^^^ ...because method `func`'s `self` parameter cannot be dispatched on
§
Requirements for using the macro
This macro can only be used if:
The type is a
#[repr(transparent)]
struct.
The type of its non-zero-sized field must either be a standard library pointer type
(reference, raw pointer,
NonNull
,
Box
,
Rc
,
Arc
, etc.) or another user-defined type
also using the
#[derive(CoercePointee)]
macro.
Zero-sized fields must not mention any generic parameters unless the zero-sized field has
type
PhantomData
.
§
Multiple type parameters
If the type has multiple type parameters, then you must explicitly specify which one should be
used for dynamic dispatch. For example:
#[derive(CoercePointee)]
#[repr(transparent)]
struct
MySmartPointer<
#[pointee]
T:
?
Sized, U> {
    ptr: Box<T>,
    _phantom: PhantomData<U>,
}
Specifying
#[pointee]
when the struct has only one type parameter is allowed, but not required.
§
Examples
A custom implementation of the
Rc
type:
#![feature(derive_coerce_pointee)]
use
std::marker::CoercePointee;
use
std::ops::Deref;
use
std::ptr::NonNull;
#[derive(CoercePointee)]
#[repr(transparent)]
pub struct
Rc<T:
?
Sized> {
    inner: NonNull<RcInner<T>>,
}
struct
RcInner<T:
?
Sized> {
    refcount: usize,
    value: T,
}
impl
<T:
?
Sized> Deref
for
Rc<T> {
type
Target = T;
fn
deref(
&
self
) ->
&
T {
let
ptr =
self
.inner.as_ptr();
unsafe
{
&
(
*
ptr).value }
    }
}
impl
<T> Rc<T> {
pub fn
new(value: T) ->
Self
{
let
inner = Box::new(RcInner {
            refcount:
1
,
            value,
        });
Self
{
            inner: NonNull::from(Box::leak(inner)),
        }
    }
}
impl
<T:
?
Sized> Clone
for
Rc<T> {
fn
clone(
&
self
) ->
Self
{
// A real implementation would handle overflow here.
unsafe
{ (
*
self
.inner.as_ptr()).refcount +=
1
};
Self
{ inner:
self
.inner }
    }
}
impl
<T:
?
Sized> Drop
for
Rc<T> {
fn
drop(
&mut
self
) {
let
ptr =
self
.inner.as_ptr();
unsafe
{ (
*
ptr).refcount -=
1
};
if unsafe
{ (
*
ptr).refcount } ==
0
{
            drop(
unsafe
{ Box::from_raw(ptr) });
        }
    }
}