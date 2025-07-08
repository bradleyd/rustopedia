AddrParseError in std::net - Rust
std
::
net
Struct
AddrParseError
Copy item path
1.0.0
·
Source
pub struct AddrParseError(
/* private fields */
);
Expand description
An error which can be returned when parsing an IP address or a socket address.
This error is used as the error type for the
FromStr
implementation for
IpAddr
,
Ipv4Addr
,
Ipv6Addr
,
SocketAddr
,
SocketAddrV4
, and
SocketAddrV6
.
§
Potential causes
AddrParseError
may be thrown because the provided string does not parse as the given type,
often because it includes information only handled by a different address type.
ⓘ
use
std::net::IpAddr;
let
_foo: IpAddr =
"127.0.0.1:8080"
.parse().expect(
"Cannot handle the socket port"
);
IpAddr
doesn’t handle the port. Use
SocketAddr
instead.
use
std::net::SocketAddr;
// No problem, the `panic!` message has disappeared.
let
_foo: SocketAddr =
"127.0.0.1:8080"
.parse().expect(
"unreachable panic"
);
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
AddrParseError
Source
§
fn
clone
(&self) ->
AddrParseError
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
1.0.0
·
Source
§
impl
Debug
for
AddrParseError
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
1.4.0
·
Source
§
impl
Display
for
AddrParseError
Source
§
fn
fmt
(&self, fmt: &mut
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
1.4.0
·
Source
§
impl
Error
for
AddrParseError
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.30.0
·
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
1.0.0
·
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.0.0
·
Source
§
impl
PartialEq
for
AddrParseError
Source
§
fn
eq
(&self, other: &
AddrParseError
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl
Eq
for
AddrParseError
1.0.0
·
Source
§
impl
StructuralPartialEq
for
AddrParseError
Auto Trait Implementations
§
§
impl
Freeze
for
AddrParseError
§
impl
RefUnwindSafe
for
AddrParseError
§
impl
Send
for
AddrParseError
§
impl
Sync
for
AddrParseError
§
impl
Unpin
for
AddrParseError
§
impl
UnwindSafe
for
AddrParseError
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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