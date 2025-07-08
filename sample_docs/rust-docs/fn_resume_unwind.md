resume_unwind in std::panic - Rust
std
::
panic
Function
resume_unwind
Copy item path
1.9.0
·
Source
pub fn resume_unwind(payload:
Box
<dyn
Any
+
Send
>) ->
!
Expand description
Triggers a panic without invoking the panic hook.
This is designed to be used in conjunction with
catch_unwind
to, for
example, carry a panic across a layer of C code.
§
Notes
Note that panics in Rust are not always implemented via unwinding, but they
may be implemented by aborting the process. If this function is called when
panics are implemented this way then this function will abort the process,
not trigger an unwind.
§
Examples
ⓘ
use
std::panic;
let
result = panic::catch_unwind(|| {
if
1
!=
2
{
panic!
(
"oh no!"
);
    }
});
if let
Err
(err) = result {
    panic::resume_unwind(err);
}