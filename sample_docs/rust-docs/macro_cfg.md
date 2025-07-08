cfg in std - Rust
std
Macro
cfg
Copy item path
1.38.0
·
Source
macro_rules! cfg {
    ($($cfg:tt)*) => { ... };
}
Expand description
Evaluates boolean combinations of configuration flags at compile-time.
In addition to the
#[cfg]
attribute, this macro is provided to allow
boolean expression evaluation of configuration flags. This frequently
leads to less duplicated code.
The syntax given to this macro is the same syntax as the
cfg
attribute.
cfg!
, unlike
#[cfg]
, does not remove any code and only evaluates to true or false. For
example, all blocks in an if/else expression need to be valid when
cfg!
is used for
the condition, regardless of what
cfg!
is evaluating.
§
Examples
let
my_directory =
if
cfg!
(windows) {
"windows-specific-directory"
}
else
{
"unix-directory"
};