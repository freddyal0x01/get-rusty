# Reference Counted Smart Pointer

In Rust, a Reference Counted Smart Pointer is represented by the type Rc<T>,
which stands for Reference Counted. It is used to enable multiple ownership of
the same data, with reference counting to track how many owners exist at any
given time. When the last owner goes out of scope, the data is automatically
cleaned up.

Key Concepts:

- Single-threaded only: Rc<T> is not thread-safe and can only be used in
  single-threaded scenarios. For multithreaded programs, use Arc<T> (atomic
  reference counting).
- Reference counting: Tracks how many references exist to a piece of data.
- Shared ownership: Multiple parts of the program can own the same data.
- Immutable access: You can only borrow the data as immutable (&T), not mutable
  (&mut T).

When to Use Rc<T>:

- You need shared ownership in a single-threaded context.
- For building data structures like graphs or trees where multiple nodes might
  share ownership of the same data.
- When you want to avoid copying large data structures and prefer sharing
  references.

Pitfalls:

1. Not thread-safe: Use Arc<T> for multithreaded programs.
2. Cyclic references: Can lead to memory leaks. Rc<T> does not detect and clean
   up cycles. To prevent this, you can use Weak<T> for references that don’t
   increment the reference count.
