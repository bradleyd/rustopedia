true - Rust
Keyword
true
Copy item path
Source
Expand description
A value of type
bool
representing logical
true
.
Logically
true
is not equal to
false
.
§
Control structures that check for
true
Several of Rust’s control structures will check for a
bool
condition evaluating to
true
.
The condition in an
if
expression must be of type
bool
.
Whenever that condition evaluates to
true
, the
if
expression takes
on the value of the first block. If however, the condition evaluates
to
false
, the expression takes on value of the
else
block if there is one.
while
is another control flow construct expecting a
bool
-typed condition.
As long as the condition evaluates to
true
, the
while
loop will continually
evaluate its associated block.
match
arms can have guard clauses on them.