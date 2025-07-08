type - Rust
Keyword
type
Copy item path
Source
Expand description
Define an
alias
for an existing type.
The syntax is
type Name = ExistingType;
.
§
Examples
type
does
not
create a new type:
type
Meters = u32;
type
Kilograms = u32;
let
m: Meters =
3
;
let
k: Kilograms =
3
;
assert_eq!
(m, k);
A type can be generic:
type
ArcMutex<T> = Arc<Mutex<T>>;
In traits,
type
is used to declare an
associated type
:
trait
Iterator {
// associated type declaration
type
Item;
fn
next(
&mut
self
) ->
Option
<
Self
::Item>;
}
struct
Once<T>(
Option
<T>);
impl
<T> Iterator
for
Once<T> {
// associated type definition
type
Item = T;
fn
next(
&mut
self
) ->
Option
<
Self
::Item> {
self
.
0
.take()
    }
}