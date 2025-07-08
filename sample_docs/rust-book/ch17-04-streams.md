Streams: Futures in Sequence - The Rust Programming Language
Light
Rust
Coal
Navy
Ayu
The Rust Programming Language
Streams: Futures in Sequence
So far in this chapter, we’ve mostly stuck to individual futures. The one big
exception was the async channel we used. Recall how we used the receiver for our
async channel earlier in this chapter in the
“Message
Passing”
section. The async
recv
method
produces a sequence of items over time. This is an instance of a much more
general pattern known as a
stream
.
We saw a sequence of items back in Chapter 13, when we looked at the
Iterator
trait in
The Iterator Trait and the
next
Method
section, but there are two differences between iterators and the async
channel receiver. The first difference is time: iterators are synchronous, while
the channel receiver is asynchronous. The second is the API. When working
directly with
Iterator
, we call its synchronous
next
method. With the
trpl::Receiver
stream in particular, we called an asynchronous
recv
method
instead. Otherwise, these APIs feel very similar, and that similarity
isn’t a coincidence. A stream is like an asynchronous form of iteration. Whereas
the
trpl::Receiver
specifically waits to receive messages, though, the
general-purpose stream API is much broader: it provides the next item the
way
Iterator
does, but asynchronously.
The similarity between iterators and streams in Rust means we can actually
create a stream from any iterator. As with an iterator, we can work with a
stream by calling its
next
method and then awaiting the output, as in Listing
17-30.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
fn main() {
trpl::run(async {
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
});
}
Listing 17-30
: Creating a stream from an iterator and printing its values
We start with an array of numbers, which we convert to an iterator and then call
map
on to double all the values. Then we convert the iterator into a stream
using the
trpl::stream_from_iter
function. Next, we loop over the items in the
stream as they arrive with the
while let
loop.
Unfortunately, when we try to run the code, it doesn’t compile, but instead it
reports that there’s no
next
method available:
error[E0599]: no method named `next` found for struct `Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = note: the full type name has been written to 'file:///projects/async-await/target/debug/deps/async_await-575db3dd3197d257.long-type-14490787947592691573.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
As this output explains, the reason for the compiler error is that we need the
right trait in scope to be able to use the
next
method. Given our discussion
so far, you might reasonably expect that trait to be
Stream
, but it’s actually
StreamExt
. Short for
extension
,
Ext
is a common pattern in the
Rust community for extending one trait with another.
We’ll explain the
Stream
and
StreamExt
traits in a bit more detail at the
end of the chapter, but for now all you need to know is that the
Stream
trait
defines a low-level interface that effectively combines the
Iterator
and
Future
traits.
StreamExt
supplies a higher-level set of APIs on top of
Stream
, including the
next
method as well as other utility methods similar
to those provided by the
Iterator
trait.
Stream
and
StreamExt
are not yet
part of Rust’s standard library, but most ecosystem crates use the same
definition.
The fix to the compiler error is to add a
use
statement for
trpl::StreamExt
,
as in Listing 17-31.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
Listing 17-31
: Successfully using an iterator as the basis for a stream
With all those pieces put together, this code works the way we want! What’s
more, now that we have
StreamExt
in scope, we can use all of its utility
methods, just as with iterators. For example, in Listing 17-32, we use the
filter
method to filter out everything but multiples of three and five.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
Listing 17-32
: Filtering a stream with the
StreamExt::filter
method
Of course, this isn’t very interesting, since we could do the same with normal
iterators and without any async at all. Let’s look at what
we can do that
is
unique to streams.
Composing Streams
Many concepts are naturally represented as streams: items becoming available in
a queue, chunks of data being pulled incrementally from the filesystem when the
full data set is too large for the computer’s memory, or data arriving over the
network over time. Because streams are futures, we can use them with any other
kind of future and combine them in interesting ways. For example, we can batch
up events to avoid triggering too many network calls, set timeouts on sequences
of long-running operations, or throttle user interface events to avoid doing
needless work.
Let’s start by building a little stream of messages as a stand-in for a stream
of data we might see from a WebSocket or another real-time communication
protocol, as shown in Listing 17-33.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
Listing 17-33
: Using the
rx
receiver as a
ReceiverStream
First, we create a function called
get_messages
that returns
impl Stream<Item = String>
. For its implementation, we create an async channel, loop over the
first 10 letters of the English alphabet, and send them across the channel.
We also use a new type:
ReceiverStream
, which converts the
rx
receiver from
the
trpl::channel
into a
Stream
with a
next
method. Back in
main
, we use
a
while let
loop to print all the messages from the stream.
When we run this code, we get exactly the results we would expect:
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
Again, we could do this with the regular
Receiver
API or even the regular
Iterator
API, though, so let’s add a feature that requires streams: adding a
timeout that applies to every item in the stream, and a delay on the items we
emit, as shown in Listing 17-34.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
fn get_messages() -> impl Stream<Item = String> {
let (tx, rx) = trpl::channel();
let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
for message in messages {
tx.send(format!("Message: '{message}'")).unwrap();
}
ReceiverStream::new(rx)
}
Listing 17-34
: Using the
StreamExt::timeout
method to set a time limit on the items in a stream
We start by adding a timeout to the stream with the
timeout
method, which
comes from the
StreamExt
trait. Then we update the body of the
while let
loop, because the stream now returns a
Result
. The
Ok
variant indicates a
message arrived in time; the
Err
variant indicates that the timeout elapsed
before any message arrived. We
match
on that result and either print the
message when we receive it successfully or print a notice about the timeout.
Finally, notice that we pin the messages after applying the timeout to them,
because the timeout helper produces a stream that needs to be pinned to be
polled.
However, because there are no delays between messages, this timeout does not
change the behavior of the program. Let’s add a variable delay to the messages
we send, as shown in Listing 17-35.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let mut messages =
pin!(get_messages().timeout(Duration::from_millis(200)));
while let Some(result) = messages.next().await {
match result {
Ok(message) => println!("{message}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
})
}
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
Listing 17-35
: Sending messages through
tx
with an async delay without making
get_messages
an async function
In
get_messages
, we use the
enumerate
iterator method with the
messages
array so that we can get the index of each item we’re sending along with the
item itself. Then we apply a 100-millisecond delay to even-index items and a
300-millisecond delay to odd-index items to simulate the different delays we
might see from a stream of messages in the real world. Because our timeout is
for 200 milliseconds, this should affect half of the messages.
To sleep between messages in the
get_messages
function without blocking, we
need to use async. However, we can’t make
get_messages
itself into an async
function, because then we’d return a
Future<Output = Stream<Item = String>>
instead of a
Stream<Item = String>>
. The caller would have to await
get_messages
itself to get access to the stream. But remember: everything in a
given future happens linearly; concurrency happens
between
futures. Awaiting
get_messages
would require it to send all the messages, including the sleep
delay between each message, before returning the receiver stream. As a result,
the timeout would be useless. There would be no delays in the stream itself;
they would all happen before the stream was even available.
Instead, we leave
get_messages
as a regular function that returns a stream,
and we spawn a task to handle the async
sleep
calls.
Note: Calling
spawn_task
in this way works because we already set up our
runtime; had we not, it would cause a panic. Other implementations choose
different tradeoffs: they might spawn a new runtime and avoid the panic but
end up with a bit of extra overhead, or they may simply not provide a
standalone way to spawn tasks without reference to a runtime. Make sure you
know what tradeoff your runtime has chosen and write your code accordingly!
Now our code has a much more interesting result. Between every other pair of
messages, a
Problem: Elapsed(())
error.
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
The timeout doesn’t prevent the messages from arriving in the end. We still get
all of the original messages, because our channel is
unbounded
: it can hold as
many messages as we can fit in memory. If the message doesn’t arrive before the
timeout, our stream handler will account for that, but when it polls the stream
again, the message may now have arrived.
You can get different behavior if needed by using other kinds of channels or
other kinds of streams more generally. Let’s see one of those in practice by
combining a stream of time intervals with this stream of messages.
Merging Streams
First, let’s create another stream, which will emit an item every millisecond if
we let it run directly. For simplicity, we can use the
sleep
function to send
a message on a delay and combine it with the same approach we used in
get_messages
of creating a stream from a channel. The difference is that this
time, we’re going to send back the count of intervals that have elapsed, so the
return type will be
impl Stream<Item = u32>
, and we can call the function
get_intervals
(see Listing 17-36).
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let mut messages =
pin!(get_messages().timeout(Duration::from_millis(200)));
while let Some(result) = messages.next().await {
match result {
Ok(message) => println!("{message}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
})
}
fn get_messages() -> impl Stream<Item = String> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
for (index, message) in messages.into_iter().enumerate() {
let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
trpl::sleep(Duration::from_millis(time_to_sleep)).await;
tx.send(format!("Message: '{message}'")).unwrap();
}
});
ReceiverStream::new(rx)
}
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
Listing 17-36
: Creating a stream with a counter that will be emitted once every millisecond
We start by defining a
count
in the task. (We could define it outside the
task, too, but it’s clearer to limit the scope of any given variable.) Then we
create an infinite loop. Each iteration of the loop asynchronously sleeps for
one millisecond, increments the count, and then sends it over the channel.
Because this is all wrapped in the task created by
spawn_task
, all of
it—including the infinite loop—will get cleaned up along with the runtime.
This kind of infinite loop, which ends only when the whole runtime gets torn
down, is fairly common in async Rust: many programs need to keep running
indefinitely. With async, this doesn’t block anything else, as long as there is
at least one await point in each iteration through the loop.
Now, back in our main function’s async block, we can attempt to merge the
messages
and
intervals
streams, as shown in Listing 17-37.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);
while let Some(result) = merged.next().await {
match result {
Ok(message) => println!("{message}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
})
}
fn get_messages() -> impl Stream<Item = String> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
for (index, message) in messages.into_iter().enumerate() {
let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
trpl::sleep(Duration::from_millis(time_to_sleep)).await;
tx.send(format!("Message: '{message}'")).unwrap();
}
});
ReceiverStream::new(rx)
}
fn get_intervals() -> impl Stream<Item = u32> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let mut count = 0;
loop {
trpl::sleep(Duration::from_millis(1)).await;
count += 1;
tx.send(count).unwrap();
}
});
ReceiverStream::new(rx)
}
Listing 17-37
: Attempting to merge the
messages
and
intervals
streams
We start by calling
get_intervals
. Then we merge the
messages
and
intervals
streams with the
merge
method, which combines multiple streams
into one stream that produces items from any of the source streams as soon as
the items are available, without imposing any particular ordering. Finally, we
loop over that combined stream instead of over
messages
.
At this point, neither
messages
nor
intervals
needs to be pinned or mutable,
because both will be combined into the single
merged
stream. However, this
call to
merge
doesn’t compile! (Neither does the
next
call in the
while let
loop, but we’ll come back to that.) This is because the two streams have
different types. The
messages
stream has the type
Timeout<impl Stream<Item = String>>
, where
Timeout
is the type that implements
Stream
for a
timeout
call. The
intervals
stream has the type
impl Stream<Item = u32>
. To merge
these two streams, we need to transform one of them to match the other. We’ll
rework the intervals stream, because messages is already in the basic format we
want and has to handle timeout errors (see Listing 17-38).
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);
while let Some(result) = stream.next().await {
match result {
Ok(message) => println!("{message}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
})
}
fn get_messages() -> impl Stream<Item = String> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
for (index, message) in messages.into_iter().enumerate() {
let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
trpl::sleep(Duration::from_millis(time_to_sleep)).await;
tx.send(format!("Message: '{message}'")).unwrap();
}
});
ReceiverStream::new(rx)
}
fn get_intervals() -> impl Stream<Item = u32> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let mut count = 0;
loop {
trpl::sleep(Duration::from_millis(1)).await;
count += 1;
tx.send(count).unwrap();
}
});
ReceiverStream::new(rx)
}
Listing 17-38
: Aligning the type of the the
intervals
stream with the type of the
messages
stream
First, we can use the
map
helper method to transform the
intervals
into a
string. Second, we need to match the
Timeout
from
messages
. Because we don’t
actually
want
a timeout for
intervals
, though, we can just create a timeout
which is longer than the other durations we are using. Here, we create a
10-second timeout with
Duration::from_secs(10)
. Finally, we need to make
stream
mutable, so that the
while let
loop’s
next
calls can iterate
through the stream, and pin it so that it’s safe to do so. That gets us
almost
to where we need to be. Everything type checks. If you run this, though, there
will be two problems. First, it will never stop! You’ll need to stop it with
ctrl-c
. Second, the messages from the English
alphabet will be buried in the midst of all the interval counter messages:
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
Listing 17-39 shows one way to solve these last two problems.
Filename: src/main.rs
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
while let Some(result) = stream.next().await {
match result {
Ok(message) => println!("{message}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
})
}
fn get_messages() -> impl Stream<Item = String> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
for (index, message) in messages.into_iter().enumerate() {
let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
trpl::sleep(Duration::from_millis(time_to_sleep)).await;
tx.send(format!("Message: '{message}'")).unwrap();
}
});
ReceiverStream::new(rx)
}
fn get_intervals() -> impl Stream<Item = u32> {
let (tx, rx) = trpl::channel();
trpl::spawn_task(async move {
let mut count = 0;
loop {
trpl::sleep(Duration::from_millis(1)).await;
count += 1;
tx.send(count).unwrap();
}
});
ReceiverStream::new(rx)
}
Listing 17-39
: Using
throttle
and
take
to manage the merged streams
First, we use the
throttle
method on the
intervals
stream so that it doesn’t
overwhelm the
messages
stream.
Throttling
is a way of limiting the rate at
which a function will be called—or, in this case, how often the stream will be
polled. Once every 100 milliseconds should do, because that’s roughly how often
our messages arrive.
To limit the number of items we will accept from a stream, we apply the
take
method to the
merged
stream, because we want to limit the final output, not
just one stream or the other.
Now when we run the program, it stops after pulling 20 items from the stream,
and the intervals don’t overwhelm the messages. We also don’t get
Interval: 100
or
Interval: 200
or so on, but instead get
Interval: 1
,
Interval: 2
,
and so on—even though we have a source stream that
can
produce an event every
millisecond. That’s because the
throttle
call produces a new stream that wraps
the original stream so that the original stream gets polled only at the throttle
rate, not its own “native” rate. We don’t have a bunch of unhandled interval
messages we’re choosing to ignore. Instead, we never produce those interval
messages in the first place! This is the inherent “laziness” of Rust’s futures
at work again, allowing us to choose our performance characteristics.
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
There’s one last thing we need to handle: errors! With both of these
channel-based streams, the
send
calls could fail when the other side of the
channel closes—and that’s just a matter of how the runtime executes the futures
that make up the stream. Up until now, we’ve ignored this possibility by calling
unwrap
, but in a well-behaved app, we should explicitly handle the error, at
minimum by ending the loop so we don’t try to send any more messages. Listing
17-40 shows a simple error strategy: print the issue and then
break
from the
loops.
extern crate trpl; // required for mdbook test
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};
fn main() {
trpl::run(async {
let messages = get_messages().timeout(Duration::from_millis(200));
let intervals = get_intervals()
.map(|count| format!("Interval #{count}"))
.throttle(Duration::from_millis(500))
.timeout(Duration::from_secs(10));
let merged = messages.merge(intervals).take(20);
let mut stream = pin!(merged);
while let Some(result) = stream.next().await {
match result {
Ok(item) => println!("{item}"),
Err(reason) => eprintln!("Problem: {reason:?}"),
}
}
});
}
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
Listing 17-40
: Handling errors and shutting down the loops
As usual, the correct way to handle a message send error will vary; just make
sure you have a strategy.
Now that we’ve seen a bunch of async in practice, let’s take a step back and dig
into a few of the details of how
Future
,
Stream
, and the other key traits
Rust uses to make async work.