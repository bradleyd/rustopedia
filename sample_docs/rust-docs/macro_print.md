print in std - Rust
std
Macro
print
Copy item path
1.0.0
·
Source
macro_rules! print {
    ($($arg:tt)*) => { ... };
}
Expand description
Prints to the standard output.
Equivalent to the
println!
macro except that a newline is not printed at
the end of the message.
Note that stdout is frequently line-buffered by default so it may be
necessary to use
io::stdout().flush()
to ensure the output is emitted
immediately.
The
print!
macro will lock the standard output on each call. If you call
print!
within a hot loop, this behavior may be the bottleneck of the loop.
To avoid this, lock stdout with
io::stdout().lock()
:
use
std::io::{stdout, Write};
let
mut
lock = stdout().lock();
write!
(lock,
"hello world"
).unwrap();
Use
print!
only for the primary output of your program. Use
eprint!
instead to print error and progress messages.
See the formatting documentation in
std::fmt
for details of the macro argument syntax.
§
Panics
Panics if writing to
io::stdout()
fails.
Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.
§
Examples
use
std::io::{
self
, Write};
print!
(
"this "
);
print!
(
"will "
);
print!
(
"be "
);
print!
(
"on "
);
print!
(
"the "
);
print!
(
"same "
);
print!
(
"line "
);

io::stdout().flush().unwrap();
print!
(
"this string has a newline, why not choose println! instead?\n"
);

io::stdout().flush().unwrap();