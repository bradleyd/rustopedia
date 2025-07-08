panicking in std::thread - Rust
std
::
thread
Function
panicking
Copy item path
1.0.0
·
Source
pub fn panicking() ->
bool
Expand description
Determines whether the current thread is unwinding because of panic.
A common use of this feature is to poison shared resources when writing
unsafe code, by checking
panicking
when the
drop
is called.
This is usually not needed when writing safe code, as
Mutex
es
already poison themselves when a thread panics while holding the lock.
This can also be used in multithreaded applications, in order to send a
message to other threads warning that a thread has panicked (e.g., for
monitoring purposes).
§
Examples
ⓘ
use
std::thread;
struct
SomeStruct;
impl
Drop
for
SomeStruct {
fn
drop(
&mut
self
) {
if
thread::panicking() {
println!
(
"dropped while unwinding"
);
        }
else
{
println!
(
"dropped while not unwinding"
);
        }
    }
}

{
print!
(
"a: "
);
let
a = SomeStruct;
}

{
print!
(
"b: "
);
let
b = SomeStruct;
panic!
()
}