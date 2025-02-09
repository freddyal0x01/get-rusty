# Reference Cycles Can Leak Memory

Rust can have memory leaks. We can see that Rust allows memory leaks by using
Rc<T> and RefCell<T>: it’s possible to create references where items refer to
each other in a cycle. This creates memory leaks because the reference count of
each item in the cycle will never reach 0, and the values will never be dropped.

Creating reference cycles is not easily done, but it’s not impossible either. If
you have RefCell<T> values that contain Rc<T> values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don’t create cycles; you can’t rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.

Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped.

## Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

You can also create a weak reference to the value within an Rc<T> instance by
calling Rc::downgrade and passing a reference to the Rc<T>.

Strong references are how you can share ownership of an Rc<T> instance.

Weak references don’t express an ownership relationship, and their count doesn’t
affect when an Rc<T> instance is cleaned up. They won’t cause a reference cycle
because any cycle involving some weak references will be broken once the strong
reference count of values involved is 0.
