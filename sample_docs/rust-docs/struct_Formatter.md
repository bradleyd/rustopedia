Formatter in std::fmt - Rust
std
::
fmt
Struct
Formatter
Copy item path
1.0.0
·
Source
pub struct Formatter<'a> {
/* private fields */
}
Expand description
Configuration for formatting.
A
Formatter
represents various options related to formatting. Users do not
construct
Formatter
s directly; a mutable reference to one is passed to
the
fmt
method of all formatting traits, like
Debug
and
Display
.
To interact with a
Formatter
, you’ll call various methods to change the
various options related to formatting. For examples, please see the
documentation of the methods defined on
Formatter
below.
Implementations
§
Source
§
impl<'a>
Formatter
<'a>
Source
pub fn
new
(
    write: &'a mut dyn
Write
,
    options:
FormattingOptions
,
) ->
Formatter
<'a>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Creates a new formatter with given
FormattingOptions
.
If
write
is a reference to a formatter, it is recommended to use
Formatter::with_options
instead as this can borrow the underlying
write
, thereby bypassing one layer of indirection.
You may alternatively use
FormattingOptions::create_formatter()
.
Source
pub fn
with_options
<'b>(
    &'b mut self,
    options:
FormattingOptions
,
) ->
Formatter
<'b>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Creates a new formatter based on this one with given
FormattingOptions
.
Source
§
impl<'a>
Formatter
<'a>
1.0.0
·
Source
pub fn
pad_integral
(
    &mut self,
    is_nonnegative:
bool
,
    prefix: &
str
,
    buf: &
str
,
) ->
Result
<
()
,
Error
>
Performs the correct padding for an integer which has already been
emitted into a str. The str should
not
contain the sign for the
integer, that will be added by this method.
§
Arguments
is_nonnegative - whether the original integer was either positive or zero.
prefix - if the ‘#’ character (Alternate) is provided, this
is the prefix to put in front of the number.
buf - the byte array that the number has been formatted into
This function will correctly account for the flags provided as well as
the minimum width. It will not take precision into account.
§
Examples
use
std::fmt;
struct
Foo { nb: i32 }
impl
Foo {
fn
new(nb: i32) -> Foo {
        Foo {
            nb,
        }
    }
}
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
// We need to remove "-" from the number output.
let
tmp =
self
.nb.abs().to_string();

        formatter.pad_integral(
self
.nb >=
0
,
"Foo "
,
&
tmp)
    }
}
assert_eq!
(
format!
(
"{}"
, Foo::new(
2
)),
"2"
);
assert_eq!
(
format!
(
"{}"
, Foo::new(-
1
)),
"-1"
);
assert_eq!
(
format!
(
"{}"
, Foo::new(
0
)),
"0"
);
assert_eq!
(
format!
(
"{:#}"
, Foo::new(-
1
)),
"-Foo 1"
);
assert_eq!
(
format!
(
"{:0>#8}"
, Foo::new(-
1
)),
"00-Foo 1"
);
1.0.0
·
Source
pub fn
pad
(&mut self, s: &
str
) ->
Result
<
()
,
Error
>
Takes a string slice and emits it to the internal buffer after applying
the relevant formatting flags specified.
The flags recognized for generic strings are:
width - the minimum width of what to emit
fill/align - what to emit and where to emit it if the string
provided needs to be padded
precision - the maximum length to emit, the string is truncated if it
is longer than this length
Notably this function ignores the
flag
parameters.
§
Examples
use
std::fmt;
struct
Foo;
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        formatter.pad(
"Foo"
)
    }
}
assert_eq!
(
format!
(
"{Foo:<4}"
),
"Foo "
);
assert_eq!
(
format!
(
"{Foo:0>4}"
),
"0Foo"
);
1.0.0
·
Source
pub fn
write_str
(&mut self, data: &
str
) ->
Result
<
()
,
Error
>
Writes some data to the underlying buffer contained within this
formatter.
§
Examples
use
std::fmt;
struct
Foo;
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        formatter.write_str(
"Foo"
)
// This is equivalent to:
        // write!(formatter, "Foo")
}
}
assert_eq!
(
format!
(
"{Foo}"
),
"Foo"
);
assert_eq!
(
format!
(
"{Foo:0>8}"
),
"Foo"
);
1.0.0
·
Source
pub fn
write_fmt
(&mut self, fmt:
Arguments
<'_>) ->
Result
<
()
,
Error
>
Glue for usage of the
write!
macro with implementors of this trait.
This method should generally not be invoked manually, but rather through
the
write!
macro itself.
Writes some formatted information into this instance.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        formatter.write_fmt(
format_args!
(
"Foo {}"
,
self
.
0
))
    }
}
assert_eq!
(
format!
(
"{}"
, Foo(-
1
)),
"Foo -1"
);
assert_eq!
(
format!
(
"{:0>8}"
, Foo(
2
)),
"Foo 2"
);
1.0.0
·
Source
pub fn
flags
(&self) ->
u32
👎
Deprecated since 1.24.0: use the
sign_plus
,
sign_minus
,
alternate
, or
sign_aware_zero_pad
methods instead
Returns flags for formatting.
1.5.0
·
Source
pub fn
fill
(&self) ->
char
Returns the character used as ‘fill’ whenever there is alignment.
§
Examples
use
std::fmt;
struct
Foo;
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
let
c = formatter.fill();
if let
Some
(width) = formatter.width() {
for _ in
0
..width {
write!
(formatter,
"{c}"
)
?
;
            }
Ok
(())
        }
else
{
write!
(formatter,
"{c}"
)
        }
    }
}
// We set alignment to the right with ">".
assert_eq!
(
format!
(
"{Foo:G>3}"
),
"GGG"
);
assert_eq!
(
format!
(
"{Foo:t>6}"
),
"tttttt"
);
1.28.0
·
Source
pub fn
align
(&self) ->
Option
<
Alignment
>
Returns a flag indicating what form of alignment was requested.
§
Examples
use
std::fmt::{
self
, Alignment};
struct
Foo;
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
let
s =
if let
Some
(s) = formatter.align() {
match
s {
                Alignment::Left    =>
"left"
,
                Alignment::Right   =>
"right"
,
                Alignment::Center  =>
"center"
,
            }
        }
else
{
"into the void"
};
write!
(formatter,
"{s}"
)
    }
}
assert_eq!
(
format!
(
"{Foo:<}"
),
"left"
);
assert_eq!
(
format!
(
"{Foo:>}"
),
"right"
);
assert_eq!
(
format!
(
"{Foo:^}"
),
"center"
);
assert_eq!
(
format!
(
"{Foo}"
),
"into the void"
);
1.5.0
·
Source
pub fn
width
(&self) ->
Option
<
usize
>
Returns the optionally specified integer width that the output should be.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
if let
Some
(width) = formatter.width() {
// If we received a width, we use it
write!
(formatter,
"{:width$}"
,
format!
(
"Foo({})"
,
self
.
0
), width = width)
        }
else
{
// Otherwise we do nothing special
write!
(formatter,
"Foo({})"
,
self
.
0
)
        }
    }
}
assert_eq!
(
format!
(
"{:10}"
, Foo(
23
)),
"Foo(23)   "
);
assert_eq!
(
format!
(
"{}"
, Foo(
23
)),
"Foo(23)"
);
1.5.0
·
Source
pub fn
precision
(&self) ->
Option
<
usize
>
Returns the optionally specified precision for numeric types.
Alternatively, the maximum width for string types.
§
Examples
use
std::fmt;
struct
Foo(f32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
if let
Some
(precision) = formatter.precision() {
// If we received a precision, we use it.
write!
(formatter,
"Foo({1:.*})"
, precision,
self
.
0
)
        }
else
{
// Otherwise we default to 2.
write!
(formatter,
"Foo({:.2})"
,
self
.
0
)
        }
    }
}
assert_eq!
(
format!
(
"{:.4}"
, Foo(
23.2
)),
"Foo(23.2000)"
);
assert_eq!
(
format!
(
"{}"
, Foo(
23.2
)),
"Foo(23.20)"
);
1.5.0
·
Source
pub fn
sign_plus
(&self) ->
bool
Determines if the
+
flag was specified.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
if
formatter.sign_plus() {
write!
(formatter,
"Foo({}{})"
,
if
self
.
0
<
0
{
'-'
}
else
{
'+'
},
self
.
0
.abs())
        }
else
{
write!
(formatter,
"Foo({})"
,
self
.
0
)
        }
    }
}
assert_eq!
(
format!
(
"{:+}"
, Foo(
23
)),
"Foo(+23)"
);
assert_eq!
(
format!
(
"{:+}"
, Foo(-
23
)),
"Foo(-23)"
);
assert_eq!
(
format!
(
"{}"
, Foo(
23
)),
"Foo(23)"
);
1.5.0
·
Source
pub fn
sign_minus
(&self) ->
bool
Determines if the
-
flag was specified.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
if
formatter.sign_minus() {
// You want a minus sign? Have one!
write!
(formatter,
"-Foo({})"
,
self
.
0
)
        }
else
{
write!
(formatter,
"Foo({})"
,
self
.
0
)
        }
    }
}
assert_eq!
(
format!
(
"{:-}"
, Foo(
23
)),
"-Foo(23)"
);
assert_eq!
(
format!
(
"{}"
, Foo(
23
)),
"Foo(23)"
);
1.5.0
·
Source
pub fn
alternate
(&self) ->
bool
Determines if the
#
flag was specified.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
if
formatter.alternate() {
write!
(formatter,
"Foo({})"
,
self
.
0
)
        }
else
{
write!
(formatter,
"{}"
,
self
.
0
)
        }
    }
}
assert_eq!
(
format!
(
"{:#}"
, Foo(
23
)),
"Foo(23)"
);
assert_eq!
(
format!
(
"{}"
, Foo(
23
)),
"23"
);
1.5.0
·
Source
pub fn
sign_aware_zero_pad
(&self) ->
bool
Determines if the
0
flag was specified.
§
Examples
use
std::fmt;
struct
Foo(i32);
impl
fmt::Display
for
Foo {
fn
fmt(
&
self
, formatter:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
assert!
(formatter.sign_aware_zero_pad());
assert_eq!
(formatter.width(),
Some
(
4
));
// We ignore the formatter's options.
write!
(formatter,
"{}"
,
self
.
0
)
    }
}
assert_eq!
(
format!
(
"{:04}"
, Foo(
23
)),
"23"
);
1.2.0
·
Source
pub fn
debug_struct
<'b>(&'b mut self, name: &
str
) ->
DebugStruct
<'b, 'a>
Creates a
DebugStruct
builder designed to assist with creation of
fmt::Debug
implementations for structs.
§
Examples
use
std::fmt;
use
std::net::Ipv4Addr;
struct
Foo {
    bar: i32,
    baz: String,
    addr: Ipv4Addr,
}
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_struct(
"Foo"
)
            .field(
"bar"
,
&
self
.bar)
            .field(
"baz"
,
&
self
.baz)
            .field(
"addr"
,
&
format_args!
(
"{}"
,
self
.addr))
            .finish()
    }
}
assert_eq!
(
"Foo { bar: 10, baz: \"Hello World\", addr: 127.0.0.1 }"
,
format!
(
"{:?}"
, Foo {
        bar:
10
,
        baz:
"Hello World"
.to_string(),
        addr: Ipv4Addr::new(
127
,
0
,
0
,
1
),
    })
);
1.2.0
·
Source
pub fn
debug_tuple
<'b>(&'b mut self, name: &
str
) ->
DebugTuple
<'b, 'a>
Creates a
DebugTuple
builder designed to assist with creation of
fmt::Debug
implementations for tuple structs.
§
Examples
use
std::fmt;
use
std::marker::PhantomData;
struct
Foo<T>(i32, String, PhantomData<T>);
impl
<T> fmt::Debug
for
Foo<T> {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_tuple(
"Foo"
)
            .field(
&
self
.
0
)
            .field(
&
self
.
1
)
            .field(
&
format_args!
(
"_"
))
            .finish()
    }
}
assert_eq!
(
"Foo(10, \"Hello\", _)"
,
format!
(
"{:?}"
, Foo(
10
,
"Hello"
.to_string(), PhantomData::<u8>))
);
1.2.0
·
Source
pub fn
debug_list
<'b>(&'b mut self) ->
DebugList
<'b, 'a>
Creates a
DebugList
builder designed to assist with creation of
fmt::Debug
implementations for list-like structures.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_list().entries(
self
.
0
.iter()).finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[
10
,
11
])),
"[10, 11]"
);
1.2.0
·
Source
pub fn
debug_set
<'b>(&'b mut self) ->
DebugSet
<'b, 'a>
Creates a
DebugSet
builder designed to assist with creation of
fmt::Debug
implementations for set-like structures.
§
Examples
use
std::fmt;
struct
Foo(Vec<i32>);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_set().entries(
self
.
0
.iter()).finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
, Foo(
vec!
[
10
,
11
])),
"{10, 11}"
);
In this more complex example, we use
format_args!
and
.debug_set()
to build a list of match arms:
use
std::fmt;
struct
Arm<
'a
, L, R>(
&
'a
(L, R));
struct
Table<
'a
, K, V>(
&
'a
[(K, V)], V);
impl
<
'a
, L, R> fmt::Debug
for
Arm<
'a
, L, R>
where
L:
'a
+ fmt::Debug, R:
'a
+ fmt::Debug
{
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        L::fmt(
&
(
self
.
0
).
0
, fmt)
?
;
        fmt.write_str(
" => "
)
?
;
        R::fmt(
&
(
self
.
0
).
1
, fmt)
    }
}
impl
<
'a
, K, V> fmt::Debug
for
Table<
'a
, K, V>
where
K:
'a
+ fmt::Debug, V:
'a
+ fmt::Debug
{
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_set()
        .entries(
self
.
0
.iter().map(Arm))
        .entry(
&
Arm(
&
(
format_args!
(
"_"
),
&
self
.
1
)))
        .finish()
    }
}
1.2.0
·
Source
pub fn
debug_map
<'b>(&'b mut self) ->
DebugMap
<'b, 'a>
Creates a
DebugMap
builder designed to assist with creation of
fmt::Debug
implementations for map-like structures.
§
Examples
use
std::fmt;
struct
Foo(Vec<(String, i32)>);
impl
fmt::Debug
for
Foo {
fn
fmt(
&
self
, fmt:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
        fmt.debug_map().entries(
self
.
0
.iter().map(|
&
(
ref
k,
ref
v)| (k, v))).finish()
    }
}
assert_eq!
(
format!
(
"{:?}"
,  Foo(
vec!
[(
"A"
.to_string(),
10
), (
"B"
.to_string(),
11
)])),
r#"{"A": 10, "B": 11}"#
);
Source
pub const fn
sign
(&self) ->
Option
<
Sign
>
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the sign of this formatter (
+
or
-
).
Source
pub const fn
options
(&self) ->
FormattingOptions
🔬
This is a nightly-only experimental API. (
formatting_options
#118117
)
Returns the formatting options this formatter corresponds to.
Trait Implementations
§
1.2.0
·
Source
§
impl
Write
for
Formatter
<'_>
Source
§
fn
write_str
(&mut self, s: &
str
) ->
Result
<
()
,
Error
>
Writes a string slice into this writer, returning whether the write
succeeded.
Read more
Source
§
fn
write_char
(&mut self, c:
char
) ->
Result
<
()
,
Error
>
Writes a
char
into this writer, returning whether the write succeeded.
Read more
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
,
Error
>
Glue for usage of the
write!
macro with implementors of this trait.
Read more
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Formatter
<'a>
§
impl<'a> !
RefUnwindSafe
for
Formatter
<'a>
§
impl<'a> !
Send
for
Formatter
<'a>
§
impl<'a> !
Sync
for
Formatter
<'a>
§
impl<'a>
Unpin
for
Formatter
<'a>
§
impl<'a> !
UnwindSafe
for
Formatter
<'a>
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