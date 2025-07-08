variant_count in std::mem - Rust
std
::
mem
Function
variant_count
Copy item path
Source
pub const fn variant_count<T>() ->
usize
🔬
This is a nightly-only experimental API. (
variant_count
#73662
)
Expand description
Returns the number of variants in the enum type
T
.
If
T
is not an enum, calling this function will not result in undefined behavior, but the
return value is unspecified. Equally, if
T
is an enum with more variants than
usize::MAX
the return value is unspecified. Uninhabited variants will be counted.
Note that an enum may be expanded with additional variants in the future
as a non-breaking change, for example if it is marked
#[non_exhaustive]
,
which will change the result of this function.
§
Examples
use
std::mem;
enum
Void {}
enum
Foo { A(
&
'static
str), B(i32), C(i32) }
assert_eq!
(mem::variant_count::<Void>(),
0
);
assert_eq!
(mem::variant_count::<Foo>(),
3
);
assert_eq!
(mem::variant_count::<
Option
<!>>(),
2
);
assert_eq!
(mem::variant_count::<
Result
<!, !>>(),
2
);