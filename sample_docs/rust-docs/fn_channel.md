channel in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Function
channel
Copy item path
1.0.0
·
Source
pub fn channel<T>() -> (
Sender
<T>,
Receiver
<T>)
Expand description
Creates a new asynchronous channel, returning the sender/receiver halves.
All data sent on the
Sender
will become available on the
Receiver
in
the same order as it was sent, and no
send
will block the calling thread
(this channel has an “infinite buffer”, unlike
sync_channel
, which will
block after its buffer limit is reached).
recv
will block until a message
is available while there is at least one
Sender
alive (including clones).
The
Sender
can be cloned to
send
to the same channel multiple times, but
only one
Receiver
is supported.
If the
Receiver
is disconnected while trying to
send
with the
Sender
, the
send
method will return a
SendError
. Similarly, if the
Sender
is disconnected while trying to
recv
, the
recv
method will
return a
RecvError
.
§
Examples
use
std::sync::mpsc::channel;
use
std::thread;
let
(sender, receiver) = channel();
// Spawn off an expensive computation
thread::spawn(
move
|| {
    sender.send(expensive_computation()).unwrap();
});
// Do some useful work for awhile

// Let's see what that answer was
println!
(
"{:?}"
, receiver.recv().unwrap());