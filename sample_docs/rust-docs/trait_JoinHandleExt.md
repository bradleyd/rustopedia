JoinHandleExt in std::os::unix::thread - Rust
std
::
os
::
unix
::
thread
Trait
JoinHandleExt
Copy item path
1.9.0
·
Source
pub trait JoinHandleExt {
    // Required methods
    fn
as_pthread_t
(&self) ->
RawPthread
;
fn
into_pthread_t
(self) ->
RawPthread
;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
JoinHandle
.
Required Methods
§
1.9.0
·
Source
fn
as_pthread_t
(&self) ->
RawPthread
Extracts the raw pthread_t without taking ownership
1.9.0
·
Source
fn
into_pthread_t
(self) ->
RawPthread
Consumes the thread, returning the raw pthread_t
This function
transfers ownership
of the underlying pthread_t to
the caller. Callers are then the unique owners of the pthread_t and
must either detach or join the pthread_t once it’s no longer needed.
Implementors
§
1.9.0
·
Source
§
impl<T>
JoinHandleExt
for
JoinHandle
<T>