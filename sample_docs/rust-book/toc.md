Foreword
Introduction
1.
Getting Started
1.1.
Installation
1.2.
Hello, World!
1.3.
Hello, Cargo!
2.
Programming a Guessing Game
3.
Common Programming Concepts
3.1.
Variables and Mutability
3.2.
Data Types
3.3.
How Functions Work
3.4.
Comments
3.5.
Control Flow
4.
Understanding Ownership
4.1.
What is Ownership?
4.2.
References & Borrowing
4.3.
Slices
5.
Using Structs to Structure Related Data
5.1.
Defining and Instantiating Structs
5.2.
An Example Program Using Structs
5.3.
Method Syntax
6.
Enums and Pattern Matching
6.1.
Defining an Enum
6.2.
The match Control Flow Operator
6.3.
Concise Control Flow with if let
7.
Modules
7.1.
mod and the Filesystem
7.2.
Controlling Visibility with pub
7.3.
Referring to Names in Different Modules
8.
Common Collections
8.1.
Vectors
8.2.
Strings
8.3.
Hash Maps
9.
Error Handling
9.1.
Unrecoverable Errors with panic!
9.2.
Recoverable Errors with Result
9.3.
To panic! or Not to panic!
10.
Generic Types, Traits, and Lifetimes
10.1.
Generic Data Types
10.2.
Traits: Defining Shared Behavior
10.3.
Validating References with Lifetimes
11.
Testing
11.1.
Writing tests
11.2.
Running tests
11.3.
Test Organization
12.
An I/O Project: Building a Command Line Program
12.1.
Accepting Command Line Arguments
12.2.
Reading a File
12.3.
Refactoring to Improve Modularity and Error Handling
12.4.
Developing the Library’s Functionality with Test Driven Development
12.5.
Working with Environment Variables
12.6.
Writing Error Messages to Standard Error Instead of Standard Output
13.
Functional Language Features: Iterators and Closures
13.1.
Closures: Anonymous Functions that Can Capture Their Environment
13.2.
Processing a Series of Items with Iterators
13.3.
Improving Our I/O Project
13.4.
Comparing Performance: Loops vs. Iterators
14.
More about Cargo and Crates.io
14.1.
Customizing Builds with Release Profiles
14.2.
Publishing a Crate to Crates.io
14.3.
Cargo Workspaces
14.4.
Installing Binaries from Crates.io with cargo install
14.5.
Extending Cargo with Custom Commands
15.
Smart Pointers
15.1.
Box<T> Points to Data on the Heap and Has a Known Size
15.2.
The Deref Trait Allows Access to the Data Through a Reference
15.3.
The Drop Trait Runs Code on Cleanup
15.4.
Rc<T>, the Reference Counted Smart Pointer
15.5.
RefCell<T> and the Interior Mutability Pattern
15.6.
Creating Reference Cycles and Leaking Memory is Safe
16.
Fearless Concurrency
16.1.
Threads
16.2.
Message Passing
16.3.
Shared State
16.4.
Extensible Concurrency: Sync and Send
17.
Object Oriented Programming Features of Rust
17.1.
Characteristics of Object-Oriented Languages
17.2.
Using Trait Objects that Allow for Values of Different Types
17.3.
Implementing an Object-Oriented Design Pattern
18.
Patterns Match the Structure of Values
18.1.
All the Places Patterns May be Used
18.2.
Refutability: Whether a Pattern Might Fail to Match
18.3.
All the Pattern Syntax
19.
Advanced Features
19.1.
Unsafe Rust
19.2.
Advanced Lifetimes
19.3.
Advanced Traits
19.4.
Advanced Types
19.5.
Advanced Functions & Closures
20.
Final Project: Building a Multithreaded Web Server
20.1.
A Single Threaded Web Server
20.2.
Turning our Single Threaded Server into a Multithreaded Server
20.3.
Graceful Shutdown and Cleanup
21.
Appendix
21.1.
A - Keywords
21.2.
B - Operators and Symbols
21.3.
C - Derivable Traits
21.4.
D - Macros
21.5.
E - Translations
21.6.
F - Newest Features
21.7.
G - How Rust is Made and “Nightly Rust”