Option in std::option - Rust
std
::
option
Enum
Option
Copy item path
1.0.0
·
Source
pub enum Option<T> {
    None,
    Some(T),
}
Expand description
The
Option
type. See
the module level documentation
for more.
Variants
§
§
1.0.0
None
No value.
§
1.0.0
Some(T)
Some value of type
T
.
Implementations
§
Source
§
impl<T>
Option
<T>
1.0.0 (const: 1.48.0)
·
Source
pub const fn
is_some
(&self) ->
bool
Returns
true
if the option is a
Some
value.
§
Examples
let
x:
Option
<u32> =
Some
(
2
);
assert_eq!
(x.is_some(),
true
);
let
x:
Option
<u32> =
None
;
assert_eq!
(x.is_some(),
false
);
1.70.0
·
Source
pub fn
is_some_and
(self, f: impl
FnOnce
(T) ->
bool
) ->
bool
Returns
true
if the option is a
Some
and the value inside of it matches a predicate.
§
Examples
let
x:
Option
<u32> =
Some
(
2
);
assert_eq!
(x.is_some_and(|x| x >
1
),
true
);
let
x:
Option
<u32> =
Some
(
0
);
assert_eq!
(x.is_some_and(|x| x >
1
),
false
);
let
x:
Option
<u32> =
None
;
assert_eq!
(x.is_some_and(|x| x >
1
),
false
);
1.0.0 (const: 1.48.0)
·
Source
pub const fn
is_none
(&self) ->
bool
Returns
true
if the option is a
None
value.
§
Examples
let
x:
Option
<u32> =
Some
(
2
);
assert_eq!
(x.is_none(),
false
);
let
x:
Option
<u32> =
None
;
assert_eq!
(x.is_none(),
true
);
1.82.0
·
Source
pub fn
is_none_or
(self, f: impl
FnOnce
(T) ->
bool
) ->
bool
Returns
true
if the option is a
None
or the value inside of it matches a predicate.
§
Examples
let
x:
Option
<u32> =
Some
(
2
);
assert_eq!
(x.is_none_or(|x| x >
1
),
true
);
let
x:
Option
<u32> =
Some
(
0
);
assert_eq!
(x.is_none_or(|x| x >
1
),
false
);
let
x:
Option
<u32> =
None
;
assert_eq!
(x.is_none_or(|x| x >
1
),
true
);
1.0.0 (const: 1.48.0)
·
Source
pub const fn
as_ref
(&self) ->
Option
<
&T
>
Converts from
&Option<T>
to
Option<&T>
.
§
Examples
Calculates the length of an
Option<
String
>
as an
Option<
usize
>
without moving the
String
. The
map
method takes the
self
argument by value,
consuming the original, so this technique uses
as_ref
to first take an
Option
to a
reference to the value inside the original.
let
text:
Option
<String> =
Some
(
"Hello, world!"
.to_string());
// First, cast `Option<String>` to `Option<&String>` with `as_ref`,
// then consume *that* with `map`, leaving `text` on the stack.
let
text_length:
Option
<usize> = text.as_ref().map(|s| s.len());
println!
(
"still can print text: {text:?}"
);
1.0.0 (const: 1.83.0)
·
Source
pub const fn
as_mut
(&mut self) ->
Option
<
&mut T
>
Converts from
&mut Option<T>
to
Option<&mut T>
.
§
Examples
let
mut
x =
Some
(
2
);
match
x.as_mut() {
Some
(v) =>
*
v =
42
,
None
=> {},
}
assert_eq!
(x,
Some
(
42
));
1.33.0 (const: 1.84.0)
·
Source
pub const fn
as_pin_ref
(self:
Pin
<&
Option
<T>>) ->
Option
<
Pin
<
&T
>>
Converts from
Pin
<
&
Option<T>>
to
Option<
Pin
<
&
T>>
.
1.33.0 (const: 1.84.0)
·
Source
pub const fn
as_pin_mut
(self:
Pin
<&mut
Option
<T>>) ->
Option
<
Pin
<
&mut T
>>
Converts from
Pin
<
&mut
Option<T>>
to
Option<
Pin
<
&mut
T>>
.
1.75.0 (const: 1.84.0)
·
Source
pub const fn
as_slice
(&self) -> &
[T]
Returns a slice of the contained value, if any. If this is
None
, an
empty slice is returned. This can be useful to have a single type of
iterator over an
Option
or slice.
Note: Should you have an
Option<&T>
and wish to get a slice of
T
,
you can unpack it via
opt.map_or(&[], std::slice::from_ref)
.
§
Examples
assert_eq!
(
    [
Some
(
1234
).as_slice(),
None
.as_slice()],
    [
&
[
1234
][..],
&
[][..]],
);
The inverse of this function is (discounting
borrowing)
[_]::first
:
for
i
in
[
Some
(
1234_u16
),
None
] {
assert_eq!
(i.as_ref(), i.as_slice().first());
}
1.75.0 (const: 1.84.0)
·
Source
pub const fn
as_mut_slice
(&mut self) -> &mut
[T]
Returns a mutable slice of the contained value, if any. If this is
None
, an empty slice is returned. This can be useful to have a
single type of iterator over an
Option
or slice.
Note: Should you have an
Option<&mut T>
instead of a
&mut Option<T>
, which this method takes, you can obtain a mutable
slice via
opt.map_or(&mut [], std::slice::from_mut)
.
§
Examples
assert_eq!
(
    [
Some
(
1234
).as_mut_slice(),
None
.as_mut_slice()],
    [
&mut
[
1234
][..],
&mut
[][..]],
);
The result is a mutable slice of zero or one items that points into
our original
Option
:
let
mut
x =
Some
(
1234
);
x.as_mut_slice()[
0
] +=
1
;
assert_eq!
(x,
Some
(
1235
));
The inverse of this method (discounting borrowing)
is
[_]::first_mut
:
assert_eq!
(
Some
(
123
).as_mut_slice().first_mut(),
Some
(
&mut
123
))
1.0.0 (const: 1.83.0)
·
Source
pub const fn
expect
(self, msg: &
str
) -> T
Returns the contained
Some
value, consuming the
self
value.
§
Panics
Panics if the value is a
None
with a custom panic message provided by
msg
.
§
Examples
let
x =
Some
(
"value"
);
assert_eq!
(x.expect(
"fruits are healthy"
),
"value"
);
ⓘ
let
x:
Option
<
&
str> =
None
;
x.expect(
"fruits are healthy"
);
// panics with `fruits are healthy`
§
Recommended Message Style
We recommend that
expect
messages are used to describe the reason you
expect
the
Option
should be
Some
.
ⓘ
let
item = slice.get(
0
)
    .expect(
"slice should not be empty"
);
Hint
: If you’re having trouble remembering how to phrase expect
error messages remember to focus on the word “should” as in “env
variable should be set by blah” or “the given binary should be available
and executable by the current user”.
For more detail on expect message styles and the reasoning behind our
recommendation please refer to the section on
“Common Message
Styles”
in the
std::error
module docs.
1.0.0 (const: 1.83.0)
·
Source
pub const fn
unwrap
(self) -> T
Returns the contained
Some
value, consuming the
self
value.
Because this function may panic, its use is generally discouraged.
Panics are meant for unrecoverable errors, and
may abort the entire program
.
Instead, prefer to use pattern matching and handle the
None
case explicitly, or call
unwrap_or
,
unwrap_or_else
, or
unwrap_or_default
. In functions returning
Option
, you can use
the
?
(try) operator
.
§
Panics
Panics if the self value equals
None
.
§
Examples
let
x =
Some
(
"air"
);
assert_eq!
(x.unwrap(),
"air"
);
ⓘ
let
x:
Option
<
&
str> =
None
;
assert_eq!
(x.unwrap(),
"air"
);
// fails
1.0.0
·
Source
pub fn
unwrap_or
(self, default: T) -> T
Returns the contained
Some
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
assert_eq!
(
Some
(
"car"
).unwrap_or(
"bike"
),
"car"
);
assert_eq!
(
None
.unwrap_or(
"bike"
),
"bike"
);
1.0.0
·
Source
pub fn
unwrap_or_else
<F>(self, f: F) -> T
where
    F:
FnOnce
() -> T,
Returns the contained
Some
value or computes it from a closure.
§
Examples
let
k =
10
;
assert_eq!
(
Some
(
4
).unwrap_or_else(||
2
* k),
4
);
assert_eq!
(
None
.unwrap_or_else(||
2
* k),
20
);
1.0.0
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
Some
value or a default.
Consumes the
self
argument then, if
Some
, returns the contained
value, otherwise if
None
, returns the
default value
for that
type.
§
Examples
let
x:
Option
<u32> =
None
;
let
y:
Option
<u32> =
Some
(
12
);
assert_eq!
(x.unwrap_or_default(),
0
);
assert_eq!
(y.unwrap_or_default(),
12
);
1.58.0 (const: 1.83.0)
·
Source
pub const unsafe fn
unwrap_unchecked
(self) -> T
Returns the contained
Some
value, consuming the
self
value,
without checking that the value is not
None
.
§
Safety
Calling this method on
None
is
undefined behavior
.
§
Examples
let
x =
Some
(
"air"
);
assert_eq!
(
unsafe
{ x.unwrap_unchecked() },
"air"
);
let
x:
Option
<
&
str> =
None
;
assert_eq!
(
unsafe
{ x.unwrap_unchecked() },
"air"
);
// Undefined behavior!
1.0.0
·
Source
pub fn
map
<U, F>(self, f: F) ->
Option
<U>
where
    F:
FnOnce
(T) -> U,
Maps an
Option<T>
to
Option<U>
by applying a function to a contained value (if
Some
) or returns
None
(if
None
).
§
Examples
Calculates the length of an
Option<
String
>
as an
Option<
usize
>
, consuming the original:
let
maybe_some_string =
Some
(String::from(
"Hello, World!"
));
// `Option::map` takes self *by value*, consuming `maybe_some_string`
let
maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!
(maybe_some_len,
Some
(
13
));
let
x:
Option
<
&
str> =
None
;
assert_eq!
(x.map(|s| s.len()),
None
);
1.76.0
·
Source
pub fn
inspect
<F>(self, f: F) ->
Option
<T>
where
    F:
FnOnce
(
&T
),
Calls a function with a reference to the contained value if
Some
.
Returns the original option.
§
Examples
let
list =
vec!
[
1
,
2
,
3
];
// prints "got: 2"
let
x = list
    .get(
1
)
    .inspect(|x|
println!
(
"got: {x}"
))
    .expect(
"list should be long enough"
);
// prints nothing
list.get(
5
).inspect(|x|
println!
(
"got: {x}"
));
1.0.0
·
Source
pub fn
map_or
<U, F>(self, default: U, f: F) -> U
where
    F:
FnOnce
(T) -> U,
Returns the provided default result (if none),
or applies a function to the contained value (if any).
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
x =
Some
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
Option
<
&
str> =
None
;
assert_eq!
(x.map_or(
42
, |v| v.len()),
42
);
1.0.0
·
Source
pub fn
map_or_else
<U, D, F>(self, default: D, f: F) -> U
where
    D:
FnOnce
() -> U,
    F:
FnOnce
(T) -> U,
Computes a default function result (if none), or
applies a different function to the contained value (if any).
§
Basic examples
let
k =
21
;
let
x =
Some
(
"foo"
);
assert_eq!
(x.map_or_else(||
2
* k, |v| v.len()),
3
);
let
x:
Option
<
&
str> =
None
;
assert_eq!
(x.map_or_else(||
2
* k, |v| v.len()),
42
);
§
Handling a Result-based fallback
A somewhat common occurrence when dealing with optional values
in combination with
Result<T, E>
is the case where one wants to invoke
a fallible fallback if the option is not present.  This example
parses a command line argument (if present), or the contents of a file to
an integer.  However, unlike accessing the command line argument, reading
the file is fallible, so it must be wrapped with
Ok
.
let
v: u64 = std::env::args()
   .nth(
1
)
   .map_or_else(|| std::fs::read_to_string(
"/etc/someconfig.conf"
),
Ok
)
?
.parse()
?
;
1.0.0
·
Source
pub fn
ok_or
<E>(self, err: E) ->
Result
<T, E>
Transforms the
Option<T>
into a
Result<T, E>
, mapping
Some(v)
to
Ok(v)
and
None
to
Err(err)
.
Arguments passed to
ok_or
are eagerly evaluated; if you are passing the
result of a function call, it is recommended to use
ok_or_else
, which is
lazily evaluated.
§
Examples
let
x =
Some
(
"foo"
);
assert_eq!
(x.ok_or(
0
),
Ok
(
"foo"
));
let
x:
Option
<
&
str> =
None
;
assert_eq!
(x.ok_or(
0
),
Err
(
0
));
1.0.0
·
Source
pub fn
ok_or_else
<E, F>(self, err: F) ->
Result
<T, E>
where
    F:
FnOnce
() -> E,
Transforms the
Option<T>
into a
Result<T, E>
, mapping
Some(v)
to
Ok(v)
and
None
to
Err(err())
.
§
Examples
let
x =
Some
(
"foo"
);
assert_eq!
(x.ok_or_else(||
0
),
Ok
(
"foo"
));
let
x:
Option
<
&
str> =
None
;
assert_eq!
(x.ok_or_else(||
0
),
Err
(
0
));
1.40.0
·
Source
pub fn
as_deref
(&self) ->
Option
<&<T as
Deref
>::
Target
>
where
    T:
Deref
,
Converts from
Option<T>
(or
&Option<T>
) to
Option<&T::Target>
.
Leaves the original Option in-place, creating a new one with a reference
to the original one, additionally coercing the contents via
Deref
.
§
Examples
let
x:
Option
<String> =
Some
(
"hey"
.to_owned());
assert_eq!
(x.as_deref(),
Some
(
"hey"
));
let
x:
Option
<String> =
None
;
assert_eq!
(x.as_deref(),
None
);
1.40.0
·
Source
pub fn
as_deref_mut
(&mut self) ->
Option
<&mut <T as
Deref
>::
Target
>
where
    T:
DerefMut
,
Converts from
Option<T>
(or
&mut Option<T>
) to
Option<&mut T::Target>
.
Leaves the original
Option
in-place, creating a new one containing a mutable reference to
the inner type’s
Deref::Target
type.
§
Examples
let
mut
x:
Option
<String> =
Some
(
"hey"
.to_owned());
assert_eq!
(x.as_deref_mut().map(|x| {
    x.make_ascii_uppercase();
    x
}),
Some
(
"HEY"
.to_owned().as_mut_str()));
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
§
Examples
let
x =
Some
(
4
);
assert_eq!
(x.iter().next(),
Some
(
&
4
));
let
x:
Option
<u32> =
None
;
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
§
Examples
let
mut
x =
Some
(
4
);
match
x.iter_mut().next() {
Some
(v) =>
*
v =
42
,
None
=> {},
}
assert_eq!
(x,
Some
(
42
));
let
mut
x:
Option
<u32> =
None
;
assert_eq!
(x.iter_mut().next(),
None
);
1.0.0
·
Source
pub fn
and
<U>(self, optb:
Option
<U>) ->
Option
<U>
Returns
None
if the option is
None
, otherwise returns
optb
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
x =
Some
(
2
);
let
y:
Option
<
&
str> =
None
;
assert_eq!
(x.and(y),
None
);
let
x:
Option
<u32> =
None
;
let
y =
Some
(
"foo"
);
assert_eq!
(x.and(y),
None
);
let
x =
Some
(
2
);
let
y =
Some
(
"foo"
);
assert_eq!
(x.and(y),
Some
(
"foo"
));
let
x:
Option
<u32> =
None
;
let
y:
Option
<
&
str> =
None
;
assert_eq!
(x.and(y),
None
);
1.0.0
·
Source
pub fn
and_then
<U, F>(self, f: F) ->
Option
<U>
where
    F:
FnOnce
(T) ->
Option
<U>,
Returns
None
if the option is
None
, otherwise calls
f
with the
wrapped value and returns the result.
Some languages call this operation flatmap.
§
Examples
fn
sq_then_to_string(x: u32) ->
Option
<String> {
    x.checked_mul(x).map(|sq| sq.to_string())
}
assert_eq!
(
Some
(
2
).and_then(sq_then_to_string),
Some
(
4
.to_string()));
assert_eq!
(
Some
(
1_000_000
).and_then(sq_then_to_string),
None
);
// overflowed!
assert_eq!
(
None
.and_then(sq_then_to_string),
None
);
Often used to chain fallible operations that may return
None
.
let
arr_2d = [[
"A0"
,
"A1"
], [
"B0"
,
"B1"
]];
let
item_0_1 = arr_2d.get(
0
).and_then(|row| row.get(
1
));
assert_eq!
(item_0_1,
Some
(
&
"A1"
));
let
item_2_0 = arr_2d.get(
2
).and_then(|row| row.get(
0
));
assert_eq!
(item_2_0,
None
);
1.27.0
·
Source
pub fn
filter
<P>(self, predicate: P) ->
Option
<T>
where
    P:
FnOnce
(
&T
) ->
bool
,
Returns
None
if the option is
None
, otherwise calls
predicate
with the wrapped value and returns:
Some(t)
if
predicate
returns
true
(where
t
is the wrapped
value), and
None
if
predicate
returns
false
.
This function works similar to
Iterator::filter()
. You can imagine
the
Option<T>
being an iterator over one or zero elements.
filter()
lets you decide which elements to keep.
§
Examples
fn
is_even(n:
&
i32) -> bool {
    n %
2
==
0
}
assert_eq!
(
None
.filter(is_even),
None
);
assert_eq!
(
Some
(
3
).filter(is_even),
None
);
assert_eq!
(
Some
(
4
).filter(is_even),
Some
(
4
));
1.0.0
·
Source
pub fn
or
(self, optb:
Option
<T>) ->
Option
<T>
Returns the option if it contains a value, otherwise returns
optb
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
x =
Some
(
2
);
let
y =
None
;
assert_eq!
(x.or(y),
Some
(
2
));
let
x =
None
;
let
y =
Some
(
100
);
assert_eq!
(x.or(y),
Some
(
100
));
let
x =
Some
(
2
);
let
y =
Some
(
100
);
assert_eq!
(x.or(y),
Some
(
2
));
let
x:
Option
<u32> =
None
;
let
y =
None
;
assert_eq!
(x.or(y),
None
);
1.0.0
·
Source
pub fn
or_else
<F>(self, f: F) ->
Option
<T>
where
    F:
FnOnce
() ->
Option
<T>,
Returns the option if it contains a value, otherwise calls
f
and
returns the result.
§
Examples
fn
nobody() ->
Option
<
&
'static
str> {
None
}
fn
vikings() ->
Option
<
&
'static
str> {
Some
(
"vikings"
) }
assert_eq!
(
Some
(
"barbarians"
).or_else(vikings),
Some
(
"barbarians"
));
assert_eq!
(
None
.or_else(vikings),
Some
(
"vikings"
));
assert_eq!
(
None
.or_else(nobody),
None
);
1.37.0
·
Source
pub fn
xor
(self, optb:
Option
<T>) ->
Option
<T>
Returns
Some
if exactly one of
self
,
optb
is
Some
, otherwise returns
None
.
§
Examples
let
x =
Some
(
2
);
let
y:
Option
<u32> =
None
;
assert_eq!
(x.xor(y),
Some
(
2
));
let
x:
Option
<u32> =
None
;
let
y =
Some
(
2
);
assert_eq!
(x.xor(y),
Some
(
2
));
let
x =
Some
(
2
);
let
y =
Some
(
2
);
assert_eq!
(x.xor(y),
None
);
let
x:
Option
<u32> =
None
;
let
y:
Option
<u32> =
None
;
assert_eq!
(x.xor(y),
None
);
1.53.0
·
Source
pub fn
insert
(&mut self, value: T) ->
&mut T
Inserts
value
into the option, then returns a mutable reference to it.
If the option already contains a value, the old value is dropped.
See also
Option::get_or_insert
, which doesn’t update the value if
the option already contains
Some
.
§
Example
let
mut
opt =
None
;
let
val = opt.insert(
1
);
assert_eq!
(
*
val,
1
);
assert_eq!
(opt.unwrap(),
1
);
let
val = opt.insert(
2
);
assert_eq!
(
*
val,
2
);
*
val =
3
;
assert_eq!
(opt.unwrap(),
3
);
1.20.0
·
Source
pub fn
get_or_insert
(&mut self, value: T) ->
&mut T
Inserts
value
into the option if it is
None
, then
returns a mutable reference to the contained value.
See also
Option::insert
, which updates the value even if
the option already contains
Some
.
§
Examples
let
mut
x =
None
;

{
let
y:
&mut
u32 = x.get_or_insert(
5
);
assert_eq!
(y,
&
5
);
*
y =
7
;
}
assert_eq!
(x,
Some
(
7
));
1.83.0
·
Source
pub fn
get_or_insert_default
(&mut self) ->
&mut T
where
    T:
Default
,
Inserts the default value into the option if it is
None
, then
returns a mutable reference to the contained value.
§
Examples
let
mut
x =
None
;

{
let
y:
&mut
u32 = x.get_or_insert_default();
assert_eq!
(y,
&
0
);
*
y =
7
;
}
assert_eq!
(x,
Some
(
7
));
1.20.0
·
Source
pub fn
get_or_insert_with
<F>(&mut self, f: F) ->
&mut T
where
    F:
FnOnce
() -> T,
Inserts a value computed from
f
into the option if it is
None
,
then returns a mutable reference to the contained value.
§
Examples
let
mut
x =
None
;

{
let
y:
&mut
u32 = x.get_or_insert_with(||
5
);
assert_eq!
(y,
&
5
);
*
y =
7
;
}
assert_eq!
(x,
Some
(
7
));
1.0.0 (const: 1.83.0)
·
Source
pub const fn
take
(&mut self) ->
Option
<T>
Takes the value out of the option, leaving a
None
in its place.
§
Examples
let
mut
x =
Some
(
2
);
let
y = x.take();
assert_eq!
(x,
None
);
assert_eq!
(y,
Some
(
2
));
let
mut
x:
Option
<u32> =
None
;
let
y = x.take();
assert_eq!
(x,
None
);
assert_eq!
(y,
None
);
1.80.0
·
Source
pub fn
take_if
<P>(&mut self, predicate: P) ->
Option
<T>
where
    P:
FnOnce
(
&mut T
) ->
bool
,
Takes the value out of the option, but only if the predicate evaluates to
true
on a mutable reference to the value.
In other words, replaces
self
with
None
if the predicate returns
true
.
This method operates similar to
Option::take
but conditional.
§
Examples
let
mut
x =
Some
(
42
);
let
prev = x.take_if(|v|
if
*
v ==
42
{
*
v +=
1
;
false
}
else
{
false
});
assert_eq!
(x,
Some
(
43
));
assert_eq!
(prev,
None
);
let
prev = x.take_if(|v|
*
v ==
43
);
assert_eq!
(x,
None
);
assert_eq!
(prev,
Some
(
43
));
1.31.0 (const: 1.83.0)
·
Source
pub const fn
replace
(&mut self, value: T) ->
Option
<T>
Replaces the actual value in the option by the value given in parameter,
returning the old value if present,
leaving a
Some
in its place without deinitializing either one.
§
Examples
let
mut
x =
Some
(
2
);
let
old = x.replace(
5
);
assert_eq!
(x,
Some
(
5
));
assert_eq!
(old,
Some
(
2
));
let
mut
x =
None
;
let
old = x.replace(
3
);
assert_eq!
(x,
Some
(
3
));
assert_eq!
(old,
None
);
1.46.0
·
Source
pub fn
zip
<U>(self, other:
Option
<U>) ->
Option
<
(T, U)
>
Zips
self
with another
Option
.
If
self
is
Some(s)
and
other
is
Some(o)
, this method returns
Some((s, o))
.
Otherwise,
None
is returned.
§
Examples
let
x =
Some
(
1
);
let
y =
Some
(
"hi"
);
let
z =
None
::<u8>;
assert_eq!
(x.zip(y),
Some
((
1
,
"hi"
)));
assert_eq!
(x.zip(z),
None
);
Source
pub fn
zip_with
<U, F, R>(self, other:
Option
<U>, f: F) ->
Option
<R>
where
    F:
FnOnce
(T, U) -> R,
🔬
This is a nightly-only experimental API. (
option_zip
#70086
)
Zips
self
and another
Option
with function
f
.
If
self
is
Some(s)
and
other
is
Some(o)
, this method returns
Some(f(s, o))
.
Otherwise,
None
is returned.
§
Examples
#![feature(option_zip)]

#[derive(Debug, PartialEq)]
struct
Point {
    x: f64,
    y: f64,
}
impl
Point {
fn
new(x: f64, y: f64) ->
Self
{
Self
{ x, y }
    }
}
let
x =
Some
(
17.5
);
let
y =
Some
(
42.7
);
assert_eq!
(x.zip_with(y, Point::new),
Some
(Point { x:
17.5
, y:
42.7
}));
assert_eq!
(x.zip_with(
None
, Point::new),
None
);
Source
§
impl<T, U>
Option
<
(T, U)
>
1.66.0
·
Source
pub fn
unzip
(self) -> (
Option
<T>,
Option
<U>)
Unzips an option containing a tuple of two options.
If
self
is
Some((a, b))
this method returns
(Some(a), Some(b))
.
Otherwise,
(None, None)
is returned.
§
Examples
let
x =
Some
((
1
,
"hi"
));
let
y =
None
::<(u8, u32)>;
assert_eq!
(x.unzip(), (
Some
(
1
),
Some
(
"hi"
)));
assert_eq!
(y.unzip(), (
None
,
None
));
Source
§
impl<T>
Option
<
&T
>
1.35.0 (const: 1.83.0)
·
Source
pub const fn
copied
(self) ->
Option
<T>
where
    T:
Copy
,
Maps an
Option<&T>
to an
Option<T>
by copying the contents of the
option.
§
Examples
let
x =
12
;
let
opt_x =
Some
(
&
x);
assert_eq!
(opt_x,
Some
(
&
12
));
let
copied = opt_x.copied();
assert_eq!
(copied,
Some
(
12
));
1.0.0
·
Source
pub fn
cloned
(self) ->
Option
<T>
where
    T:
Clone
,
Maps an
Option<&T>
to an
Option<T>
by cloning the contents of the
option.
§
Examples
let
x =
12
;
let
opt_x =
Some
(
&
x);
assert_eq!
(opt_x,
Some
(
&
12
));
let
cloned = opt_x.cloned();
assert_eq!
(cloned,
Some
(
12
));
Source
§
impl<T>
Option
<
&mut T
>
1.35.0 (const: 1.83.0)
·
Source
pub const fn
copied
(self) ->
Option
<T>
where
    T:
Copy
,
Maps an
Option<&mut T>
to an
Option<T>
by copying the contents of the
option.
§
Examples
let
mut
x =
12
;
let
opt_x =
Some
(
&mut
x);
assert_eq!
(opt_x,
Some
(
&mut
12
));
let
copied = opt_x.copied();
assert_eq!
(copied,
Some
(
12
));
1.26.0
·
Source
pub fn
cloned
(self) ->
Option
<T>
where
    T:
Clone
,
Maps an
Option<&mut T>
to an
Option<T>
by cloning the contents of the
option.
§
Examples
let
mut
x =
12
;
let
opt_x =
Some
(
&mut
x);
assert_eq!
(opt_x,
Some
(
&mut
12
));
let
cloned = opt_x.cloned();
assert_eq!
(cloned,
Some
(
12
));
Source
§
impl<T, E>
Option
<
Result
<T, E>>
1.33.0 (const: 1.83.0)
·
Source
pub const fn
transpose
(self) ->
Result
<
Option
<T>, E>
Transposes an
Option
of a
Result
into a
Result
of an
Option
.
None
will be mapped to
Ok
(
None
)
.
Some
(
Ok
(_))
and
Some
(
Err
(_))
will be mapped to
Ok
(
Some
(_))
and
Err
(_)
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
(x, y.transpose());
Source
§
impl<T>
Option
<
Option
<T>>
1.40.0 (const: 1.83.0)
·
Source
pub const fn
flatten
(self) ->
Option
<T>
Converts from
Option<Option<T>>
to
Option<T>
.
§
Examples
Basic usage:
let
x:
Option
<
Option
<u32>> =
Some
(
Some
(
6
));
assert_eq!
(
Some
(
6
), x.flatten());
let
x:
Option
<
Option
<u32>> =
Some
(
None
);
assert_eq!
(
None
, x.flatten());
let
x:
Option
<
Option
<u32>> =
None
;
assert_eq!
(
None
, x.flatten());
Flattening only removes one level of nesting at a time:
let
x:
Option
<
Option
<
Option
<u32>>> =
Some
(
Some
(
Some
(
6
)));
assert_eq!
(
Some
(
Some
(
6
)), x.flatten());
assert_eq!
(
Some
(
6
), x.flatten().flatten());
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Clone
for
Option
<T>
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
Option
<T>
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
Option
<T>)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<T>
Debug
for
Option
<T>
where
    T:
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
impl<T>
Default
for
Option
<T>
Source
§
fn
default
() ->
Option
<T>
Returns
None
.
§
Examples
let
opt:
Option
<u32> = Option::default();
assert!
(opt.is_none());
1.30.0
·
Source
§
impl<'a, T>
From
<&'a
Option
<T>> for
Option
<
&'a T
>
Source
§
fn
from
(o: &'a
Option
<T>) ->
Option
<
&'a T
>
Converts from
&Option<T>
to
Option<&T>
.
§
Examples
Converts an
Option
<
String
>
into an
Option
<
usize
>
, preserving
the original. The
map
method takes the
self
argument by value, consuming the original,
so this technique uses
from
to first take an
Option
to a reference
to the value inside the original.
let
s:
Option
<String> =
Some
(String::from(
"Hello, Rustaceans!"
));
let
o:
Option
<usize> = Option::from(
&
s).map(|ss:
&
String| ss.len());
println!
(
"Can still print s: {s:?}"
);
assert_eq!
(o,
Some
(
18
));
1.30.0
·
Source
§
impl<'a, T>
From
<&'a mut
Option
<T>> for
Option
<
&'a mut T
>
Source
§
fn
from
(o: &'a mut
Option
<T>) ->
Option
<
&'a mut T
>
Converts from
&mut Option<T>
to
Option<&mut T>
§
Examples
let
mut
s =
Some
(String::from(
"Hello"
));
let
o:
Option
<
&mut
String> = Option::from(
&mut
s);
match
o {
Some
(t) =>
*
t = String::from(
"Hello, Rustaceans!"
),
None
=> (),
}
assert_eq!
(s,
Some
(String::from(
"Hello, Rustaceans!"
)));
1.12.0
·
Source
§
impl<T>
From
<T> for
Option
<T>
Source
§
fn
from
(val: T) ->
Option
<T>
Moves
val
into a new
Some
.
§
Examples
let
o:
Option
<u8> = Option::from(
67
);
assert_eq!
(
Some
(
67
), o);
1.0.0
·
Source
§
impl<A, V>
FromIterator
<
Option
<A>> for
Option
<V>
where
    V:
FromIterator
<A>,
Source
§
fn
from_iter
<I>(iter: I) ->
Option
<V>
where
    I:
IntoIterator
<Item =
Option
<A>>,
Takes each element in the
Iterator
: if it is
None
,
no further elements are taken, and the
None
is
returned. Should no
None
occur, a container of type
V
containing the values of each
Option
is returned.
§
Examples
Here is an example which increments every integer in a vector.
We use the checked variant of
add
that returns
None
when the
calculation would result in an overflow.
let
items =
vec!
[
0_u16
,
1
,
2
];
let
res:
Option
<Vec<u16>> = items
    .iter()
    .map(|x| x.checked_add(
1
))
    .collect();
assert_eq!
(res,
Some
(
vec!
[
1
,
2
,
3
]));
As you can see, this will return the expected, valid items.
Here is another example that tries to subtract one from another list
of integers, this time checking for underflow:
let
items =
vec!
[
2_u16
,
1
,
0
];
let
res:
Option
<Vec<u16>> = items
    .iter()
    .map(|x| x.checked_sub(
1
))
    .collect();
assert_eq!
(res,
None
);
Since the last element is zero, it would underflow. Thus, the resulting
value is
None
.
Here is a variation on the previous example, showing that no
further elements are taken from
iter
after the first
None
.
let
items =
vec!
[
3_u16
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
Option
<Vec<u16>> = items
    .iter()
    .map(|x| { shared += x; x.checked_sub(
2
) })
    .collect();
assert_eq!
(res,
None
);
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
impl<T>
FromResidual
<
Option
<
Infallible
>> for
Option
<T>
Source
§
fn
from_residual
(residual:
Option
<
Infallible
>) ->
Option
<T>
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
impl<T>
FromResidual
<
Yeet
<
()
>> for
Option
<T>
Source
§
fn
from_residual
(_:
Yeet
<
()
>) ->
Option
<T>
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
impl<T>
Hash
for
Option
<T>
where
    T:
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
impl<'a, T>
IntoIterator
for &'a
Option
<T>
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
impl<'a, T>
IntoIterator
for &'a mut
Option
<T>
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
impl<T>
IntoIterator
for
Option
<T>
Source
§
fn
into_iter
(self) ->
IntoIter
<T>
ⓘ
Returns a consuming iterator over the possibly contained value.
§
Examples
let
x =
Some
(
"string"
);
let
v: Vec<
&
str> = x.into_iter().collect();
assert_eq!
(v, [
"string"
]);
let
x =
None
;
let
v: Vec<
&
str> = x.into_iter().collect();
assert!
(v.is_empty());
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
impl<T>
Ord
for
Option
<T>
where
    T:
Ord
,
Source
§
fn
cmp
(&self, other: &
Option
<T>) ->
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
impl<T>
PartialEq
for
Option
<T>
where
    T:
PartialEq
,
Source
§
fn
eq
(&self, other: &
Option
<T>) ->
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
impl<T>
PartialOrd
for
Option
<T>
where
    T:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
Option
<T>) ->
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
1.37.0
·
Source
§
impl<T, U>
Product
<
Option
<U>> for
Option
<T>
where
    T:
Product
<U>,
Source
§
fn
product
<I>(iter: I) ->
Option
<T>
where
    I:
Iterator
<Item =
Option
<U>>,
Takes each element in the
Iterator
: if it is a
None
, no further
elements are taken, and the
None
is returned. Should no
None
occur, the product of all elements is returned.
§
Examples
This multiplies each number in a vector of strings,
if a string could not be parsed the operation returns
None
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
Option
<usize> = nums.iter().map(|w| w.parse::<usize>().ok()).product();
assert_eq!
(total,
Some
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
Option
<usize> = nums.iter().map(|w| w.parse::<usize>().ok()).product();
assert_eq!
(total,
None
);
Source
§
impl<T>
Residual
<T> for
Option
<
Infallible
>
Source
§
type
TryType
=
Option
<T>
🔬
This is a nightly-only experimental API. (
try_trait_v2_residual
#91285
)
The “return” type of this meta-function.
1.37.0
·
Source
§
impl<T, U>
Sum
<
Option
<U>> for
Option
<T>
where
    T:
Sum
<U>,
Source
§
fn
sum
<I>(iter: I) ->
Option
<T>
where
    I:
Iterator
<Item =
Option
<U>>,
Takes each element in the
Iterator
: if it is a
None
, no further
elements are taken, and the
None
is returned. Should no
None
occur, the sum of all elements is returned.
§
Examples
This sums up the position of the character ‘a’ in a vector of strings,
if a word did not have the character ‘a’ the operation returns
None
:
let
words =
vec!
[
"have"
,
"a"
,
"great"
,
"day"
];
let
total:
Option
<usize> = words.iter().map(|w| w.find(
'a'
)).sum();
assert_eq!
(total,
Some
(
5
));
let
words =
vec!
[
"have"
,
"a"
,
"good"
,
"day"
];
let
total:
Option
<usize> = words.iter().map(|w| w.find(
'a'
)).sum();
assert_eq!
(total,
None
);
Source
§
impl<T>
Try
for
Option
<T>
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
Option
<
Infallible
>
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
Option
<T> as
Try
>::
Output
) ->
Option
<T>
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
Option
<T> as
Try
>::
Residual
, <
Option
<T> as
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
impl<T>
Copy
for
Option
<T>
where
    T:
Copy
,
1.0.0
·
Source
§
impl<T>
Eq
for
Option
<T>
where
    T:
Eq
,
1.0.0
·
Source
§
impl<T>
StructuralPartialEq
for
Option
<T>
Source
§
impl<T>
UseCloned
for
Option
<T>
where
    T:
UseCloned
,
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Option
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Option
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Option
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Option
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Option
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Option
<T>
where
    T:
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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