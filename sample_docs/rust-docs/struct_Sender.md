Sender in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Struct
Sender
Copy item path
1.0.0
·
Source
pub struct Sender<T> {
/* private fields */
}
Expand description
The sending-half of Rust’s asynchronous
channel
type.
Messages can be sent through this channel with
send
.
Note: all senders (the original and its clones) need to be dropped for the receiver
to stop blocking to receive messages with
Receiver::recv
.
§
Examples
use
std::sync::mpsc::channel;
use
std::thread;
let
(sender, receiver) = channel();
let
sender2 = sender.clone();
// First thread owns sender
thread::spawn(
move
|| {
    sender.send(
1
).unwrap();
});
// Second thread owns sender2
thread::spawn(
move
|| {
    sender2.send(
2
).unwrap();
});
let
msg = receiver.recv().unwrap();
let
msg2 = receiver.recv().unwrap();
assert_eq!
(
3
, msg + msg2);
Implementations
§
Source
§
impl<T>
Sender
<T>
1.0.0
·
Source
pub fn
send
(&self, t: T) ->
Result
<
()
,
SendError
<T>>
Attempts to send a value on this channel, returning it back if it could
not be sent.
A successful send occurs when it is determined that the other end of
the channel has not hung up already. An unsuccessful send would be one
where the corresponding receiver has already been deallocated. Note
that a return value of
Err
means that the data will never be
received, but a return value of
Ok
does
not
mean that the data
will be received. It is possible for the corresponding receiver to
hang up immediately after this function returns
Ok
.
This method will never block the current thread.
§
Examples
use
std::sync::mpsc::channel;
let
(tx, rx) = channel();
// This send is always successful
tx.send(
1
).unwrap();
// This send will fail because the receiver is gone
drop(rx);
assert_eq!
(tx.send(
1
).unwrap_err().
0
,
1
);
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
Sender
<T>
Source
§
fn
clone
(&self) ->
Sender
<T>
Clone a sender to send to other threads.
Note, be aware of the lifetime of the sender because all senders
(including the original) need to be dropped in order for
Receiver::recv
to stop blocking.
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
1.8.0
·
Source
§
impl<T>
Debug
for
Sender
<T>
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
1.0.0
·
Source
§
impl<T:
Send
>
Send
for
Sender
<T>
1.72.0
·
Source
§
impl<T:
Send
>
Sync
for
Sender
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Sender
<T>
§
impl<T>
RefUnwindSafe
for
Sender
<T>
§
impl<T>
Unpin
for
Sender
<T>
§
impl<T>
UnwindSafe
for
Sender
<T>
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