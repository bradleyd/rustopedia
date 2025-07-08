file in std - Rust
std
Macro
file
Copy item path
1.38.0
·
Source
macro_rules! file {
    () => { ... };
}
Expand description
Expands to the file name in which it was invoked.
With
line!
and
column!
, these macros provide debugging information for
developers about the location within the source.
The expanded expression has type
&'static str
, and the returned file
is not the invocation of the
file!
macro itself, but rather the
first macro invocation leading up to the invocation of the
file!
macro.
§
Examples
let
this_file =
file!
();
println!
(
"defined in file: {this_file}"
);