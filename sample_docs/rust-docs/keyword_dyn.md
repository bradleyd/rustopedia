dyn - Rust
Keyword
dyn
Copy item path
Source
Expand description
dyn
is a prefix of a
trait object
’s type.
The
dyn
keyword is used to highlight that calls to methods on the associated
Trait
are
dynamically dispatched
. To use the trait this way, it must be
dyn compatible
1
.
Unlike generic parameters or
impl Trait
, the compiler does not know the concrete type that
is being passed. That is, the type has been
erased
.
As such, a
dyn Trait
reference contains
two
pointers.
One pointer goes to the data (e.g., an instance of a struct).
Another pointer goes to a map of method call names to function pointers
(known as a virtual method table or vtable).
At run-time, when a method needs to be called on the
dyn Trait
, the vtable is consulted to get
the function pointer and then that function pointer is called.
See the Reference for more information on
trait objects
and
dyn compatibility
.
§
Trade-offs
The above indirection is the additional runtime cost of calling a function on a
dyn Trait
.
Methods called by dynamic dispatch generally cannot be inlined by the compiler.
However,
dyn Trait
is likely to produce smaller code than
impl Trait
/ generic parameters as
the method won’t be duplicated for each concrete type.
Formerly known as
object safe
.
↩