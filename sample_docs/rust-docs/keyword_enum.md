enum - Rust
Keyword
enum
Copy item path
Source
Expand description
A type that can be any one of several variants.
Enums in Rust are similar to those of other compiled languages like C, but have important
differences that make them considerably more powerful. What Rust calls enums are more commonly
known as
Algebraic Data Types
if you’re coming from a functional programming background.
The important detail is that each enum variant can have data to go along with it.
enum
SimpleEnum {
    FirstVariant,
    SecondVariant,
    ThirdVariant,
}
enum
Location {
    Unknown,
    Anonymous,
    Known(Coord),
}
enum
ComplexEnum {
    Nothing,
    Something(u32),
    LotsOfThings {
        usual_struct_stuff: bool,
        blah: String,
    }
}
enum
EmptyEnum { }
The first enum shown is the usual kind of enum you’d find in a C-style language. The second
shows off a hypothetical example of something storing location data, with
Coord
being any
other type that’s needed, for example a struct. The third example demonstrates the kind of
data a variant can store, ranging from nothing, to a tuple, to an anonymous struct.
Instantiating enum variants involves explicitly using the enum’s name as its namespace,
followed by one of its variants.
SimpleEnum::SecondVariant
would be an example from above.
When data follows along with a variant, such as with rust’s built-in
Option
type, the data
is added as the type describes, for example
Option::Some(123)
. The same follows with
struct-like variants, with things looking like
ComplexEnum::LotsOfThings { usual_struct_stuff: true, blah: "hello!".to_string(), }
. Empty Enums are similar to
!
in that they cannot be
instantiated at all, and are used mainly to mess with the type system in interesting ways.
For more information, take a look at the
Rust Book
or the
Reference