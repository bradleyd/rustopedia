in - Rust
Keyword
in
Copy item path
Source
Expand description
Iterate over a series of values with
for
.
The expression immediately following
in
must implement the
IntoIterator
trait.
§
Literal Examples:
for _ in 1..3 {}
- Iterate over an exclusive range up to but excluding 3.
for _ in 1..=3 {}
- Iterate over an inclusive range up to and including 3.
(Read more about
range patterns
)
The other use of
in
is with the keyword
pub
. It allows users to declare an item as visible
only within a given scope.
§
Literal Example:
pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
- fn is visible in
outer_mod
Starting with the 2018 edition, paths for
pub(in path)
must start with
crate
,
self
or
super
. The 2015 edition may also use paths starting with
::
or modules from the crate root.
For more information, see the
Reference
.