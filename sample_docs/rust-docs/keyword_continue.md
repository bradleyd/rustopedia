continue - Rust
Keyword
continue
Copy item path
Source
Expand description
Skip to the next iteration of a loop.
When
continue
is encountered, the current iteration is terminated, returning control to the
loop head, typically continuing with the next iteration.
// Printing odd numbers by skipping even ones
for
number
in
1
..=
10
{
if
number %
2
==
0
{
continue
;
    }
println!
(
"{number}"
);
}
Like
break
,
continue
is normally associated with the innermost enclosing loop, but labels
may be used to specify the affected loop.
// Print Odd numbers under 30 with unit <= 5
'tens
:
for
ten
in
0
..
3
{
'_units
:
for
unit
in
0
..=
9
{
if
unit %
2
==
0
{
continue
;
        }
if
unit >
5
{
continue
'tens
;
        }
println!
(
"{}"
, ten *
10
+ unit);
    }
}
See
continue expressions
from the reference for more details.