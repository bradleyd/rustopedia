Index in std::ops - Rust
std
::
ops
Trait
Index
Copy item path
1.0.0
·
Source
pub trait Index<Idx>
where
    Idx: ?
Sized
,
{
    type
Output
: ?
Sized
;

    // Required method
    fn
index
(&self, index: Idx) -> &Self::
Output
;
}
Expand description
Used for indexing operations (
container[index]
) in immutable contexts.
container[index]
is actually syntactic sugar for
*container.index(index)
,
but only when used as an immutable value. If a mutable value is requested,
IndexMut
is used instead. This allows nice things such as
let value = v[index]
if the type of
value
implements
Copy
.
§
Examples
The following example implements
Index
on a read-only
NucleotideCount
container, enabling individual counts to be retrieved with index syntax.
use
std::ops::Index;
enum
Nucleotide {
    A,
    C,
    G,
    T,
}
struct
NucleotideCount {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}
impl
Index<Nucleotide>
for
NucleotideCount {
type
Output = usize;
fn
index(
&
self
, nucleotide: Nucleotide) ->
&
Self
::Output {
match
nucleotide {
            Nucleotide::A =>
&
self
.a,
            Nucleotide::C =>
&
self
.c,
            Nucleotide::G =>
&
self
.g,
            Nucleotide::T =>
&
self
.t,
        }
    }
}
let
nucleotide_count = NucleotideCount {a:
14
, c:
9
, g:
10
, t:
12
};
assert_eq!
(nucleotide_count[Nucleotide::A],
14
);
assert_eq!
(nucleotide_count[Nucleotide::C],
9
);
assert_eq!
(nucleotide_count[Nucleotide::G],
10
);
assert_eq!
(nucleotide_count[Nucleotide::T],
12
);
Required Associated Types
§
1.0.0
·
Source
type
Output
: ?
Sized
The returned type after indexing.
Required Methods
§
1.0.0
·
Source
fn
index
(&self, index: Idx) -> &Self::
Output
Performs the indexing (
container[index]
) operation.
§
Panics
May panic if the index is out of bounds.
Implementors
§
Source
§
impl
Index
<
usize
> for
ByteStr
Source
§
type
Output
=
u8
Source
§
impl
Index
<
usize
> for
ByteString
Source
§
type
Output
=
u8
Source
§
impl
Index
<
Range
<
usize
>> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
Range
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeFrom
<
usize
>> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeFrom
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
1.47.0
·
Source
§
impl
Index
<
RangeFrom
<
usize
>> for
CStr
Source
§
type
Output
=
CStr
Source
§
impl
Index
<
RangeFull
> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeFull
> for
ByteString
Source
§
type
Output
=
ByteStr
1.7.0
·
Source
§
impl
Index
<
RangeFull
> for
CString
Source
§
type
Output
=
CStr
1.0.0
·
Source
§
impl
Index
<
RangeFull
> for
OsString
Source
§
type
Output
=
OsStr
Source
§
impl
Index
<
RangeInclusive
<
usize
>> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeInclusive
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeTo
<
usize
>> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeTo
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeToInclusive
<
usize
>> for
ByteStr
Source
§
type
Output
=
ByteStr
Source
§
impl
Index
<
RangeToInclusive
<
usize
>> for
ByteString
Source
§
type
Output
=
ByteStr
1.0.0
·
Source
§
impl<I>
Index
<I> for
str
where
    I:
SliceIndex
<
str
>,
Source
§
type
Output
= <I as
SliceIndex
<
str
>>::
Output
1.0.0
·
Source
§
impl<I>
Index
<I> for
String
where
    I:
SliceIndex
<
str
>,
Source
§
type
Output
= <I as
SliceIndex
<
str
>>::
Output
Source
§
impl<I, T, const N:
usize
>
Index
<I> for
Simd
<T, N>
where
    T:
SimdElement
,
LaneCount
<N>:
SupportedLaneCount
,
    I:
SliceIndex
<
[T]
>,
Source
§
type
Output
= <I as
SliceIndex
<
[T]
>>::
Output
1.0.0
·
Source
§
impl<K, Q, V, A>
Index
<
&Q
> for
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
,
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Source
§
type
Output
= V
1.0.0
·
Source
§
impl<K, Q, V, S>
Index
<
&Q
> for
HashMap
<K, V, S>
where
    K:
Eq
+
Hash
+
Borrow
<Q>,
    Q:
Eq
+
Hash
+ ?
Sized
,
    S:
BuildHasher
,
Source
§
type
Output
= V
1.0.0
·
Source
§
impl<T, A>
Index
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Output
= T
1.0.0
·
Source
§
impl<T, I>
Index
<I> for
[T]
where
    I:
SliceIndex
<
[T]
>,
Source
§
type
Output
= <I as
SliceIndex
<
[T]
>>::
Output
1.0.0
·
Source
§
impl<T, I, A>
Index
<I> for
Vec
<T, A>
where
    I:
SliceIndex
<
[T]
>,
    A:
Allocator
,
Source
§
type
Output
= <I as
SliceIndex
<
[T]
>>::
Output
1.50.0
·
Source
§
impl<T, I, const N:
usize
>
Index
<I> for
[T; N]
where
[T]
:
Index
<I>,
Source
§
type
Output
= <
[T]
as
Index
<I>>::
Output