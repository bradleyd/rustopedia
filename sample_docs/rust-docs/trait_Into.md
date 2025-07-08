Into in std::convert - Rust
std
::
convert
Trait
Into
Copy item path
1.0.0
·
Source
pub trait Into<T>:
Sized
{
    // Required method
    fn
into
(self) -> T;
}
Expand description
A value-to-value conversion that consumes the input value. The
opposite of
From
.
One should avoid implementing
Into
and implement
From
instead.
Implementing
From
automatically provides one with an implementation of
Into
thanks to the blanket implementation in the standard library.
Prefer using
Into
over
From
when specifying trait bounds on a generic function
to ensure that types that only implement
Into
can be used as well.
Note: This trait must not fail
. If the conversion can fail, use
TryInto
.
§
Generic Implementations
From
<T> for U
implies
Into<U> for T
Into
is reflexive, which means that
Into<T> for T
is implemented
§
Implementing
Into
for conversions to external types in old versions of Rust
Prior to Rust 1.41, if the destination type was not part of the current crate
then you couldn’t implement
From
directly.
For example, take this code:
struct
Wrapper<T>(Vec<T>);
impl
<T> From<Wrapper<T>>
for
Vec<T> {
fn
from(w: Wrapper<T>) -> Vec<T> {
        w.
0
}
}
This will fail to compile in older versions of the language because Rust’s orphaning rules
used to be a little bit more strict. To bypass this, you could implement
Into
directly:
struct
Wrapper<T>(Vec<T>);
impl
<T> Into<Vec<T>>
for
Wrapper<T> {
fn
into(
self
) -> Vec<T> {
self
.
0
}
}
It is important to understand that
Into
does not provide a
From
implementation
(as
From
does with
Into
). Therefore, you should always try to implement
From
and then fall back to
Into
if
From
can’t be implemented.
§
Examples
String
implements
Into
<
Vec
<
u8
>>
:
In order to express that we want a generic function to take all arguments that can be
converted to a specified type
T
, we can use a trait bound of
Into
<T>
.
For example: The function
is_hello
takes all arguments that can be converted into a
Vec
<
u8
>
.
fn
is_hello<T: Into<Vec<u8>>>(s: T) {
let
bytes =
b"hello"
.to_vec();
assert_eq!
(bytes, s.into());
}
let
s =
"hello"
.to_string();
is_hello(s);
Required Methods
§
1.0.0
·
Source
fn
into
(self) -> T
Converts this type into the (usually inferred) input type.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.0.0
·
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,