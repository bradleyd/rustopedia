union - Rust
Keyword
union
Copy item path
Source
Expand description
The
Rust equivalent of a C-style union
.
A
union
looks like a
struct
in terms of declaration, but all of its
fields exist in the same memory, superimposed over one another. For instance,
if we wanted some bits in memory that we sometimes interpret as a
u32
and
sometimes as an
f32
, we could write:
union
IntOrFloat {
    i: u32,
    f: f32,
}
let
mut
u = IntOrFloat { f:
1.0
};
// Reading the fields of a union is always unsafe
assert_eq!
(
unsafe
{ u.i },
1065353216
);
// Updating through any of the field will modify all of them
u.i =
1073741824
;
assert_eq!
(
unsafe
{ u.f },
2.0
);
§
Matching on unions
It is possible to use pattern matching on
union
s. A single field name must
be used and it must match the name of one of the
union
’s field.
Like reading from a
union
, pattern matching on a
union
requires
unsafe
.
union
IntOrFloat {
    i: u32,
    f: f32,
}
let
u = IntOrFloat { f:
1.0
};
unsafe
{
match
u {
        IntOrFloat { i:
10
} =>
println!
(
"Found exactly ten!"
),
// Matching the field `f` provides an `f32`.
IntOrFloat { f } =>
println!
(
"Found f = {f} !"
),
    }
}
§
References to union fields
All fields in a
union
are all at the same place in memory which means
borrowing one borrows the entire
union
, for the same lifetime:
ⓘ
union
IntOrFloat {
    i: u32,
    f: f32,
}
let
mut
u = IntOrFloat { f:
1.0
};
let
f =
unsafe
{
&
u.f };
// This will not compile because the field has already been borrowed, even
// if only immutably
let
i =
unsafe
{
&mut
u.i };
*
i =
10
;
println!
(
"f = {f} and i = {i}"
);
See the
Reference
for more information on
union
s.