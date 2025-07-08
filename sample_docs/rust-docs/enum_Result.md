Result in std::result - Rust
std
::
result
Enum
Result
Copy item path
1.0.0
·
Source
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
Expand description
Result
is a type that represents either success (
Ok
) or failure (
Err
).
See the
module documentation
for details.
Variants
§
§
1.0.0
Ok(T)
Contains the success value
§
1.0.0
Err(E)
Contains the error value
Implementations
§
Source
§
impl<T, E>
Result
<T, E>
1.0.0 (const: 1.48.0)
·
Source
pub const fn
is_ok
(&self) ->
bool
Returns
true
if the result is
Ok
.
§
Examples
let
x:
Result
<i32,
&
str> =
Ok
(-
3
);
assert_eq!
(x.is_ok(),
true
);
let
x:
Result
<i32,
&
str> =
Err
(
"Some error message"
);
assert_eq!
(x.is_ok(),
false
);
1.70.0
·
Source
pub fn
is_ok_and
(self, f: impl
FnOnce
(T) ->
bool
) ->
bool
Returns
true
if the result is
Ok
and the value inside of it matches a predicate.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.is_ok_and(|x| x >
1
),
true
);
let
x:
Result
<u32,
&
str> =
Ok
(
0
);
assert_eq!
(x.is_ok_and(|x| x >
1
),
false
);
let
x:
Result
<u32,
&
str> =
Err
(
"hey"
);
assert_eq!
(x.is_ok_and(|x| x >
1
),
false
);
1.0.0 (const: 1.48.0)
·
Source
pub const fn
is_err
(&self) ->
bool
Returns
true
if the result is
Err
.
§
Examples
let
x:
Result
<i32,
&
str> =
Ok
(-
3
);
assert_eq!
(x.is_err(),
false
);
let
x:
Result
<i32,
&
str> =
Err
(
"Some error message"
);
assert_eq!
(x.is_err(),
true
);
1.70.0
·
Source
pub fn
is_err_and
(self, f: impl
FnOnce
(E) ->
bool
) ->
bool
Returns
true
if the result is
Err
and the value inside of it matches a predicate.
§
Examples
use
std::io::{Error, ErrorKind};
let
x:
Result
<u32, Error> =
Err
(Error::new(ErrorKind::NotFound,
"!"
));
assert_eq!
(x.is_err_and(|x| x.kind() == ErrorKind::NotFound),
true
);
let
x:
Result
<u32, Error> =
Err
(Error::new(ErrorKind::PermissionDenied,
"!"
));
assert_eq!
(x.is_err_and(|x| x.kind() == ErrorKind::NotFound),
false
);
let
x:
Result
<u32, Error> =
Ok
(
123
);
assert_eq!
(x.is_err_and(|x| x.kind() == ErrorKind::NotFound),
false
);
1.0.0
·
Source
pub fn
ok
(self) ->
Option
<T>
Converts from
Result<T, E>
to
Option<T>
.
Converts
self
into an
Option<T>
, consuming
self
,
and discarding the error, if any.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.ok(),
Some
(
2
));
let
x:
Result
<u32,
&
str> =
Err
(
"Nothing here"
);
assert_eq!
(x.ok(),
None
);
1.0.0
·
Source
pub fn
err
(self) ->
Option
<E>
Converts from
Result<T, E>
to
Option<E>
.
Converts
self
into an
Option<E>
, consuming
self
,
and discarding the success value, if any.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.err(),
None
);
let
x:
Result
<u32,
&
str> =
Err
(
"Nothing here"
);
assert_eq!
(x.err(),
Some
(
"Nothing here"
));
1.0.0 (const: 1.48.0)
·
Source
pub const fn
as_ref
(&self) ->
Result
<
&T
,
&E
>
Converts from
&Result<T, E>
to
Result<&T, &E>
.
Produces a new
Result
, containing a reference
into the original, leaving the original in place.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.as_ref(),
Ok
(
&
2
));
let
x:
Result
<u32,
&
str> =
Err
(
"Error"
);
assert_eq!
(x.as_ref(),
Err
(
&
"Error"
));
1.0.0 (const: 1.83.0)
·
Source
pub const fn
as_mut
(&mut self) ->
Result
<
&mut T
,
&mut E
>
Converts from
&mut Result<T, E>
to
Result<&mut T, &mut E>
.
§
Examples
fn
mutate(r:
&mut
Result
<i32, i32>) {
match
r.as_mut() {
Ok
(v) =>
*
v =
42
,
Err
(e) =>
*
e =
0
,
    }
}
let
mut
x:
Result
<i32, i32> =
Ok
(
2
);
mutate(
&mut
x);
assert_eq!
(x.unwrap(),
42
);
let
mut
x:
Result
<i32, i32> =
Err
(
13
);
mutate(
&mut
x);
assert_eq!
(x.unwrap_err(),
0
);
1.0.0
·
Source
pub fn
map
<U, F>(self, op: F) ->
Result
<U, E>
where
    F:
FnOnce
(T) -> U,
Maps a
Result<T, E>
to
Result<U, E>
by applying a function to a
contained
Ok
value, leaving an
Err
value untouched.
This function can be used to compose the results of two functions.
§
Examples
Print the numbers on each line of a string multiplied by two.
let
line =
"1\n2\n3\n4\n"
;
for
num
in
line.lines() {
match
num.parse::<i32>().map(|i| i *
2
) {
Ok
(n) =>
println!
(
"{n}"
),
Err
(..) => {}
    }
}
1.41.0
·
Source
pub fn
map_or
<U, F>(self, default: U, f: F) -> U
where
    F:
FnOnce
(T) -> U,
Returns the provided default (if
Err
), or
applies a function to the contained value (if
Ok
).
Arguments passed to
map_or
are eagerly evaluated; if you are passing
the result of a function call, it is recommended to use
map_or_else
,
which is lazily evaluated.
§
Examples
let
x:
Result
<
_
,
&
str> =
Ok
(
"foo"
);
assert_eq!
(x.map_or(
42
, |v| v.len()),
3
);
let
x:
Result
<
&
str,
_
> =
Err
(
"bar"
);
assert_eq!
(x.map_or(
42
, |v| v.len()),
42
);
1.41.0
·
Source
pub fn
map_or_else
<U, D, F>(self, default: D, f: F) -> U
where
    D:
FnOnce
(E) -> U,
    F:
FnOnce
(T) -> U,
Maps a
Result<T, E>
to
U
by applying fallback function
default
to
a contained
Err
value, or function
f
to a contained
Ok
value.
This function can be used to unpack a successful result
while handling an error.
§
Examples
let
k =
21
;
let
x :
Result
<
_
,
&
str> =
Ok
(
"foo"
);
assert_eq!
(x.map_or_else(|e| k *
2
, |v| v.len()),
3
);
let
x :
Result
<
&
str,
_
> =
Err
(
"bar"
);
assert_eq!
(x.map_or_else(|e| k *
2
, |v| v.len()),
42
);
1.0.0
·
Source
pub fn
map_err
<F, O>(self, op: O) ->
Result
<T, F>
where
    O:
FnOnce
(E) -> F,
Maps a
Result<T, E>
to
Result<T, F>
by applying a function to a
contained
Err
value, leaving an
Ok
value untouched.
This function can be used to pass through a successful result while handling
an error.
§
Examples
fn
stringify(x: u32) -> String {
format!
(
"error code: {x}"
) }
let
x:
Result
<u32, u32> =
Ok
(
2
);
assert_eq!
(x.map_err(stringify),
Ok
(
2
));
let
x:
Result
<u32, u32> =
Err
(
13
);
assert_eq!
(x.map_err(stringify),
Err
(
"error code: 13"
.to_string()));
1.76.0
·
Source
pub fn
inspect
<F>(self, f: F) ->
Result
<T, E>
where
    F:
FnOnce
(
&T
),
Calls a function with a reference to the contained value if
Ok
.
Returns the original result.
§
Examples
let
x: u8 =
"4"
.parse::<u8>()
    .inspect(|x|
println!
(
"original: {x}"
))
    .map(|x| x.pow(
3
))
    .expect(
"failed to parse number"
);
1.76.0
·
Source
pub fn
inspect_err
<F>(self, f: F) ->
Result
<T, E>
where
    F:
FnOnce
(
&E
),
Calls a function with a reference to the contained value if
Err
.
Returns the original result.
§
Examples
use
std::{fs, io};
fn
read() -> io::Result<String> {
    fs::read_to_string(
"address.txt"
)
        .inspect_err(|e|
eprintln!
(
"failed to read file: {e}"
))
}
1.47.0
·
Source
pub fn
as_deref
(&self) ->
Result
<&<T as
Deref
>::
Target
,
&E
>
where
    T:
Deref
,
Converts from
Result<T, E>
(or
&Result<T, E>
) to
Result<&<T as Deref>::Target, &E>
.
Coerces the
Ok
variant of the original
Result
via
Deref
and returns the new
Result
.
§
Examples
let
x:
Result
<String, u32> =
Ok
(
"hello"
.to_string());
let
y:
Result
<
&
str,
&
u32> =
Ok
(
"hello"
);
assert_eq!
(x.as_deref(), y);
let
x:
Result
<String, u32> =
Err
(
42
);
let
y:
Result
<
&
str,
&
u32> =
Err
(
&
42
);
assert_eq!
(x.as_deref(), y);
1.47.0
·
Source
pub fn
as_deref_mut
(&mut self) ->
Result
<&mut <T as
Deref
>::
Target
,
&mut E
>
where
    T:
DerefMut
,
Converts from
Result<T, E>
(or
&mut Result<T, E>
) to
Result<&mut <T as DerefMut>::Target, &mut E>
.
Coerces the
Ok
variant of the original
Result
via
DerefMut
and returns the new
Result
.
§
Examples
let
mut
s =
"HELLO"
.to_string();
let
mut
x:
Result
<String, u32> =
Ok
(
"hello"
.to_string());
let
y:
Result
<
&mut
str,
&mut
u32> =
Ok
(
&mut
s);
assert_eq!
(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
let
mut
i =
42
;
let
mut
x:
Result
<String, u32> =
Err
(
42
);
let
y:
Result
<
&mut
str,
&mut
u32> =
Err
(
&mut
i);
assert_eq!
(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns an iterator over the possibly contained value.
The iterator yields one value if the result is
Result::Ok
, otherwise none.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
7
);
assert_eq!
(x.iter().next(),
Some
(
&
7
));
let
x:
Result
<u32,
&
str> =
Err
(
"nothing!"
);
assert_eq!
(x.iter().next(),
None
);
1.0.0
·
Source
pub fn
iter_mut
(&mut self) ->
IterMut
<'_, T>
ⓘ
Returns a mutable iterator over the possibly contained value.
The iterator yields one value if the result is
Result::Ok
, otherwise none.
§
Examples
let
mut
x:
Result
<u32,
&
str> =
Ok
(
7
);
match
x.iter_mut().next() {
Some
(v) =>
*
v =
40
,
None
=> {},
}
assert_eq!
(x,
Ok
(
40
));
let
mut
x:
Result
<u32,
&
str> =
Err
(
"nothing!"
);
assert_eq!
(x.iter_mut().next(),
None
);
1.4.0
·
Source
pub fn
expect
(self, msg: &
str
) -> T
where
    E:
Debug
,
Returns the contained
Ok
value, consuming the
self
value.
Because this function may panic, its use is generally discouraged.
Instead, prefer to use pattern matching and handle the
Err
case explicitly, or call
unwrap_or
,
unwrap_or_else
, or
unwrap_or_default
.
§
Panics
Panics if the value is an
Err
, with a panic message including the
passed message, and the content of the
Err
.
§
Examples
ⓘ
let
x:
Result
<u32,
&
str> =
Err
(
"emergency failure"
);
x.expect(
"Testing expect"
);
// panics with `Testing expect: emergency failure`
§
Recommended Message Style
We recommend that
expect
messages are used to describe the reason you
expect
the
Result
should be
Ok
.
ⓘ
let
path = std::env::var(
"IMPORTANT_PATH"
)
    .expect(
"env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`"
);
Hint
: If you’re having trouble remembering how to phrase expect
error messages remember to focus on the word “should” as in “env
variable should be set by blah” or “the given binary should be available
and executable by the current user”.
For more detail on expect message styles and the reasoning behind our recommendation please
refer to the section on
“Common Message
Styles”
in the
std::error
module docs.
1.0.0
·
Source
pub fn
unwrap
(self) -> T
where
    E:
Debug
,
Returns the contained
Ok
value, consuming the
self
value.
Because this function may panic, its use is generally discouraged.
Panics are meant for unrecoverable errors, and
may abort the entire program
.
Instead, prefer to use
the
?
(try) operator
, or pattern matching
to handle the
Err
case explicitly, or call
unwrap_or
,
unwrap_or_else
, or
unwrap_or_default
.
§
Panics
Panics if the value is an
Err
, with a panic message provided by the
Err
’s value.
§
Examples
Basic usage:
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.unwrap(),
2
);
ⓘ
let
x:
Result
<u32,
&
str> =
Err
(
"emergency failure"
);
x.unwrap();
// panics with `emergency failure`
1.16.0
·
Source
pub fn
unwrap_or_default
(self) -> T
where
    T:
Default
,
Returns the contained
Ok
value or a default
Consumes the
self
argument then, if
Ok
, returns the contained
value, otherwise if
Err
, returns the default value for that
type.
§
Examples
Converts a string to an integer, turning poorly-formed strings
into 0 (the default value for integers).
parse
converts
a string to any other type that implements
FromStr
, returning an
Err
on error.
let
good_year_from_input =
"1909"
;
let
bad_year_from_input =
"190blarg"
;
let
good_year = good_year_from_input.parse().unwrap_or_default();
let
bad_year = bad_year_from_input.parse().unwrap_or_default();
assert_eq!
(
1909
, good_year);
assert_eq!
(
0
, bad_year);
1.17.0
·
Source
pub fn
expect_err
(self, msg: &
str
) -> E
where
    T:
Debug
,
Returns the contained
Err
value, consuming the
self
value.
§
Panics
Panics if the value is an
Ok
, with a panic message including the
passed message, and the content of the
Ok
.
§
Examples
ⓘ
let
x:
Result
<u32,
&
str> =
Ok
(
10
);
x.expect_err(
"Testing expect_err"
);
// panics with `Testing expect_err: 10`
1.0.0
·
Source
pub fn
unwrap_err
(self) -> E
where
    T:
Debug
,
Returns the contained
Err
value, consuming the
self
value.
§
Panics
Panics if the value is an
Ok
, with a custom panic message provided
by the
Ok
’s value.
§
Examples
ⓘ
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
x.unwrap_err();
// panics with `2`
let
x:
Result
<u32,
&
str> =
Err
(
"emergency failure"
);
assert_eq!
(x.unwrap_err(),
"emergency failure"
);
Source
pub fn
into_ok
(self) -> T
where
    E:
Into
<
!
>,
🔬
This is a nightly-only experimental API. (
unwrap_infallible
#61695
)
Returns the contained
Ok
value, but never panics.
Unlike
unwrap
, this method is known to never panic on the
result types it is implemented for. Therefore, it can be used
instead of
unwrap
as a maintainability safeguard that will fail
to compile if the error type of the
Result
is later changed
to an error that can actually occur.
§
Examples
fn
only_good_news() ->
Result
<String, !> {
Ok
(
"this is fine"
.into())
}
let
s: String = only_good_news().into_ok();
println!
(
"{s}"
);
Source
pub fn
into_err
(self) -> E
where
    T:
Into
<
!
>,
🔬
This is a nightly-only experimental API. (
unwrap_infallible
#61695
)
Returns the contained
Err
value, but never panics.
Unlike
unwrap_err
, this method is known to never panic on the
result types it is implemented for. Therefore, it can be used
instead of
unwrap_err
as a maintainability safeguard that will fail
to compile if the ok type of the
Result
is later changed
to a type that can actually occur.
§
Examples
fn
only_bad_news() ->
Result
<!, String> {
Err
(
"Oops, it failed"
.into())
}
let
error: String = only_bad_news().into_err();
println!
(
"{error}"
);
1.0.0
·
Source
pub fn
and
<U>(self, res:
Result
<U, E>) ->
Result
<U, E>
Returns
res
if the result is
Ok
, otherwise returns the
Err
value of
self
.
Arguments passed to
and
are eagerly evaluated; if you are passing the
result of a function call, it is recommended to use
and_then
, which is
lazily evaluated.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
let
y:
Result
<
&
str,
&
str> =
Err
(
"late error"
);
assert_eq!
(x.and(y),
Err
(
"late error"
));
let
x:
Result
<u32,
&
str> =
Err
(
"early error"
);
let
y:
Result
<
&
str,
&
str> =
Ok
(
"foo"
);
assert_eq!
(x.and(y),
Err
(
"early error"
));
let
x:
Result
<u32,
&
str> =
Err
(
"not a 2"
);
let
y:
Result
<
&
str,
&
str> =
Err
(
"late error"
);
assert_eq!
(x.and(y),
Err
(
"not a 2"
));
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
let
y:
Result
<
&
str,
&
str> =
Ok
(
"different result type"
);
assert_eq!
(x.and(y),
Ok
(
"different result type"
));
1.0.0
·
Source
pub fn
and_then
<U, F>(self, op: F) ->
Result
<U, E>
where
    F:
FnOnce
(T) ->
Result
<U, E>,
Calls
op
if the result is
Ok
, otherwise returns the
Err
value of
self
.
This function can be used for control flow based on
Result
values.
§
Examples
fn
sq_then_to_string(x: u32) ->
Result
<String,
&
'static
str> {
    x.checked_mul(x).map(|sq| sq.to_string()).ok_or(
"overflowed"
)
}
assert_eq!
(
Ok
(
2
).and_then(sq_then_to_string),
Ok
(
4
.to_string()));
assert_eq!
(
Ok
(
1_000_000
).and_then(sq_then_to_string),
Err
(
"overflowed"
));
assert_eq!
(
Err
(
"not a number"
).and_then(sq_then_to_string),
Err
(
"not a number"
));
Often used to chain fallible operations that may return
Err
.
use
std::{io::ErrorKind, path::Path};
// Note: on Windows "/" maps to "C:\"
let
root_modified_time = Path::new(
"/"
).metadata().and_then(|md| md.modified());
assert!
(root_modified_time.is_ok());
let
should_fail = Path::new(
"/bad/path"
).metadata().and_then(|md| md.modified());
assert!
(should_fail.is_err());
assert_eq!
(should_fail.unwrap_err().kind(), ErrorKind::NotFound);
1.0.0
·
Source
pub fn
or
<F>(self, res:
Result
<T, F>) ->
Result
<T, F>
Returns
res
if the result is
Err
, otherwise returns the
Ok
value of
self
.
Arguments passed to
or
are eagerly evaluated; if you are passing the
result of a function call, it is recommended to use
or_else
, which is
lazily evaluated.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
let
y:
Result
<u32,
&
str> =
Err
(
"late error"
);
assert_eq!
(x.or(y),
Ok
(
2
));
let
x:
Result
<u32,
&
str> =
Err
(
"early error"
);
let
y:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(x.or(y),
Ok
(
2
));
let
x:
Result
<u32,
&
str> =
Err
(
"not a 2"
);
let
y:
Result
<u32,
&
str> =
Err
(
"late error"
);
assert_eq!
(x.or(y),
Err
(
"late error"
));
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
let
y:
Result
<u32,
&
str> =
Ok
(
100
);
assert_eq!
(x.or(y),
Ok
(
2
));
1.0.0
·
Source
pub fn
or_else
<F, O>(self, op: O) ->
Result
<T, F>
where
    O:
FnOnce
(E) ->
Result
<T, F>,
Calls
op
if the result is
Err
, otherwise returns the
Ok
value of
self
.
This function can be used for control flow based on result values.
§
Examples
fn
sq(x: u32) ->
Result
<u32, u32> {
Ok
(x * x) }
fn
err(x: u32) ->
Result
<u32, u32> {
Err
(x) }
assert_eq!
(
Ok
(
2
).or_else(sq).or_else(sq),
Ok
(
2
));
assert_eq!
(
Ok
(
2
).or_else(err).or_else(sq),
Ok
(
2
));
assert_eq!
(
Err
(
3
).or_else(sq).or_else(err),
Ok
(
9
));
assert_eq!
(
Err
(
3
).or_else(err).or_else(err),
Err
(
3
));
1.0.0
·
Source
pub fn
unwrap_or
(self, default: T) -> T
Returns the contained
Ok
value or a provided default.
Arguments passed to
unwrap_or
are eagerly evaluated; if you are passing
the result of a function call, it is recommended to use
unwrap_or_else
,
which is lazily evaluated.
§
Examples
let
default =
2
;
let
x:
Result
<u32,
&
str> =
Ok
(
9
);
assert_eq!
(x.unwrap_or(default),
9
);
let
x:
Result
<u32,
&
str> =
Err
(
"error"
);
assert_eq!
(x.unwrap_or(default), default);
1.0.0
·
Source
pub fn
unwrap_or_else
<F>(self, op: F) -> T
where
    F:
FnOnce
(E) -> T,
Returns the contained
Ok
value or computes it from a closure.
§
Examples
fn
count(x:
&
str) -> usize { x.len() }
assert_eq!
(
Ok
(
2
).unwrap_or_else(count),
2
);
assert_eq!
(
Err
(
"foo"
).unwrap_or_else(count),
3
);
1.58.0
·
Source
pub unsafe fn
unwrap_unchecked
(self) -> T
Returns the contained
Ok
value, consuming the
self
value,
without checking that the value is not an
Err
.
§
Safety
Calling this method on an
Err
is
undefined behavior
.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
assert_eq!
(
unsafe
{ x.unwrap_unchecked() },
2
);
let
x:
Result
<u32,
&
str> =
Err
(
"emergency failure"
);
unsafe
{ x.unwrap_unchecked(); }
// Undefined behavior!
1.58.0
·
Source
pub unsafe fn
unwrap_err_unchecked
(self) -> E
Returns the contained
Err
value, consuming the
self
value,
without checking that the value is not an
Ok
.
§
Safety
Calling this method on an
Ok
is
undefined behavior
.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
2
);
unsafe
{ x.unwrap_err_unchecked() };
// Undefined behavior!
let
x:
Result
<u32,
&
str> =
Err
(
"emergency failure"
);
assert_eq!
(
unsafe
{ x.unwrap_err_unchecked() },
"emergency failure"
);
Source
§
impl<T, E>
Result
<
&T
, E>
1.59.0 (const: 1.83.0)
·
Source
pub const fn
copied
(self) ->
Result
<T, E>
where
    T:
Copy
,
Maps a
Result<&T, E>
to a
Result<T, E>
by copying the contents of the
Ok
part.
§
Examples
let
val =
12
;
let
x:
Result
<
&
i32, i32> =
Ok
(
&
val);
assert_eq!
(x,
Ok
(
&
12
));
let
copied = x.copied();
assert_eq!
(copied,
Ok
(
12
));
1.59.0
·
Source
pub fn
cloned
(self) ->
Result
<T, E>
where
    T:
Clone
,
Maps a
Result<&T, E>
to a
Result<T, E>
by cloning the contents of the
Ok
part.
§
Examples
let
val =
12
;
let
x:
Result
<
&
i32, i32> =
Ok
(
&
val);
assert_eq!
(x,
Ok
(
&
12
));
let
cloned = x.cloned();
assert_eq!
(cloned,
Ok
(
12
));
Source
§
impl<T, E>
Result
<
&mut T
, E>
1.59.0 (const: 1.83.0)
·
Source
pub const fn
copied
(self) ->
Result
<T, E>
where
    T:
Copy
,
Maps a
Result<&mut T, E>
to a
Result<T, E>
by copying the contents of the
Ok
part.
§
Examples
let
mut
val =
12
;
let
x:
Result
<
&mut
i32, i32> =
Ok
(
&mut
val);
assert_eq!
(x,
Ok
(
&mut
12
));
let
copied = x.copied();
assert_eq!
(copied,
Ok
(
12
));
1.59.0
·
Source
pub fn
cloned
(self) ->
Result
<T, E>
where
    T:
Clone
,
Maps a
Result<&mut T, E>
to a
Result<T, E>
by cloning the contents of the
Ok
part.
§
Examples
let
mut
val =
12
;
let
x:
Result
<
&mut
i32, i32> =
Ok
(
&mut
val);
assert_eq!
(x,
Ok
(
&mut
12
));
let
cloned = x.cloned();
assert_eq!
(cloned,
Ok
(
12
));
Source
§
impl<T, E>
Result
<
Option
<T>, E>
1.33.0 (const: 1.83.0)
·
Source
pub const fn
transpose
(self) ->
Option
<
Result
<T, E>>
Transposes a
Result
of an
Option
into an
Option
of a
Result
.
Ok(None)
will be mapped to
None
.
Ok(Some(_))
and
Err(_)
will be mapped to
Some(Ok(_))
and
Some(Err(_))
.
§
Examples
#[derive(Debug, Eq, PartialEq)]
struct
SomeErr;
let
x:
Result
<
Option
<i32>, SomeErr> =
Ok
(
Some
(
5
));
let
y:
Option
<
Result
<i32, SomeErr>> =
Some
(
Ok
(
5
));
assert_eq!
(x.transpose(), y);
Source
§
impl<T, E>
Result
<
Result
<T, E>, E>
Source
pub const fn
flatten
(self) ->
Result
<T, E>
🔬
This is a nightly-only experimental API. (
result_flattening
#70142
)
Converts from
Result<Result<T, E>, E>
to
Result<T, E>
§
Examples
#![feature(result_flattening)]
let
x:
Result
<
Result
<
&
'static
str, u32>, u32> =
Ok
(
Ok
(
"hello"
));
assert_eq!
(
Ok
(
"hello"
), x.flatten());
let
x:
Result
<
Result
<
&
'static
str, u32>, u32> =
Ok
(
Err
(
6
));
assert_eq!
(
Err
(
6
), x.flatten());
let
x:
Result
<
Result
<
&
'static
str, u32>, u32> =
Err
(
6
);
assert_eq!
(
Err
(
6
), x.flatten());
Flattening only removes one level of nesting at a time:
#![feature(result_flattening)]
let
x:
Result
<
Result
<
Result
<
&
'static
str, u32>, u32>, u32> =
Ok
(
Ok
(
Ok
(
"hello"
)));
assert_eq!
(
Ok
(
Ok
(
"hello"
)), x.flatten());
assert_eq!
(
Ok
(
"hello"
), x.flatten().flatten());
Trait Implementations
§
1.0.0
·
Source
§
impl<T, E>
Clone
for
Result
<T, E>
where
    T:
Clone
,
    E:
Clone
,
Source
§
fn
clone
(&self) ->
Result
<T, E>
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
Result
<T, E>)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<T, E>
Debug
for
Result
<T, E>
where
    T:
Debug
,
    E:
Debug
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl<A, E, V>
FromIterator
<
Result
<A, E>> for
Result
<V, E>
where
    V:
FromIterator
<A>,
Source
§
fn
from_iter
<I>(iter: I) ->
Result
<V, E>
where
    I:
IntoIterator
<Item =
Result
<A, E>>,
Takes each element in the
Iterator
: if it is an
Err
, no further
elements are taken, and the
Err
is returned. Should no
Err
occur, a
container with the values of each
Result
is returned.
Here is an example which increments every integer in a vector,
checking for overflow:
let
v =
vec!
[
1
,
2
];
let
res:
Result
<Vec<u32>,
&
'static
str> = v.iter().map(|x:
&
u32|
    x.checked_add(
1
).ok_or(
"Overflow!"
)
).collect();
assert_eq!
(res,
Ok
(
vec!
[
2
,
3
]));
Here is another example that tries to subtract one from another list
of integers, this time checking for underflow:
let
v =
vec!
[
1
,
2
,
0
];
let
res:
Result
<Vec<u32>,
&
'static
str> = v.iter().map(|x:
&
u32|
    x.checked_sub(
1
).ok_or(
"Underflow!"
)
).collect();
assert_eq!
(res,
Err
(
"Underflow!"
));
Here is a variation on the previous example, showing that no
further elements are taken from
iter
after the first
Err
.
let
v =
vec!
[
3
,
2
,
1
,
10
];
let
mut
shared =
0
;
let
res:
Result
<Vec<u32>,
&
'static
str> = v.iter().map(|x:
&
u32| {
    shared += x;
    x.checked_sub(
2
).ok_or(
"Underflow!"
)
}).collect();
assert_eq!
(res,
Err
(
"Underflow!"
));
assert_eq!
(shared,
6
);
Since the third element caused an underflow, no further elements were taken,
so the final value of
shared
is 6 (=
3 + 2 + 1
), not 16.
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Option
<
Result
<T, F>>>
where
    F:
From
<E>,
Source
§
fn
from_residual
(x:
Result
<
Infallible
, E>) ->
Poll
<
Option
<
Result
<T, F>>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Poll
<
Result
<T, F>>
where
    F:
From
<E>,
Source
§
fn
from_residual
(x:
Result
<
Infallible
, E>) ->
Poll
<
Result
<T, F>>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Source
§
impl<T, E, F>
FromResidual
<
Result
<
Infallible
, E>> for
Result
<T, F>
where
    F:
From
<E>,
Source
§
fn
from_residual
(residual:
Result
<
Infallible
, E>) ->
Result
<T, F>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Source
§
impl<T, E, F>
FromResidual
<
Yeet
<E>> for
Result
<T, F>
where
    F:
From
<E>,
Source
§
fn
from_residual
(_:
Yeet
<E>) ->
Result
<T, F>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
1.0.0
·
Source
§
impl<T, E>
Hash
for
Result
<T, E>
where
    T:
Hash
,
    E:
Hash
,
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.4.0
·
Source
§
impl<'a, T, E>
IntoIterator
for &'a
Result
<T, E>
Source
§
type
Item
=
&'a T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
1.4.0
·
Source
§
impl<'a, T, E>
IntoIterator
for &'a mut
Result
<T, E>
Source
§
type
Item
=
&'a mut T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IterMut
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
IterMut
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
1.0.0
·
Source
§
impl<T, E>
IntoIterator
for
Result
<T, E>
Source
§
fn
into_iter
(self) ->
IntoIter
<T>
ⓘ
Returns a consuming iterator over the possibly contained value.
The iterator yields one value if the result is
Result::Ok
, otherwise none.
§
Examples
let
x:
Result
<u32,
&
str> =
Ok
(
5
);
let
v: Vec<u32> = x.into_iter().collect();
assert_eq!
(v, [
5
]);
let
x:
Result
<u32,
&
str> =
Err
(
"nothing!"
);
let
v: Vec<u32> = x.into_iter().collect();
assert_eq!
(v, []);
Source
§
type
Item
= T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IntoIter
<T>
Which kind of iterator are we turning this into?
1.0.0
·
Source
§
impl<T, E>
Ord
for
Result
<T, E>
where
    T:
Ord
,
    E:
Ord
,
Source
§
fn
cmp
(&self, other: &
Result
<T, E>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.0.0
·
Source
§
impl<T, E>
PartialEq
for
Result
<T, E>
where
    T:
PartialEq
,
    E:
PartialEq
,
Source
§
fn
eq
(&self, other: &
Result
<T, E>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<T, E>
PartialOrd
for
Result
<T, E>
where
    T:
PartialOrd
,
    E:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
Result
<T, E>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.16.0
·
Source
§
impl<T, U, E>
Product
<
Result
<U, E>> for
Result
<T, E>
where
    T:
Product
<U>,
Source
§
fn
product
<I>(iter: I) ->
Result
<T, E>
where
    I:
Iterator
<Item =
Result
<U, E>>,
Takes each element in the
Iterator
: if it is an
Err
, no further
elements are taken, and the
Err
is returned. Should no
Err
occur, the product of all elements is returned.
§
Examples
This multiplies each number in a vector of strings,
if a string could not be parsed the operation returns
Err
:
let
nums =
vec!
[
"5"
,
"10"
,
"1"
,
"2"
];
let
total:
Result
<usize,
_
> = nums.iter().map(|w| w.parse::<usize>()).product();
assert_eq!
(total,
Ok
(
100
));
let
nums =
vec!
[
"5"
,
"10"
,
"one"
,
"2"
];
let
total:
Result
<usize,
_
> = nums.iter().map(|w| w.parse::<usize>()).product();
assert!
(total.is_err());
Source
§
impl<T, E>
Residual
<T> for
Result
<
Infallible
, E>
Source
§
type
TryType
=
Result
<T, E>
🔬
This is a nightly-only experimental API. (
try_trait_v2_residual
#91285
)
The “return” type of this meta-function.
1.16.0
·
Source
§
impl<T, U, E>
Sum
<
Result
<U, E>> for
Result
<T, E>
where
    T:
Sum
<U>,
Source
§
fn
sum
<I>(iter: I) ->
Result
<T, E>
where
    I:
Iterator
<Item =
Result
<U, E>>,
Takes each element in the
Iterator
: if it is an
Err
, no further
elements are taken, and the
Err
is returned. Should no
Err
occur, the sum of all elements is returned.
§
Examples
This sums up every integer in a vector, rejecting the sum if a negative
element is encountered:
let
f = |
&
x:
&
i32|
if
x <
0
{
Err
(
"Negative element found"
) }
else
{
Ok
(x) };
let
v =
vec!
[
1
,
2
];
let
res:
Result
<i32,
_
> = v.iter().map(f).sum();
assert_eq!
(res,
Ok
(
3
));
let
v =
vec!
[
1
, -
2
];
let
res:
Result
<i32,
_
> = v.iter().map(f).sum();
assert_eq!
(res,
Err
(
"Negative element found"
));
1.61.0
·
Source
§
impl<T:
Termination
, E:
Debug
>
Termination
for
Result
<T, E>
Source
§
fn
report
(self) ->
ExitCode
Is called to get the representation of the value as status code.
This status code is returned to the operating system.
Source
§
impl<T, E>
Try
for
Result
<T, E>
Source
§
type
Output
= T
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value produced by
?
when
not
short-circuiting.
Source
§
type
Residual
=
Result
<
Infallible
, E>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
The type of the value passed to
FromResidual::from_residual
as part of
?
when short-circuiting.
Read more
Source
§
fn
from_output
(output: <
Result
<T, E> as
Try
>::
Output
) ->
Result
<T, E>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from its
Output
type.
Read more
Source
§
fn
branch
(
    self,
) ->
ControlFlow
<<
Result
<T, E> as
Try
>::
Residual
, <
Result
<T, E> as
Try
>::
Output
>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Used in
?
to decide whether the operator should produce a value
(because this returned
ControlFlow::Continue
)
or propagate a value back to the caller
(because this returned
ControlFlow::Break
).
Read more
1.0.0
·
Source
§
impl<T, E>
Copy
for
Result
<T, E>
where
    T:
Copy
,
    E:
Copy
,
1.0.0
·
Source
§
impl<T, E>
Eq
for
Result
<T, E>
where
    T:
Eq
,
    E:
Eq
,
1.0.0
·
Source
§
impl<T, E>
StructuralPartialEq
for
Result
<T, E>
Source
§
impl<T, E>
UseCloned
for
Result
<T, E>
where
    T:
UseCloned
,
    E:
UseCloned
,
Auto Trait Implementations
§
§
impl<T, E>
Freeze
for
Result
<T, E>
where
    T:
Freeze
,
    E:
Freeze
,
§
impl<T, E>
RefUnwindSafe
for
Result
<T, E>
where
    T:
RefUnwindSafe
,
    E:
RefUnwindSafe
,
§
impl<T, E>
Send
for
Result
<T, E>
where
    T:
Send
,
    E:
Send
,
§
impl<T, E>
Sync
for
Result
<T, E>
where
    T:
Sync
,
    E:
Sync
,
§
impl<T, E>
Unpin
for
Result
<T, E>
where
    T:
Unpin
,
    E:
Unpin
,
§
impl<T, E>
UnwindSafe
for
Result
<T, E>
where
    T:
UnwindSafe
,
    E:
UnwindSafe
,
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.