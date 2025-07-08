PathBuf in std::path - Rust
std
::
path
Struct
PathBuf
Copy item path
1.0.0
·
Source
pub struct PathBuf {
/* private fields */
}
Expand description
An owned, mutable path (akin to
String
).
This type provides methods like
push
and
set_extension
that mutate
the path in place. It also implements
Deref
to
Path
, meaning that
all methods on
Path
slices are available on
PathBuf
values as well.
More details about the overall approach can be found in
the
module documentation
.
§
Examples
You can use
push
to build up a
PathBuf
from
components:
use
std::path::PathBuf;
let
mut
path = PathBuf::new();

path.push(
r"C:\"
);
path.push(
"windows"
);
path.push(
"system32"
);

path.set_extension(
"dll"
);
However,
push
is best used for dynamic situations. This is a better way
to do this when you know all of the components ahead of time:
use
std::path::PathBuf;
let
path: PathBuf = [
r"C:\"
,
"windows"
,
"system32.dll"
].iter().collect();
We can still do better than this! Since these are all strings, we can use
From::from
:
use
std::path::PathBuf;
let
path = PathBuf::from(
r"C:\windows\system32.dll"
);
Which method works best depends on what kind of situation you’re in.
Note that
PathBuf
does not always sanitize arguments, for example
push
allows paths built from strings which include separators:
use
std::path::PathBuf;
let
mut
path = PathBuf::new();

path.push(
r"C:\"
);
path.push(
"windows"
);
path.push(
r"..\otherdir"
);
path.push(
"system32"
);
The behavior of
PathBuf
may be changed to a panic on such inputs
in the future.
Extend::extend
should be used to add multi-part paths.
Implementations
§
Source
§
impl
PathBuf
1.0.0
·
Source
pub fn
new
() ->
PathBuf
Allocates an empty
PathBuf
.
§
Examples
use
std::path::PathBuf;
let
path = PathBuf::new();
1.44.0
·
Source
pub fn
with_capacity
(capacity:
usize
) ->
PathBuf
Creates a new
PathBuf
with a given capacity used to create the
internal
OsString
. See
with_capacity
defined on
OsString
.
§
Examples
use
std::path::PathBuf;
let
mut
path = PathBuf::with_capacity(
10
);
let
capacity = path.capacity();
// This push is done without reallocating
path.push(
r"C:\"
);
assert_eq!
(capacity, path.capacity());
1.0.0
·
Source
pub fn
as_path
(&self) -> &
Path
Coerces to a
Path
slice.
§
Examples
use
std::path::{Path, PathBuf};
let
p = PathBuf::from(
"/test"
);
assert_eq!
(Path::new(
"/test"
), p.as_path());
Source
pub fn
leak
<'a>(self) -> &'a mut
Path
🔬
This is a nightly-only experimental API. (
os_string_pathbuf_leak
#125965
)
Consumes and leaks the
PathBuf
, returning a mutable reference to the contents,
&'a mut Path
.
The caller has free choice over the returned lifetime, including ’static.
Indeed, this function is ideally used for data that lives for the remainder of
the program’s life, as dropping the returned reference will cause a memory leak.
It does not reallocate or shrink the
PathBuf
, so the leaked allocation may include
unused capacity that is not part of the returned slice. If you want to discard excess
capacity, call
into_boxed_path
, and then
Box::leak
instead.
However, keep in mind that trimming the capacity may result in a reallocation and copy.
1.0.0
·
Source
pub fn
push
<P:
AsRef
<
Path
>>(&mut self, path: P)
Extends
self
with
path
.
If
path
is absolute, it replaces the current path.
On Windows:
if
path
has a root but no prefix (e.g.,
\windows
), it
replaces everything except for the prefix (if any) of
self
.
if
path
has a prefix but no root, it replaces
self
.
if
self
has a verbatim prefix (e.g.
\\?\C:\windows
)
and
path
is not empty, the new path is normalized: all references
to
.
and
..
are removed.
Consider using
Path::join
if you need a new
PathBuf
instead of
using this function on a cloned
PathBuf
.
§
Examples
Pushing a relative path extends the existing path:
use
std::path::PathBuf;
let
mut
path = PathBuf::from(
"/tmp"
);
path.push(
"file.bk"
);
assert_eq!
(path, PathBuf::from(
"/tmp/file.bk"
));
Pushing an absolute path replaces the existing path:
use
std::path::PathBuf;
let
mut
path = PathBuf::from(
"/tmp"
);
path.push(
"/etc"
);
assert_eq!
(path, PathBuf::from(
"/etc"
));
1.0.0
·
Source
pub fn
pop
(&mut self) ->
bool
Truncates
self
to
self.parent
.
Returns
false
and does nothing if
self.parent
is
None
.
Otherwise, returns
true
.
§
Examples
use
std::path::{Path, PathBuf};
let
mut
p = PathBuf::from(
"/spirited/away.rs"
);

p.pop();
assert_eq!
(Path::new(
"/spirited"
), p);
p.pop();
assert_eq!
(Path::new(
"/"
), p);
1.0.0
·
Source
pub fn
set_file_name
<S:
AsRef
<
OsStr
>>(&mut self, file_name: S)
Updates
self.file_name
to
file_name
.
If
self.file_name
was
None
, this is equivalent to pushing
file_name
.
Otherwise it is equivalent to calling
pop
and then pushing
file_name
. The new path will be a sibling of the original path.
(That is, it will have the same parent.)
The argument is not sanitized, so can include separators. This
behavior may be changed to a panic in the future.
§
Examples
use
std::path::PathBuf;
let
mut
buf = PathBuf::from(
"/"
);
assert!
(buf.file_name() ==
None
);

buf.set_file_name(
"foo.txt"
);
assert!
(buf == PathBuf::from(
"/foo.txt"
));
assert!
(buf.file_name().is_some());

buf.set_file_name(
"bar.txt"
);
assert!
(buf == PathBuf::from(
"/bar.txt"
));

buf.set_file_name(
"baz"
);
assert!
(buf == PathBuf::from(
"/baz"
));

buf.set_file_name(
"../b/c.txt"
);
assert!
(buf == PathBuf::from(
"/../b/c.txt"
));

buf.set_file_name(
"baz"
);
assert!
(buf == PathBuf::from(
"/../b/baz"
));
1.0.0
·
Source
pub fn
set_extension
<S:
AsRef
<
OsStr
>>(&mut self, extension: S) ->
bool
Updates
self.extension
to
Some(extension)
or to
None
if
extension
is empty.
Returns
false
and does nothing if
self.file_name
is
None
,
returns
true
and updates the extension otherwise.
If
self.extension
is
None
, the extension is added; otherwise
it is replaced.
If
extension
is the empty string,
self.extension
will be
None
afterwards, not
Some("")
.
§
Panics
Panics if the passed extension contains a path separator (see
is_separator
).
§
Caveats
The new
extension
may contain dots and will be used in its entirety,
but only the part after the final dot will be reflected in
self.extension
.
If the file stem contains internal dots and
extension
is empty, part
of the old file stem will be considered the new
self.extension
.
See the examples below.
§
Examples
use
std::path::{Path, PathBuf};
let
mut
p = PathBuf::from(
"/feel/the"
);

p.set_extension(
"force"
);
assert_eq!
(Path::new(
"/feel/the.force"
), p.as_path());

p.set_extension(
"dark.side"
);
assert_eq!
(Path::new(
"/feel/the.dark.side"
), p.as_path());

p.set_extension(
"cookie"
);
assert_eq!
(Path::new(
"/feel/the.dark.cookie"
), p.as_path());

p.set_extension(
""
);
assert_eq!
(Path::new(
"/feel/the.dark"
), p.as_path());

p.set_extension(
""
);
assert_eq!
(Path::new(
"/feel/the"
), p.as_path());

p.set_extension(
""
);
assert_eq!
(Path::new(
"/feel/the"
), p.as_path());
Source
pub fn
add_extension
<S:
AsRef
<
OsStr
>>(&mut self, extension: S) ->
bool
🔬
This is a nightly-only experimental API. (
path_add_extension
#127292
)
Append
self.extension
with
extension
.
Returns
false
and does nothing if
self.file_name
is
None
,
returns
true
and updates the extension otherwise.
§
Caveats
The appended
extension
may contain dots and will be used in its entirety,
but only the part after the final dot will be reflected in
self.extension
.
See the examples below.
§
Examples
#![feature(path_add_extension)]
use
std::path::{Path, PathBuf};
let
mut
p = PathBuf::from(
"/feel/the"
);

p.add_extension(
"formatted"
);
assert_eq!
(Path::new(
"/feel/the.formatted"
), p.as_path());

p.add_extension(
"dark.side"
);
assert_eq!
(Path::new(
"/feel/the.formatted.dark.side"
), p.as_path());

p.set_extension(
"cookie"
);
assert_eq!
(Path::new(
"/feel/the.formatted.dark.cookie"
), p.as_path());

p.set_extension(
""
);
assert_eq!
(Path::new(
"/feel/the.formatted.dark"
), p.as_path());

p.add_extension(
""
);
assert_eq!
(Path::new(
"/feel/the.formatted.dark"
), p.as_path());
1.70.0
·
Source
pub fn
as_mut_os_string
(&mut self) -> &mut
OsString
Yields a mutable reference to the underlying
OsString
instance.
§
Examples
use
std::path::{Path, PathBuf};
let
mut
path = PathBuf::from(
"/foo"
);

path.push(
"bar"
);
assert_eq!
(path, Path::new(
"/foo/bar"
));
// OsString's `push` does not add a separator.
path.as_mut_os_string().push(
"baz"
);
assert_eq!
(path, Path::new(
"/foo/barbaz"
));
1.0.0
·
Source
pub fn
into_os_string
(self) ->
OsString
Consumes the
PathBuf
, yielding its internal
OsString
storage.
§
Examples
use
std::path::PathBuf;
let
p = PathBuf::from(
"/the/head"
);
let
os_str = p.into_os_string();
1.20.0
·
Source
pub fn
into_boxed_path
(self) ->
Box
<
Path
>
Converts this
PathBuf
into a
boxed
Path
.
1.44.0
·
Source
pub fn
capacity
(&self) ->
usize
Invokes
capacity
on the underlying instance of
OsString
.
1.44.0
·
Source
pub fn
clear
(&mut self)
Invokes
clear
on the underlying instance of
OsString
.
1.44.0
·
Source
pub fn
reserve
(&mut self, additional:
usize
)
Invokes
reserve
on the underlying instance of
OsString
.
1.63.0
·
Source
pub fn
try_reserve
(&mut self, additional:
usize
) ->
Result
<
()
,
TryReserveError
>
Invokes
try_reserve
on the underlying instance of
OsString
.
1.44.0
·
Source
pub fn
reserve_exact
(&mut self, additional:
usize
)
Invokes
reserve_exact
on the underlying instance of
OsString
.
1.63.0
·
Source
pub fn
try_reserve_exact
(
    &mut self,
    additional:
usize
,
) ->
Result
<
()
,
TryReserveError
>
Invokes
try_reserve_exact
on the underlying instance of
OsString
.
1.44.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Invokes
shrink_to_fit
on the underlying instance of
OsString
.
1.56.0
·
Source
pub fn
shrink_to
(&mut self, min_capacity:
usize
)
Invokes
shrink_to
on the underlying instance of
OsString
.
Methods from
Deref
<Target =
Path
>
§
1.0.0
·
Source
pub fn
as_os_str
(&self) -> &
OsStr
Yields the underlying
OsStr
slice.
§
Examples
use
std::path::Path;
let
os_str = Path::new(
"foo.txt"
).as_os_str();
assert_eq!
(os_str, std::ffi::OsStr::new(
"foo.txt"
));
1.70.0
·
Source
pub fn
as_mut_os_str
(&mut self) -> &mut
OsStr
Yields a mutable reference to the underlying
OsStr
slice.
§
Examples
use
std::path::{Path, PathBuf};
let
mut
path = PathBuf::from(
"Foo.TXT"
);
assert_ne!
(path, Path::new(
"foo.txt"
));

path.as_mut_os_str().make_ascii_lowercase();
assert_eq!
(path, Path::new(
"foo.txt"
));
1.0.0
·
Source
pub fn
to_str
(&self) ->
Option
<&
str
>
Yields a
&str
slice if the
Path
is valid unicode.
This conversion may entail doing a check for UTF-8 validity.
Note that validation is performed because non-UTF-8 strings are
perfectly valid for some OS.
§
Examples
use
std::path::Path;
let
path = Path::new(
"foo.txt"
);
assert_eq!
(path.to_str(),
Some
(
"foo.txt"
));
1.0.0
·
Source
pub fn
to_string_lossy
(&self) ->
Cow
<'_,
str
>
Converts a
Path
to a
Cow<str>
.
Any non-UTF-8 sequences are replaced with
U+FFFD REPLACEMENT CHARACTER
.
§
Examples
Calling
to_string_lossy
on a
Path
with valid unicode:
use
std::path::Path;
let
path = Path::new(
"foo.txt"
);
assert_eq!
(path.to_string_lossy(),
"foo.txt"
);
Had
path
contained invalid unicode, the
to_string_lossy
call might
have returned
"fo�.txt"
.
1.0.0
·
Source
pub fn
to_path_buf
(&self) ->
PathBuf
Converts a
Path
to an owned
PathBuf
.
§
Examples
use
std::path::{Path, PathBuf};
let
path_buf = Path::new(
"foo.txt"
).to_path_buf();
assert_eq!
(path_buf, PathBuf::from(
"foo.txt"
));
1.0.0
·
Source
pub fn
is_absolute
(&self) ->
bool
Returns
true
if the
Path
is absolute, i.e., if it is independent of
the current directory.
On Unix, a path is absolute if it starts with the root, so
is_absolute
and
has_root
are equivalent.
On Windows, a path is absolute if it has a prefix and starts with the
root:
c:\windows
is absolute, while
c:temp
and
\temp
are not.
§
Examples
use
std::path::Path;
assert!
(!Path::new(
"foo.txt"
).is_absolute());
1.0.0
·
Source
pub fn
is_relative
(&self) ->
bool
Returns
true
if the
Path
is relative, i.e., not absolute.
See
is_absolute
’s documentation for more details.
§
Examples
use
std::path::Path;
assert!
(Path::new(
"foo.txt"
).is_relative());
1.0.0
·
Source
pub fn
has_root
(&self) ->
bool
Returns
true
if the
Path
has a root.
On Unix, a path has a root if it begins with
/
.
On Windows, a path has a root if it:
has no prefix and begins with a separator, e.g.,
\windows
has a prefix followed by a separator, e.g.,
c:\windows
but not
c:windows
has any non-disk prefix, e.g.,
\\server\share
§
Examples
use
std::path::Path;
assert!
(Path::new(
"/etc/passwd"
).has_root());
1.0.0
·
Source
pub fn
parent
(&self) ->
Option
<&
Path
>
Returns the
Path
without its final component, if there is one.
This means it returns
Some("")
for relative paths with one component.
Returns
None
if the path terminates in a root or prefix, or if it’s
the empty string.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/foo/bar"
);
let
parent = path.parent().unwrap();
assert_eq!
(parent, Path::new(
"/foo"
));
let
grand_parent = parent.parent().unwrap();
assert_eq!
(grand_parent, Path::new(
"/"
));
assert_eq!
(grand_parent.parent(),
None
);
let
relative_path = Path::new(
"foo/bar"
);
let
parent = relative_path.parent();
assert_eq!
(parent,
Some
(Path::new(
"foo"
)));
let
grand_parent = parent.and_then(Path::parent);
assert_eq!
(grand_parent,
Some
(Path::new(
""
)));
let
great_grand_parent = grand_parent.and_then(Path::parent);
assert_eq!
(great_grand_parent,
None
);
1.28.0
·
Source
pub fn
ancestors
(&self) ->
Ancestors
<'_>
ⓘ
Produces an iterator over
Path
and its ancestors.
The iterator will yield the
Path
that is returned if the
parent
method is used zero
or more times. If the
parent
method returns
None
, the iterator will do likewise.
The iterator will always yield at least one value, namely
Some(&self)
. Next it will yield
&self.parent()
,
&self.parent().and_then(Path::parent)
and so on.
§
Examples
use
std::path::Path;
let
mut
ancestors = Path::new(
"/foo/bar"
).ancestors();
assert_eq!
(ancestors.next(),
Some
(Path::new(
"/foo/bar"
)));
assert_eq!
(ancestors.next(),
Some
(Path::new(
"/foo"
)));
assert_eq!
(ancestors.next(),
Some
(Path::new(
"/"
)));
assert_eq!
(ancestors.next(),
None
);
let
mut
ancestors = Path::new(
"../foo/bar"
).ancestors();
assert_eq!
(ancestors.next(),
Some
(Path::new(
"../foo/bar"
)));
assert_eq!
(ancestors.next(),
Some
(Path::new(
"../foo"
)));
assert_eq!
(ancestors.next(),
Some
(Path::new(
".."
)));
assert_eq!
(ancestors.next(),
Some
(Path::new(
""
)));
assert_eq!
(ancestors.next(),
None
);
1.0.0
·
Source
pub fn
file_name
(&self) ->
Option
<&
OsStr
>
Returns the final component of the
Path
, if there is one.
If the path is a normal file, this is the file name. If it’s the path of a directory, this
is the directory name.
Returns
None
if the path terminates in
..
.
§
Examples
use
std::path::Path;
use
std::ffi::OsStr;
assert_eq!
(
Some
(OsStr::new(
"bin"
)), Path::new(
"/usr/bin/"
).file_name());
assert_eq!
(
Some
(OsStr::new(
"foo.txt"
)), Path::new(
"tmp/foo.txt"
).file_name());
assert_eq!
(
Some
(OsStr::new(
"foo.txt"
)), Path::new(
"foo.txt/."
).file_name());
assert_eq!
(
Some
(OsStr::new(
"foo.txt"
)), Path::new(
"foo.txt/.//"
).file_name());
assert_eq!
(
None
, Path::new(
"foo.txt/.."
).file_name());
assert_eq!
(
None
, Path::new(
"/"
).file_name());
1.7.0
·
Source
pub fn
strip_prefix
<P>(&self, base: P) ->
Result
<&
Path
,
StripPrefixError
>
where
    P:
AsRef
<
Path
>,
Returns a path that, when joined onto
base
, yields
self
.
§
Errors
If
base
is not a prefix of
self
(i.e.,
starts_with
returns
false
), returns
Err
.
§
Examples
use
std::path::{Path, PathBuf};
let
path = Path::new(
"/test/haha/foo.txt"
);
assert_eq!
(path.strip_prefix(
"/"
),
Ok
(Path::new(
"test/haha/foo.txt"
)));
assert_eq!
(path.strip_prefix(
"/test"
),
Ok
(Path::new(
"haha/foo.txt"
)));
assert_eq!
(path.strip_prefix(
"/test/"
),
Ok
(Path::new(
"haha/foo.txt"
)));
assert_eq!
(path.strip_prefix(
"/test/haha/foo.txt"
),
Ok
(Path::new(
""
)));
assert_eq!
(path.strip_prefix(
"/test/haha/foo.txt/"
),
Ok
(Path::new(
""
)));
assert!
(path.strip_prefix(
"test"
).is_err());
assert!
(path.strip_prefix(
"/te"
).is_err());
assert!
(path.strip_prefix(
"/haha"
).is_err());
let
prefix = PathBuf::from(
"/test/"
);
assert_eq!
(path.strip_prefix(prefix),
Ok
(Path::new(
"haha/foo.txt"
)));
1.0.0
·
Source
pub fn
starts_with
<P:
AsRef
<
Path
>>(&self, base: P) ->
bool
Determines whether
base
is a prefix of
self
.
Only considers whole path components to match.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/etc/passwd"
);
assert!
(path.starts_with(
"/etc"
));
assert!
(path.starts_with(
"/etc/"
));
assert!
(path.starts_with(
"/etc/passwd"
));
assert!
(path.starts_with(
"/etc/passwd/"
));
// extra slash is okay
assert!
(path.starts_with(
"/etc/passwd///"
));
// multiple extra slashes are okay
assert!
(!path.starts_with(
"/e"
));
assert!
(!path.starts_with(
"/etc/passwd.txt"
));
assert!
(!Path::new(
"/etc/foo.rs"
).starts_with(
"/etc/foo"
));
1.0.0
·
Source
pub fn
ends_with
<P:
AsRef
<
Path
>>(&self, child: P) ->
bool
Determines whether
child
is a suffix of
self
.
Only considers whole path components to match.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/etc/resolv.conf"
);
assert!
(path.ends_with(
"resolv.conf"
));
assert!
(path.ends_with(
"etc/resolv.conf"
));
assert!
(path.ends_with(
"/etc/resolv.conf"
));
assert!
(!path.ends_with(
"/resolv.conf"
));
assert!
(!path.ends_with(
"conf"
));
// use .extension() instead
1.0.0
·
Source
pub fn
file_stem
(&self) ->
Option
<&
OsStr
>
Extracts the stem (non-extension) portion of
self.file_name
.
The stem is:
None
, if there is no file name;
The entire file name if there is no embedded
.
;
The entire file name if the file name begins with
.
and has no other
.
s within;
Otherwise, the portion of the file name before the final
.
§
Examples
use
std::path::Path;
assert_eq!
(
"foo"
, Path::new(
"foo.rs"
).file_stem().unwrap());
assert_eq!
(
"foo.tar"
, Path::new(
"foo.tar.gz"
).file_stem().unwrap());
§
See Also
This method is similar to
Path::file_prefix
, which extracts the portion of the file name
before the
first
.
Source
pub fn
file_prefix
(&self) ->
Option
<&
OsStr
>
🔬
This is a nightly-only experimental API. (
path_file_prefix
#86319
)
Extracts the prefix of
self.file_name
.
The prefix is:
None
, if there is no file name;
The entire file name if there is no embedded
.
;
The portion of the file name before the first non-beginning
.
;
The entire file name if the file name begins with
.
and has no other
.
s within;
The portion of the file name before the second
.
if the file name begins with
.
§
Examples
use
std::path::Path;
assert_eq!
(
"foo"
, Path::new(
"foo.rs"
).file_prefix().unwrap());
assert_eq!
(
"foo"
, Path::new(
"foo.tar.gz"
).file_prefix().unwrap());
§
See Also
This method is similar to
Path::file_stem
, which extracts the portion of the file name
before the
last
.
1.0.0
·
Source
pub fn
extension
(&self) ->
Option
<&
OsStr
>
Extracts the extension (without the leading dot) of
self.file_name
, if possible.
The extension is:
None
, if there is no file name;
None
, if there is no embedded
.
;
None
, if the file name begins with
.
and has no other
.
s within;
Otherwise, the portion of the file name after the final
.
§
Examples
use
std::path::Path;
assert_eq!
(
"rs"
, Path::new(
"foo.rs"
).extension().unwrap());
assert_eq!
(
"gz"
, Path::new(
"foo.tar.gz"
).extension().unwrap());
1.0.0
·
Source
pub fn
join
<P:
AsRef
<
Path
>>(&self, path: P) ->
PathBuf
Creates an owned
PathBuf
with
path
adjoined to
self
.
If
path
is absolute, it replaces the current path.
See
PathBuf::push
for more details on what it means to adjoin a path.
§
Examples
use
std::path::{Path, PathBuf};
assert_eq!
(Path::new(
"/etc"
).join(
"passwd"
), PathBuf::from(
"/etc/passwd"
));
assert_eq!
(Path::new(
"/etc"
).join(
"/bin/sh"
), PathBuf::from(
"/bin/sh"
));
1.0.0
·
Source
pub fn
with_file_name
<S:
AsRef
<
OsStr
>>(&self, file_name: S) ->
PathBuf
Creates an owned
PathBuf
like
self
but with the given file name.
See
PathBuf::set_file_name
for more details.
§
Examples
use
std::path::{Path, PathBuf};
let
path = Path::new(
"/tmp/foo.png"
);
assert_eq!
(path.with_file_name(
"bar"
), PathBuf::from(
"/tmp/bar"
));
assert_eq!
(path.with_file_name(
"bar.txt"
), PathBuf::from(
"/tmp/bar.txt"
));
let
path = Path::new(
"/tmp"
);
assert_eq!
(path.with_file_name(
"var"
), PathBuf::from(
"/var"
));
1.0.0
·
Source
pub fn
with_extension
<S:
AsRef
<
OsStr
>>(&self, extension: S) ->
PathBuf
Creates an owned
PathBuf
like
self
but with the given extension.
See
PathBuf::set_extension
for more details.
§
Examples
use
std::path::{Path, PathBuf};
let
path = Path::new(
"foo.rs"
);
assert_eq!
(path.with_extension(
"txt"
), PathBuf::from(
"foo.txt"
));
let
path = Path::new(
"foo.tar.gz"
);
assert_eq!
(path.with_extension(
""
), PathBuf::from(
"foo.tar"
));
assert_eq!
(path.with_extension(
"xz"
), PathBuf::from(
"foo.tar.xz"
));
assert_eq!
(path.with_extension(
""
).with_extension(
"txt"
), PathBuf::from(
"foo.txt"
));
Source
pub fn
with_added_extension
<S:
AsRef
<
OsStr
>>(&self, extension: S) ->
PathBuf
🔬
This is a nightly-only experimental API. (
path_add_extension
#127292
)
Creates an owned
PathBuf
like
self
but with the extension added.
See
PathBuf::add_extension
for more details.
§
Examples
#![feature(path_add_extension)]
use
std::path::{Path, PathBuf};
let
path = Path::new(
"foo.rs"
);
assert_eq!
(path.with_added_extension(
"txt"
), PathBuf::from(
"foo.rs.txt"
));
let
path = Path::new(
"foo.tar.gz"
);
assert_eq!
(path.with_added_extension(
""
), PathBuf::from(
"foo.tar.gz"
));
assert_eq!
(path.with_added_extension(
"xz"
), PathBuf::from(
"foo.tar.gz.xz"
));
assert_eq!
(path.with_added_extension(
""
).with_added_extension(
"txt"
), PathBuf::from(
"foo.tar.gz.txt"
));
1.0.0
·
Source
pub fn
components
(&self) ->
Components
<'_>
ⓘ
Produces an iterator over the
Component
s of the path.
When parsing the path, there is a small amount of normalization:
Repeated separators are ignored, so
a/b
and
a//b
both have
a
and
b
as components.
Occurrences of
.
are normalized away, except if they are at the
beginning of the path. For example,
a/./b
,
a/b/
,
a/b/.
and
a/b
all have
a
and
b
as components, but
./a/b
starts with
an additional
CurDir
component.
A trailing slash is normalized away,
/a/b
and
/a/b/
are equivalent.
Note that no other normalization takes place; in particular,
a/c
and
a/b/../c
are distinct, to account for the possibility that
b
is a symbolic link (so its parent isn’t
a
).
§
Examples
use
std::path::{Path, Component};
use
std::ffi::OsStr;
let
mut
components = Path::new(
"/tmp/foo.txt"
).components();
assert_eq!
(components.next(),
Some
(Component::RootDir));
assert_eq!
(components.next(),
Some
(Component::Normal(OsStr::new(
"tmp"
))));
assert_eq!
(components.next(),
Some
(Component::Normal(OsStr::new(
"foo.txt"
))));
assert_eq!
(components.next(),
None
)
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_>
ⓘ
Produces an iterator over the path’s components viewed as
OsStr
slices.
For more information about the particulars of how the path is separated
into components, see
components
.
§
Examples
use
std::path::{
self
, Path};
use
std::ffi::OsStr;
let
mut
it = Path::new(
"/tmp/foo.txt"
).iter();
assert_eq!
(it.next(),
Some
(OsStr::new(
&
path::MAIN_SEPARATOR.to_string())));
assert_eq!
(it.next(),
Some
(OsStr::new(
"tmp"
)));
assert_eq!
(it.next(),
Some
(OsStr::new(
"foo.txt"
)));
assert_eq!
(it.next(),
None
)
1.0.0
·
Source
pub fn
display
(&self) ->
Display
<'_>
Returns an object that implements
Display
for safely printing paths
that may contain non-Unicode data. This may perform lossy conversion,
depending on the platform.  If you would like an implementation which
escapes the path please use
Debug
instead.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/tmp/foo.rs"
);
println!
(
"{}"
, path.display());
1.5.0
·
Source
pub fn
metadata
(&self) ->
Result
<
Metadata
>
Queries the file system to get information about a file, directory, etc.
This function will traverse symbolic links to query information about the
destination file.
This is an alias to
fs::metadata
.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/Minas/tirith"
);
let
metadata = path.metadata().expect(
"metadata call failed"
);
println!
(
"{:?}"
, metadata.file_type());
1.5.0
·
Source
pub fn
symlink_metadata
(&self) ->
Result
<
Metadata
>
Queries the metadata about a file without following symlinks.
This is an alias to
fs::symlink_metadata
.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/Minas/tirith"
);
let
metadata = path.symlink_metadata().expect(
"symlink_metadata call failed"
);
println!
(
"{:?}"
, metadata.file_type());
1.5.0
·
Source
pub fn
canonicalize
(&self) ->
Result
<
PathBuf
>
Returns the canonical, absolute form of the path with all intermediate
components normalized and symbolic links resolved.
This is an alias to
fs::canonicalize
.
§
Examples
use
std::path::{Path, PathBuf};
let
path = Path::new(
"/foo/test/../test/bar.rs"
);
assert_eq!
(path.canonicalize().unwrap(), PathBuf::from(
"/foo/test/bar.rs"
));
1.5.0
·
Source
pub fn
read_link
(&self) ->
Result
<
PathBuf
>
Reads a symbolic link, returning the file that the link points to.
This is an alias to
fs::read_link
.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/laputa/sky_castle.rs"
);
let
path_link = path.read_link().expect(
"read_link call failed"
);
1.5.0
·
Source
pub fn
read_dir
(&self) ->
Result
<
ReadDir
>
Returns an iterator over the entries within a directory.
The iterator will yield instances of
io::Result
<
fs::DirEntry
>
. New
errors may be encountered after an iterator is initially constructed.
This is an alias to
fs::read_dir
.
§
Examples
use
std::path::Path;
let
path = Path::new(
"/laputa"
);
for
entry
in
path.read_dir().expect(
"read_dir call failed"
) {
if let
Ok
(entry) = entry {
println!
(
"{:?}"
, entry.path());
    }
}
1.5.0
·
Source
pub fn
exists
(&self) ->
bool
Returns
true
if the path points at an existing entity.
Warning: this method may be error-prone, consider using
try_exists()
instead!
It also has a risk of introducing time-of-check to time-of-use (TOCTOU) bugs.
This function will traverse symbolic links to query information about the
destination file.
If you cannot access the metadata of the file, e.g. because of a
permission error or broken symbolic links, this will return
false
.
§
Examples
use
std::path::Path;
assert!
(!Path::new(
"does_not_exist.txt"
).exists());
§
See Also
This is a convenience function that coerces errors to false. If you want to
check errors, call
Path::try_exists
.
1.63.0
·
Source
pub fn
try_exists
(&self) ->
Result
<
bool
>
Returns
Ok(true)
if the path points at an existing entity.
This function will traverse symbolic links to query information about the
destination file. In case of broken symbolic links this will return
Ok(false)
.
Path::exists()
only checks whether or not a path was both found and readable. By
contrast,
try_exists
will return
Ok(true)
or
Ok(false)
, respectively, if the path
was
verified
to exist or not exist. If its existence can neither be confirmed nor
denied, it will propagate an
Err(_)
instead. This can be the case if e.g. listing
permission is denied on one of the parent directories.
Note that while this avoids some pitfalls of the
exists()
method, it still can not
prevent time-of-check to time-of-use (TOCTOU) bugs. You should only use it in scenarios
where those bugs are not an issue.
This is an alias for
std::fs::exists
.
§
Examples
use
std::path::Path;
assert!
(!Path::new(
"does_not_exist.txt"
).try_exists().expect(
"Can't check existence of file does_not_exist.txt"
));
assert!
(Path::new(
"/root/secret_file.txt"
).try_exists().is_err());
1.5.0
·
Source
pub fn
is_file
(&self) ->
bool
Returns
true
if the path exists on disk and is pointing at a regular file.
This function will traverse symbolic links to query information about the
destination file.
If you cannot access the metadata of the file, e.g. because of a
permission error or broken symbolic links, this will return
false
.
§
Examples
use
std::path::Path;
assert_eq!
(Path::new(
"./is_a_directory/"
).is_file(),
false
);
assert_eq!
(Path::new(
"a_file.txt"
).is_file(),
true
);
§
See Also
This is a convenience function that coerces errors to false. If you want to
check errors, call
fs::metadata
and handle its
Result
. Then call
fs::Metadata::is_file
if it was
Ok
.
When the goal is simply to read from (or write to) the source, the most
reliable way to test the source can be read (or written to) is to open
it. Only using
is_file
can break workflows like
diff <( prog_a )
on
a Unix-like system for example. See
fs::File::open
or
fs::OpenOptions::open
for more information.
1.5.0
·
Source
pub fn
is_dir
(&self) ->
bool
Returns
true
if the path exists on disk and is pointing at a directory.
This function will traverse symbolic links to query information about the
destination file.
If you cannot access the metadata of the file, e.g. because of a
permission error or broken symbolic links, this will return
false
.
§
Examples
use
std::path::Path;
assert_eq!
(Path::new(
"./is_a_directory/"
).is_dir(),
true
);
assert_eq!
(Path::new(
"a_file.txt"
).is_dir(),
false
);
§
See Also
This is a convenience function that coerces errors to false. If you want to
check errors, call
fs::metadata
and handle its
Result
. Then call
fs::Metadata::is_dir
if it was
Ok
.
1.58.0
·
Source
pub fn
is_symlink
(&self) ->
bool
Returns
true
if the path exists on disk and is pointing at a symbolic link.
This function will not traverse symbolic links.
In case of a broken symbolic link this will also return true.
If you cannot access the directory containing the file, e.g., because of a
permission error, this will return false.
§
Examples
use
std::path::Path;
use
std::os::unix::fs::symlink;
let
link_path = Path::new(
"link"
);
symlink(
"/origin_does_not_exist/"
, link_path).unwrap();
assert_eq!
(link_path.is_symlink(),
true
);
assert_eq!
(link_path.exists(),
false
);
§
See Also
This is a convenience function that coerces errors to false. If you want to
check errors, call
fs::symlink_metadata
and handle its
Result
. Then call
fs::Metadata::is_symlink
if it was
Ok
.
Trait Implementations
§
1.0.0
·
Source
§
impl
AsRef
<
OsStr
> for
PathBuf
Source
§
fn
as_ref
(&self) -> &
OsStr
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
AsRef
<
Path
> for
PathBuf
Source
§
fn
as_ref
(&self) -> &
Path
Converts this type into a shared reference of the (usually inferred) input type.
1.0.0
·
Source
§
impl
Borrow
<
Path
> for
PathBuf
Source
§
fn
borrow
(&self) -> &
Path
Immutably borrows from an owned value.
Read more
1.0.0
·
Source
§
impl
Clone
for
PathBuf
Source
§
fn
clone_from
(&mut self, source: &Self)
Clones the contents of
source
into
self
.
This method is preferred over simply assigning
source.clone()
to
self
,
as it avoids reallocation if possible.
Source
§
fn
clone
(&self) -> Self
Returns a copy of the value.
Read more
1.0.0
·
Source
§
impl
Debug
for
PathBuf
Source
§
fn
fmt
(&self, formatter: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.17.0
·
Source
§
impl
Default
for
PathBuf
Source
§
fn
default
() -> Self
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl
Deref
for
PathBuf
Source
§
type
Target
=
Path
The resulting type after dereferencing.
Source
§
fn
deref
(&self) -> &
Path
Dereferences the value.
1.68.0
·
Source
§
impl
DerefMut
for
PathBuf
Source
§
fn
deref_mut
(&mut self) -> &mut
Path
Mutably dereferences the value.
1.0.0
·
Source
§
impl<P:
AsRef
<
Path
>>
Extend
<P> for
PathBuf
Source
§
fn
extend
<I:
IntoIterator
<Item = P>>(&mut self, iter: I)
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, p: P)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
Read more
1.28.0
·
Source
§
impl<'a>
From
<&'a
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
from
(p: &'a
PathBuf
) ->
Cow
<'a,
Path
>
Creates a clone-on-write pointer from a reference to
PathBuf
.
This conversion does not clone or allocate.
1.0.0
·
Source
§
impl<T: ?
Sized
+
AsRef
<
OsStr
>>
From
<
&T
> for
PathBuf
Source
§
fn
from
(s:
&T
) ->
PathBuf
Converts a borrowed
OsStr
to a
PathBuf
.
Allocates a
PathBuf
and copies the data into it.
1.18.0
·
Source
§
impl
From
<
Box
<
Path
>> for
PathBuf
Source
§
fn
from
(boxed:
Box
<
Path
>) ->
PathBuf
Converts a
Box
<
Path
>
into a
PathBuf
.
This conversion does not allocate or copy memory.
1.28.0
·
Source
§
impl<'a>
From
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
from
(p:
Cow
<'a,
Path
>) -> Self
Converts a clone-on-write pointer to an owned path.
Converting from a
Cow::Owned
does not clone or allocate.
1.0.0
·
Source
§
impl
From
<
OsString
> for
PathBuf
Source
§
fn
from
(s:
OsString
) ->
PathBuf
Converts an
OsString
into a
PathBuf
.
This conversion does not allocate or copy memory.
1.24.0
·
Source
§
impl
From
<
PathBuf
> for
Arc
<
Path
>
Source
§
fn
from
(s:
PathBuf
) ->
Arc
<
Path
>
Converts a
PathBuf
into an
Arc
<
Path
>
by moving the
PathBuf
data
into a new
Arc
buffer.
1.20.0
·
Source
§
impl
From
<
PathBuf
> for
Box
<
Path
>
Source
§
fn
from
(p:
PathBuf
) ->
Box
<
Path
>
Converts a
PathBuf
into a
Box
<
Path
>
.
This conversion currently should not allocate memory,
but this behavior is not guaranteed on all platforms or in all future versions.
1.6.0
·
Source
§
impl<'a>
From
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
from
(s:
PathBuf
) ->
Cow
<'a,
Path
>
Creates a clone-on-write pointer from an owned
instance of
PathBuf
.
This conversion does not clone or allocate.
1.14.0
·
Source
§
impl
From
<
PathBuf
> for
OsString
Source
§
fn
from
(path_buf:
PathBuf
) ->
OsString
Converts a
PathBuf
into an
OsString
This conversion does not allocate or copy memory.
1.24.0
·
Source
§
impl
From
<
PathBuf
> for
Rc
<
Path
>
Source
§
fn
from
(s:
PathBuf
) ->
Rc
<
Path
>
Converts a
PathBuf
into an
Rc
<
Path
>
by moving the
PathBuf
data into
a new
Rc
buffer.
1.0.0
·
Source
§
impl
From
<
String
> for
PathBuf
Source
§
fn
from
(s:
String
) ->
PathBuf
Converts a
String
into a
PathBuf
This conversion does not allocate or copy memory.
1.0.0
·
Source
§
impl<P:
AsRef
<
Path
>>
FromIterator
<P> for
PathBuf
Source
§
fn
from_iter
<I:
IntoIterator
<Item = P>>(iter: I) ->
PathBuf
Creates a value from an iterator.
Read more
1.32.0
·
Source
§
impl
FromStr
for
PathBuf
Source
§
type
Err
=
Infallible
The associated error which can be returned from parsing.
Source
§
fn
from_str
(s: &
str
) ->
Result
<Self, Self::
Err
>
Parses a string
s
to return a value of this type.
Read more
1.0.0
·
Source
§
impl
Hash
for
PathBuf
Source
§
fn
hash
<H:
Hasher
>(&self, h:
&mut H
)
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.6.0
·
Source
§
impl<'a>
IntoIterator
for &'a
PathBuf
Source
§
type
Item
= &'a
OsStr
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a>
ⓘ
Creates an iterator from a value.
Read more
1.0.0
·
Source
§
impl
Ord
for
PathBuf
Source
§
fn
cmp
(&self, other: &
PathBuf
) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.8.0
·
Source
§
impl<'a>
PartialEq
<&'a
OsStr
> for
PathBuf
Source
§
fn
eq
(&self, other: &&'a
OsStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl<'a>
PartialEq
<&'a
Path
> for
PathBuf
Source
§
fn
eq
(&self, other: &&'a
Path
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
OsStr
>> for
PathBuf
Source
§
fn
eq
(&self, other: &
Cow
<'a,
OsStr
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl<'a>
PartialEq
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
eq
(&self, other: &
Cow
<'a,
Path
>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl
PartialEq
<
OsStr
> for
PathBuf
Source
§
fn
eq
(&self, other: &
OsStr
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl
PartialEq
<
OsString
> for
PathBuf
Source
§
fn
eq
(&self, other: &
OsString
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl
PartialEq
<
Path
> for
PathBuf
Source
§
fn
eq
(&self, other: &
Path
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for &'a
OsStr
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for &'a
Path
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
OsStr
>
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl<'a>
PartialEq
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
OsStr
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
OsString
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.6.0
·
Source
§
impl
PartialEq
<
PathBuf
> for
Path
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl
PartialEq
for
PathBuf
Source
§
fn
eq
(&self, other: &
PathBuf
) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
OsStr
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &&'a
OsStr
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<&'a
Path
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &&'a
Path
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
OsStr
>> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
OsStr
>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
Cow
<'a,
Path
>> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
Cow
<'a,
Path
>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
OsStr
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
OsStr
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
OsString
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
OsString
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
Path
> for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
Path
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for &'a
OsStr
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for &'a
Path
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
OsStr
>
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl<'a>
PartialOrd
<
PathBuf
> for
Cow
<'a,
Path
>
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
OsStr
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
OsString
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.8.0
·
Source
§
impl
PartialOrd
<
PathBuf
> for
Path
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.0.0
·
Source
§
impl
PartialOrd
for
PathBuf
Source
§
fn
partial_cmp
(&self, other: &
PathBuf
) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.0.0
·
Source
§
impl
Eq
for
PathBuf
Auto Trait Implementations
§
§
impl
Freeze
for
PathBuf
§
impl
RefUnwindSafe
for
PathBuf
§
impl
Send
for
PathBuf
§
impl
Sync
for
PathBuf
§
impl
Unpin
for
PathBuf
§
impl
UnwindSafe
for
PathBuf
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.