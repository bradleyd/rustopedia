const - Rust
Keyword
const
Copy item path
Source
Expand description
Compile-time constants, compile-time evaluable functions, and raw pointers.
§
Compile-time constants
Sometimes a certain value is used many times throughout a program, and it can become
inconvenient to copy it over and over. What’s more, it’s not always possible or desirable to
make it a variable that gets carried around to each function that needs it. In these cases, the
const
keyword provides a convenient alternative to code duplication:
const
THING: u32 =
0xABAD1DEA
;
let
foo =
123
+ THING;
Constants must be explicitly typed; unlike with
let
, you can’t ignore their type and let the
compiler figure it out. Any constant value can be defined in a
const
, which in practice happens
to be most things that would be reasonable to have in a constant (barring
const fn
s). For
example, you can’t have a
File
as a
const
.
The only lifetime allowed in a constant is
'static
, which is the lifetime that encompasses
all others in a Rust program. For example, if you wanted to define a constant string, it would
look like this:
const
WORDS:
&
'static
str =
"hello rust!"
;
Thanks to static lifetime elision, you usually don’t have to explicitly use
'static
:
const
WORDS:
&
str =
"hello convenience!"
;
const
items look remarkably similar to
static
items, which introduces some confusion as
to which one should be used at which times. To put it simply, constants are inlined wherever
they’re used, making using them identical to simply replacing the name of the
const
with its
value. Static variables, on the other hand, point to a single location in memory, which all
accesses share. This means that, unlike with constants, they can’t have destructors, and act as
a single value across the entire codebase.
Constants, like statics, should always be in
SCREAMING_SNAKE_CASE
.
For more detail on
const
, see the
Rust Book
or the
Reference
.
§
Compile-time evaluable functions
The other main use of the
const
keyword is in
const fn
. This marks a function as being
callable in the body of a
const
or
static
item and in array initializers (commonly called
“const contexts”).
const fn
are restricted in the set of operations they can perform, to
ensure that they can be evaluated at compile-time. See the
Reference
for more
detail.
Turning a
fn
into a
const fn
has no effect on run-time uses of that function.
§
Other uses of
const
The
const
keyword is also used in raw pointers in combination with
mut
, as seen in
*const T
and
*mut T
. More about
const
as used in raw pointers can be read at the Rust docs for the
pointer primitive
.