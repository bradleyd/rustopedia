Termination in std::process - Rust
std
::
process
Trait
Termination
Copy item path
1.61.0
·
Source
pub trait Termination {
    // Required method
    fn
report
(self) ->
ExitCode
;
}
Expand description
A trait for implementing arbitrary return types in the
main
function.
The C-main function only supports returning integers.
So, every type implementing the
Termination
trait has to be converted
to an integer.
The default implementations are returning
libc::EXIT_SUCCESS
to indicate
a successful execution. In case of a failure,
libc::EXIT_FAILURE
is returned.
Because different runtimes have different specifications on the return value
of the
main
function, this trait is likely to be available only on
standard library’s runtime for convenience. Other runtimes are not required
to provide similar functionality.
Required Methods
§
1.61.0
·
Source
fn
report
(self) ->
ExitCode
Is called to get the representation of the value as status code.
This status code is returned to the operating system.
Implementors
§
1.61.0
·
Source
§
impl
Termination
for
Infallible
1.61.0
·
Source
§
impl
Termination
for
!
1.61.0
·
Source
§
impl
Termination
for
()
1.61.0
·
Source
§
impl
Termination
for
ExitCode
1.61.0
·
Source
§
impl<T:
Termination
, E:
Debug
>
Termination
for
Result
<T, E>