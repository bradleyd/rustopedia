Field in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
Field
Copy item path
Source
pub fn Field<F>(place:
()
, field:
u32
) -> F
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Access the field with the given index of some place.
This only makes sense to use in conjunction with
Variant
. If the type you are looking to
access the field of does not have variants, you can use normal field projection syntax.
There is no proper way to do a place projection to a variant in Rust, and so these two
functions are a workaround. You can access a field of a variant via
Field(Variant(place, var_idx), field_idx)
, where
var_idx
and
field_idx
are appropriate literals. Some
caveats:
The return type of
Variant
is always
()
. Don’t worry about that, the correct MIR will
still be generated.
In some situations, the return type of
Field
cannot be inferred. You may need to
annotate it on the function in these cases.
Since
Field
is a function call which is not a place expression, using this on the left
hand side of an expression is rejected by the compiler.
place!
is a macro provided to
work around that issue. Wrap the left hand side of an assignment in the macro to convince
the compiler that it’s ok.
§
Examples
#![allow(internal_features)]
#![feature(custom_mir, core_intrinsics)]
use
core::intrinsics::mir::
*
;
#[custom_mir(dialect =
"built"
)]
fn
unwrap_deref(opt:
Option
<
&
i32>) -> i32 {
mir!
{
        {
            RET =
*
Field::<
&
i32>(Variant(opt,
1
),
0
);
            Return()
        }
    }
}
#[custom_mir(dialect =
"built"
)]
fn
set(opt:
&mut
Option
<i32>) {
mir!
{
        {
place!
(Field(Variant(
*
opt,
1
),
0
)) =
5
;
            Return()
        }
    }
}