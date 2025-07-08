SliceIndex in std::slice - Rust
std
::
slice
Trait
SliceIndex
Copy item path
1.28.0
·
Source
pub unsafe trait SliceIndex<T>: Sealed
where
    T: ?
Sized
,
{
    type
Output
: ?
Sized
;

    // Required methods
    fn
get
(self, slice:
&T
) ->
Option
<&Self::
Output
>;
fn
get_mut
(self, slice:
&mut T
) ->
Option
<&mut Self::
Output
>;
unsafe fn
get_unchecked
(self, slice:
*const T
) ->
*const
Self::
Output
;
unsafe fn
get_unchecked_mut
(self, slice:
*mut T
) ->
*mut
Self::
Output
;
fn
index
(self, slice:
&T
) -> &Self::
Output
;
fn
index_mut
(self, slice:
&mut T
) -> &mut Self::
Output
;
}
Expand description
A helper trait used for indexing operations.
Implementations of this trait have to promise that if the argument
to
get_unchecked(_mut)
is a safe reference, then so is the result.
Required Associated Types
§
1.28.0
·
Source
type
Output
: ?
Sized
The output type returned by methods.
Required Methods
§
Source
fn
get
(self, slice:
&T
) ->
Option
<&Self::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
fn
get_mut
(self, slice:
&mut T
) ->
Option
<&mut Self::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
unsafe fn
get_unchecked
(self, slice:
*const T
) ->
*const
Self::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Calling this method with an out-of-bounds index or a dangling
slice
pointer
is
undefined behavior
even if the resulting pointer is not used.
Source
unsafe fn
get_unchecked_mut
(self, slice:
*mut T
) ->
*mut
Self::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Calling this method with an out-of-bounds index or a dangling
slice
pointer
is
undefined behavior
even if the resulting pointer is not used.
Source
fn
index
(self, slice:
&T
) -> &Self::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
fn
index_mut
(self, slice:
&mut T
) -> &mut Self::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
Implementors
§
1.73.0
·
Source
§
impl
SliceIndex
<
str
> for (
Bound
<
usize
>,
Bound
<
usize
>)
Implements substring slicing for arbitrary bounds.
Returns a slice of the given string bounded by the byte indices
provided by each bound.
This operation is
O
(1).
§
Panics
Panics if
begin
or
end
(if it exists and once adjusted for
inclusion/exclusion) does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
begin > end
, or if
end > len
.
Source
§
type
Output
=
str
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for std::ops::
Range
<
usize
>
Implements substring slicing with syntax
&self[begin .. end]
or
&mut self[begin .. end]
.
Returns a slice of the given string from the byte range
[
begin
,
end
).
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
begin
or
end
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
begin > end
, or if
end > len
.
§
Examples
let
s =
"Löwe 老虎 Léopard"
;
assert_eq!
(
&
s[
0
..
1
],
"L"
);
assert_eq!
(
&
s[
1
..
9
],
"öwe 老"
);
// these will panic:
// byte 2 lies within `ö`:
// &s[2 ..3];

// byte 8 lies within `老`
// &s[1 .. 8];

// byte 100 is outside the string
// &s[3 .. 100];
Source
§
type
Output
=
str
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for std::ops::
RangeFrom
<
usize
>
Implements substring slicing with syntax
&self[begin ..]
or
&mut self[begin ..]
.
Returns a slice of the given string from the byte range [
begin
,
len
).
Equivalent to
&self[begin .. len]
or
&mut self[begin .. len]
.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
begin
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), or if
begin > len
.
Source
§
type
Output
=
str
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeFull
Implements substring slicing with syntax
&self[..]
or
&mut self[..]
.
Returns a slice of the whole string, i.e., returns
&self
or
&mut self
. Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
. Unlike
other indexing operations, this can never panic.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
.
Source
§
type
Output
=
str
1.26.0
·
Source
§
impl
SliceIndex
<
str
> for std::ops::
RangeInclusive
<
usize
>
Implements substring slicing with syntax
&self[begin ..= end]
or
&mut self[begin ..= end]
.
Returns a slice of the given string from the byte range
[
begin
,
end
]. Equivalent to
&self [begin .. end + 1]
or
&mut self[begin .. end + 1]
, except if
end
has the maximum value for
usize
.
This operation is
O
(1).
§
Panics
Panics if
begin
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
end
does not point
to the ending byte offset of a character (
end + 1
is either a starting
byte offset or equal to
len
), if
begin > end
, or if
end >= len
.
Source
§
type
Output
=
str
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeTo
<
usize
>
Implements substring slicing with syntax
&self[.. end]
or
&mut self[.. end]
.
Returns a slice of the given string from the byte range [0,
end
).
Equivalent to
&self[0 .. end]
or
&mut self[0 .. end]
.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
end
does not point to the starting byte offset of a
character (as defined by
is_char_boundary
), or if
end > len
.
Source
§
type
Output
=
str
1.26.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeToInclusive
<
usize
>
Implements substring slicing with syntax
&self[..= end]
or
&mut self[..= end]
.
Returns a slice of the given string from the byte range [0,
end
].
Equivalent to
&self [0 .. end + 1]
, except if
end
has the maximum
value for
usize
.
This operation is
O
(1).
§
Panics
Panics if
end
does not point to the ending byte offset of a character
(
end + 1
is either a starting byte offset as defined by
is_char_boundary
, or equal to
len
), or if
end >= len
.
Source
§
type
Output
=
str
Source
§
impl
SliceIndex
<
str
> for std::range::
Range
<
usize
>
Source
§
type
Output
=
str
Source
§
impl
SliceIndex
<
str
> for std::range::
RangeFrom
<
usize
>
Source
§
type
Output
=
str
Source
§
impl
SliceIndex
<
str
> for std::range::
RangeInclusive
<
usize
>
Source
§
type
Output
=
str
1.53.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for (
Bound
<
usize
>,
Bound
<
usize
>)
Source
§
type
Output
=
[T]
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
usize
The methods
index
and
index_mut
panic if the index is out of bounds.
Source
§
type
Output
= T
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for std::ops::
Range
<
usize
>
The methods
index
and
index_mut
panic if:
the start of the range is greater than the end of the range or
the end of the range is out of bounds.
Source
§
type
Output
=
[T]
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for std::ops::
RangeFrom
<
usize
>
The methods
index
and
index_mut
panic if the start of the range is out of bounds.
Source
§
type
Output
=
[T]
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
RangeFull
Source
§
type
Output
=
[T]
1.26.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for std::ops::
RangeInclusive
<
usize
>
The methods
index
and
index_mut
panic if:
the end of the range is
usize::MAX
or
the start of the range is greater than the end of the range or
the end of the range is out of bounds.
Source
§
type
Output
=
[T]
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
RangeTo
<
usize
>
The methods
index
and
index_mut
panic if the end of the range is out of bounds.
Source
§
type
Output
=
[T]
1.26.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
RangeToInclusive
<
usize
>
The methods
index
and
index_mut
panic if the end of the range is out of bounds.
Source
§
type
Output
=
[T]
Source
§
impl<T>
SliceIndex
<
[T]
> for std::range::
Range
<
usize
>
Source
§
type
Output
=
[T]
Source
§
impl<T>
SliceIndex
<
[T]
> for std::range::
RangeFrom
<
usize
>
Source
§
type
Output
=
[T]
Source
§
impl<T>
SliceIndex
<
[T]
> for std::range::
RangeInclusive
<
usize
>
Source
§
type
Output
=
[T]