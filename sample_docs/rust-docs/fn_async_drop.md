async_drop in std::future - Rust
std
::
future
Function
async_drop
Copy item path
Source
pub fn async_drop<T>(value: T) -> AsyncDropOwning<T>
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
Expand description
Asynchronously drops a value by running
AsyncDrop::async_drop
on a value and its fields recursively.