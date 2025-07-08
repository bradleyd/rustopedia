unwrap_binder in std::unsafe_binder - Rust
std
::
unsafe_binder
Macro
unwrap_binder
Copy item path
Source
pub macro unwrap_binder {
    ($expr:expr) => { ... },
    ($expr:expr; $ty:ty) => { ... },
}
🔬
This is a nightly-only experimental API. (
unsafe_binders
#130516
)
Expand description
Unwrap an unsafe binder into its underlying type.