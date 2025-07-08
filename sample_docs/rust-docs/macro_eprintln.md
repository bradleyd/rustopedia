eprintln in std - Rust
std
Macro
eprintln
Copy item path
1.19.0
·
Source
macro_rules! eprintln {
    () => { ... };
    ($($arg:tt)*) => { ... };
}
Expand description
Prints to the standard error, with a newline.
Equivalent to the
println!
macro, except that output goes to
io::stderr
instead of
io::stdout
. See
println!
for
example usage.
Use
eprintln!
only for error and progress messages. Use
println!
instead for the primary output of your program.
See the formatting documentation in
std::fmt
for details of the macro argument syntax.
§
Panics
Panics if writing to
io::stderr
fails.
Writing to non-blocking stderr can cause an error, which will lead
this macro to panic.
§
Examples
eprintln!
(
"Error: Could not complete task"
);