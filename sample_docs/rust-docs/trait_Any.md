Any in std::any - Rust
std
::
any
Trait
Any
Copy item path
1.0.0
·
Source
pub trait Any: 'static {
    // Required method
    fn
type_id
(&self) ->
TypeId
;
}
Expand description
A trait to emulate dynamic typing.
Most types implement
Any
. However, any type which contains a non-
'static
reference does not.
See the
module-level documentation
for more details.
Required Methods
§
1.34.0
·
Source
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
If called on a
dyn Any
trait object
(or a trait object of a subtrait of
Any
),
this returns the
TypeId
of the underlying
concrete type, not that of
dyn Any
itself.
§
Examples
use
std::any::{Any, TypeId};
fn
is_string(s:
&
dyn
Any) -> bool {
    TypeId::of::<String>() == s.type_id()
}
assert_eq!
(is_string(
&
0
),
false
);
assert_eq!
(is_string(
&
"cookie monster"
.to_string()),
true
);
Implementations
§
Source
§
impl dyn
Any
1.0.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Any
,
Returns
true
if the inner type is the same as
T
.
§
Examples
use
std::any::Any;
fn
is_string(s:
&
dyn
Any) {
if
s.is::<String>() {
println!
(
"It's a string!"
);
    }
else
{
println!
(
"Not a string..."
);
    }
}

is_string(
&
0
);
is_string(
&
"cookie monster"
.to_string());
1.0.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Any
,
Returns some reference to the inner value if it is of type
T
, or
None
if it isn’t.
§
Examples
use
std::any::Any;
fn
print_if_string(s:
&
dyn
Any) {
if let
Some
(string) = s.downcast_ref::<String>() {
println!
(
"It's a string({}): '{}'"
, string.len(), string);
    }
else
{
println!
(
"Not a string..."
);
    }
}

print_if_string(
&
0
);
print_if_string(
&
"cookie monster"
.to_string());
1.0.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Any
,
Returns some mutable reference to the inner value if it is of type
T
, or
None
if it isn’t.
§
Examples
use
std::any::Any;
fn
modify_if_u32(s:
&mut
dyn
Any) {
if let
Some
(num) = s.downcast_mut::<u32>() {
*
num =
42
;
    }
}
let
mut
x =
10u32
;
let
mut
s =
"starlord"
.to_string();

modify_if_u32(
&mut
x);
modify_if_u32(
&mut
s);
assert_eq!
(x,
42
);
assert_eq!
(
&
s,
"starlord"
);
Source
pub unsafe fn
downcast_ref_unchecked
<T>(&self) ->
&T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Returns a reference to the inner value as type
dyn T
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_ref_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
pub unsafe fn
downcast_mut_unchecked
<T>(&mut self) ->
&mut T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Returns a mutable reference to the inner value as type
dyn T
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
mut
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
*
x.downcast_mut_unchecked::<usize>() +=
1
;
}
assert_eq!
(
*
x.downcast_ref::<usize>().unwrap(),
2
);
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
§
impl dyn
Any
+
Send
1.0.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Any
,
Forwards to the method defined on the type
dyn Any
.
§
Examples
use
std::any::Any;
fn
is_string(s:
&
(
dyn
Any + Send)) {
if
s.is::<String>() {
println!
(
"It's a string!"
);
    }
else
{
println!
(
"Not a string..."
);
    }
}

is_string(
&
0
);
is_string(
&
"cookie monster"
.to_string());
1.0.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Any
,
Forwards to the method defined on the type
dyn Any
.
§
Examples
use
std::any::Any;
fn
print_if_string(s:
&
(
dyn
Any + Send)) {
if let
Some
(string) = s.downcast_ref::<String>() {
println!
(
"It's a string({}): '{}'"
, string.len(), string);
    }
else
{
println!
(
"Not a string..."
);
    }
}

print_if_string(
&
0
);
print_if_string(
&
"cookie monster"
.to_string());
1.0.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Any
,
Forwards to the method defined on the type
dyn Any
.
§
Examples
use
std::any::Any;
fn
modify_if_u32(s:
&mut
(
dyn
Any + Send)) {
if let
Some
(num) = s.downcast_mut::<u32>() {
*
num =
42
;
    }
}
let
mut
x =
10u32
;
let
mut
s =
"starlord"
.to_string();

modify_if_u32(
&mut
x);
modify_if_u32(
&mut
s);
assert_eq!
(x,
42
);
assert_eq!
(
&
s,
"starlord"
);
Source
pub unsafe fn
downcast_ref_unchecked
<T>(&self) ->
&T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Forwards to the method defined on the type
dyn Any
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_ref_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
pub unsafe fn
downcast_mut_unchecked
<T>(&mut self) ->
&mut T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Forwards to the method defined on the type
dyn Any
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
mut
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
*
x.downcast_mut_unchecked::<usize>() +=
1
;
}
assert_eq!
(
*
x.downcast_ref::<usize>().unwrap(),
2
);
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
§
impl dyn
Any
+
Sync
+
Send
1.28.0
·
Source
pub fn
is
<T>(&self) ->
bool
where
    T:
Any
,
Forwards to the method defined on the type
Any
.
§
Examples
use
std::any::Any;
fn
is_string(s:
&
(
dyn
Any + Send + Sync)) {
if
s.is::<String>() {
println!
(
"It's a string!"
);
    }
else
{
println!
(
"Not a string..."
);
    }
}

is_string(
&
0
);
is_string(
&
"cookie monster"
.to_string());
1.28.0
·
Source
pub fn
downcast_ref
<T>(&self) ->
Option
<
&T
>
where
    T:
Any
,
Forwards to the method defined on the type
Any
.
§
Examples
use
std::any::Any;
fn
print_if_string(s:
&
(
dyn
Any + Send + Sync)) {
if let
Some
(string) = s.downcast_ref::<String>() {
println!
(
"It's a string({}): '{}'"
, string.len(), string);
    }
else
{
println!
(
"Not a string..."
);
    }
}

print_if_string(
&
0
);
print_if_string(
&
"cookie monster"
.to_string());
1.28.0
·
Source
pub fn
downcast_mut
<T>(&mut self) ->
Option
<
&mut T
>
where
    T:
Any
,
Forwards to the method defined on the type
Any
.
§
Examples
use
std::any::Any;
fn
modify_if_u32(s:
&mut
(
dyn
Any + Send + Sync)) {
if let
Some
(num) = s.downcast_mut::<u32>() {
*
num =
42
;
    }
}
let
mut
x =
10u32
;
let
mut
s =
"starlord"
.to_string();

modify_if_u32(
&mut
x);
modify_if_u32(
&mut
s);
assert_eq!
(x,
42
);
assert_eq!
(
&
s,
"starlord"
);
Source
pub unsafe fn
downcast_ref_unchecked
<T>(&self) ->
&T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Forwards to the method defined on the type
Any
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_ref_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
pub unsafe fn
downcast_mut_unchecked
<T>(&mut self) ->
&mut T
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Forwards to the method defined on the type
Any
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
mut
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
*
x.downcast_mut_unchecked::<usize>() +=
1
;
}
assert_eq!
(
*
x.downcast_ref::<usize>().unwrap(),
2
);
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
§
impl<A>
Box
<dyn
Any
, A>
where
    A:
Allocator
,
1.0.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
<dyn
Any
, A>>
where
    T:
Any
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
dyn
Any>) {
if let
Ok
(string) = value.downcast::<String>() {
println!
(
"String ({}): {}"
, string.len(), string);
    }
}
let
my_string =
"Hello World"
.to_string();
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
<T, A>
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
§
impl<A>
Box
<dyn
Any
+
Send
, A>
where
    A:
Allocator
,
1.0.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
<dyn
Any
+
Send
, A>>
where
    T:
Any
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
dyn
Any + Send>) {
if let
Ok
(string) = value.downcast::<String>() {
println!
(
"String ({}): {}"
, string.len(), string);
    }
}
let
my_string =
"Hello World"
.to_string();
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
<T, A>
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any + Send> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Source
§
impl<A>
Box
<dyn
Any
+
Sync
+
Send
, A>
where
    A:
Allocator
,
1.51.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
<dyn
Any
+
Sync
+
Send
, A>>
where
    T:
Any
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
dyn
Any + Send + Sync>) {
if let
Ok
(string) = value.downcast::<String>() {
println!
(
"String ({}): {}"
, string.len(), string);
    }
}
let
my_string =
"Hello World"
.to_string();
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
<T, A>
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any + Send + Sync> = Box::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Trait Implementations
§
1.0.0
·
Source
§
impl
Debug
for dyn
Any
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
1.0.0
·
Source
§
impl
Debug
for dyn
Any
+
Send
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
1.28.0
·
Source
§
impl
Debug
for dyn
Any
+
Sync
+
Send
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
Implementors
§
1.0.0
·
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,