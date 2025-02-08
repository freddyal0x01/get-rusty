// A pointer is a general concept for a variable that contains an address in memory.
// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// Smart pointers own the data they point to.

// Smart pointers are usually implemented using structs.
// Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
// The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.
// The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

// Most common smart pointers:
// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

// Box smart pointers are used for the following:
// 1. If you can't know the exact size at compile time
// & we want to use a value of that type in a context which requires knowing the exact size.
// 2. We have a large amount of data and we want to transfer ownership of the data
// but we want to make sure that the data isn't copied because it's a large
// 3. We own the value and we only care that the value implements a specific trait
// rather than it being a specific type.

// Box Smart pointers are useful for dealing with recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
