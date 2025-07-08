panic_any in std::panic - Rust
std
::
panic
Function
panic_any
Copy item path
1.51.0
·
Source
pub fn panic_any<M: 'static +
Any
+
Send
>(msg: M) ->
!
Expand description
Panics the current thread with the given message as the panic payload.
The message can be of any (
Any + Send
) type, not just strings.
The message is wrapped in a
Box<'static + Any + Send>
, which can be
accessed later using
PanicHookInfo::payload
.
See the
panic!
macro for more information about panicking.