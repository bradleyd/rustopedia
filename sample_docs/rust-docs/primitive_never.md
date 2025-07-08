never - Rust
Primitive Type
never
Copy item path
🔬
This is a nightly-only experimental API. (
never_type
#35121
)
Expand description
The
!
type, also called “never”.
!
represents the type of computations which never resolve to any value at all. For example,
the
exit
function
fn exit(code: i32) -> !
exits the process without ever returning, and
so returns
!
.
break
,
continue
and
return
expressions also have type
!
. For example we are allowed to
write:
#![feature(never_type)]
let
x: ! = {
return
123
};
Although the
let
is pointless here, it illustrates the meaning of
!
. Since
x
is never
assigned a value (because
return
returns from the entire function),
x
can be given type
!
. We could also replace
return 123
with a
panic!
or a never-ending
loop
and this code
would still be valid.
A more realistic usage of
!
is in this code:
let
num: u32 =
match
get_a_number() {
Some
(num) => num,
None
=>
break
,
};
Both match arms must produce values of type
u32
, but since
break
never produces a value
at all we know it can never produce a value which isn’t a
u32
. This illustrates another
behavior of the
!
type - expressions with type
!
will coerce into any other type.
§
!
and generics
§
Infallible errors
The main place you’ll see
!
used explicitly is in generic code. Consider the
FromStr
trait:
trait
FromStr: Sized {
type
Err
;
fn
from_str(s:
&
str) ->
Result
<
Self
,
Self
::Err>;
}
When implementing this trait for
String
we need to pick a type for
Err
. And since
converting a string into a string will never result in an error, the appropriate type is
!
.
(Currently the type actually used is an enum with no variants, though this is only because
!
was added to Rust at a later date and it may change in the future.) With an
Err
type of
!
, if we have to call
String::from_str
for some reason the result will be a
Result<String, !>
which we can unpack like this:
#![feature(exhaustive_patterns)]
use
std::str::FromStr;
let
Ok
(s) = String::from_str(
"hello"
);
Since the
Err
variant contains a
!
, it can never occur. If the
exhaustive_patterns
feature is present this means we can exhaustively match on
Result<T, !>
by just taking the
Ok
variant. This illustrates another behavior of
!
- it can be used to “delete” certain
enum variants from generic types like
Result
.
§
Infinite loops
While
Result<T, !>
is very useful for removing errors,
!
can also be used to remove
successes as well. If we think of
Result<T, !>
as “if this function returns, it has not
errored,” we get a very intuitive idea of
Result<!, E>
as well: if the function returns, it
has
errored.
For example, consider the case of a simple web server, which can be simplified to:
ⓘ
loop
{
let
(client, request) = get_request().expect(
"disconnected"
);
let
response = request.process();
    response.send(client);
}
Currently, this isn’t ideal, because we simply panic whenever we fail to get a new connection.
Instead, we’d like to keep track of this error, like this:
ⓘ
loop
{
match
get_request() {
Err
(err) =>
break
err,
Ok
((client, request)) => {
let
response = request.process();
            response.send(client);
        },
    }
}
Now, when the server disconnects, we exit the loop with an error instead of panicking. While it
might be intuitive to simply return the error, we might want to wrap it in a
Result<!, E>
instead:
ⓘ
fn
server_loop() ->
Result
<!, ConnectionError> {
loop
{
let
(client, request) = get_request()
?
;
let
response = request.process();
        response.send(client);
    }
}
Now, we can use
?
instead of
match
, and the return type makes a lot more sense: if the loop
ever stops, it means that an error occurred. We don’t even have to wrap the loop in an
Ok
because
!
coerces to
Result<!, ConnectionError>
automatically.
§
!
and traits
When writing your own traits,
!
should have an
impl
whenever there is an obvious
impl
which doesn’t
panic!
. The reason is that functions returning an
impl Trait
where
!
does not have an
impl
of
Trait
cannot diverge as their only possible code path. In other
words, they can’t return
!
from every code path. As an example, this code doesn’t compile:
ⓘ
use
std::ops::Add;
fn
foo() ->
impl
Add<u32> {
unimplemented!
()
}
But this code does:
use
std::ops::Add;
fn
foo() ->
impl
Add<u32> {
if
true
{
unimplemented!
()
    }
else
{
0
}
}
The reason is that, in the first example, there are many possible types that
!
could coerce
to, because many types implement
Add<u32>
. However, in the second example,
the
else
branch returns a
0
, which the compiler infers from the return type to be of type
u32
. Since
u32
is a concrete type,
!
can and will be coerced to it. See issue
#36375
for more information on this quirk of
!
.
As it turns out, though, most traits can have an
impl
for
!
. Take
Debug
for example:
#![feature(never_type)]
impl
Debug
for
! {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
*
self
}
}
Once again we’re using
!
’s ability to coerce into any other type, in this case
fmt::Result
. Since this method takes a
&!
as an argument we know that it can never be
called (because there is no value of type
!
for it to be called with). Writing
*self
essentially tells the compiler “We know that this code can never be run, so just treat the
entire function body as having type
fmt::Result
”. This pattern can be used a lot when
implementing traits for
!
. Generally, any trait which only has methods which take a
self
parameter should have such an impl.
On the other hand, one trait which would not be appropriate to implement is
Default
:
trait
Default {
fn
default() ->
Self
;
}
Since
!
has no values, it has no default value either. It’s true that we could write an
impl
for this which simply panics, but the same is true for any type (we could
impl Default
for (eg.)
File
by just making
default()
panic.)
§
Never type fallback
When the compiler sees a value of type
!
in a
coercion site
, it implicitly inserts a
coercion to allow the type checker to infer any type:
ⓘ
// this
let
x: u8 =
panic!
();
// is (essentially) turned by the compiler into
let
x: u8 = absurd(
panic!
());
// where absurd is a function with the following signature
// (it's sound, because `!` always marks unreachable code):
fn
absurd<T>(
_
: !) -> T { ... }
This can lead to compilation errors if the type cannot be inferred:
ⓘ
// this
{
panic!
() };
// gets turned into this
{ absurd(
panic!
()) };
// error: can't infer the type of `absurd`
To prevent such errors, the compiler remembers where it inserted
absurd
calls, and
if it can’t infer the type, it uses the fallback type instead:
ⓘ
type
Fallback =
/* An arbitrarily selected type! */
;
{ absurd::<Fallback>(
panic!
()) }
This is what is known as “never type fallback”.
Historically, the fallback type was
()
, causing confusing behavior where
!
spontaneously
coerced to
()
, even when it would not infer
()
without the fallback. There are plans to
change it in the
2024 edition
(and possibly in all editions on a later date); see
Tracking Issue for making
!
fall back to
!
.
Trait Implementations
§
Source
§
impl
Clone
for
!
Source
§
fn
clone
(&self) ->
!
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
Source
§
impl
Debug
for
!
Source
§
fn
fmt
(&self, _: &mut
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
Source
§
impl
Display
for
!
Source
§
fn
fmt
(&self, _: &mut
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
Source
§
impl
Error
for
!
1.30.0
·
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
1.0.0
·
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0
·
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.34.0
·
Source
§
impl
From
<
!
> for
Infallible
Source
§
fn
from
(x:
!
) ->
Infallible
Converts to this type from the input type.
Source
§
impl
From
<
!
> for
TryFromIntError
Source
§
fn
from
(never:
!
) ->
TryFromIntError
Converts to this type from the input type.
1.29.0
·
Source
§
impl
Hash
for
!
Source
§
fn
hash
<H>(&self, _:
&mut H
)
where
    H:
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
1.60.0
·
Source
§
impl
Not
for
!
Source
§
type
Output
=
!
The resulting type after applying the
!
operator.
Source
§
fn
not
(self) ->
!
Performs the unary
!
operation.
Read more
Source
§
impl
Ord
for
!
Source
§
fn
cmp
(&self, _: &
!
) ->
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
Source
§
impl
PartialEq
for
!
Source
§
fn
eq
(&self, _: &
!
) ->
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
Source
§
impl
PartialOrd
for
!
Source
§
fn
partial_cmp
(&self, _: &
!
) ->
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
1.61.0
·
Source
§
impl
Termination
for
!
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
impl
Copy
for
!
Source
§
impl
Eq
for
!
Blanket Implementations
§
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