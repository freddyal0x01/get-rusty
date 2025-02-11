# Advanced Functions and Closures

## Function Pointers

We can pass functions into functions. This technique is useful when you want to
pass a function you’ve already defined rather than defining a new closure.

Functions coerce to the type `fn` (with a lowercase f), not to be confused with
the `Fn` closure trait. The `fn` type is called a _function pointer_. Passing
functions with function pointers will allow you to use functions as arguments to
other functions.

Example code: Using the `fn` type to accept a function pointer as an argument

```rust
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
```

Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), meaning you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either functions
or closures.

One example of where you would want to only accept `fn` and not closures is when
interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures.

## Returning Closures

Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function pointer
fn as a return type

The following code tries to return a closure directly, but it won’t compile:

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

The error references the Sized trait again! Rust doesn’t know how much space it
will need to store the closure.

We can use the `Box` trait object to return a heap allocated closure:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
