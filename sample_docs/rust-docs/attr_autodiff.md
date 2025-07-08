autodiff in std::autodiff - Rust
std
::
autodiff
Attribute Macro
autodiff
Copy item path
Source
#[autodiff]
🔬
This is a nightly-only experimental API. (
autodiff
#124509
)
Expand description
This macro handles automatic differentiation.
Automatic Differentiation macro which allows generating a new function to compute
the derivative of a given function. It may only be applied to a function.
The expected usage syntax is
#[autodiff(NAME, MODE, INPUT_ACTIVITIES, OUTPUT_ACTIVITY)]
where:
NAME is a string that represents a valid function name.
MODE is any of Forward, Reverse, ForwardFirst, ReverseFirst.
INPUT_ACTIVITIES consists of one valid activity for each input parameter.
OUTPUT_ACTIVITY must not be set if we implicitly return nothing (or explicitly return
-> ()
). Otherwise it must be set to one of the allowed activities.