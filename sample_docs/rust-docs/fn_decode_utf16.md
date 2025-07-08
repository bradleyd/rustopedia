decode_utf16 in std::char - Rust
std
::
char
Function
decode_utf16
Copy item path
1.9.0
·
Source
pub fn decode_utf16<I>(iter: I) ->
DecodeUtf16
<<I as
IntoIterator
>::
IntoIter
>
ⓘ
where
    I:
IntoIterator
<Item =
u16
>,
Expand description
Creates an iterator over the UTF-16 encoded code points in
iter
, returning
unpaired surrogates as
Err
s. Use
char::decode_utf16
instead.