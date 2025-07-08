try in std - Rust
std
Macro
try
Copy item path
1.0.0
·
Source
macro_rules! try {
    ($expr:expr $(,)?) => { ... };
}
👎
Deprecated since 1.39.0: use the
?
operator instead
Expand description
Unwraps a result or propagates its error.
The
?
operator
was added to replace
try!
and should be used instead. Furthermore,
try
is a reserved word
in Rust 2018, so if you must use it, you will need to use the
raw-identifier syntax
:
r#try
.
try!
matches the given
Result
. In case of the
Ok
variant, the
expression has the value of the wrapped value.
In case of the
Err
variant, it retrieves the inner error.
try!
then
performs conversion using
From
. This provides automatic conversion
between specialized errors and more general ones. The resulting
error is then immediately returned.
Because of the early return,
try!
can only be used in functions that
return
Result
.
§
Examples
use
std::io;
use
std::fs::File;
use
std::io::prelude::
*
;
enum
MyError {
    FileWriteError
}
impl
From<io::Error>
for
MyError {
fn
from(e: io::Error) -> MyError {
        MyError::FileWriteError
    }
}
// The preferred method of quick returning Errors
fn
write_to_file_question() ->
Result
<(), MyError> {
let
mut
file = File::create(
"my_best_friends.txt"
)
?
;
    file.write_all(
b"This is a list of my best friends."
)
?
;
Ok
(())
}
// The previous method of quick returning Errors
fn
write_to_file_using_try() ->
Result
<(), MyError> {
let
mut
file =
r#try!
(File::create(
"my_best_friends.txt"
));
r#try!
(file.write_all(
b"This is a list of my best friends."
));
Ok
(())
}
// This is equivalent to:
fn
write_to_file_using_match() ->
Result
<(), MyError> {
let
mut
file =
r#try!
(File::create(
"my_best_friends.txt"
));
match
file.write_all(
b"This is a list of my best friends."
) {
Ok
(v) => v,
Err
(e) =>
return
Err
(From::from(e)),
    }
Ok
(())
}