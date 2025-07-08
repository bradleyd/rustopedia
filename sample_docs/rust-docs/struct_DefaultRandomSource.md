DefaultRandomSource in std::random - Rust
std
::
random
Struct
DefaultRandomSource
Copy item path
Source
pub struct DefaultRandomSource;
🔬
This is a nightly-only experimental API. (
random
#130703
)
Expand description
The default random source.
This asks the system for random data suitable for cryptographic purposes
such as key generation. If security is a concern, consult the platform
documentation below for the specific guarantees your target provides.
The high quality of randomness provided by this source means it can be quite
slow on some targets. If you need a large quantity of random numbers and
security is not a concern,  consider using an alternative random number
generator (potentially seeded from this one).
§
Underlying sources
Platform
Source
Linux
getrandom
or
/dev/urandom
after polling
/dev/random
Windows
ProcessPrng
Apple
CCRandomGenerateBytes
DragonFly
arc4random_buf
ESP-IDF
esp_fill_random
FreeBSD
arc4random_buf
Fuchsia
cprng_draw
Haiku
arc4random_buf
Illumos
arc4random_buf
NetBSD
arc4random_buf
OpenBSD
arc4random_buf
Solaris
arc4random_buf
Vita
arc4random_buf
Hermit
read_entropy
Horizon, Cygwin
getrandom
AIX, Hurd, L4Re, QNX
/dev/urandom
Redox
/scheme/rand
RTEMS
arc4random_buf
SGX
rdrand
SOLID
SOLID_RNG_SampleRandomBytes
TEEOS
TEE_GenerateRandom
UEFI
EFI_RNG_PROTOCOL
VxWorks
randABytes
after waiting for
randSecure
to become ready
WASI
random_get
ZKVM
sys_rand
Note that the sources used might change over time.
Consult the documentation for the underlying operations on your supported
targets to determine whether they provide any particular desired properties,
such as support for reseeding on VM fork operations.
Trait Implementations
§
Source
§
impl
Clone
for
DefaultRandomSource
Source
§
fn
clone
(&self) ->
DefaultRandomSource
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
Source
§
impl
Debug
for
DefaultRandomSource
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
Source
§
impl
Default
for
DefaultRandomSource
Source
§
fn
default
() ->
DefaultRandomSource
Returns the “default value” for a type.
Read more
Source
§
impl
RandomSource
for
DefaultRandomSource
Source
§
fn
fill_bytes
(&mut self, bytes: &mut [
u8
])
🔬
This is a nightly-only experimental API. (
random
#130703
)
Fills
bytes
with random bytes.
Source
§
impl
Copy
for
DefaultRandomSource
Auto Trait Implementations
§
§
impl
Freeze
for
DefaultRandomSource
§
impl
RefUnwindSafe
for
DefaultRandomSource
§
impl
Send
for
DefaultRandomSource
§
impl
Sync
for
DefaultRandomSource
§
impl
Unpin
for
DefaultRandomSource
§
impl
UnwindSafe
for
DefaultRandomSource
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