identity in std::convert - Rust
std
::
convert
Function
identity
Copy item path
1.33.0 (const: 1.33.0)
·
Source
pub const fn identity<T>(x: T) -> T
Expand description
The identity function.
Two things are important to note about this function:
It is not always equivalent to a closure like
|x| x
, since the
closure may coerce
x
into a different type.
It moves the input
x
passed to the function.
While it might seem strange to have a function that just returns back the
input, there are some interesting uses.
§
Examples
Using
identity
to do nothing in a sequence of other, interesting,
functions:
use
std::convert::identity;
fn
manipulation(x: u32) -> u32 {
// Let's pretend that adding one is an interesting function.
x +
1
}
let
_arr =
&
[identity, manipulation];
Using
identity
as a “do nothing” base case in a conditional:
use
std::convert::identity;
let
do_stuff =
if
condition { manipulation }
else
{ identity };
// Do more interesting stuff...
let
_results = do_stuff(
42
);
Using
identity
to keep the
Some
variants of an iterator of
Option<T>
:
use
std::convert::identity;
let
iter = [
Some
(
1
),
None
,
Some
(
3
)].into_iter();
let
filtered = iter.filter_map(identity).collect::<Vec<
_
>>();
assert_eq!
(
vec!
[
1
,
3
], filtered);