// A pointer is a general concept for a variable that contains an address in memory.
// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// Smart pointers own the data they point to.

// Most common pointers:
// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
