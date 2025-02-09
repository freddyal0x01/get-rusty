# Shared-State Concurrency

Shared memory concurrency is like multiple ownership: multiple threads can
access the same memory location at the same time.

## Using Mutexes to Allow Access to Data from One Thread at a Time

Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one
thread to access some data at any given time. To access the data in a mutex, a
thread must first signal that it wants access by asking to acquire the mutex’s
lock. The lock is a data structure that is part of the mutex that keeps track of
which thread has exclusive access to the data. Therefore, the mutex is described
as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to
remember two rules:

1. You must attempt to acquire the lock before using the data.
2. When you’re done with the data that the mutex guards, you must unlock the
   data so other threads can acquire the lock.

## The API of Mutex<T>

Example: Using a mutex

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

`Mutex<T>` is a smart pointer. More accurately, the call to lock returns a smart
pointer called `MutexGuard`, wrapped in a `LockResult` that we handled with the
call to unwrap. The `MutexGuard` smart pointer implements `Deref` to point at
our inner data; the smart pointer also has a `Drop` implementation that releases
the lock automatically when a `MutexGuard` goes out of scope, which happens at
the end of the inner scope. As a result, we don’t risk forgetting to release the
lock and blocking the mutex from being used by other threads, because the lock
release happens automatically.

## Sharing a `Mutex<T>` Between Multiple Threads

Example: Sharing a value between multiple threads

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

When we compile this message, we get an error. The error message states that the
`counter` value was moved in the previous iteration of the loop. Rust is telling
us that we can’t move the ownership of `counter` into multiple threads.

## Multiple Ownership with Multiple Threads

We can give a value multiple owners by using the smart pointer `Rc<T>` to create
a reference counted value. In the example below, we can wrap the `Mutex<T>` in
`Rc<T>` and clone the `Rc<T>` before moving ownership to the thread.

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

We'll get another error:
`` Rc<Mutex<i32>>` cannot be sent between threads safely. The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>> ``

`Rc<T>` is not safe to share across threads. When `Rc<T>` manages the reference
count, it adds to the count for each call to `clone` and subtracts from the
count when each clone is dropped. But it doesn’t use any concurrency primitives
to make sure that changes to the count can’t be interrupted by another thread.
This could lead to wrong counts—subtle bugs that could in turn lead to memory
leaks or a value being dropped before we’re done with it. What we need is a type
exactly like `Rc<T>` but one that makes changes to the reference count in a
thread-safe way.

## Atomic Reference Counting with `Arc<T>`

`Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
The a stands for atomic, meaning it’s an atomically reference counted type.
Atomics are an additional kind of concurrency primitive that work like primitive
types but are safe to share across threads.

All primary types aren't atomic and standard library types don't use `Arc<T>` by
default becayse it comes with a performance penalty.

`Arc<T>` and `Rc<T>` have the same API, so we fix our program by changing the
`use` line, the call to `new`, and the call to `clone`.

Example code using `Arc<T>`:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

`counter` is immutable but we could get a mutable reference to the value inside
it; this means `Mutex<T>` provides interior mutability, as the `Cell` family
does. In the same way we used `RefCell<T>` to allow us to mutate contents inside
an `Rc<T>`, we use Mutex<T> to mutate contents inside an `Arc<T>`.

Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>.`
Using `Rc<T>` came with the risk of creating reference cycles, where two `Rc<T>`
values refer to each other, causing memory leaks. Similarly, `Mutex<T>` comes
with the risk of creating _deadlocks_. These occur when an operation needs to
lock two resources and two threads have each acquired one of the locks, causing
them to wait for each other forever.
