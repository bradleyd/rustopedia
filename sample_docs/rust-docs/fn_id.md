id in std::process - Rust
std
::
process
Function
id
Copy item path
1.26.0
·
Source
pub fn id() ->
u32
Expand description
Returns the OS-assigned process identifier associated with this process.
§
Examples
use
std::process;
println!
(
"My pid is {}"
, process::id());