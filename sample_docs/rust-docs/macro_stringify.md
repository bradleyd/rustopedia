stringify in std - Rust
std
Macro
stringify
Copy item path
1.38.0
·
Source
macro_rules! stringify {
    ($($t:tt)*) => { ... };
}
Expand description
Stringifies its arguments.
This macro will yield an expression of type
&'static str
which is the
stringification of all the tokens passed to the macro. No restrictions
are placed on the syntax of the macro invocation itself.
Note that the expanded results of the input tokens may change in the
future. You should be careful if you rely on the output.
§
Examples
let
one_plus_one =
stringify!
(
1
+
1
);
assert_eq!
(one_plus_one,
"1 + 1"
);