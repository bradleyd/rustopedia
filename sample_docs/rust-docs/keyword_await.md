await - Rust
Keyword
await
Copy item path
Source
Expand description
Suspend execution until the result of a
Future
is ready.
.await
ing a future will suspend the current function’s execution until the executor
has run the future to completion.
Read the
async book
for details on how
async
/
await
and executors work.
§
Editions
await
is a keyword from the 2018 edition onwards.
It is available for use in stable Rust from version 1.39 onwards.