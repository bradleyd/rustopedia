Builder in std::thread - Rust
std
::
thread
Struct
Builder
Copy item path
1.0.0
·
Source
pub struct Builder {
/* private fields */
}
Expand description
Thread factory, which can be used in order to configure the properties of
a new thread.
Methods can be chained on it in order to configure it.
The two configurations available are:
name
: specifies an
associated name for the thread
stack_size
: specifies the
desired stack size for the thread
The
spawn
method will take ownership of the builder and create an
io::Result
to the thread handle with the given configuration.
The
thread::spawn
free function uses a
Builder
with default
configuration and
unwrap
s its return value.
You may want to use
spawn
instead of
thread::spawn
, when you want
to recover from a failure to launch a thread, indeed the free function will
panic where the
Builder
method will return a
io::Result
.
§
Examples
use
std::thread;
let
builder = thread::Builder::new();
let
handler = builder.spawn(|| {
// thread code
}).unwrap();

handler.join().unwrap();
Implementations
§
Source
§
impl
Builder
1.63.0
·
Source
pub fn
spawn_scoped
<'scope, 'env, F, T>(
    self,
    scope: &'scope
Scope
<'scope, 'env>,
    f: F,
) ->
Result
<
ScopedJoinHandle
<'scope, T>>
where
    F:
FnOnce
() -> T +
Send
+ 'scope,
    T:
Send
+ 'scope,
Spawns a new scoped thread using the settings set through this
Builder
.
Unlike
Scope::spawn
, this method yields an
io::Result
to
capture any failure to create the thread at the OS level.
§
Panics
Panics if a thread name was set and it contained null bytes.
§
Example
use
std::thread;
let
mut
a =
vec!
[
1
,
2
,
3
];
let
mut
x =
0
;

thread::scope(|s| {
    thread::Builder::new()
        .name(
"first"
.to_string())
        .spawn_scoped(s, ||
    {
println!
(
"hello from the {:?} scoped thread"
, thread::current().name());
// We can borrow `a` here.
dbg!
(
&
a);
    })
    .unwrap();
    thread::Builder::new()
        .name(
"second"
.to_string())
        .spawn_scoped(s, ||
    {
println!
(
"hello from the {:?} scoped thread"
, thread::current().name());
// We can even mutably borrow `x` here,
        // because no other threads are using it.
x += a[
0
] + a[
2
];
    })
    .unwrap();
println!
(
"hello from the main thread"
);
});
// After the scope, we can modify and access our variables again:
a.push(
4
);
assert_eq!
(x, a.len());
Source
§
impl
Builder
1.0.0
·
Source
pub fn
new
() ->
Builder
Generates the base configuration for spawning a thread, from which
configuration methods can be chained.
§
Examples
use
std::thread;
let
builder = thread::Builder::new()
                              .name(
"foo"
.into())
                              .stack_size(
32
*
1024
);
let
handler = builder.spawn(|| {
// thread code
}).unwrap();

handler.join().unwrap();
1.0.0
·
Source
pub fn
name
(self, name:
String
) ->
Builder
Names the thread-to-be. Currently the name is used for identification
only in panic messages.
The name must not contain null bytes (
\0
).
For more information about named threads, see
this module-level documentation
.
§
Examples
use
std::thread;
let
builder = thread::Builder::new()
    .name(
"foo"
.into());
let
handler = builder.spawn(|| {
assert_eq!
(thread::current().name(),
Some
(
"foo"
))
}).unwrap();

handler.join().unwrap();
1.0.0
·
Source
pub fn
stack_size
(self, size:
usize
) ->
Builder
Sets the size of the stack (in bytes) for the new thread.
The actual stack size may be greater than this value if
the platform specifies a minimal stack size.
For more information about the stack size for threads, see
this module-level documentation
.
§
Examples
use
std::thread;
let
builder = thread::Builder::new().stack_size(
32
*
1024
);
Source
pub fn
no_hooks
(self) ->
Builder
🔬
This is a nightly-only experimental API. (
thread_spawn_hook
#132951
)
Disables running and inheriting
spawn hooks
.
Use this if the parent thread is in no way relevant for the child thread.
For example, when lazily spawning threads for a thread pool.
1.0.0
·
Source
pub fn
spawn
<F, T>(self, f: F) ->
Result
<
JoinHandle
<T>>
where
    F:
FnOnce
() -> T +
Send
+ 'static,
    T:
Send
+ 'static,
Spawns a new thread by taking ownership of the
Builder
, and returns an
io::Result
to its
JoinHandle
.
The spawned thread may outlive the caller (unless the caller thread
is the main thread; the whole process is terminated when the main
thread finishes). The join handle can be used to block on
termination of the spawned thread, including recovering its panics.
For a more complete documentation see
thread::spawn
.
§
Errors
Unlike the
spawn
free function, this method yields an
io::Result
to capture any failure to create the thread at
the OS level.
§
Panics
Panics if a thread name was set and it contained null bytes.
§
Examples
use
std::thread;
let
builder = thread::Builder::new();
let
handler = builder.spawn(|| {
// thread code
}).unwrap();

handler.join().unwrap();
1.82.0
·
Source
pub unsafe fn
spawn_unchecked
<F, T>(self, f: F) ->
Result
<
JoinHandle
<T>>
where
    F:
FnOnce
() -> T +
Send
,
    T:
Send
,
Spawns a new thread without any lifetime restrictions by taking ownership
of the
Builder
, and returns an
io::Result
to its
JoinHandle
.
The spawned thread may outlive the caller (unless the caller thread
is the main thread; the whole process is terminated when the main
thread finishes). The join handle can be used to block on
termination of the spawned thread, including recovering its panics.
This method is identical to
thread::Builder::spawn
,
except for the relaxed lifetime bounds, which render it unsafe.
For a more complete documentation see
thread::spawn
.
§
Errors
Unlike the
spawn
free function, this method yields an
io::Result
to capture any failure to create the thread at
the OS level.
§
Panics
Panics if a thread name was set and it contained null bytes.
§
Safety
The caller has to ensure that the spawned thread does not outlive any
references in the supplied thread closure and its return type.
This can be guaranteed in two ways:
ensure that
join
is called before any referenced
data is dropped
use only types with
'static
lifetime bounds, i.e., those with no or only
'static
references (both
thread::Builder::spawn
and
thread::spawn
enforce this property statically)
§
Examples
use
std::thread;
let
builder = thread::Builder::new();
let
x =
1
;
let
thread_x =
&
x;
let
handler =
unsafe
{
    builder.spawn_unchecked(
move
|| {
println!
(
"x = {}"
,
*
thread_x);
    }).unwrap()
};
// caller has to ensure `join()` is called, otherwise
// it is possible to access freed memory if `x` gets
// dropped before the thread closure is executed!
handler.join().unwrap();
Trait Implementations
§
1.0.0
·
Source
§
impl
Debug
for
Builder
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
Auto Trait Implementations
§
§
impl
Freeze
for
Builder
§
impl
RefUnwindSafe
for
Builder
§
impl
Send
for
Builder
§
impl
Sync
for
Builder
§
impl
Unpin
for
Builder
§
impl
UnwindSafe
for
Builder
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