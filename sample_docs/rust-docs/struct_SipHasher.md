SipHasher in std::hash - Rust
std
::
hash
Struct
SipHasher
Copy item path
1.0.0
·
Source
pub struct SipHasher(
/* private fields */
);
👎
Deprecated since 1.13.0: use
std::hash::DefaultHasher
instead
Expand description
An implementation of SipHash 2-4.
See:
https://131002.net/siphash/
SipHash is a general-purpose hashing function: it runs at a good
speed (competitive with Spooky and City) and permits strong
keyed
hashing. This lets you key your hash tables from a strong RNG, such as
rand::os::OsRng
.
Although the SipHash algorithm is considered to be generally strong,
it is not intended for cryptographic purposes. As such, all
cryptographic uses of this implementation are
strongly discouraged
.
Implementations
§
Source
§
impl
SipHasher
1.0.0
·
Source
pub fn
new
() ->
SipHasher
👎
Deprecated since 1.13.0: use
std::hash::DefaultHasher
instead
Creates a new
SipHasher
with the two initial keys set to 0.
1.0.0
·
Source
pub fn
new_with_keys
(key0:
u64
, key1:
u64
) ->
SipHasher
👎
Deprecated since 1.13.0: use
std::hash::DefaultHasher
instead
Creates a
SipHasher
that is keyed off the provided keys.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
SipHasher
Source
§
fn
clone
(&self) ->
SipHasher
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
SipHasher
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
Default
for
SipHasher
Source
§
fn
default
() ->
SipHasher
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl
Hasher
for
SipHasher
Source
§
fn
write
(&mut self, msg: &[
u8
])
Writes some data into this
Hasher
.
Read more
Source
§
fn
write_str
(&mut self, s: &
str
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a single
str
into this hasher.
Read more
Source
§
fn
finish
(&self) ->
u64
Returns the hash value for the values written so far.
Read more
1.3.0
·
Source
§
fn
write_u8
(&mut self, i:
u8
)
Writes a single
u8
into this hasher.
1.3.0
·
Source
§
fn
write_u16
(&mut self, i:
u16
)
Writes a single
u16
into this hasher.
1.3.0
·
Source
§
fn
write_u32
(&mut self, i:
u32
)
Writes a single
u32
into this hasher.
1.3.0
·
Source
§
fn
write_u64
(&mut self, i:
u64
)
Writes a single
u64
into this hasher.
1.26.0
·
Source
§
fn
write_u128
(&mut self, i:
u128
)
Writes a single
u128
into this hasher.
1.3.0
·
Source
§
fn
write_usize
(&mut self, i:
usize
)
Writes a single
usize
into this hasher.
1.3.0
·
Source
§
fn
write_i8
(&mut self, i:
i8
)
Writes a single
i8
into this hasher.
1.3.0
·
Source
§
fn
write_i16
(&mut self, i:
i16
)
Writes a single
i16
into this hasher.
1.3.0
·
Source
§
fn
write_i32
(&mut self, i:
i32
)
Writes a single
i32
into this hasher.
1.3.0
·
Source
§
fn
write_i64
(&mut self, i:
i64
)
Writes a single
i64
into this hasher.
1.26.0
·
Source
§
fn
write_i128
(&mut self, i:
i128
)
Writes a single
i128
into this hasher.
1.3.0
·
Source
§
fn
write_isize
(&mut self, i:
isize
)
Writes a single
isize
into this hasher.
Source
§
fn
write_length_prefix
(&mut self, len:
usize
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a length prefix into this hasher, as part of being prefix-free.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
SipHasher
§
impl
RefUnwindSafe
for
SipHasher
§
impl
Send
for
SipHasher
§
impl
Sync
for
SipHasher
§
impl
Unpin
for
SipHasher
§
impl
UnwindSafe
for
SipHasher
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