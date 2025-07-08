super - Rust
Keyword
super
Copy item path
Source
Expand description
The parent of the current
module
.
mod
a {
pub fn
foo() {}
}
mod
b {
pub fn
foo() {
super
::a::foo();
// call a's foo function
}
}
It is also possible to use
super
multiple times:
super::super::foo
,
going up the ancestor chain.
See the
Reference
for more information.