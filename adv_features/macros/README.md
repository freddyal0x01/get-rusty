# Macros

The term _macro_ refers to a family of features in Rust: _declarative_ macros with `macro_rules!` and three kinds of _procedural_ macros:

- Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument

## The Difference Between Macros and Functions

Macros are a way of writing code that writes other code, which is known as _metaprogramming_.

The `derive` attribute, which generates an implementation of various traits for you. There are 2 other common macros as well that are commonly used: `println!` and `vec!`.

Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions. However, macros have some additional powers that functions don’t.A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters: we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments. Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code. Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.

Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

# Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust is the _declarative macro_. These are also sometimes referred to as “macros by example,” “`macro_rules!` macros,” or just plain “macros.” At their core, declarative macros allow you to write something similar to a Rust match expression.

To define a macro, you use the `macro_rules!` construct. 

Example code: 

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

vec! library example: 

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

We then start the macro definition with `macro_rules!` and the name of the macro we’re defining without the exclamation mark.

Valid pattern syntax in macro definitions is different than the pattern syntax because macro patterns are matched against Rust code structure rather than values.

First, we use a set of parentheses to encompass the whole pattern. We use a dollar sign (`$`) to declare a variable in the macro system that will contain the Rust code matching the pattern. The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable.

Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code. Within `$()` is `$x:expr`, which matches any Rust expression and gives the expression the name `$x`.

The comma following `$()` indicates that a literal comma separator character could optionally appear after the code that matches the code in `$()`. The `*` specifies that the pattern matches zero or more of whatever precedes the `*`.

# Procedural Macros for Generating Code from Attributes

Procedural macros act more like a function (and is a type of procedure). Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.

There are 3 kinds of procedural macros and they all work in a similar fashion: 

1. custom derive
2. attribute-like
3. function-like

When creating procedural macros, the definitions must reside in their own crate with a special crate type. This is for complex technical reasons that we hope to eliminate in the future.

Example: How to define a procedural macro

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

Tokens are the smallest individual elements of a program, they can represent keywords, identifiers, operators, separators, or literals. The function must have an attribute which specifies the kind of procedural macro we're creating.

## How to write a Custom `derive` Macro

Example Steps: 
1. Create a crate named `hello_macro` that defines a trait named `HelloMacro` with one associated function named `hello_macro`.
   - `cargo new hello_macro --lib`
2. Provide a procedural macro so users can annotate their type with `#[derive(HelloMacro)]` to get a default implementation of the `hello_macro` function.
   - Define a procedural macro: `cargo new hello_macro_derive --lib`. Procedural macros need to be in their own crate. 

The default implementation will print `Hello, Macro! My name is TypeName!` where `TypeName` is the name of the type on which this trait has been defined. 

Example code using the created procedural macro

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

When creating a procedural macro the convention for structuring crates and macro crates is as follows: for a crate named `foo`, a custom derive procedural macro crate is called `foo_derive`.


We need to declare the `hello_macro_derive` crate as a procedural macro crate. We’ll also need functionality from the `syn` and `quote` crates, we need to add them as dependencies. 

Add this under the Cargo.toml

```toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

When define the procedural macro, the code won't compile until we add a definition for the `impl_hello_macro` function.

## Attribute-like macros

Attribute-like macros are similar to custom derive macros, but instead of generating code for the `derive` attribute, they allow you to create new attributes. *They’re also more flexible: `derive` only works for structs and enums; attributes can be applied to other items as well, such as functions.*

Code example using an attribute-like macro: We have an attribute named 'route' that annotates functions when using a web app framework. 

```rust
#[route(GET, "/")]
fn index() {}
```

The `#[route]` attribute would be defined by the framework as a procedural macro. The signature would look like this: 

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

Here, we have two parameters of type `TokenStream`. The first is for the contents of the attribute: the `GET, "/"` part. The second is the body of the item the attribute is attached to: in this case, `fn index() {}` and the rest of the function’s body.

Other than that, attribute-like macros work the same way as custom derive macros: yweou create a crate with the `proc-macro` crate type and implement a function that generates the code you want.


## Function-like macros

Function-like macros define macros that look like function calls. Similarly to `macro_rules!` macros, they’re more flexible than functions. Example: they can take an unknown number of arguments. However, `macro_rules!` macros can be defined only using the match-like syntax

Function-like macros take a `TokenStream` parameter and their definition manipulates that `TokenStream` using Rust code as the other two types of procedural macros do.

Code Example: function-like macros is a `sql!` macro that might be called like a function.

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a `macro_rules!` macro can do. 

The `sql!` macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}
```
This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.