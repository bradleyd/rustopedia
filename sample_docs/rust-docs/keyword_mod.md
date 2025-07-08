mod - Rust
Keyword
mod
Copy item path
Source
Expand description
Organize code into
modules
.
Use
mod
to create new
modules
to encapsulate code, including other
modules:
mod
foo {
mod
bar {
type
MyType = (u8, u8);
fn
baz() {}
    }
}
Like
struct
s and
enum
s, a module and its content are private by
default, inaccessible to code outside of the module.
To learn more about allowing access, see the documentation for the
pub
keyword.