//! This compiler is made to parse macros with custom syntax written in userspace,
//! reduce the macros to lambda calculus, and for each macro left unevaluated,
//! add another parameter to the whole program, allowing for (functional!) bindings.
//! 
//! Because we'll be discussing both the semantics of metaprogramming (macros to construct syntax)
//! and the semantics of the program generated,it may become confusing at times when I'm using
//! in-language concepts, and when I'm using out-of-language concepts to describe the expected
//! behavior.
//! 
//! To solve this, we'll use some unusual conventions. 
//!     - Typing:       instead of `a: T`, we use `a ~~ T`
//!     - Return:       instead of `f: A -> B`, we use `a $ (A) ~> B`
//!     - Assignment:   instead of `a = b;` we use `a <~ b`
//! 
//! Types:
//!     1. Identifier
//!     2. MacroIdentifier
//!     3. Unparsed
//!     4. Term
//! 
//! It starts with three macros:
//!     1. `_: _= _; _`, where `_:_=_;_ ~~ (a: t = v; rest) ~> c, where:
//!          - a is an identifier
//!          - t is Type
//!          - c is Syntax
//!          - rest is Unparsed.
//!         This defines a macro with the name before the colon, which has the type after the colon.
//!         It has the value after the equals sign, which may be either Unparsed or may be a Term 
//!     2. `Drop _; _`, where `Drop_ ~~ (Drop Identifier ; Unparsed)`
//!     
//!     3. 

fn main() {
    println!("Hello, world!");
}
