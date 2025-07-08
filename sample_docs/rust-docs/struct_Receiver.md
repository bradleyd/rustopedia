Receiver in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Struct
Receiver
Copy item path
1.0.0
·
Source
pub struct Receiver<T> {
/* private fields */
}
Expand description
The receiving half of Rust’s
channel
(or
sync_channel
) type.
This half can only be owned by one thread.
Messages sent to the channel can be retrieved using
recv
.
§
Examples
use
std::sync::mpsc::channel;
use
std::thread;
use
std::time::Duration;
let
(send, recv) = channel();

thread::spawn(
move
|| {
    send.send(
"Hello world!"
).unwrap();
    thread::sleep(Duration::from_secs(
2
));
// block for two seconds
send.send(
"Delayed for 2 seconds"
).unwrap();
});
println!
(
"{}"
, recv.recv().unwrap());
// Received immediately
println!
(
"Waiting..."
);
println!
(
"{}"
, recv.recv().unwrap());
// Received after 2 seconds
Implementations
§
Source
§
impl<T>
Receiver
<T>
1.0.0
·
Source
pub fn
try_recv
(&self) ->
Result
<T,
TryRecvError
>
Attempts to return a pending value on this receiver without blocking.
This method will never block the caller in order to wait for data to
become available. Instead, this will always return immediately with a
possible option of pending data on the channel.
This is useful for a flavor of “optimistic check” before deciding to
block on a receiver.
Compared with
recv
, this function has two failure cases instead of one
(one for disconnection, one for an empty buffer).
§
Examples
use
std::sync::mpsc::{Receiver, channel};
let
(
_
, receiver): (
_
, Receiver<i32>) = channel();
assert!
(receiver.try_recv().is_err());
1.0.0
·
Source
pub fn
recv
(&self) ->
Result
<T,
RecvError
>
Attempts to wait for a value on this receiver, returning an error if the
corresponding channel has hung up.
This function will always block the current thread if there is no data
available and it’s possible for more data to be sent (at least one sender
still exists). Once a message is sent to the corresponding
Sender
(or
SyncSender
), this receiver will wake up and return that
message.
If the corresponding
Sender
has disconnected, or it disconnects while
this call is blocking, this call will wake up and return
Err
to
indicate that no more messages can ever be received on this channel.
However, since channels are buffered, messages sent before the disconnect
will still be properly received.
§
Examples
use
std::sync::mpsc;
use
std::thread;
let
(send, recv) = mpsc::channel();
let
handle = thread::spawn(
move
|| {
    send.send(
1u8
).unwrap();
});

handle.join().unwrap();
assert_eq!
(
Ok
(
1
), recv.recv());
Buffering behavior:
use
std::sync::mpsc;
use
std::thread;
use
std::sync::mpsc::RecvError;
let
(send, recv) = mpsc::channel();
let
handle = thread::spawn(
move
|| {
    send.send(
1u8
).unwrap();
    send.send(
2
).unwrap();
    send.send(
3
).unwrap();
    drop(send);
});
// wait for the thread to join so we ensure the sender is dropped
handle.join().unwrap();
assert_eq!
(
Ok
(
1
), recv.recv());
assert_eq!
(
Ok
(
2
), recv.recv());
assert_eq!
(
Ok
(
3
), recv.recv());
assert_eq!
(
Err
(RecvError), recv.recv());
1.12.0
·
Source
pub fn
recv_timeout
(&self, timeout:
Duration
) ->
Result
<T,
RecvTimeoutError
>
Attempts to wait for a value on this receiver, returning an error if the
corresponding channel has hung up, or if it waits more than
timeout
.
This function will always block the current thread if there is no data
available and it’s possible for more data to be sent (at least one sender
still exists). Once a message is sent to the corresponding
Sender
(or
SyncSender
), this receiver will wake up and return that
message.
If the corresponding
Sender
has disconnected, or it disconnects while
this call is blocking, this call will wake up and return
Err
to
indicate that no more messages can ever be received on this channel.
However, since channels are buffered, messages sent before the disconnect
will still be properly received.
§
Examples
Successfully receiving value before encountering timeout:
use
std::thread;
use
std::time::Duration;
use
std::sync::mpsc;
let
(send, recv) = mpsc::channel();

thread::spawn(
move
|| {
    send.send(
'a'
).unwrap();
});
assert_eq!
(
    recv.recv_timeout(Duration::from_millis(
400
)),
Ok
(
'a'
)
);
Receiving an error upon reaching timeout:
use
std::thread;
use
std::time::Duration;
use
std::sync::mpsc;
let
(send, recv) = mpsc::channel();

thread::spawn(
move
|| {
    thread::sleep(Duration::from_millis(
800
));
    send.send(
'a'
).unwrap();
});
assert_eq!
(
    recv.recv_timeout(Duration::from_millis(
400
)),
Err
(mpsc::RecvTimeoutError::Timeout)
);
Source
pub fn
recv_deadline
(&self, deadline:
Instant
) ->
Result
<T,
RecvTimeoutError
>
🔬
This is a nightly-only experimental API. (
deadline_api
#46316
)
Attempts to wait for a value on this receiver, returning an error if the
corresponding channel has hung up, or if
deadline
is reached.
This function will always block the current thread if there is no data
available and it’s possible for more data to be sent. Once a message is
sent to the corresponding
Sender
(or
SyncSender
), then this
receiver will wake up and return that message.
If the corresponding
Sender
has disconnected, or it disconnects while
this call is blocking, this call will wake up and return
Err
to
indicate that no more messages can ever be received on this channel.
However, since channels are buffered, messages sent before the disconnect
will still be properly received.
§
Examples
Successfully receiving value before reaching deadline:
#![feature(deadline_api)]
use
std::thread;
use
std::time::{Duration, Instant};
use
std::sync::mpsc;
let
(send, recv) = mpsc::channel();

thread::spawn(
move
|| {
    send.send(
'a'
).unwrap();
});
assert_eq!
(
    recv.recv_deadline(Instant::now() + Duration::from_millis(
400
)),
Ok
(
'a'
)
);
Receiving an error upon reaching deadline:
#![feature(deadline_api)]
use
std::thread;
use
std::time::{Duration, Instant};
use
std::sync::mpsc;
let
(send, recv) = mpsc::channel();

thread::spawn(
move
|| {
    thread::sleep(Duration::from_millis(
800
));
    send.send(
'a'
).unwrap();
});
assert_eq!
(
    recv.recv_deadline(Instant::now() + Duration::from_millis(
400
)),
Err
(mpsc::RecvTimeoutError::Timeout)
);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns an iterator that will block waiting for messages, but never
panic!
. It will return
None
when the channel has hung up.
§
Examples
use
std::sync::mpsc::channel;
use
std::thread;
let
(send, recv) = channel();

thread::spawn(
move
|| {
    send.send(
1
).unwrap();
    send.send(
2
).unwrap();
    send.send(
3
).unwrap();
});
let
mut
iter = recv.iter();
assert_eq!
(iter.next(),
Some
(
1
));
assert_eq!
(iter.next(),
Some
(
2
));
assert_eq!
(iter.next(),
Some
(
3
));
assert_eq!
(iter.next(),
None
);
1.15.0
·
Source
pub fn
try_iter
(&self) ->
TryIter
<'_, T>
ⓘ
Returns an iterator that will attempt to yield all pending values.
It will return
None
if there are no more pending values or if the
channel has hung up. The iterator will never
panic!
or block the
user by waiting for values.
§
Examples
use
std::sync::mpsc::channel;
use
std::thread;
use
std::time::Duration;
let
(sender, receiver) = channel();
// nothing is in the buffer yet
assert!
(receiver.try_iter().next().is_none());

thread::spawn(
move
|| {
    thread::sleep(Duration::from_secs(
1
));
    sender.send(
1
).unwrap();
    sender.send(
2
).unwrap();
    sender.send(
3
).unwrap();
});
// nothing is in the buffer yet
assert!
(receiver.try_iter().next().is_none());
// block for two seconds
thread::sleep(Duration::from_secs(
2
));
let
mut
iter = receiver.try_iter();
assert_eq!
(iter.next(),
Some
(
1
));
assert_eq!
(iter.next(),
Some
(
2
));
assert_eq!
(iter.next(),
Some
(
3
));
assert_eq!
(iter.next(),
None
);
Trait Implementations
§
1.8.0
·
Source
§
impl<T>
Debug
for
Receiver
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
1.1.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a
Receiver
<T>
Source
§
type
Item
= T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
1.1.0
·
Source
§
impl<T>
IntoIterator
for
Receiver
<T>
Source
§
type
Item
= T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IntoIter
<T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
IntoIter
<T>
ⓘ
Creates an iterator from a value.
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
Receiver
<T>
1.0.0
·
Source
§
impl<T> !
Sync
for
Receiver
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Receiver
<T>
§
impl<T>
RefUnwindSafe
for
Receiver
<T>
§
impl<T>
Unpin
for
Receiver
<T>
§
impl<T>
UnwindSafe
for
Receiver
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