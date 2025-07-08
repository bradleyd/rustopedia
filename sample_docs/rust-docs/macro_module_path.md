module_path in std - Rust
std
Macro
module_path
Copy item path
1.38.0
·
Source
macro_rules! module_path {
    () => { ... };
}
Expand description
Expands to a string that represents the current module path.
The current module path can be thought of as the hierarchy of modules
leading back up to the crate root. The first component of the path
returned is the name of the crate currently being compiled.
§
Examples
mod
test {
pub fn
foo() {
assert!
(
module_path!
().ends_with(
"test"
));
    }
}

test::foo();