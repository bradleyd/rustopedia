sleep_until in std::thread - Rust
std
::
thread
Function
sleep_until
Copy item path
Source
pub fn sleep_until(deadline:
Instant
)
🔬
This is a nightly-only experimental API. (
thread_sleep_until
#113752
)
Expand description
Puts the current thread to sleep until the specified deadline has passed.
The thread may still be asleep after the deadline specified due to
scheduling specifics or platform-dependent functionality. It will never
wake before.
This function is blocking, and should not be used in
async
functions.
§
Platform-specific behavior
This function uses
sleep
internally, see its platform-specific behavior.
§
Examples
A simple game loop that limits the game to 60 frames per second.
#![feature(thread_sleep_until)]
let
max_fps =
60.0
;
let
frame_time = Duration::from_secs_f32(
1.0
/max_fps);
let
mut
next_frame = Instant::now();
loop
{
    thread::sleep_until(next_frame);
    next_frame += frame_time;
    update();
    render();
}
A slow api we must not call too fast and which takes a few
tries before succeeding. By using
sleep_until
the time the
api call takes does not influence when we retry or when we give up
#![feature(thread_sleep_until)]
let
deadline = Instant::now() + MAX_DURATION;
let
delay = Duration::from_millis(
250
);
let
mut
next_attempt = Instant::now();
loop
{
if
Instant::now() > deadline {
break
Err
(());
    }
if let
Status::Ready(data) = slow_web_api_call() {
break
Ok
(data);
    }

    next_attempt = deadline.min(next_attempt + delay);
    thread::sleep_until(next_attempt);
}