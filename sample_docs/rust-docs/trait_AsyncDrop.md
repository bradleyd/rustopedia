AsyncDrop in std::future - Rust
std
::
future
Trait
AsyncDrop
Copy item path
Source
pub trait AsyncDrop {
    type
Dropper
<'a>:
Future
<Output =
()
>
where Self: 'a
;

    // Required method
    fn
async_drop
(self:
Pin
<&mut Self>) -> Self::
Dropper
<'_>;
}
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
Expand description
Custom code within the asynchronous destructor.
Required Associated Types
§
Source
type
Dropper
<'a>:
Future
<Output =
()
>
where
    Self: 'a
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
A future returned by the
AsyncDrop::async_drop
to be part
of the async destructor.
Required Methods
§
Source
fn
async_drop
(self:
Pin
<&mut Self>) -> Self::
Dropper
<'_>
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
Constructs the asynchronous destructor for this type.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§