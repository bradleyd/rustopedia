mir in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Macro
mir
Copy item path
Source
pub macro mir(
        $(type RET = $ret_ty:ty;)? $(let $local_decl:ident $(:
        $local_decl_ty:ty)?;)* $(debug $dbg_name:ident => $dbg_data:expr;)* {
        $($entry:tt)* } $($block_name:ident $(($block_cleanup:ident))? = {
        $($block:tt)* })*
    ) {
    ...
}
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Macro for generating custom MIR.
See the module documentation for syntax details. This macro is not magic - it only transforms
your MIR into something that is easier to parse in the compiler.