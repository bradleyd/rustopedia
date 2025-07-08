Cow in std::borrow - Rust
std
::
borrow
Enum
Cow
Copy item path
1.0.0
·
Source
pub enum Cow<'a, B>
where
    B: 'a +
ToOwned
+ ?
Sized
,
{
    Borrowed(
&'a B
),
    Owned(<B as
ToOwned
>::
Owned
),
}
Expand description
A clone-on-write smart pointer.
The type
Cow
is a smart pointer providing clone-on-write functionality: it
can enclose and provide immutable access to borrowed data, and clone the
data lazily when mutation or ownership is required. The type is designed to
work with general borrowed data via the
Borrow
trait.
Cow
implements
Deref
, which means that you can call
non-mutating methods directly on the data it encloses. If mutation
is desired,
to_mut
will obtain a mutable reference to an owned
value, cloning if necessary.
If you need reference-counting pointers, note that
Rc::make_mut
and
Arc::make_mut
can provide clone-on-write
functionality as well.
§
Examples
use
std::borrow::Cow;
fn
abs_all(input:
&mut
Cow<
'_
, [i32]>) {
for
i
in
0
..input.len() {
let
v = input[i];
if
v <
0
{
// Clones into a vector if not already owned.
input.to_mut()[i] = -v;
        }
    }
}
// No clone occurs because `input` doesn't need to be mutated.
let
slice = [
0
,
1
,
2
];
let
mut
input = Cow::from(
&
slice[..]);
abs_all(
&mut
input);
// Clone occurs because `input` needs to be mutated.
let
slice = [-
1
,
0
,
1
];
let
mut
input = Cow::from(
&
slice[..]);
abs_all(
&mut
input);
// No clone occurs because `input` is already owned.
let
mut
input = Cow::from(
vec!
[-
1
,
0
,
1
]);
abs_all(
&mut
input);
Another example showing how to keep
Cow
in a struct:
use
std::borrow::Cow;
struct
Items<
'a
, X>
where
[X]: ToOwned<Owned = Vec<X>> {
    values: Cow<
'a
, [X]>,
}
impl
<
'a
, X: Clone +
'a
> Items<
'a
, X>
where
[X]: ToOwned<Owned = Vec<X>> {
fn
new(v: Cow<
'a
, [X]>) ->
Self
{
        Items { values: v }
    }
}
// Creates a container from borrowed values of a slice
let
readonly = [
1
,
2
];
let
borrowed = Items::new((
&
readonly[..]).into());
match
borrowed {
    Items { values: Cow::Borrowed(b) } =>
println!
(
"borrowed {b:?}"
),
_
=>
panic!
(
"expect borrowed value"
),
}
let
mut
clone_on_write = borrowed;
// Mutates the data from slice into owned vec and pushes a new value on top
clone_on_write.values.to_mut().push(
3
);
println!
(
"clone_on_write = {:?}"
, clone_on_write.values);
// The data was mutated. Let's check it out.
match
clone_on_write {
    Items { values: Cow::Owned(
_
) } =>
println!
(
"clone_on_write contains owned data"
),
_
=>
panic!
(
"expect owned data"
),
}
Variants
§
§
1.0.0
Borrowed(
&'a B
)
Borrowed data.
§
1.0.0
Owned(<B as
ToOwned
>::
Owned
)
Owned data.
Implementations
§
Source
§
impl<B>
Cow
<'_, B>
where
    B:
ToOwned
+ ?
Sized
,
Source
pub const fn
is_borrowed
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
cow_is_borrowed
#65143
)
Returns true if the data is borrowed, i.e. if
to_mut
would require additional work.
§
Examples
#![feature(cow_is_borrowed)]
use
std::borrow::Cow;
let
cow = Cow::Borrowed(
"moo"
);
assert!
(cow.is_borrowed());
let
bull: Cow<
'_
, str> = Cow::Owned(
"...moo?"
.to_string());
assert!
(!bull.is_borrowed());
Source
pub const fn
is_owned
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
cow_is_borrowed
#65143
)
Returns true if the data is owned, i.e. if
to_mut
would be a no-op.
§
Examples
#![feature(cow_is_borrowed)]
use
std::borrow::Cow;
let
cow: Cow<
'_
, str> = Cow::Owned(
"moo"
.to_string());
assert!
(cow.is_owned());
let
bull = Cow::Borrowed(
"...moo?"
);
assert!
(!bull.is_owned());
1.0.0
·
Source
pub fn
to_mut
(&mut self) -> &mut <B as
ToOwned
>::
Owned
Acquires a mutable reference to the owned form of the data.
Clones the data if it is not already owned.
§
Examples
use
std::borrow::Cow;
let
mut
cow = Cow::Borrowed(
"foo"
);
cow.to_mut().make_ascii_uppercase();
assert_eq!
(
  cow,
  Cow::Owned(String::from(
"FOO"
))
as
Cow<
'_
, str>
);
1.0.0
·
Source
pub fn
into_owned
(self) -> <B as
ToOwned
>::
Owned
Extracts the owned data.
Clones the data if it is not already owned.
§
Examples
Calling
into_owned
on a
Cow::Borrowed
returns a clone of the borrowed data:
use
std::borrow::Cow;
let
s =
"Hello world!"
;
let
cow = Cow::Borrowed(s);
assert_eq!
(
  cow.into_owned(),
  String::from(s)
);
Calling
into_owned
on a
Cow::Owned
returns the owned data. The data is moved out of the
Cow
without being cloned.
use
std::borrow::Cow;
let
s =
"Hello world!"
;
let
cow: Cow<
'_
, str> = Cow::Owned(String::from(s));
assert_eq!
(
  cow.into_owned(),
  String::from(s)
);
Trait Implementations
§
1.14.0
·
Source
§
impl<'a>
Add
<&'a
str
> for
Cow
<'a,
str
>
Source
§
type
Output
=
Cow
<'a,
str
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs: &'a
str
) -> <
Cow
<'a,
str
> as
Add
<&'a
str
>>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
Add
for
Cow
<'a,
str
>
Source
§
type
Output
=
Cow
<'a,
str
>
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Cow
<'a,
str
>) -> <
Cow
<'a,
str
> as
Add
>::
Output
Performs the
+
operation.
Read more
1.14.0
·
Source
§
impl<'a>
AddAssign
<&'a
str
> for
Cow
<'a,
str
>
Source
§
fn
add_assign
(&mut self, rhs: &'a
str
)
Performs the
+=
operation.
Read more
1.14.0
·
Source
§
impl<'a>
AddAssign
for
Cow
<'a,
str
>
Source
§
fn
add_assign
(&mut self, rhs:
Cow
<'a,
str
>)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
AsRef
<
Path
> for
Cow
<'_,
OsStr
>
Source
§
fn
as_ref
(&self) -> &
Path
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl<T>
AsRef
<T> for
Cow
<'_, T>
where
    T:
ToOwned
+ ?
Sized
,
Source
§
fn
as_ref
(&self) ->
&T
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl<'a, B>
Borrow
<B> for
Cow
<'a, B>
where
    B:
ToOwned
+ ?
Sized
,
Source
§
fn
borrow
(&self) ->
&B
Immutably borrows from an owned value.
Read more
1.0.0
·
Source
§
impl<B>
Clone
for
Cow
<'_, B>
where
    B:
ToOwned
+ ?
Sized
,
Source
§
fn
clone
(&self) ->
Cow
<'_, B>
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
Cow
<'_, B>)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<B>
Debug
for
Cow
<'_, B>
where
    B:
Debug
+
ToOwned
+ ?
Sized
,
    <B as
ToOwned
>::
Owned
:
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
1.11.0
·
Source
§
impl<B>
Default
for
Cow
<'_, B>
where
    B:
ToOwned
+ ?
Sized
,
    <B as
ToOwned
>::
Owned
:
Default
,
Source
§
fn
default
() ->
Cow
<'_, B>
Creates an owned Cow<’a, B> with the default value for the contained owned value.
1.0.0
·
Source
§
impl<B>
Deref
for
Cow
<'_, B>
where
    B:
ToOwned
+ ?
Sized
,
    <B as
ToOwned
>::
Owned
:
Borrow
<B>,
Source
§
type
Target
= B
The resulting type after dereferencing.
Source
§
fn
deref
(&self) ->
&B
Dereferences the value.
1.0.0
·
Source
§
impl<B>
Display
for
Cow
<'_, B>
where
    B:
Display
+
ToOwned
+ ?
Sized
,
    <B as
ToOwned
>::
Owned
:
Display
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
1.52.0
·
Source
§
impl<'a>
Extend
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
extend
<T:
IntoIterator
<Item =
Cow
<'a,
OsStr
>>>(&mut self, iter: T)
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, item: A)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
1.19.0
·
Source
§
impl<'a>
Extend
<
Cow
<'a,
str
>> for
String
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, s:
Cow
<'a,
str
>)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
1.8.0
·
Source
§
impl<'a, T>
From
<&'a
[T]
> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(s: &'a
[T]
) ->
Cow
<'a,
[T]
>
Creates a
Borrowed
variant of
Cow
from a slice.
This conversion does not allocate or clone the data.
1.77.0
·
Source
§
impl<'a, T, const N:
usize
>
From
<&'a
[T; N]
> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(s: &'a
[T; N]
) ->
Cow
<'a,
[T]
>
Creates a
Borrowed
variant of
Cow
from a reference to an array.
This conversion does not allocate or clone the data.
Source
§
impl<'a>
From
<&'a
ByteStr
> for
Cow
<'a,
ByteStr
>
Source
§
fn
from
(s: &'a
ByteStr
) ->
Cow
<'a,
ByteStr
>
Converts to this type from the input type.
Source
§
impl<'a>
From
<&'a
ByteString
> for
Cow
<'a,
ByteStr
>
Source
§
fn
from
(s: &'a
ByteString
) ->
Cow
<'a,
ByteStr
>
Converts to this type from the input type.
1.28.0
·
Source
§
impl<'a>
From
<&'a
CStr
> for
Cow
<'a,
CStr
>
Source
§
fn
from
(s: &'a
CStr
) ->
Cow
<'a,
CStr
>
Converts a
CStr
into a borrowed
Cow
without copying or allocating.
1.28.0
·
Source
§
impl<'a>
From
<&'a
CString
> for
Cow
<'a,
CStr
>
Source
§
fn
from
(s: &'a
CString
) ->
Cow
<'a,
CStr
>
Converts a
&
CString
into a borrowed
Cow
without copying or allocating.
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
from
(s: &'a
OsStr
) ->
Cow
<'a,
OsStr
>
Converts the string reference into a
Cow::Borrowed
.
1.28.0
·
Source
§
impl<'a>
From
<&'a
OsString
> for
Cow
<'a,
OsStr
>
Source
§
fn
from
(s: &'a
OsString
) ->
Cow
<'a,
OsStr
>
Converts the string reference into a
Cow::Borrowed
.
1.6.0
·
Source
§
impl<'a>
From
<&'a
Path
> for
Cow
<'a,
Path
>
Source
§
fn
from
(s: &'a
Path
) ->
Cow
<'a,
Path
>
Creates a clone-on-write pointer from a reference to
Path
.
This conversion does not clone or allocate.
1.28.0
·
Source
§
impl<'a>
From
<&'a
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
from
(p: &'a
PathBuf
) ->
Cow
<'a,
Path
>
Creates a clone-on-write pointer from a reference to
PathBuf
.
This conversion does not clone or allocate.
1.28.0
·
Source
§
impl<'a>
From
<&'a
String
> for
Cow
<'a,
str
>
Source
§
fn
from
(s: &'a
String
) ->
Cow
<'a,
str
>
Converts a
String
reference into a
Borrowed
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
let
s =
"eggplant"
.to_string();
assert_eq!
(Cow::from(
&
s), Cow::Borrowed(
"eggplant"
));
1.28.0
·
Source
§
impl<'a, T>
From
<&'a
Vec
<T>> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(v: &'a
Vec
<T>) ->
Cow
<'a,
[T]
>
Creates a
Borrowed
variant of
Cow
from a reference to
Vec
.
This conversion does not allocate or clone the data.
1.0.0
·
Source
§
impl<'a>
From
<&'a
str
> for
Cow
<'a,
str
>
Source
§
fn
from
(s: &'a
str
) ->
Cow
<'a,
str
>
Converts a string slice into a
Borrowed
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
assert_eq!
(Cow::from(
"eggplant"
), Cow::Borrowed(
"eggplant"
));
Source
§
impl<'a>
From
<
ByteString
> for
Cow
<'a,
ByteStr
>
Source
§
fn
from
(s:
ByteString
) ->
Cow
<'a,
ByteStr
>
Converts to this type from the input type.
1.28.0
·
Source
§
impl<'a>
From
<
CString
> for
Cow
<'a,
CStr
>
Source
§
fn
from
(s:
CString
) ->
Cow
<'a,
CStr
>
Converts a
CString
into an owned
Cow
without copying or allocating.
1.45.0
·
Source
§
impl<T>
From
<
Cow
<'_,
[T]
>> for
Box
<
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(cow:
Cow
<'_,
[T]
>) ->
Box
<
[T]
>
Converts a
Cow<'_, [T]>
into a
Box<[T]>
When
cow
is the
Cow::Borrowed
variant, this
conversion allocates on the heap and copies the
underlying slice. Otherwise, it will try to reuse the owned
Vec
’s allocation.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
CStr
>> for
Box
<
CStr
>
Source
§
fn
from
(cow:
Cow
<'_,
CStr
>) ->
Box
<
CStr
>
Converts a
Cow<'a, CStr>
into a
Box<CStr>
,
by copying the contents if they are borrowed.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
OsStr
>> for
Box
<
OsStr
>
Source
§
fn
from
(cow:
Cow
<'_,
OsStr
>) ->
Box
<
OsStr
>
Converts a
Cow<'a, OsStr>
into a
Box
<
OsStr
>
,
by copying the contents if they are borrowed.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
Path
>> for
Box
<
Path
>
Source
§
fn
from
(cow:
Cow
<'_,
Path
>) ->
Box
<
Path
>
Creates a boxed
Path
from a clone-on-write pointer.
Converting from a
Cow::Owned
does not clone or allocate.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
str
>> for
Box
<
str
>
Source
§
fn
from
(cow:
Cow
<'_,
str
>) ->
Box
<
str
>
Converts a
Cow<'_, str>
into a
Box<str>
When
cow
is the
Cow::Borrowed
variant, this
conversion allocates on the heap and copies the
underlying
str
. Otherwise, it will try to reuse the owned
String
’s allocation.
§
Examples
use
std::borrow::Cow;
let
unboxed = Cow::Borrowed(
"hello"
);
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
let
unboxed = Cow::Owned(
"hello"
.to_string());
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
1.14.0
·
Source
§
impl<'a, T>
From
<
Cow
<'a,
[T]
>> for
Vec
<T>
where
[T]
:
ToOwned
<Owned =
Vec
<T>>,
Source
§
fn
from
(s:
Cow
<'a,
[T]
>) ->
Vec
<T>
Converts a clone-on-write slice into a vector.
If
s
already owns a
Vec<T>
, it will be returned directly.
If
s
is borrowing a slice, a new
Vec<T>
will be allocated and
filled by cloning
s
’s items into it.
§
Examples
let
o: Cow<
'_
, [i32]> = Cow::Owned(
vec!
[
1
,
2
,
3
]);
let
b: Cow<
'_
, [i32]> = Cow::Borrowed(
&
[
1
,
2
,
3
]);
assert_eq!
(Vec::from(o), Vec::from(b));
1.45.0
·
Source
§
impl<'a, B>
From
<
Cow
<'a, B>> for
Arc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Arc
<B>:
From
<
&'a B
> +
From
<<B as
ToOwned
>::
Owned
>,
Source
§
fn
from
(cow:
Cow
<'a, B>) ->
Arc
<B>
Creates an atomically reference-counted pointer from a clone-on-write
pointer by copying its content.
§
Example
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
let
shared: Arc<str> = Arc::from(cow);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.45.0
·
Source
§
impl<'a, B>
From
<
Cow
<'a, B>> for
Rc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Rc
<B>:
From
<
&'a B
> +
From
<<B as
ToOwned
>::
Owned
>,
Source
§
fn
from
(cow:
Cow
<'a, B>) ->
Rc
<B>
Creates a reference-counted pointer from a clone-on-write pointer by
copying its content.
§
Example
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
let
shared: Rc<str> = Rc::from(cow);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
CStr
>> for
CString
Source
§
fn
from
(s:
Cow
<'a,
CStr
>) ->
CString
Converts a
Cow<'a, CStr>
into a
CString
, by copying the contents if they are
borrowed.
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
from
(s:
Cow
<'a,
OsStr
>) -> Self
Converts a
Cow<'a, OsStr>
into an
OsString
,
by copying the contents if they are borrowed.
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
from
(p:
Cow
<'a,
Path
>) -> Self
Converts a clone-on-write pointer to an owned path.
Converting from a
Cow::Owned
does not clone or allocate.
1.14.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
str
>> for
String
Source
§
fn
from
(s:
Cow
<'a,
str
>) ->
String
Converts a clone-on-write string to an owned
instance of
String
.
This extracts the owned string,
clones the string if it is not already owned.
§
Example
// If the string is not owned...
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
// It will allocate on the heap and copy the string.
let
owned: String = String::from(cow);
assert_eq!
(
&
owned[..],
"eggplant"
);
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+ 'a>
Converts a
Cow
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error>::from(a_cow_str_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
Cow
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_cow_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.28.0
·
Source
§
impl<'a>
From
<
OsString
> for
Cow
<'a,
OsStr
>
Source
§
fn
from
(s:
OsString
) ->
Cow
<'a,
OsStr
>
Moves the string into a
Cow::Owned
.
1.6.0
·
Source
§
impl<'a>
From
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
from
(s:
PathBuf
) ->
Cow
<'a,
Path
>
Creates a clone-on-write pointer from an owned
instance of
PathBuf
.
This conversion does not clone or allocate.
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Cow
<'a,
str
>
Source
§
fn
from
(s:
String
) ->
Cow
<'a,
str
>
Converts a
String
into an
Owned
variant.
No heap allocation is performed, and the string
is not copied.
§
Example
let
s =
"eggplant"
.to_string();
let
s2 =
"eggplant"
.to_string();
assert_eq!
(Cow::from(s), Cow::<
'static
, str>::Owned(s2));
1.8.0
·
Source
§
impl<'a, T>
From
<
Vec
<T>> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(v:
Vec
<T>) ->
Cow
<'a,
[T]
>
Creates an
Owned
variant of
Cow
from an owned instance of
Vec
.
This conversion does not allocate or clone the data.
1.12.0
·
Source
§
impl<'a, 'b>
FromIterator
<&'b
str
> for
Cow
<'a,
str
>
Source
§
fn
from_iter
<I>(it: I) ->
Cow
<'a,
str
>
where
    I:
IntoIterator
<Item = &'b
str
>,
Creates a value from an iterator.
Read more
1.52.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
from_iter
<I:
IntoIterator
<Item =
Cow
<'a,
OsStr
>>>(iter: I) -> Self
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Creates a value from an iterator.
Read more
1.19.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
String
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Creates a value from an iterator.
Read more
1.12.0
·
Source
§
impl<'a>
FromIterator
<
String
> for
Cow
<'a,
str
>
Source
§
fn
from_iter
<I>(it: I) ->
Cow
<'a,
str
>
where
    I:
IntoIterator
<Item =
String
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl<'a, T>
FromIterator
<T> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
Source
§
fn
from_iter
<I>(it: I) ->
Cow
<'a,
[T]
>
where
    I:
IntoIterator
<Item = T>,
Creates a value from an iterator.
Read more
1.12.0
·
Source
§
impl<'a>
FromIterator
<
char
> for
Cow
<'a,
str
>
Source
§
fn
from_iter
<I>(it: I) ->
Cow
<'a,
str
>
where
    I:
IntoIterator
<Item =
char
>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl<B>
Hash
for
Cow
<'_, B>
where
    B:
Hash
+
ToOwned
+ ?
Sized
,
Source
§
fn
hash
<H>(&self, state:
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
1.0.0
·
Source
§
impl<B>
Ord
for
Cow
<'_, B>
where
    B:
Ord
+
ToOwned
+ ?
Sized
,
Source
§
fn
cmp
(&self, other: &
Cow
<'_, B>) ->
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
impl<T, U>
PartialEq
<&
[U]
> for
Cow
<'_,
[T]
>
where
    T:
PartialEq
<U> +
Clone
,
Source
§
fn
eq
(&self, other: &&
[U]
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &&
[U]
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<&'a
ByteStr
> for
Cow
<'a, [
u8
]>
Source
§
fn
eq
(&self, other: &&'a
ByteStr
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
impl<'a>
PartialEq
<&'a
ByteStr
> for
Cow
<'a,
ByteStr
>
Source
§
fn
eq
(&self, other: &&'a
ByteStr
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
impl<'a>
PartialEq
<&'a
ByteStr
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &&'a
ByteStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &&'b
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &&'b
OsStr
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
1.6.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'b
Path
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &&'b
Path
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<&'a
Path
> for
Cow
<'b,
OsStr
>
Source
§
fn
eq
(&self, other: &&'a
Path
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
1.0.0
·
Source
§
impl<T, U>
PartialEq
<&mut
[U]
> for
Cow
<'_,
[T]
>
where
    T:
PartialEq
<U> +
Clone
,
Source
§
fn
eq
(&self, other: &&mut
[U]
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &&mut
[U]
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
impl<'a, 'b>
PartialEq
<&'b
str
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &&'b
str
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &&'b
str
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_, [
u8
]>
Source
§
fn
eq
(&self, other: &
ByteString
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
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_,
ByteStr
>
Source
§
fn
eq
(&self, other: &
ByteString
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
impl<'a>
PartialEq
<
ByteString
> for
Cow
<'_,
str
>
Source
§
fn
eq
(&self, other: &
ByteString
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
impl<'a>
PartialEq
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_, [
u8
]>) ->
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
impl<'a>
PartialEq
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_,
ByteStr
>) ->
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
impl<'a>
PartialEq
<
Cow
<'_,
str
>> for
ByteString
Source
§
fn
eq
(&self, other: &
Cow
<'_,
str
>) ->
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
impl<'a>
PartialEq
<
Cow
<'a, [
u8
]>> for &'a
ByteStr
Source
§
fn
eq
(&self, other: &
Cow
<'a, [
u8
]>) ->
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
impl<'a>
PartialEq
<
Cow
<'a,
ByteStr
>> for &'a
ByteStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
ByteStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
OsStr
>> for &'b
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
OsStr
>> for
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
OsStr
>> for
Path
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
OsStr
>> for
PathBuf
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
Path
>> for &'b
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.6.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
Path
>> for &'b
Path
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
OsStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
OsString
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
Path
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
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
impl<'a>
PartialEq
<
Cow
<'a,
str
>> for &'a
ByteStr
Source
§
fn
eq
(&self, other: &
Cow
<'a,
str
>) ->
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
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for &'b
str
Source
§
fn
eq
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for
String
Source
§
fn
eq
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'a,
str
>> for
str
Source
§
fn
eq
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Cow
<'a,
str
>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<'a, 'b, B, C>
PartialEq
<
Cow
<'b, C>> for
Cow
<'a, B>
where
    B:
PartialEq
<C> +
ToOwned
+ ?
Sized
,
    C:
ToOwned
+ ?
Sized
,
Source
§
fn
eq
(&self, other: &
Cow
<'b, C>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
Cow
<'b,
OsStr
>> for &'a
Path
Source
§
fn
eq
(&self, other: &
Cow
<'b,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialEq
<
OsString
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
OsString
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Path
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
Path
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
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Path
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
Path
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
1.8.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
PathBuf
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
1.6.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
PathBuf
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
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
String
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &
String
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
String
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
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for
Cow
<'_,
[T]
>
where
    A:
Allocator
,
    T:
PartialEq
<U> +
Clone
,
Source
§
fn
eq
(&self, other: &
Vec
<U, A>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
Vec
<U, A>) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<'a, 'b>
PartialEq
<
str
> for
Cow
<'a,
str
>
Source
§
fn
eq
(&self, other: &
str
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
str
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a, [
u8
]>
Source
§
fn
partial_cmp
(&self, other: &&'a
ByteStr
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
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a,
ByteStr
>
Source
§
fn
partial_cmp
(&self, other: &&'a
ByteStr
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
Source
§
impl<'a>
PartialOrd
<&'a
ByteStr
> for
Cow
<'a,
str
>
Source
§
fn
partial_cmp
(&self, other: &&'a
ByteStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &&'b
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &&'b
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'b
Path
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &&'b
Path
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<&'a
Path
> for
Cow
<'b,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &&'a
Path
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
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_, [
u8
]>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
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
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
ByteStr
>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
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
Source
§
impl<'a>
PartialOrd
<
ByteString
> for
Cow
<'_,
str
>
Source
§
fn
partial_cmp
(&self, other: &
ByteString
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'_, [
u8
]>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_, [
u8
]>) ->
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
ByteStr
>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_,
ByteStr
>) ->
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'_,
str
>> for
ByteString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'_,
str
>) ->
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'a, [
u8
]>> for &'a
ByteStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a, [
u8
]>) ->
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
ByteStr
>> for &'a
ByteStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
ByteStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for &'b
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
OsStr
>> for
OsString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
OsStr
>> for
Path
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
OsStr
>> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
Path
>> for &'b
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'a,
Path
>> for &'b
Path
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
OsString
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
Path
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
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
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
str
>> for &'a
ByteStr
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
str
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
Cow
<'b,
OsStr
>> for &'a
Path
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'b,
OsStr
>) ->
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsStr
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsStr
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
OsStr
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
1.8.0
·
Source
§
impl<'a, 'b>
PartialOrd
<
OsString
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
OsString
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
OsString
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
Path
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Path
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
Path
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
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
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
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
1.0.0
·
Source
§
impl<'a, B>
PartialOrd
for
Cow
<'a, B>
where
    B:
PartialOrd
+
ToOwned
+ ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a, B>) ->
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
Source
§
impl<T>
DerefPure
for
Cow
<'_,
[T]
>
where
    T:
Clone
,
Source
§
impl<T>
DerefPure
for
Cow
<'_, T>
where
    T:
Clone
,
Source
§
impl
DerefPure
for
Cow
<'_,
str
>
1.0.0
·
Source
§
impl<B>
Eq
for
Cow
<'_, B>
where
    B:
Eq
+
ToOwned
+ ?
Sized
,
Auto Trait Implementations
§
§
impl<'a, B>
Freeze
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Freeze
,
    B: ?
Sized
,
§
impl<'a, B>
RefUnwindSafe
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
RefUnwindSafe
,
    B:
RefUnwindSafe
+ ?
Sized
,
§
impl<'a, B>
Send
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Send
,
    B:
Sync
+ ?
Sized
,
§
impl<'a, B>
Sync
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Sync
,
    B:
Sync
+ ?
Sized
,
§
impl<'a, B>
Unpin
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
Unpin
,
    B: ?
Sized
,
§
impl<'a, B>
UnwindSafe
for
Cow
<'a, B>
where
    <B as
ToOwned
>::
Owned
:
UnwindSafe
,
    B:
RefUnwindSafe
+ ?
Sized
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
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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