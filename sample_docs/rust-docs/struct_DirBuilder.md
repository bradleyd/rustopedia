DirBuilder in std::fs - Rust
std
::
fs
Struct
DirBuilder
Copy item path
1.6.0
·
Source
pub struct DirBuilder {
/* private fields */
}
Expand description
A builder used to create directories in various manners.
This builder also supports platform-specific options.
Implementations
§
Source
§
impl
DirBuilder
1.6.0
·
Source
pub fn
new
() ->
DirBuilder
Creates a new set of options with default mode/security settings for all
platforms and also non-recursive.
§
Examples
use
std::fs::DirBuilder;
let
builder = DirBuilder::new();
1.6.0
·
Source
pub fn
recursive
(&mut self, recursive:
bool
) -> &mut Self
Indicates that directories should be created recursively, creating all
parent directories. Parents that do not exist are created with the same
security and permissions settings.
This option defaults to
false
.
§
Examples
use
std::fs::DirBuilder;
let
mut
builder = DirBuilder::new();
builder.recursive(
true
);
1.6.0
·
Source
pub fn
create
<P:
AsRef
<
Path
>>(&self, path: P) ->
Result
<
()
>
Creates the specified directory with the options configured in this
builder.
It is considered an error if the directory already exists unless
recursive mode is enabled.
§
Examples
use
std::fs::{
self
, DirBuilder};
let
path =
"/tmp/foo/bar/baz"
;
DirBuilder::new()
    .recursive(
true
)
    .create(path).unwrap();
assert!
(fs::metadata(path).unwrap().is_dir());
Trait Implementations
§
1.6.0
·
Source
§
impl
Debug
for
DirBuilder
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
1.6.0
·
Source
§
impl
DirBuilderExt
for
DirBuilder
Available on
Unix
only.
Source
§
fn
mode
(&mut self, mode:
u32
) -> &mut
DirBuilder
Sets the mode to create new directories with. This option defaults to
0o777.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
DirBuilder
§
impl
RefUnwindSafe
for
DirBuilder
§
impl
Send
for
DirBuilder
§
impl
Sync
for
DirBuilder
§
impl
Unpin
for
DirBuilder
§
impl
UnwindSafe
for
DirBuilder
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