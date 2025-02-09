# RefCell<T> and the Interior Mutability Pattern

Interior mutability is a design pattern in Rust that allows you to mutate data
even when there are immutable references to that data; normally, this action is
disallowed by the borrowing rules.

To mutate data, the pattern uses unsafe code inside a data structure to bend
Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to
the compiler that we’re checking the rules manually instead of relying on the
compiler to check them for us

## Enforcing Borrowing Rules at Runtime

The RefCell smart pointer represents single ownership over the data that it
holds much like the Box pointer. The RefCell smart pointer enforces borrowing
rules at runtime.

The advantage of checking the borrowing rules at runtime instead is that certain
memory-safe scenarios are then allowed, where they would’ve been disallowed by
the compile-time checks. Static analysis, like the Rust compiler, is inherently
conservative.

The RefCell<T> type is useful when you’re sure your code follows the borrowing
rules but the compiler is unable to understand and guarantee that.

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and
will give you a compile-time error if you try using it in a multithreaded
context.

### Recap

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

- Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
  single owners.
- Box<T> allows immutable or mutable borrows checked at compile time; Rc<T>
  allows only immutable borrows checked at compile time; RefCell<T> allows
  immutable or mutable borrows checked at runtime.
- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate
  the value inside the RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the interior mutability pattern.

## Interior Mutability: A Mutable Borrow to an Immutable Value

A consequence of the borrowing rules is that when you have an immutable value,
you can’t borrow it mutably.

There are situations in which it would be useful for a value to mutate itself in
its methods but appear immutable to other code. Code outside the value’s methods
would not be able to mutate the value. Using RefCell<T> is one way to get the
ability to have interior mutability, but RefCell<T> doesn’t get around the
borrowing rules completely: the borrow checker in the compiler allows this
interior mutability, and the borrowing rules are checked at runtime instead. If
you violate the rules, you’ll get a panic! instead of a compiler error.

## Keeping Track of Borrows at Runtime with RefCell<T>

The RefCell<T> smart pointer checks the borrowing rules at runtime.

Borrowing rules:

1. RefCell<T> lets us have many immutable borrows or one mutable borrow at any
   point in time.
2. When a Ref<T> goes out of scope, the count of immutable borrows goes down by
   one. Just like the compile-time borrowing rules

## Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T>
lets you have multiple owners of some data, but it only gives immutable access
to that data. If you have an Rc<T> that holds a RefCell<T>, you can get a value
that can have multiple owners and that you can mutate!
