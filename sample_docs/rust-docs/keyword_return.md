return - Rust
Keyword
return
Copy item path
Source
Expand description
Returns a value from a function.
A
return
marks the end of an execution path in a function:
fn
foo() -> i32 {
return
3
;
}
assert_eq!
(foo(),
3
);
return
is not needed when the returned value is the last expression in the
function. In this case the
;
is omitted:
fn
foo() -> i32 {
3
}
assert_eq!
(foo(),
3
);
return
returns from the function immediately (an “early return”):
use
std::fs::File;
use
std::io::{Error, ErrorKind, Read,
Result
};
fn
main() ->
Result
<()> {
let
mut
file =
match
File::open(
"foo.txt"
) {
Ok
(f) => f,
Err
(e) =>
return
Err
(e),
    };
let
mut
contents = String::new();
let
size =
match
file.read_to_string(
&mut
contents) {
Ok
(s) => s,
Err
(e) =>
return
Err
(e),
    };
if
contents.contains(
"impossible!"
) {
return
Err
(Error::new(ErrorKind::Other,
"oh no!"
));
    }
if
size >
9000
{
return
Err
(Error::new(ErrorKind::Other,
"over 9000!"
));
    }
assert_eq!
(contents,
"Hello, world!"
);
Ok
(())
}