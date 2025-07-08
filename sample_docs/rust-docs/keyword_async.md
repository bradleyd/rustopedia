async - Rust
Keyword
async
Copy item path
Source
Expand description
Returns a
Future
instead of blocking the current thread.
Use
async
in front of
fn
,
closure
, or a
block
to turn the marked code into a
Future
.
As such the code will not be run immediately, but will only be evaluated when the returned
future is
.await
ed.
We have written an
async book
detailing
async
/
await
and trade-offs compared to using threads.
§
Editions
async
is a keyword from the 2018 edition onwards.
It is available for use in stable Rust from version 1.39 onwards.