move - Rust
Keyword
move
Copy item path
Source
Expand description
Capture a
closure
’s environment by value.
move
converts any variables captured by reference or mutable reference
to variables captured by value.
let
data =
vec!
[
1
,
2
,
3
];
let
closure =
move
||
println!
(
"captured {data:?} by value"
);
// data is no longer available, it is owned by the closure
Note:
move
closures may still implement
Fn
or
FnMut
, even though
they capture variables by
move
. This is because the traits implemented by
a closure type are determined by
what
the closure does with captured
values, not
how
it captures them:
fn
create_fn() ->
impl
Fn() {
let
text =
"Fn"
.to_owned();
move
||
println!
(
"This is a: {text}"
)
}
let
fn_plain = create_fn();
fn_plain();
move
is often used when
threads
are involved.
let
data =
vec!
[
1
,
2
,
3
];

std::thread::spawn(
move
|| {
println!
(
"captured {data:?} by value"
)
}).join().unwrap();
// data was moved to the spawned thread, so we cannot use it here
move
is also valid before an async block.
let
capture =
"hello"
.to_owned();
let
block =
async move
{
println!
(
"rust says {capture} from async block"
);
};
For more information on the
move
keyword, see the
closures
section
of the Rust book or the
threads
section.