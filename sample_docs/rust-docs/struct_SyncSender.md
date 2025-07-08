SyncSender in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Struct
SyncSender
Copy item path
1.0.0
·
Source
pub struct SyncSender<T> {
/* private fields */
}
Expand description
The sending-half of Rust’s synchronous
sync_channel
type.
Messages can be sent through this channel with
send
or
try_send
.
send
will block if there is no space in the internal buffer.
§
Examples
use
std::sync::mpsc::sync_channel;
use
std::thread;
// Create a sync_channel with buffer size 2
let
(sync_sender, receiver) = sync_channel(
2
);
let
sync_sender2 = sync_sender.clone();
// First thread owns sync_sender
thread::spawn(
move
|| {
    sync_sender.send(
1
).unwrap();
    sync_sender.send(
2
).unwrap();
});
// Second thread owns sync_sender2
thread::spawn(
move
|| {
    sync_sender2.send(
3
).unwrap();
// thread will now block since the buffer is full
println!
(
"Thread unblocked!"
);
});
let
mut
msg;

msg = receiver.recv().unwrap();
println!
(
"message {msg} received"
);
// "Thread unblocked!" will be printed now
msg = receiver.recv().unwrap();
println!
(
"message {msg} received"
);

msg = receiver.recv().unwrap();
println!
(
"message {msg} received"
);
Implementations
§
Source
§
impl<T>
SyncSender
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
Sends a value on this synchronous channel.
This function will
block
until space in the internal buffer becomes
available or a receiver is available to hand off the message to.
Note that a successful send does
not
guarantee that the receiver will
ever see the data if there is a buffer on this channel. Items may be
enqueued in the internal buffer for the receiver to receive at a later
time. If the buffer size is 0, however, the channel becomes a rendezvous
channel and it guarantees that the receiver has indeed received
the data if this function returns success.
This function will never panic, but it may return
Err
if the
Receiver
has disconnected and is no longer able to receive
information.
§
Examples
use
std::sync::mpsc::sync_channel;
use
std::thread;
// Create a rendezvous sync_channel with buffer size 0
let
(sync_sender, receiver) = sync_channel(
0
);

thread::spawn(
move
|| {
println!
(
"sending message..."
);
   sync_sender.send(
1
).unwrap();
// Thread is now blocked until the message is received
println!
(
"...message received!"
);
});
let
msg = receiver.recv().unwrap();
assert_eq!
(
1
, msg);
1.0.0
·
Source
pub fn
try_send
(&self, t: T) ->
Result
<
()
,
TrySendError
<T>>
Attempts to send a value on this channel without blocking.
This method differs from
send
by returning immediately if the
channel’s buffer is full or no receiver is waiting to acquire some
data. Compared with
send
, this function has two failure cases
instead of one (one for disconnection, one for a full buffer).
See
send
for notes about guarantees of whether the
receiver has received the data or not if this function is successful.
§
Examples
use
std::sync::mpsc::sync_channel;
use
std::thread;
// Create a sync_channel with buffer size 1
let
(sync_sender, receiver) = sync_channel(
1
);
let
sync_sender2 = sync_sender.clone();
// First thread owns sync_sender
thread::spawn(
move
|| {
    sync_sender.send(
1
).unwrap();
    sync_sender.send(
2
).unwrap();
// Thread blocked
});
// Second thread owns sync_sender2
thread::spawn(
move
|| {
// This will return an error and send
    // no message if the buffer is full
let _
= sync_sender2.try_send(
3
);
});
let
mut
msg;
msg = receiver.recv().unwrap();
println!
(
"message {msg} received"
);

msg = receiver.recv().unwrap();
println!
(
"message {msg} received"
);
// Third message may have never been sent
match
receiver.try_recv() {
Ok
(msg) =>
println!
(
"message {msg} received"
),
Err
(
_
) =>
println!
(
"the third message was never sent"
),
}
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
SyncSender
<T>
Source
§
fn
clone
(&self) ->
SyncSender
<T>
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
1.8.0
·
Source
§
impl<T>
Debug
for
SyncSender
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
SyncSender
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
SyncSender
<T>
§
impl<T>
RefUnwindSafe
for
SyncSender
<T>
§
impl<T>
Sync
for
SyncSender
<T>
where
    T:
Send
,
§
impl<T>
Unpin
for
SyncSender
<T>
§
impl<T>
UnwindSafe
for
SyncSender
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