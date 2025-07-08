Advanced Functions and Closures - The Rust Programming Language
Light
Rust
Coal
Navy
Ayu
The Rust Programming Language
Advanced Functions and Closures
This section explores some advanced features related to functions and closures,
including function pointers and returning closures.
Function Pointers
We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Functions
coerce to the type
fn
(with a lowercase
f
), not to be confused with the
Fn
closure trait. The
fn
type is called a
function pointer
. Passing functions
with function pointers will allow you to use functions as arguments to other
functions.
The syntax for specifying that a parameter is a function pointer is similar to
that of closures, as shown in Listing 20-28, where we’ve defined a function
add_one
that adds 1 to its parameter. The function
do_twice
takes two
parameters: a function pointer to any function that takes an
i32
parameter
and returns an
i32
, and one
i32
value. The
do_twice
function calls the
function
f
twice, passing it the
arg
value, then adds the two function call
results together. The
main
function calls
do_twice
with the arguments
add_one
and
5
.
Filename: src/main.rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}
Listing 20-28
: Using the
fn
type to accept a function pointer as an argument
This code prints
The answer is: 12
. We specify that the parameter
f
in
do_twice
is an
fn
that takes one parameter of type
i32
and returns an
i32
. We can then call
f
in the body of
do_twice
. In
main
, we can pass
the function name
add_one
as the first argument to
do_twice
.
Unlike closures,
fn
is a type rather than a trait, so we specify
fn
as the
parameter type directly rather than declaring a generic type parameter with one
of the
Fn
traits as a trait bound.
Function pointers implement all three of the closure traits (
Fn
,
FnMut
, and
FnOnce
), meaning you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures.
That said, one example of where you would want to only accept
fn
and not
closures is when interfacing with external code that doesn’t have closures: C
functions can accept functions as arguments, but C doesn’t have closures.
As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of the
map
method provided by the
Iterator
trait in the standard library. To use the
map
method to turn a vector of
numbers into a vector of strings, we could use a closure, as in Listing 20-29.
fn main() {
let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
}
Listing 20-29
: Using a closure with the
map
method to convert numbers to strings
Or we could name a function as the argument to map instead of the closure.
Listing 20-30 shows what this would look like.
fn main() {
let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
}
Listing 20-30
: Using the
String::to_string
method to convert numbers to strings
Note that we must use the fully qualified syntax that we talked about in
“Advanced Traits”
because there are multiple
functions available named
to_string
.
Here, we’re using the
to_string
function defined in the
ToString
trait,
which the standard library has implemented for any type that implements
Display
.
Recall from
“Enum values”
in Chapter 6 that the
name of each enum variant that we define also becomes an initializer function.
We can use these initializer functions as function pointers that implement the
closure traits, which means we can specify the initializer functions as
arguments for methods that take closures, as seen in Listing 20-31.
fn main() {
enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
Listing 20-31
: Using an enum initializers with the
map
method to create a
Status
instance from numbers
Here we create
Status::Value
instances using each
u32
value in the range
that
map
is called on by using the initializer function of
Status::Value
.
Some people prefer this style and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.
Returning Closures
Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can’t usually do that with closures because they don’t
have a concrete type that is returnable. You’re not allowed to use the function
pointer
fn
as a return type if the closure captures any values from its scope,
for example.
Instead, you will normally use the
impl Trait
syntax we learned about in
Chapter 10. You can return any function type, using
Fn
,
FnOnce
and
FnMut
.
For example, the code in Listing 20-32 will work just fine.
#![allow(unused)]
fn main() {
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
}
Listing 20-32
: Returning a closure from a function using the
impl Trait
syntax
However, as we noted in
“Closure Type Inference and
Annotation”
in Chapter 13, each closure is also
its own distinct type. If you need to work with multiple functions that have the
same signature but different implementations, you will need to use a trait
object for them. Consider what happens if you write code like that shown in
Listing 20-33.
Filename: src/main.rs
fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
Listing 20-33
: Creating a
Vec<T>
of closures defined by functions that return
impl Fn
Here we have two functions,
returns_closure
and
returns_initialized_closure
,
which both return
impl Fn(i32) -> i32
. Notice that he closures that they
return are different, even though they implement the same type. If we try to
compile this, Rust lets us know that it won’t work:
$ cargo build
   Compiling functions-example v0.1.0 (file:///projects/functions-example)
    error[E0308]: mismatched types
    --> src/main.rs:4:9
    |
    4  |         returns_initialized_closure(123)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
    ...
    12 | fn returns_closure() -> impl Fn(i32) -> i32 {
    |                         ------------------- the expected opaque type
    ...
    16 | fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    |                                              ------------------- the found opaque type
    |
    = note: expected opaque type `impl Fn(i32) -> i32` (opaque type at <src/main.rs:12:25>)
                found opaque type `impl Fn(i32) -> i32` (opaque type at <src/main.rs:16:46>)
    = note: distinct uses of `impl Trait` result in different opaque types

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `functions-example` (bin "functions-example") due to 1 previous error
The error message tells us that whenever we return an
impl Trait
Rust creates
a unique
opaque type
, a type where we cannot see into the details of what Rust
constructs for us. So even though these functions both return closures that
implements the same trait,
Fn(i32) -> i32
, the opaque types Rust generates for
each are distinct. (This is similar to how Rust produces different concrete
types for distinct async blocks even when they have the same output type, as we
saw in
“Working with Any Number of Futures”
in Chapter
17. We have seen a solution to this problem a few times now: we can use a trait
object, as in Listing 20-34.
fn main() {
let handlers = vec![returns_closure(), returns_initialized_closure(123)];
for handler in handlers {
let output = handler(5);
println!("{output}");
}
}
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
Listing 20-34
: Creating a
Vec<T>
of closures defined by functions that return
Box<dyn Fn>
so they have the same type
This code will compile just fine. For more about trait objects, refer to the
section
“Using Trait Objects That Allow for Values of Different
Types”
in Chapter 18.
Next, let’s look at macros!