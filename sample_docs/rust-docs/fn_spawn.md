spawn in std::thread - Rust
std
::
thread
Function
spawn
Copy item path
1.0.0
·
Source
pub fn spawn<F, T>(f: F) ->
JoinHandle
<T>
where
    F:
FnOnce
() -> T +
Send
+ 'static,
    T:
Send
+ 'static,
Expand description
Spawns a new thread, returning a
JoinHandle
for it.
The join handle provides a
join
method that can be used to join the spawned
thread. If the spawned thread panics,
join
will return an
Err
containing
the argument given to
panic!
.
If the join handle is dropped, the spawned thread will implicitly be
detached
.
In this case, the spawned thread may no longer be joined.
(It is the responsibility of the program to either eventually join threads it
creates or detach them; otherwise, a resource leak will result.)
This call will create a thread using default parameters of
Builder
, if you
want to specify the stack size or the name of the thread, use this API
instead.
As you can see in the signature of
spawn
there are two constraints on
both the closure given to
spawn
and its return value, let’s explain them:
The
'static
constraint means that the closure and its return value
must have a lifetime of the whole program execution. The reason for this
is that threads can outlive the lifetime they have been created in.
Indeed if the thread, and by extension its return value, can outlive their
caller, we need to make sure that they will be valid afterwards, and since
we
can’t
know when it will return we need to have them valid as long as
possible, that is until the end of the program, hence the
'static
lifetime.
The
Send
constraint is because the closure will need to be passed
by value
from the thread where it is spawned to the new thread. Its
return value will need to be passed from the new thread to the thread
where it is
join
ed.
As a reminder, the
Send
marker trait expresses that it is safe to be
passed from thread to thread.
Sync
expresses that it is safe to have a
reference be passed from thread to thread.
§
Panics
Panics if the OS fails to create a thread; use
Builder::spawn
to recover from such errors.
§
Examples
Creating a thread.
use
std::thread;
let
handler = thread::spawn(|| {
// thread code
});

handler.join().unwrap();
As mentioned in the module documentation, threads are usually made to
communicate using
channels
, here is how it usually looks.
This example also shows how to use
move
, in order to give ownership
of values to a thread.
use
std::thread;
use
std::sync::mpsc::channel;
let
(tx, rx) = channel();
let
sender = thread::spawn(
move
|| {
    tx.send(
"Hello, thread"
.to_owned())
        .expect(
"Unable to send on channel"
);
});
let
receiver = thread::spawn(
move
|| {
let
value = rx.recv().expect(
"Unable to receive from channel"
);
println!
(
"{value}"
);
});

sender.join().expect(
"The sender thread has panicked"
);
receiver.join().expect(
"The receiver thread has panicked"
);
A thread can also return a value through its
JoinHandle
, you can use
this to make asynchronous computations (futures might be more appropriate
though).
use
std::thread;
let
computation = thread::spawn(|| {
// Some expensive computation.
42
});
let
result = computation.join().unwrap();
println!
(
"{result}"
);
§
Notes
This function has the same minimal guarantee regarding “foreign” unwinding operations (e.g.
an exception thrown from C++ code, or a
panic!
in Rust code compiled or linked with a
different runtime) as
catch_unwind
; namely, if the thread created with
thread::spawn
unwinds all the way to the root with such an exception, one of two behaviors are possible,
and it is unspecified which will occur:
The process aborts.
The process does not abort, and
join
will return a
Result::Err
containing an opaque type.