ChildExt in std::os::linux::process - Rust
std
::
os
::
linux
::
process
Trait
ChildExt
Copy item path
Source
pub trait ChildExt: Sealed {
    // Required methods
    fn
pidfd
(&self) ->
Result
<&
PidFd
>;
fn
into_pidfd
(self) ->
Result
<
PidFd
, Self>
where Self:
Sized
;
}
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Available on
Linux
only.
Expand description
Os-specific extensions for
Child
Required Methods
§
Source
fn
pidfd
(&self) ->
Result
<&
PidFd
>
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Obtains a reference to the
PidFd
created for this
Child
, if available.
A pidfd will only be available if its creation was requested with
create_pidfd
when the corresponding
Command
was created.
Even if requested, a pidfd may not be available due to an older
version of Linux being in use, or if some other error occurred.
Source
fn
into_pidfd
(self) ->
Result
<
PidFd
, Self>
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Returns the
PidFd
created for this
Child
, if available.
Otherwise self is returned.
A pidfd will only be available if its creation was requested with
create_pidfd
when the corresponding
Command
was created.
Taking ownership of the PidFd consumes the Child to avoid pid reuse
races. Use
pidfd
and
BorrowedFd::try_clone_to_owned
if
you don’t want to disassemble the Child yet.
Even if requested, a pidfd may not be available due to an older
version of Linux being in use, or if some other error occurred.
Implementors
§