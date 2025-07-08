exists in std::fs - Rust
std
::
fs
Function
exists
Copy item path
1.81.0
·
Source
pub fn exists<P:
AsRef
<
Path
>>(path: P) ->
Result
<
bool
>
Expand description
Returns
Ok(true)
if the path points at an existing entity.
This function will traverse symbolic links to query information about the
destination file. In case of broken symbolic links this will return
Ok(false)
.
As opposed to the
Path::exists
method, this will only return
Ok(true)
or
Ok(false)
if the path was
verified
to exist or not exist. If its existence can neither be confirmed
nor denied, an
Err(_)
will be propagated instead. This can be the case if e.g. listing
permission is denied on one of the parent directories.
Note that while this avoids some pitfalls of the
exists()
method, it still can not
prevent time-of-check to time-of-use (TOCTOU) bugs. You should only use it in scenarios
where those bugs are not an issue.
§
Examples
use
std::fs;
assert!
(!fs::exists(
"does_not_exist.txt"
).expect(
"Can't check existence of file does_not_exist.txt"
));
assert!
(fs::exists(
"/root/secret_file.txt"
).is_err());