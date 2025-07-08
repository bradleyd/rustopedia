ToString in std::string - Rust
std
::
string
Trait
ToString
Copy item path
1.0.0
·
Source
pub trait ToString {
    // Required method
    fn
to_string
(&self) ->
String
;
}
Expand description
A trait for converting a value to a
String
.
This trait is automatically implemented for any type which implements the
Display
trait. As such,
ToString
shouldn’t be implemented directly:
Display
should be implemented instead, and you get the
ToString
implementation for free.
Required Methods
§
1.0.0
·
Source
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
§
Examples
let
i =
5
;
let
five = String::from(
"5"
);
assert_eq!
(five, i.to_string());
Implementors
§
1.0.0
·
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
§
Panics
In this implementation, the
to_string
method panics
if the
Display
implementation returns an error.
This indicates an incorrect
Display
implementation
since
fmt::Write for String
never returns an error itself.