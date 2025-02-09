// The Deref trait allows you to treat pointers like regular references.
// It also allows you to customize the behavior of the dereference operator *

// The deref trait allows the compiler to take any value that implements deref
// call the deref method to get a reference which the compiler knows how to derefence

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The deref trait requires that we use the deref function
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Deref coercion converts a reference to a type that implements the Deref trait into a reference
// to another type.
// For example, deref coercion can convert &String to &str because String implements the Deref
// trait such that it returns &str.
// Deref coercion is a convenience Rust performs on arguments to functions and methods,
// and works only on types that implement the Deref trait.
// It happens automatically when we pass a reference to a particular type’s value as an argument
// to a function or method that doesn’t match the parameter type in the function or method definition.
// A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Similar to how you use the Deref trait to override the * operator on immutable references,
// you can use the DerefMut trait to override the * operator on mutable references.

// Rust does deref coercion when it finds types and trait implementations in three cases:
// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>

// Rust will also coerce a mutable reference to an immutable one.
// But the reverse is not possible: immutable references will never coerce to mutable references.
// Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
