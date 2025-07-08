Receiver in std::ops - Rust
std
::
ops
Trait
Receiver
Copy item path
Source
pub trait Receiver {
    type
Target
: ?
Sized
;
}
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
Expand description
Indicates that a struct can be used as a method receiver.
That is, a type can use this type as a type of
self
, like this:
ⓘ
use
std::ops::Receiver;
struct
SmartPointer<T>(T);
impl
<T> Receiver
for
SmartPointer<T> {
type
Target = T;
}
struct
MyContainedType;
impl
MyContainedType {
fn
method(
self
: SmartPointer<
Self
>) {
// ...
}
}
fn
main() {
let
ptr = SmartPointer(MyContainedType);
  ptr.method();
}
This trait is blanket implemented for any type which implements
Deref
, which includes stdlib pointer types like
Box<T>
,
Rc<T>
,
&T
,
and
Pin<P>
. For that reason, it’s relatively rare to need to
implement this directly. You’ll typically do this only if you need
to implement a smart pointer type which can’t implement
Deref
; perhaps
because you’re interfacing with another programming language and can’t
guarantee that references comply with Rust’s aliasing rules.
When looking for method candidates, Rust will explore a chain of possible
Receiver
s, so for example each of the following methods work:
use
std::boxed::Box;
use
std::rc::Rc;
// Both `Box` and `Rc` (indirectly) implement Receiver
struct
MyContainedType;
fn
main() {
let
t = Rc::new(Box::new(MyContainedType));
  t.method_a();
  t.method_b();
  t.method_c();
}
impl
MyContainedType {
fn
method_a(
&
self
) {

  }
fn
method_b(
self
:
&
Box<
Self
>) {

  }
fn
method_c(
self
:
&
Rc<Box<
Self
>>) {

  }
}
Required Associated Types
§
Source
type
Target
: ?
Sized
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
Implementors
§
Source
§
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T