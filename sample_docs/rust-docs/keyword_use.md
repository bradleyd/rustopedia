use - Rust
Keyword
use
Copy item path
Source
Expand description
Import or rename items from other crates or modules, use values under ergonomic clones
semantic, or specify precise capturing with
use<..>
.
§
Importing items
The
use
keyword is employed to shorten the path required to refer to a module item.
The keyword may appear in modules, blocks, and even functions, typically at the top.
The most basic usage of the keyword is
use path::to::item;
,
though a number of convenient shortcuts are supported:
Simultaneously binding a list of paths with a common prefix,
using the glob-like brace syntax
use a::b::{c, d, e::f, g::h::i};
Simultaneously binding a list of paths with a common prefix and their common parent module,
using the
self
keyword, such as
use a::b::{self, c, d::e};
Rebinding the target name as a new local name, using the syntax
use p::q::r as x;
.
This can also be used with the last two features:
use a::b::{self as ab, c as abc}
.
Binding all paths matching a given prefix,
using the asterisk wildcard syntax
use a::b::*;
.
Nesting groups of the previous features multiple times,
such as
use a::b::{self as ab, c, d::{*, e::f}};
Reexporting with visibility modifiers such as
pub use a::b;
Importing with
_
to only import the methods of a trait without binding it to a name
(to avoid conflict for example):
use ::std::io::Read as _;
.
Using path qualifiers like
crate
,
super
or
self
is supported:
use crate::a::b;
.
Note that when the wildcard
*
is used on a type, it does not import its methods (though
for
enum
s it imports the variants, as shown in the example below).
ⓘ
enum
ExampleEnum {
    VariantA,
    VariantB,
}
impl
ExampleEnum {
fn
new() ->
Self
{
Self
::VariantA
    }
}
use
ExampleEnum::
*
;
// Compiles.
let _
= VariantA;
// Does not compile!
let
n = new();
For more information on
use
and paths in general, see the
Reference
.
The differences about paths and the
use
keyword between the 2015 and 2018 editions
can also be found in the
Reference
.
§
Precise capturing
The
use<..>
syntax is used within certain
impl Trait
bounds to control which generic
parameters are captured. This is important for return-position
impl Trait
(RPIT) types,
as it affects borrow checking by controlling which generic parameters can be used in the
hidden type.
For example, the following function demonstrates an error without precise capturing in
Rust 2021 and earlier editions:
ⓘ
fn
f(x:
&
()) ->
impl
Sized { x }
By using
use<'_>
for precise capturing, it can be resolved:
fn
f(x:
&
()) ->
impl
Sized +
use
<
'_
> { x }
This syntax specifies that the elided lifetime be captured and therefore available for
use in the hidden type.
In Rust 2024, opaque types automatically capture all lifetime parameters in scope.
use<..>
syntax serves as an important way of opting-out of that default.
For more details about precise capturing, see the
Reference
.
§
Ergonomic clones
Use a values, copying its content if the value implements
Copy
, cloning the contents if the
value implements
UseCloned
or moving it otherwise.