static - Rust
Keyword
static
Copy item path
Source
Expand description
A static item is a value which is valid for the entire duration of your
program (a
'static
lifetime).
On the surface,
static
items seem very similar to
const
s: both contain
a value, both require type annotations and both can only be initialized with
constant functions and values. However,
static
s are notably different in
that they represent a location in memory. That means that you can have
references to
static
items and potentially even modify them, making them
essentially global variables.
Static items do not call
drop
at the end of the program.
There are two types of
static
items: those declared in association with
the
mut
keyword and those without.
Static items cannot be moved:
ⓘ
static
VEC: Vec<u32> =
vec!
[];
fn
move_vec(v: Vec<u32>) -> Vec<u32> {
    v
}
// This line causes an error
move_vec(VEC);
§
Simple
static
s
Accessing non-
mut
static
items is considered safe, but some
restrictions apply. Most notably, the type of a
static
value needs to
implement the
Sync
trait, ruling out interior mutability containers
like
RefCell
. See the
Reference
for more information.
static
FOO: [i32;
5
] = [
1
,
2
,
3
,
4
,
5
];
let
r1 =
&
FOO
as
*const
_
;
let
r2 =
&
FOO
as
*const
_
;
// With a strictly read-only static, references will have the same address
assert_eq!
(r1, r2);
// A static item can be used just like a variable in many cases
println!
(
"{FOO:?}"
);
§
Mutable
static
s
If a
static
item is declared with the
mut
keyword, then it is allowed
to be modified by the program. However, accessing mutable
static
s can
cause undefined behavior in a number of ways, for example due to data races
in a multithreaded context. As such, all accesses to mutable
static
s
require an
unsafe
block.
When possible, it’s often better to use a non-mutable
static
with an
interior mutable type such as
Mutex
,
OnceLock
, or an
atomic
.
Despite their unsafety, mutable
static
s are necessary in many contexts:
they can be used to represent global state shared by the whole program or in
extern
blocks to bind to variables from C libraries.
In an
extern
block:
unsafe extern
"C"
{
static
mut
ERROR_MESSAGE:
*mut
std::os::raw::c_char;
}
Mutable
static
s, just like simple
static
s, have some restrictions that
apply to them. See the
Reference
for more information.