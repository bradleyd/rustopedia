IoSliceMut in std::io - Rust
std
::
io
Struct
IoSliceMut
Copy item path
1.36.0
·
Source
pub struct IoSliceMut<'a>(
/* private fields */
);
Expand description
A buffer type used with
Read::read_vectored
.
It is semantically a wrapper around a
&mut [u8]
, but is guaranteed to be
ABI compatible with the
iovec
type on Unix platforms and
WSABUF
on
Windows.
Implementations
§
Source
§
impl<'a>
IoSliceMut
<'a>
1.36.0
·
Source
pub fn
new
(buf: &'a mut [
u8
]) ->
IoSliceMut
<'a>
Creates a new
IoSliceMut
wrapping a byte slice.
§
Panics
Panics on Windows if the slice is larger than 4GB.
1.81.0
·
Source
pub fn
advance
(&mut self, n:
usize
)
Advance the internal cursor of the slice.
Also see
IoSliceMut::advance_slices
to advance the cursors of
multiple buffers.
§
Panics
Panics when trying to advance beyond the end of the slice.
§
Examples
use
std::io::IoSliceMut;
use
std::ops::Deref;
let
mut
data = [
1
;
8
];
let
mut
buf = IoSliceMut::new(
&mut
data);
// Mark 3 bytes as read.
buf.advance(
3
);
assert_eq!
(buf.deref(), [
1
;
5
].as_ref());
1.81.0
·
Source
pub fn
advance_slices
(bufs: &mut &mut [
IoSliceMut
<'a>], n:
usize
)
Advance a slice of slices.
Shrinks the slice to remove any
IoSliceMut
s that are fully advanced over.
If the cursor ends up in the middle of an
IoSliceMut
, it is modified
to start at that cursor.
For example, if we have a slice of two 8-byte
IoSliceMut
s, and we advance by 10 bytes,
the result will only include the second
IoSliceMut
, advanced by 2 bytes.
§
Panics
Panics when trying to advance beyond the end of the slices.
§
Examples
use
std::io::IoSliceMut;
use
std::ops::Deref;
let
mut
buf1 = [
1
;
8
];
let
mut
buf2 = [
2
;
16
];
let
mut
buf3 = [
3
;
8
];
let
mut
bufs =
&mut
[
    IoSliceMut::new(
&mut
buf1),
    IoSliceMut::new(
&mut
buf2),
    IoSliceMut::new(
&mut
buf3),
][..];
// Mark 10 bytes as read.
IoSliceMut::advance_slices(
&mut
bufs,
10
);
assert_eq!
(bufs[
0
].deref(), [
2
;
14
].as_ref());
assert_eq!
(bufs[
1
].deref(), [
3
;
8
].as_ref());
Source
pub const fn
into_slice
(self) -> &'a mut [
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
io_slice_as_bytes
#132818
)
Get the underlying bytes as a mutable slice with the original lifetime.
§
Examples
#![feature(io_slice_as_bytes)]
use
std::io::IoSliceMut;
let
mut
data =
*
b"abcdef"
;
let
io_slice = IoSliceMut::new(
&mut
data);
io_slice.into_slice()[
0
] =
b'A'
;
assert_eq!
(
&
data,
b"Abcdef"
);
Methods from
Deref
<Target = [
u8
]>
§
1.23.0
·
Source
pub fn
is_ascii
(&self) ->
bool
Checks if all bytes in this slice are within the ASCII range.
Source
pub fn
as_ascii
(&self) ->
Option
<&[
AsciiChar
]>
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
If this slice
is_ascii
, returns it as a slice of
ASCII characters
, otherwise returns
None
.
Source
pub unsafe fn
as_ascii_unchecked
(&self) -> &[
AsciiChar
]
🔬
This is a nightly-only experimental API. (
ascii_char
#110998
)
Converts this slice of bytes into a slice of ASCII characters,
without checking whether they’re valid.
§
Safety
Every byte in the slice must be in
0..=127
, or else this is UB.
1.23.0
·
Source
pub fn
eq_ignore_ascii_case
(&self, other: &[
u8
]) ->
bool
Checks that two slices are an ASCII case-insensitive match.
Same as
to_ascii_lowercase(a) == to_ascii_lowercase(b)
,
but without allocating and copying temporaries.
1.23.0
·
Source
pub fn
make_ascii_uppercase
(&mut self)
Converts this slice to its ASCII upper case equivalent in-place.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To return a new uppercased value without modifying the existing one, use
to_ascii_uppercase
.
1.23.0
·
Source
pub fn
make_ascii_lowercase
(&mut self)
Converts this slice to its ASCII lower case equivalent in-place.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To return a new lowercased value without modifying the existing one, use
to_ascii_lowercase
.
1.60.0
·
Source
pub fn
escape_ascii
(&self) ->
EscapeAscii
<'_>
ⓘ
Returns an iterator that produces an escaped version of this slice,
treating it as an ASCII string.
§
Examples
let
s =
b"0\t\r\n'\"\\\x9d"
;
let
escaped = s.escape_ascii().to_string();
assert_eq!
(escaped,
"0\\t\\r\\n\\'\\\"\\\\\\x9d"
);
1.80.0
·
Source
pub fn
trim_ascii_start
(&self) -> &[
u8
]
ⓘ
Returns a byte slice with leading ASCII whitespace bytes removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
b" \t hello world\n"
.trim_ascii_start(),
b"hello world\n"
);
assert_eq!
(
b"  "
.trim_ascii_start(),
b""
);
assert_eq!
(
b""
.trim_ascii_start(),
b""
);
1.80.0
·
Source
pub fn
trim_ascii_end
(&self) -> &[
u8
]
ⓘ
Returns a byte slice with trailing ASCII whitespace bytes removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
b"\r hello world\n "
.trim_ascii_end(),
b"\r hello world"
);
assert_eq!
(
b"  "
.trim_ascii_end(),
b""
);
assert_eq!
(
b""
.trim_ascii_end(),
b""
);
1.80.0
·
Source
pub fn
trim_ascii
(&self) -> &[
u8
]
ⓘ
Returns a byte slice with leading and trailing ASCII whitespace bytes
removed.
‘Whitespace’ refers to the definition used by
u8::is_ascii_whitespace
.
§
Examples
assert_eq!
(
b"\r hello world\n "
.trim_ascii(),
b"hello world"
);
assert_eq!
(
b"  "
.trim_ascii(),
b""
);
assert_eq!
(
b""
.trim_ascii(),
b""
);
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the slice.
§
Examples
let
a = [
1
,
2
,
3
];
assert_eq!
(a.len(),
3
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the slice has a length of 0.
§
Examples
let
a = [
1
,
2
,
3
];
assert!
(!a.is_empty());
let
b:
&
[i32] =
&
[];
assert!
(b.is_empty());
1.0.0
·
Source
pub fn
first
(&self) ->
Option
<
&T
>
Returns the first element of the slice, or
None
if it is empty.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
10
), v.first());
let
w:
&
[i32] =
&
[];
assert_eq!
(
None
, w.first());
1.0.0
·
Source
pub fn
first_mut
(&mut self) ->
Option
<
&mut T
>
Returns a mutable reference to the first element of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(first) = x.first_mut() {
*
first =
5
;
}
assert_eq!
(x,
&
[
5
,
1
,
2
]);
let
y:
&mut
[i32] =
&mut
[];
assert_eq!
(
None
, y.first_mut());
1.5.0
·
Source
pub fn
split_first
(&self) ->
Option
<(
&T
, &
[T]
)>
Returns the first and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first() {
assert_eq!
(first,
&
0
);
assert_eq!
(elements,
&
[
1
,
2
]);
}
1.5.0
·
Source
pub fn
split_first_mut
(&mut self) ->
Option
<(
&mut T
, &mut
[T]
)>
Returns the first and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_mut() {
*
first =
3
;
    elements[
0
] =
4
;
    elements[
1
] =
5
;
}
assert_eq!
(x,
&
[
3
,
4
,
5
]);
1.5.0
·
Source
pub fn
split_last
(&self) ->
Option
<(
&T
, &
[T]
)>
Returns the last and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((last, elements)) = x.split_last() {
assert_eq!
(last,
&
2
);
assert_eq!
(elements,
&
[
0
,
1
]);
}
1.5.0
·
Source
pub fn
split_last_mut
(&mut self) ->
Option
<(
&mut T
, &mut
[T]
)>
Returns the last and all the rest of the elements of the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((last, elements)) = x.split_last_mut() {
*
last =
3
;
    elements[
0
] =
4
;
    elements[
1
] =
5
;
}
assert_eq!
(x,
&
[
4
,
5
,
3
]);
1.0.0
·
Source
pub fn
last
(&self) ->
Option
<
&T
>
Returns the last element of the slice, or
None
if it is empty.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
30
), v.last());
let
w:
&
[i32] =
&
[];
assert_eq!
(
None
, w.last());
1.0.0
·
Source
pub fn
last_mut
(&mut self) ->
Option
<
&mut T
>
Returns a mutable reference to the last item in the slice, or
None
if it is empty.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(last) = x.last_mut() {
*
last =
10
;
}
assert_eq!
(x,
&
[
0
,
1
,
10
]);
let
y:
&mut
[i32] =
&mut
[];
assert_eq!
(
None
, y.last_mut());
1.77.0
·
Source
pub fn
first_chunk
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
Returns an array reference to the first
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
u = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
[
10
,
40
]), u.first_chunk::<
2
>());
let
v:
&
[i32] =
&
[
10
];
assert_eq!
(
None
, v.first_chunk::<
2
>());
let
w:
&
[i32] =
&
[];
assert_eq!
(
Some
(
&
[]), w.first_chunk::<
0
>());
1.77.0
·
Source
pub fn
first_chunk_mut
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
Returns a mutable array reference to the first
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(first) = x.first_chunk_mut::<
2
>() {
    first[
0
] =
5
;
    first[
1
] =
4
;
}
assert_eq!
(x,
&
[
5
,
4
,
2
]);
assert_eq!
(
None
, x.first_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
split_first_chunk
<const N:
usize
>(&self) ->
Option
<(&
[T; N]
, &
[T]
)>
Returns an array reference to the first
N
items in the slice and the remaining slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_chunk::<
2
>() {
assert_eq!
(first,
&
[
0
,
1
]);
assert_eq!
(elements,
&
[
2
]);
}
assert_eq!
(
None
, x.split_first_chunk::<
4
>());
1.77.0
·
Source
pub fn
split_first_chunk_mut
<const N:
usize
>(
    &mut self,
) ->
Option
<(&mut
[T; N]
, &mut
[T]
)>
Returns a mutable array reference to the first
N
items in the slice and the remaining
slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((first, elements)) = x.split_first_chunk_mut::<
2
>() {
    first[
0
] =
3
;
    first[
1
] =
4
;
    elements[
0
] =
5
;
}
assert_eq!
(x,
&
[
3
,
4
,
5
]);
assert_eq!
(
None
, x.split_first_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
split_last_chunk
<const N:
usize
>(&self) ->
Option
<(&
[T]
, &
[T; N]
)>
Returns an array reference to the last
N
items in the slice and the remaining slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&
[
0
,
1
,
2
];
if let
Some
((elements, last)) = x.split_last_chunk::<
2
>() {
assert_eq!
(elements,
&
[
0
]);
assert_eq!
(last,
&
[
1
,
2
]);
}
assert_eq!
(
None
, x.split_last_chunk::<
4
>());
1.77.0
·
Source
pub fn
split_last_chunk_mut
<const N:
usize
>(
    &mut self,
) ->
Option
<(&mut
[T]
, &mut
[T; N]
)>
Returns a mutable array reference to the last
N
items in the slice and the remaining
slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
((elements, last)) = x.split_last_chunk_mut::<
2
>() {
    last[
0
] =
3
;
    last[
1
] =
4
;
    elements[
0
] =
5
;
}
assert_eq!
(x,
&
[
5
,
3
,
4
]);
assert_eq!
(
None
, x.split_last_chunk_mut::<
4
>());
1.77.0
·
Source
pub fn
last_chunk
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
Returns an array reference to the last
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
u = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
[
40
,
30
]), u.last_chunk::<
2
>());
let
v:
&
[i32] =
&
[
10
];
assert_eq!
(
None
, v.last_chunk::<
2
>());
let
w:
&
[i32] =
&
[];
assert_eq!
(
Some
(
&
[]), w.last_chunk::<
0
>());
1.77.0
·
Source
pub fn
last_chunk_mut
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
Returns a mutable array reference to the last
N
items in the slice.
If the slice is not at least
N
in length, this will return
None
.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(last) = x.last_chunk_mut::<
2
>() {
    last[
0
] =
10
;
    last[
1
] =
20
;
}
assert_eq!
(x,
&
[
0
,
10
,
20
]);
assert_eq!
(
None
, x.last_chunk_mut::<
4
>());
1.0.0
·
Source
pub fn
get
<I>(&self, index: I) ->
Option
<&<I as
SliceIndex
<
[T]
>>::
Output
>
where
    I:
SliceIndex
<
[T]
>,
Returns a reference to an element or subslice depending on the type of
index.
If given a position, returns a reference to the element at that
position or
None
if out of bounds.
If given a range, returns the subslice corresponding to that range,
or
None
if out of bounds.
§
Examples
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
&
40
), v.get(
1
));
assert_eq!
(
Some
(
&
[
10
,
40
][..]), v.get(
0
..
2
));
assert_eq!
(
None
, v.get(
3
));
assert_eq!
(
None
, v.get(
0
..
4
));
1.0.0
·
Source
pub fn
get_mut
<I>(
    &mut self,
    index: I,
) ->
Option
<&mut <I as
SliceIndex
<
[T]
>>::
Output
>
where
    I:
SliceIndex
<
[T]
>,
Returns a mutable reference to an element or subslice depending on the
type of index (see
get
) or
None
if the index is out of bounds.
§
Examples
let
x =
&mut
[
0
,
1
,
2
];
if let
Some
(elem) = x.get_mut(
1
) {
*
elem =
42
;
}
assert_eq!
(x,
&
[
0
,
42
,
2
]);
1.0.0
·
Source
pub unsafe fn
get_unchecked
<I>(
    &self,
    index: I,
) -> &<I as
SliceIndex
<
[T]
>>::
Output
where
    I:
SliceIndex
<
[T]
>,
Returns a reference to an element or subslice, without doing bounds
checking.
For a safe alternative see
get
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used.
You can think of this like
.get(index).unwrap_unchecked()
.  It’s UB
to call
.get_unchecked(len)
, even if you immediately convert to a
pointer.  And it’s UB to call
.get_unchecked(..len + 1)
,
.get_unchecked(..=len)
, or similar.
§
Examples
let
x =
&
[
1
,
2
,
4
];
unsafe
{
assert_eq!
(x.get_unchecked(
1
),
&
2
);
}
1.0.0
·
Source
pub unsafe fn
get_unchecked_mut
<I>(
    &mut self,
    index: I,
) -> &mut <I as
SliceIndex
<
[T]
>>::
Output
where
    I:
SliceIndex
<
[T]
>,
Returns a mutable reference to an element or subslice, without doing
bounds checking.
For a safe alternative see
get_mut
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used.
You can think of this like
.get_mut(index).unwrap_unchecked()
.  It’s
UB to call
.get_unchecked_mut(len)
, even if you immediately convert
to a pointer.  And it’s UB to call
.get_unchecked_mut(..len + 1)
,
.get_unchecked_mut(..=len)
, or similar.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
unsafe
{
let
elem = x.get_unchecked_mut(
1
);
*
elem =
13
;
}
assert_eq!
(x,
&
[
1
,
13
,
4
]);
1.0.0
·
Source
pub fn
as_ptr
(&self) ->
*const T
Returns a raw pointer to the slice’s buffer.
The caller must ensure that the slice outlives the pointer this
function returns, or else it will end up dangling.
The caller must also ensure that the memory the pointer (non-transitively) points to
is never written to (except inside an
UnsafeCell
) using this pointer or any pointer
derived from it. If you need to mutate the contents of the slice, use
as_mut_ptr
.
Modifying the container referenced by this slice may cause its buffer
to be reallocated, which would also make any pointers to it invalid.
§
Examples
let
x =
&
[
1
,
2
,
4
];
let
x_ptr = x.as_ptr();
unsafe
{
for
i
in
0
..x.len() {
assert_eq!
(x.get_unchecked(i),
&*
x_ptr.add(i));
    }
}
1.0.0
·
Source
pub fn
as_mut_ptr
(&mut self) ->
*mut T
Returns an unsafe mutable pointer to the slice’s buffer.
The caller must ensure that the slice outlives the pointer this
function returns, or else it will end up dangling.
Modifying the container referenced by this slice may cause its buffer
to be reallocated, which would also make any pointers to it invalid.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
let
x_ptr = x.as_mut_ptr();
unsafe
{
for
i
in
0
..x.len() {
*
x_ptr.add(i) +=
2
;
    }
}
assert_eq!
(x,
&
[
3
,
4
,
6
]);
1.48.0
·
Source
pub fn
as_ptr_range
(&self) ->
Range
<
*const T
>
ⓘ
Returns the two raw pointers spanning the slice.
The returned range is half-open, which means that the end pointer
points
one past
the last element of the slice. This way, an empty
slice is represented by two equal pointers, and the difference between
the two pointers represents the size of the slice.
See
as_ptr
for warnings on using these pointers. The end pointer
requires extra caution, as it does not point to a valid element in the
slice.
This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.
It can also be useful to check if a pointer to an element refers to an
element of this slice:
let
a = [
1
,
2
,
3
];
let
x =
&
a[
1
]
as
*const
_
;
let
y =
&
5
as
*const
_
;
assert!
(a.as_ptr_range().contains(
&
x));
assert!
(!a.as_ptr_range().contains(
&
y));
1.48.0
·
Source
pub fn
as_mut_ptr_range
(&mut self) ->
Range
<
*mut T
>
ⓘ
Returns the two unsafe mutable pointers spanning the slice.
The returned range is half-open, which means that the end pointer
points
one past
the last element of the slice. This way, an empty
slice is represented by two equal pointers, and the difference between
the two pointers represents the size of the slice.
See
as_mut_ptr
for warnings on using these pointers. The end
pointer requires extra caution, as it does not point to a valid element
in the slice.
This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.
Source
pub fn
as_array
<const N:
usize
>(&self) ->
Option
<&
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a reference to the underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
Source
pub fn
as_mut_array
<const N:
usize
>(&mut self) ->
Option
<&mut
[T; N]
>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Gets a mutable reference to the slice’s underlying array.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
1.0.0
·
Source
pub fn
swap
(&mut self, a:
usize
, b:
usize
)
Swaps two elements in the slice.
If
a
equals to
b
, it’s guaranteed that elements won’t change value.
§
Arguments
a - The index of the first element
b - The index of the second element
§
Panics
Panics if
a
or
b
are out of bounds.
§
Examples
let
mut
v = [
"a"
,
"b"
,
"c"
,
"d"
,
"e"
];
v.swap(
2
,
4
);
assert!
(v == [
"a"
,
"b"
,
"e"
,
"d"
,
"c"
]);
Source
pub unsafe fn
swap_unchecked
(&mut self, a:
usize
, b:
usize
)
🔬
This is a nightly-only experimental API. (
slice_swap_unchecked
#88539
)
Swaps two elements in the slice, without doing bounds checking.
For a safe alternative see
swap
.
§
Arguments
a - The index of the first element
b - The index of the second element
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
.
The caller has to ensure that
a < self.len()
and
b < self.len()
.
§
Examples
#![feature(slice_swap_unchecked)]
let
mut
v = [
"a"
,
"b"
,
"c"
,
"d"
];
// SAFETY: we know that 1 and 3 are both indices of the slice
unsafe
{ v.swap_unchecked(
1
,
3
) };
assert!
(v == [
"a"
,
"d"
,
"c"
,
"b"
]);
1.0.0
·
Source
pub fn
reverse
(&mut self)
Reverses the order of elements in the slice, in place.
§
Examples
let
mut
v = [
1
,
2
,
3
];
v.reverse();
assert!
(v == [
3
,
2
,
1
]);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns an iterator over the slice.
The iterator yields all items from start to end.
§
Examples
let
x =
&
[
1
,
2
,
4
];
let
mut
iterator = x.iter();
assert_eq!
(iterator.next(),
Some
(
&
1
));
assert_eq!
(iterator.next(),
Some
(
&
2
));
assert_eq!
(iterator.next(),
Some
(
&
4
));
assert_eq!
(iterator.next(),
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
Returns an iterator that allows modifying each value.
The iterator yields all items from start to end.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
for
elem
in
x.iter_mut() {
*
elem +=
2
;
}
assert_eq!
(x,
&
[
3
,
4
,
6
]);
1.0.0
·
Source
pub fn
windows
(&self, size:
usize
) ->
Windows
<'_, T>
ⓘ
Returns an iterator over all contiguous windows of length
size
. The windows overlap. If the slice is shorter than
size
, the iterator returns no values.
§
Panics
Panics if
size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.windows(
3
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
,
'r'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
,
'e'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
,
'm'
]);
assert!
(iter.next().is_none());
If the slice is shorter than
size
:
let
slice = [
'f'
,
'o'
,
'o'
];
let
mut
iter = slice.windows(
4
);
assert!
(iter.next().is_none());
Because the
Iterator
trait cannot represent the required lifetimes,
there is no
windows_mut
analog to
windows
;
[0,1,2].windows_mut(2).collect()
would violate
the rules of references
(though a
LendingIterator
analog is possible). You can sometimes use
Cell::as_slice_of_cells
in
conjunction with
windows
instead:
use
std::cell::Cell;
let
mut
array = [
'R'
,
'u'
,
's'
,
't'
,
' '
,
'2'
,
'0'
,
'1'
,
'5'
];
let
slice =
&mut
array[..];
let
slice_of_cells:
&
[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
for
w
in
slice_of_cells.windows(
3
) {
    Cell::swap(
&
w[
0
],
&
w[
2
]);
}
assert_eq!
(array, [
's'
,
't'
,
' '
,
'2'
,
'0'
,
'1'
,
'5'
,
'u'
,
'R'
]);
1.0.0
·
Source
pub fn
chunks
(&self, chunk_size:
usize
) ->
Chunks
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last chunk will not have length
chunk_size
.
See
chunks_exact
for a variant of this iterator that returns chunks of always exactly
chunk_size
elements, and
rchunks
for the same iterator but starting at the end of the
slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.chunks(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'm'
]);
assert!
(iter.next().is_none());
1.0.0
·
Source
pub fn
chunks_mut
(&mut self, chunk_size:
usize
) ->
ChunksMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last chunk will not have length
chunk_size
.
See
chunks_exact_mut
for a variant of this iterator that returns chunks of always
exactly
chunk_size
elements, and
rchunks_mut
for the same iterator but starting at
the end of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.chunks_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
3
]);
1.31.0
·
Source
pub fn
chunks_exact
(&self, chunk_size:
usize
) ->
ChunksExact
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last up to
chunk_size-1
elements will be omitted and can be retrieved
from the
remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks
.
See
chunks
for a variant of this iterator that also returns the remainder as a smaller
chunk, and
rchunks_exact
for the same iterator but starting at the end of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.chunks_exact(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'm'
]);
1.31.0
·
Source
pub fn
chunks_exact_mut
(&mut self, chunk_size:
usize
) ->
ChunksExactMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last up to
chunk_size-1
elements will be omitted and can be
retrieved from the
into_remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks_mut
.
See
chunks_mut
for a variant of this iterator that also returns the remainder as a
smaller chunk, and
rchunks_exact_mut
for the same iterator but starting at the end of
the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.chunks_exact_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
0
]);
Source
pub unsafe fn
as_chunks_unchecked
<const N:
usize
>(&self) -> &[
[T; N]
]
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
assuming that there’s no remainder.
§
Safety
This may only be called when
The slice splits exactly into
N
-element chunks (aka
self.len() % N == 0
).
N != 0
.
§
Examples
#![feature(slice_as_chunks)]
let
slice:
&
[char] =
&
[
'l'
,
'o'
,
'r'
,
'e'
,
'm'
,
'!'
];
let
chunks:
&
[[char;
1
]] =
// SAFETY: 1-element chunks never have remainder
unsafe
{ slice.as_chunks_unchecked() };
assert_eq!
(chunks,
&
[[
'l'
], [
'o'
], [
'r'
], [
'e'
], [
'm'
], [
'!'
]]);
let
chunks:
&
[[char;
3
]] =
// SAFETY: The slice length (6) is a multiple of 3
unsafe
{ slice.as_chunks_unchecked() };
assert_eq!
(chunks,
&
[[
'l'
,
'o'
,
'r'
], [
'e'
,
'm'
,
'!'
]]);
// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed
Source
pub fn
as_chunks
<const N:
usize
>(&self) -> (&[
[T; N]
], &
[T]
)
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the beginning of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
(chunks, remainder) = slice.as_chunks();
assert_eq!
(chunks,
&
[[
'l'
,
'o'
], [
'r'
,
'e'
]]);
assert_eq!
(remainder,
&
[
'm'
]);
If you expect the slice to be an exact multiple, you can combine
let
-
else
with an empty slice pattern:
#![feature(slice_as_chunks)]
let
slice = [
'R'
,
'u'
,
's'
,
't'
];
let
(chunks, []) = slice.as_chunks::<
2
>()
else
{
panic!
(
"slice didn't have even length"
)
};
assert_eq!
(chunks,
&
[[
'R'
,
'u'
], [
's'
,
't'
]]);
Source
pub fn
as_rchunks
<const N:
usize
>(&self) -> (&
[T]
, &[
[T; N]
])
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the end of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
(remainder, chunks) = slice.as_rchunks();
assert_eq!
(remainder,
&
[
'l'
]);
assert_eq!
(chunks,
&
[[
'o'
,
'r'
], [
'e'
,
'm'
]]);
Source
pub fn
array_chunks
<const N:
usize
>(&self) ->
ArrayChunks
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_chunks
#74985
)
Returns an iterator over
N
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are array references and do not overlap. If
N
does not divide the
length of the slice, then the last up to
N-1
elements will be omitted and can be
retrieved from the
remainder
function of the iterator.
This method is the const generic equivalent of
chunks_exact
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_chunks)]
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.array_chunks();
assert_eq!
(iter.next().unwrap(),
&
[
'l'
,
'o'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'r'
,
'e'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'm'
]);
Source
pub unsafe fn
as_chunks_unchecked_mut
<const N:
usize
>(
    &mut self,
) -> &mut [
[T; N]
]
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
assuming that there’s no remainder.
§
Safety
This may only be called when
The slice splits exactly into
N
-element chunks (aka
self.len() % N == 0
).
N != 0
.
§
Examples
#![feature(slice_as_chunks)]
let
slice:
&mut
[char] =
&mut
[
'l'
,
'o'
,
'r'
,
'e'
,
'm'
,
'!'
];
let
chunks:
&mut
[[char;
1
]] =
// SAFETY: 1-element chunks never have remainder
unsafe
{ slice.as_chunks_unchecked_mut() };
chunks[
0
] = [
'L'
];
assert_eq!
(chunks,
&
[[
'L'
], [
'o'
], [
'r'
], [
'e'
], [
'm'
], [
'!'
]]);
let
chunks:
&mut
[[char;
3
]] =
// SAFETY: The slice length (6) is a multiple of 3
unsafe
{ slice.as_chunks_unchecked_mut() };
chunks[
1
] = [
'a'
,
'x'
,
'?'
];
assert_eq!
(slice,
&
[
'L'
,
'o'
,
'r'
,
'a'
,
'x'
,
'?'
]);
// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked_mut() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked_mut() // Zero-length chunks are never allowed
Source
pub fn
as_chunks_mut
<const N:
usize
>(&mut self) -> (&mut [
[T; N]
], &mut
[T]
)
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the beginning of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
let
(chunks, remainder) = v.as_chunks_mut();
remainder[
0
] =
9
;
for
chunk
in
chunks {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
9
]);
Source
pub fn
as_rchunks_mut
<const N:
usize
>(&mut self) -> (&mut
[T]
, &mut [
[T; N]
])
🔬
This is a nightly-only experimental API. (
slice_as_chunks
#74985
)
Splits the slice into a slice of
N
-element arrays,
starting at the end of the slice,
and a remainder slice with length strictly less than
N
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(slice_as_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
let
(remainder, chunks) = v.as_rchunks_mut();
remainder[
0
] =
9
;
for
chunk
in
chunks {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
9
,
1
,
1
,
2
,
2
]);
Source
pub fn
array_chunks_mut
<const N:
usize
>(&mut self) ->
ArrayChunksMut
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_chunks
#74985
)
Returns an iterator over
N
elements of the slice at a time, starting at the
beginning of the slice.
The chunks are mutable array references and do not overlap. If
N
does not divide
the length of the slice, then the last up to
N-1
elements will be omitted and
can be retrieved from the
into_remainder
function of the iterator.
This method is the const generic equivalent of
chunks_exact_mut
.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_chunks)]
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.array_chunks_mut() {
*
chunk = [count;
2
];
    count +=
1
;
}
assert_eq!
(v,
&
[
1
,
1
,
2
,
2
,
0
]);
Source
pub fn
array_windows
<const N:
usize
>(&self) ->
ArrayWindows
<'_, T, N>
ⓘ
🔬
This is a nightly-only experimental API. (
array_windows
#75027
)
Returns an iterator over overlapping windows of
N
elements of a slice,
starting at the beginning of the slice.
This is the const generic equivalent of
windows
.
If
N
is greater than the size of the slice, it will return no windows.
§
Panics
Panics if
N
is zero. This check will most probably get changed to a compile time
error before this method gets stabilized.
§
Examples
#![feature(array_windows)]
let
slice = [
0
,
1
,
2
,
3
];
let
mut
iter = slice.array_windows();
assert_eq!
(iter.next().unwrap(),
&
[
0
,
1
]);
assert_eq!
(iter.next().unwrap(),
&
[
1
,
2
]);
assert_eq!
(iter.next().unwrap(),
&
[
2
,
3
]);
assert!
(iter.next().is_none());
1.31.0
·
Source
pub fn
rchunks
(&self, chunk_size:
usize
) ->
RChunks
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last chunk will not have length
chunk_size
.
See
rchunks_exact
for a variant of this iterator that returns chunks of always exactly
chunk_size
elements, and
chunks
for the same iterator but starting at the beginning
of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.rchunks(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'e'
,
'm'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'l'
]);
assert!
(iter.next().is_none());
1.31.0
·
Source
pub fn
rchunks_mut
(&mut self, chunk_size:
usize
) ->
RChunksMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last chunk will not have length
chunk_size
.
See
rchunks_exact_mut
for a variant of this iterator that returns chunks of always
exactly
chunk_size
elements, and
chunks_mut
for the same iterator but starting at the
beginning of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.rchunks_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
3
,
2
,
2
,
1
,
1
]);
1.31.0
·
Source
pub fn
rchunks_exact
(&self, chunk_size:
usize
) ->
RChunksExact
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the
end of the slice.
The chunks are slices and do not overlap. If
chunk_size
does not divide the length of the
slice, then the last up to
chunk_size-1
elements will be omitted and can be retrieved
from the
remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
rchunks
.
See
rchunks
for a variant of this iterator that also returns the remainder as a smaller
chunk, and
chunks_exact
for the same iterator but starting at the beginning of the
slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
slice = [
'l'
,
'o'
,
'r'
,
'e'
,
'm'
];
let
mut
iter = slice.rchunks_exact(
2
);
assert_eq!
(iter.next().unwrap(),
&
[
'e'
,
'm'
]);
assert_eq!
(iter.next().unwrap(),
&
[
'o'
,
'r'
]);
assert!
(iter.next().is_none());
assert_eq!
(iter.remainder(),
&
[
'l'
]);
1.31.0
·
Source
pub fn
rchunks_exact_mut
(&mut self, chunk_size:
usize
) ->
RChunksExactMut
<'_, T>
ⓘ
Returns an iterator over
chunk_size
elements of the slice at a time, starting at the end
of the slice.
The chunks are mutable slices, and do not overlap. If
chunk_size
does not divide the
length of the slice, then the last up to
chunk_size-1
elements will be omitted and can be
retrieved from the
into_remainder
function of the iterator.
Due to each chunk having exactly
chunk_size
elements, the compiler can often optimize the
resulting code better than in the case of
chunks_mut
.
See
rchunks_mut
for a variant of this iterator that also returns the remainder as a
smaller chunk, and
chunks_exact_mut
for the same iterator but starting at the beginning
of the slice.
§
Panics
Panics if
chunk_size
is zero.
§
Examples
let
v =
&mut
[
0
,
0
,
0
,
0
,
0
];
let
mut
count =
1
;
for
chunk
in
v.rchunks_exact_mut(
2
) {
for
elem
in
chunk.iter_mut() {
*
elem += count;
    }
    count +=
1
;
}
assert_eq!
(v,
&
[
0
,
2
,
2
,
1
,
1
]);
1.77.0
·
Source
pub fn
chunk_by
<F>(&self, pred: F) ->
ChunkBy
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
,
&T
) ->
bool
,
Returns an iterator over the slice producing non-overlapping runs
of elements using the predicate to separate them.
The predicate is called for every pair of consecutive elements,
meaning that it is called on
slice[0]
and
slice[1]
,
followed by
slice[1]
and
slice[2]
, and so on.
§
Examples
let
slice =
&
[
1
,
1
,
1
,
3
,
3
,
2
,
2
,
2
];
let
mut
iter = slice.chunk_by(|a, b| a == b);
assert_eq!
(iter.next(),
Some
(
&
[
1
,
1
,
1
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
3
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
2
,
2
][..]));
assert_eq!
(iter.next(),
None
);
This method can be used to extract the sorted subslices:
let
slice =
&
[
1
,
1
,
2
,
3
,
2
,
3
,
2
,
3
,
4
];
let
mut
iter = slice.chunk_by(|a, b| a <= b);
assert_eq!
(iter.next(),
Some
(
&
[
1
,
1
,
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&
[
2
,
3
,
4
][..]));
assert_eq!
(iter.next(),
None
);
1.77.0
·
Source
pub fn
chunk_by_mut
<F>(&mut self, pred: F) ->
ChunkByMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
,
&T
) ->
bool
,
Returns an iterator over the slice producing non-overlapping mutable
runs of elements using the predicate to separate them.
The predicate is called for every pair of consecutive elements,
meaning that it is called on
slice[0]
and
slice[1]
,
followed by
slice[1]
and
slice[2]
, and so on.
§
Examples
let
slice =
&mut
[
1
,
1
,
1
,
3
,
3
,
2
,
2
,
2
];
let
mut
iter = slice.chunk_by_mut(|a, b| a == b);
assert_eq!
(iter.next(),
Some
(
&mut
[
1
,
1
,
1
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
3
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
2
,
2
][..]));
assert_eq!
(iter.next(),
None
);
This method can be used to extract the sorted subslices:
let
slice =
&mut
[
1
,
1
,
2
,
3
,
2
,
3
,
2
,
3
,
4
];
let
mut
iter = slice.chunk_by_mut(|a, b| a <= b);
assert_eq!
(iter.next(),
Some
(
&mut
[
1
,
1
,
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
3
][..]));
assert_eq!
(iter.next(),
Some
(
&mut
[
2
,
3
,
4
][..]));
assert_eq!
(iter.next(),
None
);
1.0.0
·
Source
pub fn
split_at
(&self, mid:
usize
) -> (&
[T]
, &
[T]
)
Divides one slice into two at an index.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
§
Panics
Panics if
mid > len
.  For a non-panicking alternative see
split_at_checked
.
§
Examples
let
v = [
'a'
,
'b'
,
'c'
];

{
let
(left, right) = v.split_at(
0
);
assert_eq!
(left, []);
assert_eq!
(right, [
'a'
,
'b'
,
'c'
]);
}

{
let
(left, right) = v.split_at(
2
);
assert_eq!
(left, [
'a'
,
'b'
]);
assert_eq!
(right, [
'c'
]);
}

{
let
(left, right) = v.split_at(
3
);
assert_eq!
(left, [
'a'
,
'b'
,
'c'
]);
assert_eq!
(right, []);
}
1.0.0
·
Source
pub fn
split_at_mut
(&mut self, mid:
usize
) -> (&mut
[T]
, &mut
[T]
)
Divides one mutable slice into two at an index.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
§
Panics
Panics if
mid > len
.  For a non-panicking alternative see
split_at_mut_checked
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
let
(left, right) = v.split_at_mut(
2
);
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
left[
1
] =
2
;
right[
1
] =
4
;
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
1.79.0
·
Source
pub unsafe fn
split_at_unchecked
(&self, mid:
usize
) -> (&
[T]
, &
[T]
)
Divides one slice into two at an index, without doing bounds checking.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
For a safe alternative see
split_at
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used. The caller has to ensure that
0 <= mid <= self.len()
.
§
Examples
let
v = [
'a'
,
'b'
,
'c'
];
unsafe
{
let
(left, right) = v.split_at_unchecked(
0
);
assert_eq!
(left, []);
assert_eq!
(right, [
'a'
,
'b'
,
'c'
]);
}
unsafe
{
let
(left, right) = v.split_at_unchecked(
2
);
assert_eq!
(left, [
'a'
,
'b'
]);
assert_eq!
(right, [
'c'
]);
}
unsafe
{
let
(left, right) = v.split_at_unchecked(
3
);
assert_eq!
(left, [
'a'
,
'b'
,
'c'
]);
assert_eq!
(right, []);
}
1.79.0
·
Source
pub unsafe fn
split_at_mut_unchecked
(
    &mut self,
    mid:
usize
,
) -> (&mut
[T]
, &mut
[T]
)
Divides one mutable slice into two at an index, without doing bounds checking.
The first will contain all indices from
[0, mid)
(excluding
the index
mid
itself) and the second will contain all
indices from
[mid, len)
(excluding the index
len
itself).
For a safe alternative see
split_at_mut
.
§
Safety
Calling this method with an out-of-bounds index is
undefined behavior
even if the resulting reference is not used. The caller has to ensure that
0 <= mid <= self.len()
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
// scoped to restrict the lifetime of the borrows
unsafe
{
let
(left, right) = v.split_at_mut_unchecked(
2
);
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
    left[
1
] =
2
;
    right[
1
] =
4
;
}
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
1.80.0
·
Source
pub fn
split_at_checked
(&self, mid:
usize
) ->
Option
<(&
[T]
, &
[T]
)>
Divides one slice into two at an index, returning
None
if the slice is
too short.
If
mid ≤ len
returns a pair of slices where the first will contain all
indices from
[0, mid)
(excluding the index
mid
itself) and the
second will contain all indices from
[mid, len)
(excluding the index
len
itself).
Otherwise, if
mid > len
, returns
None
.
§
Examples
let
v = [
1
, -
2
,
3
, -
4
,
5
, -
6
];

{
let
(left, right) = v.split_at_checked(
0
).unwrap();
assert_eq!
(left, []);
assert_eq!
(right, [
1
, -
2
,
3
, -
4
,
5
, -
6
]);
}

{
let
(left, right) = v.split_at_checked(
2
).unwrap();
assert_eq!
(left, [
1
, -
2
]);
assert_eq!
(right, [
3
, -
4
,
5
, -
6
]);
}

{
let
(left, right) = v.split_at_checked(
6
).unwrap();
assert_eq!
(left, [
1
, -
2
,
3
, -
4
,
5
, -
6
]);
assert_eq!
(right, []);
}
assert_eq!
(
None
, v.split_at_checked(
7
));
1.80.0
·
Source
pub fn
split_at_mut_checked
(
    &mut self,
    mid:
usize
,
) ->
Option
<(&mut
[T]
, &mut
[T]
)>
Divides one mutable slice into two at an index, returning
None
if the
slice is too short.
If
mid ≤ len
returns a pair of slices where the first will contain all
indices from
[0, mid)
(excluding the index
mid
itself) and the
second will contain all indices from
[mid, len)
(excluding the index
len
itself).
Otherwise, if
mid > len
, returns
None
.
§
Examples
let
mut
v = [
1
,
0
,
3
,
0
,
5
,
6
];
if let
Some
((left, right)) = v.split_at_mut_checked(
2
) {
assert_eq!
(left, [
1
,
0
]);
assert_eq!
(right, [
3
,
0
,
5
,
6
]);
    left[
1
] =
2
;
    right[
1
] =
4
;
}
assert_eq!
(v, [
1
,
2
,
3
,
4
,
5
,
6
]);
assert_eq!
(
None
, v.split_at_mut_checked(
7
));
1.0.0
·
Source
pub fn
split
<F>(&self, pred: F) ->
Split
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
. The matched element is not contained in the subslices.
§
Examples
let
slice = [
10
,
40
,
33
,
20
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
If the first element is matched, an empty slice will be the first item
returned by the iterator. Similarly, if the last element in the slice
is matched, an empty slice will be the last item returned by the
iterator:
let
slice = [
10
,
40
,
33
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
]);
assert_eq!
(iter.next().unwrap(),
&
[]);
assert!
(iter.next().is_none());
If two matched elements are directly adjacent, an empty slice will be
present between them:
let
slice = [
10
,
6
,
33
,
20
];
let
mut
iter = slice.split(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
]);
assert_eq!
(iter.next().unwrap(),
&
[]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
1.0.0
·
Source
pub fn
split_mut
<F>(&mut self, pred: F) ->
SplitMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
. The matched element is not contained in the subslices.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.split_mut(|num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(v, [
1
,
40
,
30
,
1
,
60
,
1
]);
1.51.0
·
Source
pub fn
split_inclusive
<F>(&self, pred: F) ->
SplitInclusive
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
. The matched element is contained in the end of the previous
subslice as a terminator.
§
Examples
let
slice = [
10
,
40
,
33
,
20
];
let
mut
iter = slice.split_inclusive(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
,
33
]);
assert_eq!
(iter.next().unwrap(),
&
[
20
]);
assert!
(iter.next().is_none());
If the last element of the slice is matched,
that element will be considered the terminator of the preceding slice.
That slice will be the last item returned by the iterator.
let
slice = [
3
,
10
,
40
,
33
];
let
mut
iter = slice.split_inclusive(|num| num %
3
==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
3
]);
assert_eq!
(iter.next().unwrap(),
&
[
10
,
40
,
33
]);
assert!
(iter.next().is_none());
1.51.0
·
Source
pub fn
split_inclusive_mut
<F>(&mut self, pred: F) ->
SplitInclusiveMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
. The matched element is contained in the previous
subslice as a terminator.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.split_inclusive_mut(|num|
*
num %
3
==
0
) {
let
terminator_idx = group.len()-
1
;
    group[terminator_idx] =
1
;
}
assert_eq!
(v, [
10
,
40
,
1
,
20
,
1
,
1
]);
1.27.0
·
Source
pub fn
rsplit
<F>(&self, pred: F) ->
RSplit
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
, starting at the end of the slice and working backwards.
The matched element is not contained in the subslices.
§
Examples
let
slice = [
11
,
22
,
33
,
0
,
44
,
55
];
let
mut
iter = slice.rsplit(|num|
*
num ==
0
);
assert_eq!
(iter.next().unwrap(),
&
[
44
,
55
]);
assert_eq!
(iter.next().unwrap(),
&
[
11
,
22
,
33
]);
assert_eq!
(iter.next(),
None
);
As with
split()
, if the first or last element is matched, an empty
slice will be the first (or last) item returned by the iterator.
let
v =
&
[
0
,
1
,
1
,
2
,
3
,
5
,
8
];
let
mut
it = v.rsplit(|n|
*
n %
2
==
0
);
assert_eq!
(it.next().unwrap(),
&
[]);
assert_eq!
(it.next().unwrap(),
&
[
3
,
5
]);
assert_eq!
(it.next().unwrap(),
&
[
1
,
1
]);
assert_eq!
(it.next().unwrap(),
&
[]);
assert_eq!
(it.next(),
None
);
1.27.0
·
Source
pub fn
rsplit_mut
<F>(&mut self, pred: F) ->
RSplitMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that
match
pred
, starting at the end of the slice and working
backwards. The matched element is not contained in the subslices.
§
Examples
let
mut
v = [
100
,
400
,
300
,
200
,
600
,
500
];
let
mut
count =
0
;
for
group
in
v.rsplit_mut(|num|
*
num %
3
==
0
) {
    count +=
1
;
    group[
0
] = count;
}
assert_eq!
(v, [
3
,
400
,
300
,
2
,
600
,
1
]);
1.0.0
·
Source
pub fn
splitn
<F>(&self, n:
usize
, pred: F) ->
SplitN
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
, limited to returning at most
n
items. The matched element is
not contained in the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
Print the slice split once by numbers divisible by 3 (i.e.,
[10, 40]
,
[20, 60, 50]
):
let
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.splitn(
2
, |num|
*
num %
3
==
0
) {
println!
(
"{group:?}"
);
}
1.0.0
·
Source
pub fn
splitn_mut
<F>(&mut self, n:
usize
, pred: F) ->
SplitNMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over mutable subslices separated by elements that match
pred
, limited to returning at most
n
items. The matched element is
not contained in the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
let
mut
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.splitn_mut(
2
, |num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(v, [
1
,
40
,
30
,
1
,
60
,
50
]);
1.0.0
·
Source
pub fn
rsplitn
<F>(&self, n:
usize
, pred: F) ->
RSplitN
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
limited to returning at most
n
items. This starts at the end of
the slice and works backwards. The matched element is not contained in
the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
Print the slice split once, starting from the end, by numbers divisible
by 3 (i.e.,
[50]
,
[10, 40, 30, 20]
):
let
v = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
v.rsplitn(
2
, |num|
*
num %
3
==
0
) {
println!
(
"{group:?}"
);
}
1.0.0
·
Source
pub fn
rsplitn_mut
<F>(&mut self, n:
usize
, pred: F) ->
RSplitNMut
<'_, T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
bool
,
Returns an iterator over subslices separated by elements that match
pred
limited to returning at most
n
items. This starts at the end of
the slice and works backwards. The matched element is not contained in
the subslices.
The last element returned, if any, will contain the remainder of the
slice.
§
Examples
let
mut
s = [
10
,
40
,
30
,
20
,
60
,
50
];
for
group
in
s.rsplitn_mut(
2
, |num|
*
num %
3
==
0
) {
    group[
0
] =
1
;
}
assert_eq!
(s, [
1
,
40
,
30
,
20
,
60
,
1
]);
Source
pub fn
split_once
<F>(&self, pred: F) ->
Option
<(&
[T]
, &
[T]
)>
where
    F:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_split_once
#112811
)
Splits the slice on the first element that matches the specified
predicate.
If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returns
None
.
§
Examples
#![feature(slice_split_once)]
let
s = [
1
,
2
,
3
,
2
,
4
];
assert_eq!
(s.split_once(|
&
x| x ==
2
),
Some
((
&
[
1
][..],
&
[
3
,
2
,
4
][..]
)));
assert_eq!
(s.split_once(|
&
x| x ==
0
),
None
);
Source
pub fn
rsplit_once
<F>(&self, pred: F) ->
Option
<(&
[T]
, &
[T]
)>
where
    F:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_split_once
#112811
)
Splits the slice on the last element that matches the specified
predicate.
If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returns
None
.
§
Examples
#![feature(slice_split_once)]
let
s = [
1
,
2
,
3
,
2
,
4
];
assert_eq!
(s.rsplit_once(|
&
x| x ==
2
),
Some
((
&
[
1
,
2
,
3
][..],
&
[
4
][..]
)));
assert_eq!
(s.rsplit_once(|
&
x| x ==
0
),
None
);
1.0.0
·
Source
pub fn
contains
(&self, x:
&T
) ->
bool
where
    T:
PartialEq
,
Returns
true
if the slice contains an element with the given value.
This operation is
O
(
n
).
Note that if you have a sorted slice,
binary_search
may be faster.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.contains(
&
30
));
assert!
(!v.contains(
&
50
));
If you do not have a
&T
, but some other value that you can compare
with one (for example,
String
implements
PartialEq<str>
), you can
use
iter().any
:
let
v = [String::from(
"hello"
), String::from(
"world"
)];
// slice of `String`
assert!
(v.iter().any(|e| e ==
"hello"
));
// search with `&str`
assert!
(!v.iter().any(|e| e ==
"hi"
));
1.0.0
·
Source
pub fn
starts_with
(&self, needle: &
[T]
) ->
bool
where
    T:
PartialEq
,
Returns
true
if
needle
is a prefix of the slice or equal to the slice.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.starts_with(
&
[
10
]));
assert!
(v.starts_with(
&
[
10
,
40
]));
assert!
(v.starts_with(
&
v));
assert!
(!v.starts_with(
&
[
50
]));
assert!
(!v.starts_with(
&
[
10
,
50
]));
Always returns
true
if
needle
is an empty slice:
let
v =
&
[
10
,
40
,
30
];
assert!
(v.starts_with(
&
[]));
let
v:
&
[u8] =
&
[];
assert!
(v.starts_with(
&
[]));
1.0.0
·
Source
pub fn
ends_with
(&self, needle: &
[T]
) ->
bool
where
    T:
PartialEq
,
Returns
true
if
needle
is a suffix of the slice or equal to the slice.
§
Examples
let
v = [
10
,
40
,
30
];
assert!
(v.ends_with(
&
[
30
]));
assert!
(v.ends_with(
&
[
40
,
30
]));
assert!
(v.ends_with(
&
v));
assert!
(!v.ends_with(
&
[
50
]));
assert!
(!v.ends_with(
&
[
50
,
30
]));
Always returns
true
if
needle
is an empty slice:
let
v =
&
[
10
,
40
,
30
];
assert!
(v.ends_with(
&
[]));
let
v:
&
[u8] =
&
[];
assert!
(v.ends_with(
&
[]));
1.51.0
·
Source
pub fn
strip_prefix
<P>(&self, prefix:
&P
) ->
Option
<&
[T]
>
where
    P:
SlicePattern
<Item = T> + ?
Sized
,
    T:
PartialEq
,
Returns a subslice with the prefix removed.
If the slice starts with
prefix
, returns the subslice after the prefix, wrapped in
Some
.
If
prefix
is empty, simply returns the original slice. If
prefix
is equal to the
original slice, returns an empty slice.
If the slice does not start with
prefix
, returns
None
.
§
Examples
let
v =
&
[
10
,
40
,
30
];
assert_eq!
(v.strip_prefix(
&
[
10
]),
Some
(
&
[
40
,
30
][..]));
assert_eq!
(v.strip_prefix(
&
[
10
,
40
]),
Some
(
&
[
30
][..]));
assert_eq!
(v.strip_prefix(
&
[
10
,
40
,
30
]),
Some
(
&
[][..]));
assert_eq!
(v.strip_prefix(
&
[
50
]),
None
);
assert_eq!
(v.strip_prefix(
&
[
10
,
50
]),
None
);
let
prefix :
&
str =
"he"
;
assert_eq!
(
b"hello"
.strip_prefix(prefix.as_bytes()),
Some
(
b"llo"
.as_ref()));
1.51.0
·
Source
pub fn
strip_suffix
<P>(&self, suffix:
&P
) ->
Option
<&
[T]
>
where
    P:
SlicePattern
<Item = T> + ?
Sized
,
    T:
PartialEq
,
Returns a subslice with the suffix removed.
If the slice ends with
suffix
, returns the subslice before the suffix, wrapped in
Some
.
If
suffix
is empty, simply returns the original slice. If
suffix
is equal to the
original slice, returns an empty slice.
If the slice does not end with
suffix
, returns
None
.
§
Examples
let
v =
&
[
10
,
40
,
30
];
assert_eq!
(v.strip_suffix(
&
[
30
]),
Some
(
&
[
10
,
40
][..]));
assert_eq!
(v.strip_suffix(
&
[
40
,
30
]),
Some
(
&
[
10
][..]));
assert_eq!
(v.strip_suffix(
&
[
10
,
40
,
30
]),
Some
(
&
[][..]));
assert_eq!
(v.strip_suffix(
&
[
50
]),
None
);
assert_eq!
(v.strip_suffix(
&
[
50
,
30
]),
None
);
1.0.0
·
Source
pub fn
binary_search
(&self, x:
&T
) ->
Result
<
usize
,
usize
>
where
    T:
Ord
,
Binary searches this slice for a given element.
If the slice is not sorted, the returned result is unspecified and
meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search_by
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
assert_eq!
(s.binary_search(
&
13
),
Ok
(
9
));
assert_eq!
(s.binary_search(
&
4
),
Err
(
7
));
assert_eq!
(s.binary_search(
&
100
),
Err
(
13
));
let
r = s.binary_search(
&
1
);
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
If you want to find that whole
range
of matching items, rather than
an arbitrary matching one, that can be done using
partition_point
:
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
low = s.partition_point(|x| x <
&
1
);
assert_eq!
(low,
1
);
let
high = s.partition_point(|x| x <=
&
1
);
assert_eq!
(high,
5
);
let
r = s.binary_search(
&
1
);
assert!
((low..high).contains(
&
r.unwrap()));
assert!
(s[..low].iter().all(|
&
x| x <
1
));
assert!
(s[low..high].iter().all(|
&
x| x ==
1
));
assert!
(s[high..].iter().all(|
&
x| x >
1
));
// For something not found, the "range" of equal items is empty
assert_eq!
(s.partition_point(|x| x <
&
11
),
9
);
assert_eq!
(s.partition_point(|x| x <=
&
11
),
9
);
assert_eq!
(s.binary_search(
&
11
),
Err
(
9
));
If you want to insert an item to a sorted vector, while maintaining
sort order, consider using
partition_point
:
let
mut
s =
vec!
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
num =
42
;
let
idx = s.partition_point(|
&
x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` will allow `insert`
// to shift less elements.
s.insert(idx, num);
assert_eq!
(s, [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
1.0.0
·
Source
pub fn
binary_search_by
<'a, F>(&'a self, f: F) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) ->
Ordering
,
Binary searches this slice with a comparator function.
The comparator function should return an order code that indicates
whether its argument is
Less
,
Equal
or
Greater
the desired
target.
If the slice is not sorted or if the comparator function does not
implement an order consistent with the sort order of the underlying
slice, the returned result is unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
let
s = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
seek =
13
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Ok
(
9
));
let
seek =
4
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Err
(
7
));
let
seek =
100
;
assert_eq!
(s.binary_search_by(|probe| probe.cmp(
&
seek)),
Err
(
13
));
let
seek =
1
;
let
r = s.binary_search_by(|probe| probe.cmp(
&
seek));
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
1.10.0
·
Source
pub fn
binary_search_by_key
<'a, B, F>(
    &'a self,
    b:
&B
,
    f: F,
) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) -> B,
    B:
Ord
,
Binary searches this slice with a key extraction function.
Assumes that the slice is sorted by the key, for instance with
sort_by_key
using the same key extraction function.
If the slice is not sorted by the key, the returned result is
unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found then
Result::Err
is returned, containing
the index where a matching element could be inserted while maintaining
sorted order.
See also
binary_search
,
binary_search_by
, and
partition_point
.
§
Examples
Looks up a series of four elements in a slice of pairs sorted by
their second elements. The first is found, with a uniquely
determined position; the second and third are not found; the
fourth could match any position in
[1, 4]
.
let
s = [(
0
,
0
), (
2
,
1
), (
4
,
1
), (
5
,
1
), (
3
,
1
),
         (
1
,
2
), (
2
,
3
), (
4
,
5
), (
5
,
8
), (
3
,
13
),
         (
1
,
21
), (
2
,
34
), (
4
,
55
)];
assert_eq!
(s.binary_search_by_key(
&
13
, |
&
(a, b)| b),
Ok
(
9
));
assert_eq!
(s.binary_search_by_key(
&
4
, |
&
(a, b)| b),
Err
(
7
));
assert_eq!
(s.binary_search_by_key(
&
100
, |
&
(a, b)| b),
Err
(
13
));
let
r = s.binary_search_by_key(
&
1
, |
&
(a, b)| b);
assert!
(
match
r {
Ok
(
1
..=
4
) =>
true
,
_
=>
false
, });
1.20.0
·
Source
pub fn
sort_unstable
(&mut self)
where
    T:
Ord
,
Sorts the slice
without
preserving the initial order of equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the implementation of
Ord
for
T
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if the implementation of
Ord
for
T
panics.
Sorting types that only implement
PartialOrd
such as
f32
and
f64
require
additional precautions. For example,
f32::NAN != f32::NAN
, which doesn’t fulfill the
reflexivity requirement of
Ord
. By using an alternative comparison function with
slice::sort_unstable_by
such as
f32::total_cmp
or
f64::total_cmp
that defines a
total order
users can sort slices containing floating-point values. Alternatively, if all
values in the slice are guaranteed to be in a subset for which
PartialOrd::partial_cmp
forms a
total order
, it’s possible to sort the slice with
sort_unstable_by(|a, b| a.partial_cmp(b).unwrap())
.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the implementation of
Ord
for
T
does not implement a
total order
, or if
the
Ord
implementation panics.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];

v.sort_unstable();
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
1.20.0
·
Source
pub fn
sort_unstable_by
<F>(&mut self, compare: F)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Sorts the slice with a comparison function,
without
preserving the initial order of
equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the comparison function
compare
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if
compare
panics.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the
compare
does not implement a
total order
, or if
the
compare
itself panics.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];
v.sort_unstable_by(|a, b| a.cmp(b));
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
// reverse sorting
v.sort_unstable_by(|a, b| b.cmp(a));
assert_eq!
(v, [
4
,
2
,
1
, -
3
, -
5
]);
1.20.0
·
Source
pub fn
sort_unstable_by_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function,
without
preserving the initial order of
equal elements.
This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not
allocate), and
O
(
n
* log(
n
)) worst-case.
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
All original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. Same is true if the implementation of
Ord
for
K
panics.
§
Current implementation
The current implementation is based on
ipnsort
by Lukas Bergdoll and Orson Peters, which
combines the fast average case of quicksort with the fast worst case of heapsort, achieving
linear time on fully sorted and reversed inputs. On inputs with k distinct elements, the
expected time to sort the data is
O
(
n
* log(
k
)).
It is typically faster than stable sorting, except in a few special cases, e.g., when the
slice is partially sorted.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation panics.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
];

v.sort_unstable_by_key(|k| k.abs());
assert_eq!
(v, [
1
,
2
, -
3
,
4
, -
5
]);
1.49.0
·
Source
pub fn
select_nth_unstable
(
    &mut self,
    index:
usize
,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    T:
Ord
,
Reorders the slice such that the element at
index
is at a sort-order position. All
elements before
index
will be
<=
to this value, and all elements after will be
>=
to
it.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple that partitions the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
x <= self[index]
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
x >= self[index]
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, and so always panics on empty slices.
May panic if the implementation of
Ord
for
T
does not implement a
total order
.
§
Examples
let
mut
v = [-
5i32
,
4
,
2
, -
3
,
1
];
// Find the items `<=` to the median, the median itself, and the items `>=` to it.
let
(lesser, median, greater) = v.select_nth_unstable(
2
);
assert!
(lesser == [-
3
, -
5
] || lesser == [-
5
, -
3
]);
assert_eq!
(median,
&mut
1
);
assert!
(greater == [
4
,
2
] || greater == [
2
,
4
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [-
3
, -
5
,
1
,
2
,
4
] ||
        v == [-
5
, -
3
,
1
,
2
,
4
] ||
        v == [-
3
, -
5
,
1
,
4
,
2
] ||
        v == [-
5
, -
3
,
1
,
4
,
2
]);
1.49.0
·
Source
pub fn
select_nth_unstable_by
<F>(
    &mut self,
    index:
usize
,
    compare: F,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Reorders the slice with a comparator function such that the element at
index
is at a
sort-order position. All elements before
index
will be
<=
to this value, and all
elements after will be
>=
to it, according to the comparator function.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple partitioning the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
compare(x, self[index]).is_le()
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
compare(x, self[index]).is_ge()
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, and so always panics on empty slices.
May panic if
compare
does not implement a
total order
.
§
Examples
let
mut
v = [-
5i32
,
4
,
2
, -
3
,
1
];
// Find the items `>=` to the median, the median itself, and the items `<=` to it, by using
// a reversed comparator.
let
(before, median, after) = v.select_nth_unstable_by(
2
, |a, b| b.cmp(a));
assert!
(before == [
4
,
2
] || before == [
2
,
4
]);
assert_eq!
(median,
&mut
1
);
assert!
(after == [-
3
, -
5
] || after == [-
5
, -
3
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [
2
,
4
,
1
, -
5
, -
3
] ||
        v == [
2
,
4
,
1
, -
3
, -
5
] ||
        v == [
4
,
2
,
1
, -
5
, -
3
] ||
        v == [
4
,
2
,
1
, -
3
, -
5
]);
1.49.0
·
Source
pub fn
select_nth_unstable_by_key
<K, F>(
    &mut self,
    index:
usize
,
    f: F,
) -> (&mut
[T]
,
&mut T
, &mut
[T]
)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Reorders the slice with a key extraction function such that the element at
index
is at a
sort-order position. All elements before
index
will have keys
<=
to the key at
index
,
and all elements after will have keys
>=
to it.
This reordering is unstable (i.e. any element that compares equal to the nth element may end
up at that position), in-place (i.e.  does not allocate), and runs in
O
(
n
) time. This
function is also known as “kth element” in other libraries.
Returns a triple partitioning the reordered slice:
The unsorted subslice before
index
, whose elements all satisfy
f(x) <= f(self[index])
.
The element at
index
.
The unsorted subslice after
index
, whose elements all satisfy
f(x) >= f(self[index])
.
§
Current implementation
The current algorithm is an introselect implementation based on
ipnsort
by Lukas Bergdoll
and Orson Peters, which is also the basis for
sort_unstable
. The fallback algorithm is
Median of Medians using Tukey’s Ninther for pivot selection, which guarantees linear runtime
for all inputs.
§
Panics
Panics when
index >= len()
, meaning it always panics on empty slices.
May panic if
K: Ord
does not implement a total order.
§
Examples
let
mut
v = [-
5i32
,
4
,
1
, -
3
,
2
];
// Find the items `<=` to the absolute median, the absolute median itself, and the items
// `>=` to it.
let
(lesser, median, greater) = v.select_nth_unstable_by_key(
2
, |a| a.abs());
assert!
(lesser == [
1
,
2
] || lesser == [
2
,
1
]);
assert_eq!
(median,
&mut
-
3
);
assert!
(greater == [
4
, -
5
] || greater == [-
5
,
4
]);
// We are only guaranteed the slice will be one of the following, based on the way we sort
// about the specified index.
assert!
(v == [
1
,
2
, -
3
,
4
, -
5
] ||
        v == [
1
,
2
, -
3
, -
5
,
4
] ||
        v == [
2
,
1
, -
3
,
4
, -
5
] ||
        v == [
2
,
1
, -
3
, -
5
,
4
]);
Source
pub fn
partition_dedup
(&mut self) -> (&mut
[T]
, &mut
[T]
)
where
    T:
PartialEq
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all consecutive repeated elements to the end of the slice according to the
PartialEq
trait implementation.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
1
,
2
,
2
,
3
,
3
,
2
,
1
,
1
];
let
(dedup, duplicates) = slice.partition_dedup();
assert_eq!
(dedup, [
1
,
2
,
3
,
2
,
1
]);
assert_eq!
(duplicates, [
2
,
3
,
1
]);
Source
pub fn
partition_dedup_by
<F>(&mut self, same_bucket: F) -> (&mut
[T]
, &mut
[T]
)
where
    F:
FnMut
(
&mut T
,
&mut T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all but the first of consecutive elements to the end of the slice satisfying
a given equality relation.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
The
same_bucket
function is passed references to two elements from the slice and
must determine if the elements compare equal. The elements are passed in opposite order
from their order in the slice, so if
same_bucket(a, b)
returns
true
,
a
is moved
at the end of the slice.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
"foo"
,
"Foo"
,
"BAZ"
,
"Bar"
,
"bar"
,
"baz"
,
"BAZ"
];
let
(dedup, duplicates) = slice.partition_dedup_by(|a, b| a.eq_ignore_ascii_case(b));
assert_eq!
(dedup, [
"foo"
,
"BAZ"
,
"Bar"
,
"baz"
]);
assert_eq!
(duplicates, [
"bar"
,
"Foo"
,
"BAZ"
]);
Source
pub fn
partition_dedup_by_key
<K, F>(&mut self, key: F) -> (&mut
[T]
, &mut
[T]
)
where
    F:
FnMut
(
&mut T
) -> K,
    K:
PartialEq
,
🔬
This is a nightly-only experimental API. (
slice_partition_dedup
#54279
)
Moves all but the first of consecutive elements to the end of the slice that resolve
to the same key.
Returns two slices. The first contains no consecutive repeated elements.
The second contains all the duplicates in no specified order.
If the slice is sorted, the first returned slice contains no duplicates.
§
Examples
#![feature(slice_partition_dedup)]
let
mut
slice = [
10
,
20
,
21
,
30
,
30
,
20
,
11
,
13
];
let
(dedup, duplicates) = slice.partition_dedup_by_key(|i|
*
i /
10
);
assert_eq!
(dedup, [
10
,
20
,
30
,
20
,
11
]);
assert_eq!
(duplicates, [
21
,
30
,
13
]);
1.26.0
·
Source
pub fn
rotate_left
(&mut self, mid:
usize
)
Rotates the slice in-place such that the first
mid
elements of the
slice move to the end while the last
self.len() - mid
elements move to
the front.
After calling
rotate_left
, the element previously at index
mid
will
become the first element in the slice.
§
Panics
This function will panic if
mid
is greater than the length of the
slice. Note that
mid == self.len()
does
not
panic and is a no-op
rotation.
§
Complexity
Takes linear (in
self.len()
) time.
§
Examples
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a.rotate_left(
2
);
assert_eq!
(a, [
'c'
,
'd'
,
'e'
,
'f'
,
'a'
,
'b'
]);
Rotating a subslice:
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a[
1
..
5
].rotate_left(
1
);
assert_eq!
(a, [
'a'
,
'c'
,
'd'
,
'e'
,
'b'
,
'f'
]);
1.26.0
·
Source
pub fn
rotate_right
(&mut self, k:
usize
)
Rotates the slice in-place such that the first
self.len() - k
elements of the slice move to the end while the last
k
elements move
to the front.
After calling
rotate_right
, the element previously at index
self.len() - k
will become the first element in the slice.
§
Panics
This function will panic if
k
is greater than the length of the
slice. Note that
k == self.len()
does
not
panic and is a no-op
rotation.
§
Complexity
Takes linear (in
self.len()
) time.
§
Examples
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a.rotate_right(
2
);
assert_eq!
(a, [
'e'
,
'f'
,
'a'
,
'b'
,
'c'
,
'd'
]);
Rotating a subslice:
let
mut
a = [
'a'
,
'b'
,
'c'
,
'd'
,
'e'
,
'f'
];
a[
1
..
5
].rotate_right(
1
);
assert_eq!
(a, [
'a'
,
'e'
,
'b'
,
'c'
,
'd'
,
'f'
]);
1.50.0
·
Source
pub fn
fill
(&mut self, value: T)
where
    T:
Clone
,
Fills
self
with elements by cloning
value
.
§
Examples
let
mut
buf =
vec!
[
0
;
10
];
buf.fill(
1
);
assert_eq!
(buf,
vec!
[
1
;
10
]);
1.51.0
·
Source
pub fn
fill_with
<F>(&mut self, f: F)
where
    F:
FnMut
() -> T,
Fills
self
with elements returned by calling a closure repeatedly.
This method uses a closure to create new values. If you’d rather
Clone
a given value, use
fill
. If you want to use the
Default
trait to generate values, you can pass
Default::default
as the
argument.
§
Examples
let
mut
buf =
vec!
[
1
;
10
];
buf.fill_with(Default::default);
assert_eq!
(buf,
vec!
[
0
;
10
]);
1.7.0
·
Source
pub fn
clone_from_slice
(&mut self, src: &
[T]
)
where
    T:
Clone
,
Copies the elements from
src
into
self
.
The length of
src
must be the same as
self
.
§
Panics
This function will panic if the two slices have different lengths.
§
Examples
Cloning two elements from a slice into another:
let
src = [
1
,
2
,
3
,
4
];
let
mut
dst = [
0
,
0
];
// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.clone_from_slice(
&
src[
2
..]);
assert_eq!
(src, [
1
,
2
,
3
,
4
]);
assert_eq!
(dst, [
3
,
4
]);
Rust enforces that there can only be one mutable reference with no
immutable references to a particular piece of data in a particular
scope. Because of this, attempting to use
clone_from_slice
on a
single slice will result in a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

slice[..
2
].clone_from_slice(
&
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.clone_from_slice(
&
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
4
,
5
]);
1.9.0
·
Source
pub fn
copy_from_slice
(&mut self, src: &
[T]
)
where
    T:
Copy
,
Copies all elements from
src
into
self
, using a memcpy.
The length of
src
must be the same as
self
.
If
T
does not implement
Copy
, use
clone_from_slice
.
§
Panics
This function will panic if the two slices have different lengths.
§
Examples
Copying two elements from a slice into another:
let
src = [
1
,
2
,
3
,
4
];
let
mut
dst = [
0
,
0
];
// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.copy_from_slice(
&
src[
2
..]);
assert_eq!
(src, [
1
,
2
,
3
,
4
]);
assert_eq!
(dst, [
3
,
4
]);
Rust enforces that there can only be one mutable reference with no
immutable references to a particular piece of data in a particular
scope. Because of this, attempting to use
copy_from_slice
on a
single slice will result in a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

slice[..
2
].copy_from_slice(
&
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.copy_from_slice(
&
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
4
,
5
]);
1.37.0
·
Source
pub fn
copy_within
<R>(&mut self, src: R, dest:
usize
)
where
    R:
RangeBounds
<
usize
>,
    T:
Copy
,
Copies elements from one part of the slice to another part of itself,
using a memmove.
src
is the range within
self
to copy from.
dest
is the starting
index of the range within
self
to copy to, which will have the same
length as
src
. The two ranges may overlap. The ends of the two ranges
must be less than or equal to
self.len()
.
§
Panics
This function will panic if either range exceeds the end of the slice,
or if the end of
src
is before the start.
§
Examples
Copying four bytes within a slice:
let
mut
bytes =
*
b"Hello, World!"
;

bytes.copy_within(
1
..
5
,
8
);
assert_eq!
(
&
bytes,
b"Hello, Wello!"
);
1.27.0
·
Source
pub fn
swap_with_slice
(&mut self, other: &mut
[T]
)
Swaps all elements in
self
with those in
other
.
The length of
other
must be the same as
self
.
§
Panics
This function will panic if the two slices have different lengths.
§
Example
Swapping two elements across slices:
let
mut
slice1 = [
0
,
0
];
let
mut
slice2 = [
1
,
2
,
3
,
4
];

slice1.swap_with_slice(
&mut
slice2[
2
..]);
assert_eq!
(slice1, [
3
,
4
]);
assert_eq!
(slice2, [
1
,
2
,
0
,
0
]);
Rust enforces that there can only be one mutable reference to a
particular piece of data in a particular scope. Because of this,
attempting to use
swap_with_slice
on a single slice will result in
a compile failure:
ⓘ
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];
slice[..
2
].swap_with_slice(
&mut
slice[
3
..]);
// compile fail!
To work around this, we can use
split_at_mut
to create two distinct
mutable sub-slices from a slice:
let
mut
slice = [
1
,
2
,
3
,
4
,
5
];

{
let
(left, right) = slice.split_at_mut(
2
);
    left.swap_with_slice(
&mut
right[
1
..]);
}
assert_eq!
(slice, [
4
,
5
,
3
,
1
,
2
]);
1.30.0
·
Source
pub unsafe fn
align_to
<U>(&self) -> (&
[T]
, &
[U]
, &
[T]
)
Transmutes the slice to a slice of another type, ensuring alignment of the types is
maintained.
This method splits the slice into three distinct slices: prefix, correctly aligned middle
slice of a new type, and the suffix slice. The middle part will be as big as possible under
the given alignment constraint and element size.
This method has no purpose when either input element
T
or output element
U
are
zero-sized and will return the original slice without splitting anything.
§
Safety
This method is essentially a
transmute
with respect to the elements in the returned
middle slice, so all the usual caveats pertaining to
transmute::<T, U>
also apply here.
§
Examples
Basic usage:
unsafe
{
let
bytes: [u8;
7
] = [
1
,
2
,
3
,
4
,
5
,
6
,
7
];
let
(prefix, shorts, suffix) = bytes.align_to::<u16>();
// less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
1.30.0
·
Source
pub unsafe fn
align_to_mut
<U>(&mut self) -> (&mut
[T]
, &mut
[U]
, &mut
[T]
)
Transmutes the mutable slice to a mutable slice of another type, ensuring alignment of the
types is maintained.
This method splits the slice into three distinct slices: prefix, correctly aligned middle
slice of a new type, and the suffix slice. The middle part will be as big as possible under
the given alignment constraint and element size.
This method has no purpose when either input element
T
or output element
U
are
zero-sized and will return the original slice without splitting anything.
§
Safety
This method is essentially a
transmute
with respect to the elements in the returned
middle slice, so all the usual caveats pertaining to
transmute::<T, U>
also apply here.
§
Examples
Basic usage:
unsafe
{
let
mut
bytes: [u8;
7
] = [
1
,
2
,
3
,
4
,
5
,
6
,
7
];
let
(prefix, shorts, suffix) = bytes.align_to_mut::<u16>();
// less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
Source
pub fn
as_simd
<const LANES:
usize
>(&self) -> (&
[T]
, &[
Simd
<T, LANES>], &
[T]
)
where
Simd
<T, LANES>:
AsRef
<
[T; LANES]
>,
    T:
SimdElement
,
LaneCount
<LANES>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.
This is a safe wrapper around
slice::align_to
, so inherits the same
guarantees as that method.
§
Panics
This will panic if the size of the SIMD type is different from
LANES
times that of the scalar.
At the time of writing, the trait restrictions on
Simd<T, LANES>
keeps
that from ever happening, as only power-of-two numbers of lanes are
supported.  It’s possible that, in the future, those restrictions might
be lifted in a way that would make it possible to see panics from this
method for something like
LANES == 3
.
§
Examples
#![feature(portable_simd)]
use
core::simd::prelude::
*
;
let
short =
&
[
1
,
2
,
3
];
let
(prefix, middle, suffix) = short.as_simd::<
4
>();
assert_eq!
(middle, []);
// Not enough elements for anything in the middle

// They might be split in any possible way between prefix and suffix
let
it = prefix.iter().chain(suffix).copied();
assert_eq!
(it.collect::<Vec<
_
>>(),
vec!
[
1
,
2
,
3
]);
fn
basic_simd_sum(x:
&
[f32]) -> f32 {
use
std::ops::Add;
let
(prefix, middle, suffix) = x.as_simd();
let
sums = f32x4::from_array([
        prefix.iter().copied().sum(),
0.0
,
0.0
,
        suffix.iter().copied().sum(),
    ]);
let
sums = middle.iter().copied().fold(sums, f32x4::add);
    sums.reduce_sum()
}
let
numbers: Vec<f32> = (
1
..
101
).map(|x| x
as _
).collect();
assert_eq!
(basic_simd_sum(
&
numbers[
1
..
99
]),
4949.0
);
Source
pub fn
as_simd_mut
<const LANES:
usize
>(
    &mut self,
) -> (&mut
[T]
, &mut [
Simd
<T, LANES>], &mut
[T]
)
where
Simd
<T, LANES>:
AsMut
<
[T; LANES]
>,
    T:
SimdElement
,
LaneCount
<LANES>:
SupportedLaneCount
,
🔬
This is a nightly-only experimental API. (
portable_simd
#86656
)
Splits a mutable slice into a mutable prefix, a middle of aligned SIMD types,
and a mutable suffix.
This is a safe wrapper around
slice::align_to_mut
, so inherits the same
guarantees as that method.
This is the mutable version of
slice::as_simd
; see that for examples.
§
Panics
This will panic if the size of the SIMD type is different from
LANES
times that of the scalar.
At the time of writing, the trait restrictions on
Simd<T, LANES>
keeps
that from ever happening, as only power-of-two numbers of lanes are
supported.  It’s possible that, in the future, those restrictions might
be lifted in a way that would make it possible to see panics from this
method for something like
LANES == 3
.
1.82.0
·
Source
pub fn
is_sorted
(&self) ->
bool
where
    T:
PartialOrd
,
Checks if the elements of this slice are sorted.
That is, for each element
a
and its following element
b
,
a <= b
must hold. If the
slice yields exactly zero or one element,
true
is returned.
Note that if
Self::Item
is only
PartialOrd
, but not
Ord
, the above definition
implies that this function returns
false
if any two consecutive items are not
comparable.
§
Examples
let
empty: [i32;
0
] = [];
assert!
([
1
,
2
,
2
,
9
].is_sorted());
assert!
(![
1
,
3
,
2
,
4
].is_sorted());
assert!
([
0
].is_sorted());
assert!
(empty.is_sorted());
assert!
(![
0.0
,
1.0
, f32::NAN].is_sorted());
1.82.0
·
Source
pub fn
is_sorted_by
<'a, F>(&'a self, compare: F) ->
bool
where
    F:
FnMut
(
&'a T
,
&'a T
) ->
bool
,
Checks if the elements of this slice are sorted using the given comparator function.
Instead of using
PartialOrd::partial_cmp
, this function uses the given
compare
function to determine whether two elements are to be considered in sorted order.
§
Examples
assert!
([
1
,
2
,
2
,
9
].is_sorted_by(|a, b| a <= b));
assert!
(![
1
,
2
,
2
,
9
].is_sorted_by(|a, b| a < b));
assert!
([
0
].is_sorted_by(|a, b|
true
));
assert!
([
0
].is_sorted_by(|a, b|
false
));
let
empty: [i32;
0
] = [];
assert!
(empty.is_sorted_by(|a, b|
false
));
assert!
(empty.is_sorted_by(|a, b|
true
));
1.82.0
·
Source
pub fn
is_sorted_by_key
<'a, F, K>(&'a self, f: F) ->
bool
where
    F:
FnMut
(
&'a T
) -> K,
    K:
PartialOrd
,
Checks if the elements of this slice are sorted using the given key extraction function.
Instead of comparing the slice’s elements directly, this function compares the keys of the
elements, as determined by
f
. Apart from that, it’s equivalent to
is_sorted
; see its
documentation for more information.
§
Examples
assert!
([
"c"
,
"bb"
,
"aaa"
].is_sorted_by_key(|s| s.len()));
assert!
(![-
2i32
, -
1
,
0
,
3
].is_sorted_by_key(|n| n.abs()));
1.52.0
·
Source
pub fn
partition_point
<P>(&self, pred: P) ->
usize
where
    P:
FnMut
(
&T
) ->
bool
,
Returns the index of the partition point according to the given predicate
(the index of the first element of the second partition).
The slice is assumed to be partitioned according to the given predicate.
This means that all elements for which the predicate returns true are at the start of the slice
and all elements for which the predicate returns false are at the end.
For example,
[7, 15, 3, 5, 4, 12, 6]
is partitioned under the predicate
x % 2 != 0
(all odd numbers are at the start, all even at the end).
If this slice is not partitioned, the returned result is unspecified and meaningless,
as this method performs a kind of binary search.
See also
binary_search
,
binary_search_by
, and
binary_search_by_key
.
§
Examples
let
v = [
1
,
2
,
3
,
3
,
5
,
6
,
7
];
let
i = v.partition_point(|
&
x| x <
5
);
assert_eq!
(i,
4
);
assert!
(v[..i].iter().all(|
&
x| x <
5
));
assert!
(v[i..].iter().all(|
&
x| !(x <
5
)));
If all elements of the slice match the predicate, including if the slice
is empty, then the length of the slice will be returned:
let
a = [
2
,
4
,
8
];
assert_eq!
(a.partition_point(|x| x <
&
100
), a.len());
let
a: [i32;
0
] = [];
assert_eq!
(a.partition_point(|x| x <
&
100
),
0
);
If you want to insert an item to a sorted vector, while maintaining
sort order:
let
mut
s =
vec!
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
];
let
num =
42
;
let
idx = s.partition_point(|
&
x| x <= num);
s.insert(idx, num);
assert_eq!
(s, [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
1.87.0
·
Source
pub fn
split_off
<'a, R>(self: &mut &'a
[T]
, range: R) ->
Option
<&'a
[T]
>
where
    R:
OneSidedRange
<
usize
>,
Removes the subslice corresponding to the given range
and returns a reference to it.
Returns
None
and does not modify the slice if the given
range is out of bounds.
Note that this method only accepts one-sided ranges such as
2..
or
..6
, but not
2..6
.
§
Examples
Splitting off the first three elements of a slice:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
first_three = slice.split_off(..
3
).unwrap();
assert_eq!
(slice,
&
[
'd'
]);
assert_eq!
(first_three,
&
[
'a'
,
'b'
,
'c'
]);
Splitting off the last two elements of a slice:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
tail = slice.split_off(
2
..).unwrap();
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(tail,
&
[
'c'
,
'd'
]);
Getting
None
when
range
is out of bounds:
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
None
, slice.split_off(
5
..));
assert_eq!
(
None
, slice.split_off(..
5
));
assert_eq!
(
None
, slice.split_off(..=
4
));
let
expected:
&
[char] =
&
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
Some
(expected), slice.split_off(..
4
));
1.87.0
·
Source
pub fn
split_off_mut
<'a, R>(
    self: &mut &'a mut
[T]
,
    range: R,
) ->
Option
<&'a mut
[T]
>
where
    R:
OneSidedRange
<
usize
>,
Removes the subslice corresponding to the given range
and returns a mutable reference to it.
Returns
None
and does not modify the slice if the given
range is out of bounds.
Note that this method only accepts one-sided ranges such as
2..
or
..6
, but not
2..6
.
§
Examples
Splitting off the first three elements of a slice:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
first_three = slice.split_off_mut(..
3
).unwrap();
assert_eq!
(slice,
&mut
[
'd'
]);
assert_eq!
(first_three,
&mut
[
'a'
,
'b'
,
'c'
]);
Taking the last two elements of a slice:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
let
mut
tail = slice.split_off_mut(
2
..).unwrap();
assert_eq!
(slice,
&mut
[
'a'
,
'b'
]);
assert_eq!
(tail,
&mut
[
'c'
,
'd'
]);
Getting
None
when
range
is out of bounds:
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
None
, slice.split_off_mut(
5
..));
assert_eq!
(
None
, slice.split_off_mut(..
5
));
assert_eq!
(
None
, slice.split_off_mut(..=
4
));
let
expected:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
,
'd'
];
assert_eq!
(
Some
(expected), slice.split_off_mut(..
4
));
1.87.0
·
Source
pub fn
split_off_first
<'a>(self: &mut &'a
[T]
) ->
Option
<
&'a T
>
Removes the first element of the slice and returns a reference
to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
];
let
first = slice.split_off_first().unwrap();
assert_eq!
(slice,
&
[
'b'
,
'c'
]);
assert_eq!
(first,
&
'a'
);
1.87.0
·
Source
pub fn
split_off_first_mut
<'a>(self: &mut &'a mut
[T]
) ->
Option
<
&'a mut T
>
Removes the first element of the slice and returns a mutable
reference to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
];
let
first = slice.split_off_first_mut().unwrap();
*
first =
'd'
;
assert_eq!
(slice,
&
[
'b'
,
'c'
]);
assert_eq!
(first,
&
'd'
);
1.87.0
·
Source
pub fn
split_off_last
<'a>(self: &mut &'a
[T]
) ->
Option
<
&'a T
>
Removes the last element of the slice and returns a reference
to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&
[
_
] =
&
[
'a'
,
'b'
,
'c'
];
let
last = slice.split_off_last().unwrap();
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(last,
&
'c'
);
1.87.0
·
Source
pub fn
split_off_last_mut
<'a>(self: &mut &'a mut
[T]
) ->
Option
<
&'a mut T
>
Removes the last element of the slice and returns a mutable
reference to it.
Returns
None
if the slice is empty.
§
Examples
let
mut
slice:
&mut
[
_
] =
&mut
[
'a'
,
'b'
,
'c'
];
let
last = slice.split_off_last_mut().unwrap();
*
last =
'd'
;
assert_eq!
(slice,
&
[
'a'
,
'b'
]);
assert_eq!
(last,
&
'd'
);
1.86.0
·
Source
pub unsafe fn
get_disjoint_unchecked_mut
<I, const N:
usize
>(
    &mut self,
    indices:
[I; N]
,
) -> [&mut <I as
SliceIndex
<
[T]
>>::
Output
;
N
]
where
    I:
GetDisjointMutIndex
+
SliceIndex
<
[T]
>,
Returns mutable references to many indices at once, without doing any checks.
An index can be either a
usize
, a
Range
or a
RangeInclusive
. Note
that this method takes an array, so all indices must be of the same type.
If passed an array of
usize
s this method gives back an array of mutable references
to single elements, while if passed an array of ranges it gives back an array of
mutable references to slices.
For a safe alternative see
get_disjoint_mut
.
§
Safety
Calling this method with overlapping or out-of-bounds indices is
undefined behavior
even if the resulting references are not used.
§
Examples
let
x =
&mut
[
1
,
2
,
4
];
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
0
,
2
]);
*
a
*
=
10
;
*
b
*
=
100
;
}
assert_eq!
(x,
&
[
10
,
2
,
400
]);
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
0
..
1
,
1
..
3
]);
    a[
0
] =
8
;
    b[
0
] =
88
;
    b[
1
] =
888
;
}
assert_eq!
(x,
&
[
8
,
88
,
888
]);
unsafe
{
let
[a, b] = x.get_disjoint_unchecked_mut([
1
..=
2
,
0
..=
0
]);
    a[
0
] =
11
;
    a[
1
] =
111
;
    b[
0
] =
1
;
}
assert_eq!
(x,
&
[
1
,
11
,
111
]);
1.86.0
·
Source
pub fn
get_disjoint_mut
<I, const N:
usize
>(
    &mut self,
    indices:
[I; N]
,
) ->
Result
<[&mut <I as
SliceIndex
<
[T]
>>::
Output
;
N
],
GetDisjointMutError
>
where
    I:
GetDisjointMutIndex
+
SliceIndex
<
[T]
>,
Returns mutable references to many indices at once.
An index can be either a
usize
, a
Range
or a
RangeInclusive
. Note
that this method takes an array, so all indices must be of the same type.
If passed an array of
usize
s this method gives back an array of mutable references
to single elements, while if passed an array of ranges it gives back an array of
mutable references to slices.
Returns an error if any index is out-of-bounds, or if there are overlapping indices.
An empty range is not considered to overlap if it is located at the beginning or at
the end of another range, but is considered to overlap if it is located in the middle.
This method does a O(n^2) check to check that there are no overlapping indices, so be careful
when passing many indices.
§
Examples
let
v =
&mut
[
1
,
2
,
3
];
if let
Ok
([a, b]) = v.get_disjoint_mut([
0
,
2
]) {
*
a =
413
;
*
b =
612
;
}
assert_eq!
(v,
&
[
413
,
2
,
612
]);
if let
Ok
([a, b]) = v.get_disjoint_mut([
0
..
1
,
1
..
3
]) {
    a[
0
] =
8
;
    b[
0
] =
88
;
    b[
1
] =
888
;
}
assert_eq!
(v,
&
[
8
,
88
,
888
]);
if let
Ok
([a, b]) = v.get_disjoint_mut([
1
..=
2
,
0
..=
0
]) {
    a[
0
] =
11
;
    a[
1
] =
111
;
    b[
0
] =
1
;
}
assert_eq!
(v,
&
[
1
,
11
,
111
]);
Source
pub fn
element_offset
(&self, element:
&T
) ->
Option
<
usize
>
🔬
This is a nightly-only experimental API. (
substr_range
#126769
)
Returns the index that an element reference points to.
Returns
None
if
element
does not point to the start of an element within the slice.
This method is useful for extending slice iterators like
slice::split
.
Note that this uses pointer arithmetic and
does not compare elements
.
To find the index of an element via comparison, use
.iter().position()
instead.
§
Panics
Panics if
T
is zero-sized.
§
Examples
Basic usage:
#![feature(substr_range)]
let
nums:
&
[u32] =
&
[
1
,
7
,
1
,
1
];
let
num =
&
nums[
2
];
assert_eq!
(num,
&
1
);
assert_eq!
(nums.element_offset(num),
Some
(
2
));
Returning
None
with an unaligned element:
#![feature(substr_range)]
let
arr:
&
[[u32;
2
]] =
&
[[
0
,
1
], [
2
,
3
]];
let
flat_arr:
&
[u32] = arr.as_flattened();
let
ok_elm:
&
[u32;
2
] = flat_arr[
0
..
2
].try_into().unwrap();
let
weird_elm:
&
[u32;
2
] = flat_arr[
1
..
3
].try_into().unwrap();
assert_eq!
(ok_elm,
&
[
0
,
1
]);
assert_eq!
(weird_elm,
&
[
1
,
2
]);
assert_eq!
(arr.element_offset(ok_elm),
Some
(
0
));
// Points to element 0
assert_eq!
(arr.element_offset(weird_elm),
None
);
// Points between element 0 and 1
Source
pub fn
subslice_range
(&self, subslice: &
[T]
) ->
Option
<
Range
<
usize
>>
🔬
This is a nightly-only experimental API. (
substr_range
#126769
)
Returns the range of indices that a subslice points to.
Returns
None
if
subslice
does not point within the slice or if it is not aligned with the
elements in the slice.
This method
does not compare elements
. Instead, this method finds the location in the slice that
subslice
was obtained from. To find the index of a subslice via comparison, instead use
.windows()
.position()
.
This method is useful for extending slice iterators like
slice::split
.
Note that this may return a false positive (either
Some(0..0)
or
Some(self.len()..self.len())
)
if
subslice
has a length of zero and points to the beginning or end of another, separate, slice.
§
Panics
Panics if
T
is zero-sized.
§
Examples
Basic usage:
#![feature(substr_range)]
let
nums =
&
[
0
,
5
,
10
,
0
,
0
,
5
];
let
mut
iter = nums
    .split(|t|
*
t ==
0
)
    .map(|n| nums.subslice_range(n).unwrap());
assert_eq!
(iter.next(),
Some
(
0
..
0
));
assert_eq!
(iter.next(),
Some
(
1
..
3
));
assert_eq!
(iter.next(),
Some
(
4
..
4
));
assert_eq!
(iter.next(),
Some
(
5
..
6
));
1.79.0
·
Source
pub fn
utf8_chunks
(&self) ->
Utf8Chunks
<'_>
ⓘ
Creates an iterator over the contiguous valid UTF-8 ranges of this
slice, and the non-UTF-8 fragments in between.
See the
Utf8Chunk
type for documentation of the items yielded by this iterator.
§
Examples
This function formats arbitrary but mostly-UTF-8 bytes into Rust source
code in the form of a C-string literal (
c"..."
).
use
std::fmt::Write
as _
;
pub fn
cstr_literal(bytes:
&
[u8]) -> String {
let
mut
repr = String::new();
    repr.push_str(
"c\""
);
for
chunk
in
bytes.utf8_chunks() {
for
ch
in
chunk.valid().chars() {
// Escapes \0, \t, \r, \n, \\, \', \", and uses \u{...} for non-printable characters.
write!
(repr,
"{}"
, ch.escape_debug()).unwrap();
        }
for
byte
in
chunk.invalid() {
write!
(repr,
"\\x{:02X}"
, byte).unwrap();
        }
    }
    repr.push(
'"'
);
    repr
}
fn
main() {
let
lit = cstr_literal(
b"\xferris the \xf0\x9f\xa6\x80\x07"
);
let
expected =
stringify!
(
c"\xFErris the 🦀\u{7}"
);
assert_eq!
(lit, expected);
}
1.0.0
·
Source
pub fn
sort
(&mut self)
where
    T:
Ord
,
Sorts the slice, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
n
* log(
n
))
worst-case.
If the implementation of
Ord
for
T
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
When applicable, unstable sorting is preferred because it is generally faster than stable
sorting and it doesn’t allocate auxiliary memory. See
sort_unstable
. The exception are partially sorted slices, which
may be better served with
slice::sort
.
Sorting types that only implement
PartialOrd
such as
f32
and
f64
require
additional precautions. For example,
f32::NAN != f32::NAN
, which doesn’t fulfill the
reflexivity requirement of
Ord
. By using an alternative comparison function with
slice::sort_by
such as
f32::total_cmp
or
f64::total_cmp
that defines a
total
order
users can sort slices containing floating-point values. Alternatively, if all values
in the slice are guaranteed to be in a subset for which
PartialOrd::partial_cmp
forms a
total order
, it’s possible to sort the slice with
sort_by(|a, b| a.partial_cmp(b).unwrap())
.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if the implementation of
Ord
for
T
does not implement a
total order
, or if
the
Ord
implementation itself panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];

v.sort();
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
1.0.0
·
Source
pub fn
sort_by
<F>(&mut self, compare: F)
where
    F:
FnMut
(
&T
,
&T
) ->
Ordering
,
Sorts the slice with a comparison function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
n
* log(
n
))
worst-case.
If the comparison function
compare
does not implement a
total order
, the function may
panic; even if the function exits normally, the resulting order of elements in the slice is
unspecified. See also the note on panicking below.
For example
|a, b| (a - b).cmp(a)
is a comparison function that is neither transitive nor
reflexive nor total,
a < b < c < a
with
a = 1, b = 2, c = 3
. For more information and
examples see the
Ord
documentation.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if
compare
does not implement a
total order
, or if
compare
itself panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4
, -
5
,
1
, -
3
,
2
];
v.sort_by(|a, b| a.cmp(b));
assert_eq!
(v, [-
5
, -
3
,
1
,
2
,
4
]);
// reverse sorting
v.sort_by(|a, b| b.cmp(a));
assert_eq!
(v, [
4
,
2
,
1
, -
3
, -
5
]);
1.7.0
·
Source
pub fn
sort_by_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
m
*
n
* log(
n
))
worst-case, where the key function is
O
(
m
).
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
§
Current implementation
The current implementation is based on
driftsort
by Orson Peters and Lukas Bergdoll, which
combines the fast average case of quicksort with the fast worst case and partial run
detection of mergesort, achieving linear time on fully sorted and reversed inputs. On inputs
with k distinct elements, the expected time to sort the data is
O
(
n
* log(
k
)).
The auxiliary memory allocation behavior depends on the input length. Short slices are
handled without allocation, medium sized slices allocate
self.len()
and beyond that it
clamps at
self.len() / 2
.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation or the key-function
f
panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
];

v.sort_by_key(|k| k.abs());
assert_eq!
(v, [
1
,
2
, -
3
,
4
, -
5
]);
1.34.0
·
Source
pub fn
sort_by_cached_key
<K, F>(&mut self, f: F)
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Sorts the slice with a key extraction function, preserving initial order of equal elements.
This sort is stable (i.e., does not reorder equal elements) and
O
(
m
*
n
+
n
*
log(
n
)) worst-case, where the key function is
O
(
m
).
During sorting, the key function is called at most once per element, by using temporary
storage to remember the results of key evaluation. The order of calls to the key function is
unspecified and may change in future versions of the standard library.
If the implementation of
Ord
for
K
does not implement a
total order
, the function
may panic; even if the function exits normally, the resulting order of elements in the slice
is unspecified. See also the note on panicking below.
For simple key functions (e.g., functions that are property accesses or basic operations),
sort_by_key
is likely to be faster.
§
Current implementation
The current implementation is based on
instruction-parallel-network sort
by Lukas
Bergdoll, which combines the fast average case of randomized quicksort with the fast worst
case of heapsort, while achieving linear time on fully sorted and reversed inputs. And
O
(
k
* log(
n
)) where
k
is the number of distinct elements in the input. It leverages
superscalar out-of-order execution capabilities commonly found in CPUs, to efficiently
perform the operation.
In the worst case, the algorithm allocates temporary storage in a
Vec<(K, usize)>
the
length of the slice.
§
Panics
May panic if the implementation of
Ord
for
K
does not implement a
total order
, or if
the
Ord
implementation panics.
All safe functions on slices preserve the invariant that even if the function panics, all
original elements will remain in the slice and any possible modifications via interior
mutability are observed in the input. This ensures that recovery code (for instance inside
of a
Drop
or following a
catch_unwind
) will still have access to all the original
elements. For instance, if the slice belongs to a
Vec
, the
Vec::drop
method will be able
to dispose of all contained elements.
§
Examples
let
mut
v = [
4i32
, -
5
,
1
, -
3
,
2
,
10
];
// Strings are sorted by lexicographical order.
v.sort_by_cached_key(|k| k.to_string());
assert_eq!
(v, [-
3
, -
5
,
1
,
10
,
2
,
4
]);
1.0.0
·
Source
pub fn
to_vec
(&self) ->
Vec
<T>
where
    T:
Clone
,
Copies
self
into a new
Vec
.
§
Examples
let
s = [
10
,
40
,
30
];
let
x = s.to_vec();
// Here, `s` and `x` can be modified independently.
Source
pub fn
to_vec_in
<A>(&self, alloc: A) ->
Vec
<T, A>
where
    A:
Allocator
,
    T:
Clone
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Copies
self
into a new
Vec
with an allocator.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
s = [
10
,
40
,
30
];
let
x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
1.40.0
·
Source
pub fn
repeat
(&self, n:
usize
) ->
Vec
<T>
where
    T:
Copy
,
Creates a vector by copying a slice
n
times.
§
Panics
This function will panic if the capacity would overflow.
§
Examples
Basic usage:
assert_eq!
([
1
,
2
].repeat(
3
),
vec!
[
1
,
2
,
1
,
2
,
1
,
2
]);
A panic upon overflow:
ⓘ
// this will panic at runtime
b"0123456789abcdef"
.repeat(usize::MAX);
1.0.0
·
Source
pub fn
concat
<Item>(&self) -> <
[T]
as
Concat
<Item>>::
Output
ⓘ
where
[T]
:
Concat
<Item>,
    Item: ?
Sized
,
Flattens a slice of
T
into a single value
Self::Output
.
§
Examples
assert_eq!
([
"hello"
,
"world"
].concat(),
"helloworld"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].concat(), [
1
,
2
,
3
,
4
]);
1.3.0
·
Source
pub fn
join
<Separator>(
    &self,
    sep: Separator,
) -> <
[T]
as
Join
<Separator>>::
Output
ⓘ
where
[T]
:
Join
<Separator>,
Flattens a slice of
T
into a single value
Self::Output
, placing a
given separator between each.
§
Examples
assert_eq!
([
"hello"
,
"world"
].join(
" "
),
"hello world"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].join(
&
0
), [
1
,
2
,
0
,
3
,
4
]);
assert_eq!
([[
1
,
2
], [
3
,
4
]].join(
&
[
0
,
0
][..]), [
1
,
2
,
0
,
0
,
3
,
4
]);
1.0.0
·
Source
pub fn
connect
<Separator>(
    &self,
    sep: Separator,
) -> <
[T]
as
Join
<Separator>>::
Output
ⓘ
where
[T]
:
Join
<Separator>,
👎
Deprecated since 1.3.0: renamed to join
Flattens a slice of
T
into a single value
Self::Output
, placing a
given separator between each.
§
Examples
assert_eq!
([
"hello"
,
"world"
].connect(
" "
),
"hello world"
);
assert_eq!
([[
1
,
2
], [
3
,
4
]].connect(
&
0
), [
1
,
2
,
0
,
3
,
4
]);
1.23.0
·
Source
pub fn
to_ascii_uppercase
(&self) ->
Vec
<
u8
>
ⓘ
Returns a vector containing a copy of this slice where each byte
is mapped to its ASCII upper case equivalent.
ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.
To uppercase the value in-place, use
make_ascii_uppercase
.
1.23.0
·
Source
pub fn
to_ascii_lowercase
(&self) ->
Vec
<
u8
>
ⓘ
Returns a vector containing a copy of this slice where each byte
is mapped to its ASCII lower case equivalent.
ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.
To lowercase the value in-place, use
make_ascii_lowercase
.
Trait Implementations
§
1.36.0
·
Source
§
impl<'a>
Debug
for
IoSliceMut
<'a>
Source
§
fn
fmt
(&self, fmt: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.36.0
·
Source
§
impl<'a>
Deref
for
IoSliceMut
<'a>
Source
§
type
Target
= [
u8
]
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &[
u8
]
ⓘ
Dereferences the value.
1.36.0
·
Source
§
impl<'a>
DerefMut
for
IoSliceMut
<'a>
Source
§
fn
deref_mut
(&mut self) -> &mut [
u8
]
ⓘ
Mutably dereferences the value.
1.44.0
·
Source
§
impl<'a>
Send
for
IoSliceMut
<'a>
1.44.0
·
Source
§
impl<'a>
Sync
for
IoSliceMut
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
IoSliceMut
<'a>
§
impl<'a>
RefUnwindSafe
for
IoSliceMut
<'a>
§
impl<'a>
Unpin
for
IoSliceMut
<'a>
§
impl<'a> !
UnwindSafe
for
IoSliceMut
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