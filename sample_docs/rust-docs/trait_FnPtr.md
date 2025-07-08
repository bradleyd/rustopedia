FnPtr in std::marker - Rust
std
::
marker
Trait
FnPtr
Copy item path
Source
pub trait FnPtr:
Copy
+
Clone
{
    // Required method
    fn
addr
(self) ->
*const
()
;
}
🔬
This is a nightly-only experimental API. (
fn_ptr_trait
)
Expand description
A common trait implemented by all function pointers.
Required Methods
§
Source
fn
addr
(self) ->
*const
()
🔬
This is a nightly-only experimental API. (
fn_ptr_trait
)
Returns the address of the function pointer.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§