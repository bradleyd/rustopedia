eprint in std - Rust
std
Macro
eprint
Copy item path
1.19.0
·
Source
macro_rules! eprint {
    ($($arg:tt)*) => { ... };
}
Expand description
Prints to the standard error.
Equivalent to the
print!
macro, except that output goes to
io::stderr
instead of
io::stdout
. See
print!
for
example usage.
Use
eprint!
only for error and progress messages. Use
print!
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
eprint!
(
"Error: Could not complete task"
);